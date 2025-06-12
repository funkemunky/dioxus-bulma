use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct CardProps {
    header: Element,

    content: Element,

    image: Element,

    footer: Element,
}

pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "card",
            header {
                    class: "card-header",
                    {props.header}
                },
            div {
                    class: "card-header",
                    {props.image}
            },
            div {
                    class: "card-header",
                    {props.content}
            },
            footer {
                    class: "card-header",
                    {props.footer}
            }
        }
    }
}
