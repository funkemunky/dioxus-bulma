use dioxus::prelude::*;

use crate::Sizes;

#[derive(Clone, PartialEq)]
pub enum Separator {
    Arrow,
    Bullet,
    Dot,
    Succeeds,
}

impl ToString for Separator {
    fn to_string(&self) -> String {
        match self {
            Separator::Arrow => "arrow",
            Separator::Bullet => "bullet",
            Separator::Dot => "dot",
            Separator::Succeeds => "succeeds",
        }.to_string()
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct BreadcurmbProps {
    #[props(default)]
    is_centered: bool,
    #[props(default)]
    is_right: bool,

    #[props(optional)]
    separator: Option<Separator>,
    
    #[props(optional)]
    size: Option<Sizes>,

    children: Element,
}

pub fn Breadcurmb(props: BreadcurmbProps) -> Element {

    let mut extra_class = String::new();
    
    if props.is_centered {
        extra_class += " is-centered";
    } else if props.is_right {
        extra_class += " is-right";
    }

    if props.separator.is_some() {
        let separator = props.separator.as_ref().unwrap().to_string();
        extra_class += &format!(" has-{}-separator", separator);
    }

    if props.size.is_some() {
        let size = props.size.as_ref().unwrap().to_string();
        extra_class += &format!(" is-{}", size);
    }

    rsx! {
        nav {
            class: "breadcrumb {extra_class}",
            {props.children}
        }    
    }
}