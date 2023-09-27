use dioxus::prelude::*;
use dioxus_free_icons::{icons::md_image_icons::MdImageNotSupported, IconShape};
use schema::MovieStub;
use sir::css;

use crate::components::general::{Card, Icon};

fn card_css() -> &'static str {
    css!(
        "
        box-sizing: border-box;
        overflow: hidden;
        padding: 0.5rem 1rem 0.5rem calc(var(--poster-width) + 1rem) !important;
        background-color: var(--clr-bg-card) !important;
        --height: 5rem;
        height: var(--height);
        position: relative;
        line-height: 1.2rem;

        --poster-aspect-ratio: 92 / 138;
        --poster-width: calc(var(--poster-aspect-ratio) * var(--height));
    "
    )
}

fn img_css() -> &'static str {
    css!(
        "
        height: var(--height);
        width: var(--poster-width);
        position: absolute;
        top: 0;
        left: 0;
    "
    )
}

#[inline_props]
pub fn SearchMovieCard<'a>(
    cx: Scope,
    movie: &'a MovieStub,
    onclick: EventHandler<'a, MouseEvent>,
) -> Element {
    let MovieStub {
        tmdb_id: _,
        title,
        description,
        release_date,
        poster,
    } = movie;

    let title_css = css!(
        "
        overflow: hidden;
        display: -webkit-box;
        -webkit-line-clamp: 1;
        -webkit-box-orient: vertical;
    "
    );
    let description_css = css!(
        "
        overflow: hidden;
        display: -webkit-box;
        -webkit-line-clamp: 2;
        -webkit-box-orient: vertical;
    "
    );

    let poster = match poster {
        Some(path) => {
            rsx! {img {
                src: "{&*crate::BASE_URL}/api/posters/small{path}",
                class: "{img_css()}"
            }
            }
        }
        None => rsx! {
            div {
                class: "{img_css()}",
                background_color: "var(--clr-bg-img)",
                display: "grid",
                align_items: "center",
                justify_content: "center",
                Icon {
                    class: "md-icon",
                    height: "2.5rem",
                    width: "2.5rem",
                    fill: "var(--clr-bg-card)",
                    icon: &MdImageNotSupported
                }
            }
        },
    };
    let title_plus_year = format!("{title} {0}", release_date.format("(%Y)"));
    render! {
        Card {
            class: "{card_css()} elevation-2",
            style: "cursor: pointer;",
            onclick: move |event| onclick.call(event),
            poster,
            div { overflow: "hidden",
                div { class: "{title_css}", margin_bottom: "0.5rem", b { title: "{title_plus_year}", "{title_plus_year}" } }
                div { class: "{description_css}", "{description}" }
            }
        }
    }
}

#[inline_props]
pub fn SearchCard<'a>(cx: Scope, icon: &'a dyn IconShape, children: Element<'a>) -> Element {
    let poster = rsx! {
        div {
            class: "{img_css()}",
            background_color: "var(--mdc-theme-primary)",
            display: "grid",
            align_items: "center",
            justify_content: "center",
            Icon {
                class: "md-icon",
                height: "2.5rem",
                width: "2.5rem",
                fill: "var(--clr-bg-card)",
                icon: *icon
            }
        }
    };
    render! {
        Card { class: "{card_css()} elevation-2",
            poster,
            div { display: "flex", align_items: "center", height: "100%", gap: "1rem", children }
        }
    }
}
