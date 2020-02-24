#![feature(proc_macro_hygiene)]
mod acme_lib;
mod lets_encrypt;
mod views;

include!(concat!(env!("OUT_DIR"), "/statics.rs"));

use {
    crate::views::devctm,
    actix_files::Files,
    actix_web::{error::ErrorNotFound, web, App, HttpResponse, HttpServer, Responder},
    listenfd::ListenFd,
    maud::{html, Markup},
    openssl::ssl::{SslAcceptor, SslFiletype, SslMethod},
    std::{env, io, path::PathBuf},
};

fn stylesheet_link_tag(filename: &str, media: &str) -> Markup {
    html! {
        link rel="stylesheet" media=(media) href=(format!("/assets/{}.css", filename));
    }
}

// height and width should be Option<u16>, but if I were to go further with
// this, we'd probaby want to use a Struct for the values so that they would
// have names associated with them rather than just their position in the
// parameter list.
fn image_tag(filename: &str, height: u16, width: u16, alt: &str) -> Markup {
    html! {
        img height=(height) width=(width) alt=(alt) src=(format!("/assets/{}", filename));
    }
}

pub async fn asset(info: web::Path<String>) -> impl Responder {
    let asset = info.into_inner();
    let asset_path: PathBuf = asset.clone().into();
    // unwrap is safe below, because we already put the &'static str
    // into the hash and we have no desire to support assets whose
    // names are non-utf8 byte sequences.
    let asset: &str = &asset;
    match STATICS.get(asset) {
        None => Err(ErrorNotFound(format!("Could not find {}", asset))),
        Some(bytes) => Ok(HttpResponse::Ok()
            .content_type(content_type(&asset_path))
            .body(*bytes)),
    }
    // probably also want an Expires and Cache-Control header (maybe also etag)
}

fn content_type(asset: &PathBuf) -> &'static str {
    static CONTENT_TYPES: Map<&'static str, &'static str> = phf_map! {
        "css" => "text/css",
        "gif" => "image/gif",
        "jpg" => "image/jpg",
        "pdf" => "application/pdf",
    };

    // unwrap is mostly safe below in that we only look for the
    // extension on an asset we've found, and *IN THEORY* all our
    // assets have known suffixes.  I guess it's possible that we
    // could add an asset w/o a suffix or w/o a known suffix, but that
    // really just shows that the tool we use to build our static
    // assets is weak.  If this were a more important app, we'd put
    // more work into build.rs and still keep the unwraps.
    let extension = asset.extension().unwrap().to_str().unwrap();

    CONTENT_TYPES.get(extension).unwrap()
}

// TODO: DRY the /var/lib/lets-encrypt

const ACME_DIR: &str = "/var/lib/lets-encrypt/acme";
const PERSISTENCE_DIR: &str = "/var/lib/lets-encrypt/persistence";
const CERT_DIR: &str = "/var/lib/lets-encrypt/key_and_cert";

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    // To run locally w/o any https support:
    // DEVCTM_LE_CONFIG='{"cert_builders":[]}'

    // My overly complicated production deploy:
    // DEVCTM_LE_CONFIG='{"nonce_directory":"/var/devctm","ssl_directory":"ssl","cert_builders":[{"addrs":["0.0.0.0:8089"],"domains":["devctm.com"],"email":"ctm@devctm.com"},{"addrs":["0.0.0.0:8090"],"domains":["ardi.com","sceim.net"],"email":"ctm@ardi.com"},{"addrs":["0.0.0.0:8091"],"domains":["test.devctm.com"],"email":"ctm@devctm.com","production":false}]}'

    // DEVCTM_HTTP_PORT (or 8088) is for all http and is bound after we set up the server.

    let mut key_path = None;
    let mut cert_path = None;

    loop {
        let mut server = HttpServer::new(move || {
            App::new()
                .route("/", web::get().to(devctm::index))
                .service(web::resource("/assets/{asset:.*}").to(asset))
                .service(Files::new("/.well-known/acme-challenge/", ACME_DIR))
        });

        server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
            server.listen(l)?
        } else {
            let port = env::var("DEVCTM_HTTP_PORT").unwrap_or_else(|_| "8088".to_string());
            let address = format!("0.0.0.0:{}", port);

            server.bind(address)?
        };
        if let Some(key_path) = key_path.take() {
            if let Some(cert_path) = cert_path.take() {
                let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                builder
                    .set_private_key_file(&key_path, SslFiletype::PEM)
                    .unwrap();
                builder.set_certificate_chain_file(&cert_path).unwrap();
                server = server.bind_openssl("0.0.0.0:8090", builder).unwrap();
            }
        }
        let server = server.run();
        if let Some((k_path, c_path)) = lets_encrypt::start(&server) {
            key_path = Some(k_path);
            cert_path = Some(c_path);
        };
        server.await?
    }
}
