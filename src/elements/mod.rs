mod button;
mod image;
mod notification;
mod progress;
pub mod tag;
use dioxus::prelude::*;

pub use button::{Button, ButtonProps, ButtonState};
pub use image::{Image, ImageProps};
pub use notification::{Notification, NotificationProps};
pub use progress::{Progress, ProgressProps};

#[component]
pub fn Block(children: Element) -> Element {
    rsx! {
        div {
            class: "block",
            {children}
        }
    }
}

#[component]
pub fn BoxElement(children: Element) -> Element {
    rsx! {
        div {
            class: "box",
            {children}
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ContentProps {
    size: Option<crate::Sizes>,

    children: Element,
}

pub fn Content<'a>(props: ContentProps) -> Element {
    let extra_class = if props.size.is_some() {
        props.size.as_ref().unwrap().to_string()
    } else {
        String::new()
    };
    rsx! {
        div {
            class: "content {extra_class}",
            {props.children}
        }
    }
}
