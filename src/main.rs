#![feature(proc_macro_hygiene)]

mod acme;

include!(concat!(env!("OUT_DIR"), "/statics.rs"));

use {
    actix_web::{
        error::ErrorNotFound, http::Method, middleware::Logger, server, App, HttpRequest,
        HttpResponse, Result,
    },
    listenfd::ListenFd,
    maud::{html, Markup, PreEscaped, DOCTYPE},
    openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod},
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

fn index(_req: &HttpRequest) -> Markup {
    html! {
        (DOCTYPE)
            html {
                meta charset="utf-8";
                head {
                    title { "Cliff's Toy Rust (Actix) Server" }
                    ( stylesheet_link_tag("application", "all") )
                }
                body#more {
                    div#container {
                        div#header {
                          ( image_tag("devctm_logo.gif", 24, 100, "devctm") )

                        }
                        div#main-body {
                            div#devctm {
                                h1 { "devctm" }
                                p {
                                    r#"
I'm Cliff Matthews, a professional programmer since 1978 and contractor since 1985.
                                    "#
                                }
                                p {
                                    r#"
I earned my MSCS from the University of New Mexico in 1985, applied to UC
Berkeley's Ph.D. program, didn't get in, and returned to industry.
                                    "#
                                }
                                p {
                                    r#"
In 1989, I founded Abacus Research & Development. We wrote a
                                    "#
                                    a href="https://archive.org/details/executor" {
                                        r#"
    clean-room reimplementation of the Macintosh ROM and portions of Mac OS"#
                                            "."
                                    }
                                }
                                p {
                                    r#"
At ARDI, I got interested in poker. Mat Hostetter's keen
insight led the two of us to writing what was then the world's
fastest poker hand evaluator ("#
a href="https://pdfs.semanticscholar.org/aa66/62b2092e148e7db7004d2030294df51fcb2f.pdf" { "footnotes 113 & 114 on p. 44" } ")."
                                }
                                p {
                                    r#"
On Halloween 1998, I wrote the core
of "multibot", the first internet server to deal multi-table poker
tournaments.  I sold multibot to Poker School Online who
were eventually bought by PokerStars.
                                    "#
                                }
                                p {
                                    r#"
Multibot was Objective-C, without a framework, just the GNU
 Objective-C runtime.  Multihreaded, using epoll it was very
efficient. However, when PSO upgraded to a dual processor, they had
an unexplained crash. Wow. I discovered "#
                                        a href="https://gcc.gnu.org/bugzilla/show_bug.cgi?id=9969" { "a race condition" }
                                    " in the Objective-C runtime."
                                }
                                p {
                                    r#"
In 2006, I co-founded Stolen Bases LLC, a Ruby on Rails shop.  
Although headquartered in Manhatten, I put together a development team
in Albuquerque.  In 2009, "#
                                    a href="https://elinemedia.com" { "E-Line Media" }
                                    " bought Stolen Bases."
                                }
                                p {
                                    "Since then, I've been doing business as "
                                    span.devctm { "devctm" }
                                    r#", a name meant to imply that I'm a coding "machine". However with almost all of my
recent work in private repositories, I don't have much code
to show.  On the other hand, I started running in 2009 and have participated
in many
                                    "#
                                    a href="https://ultrasignup.com/results_participant.aspx?fname=Clifford&lname=Matthews" { "running events" }
                                    r#", so perhaps we can stipulate that I'm
a running machine and that I am driven.
                                    "#
                                }
                                p {
                                    r#"
This site is a tiny Rust program using
                                    "#
                                    a href="https://actix.rs" { "actix" }
                                    ", "
                                    a href="https://maud.lambda.xyz" { "maud" }
                                    " and "
                                    a href="https://github.com/kaj/rsass" { "rsass" }
                                    "."
                                }
                            }
                            div#contact {
                                h1 { "Availability and Rates" }
                                div.wrapper {
                                    p {
                                        "My rate for Ruby is $200/hr."
                                        br; br;
                                        "My rate for Rust is $100/hr. through September 30th, 2019."
                                        br; br;
                                        "If interested, please email "
                                        a href="mailto:ctm@devctm.com" {
                                            "ctm@devctm.com"
                                        }
                                    }
                                }
                            }
                            div#resumes {
                                h1 { (PreEscaped("Links")) }
                                div.wrapper {
                                    ul {
                                        li {
                                            a href="/assets/resume.pdf" {
                                                "Resume"
                                            }
                                        }
                                        li {
                                            a href="https://github.com/ctm" {
                                                "GitHub"
                                            }
                                        }
                                        li {
                                            a href="https://www.builtinnm.org/people/20" {
                                                "Built in NM"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
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

fn ssl_builder(host: &str) -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(format!("ssl/{}_key.pem", host), SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(format!("ssl/{}_cert.pem", host))
        .unwrap();
    builder
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut listenfd = ListenFd::from_env();

    let mut server = server::new(|| {
        App::new()
            .middleware(Logger::default())
            .resource("/assets/{asset:.*}", |r| r.method(Method::GET).f(asset))
            .resource("/", |r| r.method(Method::GET).f(index))
            .resource("/.well-known/acme-challenge/{token}", |r| {
                r.method(Method::GET).f(acme::nonce)
            })
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server
            .bind("0.0.0.0:8088")
            .unwrap()
            .bind_ssl("0.0.0.0:8089", ssl_builder("devctm"))
            .unwrap()
            .bind_ssl("0.0.0.0:8090", ssl_builder("ardi"))
            .unwrap()
    };

    server.run();
}
