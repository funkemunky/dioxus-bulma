use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ColumnsProps {
    #[props(default)]
    is_mobile: bool,

    #[props(default)]
    is_gapless: bool,

    #[props(default)]
    is_multiline: bool,

    #[props(default)]
    is_centered: bool,

    #[props(default)]
    is_vcentered: bool,

    #[props(optional)]
    variable_gap: Option<u8>,

    // #[props(optional)]
    // custom_class: Option<String>,

    children: Element,
}

pub fn Columns<'a>(props: ColumnsProps) -> Element {
    let mut class_name = "columns".to_string();

    if props.is_mobile {
        class_name += " is-mobile";
    }

    if props.is_gapless {
        class_name += " is-gapless";
    }

    if props.is_multiline {
        class_name += " is-multiline";
    }

    if props.is_centered {
        class_name += " is-centered";
    }

    if props.is_vcentered {
        class_name += " is-vcentered";
    }

    if let Some(num) = props.variable_gap {
        if (0..=8).contains(&num) {
            class_name = format!("{class_name} is-variable is-{num}");
        }
    }

    // if let Some(class) = &props.custom_class {
    //     class_name += class;
    // }

    rsx! {
        div {
            class: "{class_name}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ColumnProps {
    #[props(default)]
    is_narrow: bool,

    #[props(optional)]
    size: Option<u8>,

    #[props(optional)]
    offset: Option<u8>,

    // #[props(optional)]
    // custom_class: Option<String>,

    children: Element,
}

pub fn Column<'a>(props: ColumnProps) -> Element {
    let mut class_name = "column".to_string();

    if props.is_narrow {
        class_name += " is-narrow";
    }

    if let Some(num) = props.size {
        if (0..12).contains(&num) {
            class_name = format!("{class_name} is-{num}");
        }
    }

    if let Some(num) = props.offset {
        if (0..12).contains(&num) {
            class_name = format!("{class_name} is-offset-{num}");
        }
    }

    // if let Some(class) = &props.custom_class {
    //     class_name += class;
    // }

    rsx! {
        div {
            class: "{class_name}",
            {props.children}
        }
    }
}
