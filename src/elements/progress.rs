use dioxus::prelude::*;

use crate::{Colors, Sizes};

#[derive(Props, PartialEq, Clone)]
pub struct ProgressProps {
    #[props(default)]
    max: u16,
    #[props(default)]
    value: u16,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(optional)]
    color: Option<Colors>,
}

pub fn Progress(props: ProgressProps) -> Element {
    let mut extra_class = String::new();

    if props.size.is_some() {
        extra_class += &format!(" is-{}", props.size.as_ref().unwrap().to_string());
    }

    if props.color.is_some() {
        extra_class += &format!(" is-{}", props.color.as_ref().unwrap().to_string());
    }

    if props.value != 0 {
        rsx! {
            progress {
                class: "progress {extra_class}",
                value: "{props.value}",
                max: "{props.max}",
            }
        }
    } else {
        rsx! {
            progress {
                class: "progress {extra_class}",
                max: "{props.max}",
            }
        }
    }
}
