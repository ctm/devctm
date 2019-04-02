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
"I'm Cliff Matthews, a contract programmer."
                            }
                            p {
                                (PreEscaped(
r#"Perhaps you're here because you're considering becoming my client,
or&mdash;for all I know&mdash;this page has been linked to as an
example of what not to do. Regardless, I'm having fun and am glad you
dropped by."#
                                        ))
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
got my Masters Degree in Computer Science from the University of New Mexico
in 1985 and I've been an entrepreneur and contractor ever since."#
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
                                a href="https://pdfs.semanticscholar.org/aa66/62b2092e148e7db7004d2030294df51fcb2f.pdf" { "footnotes 113 & 114 on p. 44" } ")."
                            }
                            p {
r#"On Halloween 1998, I wrote the core of "multibot", the first internet
server to deal multi-table poker tournaments.  I sold multibot to Poker School
Online who were eventually bought by PokerStars."#
                            }
                            p {
r#"FWIW, one of
the founders of PokerStars had actually heard about multibot between when I
had agreed to sell it to PSO and when I received my check. PokerStars
offered to pay me twice what I had already agreed on and I turned it down,
because I had already shaken hands on the deal."#
                            }
                            p {
r#"Multibot was Objective-C, without a framework, just the GNU  Objective-C
runtime.  Multihreaded, using epoll it was very efficient. However, when PSO
upgraded to a dual processor, they had an unexplained crash. Wow. I
discovered "#
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
r#"I'm currently seeking contract work as a Rust programmer, even though I'm
new to the language and not particularly good or efficient.  As such, for now,
I'm better suited to a supporting role rather than a lead architect.  I'm
inspired by what some people are doing with Rust and I'll certainly be doing
a lot of side projects in Rust, even if I have to program in Ruby (which I
still love) to pay the bills."#
                            }
                            p {
                                "This page is a "
                                a href="https://github.com/ctm/devctm" { "tiny Rust program" }
" using "
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
