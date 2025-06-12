use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ContainerProps {
    #[props(default)]
    widescreen: bool,

    #[props(default)]
    fullhd: bool,

    #[props(default)]
    max_desktop: bool,

    #[props(default)]
    max_widescreen: bool,

    #[props(default)]
    fluid: bool,

    children: Element,
}

pub fn Container(props: ContainerProps) -> Element {

    let extra_class = if props.widescreen {
        "is-widescreen"
    } else if props.fullhd {
        "is-fullhd"
    } else if props.max_desktop {
        "is-max-desktop"
    } else if props.max_widescreen {
        "is-max-widescreen"
    } else if props.fluid {
        "is-fluid"
    } else {
        ""
    };


    rsx! {
        div {
            class: "container {extra_class}",
            {props.children}
        }
    }
}
