use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use dioxus_bulma::prelude::{
    tag::{Tag, Tags, TagLink},
    *,
};

fn main() {
    dioxus_desktop::launch_cfg(App, Config::new()
        .with_window(WindowBuilder::new().with_title("Dioxus Bulma - Full Example")
        .with_inner_size(LogicalSize::new(1200, 700))))
}

#[allow(non_snake_case)]
fn App(cx: Scope) -> Element {

    let cdn_url = dioxus_bulma::get_bulma_cdn();

    let click_me = |_| {
        println!("Clicked!");
    };

    cx.render(rsx! {
        link { rel: "stylesheet", href: cdn_url }
        Container {
            Columns {
                Column {
                    Button {
                        color: Colors::Success,
                        is_fullwidth: true,
                        onclick: click_me,
                        "Click Me"
                    }
                }
            }
        }
    })
}
