use std::borrow::Cow;

use dioxus::prelude::*;
use dioxus_free_icons::icons::md_action_icons::MdLabelOutline;
use material_dioxus::{
    list::ListIndex, MatButton, MatIcon, MatList, MatRadioListItem, MatTextField,
};
use schema::Color;

use crate::components::general::{Chip, ColorInput};

const ICONS: &[&str] = &["people", "reset_tv", "question_mark"];

#[inline_props]
pub fn TagEditor(
    cx: Scope,
    id: u32,
    name: UseState<String>,
    color: UseState<Color>,
    icon: UseState<Option<Cow<'static, str>>>,
) -> Element {
    render! {
        Chip {
            label: "{name}",
            color: **color,
            icon_left: &MdLabelOutline,
            icon_right: icon.get().clone(),
            icon_size: "1.3em"
        }
        div { display: "grid", gap: "0.5rem", grid_template_columns: "1fr 2fr", margin_top: "1rem",

            span { "Name:" }
            MatTextField {
                label: "Name",
                value: "{name}",
                _oninput: {
                    to_owned![name];
                    move |new_value| name.set(new_value)
                }
            }

            span { "Color:" }
            div { class: "spaced-list",
                ColorInput { value: **color, onchange: move |new_color| color.set(new_color) }
                MatButton {
                    label: "randomize",
                    icon: "casino",
                    _onclick: {
                        to_owned![color];
                        move |_| color.set(rand::random())
                    }
                }
            }

            span { "Icon:" }
            MatList {
                _onaction: {
                    to_owned![icon];
                    move |event: ListIndex| {
                        icon
                            .set(
                                event
                                    .unwrap_single()
                                    .filter(|idx| *idx != 0)
                                    .map(|idx| ICONS[idx - 1].into()),
                            )
                    }
                },
                MatRadioListItem {
                    key: "{possible_icon}",
                    left: true,
                    group: "icon-{id}",
                    initially_selected: icon.is_none(),
                    "none"
                }
                for possible_icon in ICONS {
                    MatRadioListItem {
                        key: "{possible_icon}",
                        left: true,
                        group: "icon-{id}",
                        initially_selected: icon.as_ref().is_some_and(|i| i == possible_icon),
                        MatIcon { "{possible_icon}" }
                    }
                }
            }
        }
    }
}
