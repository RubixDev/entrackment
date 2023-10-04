use dioxus::prelude::*;
use material_dioxus::{text_inputs::TextFieldType, MatFormfield, MatSwitch, MatTextField};
use schema::Movie;
use sir::css;

use crate::components::general::Card;

pub type FilterCallback = Box<dyn Fn(&&Movie) -> bool>;

#[inline_props]
pub fn FilterCard(cx: Scope, callback: UseState<FilterCallback>, max_show_count: UseState<usize>) -> Element {
    let show_seen = use_state(cx, || false);
    let show_unseen = use_state(cx, || true);
    let search = use_state(cx, String::new);

    let card_css = css!(
        "
        display: flex;
        gap: 1.5rem;
        flex-direction: column;

        h3 {
            margin: 0;
        }
    "
    );

    let content = render! {
        h3 {
            onmounted: move |_| {
                callback
                    .set({
                        to_owned![show_seen, show_unseen, search];
                        Box::new(move |m| {
                            ((*show_seen.current() && !m.ratings.is_empty())
                                || (*show_unseen.current() && m.ratings.is_empty()))
                                && m.title.to_lowercase().contains(&search.current().to_lowercase())
                        })
                    })
            },
            "Filter"
        }
        MatFormfield { label: "Show seen",
            MatSwitch {
                selected: **show_seen,
                _onclick: {
                    to_owned![show_seen, max_show_count];
                    move |_| {
                        show_seen.set(!show_seen.get());
                        max_show_count.set(3);
                    }
                }
            }
        }
        MatFormfield { label: "Show unseen",
            MatSwitch {
                selected: **show_unseen,
                _onclick: {
                    to_owned![show_unseen, max_show_count];
                    move |_| {
                        show_unseen.set(!show_unseen.get());
                        max_show_count.set(3);
                    }
                }
            }
        }
        MatTextField {
            label: "Search",
            value: "{search}",
            icon: "search",
            outlined: true,
            field_type: TextFieldType::Search,
            _oninput: {
                to_owned![search, max_show_count];
                move |new_value| {
                    search.set(new_value);
                    max_show_count.set(3);
                }
            }
        }
    };

    render! {
        Card { class: "{card_css} elevation-4", content }
    }
}
