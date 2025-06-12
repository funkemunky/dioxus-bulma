use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ImageProps{
    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    ratio: Option<(u8, u8)>,

    #[props(default)]
    is_fullwidth: bool,

    src: String,
}

pub fn Image(props: ImageProps) -> Element {
    let mut class_name = "image".to_string();

    if let Some(size) = props.size {
        class_name = format!("{class_name} is-{size}x{size}");
    }

    if let Some(ratio) = props.ratio {
        let a = ratio.0;
        let b = ratio.1;
        class_name = format!("{class_name} is-{a}by{b}");
    }

    if props.is_fullwidth {
        class_name += " is-fullwidth";
    }

    rsx! {
        figure {
            class: "{class_name}",
            img {
                src: props.src
            }
        }
    }
}
