#![feature(proc_macro_hygiene)]
mod views;

include!(concat!(env!("OUT_DIR"), "/statics.rs"));

use {
    crate::views::devctm,
    actix_web::{
        actix::Actor, error::ErrorNotFound, http::Method, middleware::Logger, server, App,
        HttpRequest, HttpResponse, Result,
    },
    actix_web_lets_encrypt::LetsEncrypt,
    listenfd::ListenFd,
    maud::{html, Markup},
    std::{env, path::PathBuf},
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

fn asset(req: &HttpRequest) -> Result<HttpResponse> {
    let asset_path: PathBuf = req.match_info().query("asset")?;
    let asset = asset_path.to_str().unwrap();

    // unwrap is safe below, because we already put the &'static str
    // into the hash and we have no desire to support assets whose
    // names are non-utf8 byte sequences.
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

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    // To run locally w/o any https support:
    // DEVCTM_LE_CONFIG='{"cert_builders":[]}'

    // My overly complicated production deploy:
    // DEVCTM_LE_CONFIG='{"nonce_directory":"/var/devctm","ssl_directory":"ssl","cert_builders":[{"addrs":["0.0.0.0:8089"],"domains":["devctm.com"],"email":"ctm@devctm.com"},{"addrs":["0.0.0.0:8090"],"domains":["ardi.com","sceim.net"],"email":"ctm@ardi.com"},{"addrs":["0.0.0.0:8091"],"domains":["test.devctm.com"],"email":"ctm@devctm.com","production":false}]}'

    // DEVCTM_HTTP_PORT (or 8088) is for all http and is bound after we set up the server.
    let app_encryption_enabler = LetsEncrypt::encryption_enabler_from_env("DEVCTM_LE_CONFIG");

    let server_encryption_enabler = app_encryption_enabler.clone();

    let mut server = server::new(move || {
        App::new().configure(|app| {
            let app = app
                .middleware(Logger::default())
                .resource("/assets/{asset:.*}", |r| r.method(Method::GET).f(asset))
                .resource("/", |r| r.method(Method::GET).f(devctm::index));
            app_encryption_enabler.register(app)
        })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        let port = env::var("DEVCTM_HTTP_PORT").unwrap_or_else(|_| "8088".to_string());
        let address = format!("0.0.0.0:{}", port);

        server_encryption_enabler
            .attach_certificates_to(server)
            .bind(address)
            .unwrap()
    };
    server_encryption_enabler.start();
    server.run();
}
