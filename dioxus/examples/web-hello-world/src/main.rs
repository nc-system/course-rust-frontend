#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }

        div {
            h1 {color: "blue", "COUNTER - RUST - DIOXUS"}
        }
        div {
            h2 { "High-Five counter: {count}" }
            button {
                padding_top: "5px", padding_right: "8px", padding_bottom: "5px", padding_left: "8px",
                border_radius: "8px", border_style: "none", cursor: "pointer",
                onclick: move |_| count += 1, "+"
            }
            button { onclick: move |_| count -= 1, "-" }
        }
    }
}
