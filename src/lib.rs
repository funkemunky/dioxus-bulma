#![allow(non_snake_case)]

use std::fmt::Display;

pub mod columns;
pub mod components;
pub mod elements;
pub mod layout;

pub mod prelude {
    pub use crate::{columns::*, components::*, elements::*, layout::*};
    pub use crate::{get_bulma_cdn, get_bulma_css, get_bulma_ext_cdn, get_bulma_extjs_cdn, Colors, Sizes};
}

/// include bulma with css content
pub fn get_bulma_css() -> &'static str {
    include_str!("./assets/bulma.min.css")
}

/// get bulma cdn url
pub fn get_bulma_cdn() -> &'static str {
    "https://cdn.jsdelivr.net/npm/bulma@0.9.3/css/bulma.min.css"
}

pub fn get_bulma_ext_cdn() -> &'static str {
    "https://cdn.jsdelivr.net/npm/bulma-extensions@6.2.7/dist/css/bulma-extensions.min.css"
}

pub fn get_bulma_extjs_cdn() -> &'static str {
    "https://cdn.jsdelivr.net/npm/bulma-extensions@6.2.7/dist/js/bulma-extensions.min.js"
}

#[derive(Clone, PartialEq)]
pub enum Colors {
    Primary,
    Link,
    Info,
    Success,
    Warning,
    Danger,
    Black,
    Dark,
    Light,
    White,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Colors::Primary => "primary",
            Colors::Link => "link",
            Colors::Info => "info",
            Colors::Success => "success",
            Colors::Warning => "warning",
            Colors::Danger => "danger",
            Colors::Black => "black",
            Colors::Dark => "dark",
            Colors::Light => "light",
            Colors::White => "white",
        }
            .to_string();
        write!(f, "{}", str)
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Sizes {
    Small,
    Normal,
    Medium,
    Large,
}

impl Display for Sizes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Sizes::Small => "small",
            Sizes::Normal => "normal",
            Sizes::Medium => "medium",
            Sizes::Large => "large",
        }
            .to_string();
        write!(f, "{}", str)
    }
}

impl Default for Sizes {
    fn default() -> Self {
        Self::Normal
    }
}
