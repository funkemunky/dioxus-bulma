use dioxus::{events::MouseEvent, prelude::*};

use crate::{Colors, Sizes};

#[derive(PartialEq, Clone)]
pub enum ButtonState {
    Normal,
    Hover,
    Focus,
    Active,
    Loading,
    Static,
    Disabled,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    #[props(optional)]
    r#type: Option<String>,

    #[props(optional)]
    color: Option<Colors>,

    #[props(optional)]
    size: Option<Sizes>,

    #[props(default)]
    state: ButtonState,

    #[props(default)]
    is_light: bool,

    #[props(default)]
    is_outlined: bool,

    #[props(default)]
    is_inverted: bool,

    #[props(default)]
    is_rounded: bool,

    #[props(default)]
    is_fullwidth: bool,

    #[props(default)]
    onclick: EventHandler<MouseEvent>,

    #[props(default)]
    onmousedown: EventHandler<MouseEvent>,

    #[props(default)]
    onmouseup: EventHandler<MouseEvent>,


    children: Element,
}

pub fn Button<'a>(props: ButtonProps) -> Element {
    let mut class_name = "button".to_string();

    if let Some(color) = props.color {
        let color_name = color.to_string();
        class_name = format!("{class_name} is-{color_name}");
    }

    if let Some(size) = props.size {
        let size_name = size.to_string();
        class_name = format!("{class_name} is-{size_name}");
    }

    if props.is_light {
        class_name += " is-light";
    }

    if props.is_outlined {
        class_name += " is-outlined";
    }

    if props.is_inverted {
        class_name += " is-inverted";
    }

    if props.is_rounded {
        class_name += " is-rounded";
    }

    if props.is_fullwidth {
        class_name += " is-fullwidth";
    }

    let state = props.state;
    let mut disabled = "false";
    if state != ButtonState::Normal {
        match state {
            ButtonState::Normal => {}
            ButtonState::Hover => {
                class_name += " is-hovered";
            }
            ButtonState::Focus => {
                class_name += " is-focused";
            }
            ButtonState::Active => {
                class_name += " is-active";
            }
            ButtonState::Loading => {
                class_name += " is-loading";
            }
            ButtonState::Static => {
                class_name += " is-static";
            }
            ButtonState::Disabled => {
                disabled = "true";
            }
        }
    }

    let button_type: String = if let Some(t) = props.r#type {
       t
    } else {
        "button".to_string()
    };

    rsx! {
        button {
            class: "{class_name}",
            r#type: "{button_type}",
            disabled: "{disabled}",
            onclick: move |evt| props.onclick.call(evt),
            onmousedown: move |evt| props.onmousedown.call(evt),
            onmouseup: move |evt| props.onmouseup.call(evt),
            {props.children}
        }
    }
}

#[component]
pub fn Buttons(children: Element) -> Element {
    rsx! {
        div {
            class: "buttons",
            {children}
        }
    }
}