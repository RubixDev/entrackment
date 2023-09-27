use std::borrow::Cow;

use dioxus::prelude::*;
use dioxus_free_icons::IconShape;
use material_dioxus::MatIcon;
use schema::Color;

use crate::components::general::Icon;

#[derive(Props)]
pub struct ChipProps<'a> {
    label: &'a str,
    color: Color,
    icon_left: &'a dyn IconShape,
    #[props(into)]
    icon_right: Option<Option<Cow<'static, str>>>,
    icon_size: &'a str,
}

pub fn Chip<'a>(cx: Scope<'a, ChipProps<'a>>) -> Element<'a> {
    let (r, g, b) = cx.props.color;
    render! {
        // TODO: light theme
        span {
            background_color: "rgba({r}, {g}, {b}, 0.4)",
            border: "1px solid rgb({r}, {g}, {b})",
            border_radius: "2em",
            box_sizing: "border-box",
            font_size: "0.75rem",
            line_height: "1.125rem",
            display: "inline-flex",
            padding: "0 0.7rem 0 0.5rem",
            align_items: "center",
            gap: "0.5rem",
            color: "var(--mdc-theme-on-surface)",
            height: "1.5rem",
            Icon {
                class: "md-icon",
                fill: "currentColor",
                icon: cx.props.icon_left,
                width: cx.props.icon_size,
                height: cx.props.icon_size
            }
            span { "{cx.props.label}" }
            cx.props.icon_right.clone().and_then(std::convert::identity).map(|icon| rsx! {
                MatIcon {
                    style: "--mdc-icon-size: {cx.props.icon_size};",
                    class: "md-icon",
                    "{icon}"
                }
            })
        }
    }
}
