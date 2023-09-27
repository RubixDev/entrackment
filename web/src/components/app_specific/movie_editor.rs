use std::{collections::BTreeSet, rc::Rc, time::Duration};

use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_free_icons::icons::{
    md_action_icons::MdLabelOutline, md_image_icons::MdImageNotSupported,
};
use itertools::Itertools;
use material_dioxus::{
    dialog::{ActionType, MatDialogAction},
    list::ListIndex,
    MatButton, MatCheckListItem, MatCircularProgress, MatDialog, MatIconButton, MatList,
    MatTextArea, MatTextField,
};
use schema::{Platform, Tag};
use sir::css;
use strum::IntoEnumIterator;

use crate::{
    api,
    components::{
        app_specific::{PlatformChip, TagEditor},
        general::{Chip, DateInput, DialogErrorPage, DialogLoadingPage, DurationInput, Icon},
    },
};

#[derive(Props, PartialEq)]
pub struct MovieEditorProps {
    create_tag_dialog_open: UseState<bool>,
    #[props(!optional)]
    imdb_id: Option<u32>,
    tmdb_id: u64,
    title: UseState<String>,
    description: UseState<String>,
    tags: UseState<BTreeSet<u32>>,
    platforms: UseState<BTreeSet<Platform>>,
    poster: UseState<Option<String>>,
    release_date: UseState<NaiveDate>,
    runtime: UseState<Duration>,
    score: f64,
}

pub fn MovieEditor(cx: Scope<MovieEditorProps>) -> Element {
    let MovieEditorProps {
        create_tag_dialog_open,
        imdb_id,
        tmdb_id,
        title,
        description,
        tags,
        platforms,
        poster,
        release_date,
        runtime,
        score,
    } = cx.props;

    let app_state = crate::use_app_state(cx);
    let mut tag_list = app_state.read().tags.values().cloned().collect_vec();
    tag_list.sort_unstable_by_key(|t| t.name.clone());
    let tag_list: Rc<[Tag]> = tag_list.into();

    let img = css!(
        "
        width: 185px;
        height: 278px;
    "
    );

    let poster = match poster.get() {
        Some(path) => {
            rsx! { img { class: "{img}", src: "{&*crate::BASE_URL}/api/posters/big{path}" } }
        }
        None => rsx! {
            div {
                class: "{img}",
                background_color: "var(--clr-bg-img)",
                display: "grid",
                align_items: "center",
                justify_content: "center",
                Icon {
                    class: "md-icon",
                    height: "3.5rem",
                    width: "3.5rem",
                    fill: "var(--clr-bg-card)",
                    icon: &MdImageNotSupported
                }
            }
        },
    };

    render! {
        div { display: "flex", flex_wrap: "wrap", gap: "1rem", margin_bottom: "1rem",
            a { target: "_blank", href: "https://www.themoviedb.org/movie/{tmdb_id}",
                MatButton { label: "Open TMDB page", icon: "open_in_new", trailing_icon: true, outlined: true }
            }
            imdb_id.map(|id| rsx! {
                a {
                    target: "_blank",
                    href: "https://www.imdb.com/title/tt{id:07}/",
                    MatButton {
                        label: "Open IMDb page",
                        icon: "open_in_new",
                        trailing_icon: true,
                        outlined: true,
                    }
                }
            })
        }

        div { display: "grid", grid_template_columns: "1fr 2fr", gap: "0.5rem",

            span { "TMDB ID:" }
            span { "{tmdb_id}" }

            span { "IMDb ID:" }
            span { imdb_id.map(|id| format!("tt{id:07}")).unwrap_or_else(|| String::from("none")) }

            span { "TMDB Score:" }
            span { display: "flex", align_items: "center", gap: "1rem",
                MatCircularProgress { progress: (score / 10.) as f32, density: -4 }
                format_args!("{:.2}%", score * 10.)
            }

            span { "Title:" }
            MatTextField {
                label: "Title",
                value: "{title}",
                _oninput: {
                    to_owned![title];
                    move |new_value| title.set(new_value)
                }
            }

            span { "Description:" }
            MatTextArea {
                label: "Description",
                value: "{description}",
                style: "min-height: 15rem",
                _oninput: {
                    to_owned![description];
                    move |new_value| description.set(new_value)
                }
            }

            span { "Runtime:" }
            DurationInput {
                label: "Runtime",
                value: **runtime,
                _onchange: {
                    to_owned![runtime];
                    move |d| runtime.set(d)
                }
            }

            span { "Release date:" }
            DateInput {
                label: "Release date",
                value: **release_date,
                _onchange: {
                    to_owned![release_date];
                    move |d| release_date.set(d)
                }
            }

            span { "Platforms:" }
            MatList {
                multi: true,
                _onaction: {
                    to_owned![platforms];
                    move |event: ListIndex| {
                        platforms
                            .set(
                                event
                                    .unwrap_multi()
                                    .into_iter()
                                    .map(|idx| Platform::from_repr(idx as u8).unwrap())
                                    .collect(),
                            )
                    }
                },
                for platform in Platform::iter() {
                    MatCheckListItem { key: "{platform}", left: true, initially_selected: platforms.contains(&platform), PlatformChip { platform: platform } }
                }
            }

            span { "Tags:" }
            span {
                MatList {
                    multi: true,
                    _onaction: {
                        to_owned![tags, tag_list];
                        move |event: ListIndex| {
                            tags.set(event.unwrap_multi().into_iter().map(|idx| tag_list[idx].id).collect())
                        }
                    },
                    for tag in &tag_list {
                        TagListItem { key: "{tag.id}", tag: tag.clone(), selected: tags.contains(&tag.id) }
                    }
                }
                span { onclick: move |_| create_tag_dialog_open.set(true),
                    MatButton { label: "create new tag", icon: "add" }
                }
            }

            // TODO: poster edit
            span { "Poster:" }
            div { poster }
        }
    }
}

