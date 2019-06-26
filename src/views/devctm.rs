use {
    crate::{stylesheet_link_tag, image_tag},
    actix_web::HttpRequest,
    maud::{DOCTYPE, Markup, PreEscaped, html},
};

pub fn index(_req: &HttpRequest) -> Markup {
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
"I'm Cliff Matthews, programmer."
                            }
                            p {
r#"I'm an honest, loyal, driven person with sufficient experience to help you
sleep at night."#
                            }
                            p {
r#"Born in late 1962, I got into programming via the August 1976
Popular Electronics "#
                                a href="https://billr.incolor.com/elf/html/elf-1-33.htm" { r#"COSMAC "Elf" article"# }
r#". By 1978, I was paid to write PDP-8 assembly.  I 
got my MSCS from the University of New Mexico
in 1985 and I've been a contractor and entrepreneur ever since."#
                            }
                            p {
"In 1989, I founded Abacus Research & Development. We wrote a "
                                a href="https://archive.org/details/executor" {
"clean-room reimplementation of the Macintosh ROM and portions of Mac OS"
                                }
r#".  I wrote the first 10,000 lines (in C) of ROMlib on a diskless Mac+ that
only had a single floppy drive."#
                            }
                            p {
r#"At ARDI, I got interested in poker. Mat Hostetter's keen insight led the
two of us to writing what was then the world's fastest poker hand evaluator ("#
                                a href="https://pdfs.semanticscholar.org/aa66/62b2092e148e7db7004d2030294df51fcb2f.pdf" { "footnotes 113 & 114 on p. 44" }
r#"). On Halloween 1998, I wrote the core of "multibot", the first internet
server to deal multi-table poker tournaments.  I sold multibot to Poker School
Online who were eventually bought by PokerStars."#
                            }
                            p {
r#"Multibot was Objective-C, without a framework, just the GNU  Objective-C
runtime.  Multihreaded, using epoll it was very efficient. However, when PSO
upgraded to a dual processor, they had a single crash and asked me to
investigate. Whee! I discovered "#
                                a href="https://gcc.gnu.org/bugzilla/show_bug.cgi?id=9969" { "a race condition" }
" in the Objective-C runtime."
                            }
                            p {
                                r#"
In 2006, I co-founded Stolen Bases LLC, a Ruby on Rails shop.  
Although headquartered in Manhattan, I put together a development team
in Albuquerque.  In 2009, "#
                                    a href="https://elinemedia.com" { "E-Line Media" }
                                " bought Stolen Bases."
                            }
                            p {
                                "Since then, I've been doing business as "
                                    span.devctm { "devctm" }
r#", a name meant to imply that I'm a coding "machine". However with almost all
of my recent work in private repositories, I don't have much code
to show.  On the other hand, I "#
                                    a href="https://ultrasignup.com/results_participant.aspx?fname=Clifford&lname=Matthews" { "run a lot" }
", so let's stipulate that I'm extremely internally motivated."
                            }
                            p {
r#"I'm seeking work as a Rust programmer, even though I'm not yet nearly as
good or efficient in Rust as I am in Ruby.  However, I learn quickly and believe
I am "training my brain" as effectively as I've been "#
                                    a href="https://github.com/ctm/Bataan-Memorial-Death-March" { "training my body" }
".  My Rust will improve significantly."
                            }
                            p {
                                "This page is a "
                                a href="https://github.com/ctm/devctm" { "Rust program" }
" using "
                                    a href="https://actix.rs" { "actix" }
                                ", "
                                    a href="https://maud.lambda.xyz" { "maud" }
                                " and "
                                    a href="https://github.com/kaj/rsass" { "rsass" }
                                ". I wrote "
                                    a href="https://github.com/ctm/actix-web-lets-encrypt" { "actix-web-lets-encrypt" }
                                " for https support."
                            }
                        }
                        div#contact {
                            h1 { "Availability" }
                            div.wrapper {
                                p {
                                    "I'm Available and willing to work for less if I get to do so in Rust."
                                        br; br;
                                    "If interested, please email "
                                        a href="mailto:clifford.t.matthews@gmail.com" {
                                            "clifford.t.matthews@gmail.com"
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
