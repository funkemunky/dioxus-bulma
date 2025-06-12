use dioxus::prelude::*;
use crate::Colors;

#[derive(Props, PartialEq, Clone)]
pub struct NotificationProps {
    #[props(optional)]
    color: Option<Colors>,

    #[props(default)]
    is_light: bool,

    #[props(default)]
    is_deletable: bool,

    children: Element,
}

pub fn Notification(props: NotificationProps) -> Option<Element> {
    let mut closed = use_signal(|| false);
    if *closed.read() {
        return None;
    }

    let mut class_name = "notification".to_string();

    if let Some(color) = props.color {
        let color_name = color.to_string();
        class_name = format!("{class_name} is-{color_name}");
    }

    if props.is_light {
        class_name += " is-light";
    }

    if props.is_deletable {
        Some(rsx! {
            div {
                class: "{class_name}",
                button {
                    class: "delete",
                    onclick: move |_| { closed.set(true); }
                }
                {props.children}
            }
        })
    } else {
        Some(rsx! {
            div {
                class: "{class_name}",
                {props.children}
            }
        })
    }
}