#[inline_props]
fn TagListItem(cx: Scope, tag: Tag, selected: bool) -> Element {
    let dialog_open = use_state(cx, || false);

    render! {
        EditTagDialog { open: dialog_open.clone(), tag: tag }
        MatCheckListItem { left: true, initially_selected: *selected,
            span { display: "flex", gap: "1rem", align_items: "center",
                Chip {
                    label: "{tag.name}",
                    color: tag.color,
                    icon_left: &MdLabelOutline,
                    icon_right: tag.icon.clone(),
                    icon_size: "1.3em"
                }
                span { onclick: move |_| dialog_open.set(true), MatIconButton { icon: "edit" } }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EditTagDialogPage {
    Edit,
    Loading,
    Error,
}

#[inline_props]
fn EditTagDialog<'a>(cx: Scope, open: UseState<bool>, tag: &'a Tag) -> Element {
    use EditTagDialogPage as Page;

    let page = use_state(cx, || Page::Edit);
    let app_state = crate::use_app_state(cx);

    // hooks for Page::Create
    let name = use_state(cx, || tag.name.clone());
    let color = use_state(cx, || tag.color);
    let icon = use_state(cx, || tag.icon.clone());

    // hooks for Page::Error
    let error = use_state(cx, String::new);

    let patch_tag = move || {
        page.set(Page::Loading);
        let new_tag = Tag {
            id: tag.id,
            name: name.get().clone(),
            color: *color.get(),
            icon: icon.get().clone(),
        };
        let req = crate::CLIENT
            .patch(format!("{0}/api/tag", &*crate::BASE_URL))
            .json(&new_tag);
        to_owned![page, error, app_state, open];
        async move {
            if let Err(err) = api::send_request(req, api::no_body).await {
                error.set(err.to_string());
                page.set(Page::Error);
            } else {
                open.set(false);
                app_state.write().tags.insert(new_tag.id, new_tag);
            }
        }
    };

    let body = match page.get() {
        Page::Edit => rsx! {
            TagEditor {
                id: tag.id,
                name: name.clone(),
                color: color.clone(),
                icon: icon.clone()
            }
            span { slot: ActionType::Primary.as_str(), onclick: move |_| cx.spawn(patch_tag()), MatButton { label: "submit" } }
        },
        Page::Loading => rsx! { DialogLoadingPage {} },
        Page::Error => rsx! { DialogErrorPage { error: error.get() } },
    };

    render! {
        MatDialog {
            open: **open,
            _onclosed: {
                to_owned![open, page];
                move |_| {
                    open.set(false);
                    page.set(Page::Edit);
                }
            },
            heading: "Edit Tag \"{tag.name}\"",
            body,
            MatDialogAction { action_type: ActionType::Secondary,
                MatButton {
                    label: "back",
                    disabled: matches!(page.get(), Page::Edit | Page::Loading),
                    _onclick: {
                        to_owned![page];
                        move |_| page.set(Page::Edit)
                    }
                }
            }
        }
    }
}
