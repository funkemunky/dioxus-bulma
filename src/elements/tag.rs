use dioxus::{events::MouseEvent, prelude::*};

use crate::{Colors, Sizes};

#[derive(Props, PartialEq, Clone)]
pub struct TagProps {
    #[props(optional)]
    color: Option<Colors>,

    #[props(default)]
    is_light: bool,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(default)]
    is_rounded: bool,

    #[props(default)]
    deletable: bool,

    children: Element,
}

pub fn Tag<'a>(props: TagProps) -> Option<Element> {
    let mut visible = use_signal(|| true);
    if !*visible.read() {
        return None;
    }

    let mut extra_class = String::new();

    if props.color.is_some() {
        extra_class += &format!(" is-{}", props.color.as_ref().unwrap().to_string());
    }

    if props.is_light {
        extra_class += " is-light";
    }

    if props.size.is_some() {
        extra_class += &format!(" is-{}", props.size.as_ref().unwrap().to_string());
    }

    if props.is_rounded {
        extra_class += " is-rounded";
    }

    if props.deletable {
        let delete_button_size = match props.size.as_ref().unwrap_or(&Sizes::Normal) {
            Sizes::Small => "small",
            Sizes::Normal => "small",
            Sizes::Medium => "normal",
            Sizes::Large => "medium",
        };
        Some(rsx! {
            span {
                class: "tag {extra_class}",
                {props.children},
                button {
                    class: "delete is-{delete_button_size}",
                    onclick: move |_| {
                        visible.set(false);
                    }
                }
            }
        })
    } else {
        Some(rsx! {
            span {
                class: "tag {extra_class}",
                {props.children}
            }
        })
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TagLinkProps {
    #[props(optional)]
    color: Option<Colors>,

    #[props(default)]
    is_light: bool,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(default)]
    is_rounded: bool,

    #[props(default)]
    deletable: bool,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,

    children: Element,
}

pub fn TagLink<'a>(props: TagLinkProps) -> Option<Element> {
    let mut visible = use_signal(|| true);
    if !*visible.read() {
        return None;
    }

    let mut extra_class = String::new();

    if props.color.is_some() {
        extra_class += &format!(" is-{}", props.color.as_ref().unwrap().to_string());
    }

    if props.is_light {
        extra_class += " is-light";
    }

    if props.size.is_some() {
        extra_class += &format!(" is-{}", props.size.as_ref().unwrap().to_string());
    }

    if props.is_rounded {
        extra_class += " is-rounded";
    }

    if props.deletable {
        let delete_button_size = match props.size.as_ref().unwrap_or(&Sizes::Normal) {
            Sizes::Small => "small",
            Sizes::Normal => "small",
            Sizes::Medium => "normal",
            Sizes::Large => "medium",
        };
        Some(rsx! {
            a {
                class: "tag {extra_class}",
                onclick: move |evt| props.onclick.call(evt),
                {props.children},
                button {
                    class: "delete is-{delete_button_size}",
                    onclick: move |_| {
                        visible.set(false);
                    }
                }
            }
        })
    } else {
        Some(rsx! {
            a {
                class: "tag {extra_class}",
                onclick: move |evt| props.onclick.call(evt),
                {props.children}
            }
        })
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TagsProps {
    #[props(default)]
    addons: bool,

    children: Element,
}

pub fn Tags(props: TagsProps) -> Element {
    let extra_class = if props.addons { " has-addons" } else { "" };
    rsx! {
        div {
            class: "tags {extra_class}",
            {props.children}
        }
    }
}
