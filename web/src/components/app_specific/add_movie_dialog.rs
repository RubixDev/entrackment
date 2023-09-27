use std::{collections::BTreeSet, time::Duration};

use chrono::NaiveDate;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_free_icons::icons::md_image_icons::{MdMovieCreation, MdMovieFilter};
use material_dioxus::{
    dialog::{ActionType, MatDialogAction},
    text_inputs::TextFieldType,
    MatButton, MatDialog, MatFab, MatTextField,
};
use schema::{Movie, MovieStub, Platform};

use crate::{
    api::{self, ApiResult},
    components::{
        app_specific::{MovieEditor, SearchCard, SearchMovieCard},
        general::{DialogErrorPage, DialogLoadingPage},
    },
};

#[derive(Debug, Clone, Copy)]
enum Page {
    Search,
    Loading,
    Error,
    Input,
}

#[inline_props]
pub fn AddMovieDialog(
    cx: Scope,
    open: UseState<bool>,
    create_tag_dialog_open: UseState<bool>,
) -> Element {
    let app_state = crate::use_app_state(cx);
    let page = use_state(cx, || Page::Search);

    // hooks for Page::Search
    let search = use_state(cx, String::new);
    let id_search = use_state(cx, String::new);
    // gets toggled every time the search should be rerun
    let trigger_search = use_state(cx, || false);
    let search_results = api::use_api_fetch::<Vec<MovieStub>>(
        cx,
        (trigger_search,),
        // TODO: url escape string
        format!("/tmdb/search?title={search}"),
    );

    // hooks for Page::Error
    let error = use_state(cx, String::new);

    // hooks for Page::Info
    let imdb_id = use_state(cx, || None);
    let tmdb_id = use_state(cx, || 0);
    let title = use_state(cx, String::new);
    let description = use_state(cx, String::new);
    let tags = use_state(cx, BTreeSet::new);
    let platforms = use_state(cx, BTreeSet::new);
    let poster = use_state(cx, || None);
    let release_date = use_state(cx, NaiveDate::default);
    let runtime = use_state(cx, || Duration::ZERO);
    let score = use_state(cx, || 0.);

    let onsubmit = move || {
        page.set(Page::Loading);
        let new_movie = Movie {
            imdb_id: *imdb_id.get(),
            tmdb_id: *tmdb_id.get(),
            title: title.get().clone(),
            description: description.get().clone(),
            ratings: Vec::new(),
            tags: tags.get().clone(),
            platforms: platforms.get().clone(),
            poster: poster.get().clone(),
            release_date: *release_date.get(),
            runtime: *runtime.get(),
            score: *score.get(),
        };
        let req = crate::CLIENT
            .post(format!("{0}/api/movie", &*crate::BASE_URL))
            .json(&new_movie);
        to_owned![page, error, open, app_state];
        async move {
            if let Err(err) = api::send_request(req, api::no_body).await {
                error.set(err.to_string());
                page.set(Page::Error);
            } else {
                open.set(false);
                page.set(Page::Search);
                app_state
                    .write()
                    .movies
                    .insert(new_movie.tmdb_id, new_movie);
            }
        }
    };

    let body = match page.get() {
        Page::Search => {
            render! {
                SearchPage {
                    search: search.clone(),
                    id_search: id_search.clone(),
                    trigger_search: trigger_search,
                    search_results: search_results,
                    onnext: move |id| {
                        if let Some(id) = id {
                            cx.spawn({
                                page.set(Page::Loading);
                                to_owned![page, error];
                                to_owned![
                                    imdb_id, tmdb_id, title, description, tags, platforms, poster,
                                    release_date, runtime, score
                                ];
                                async move {
                                    let res = api::fetch::<Movie>(format_args!("/tmdb/by_id?id={id}")).await;
                                    match res {
                                        Ok(movie) => {
                                            imdb_id.set(movie.imdb_id);
                                            tmdb_id.set(movie.tmdb_id);
                                            title.set(movie.title);
                                            description.set(movie.description);
                                            tags.set(movie.tags);
                                            platforms.set(movie.platforms.into_iter().collect());
                                            poster.set(movie.poster);
                                            release_date.set(movie.release_date);
                                            runtime.set(movie.runtime);
                                            score.set(movie.score);
                                            page.set(Page::Input);
                                        }
                                        Err(err) => {
                                            error.set(err.to_string());
                                            page.set(Page::Error);
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
        Page::Loading => render! { DialogLoadingPage {} },
        Page::Error => render! { DialogErrorPage { error: error.get() } },
        Page::Input => {
            render! {
                InputPage {
                    imdb_id: **imdb_id,
                    tmdb_id: **tmdb_id,
                    title: title.clone(),
                    description: description.clone(),
                    tags: tags.clone(),
                    platforms: platforms.clone(),
                    poster: poster.clone(),
                    release_date: release_date.clone(),
                    runtime: runtime.clone(),
                    score: **score,
                    onsubmit: move |_| cx.spawn(onsubmit()),
                    create_tag_dialog_open: create_tag_dialog_open.clone()
                }
            }
        }
    };

    // TODO: close button or more spacing around
    render! {
        MatDialog {
            open: **open,
            _onclosed: {
                to_owned![open];
                move |_| open.set(false)
            },
            heading: "Add Movie",
            style: if let Page::Input = page.get() {
                "--mdc-dialog-min-width: min(45rem, 100vw - 2rem)"
            } else {
                ""
            },
            body,
            MatDialogAction { action_type: ActionType::Secondary,
                MatButton {
                    label: "back",
                    disabled: matches!(page.get(), Page::Search | Page::Loading),
                    _onclick: {
                        to_owned![page];
                        move |_| page.set(Page::Search)
                    }
                }
            }
        }
    }
}

#[inline_props]
fn SearchPage<'a>(
    cx: Scope,
    onnext: EventHandler<'a, Option<String>>,
    search: UseState<String>,
    id_search: UseState<String>,
    trigger_search: &'a UseState<bool>,
    search_results: &'a UseFuture<ApiResult<Vec<MovieStub>>>,
) -> Element {
    let search_results = match search_results.value() {
        Some(Ok(results)) => rsx! {
            for stub in results {
                SearchMovieCard {
                    key: "{stub.tmdb_id}",
                    movie: stub,
                    onclick: move |_| onnext.call(Some(stub.tmdb_id.to_string()))
                }
            }
        },
        Some(Err(err)) => {
            rsx! { DialogErrorPage { error: "failed to fetch search results: {err}" } }
        }
        None => rsx! { DialogLoadingPage {} },
    };

    render! {
        div { display: "flex", flex_direction: "column", gap: "0.5rem",
            SearchCard { icon: &MdMovieFilter,
                span {
                    width: "100%",
                    onkeydown: move |event| {
                        if event.key() == Key::Enter {
                            trigger_search.set(!*trigger_search);
                        }
                    },
                    MatTextField {
                        field_type: TextFieldType::Search,
                        value: "{search}",
                        label: "Find by name",
                        style: "width: 100%;",
                        outlined: true,
                        icon: "search",
                        dialog_initial_focus: true,
                        _oninput: {
                            to_owned![search];
                            move |new_value| search.set(new_value)
                        }
                    }
                }
                span { onclick: move |_| trigger_search.set(!*trigger_search), MatFab { icon: "search", mini: true } }
            }
            SearchCard { icon: &MdMovieCreation,
                span {
                    width: "100%",
                    onkeydown: move |event| {
                        if event.key() == Key::Enter {
                            onnext.call(Some(id_search.get().clone()));
                        }
                    },
                    MatTextField {
                        field_type: TextFieldType::Search,
                        value: "{id_search}",
                        label: "Find by ID",
                        style: "width: 100%;",
                        outlined: true,
                        icon: "search",
                        _oninput: {
                            to_owned![id_search];
                            move |new_value| id_search.set(new_value)
                        }
                    }
                }
                span { onclick: move |_| onnext.call(Some(id_search.get().clone())),
                    MatFab { icon: "search", mini: true }
                }
            }
            // SearchCard {
            //     icon: &MdMovieEdit,
            //     b { "Manually Input Info" }
            //     span {
            //         onclick: move |_| onnext.call(None),
            //         MatButton { label: "open", raised: true }
            //     }
            // }
            search_results
        }
    }
}

#[inline_props]
fn InputPage<'a>(
    cx: Scope,
    onsubmit: EventHandler<'a, ()>,
    create_tag_dialog_open: UseState<bool>,
    #[props(!optional)] imdb_id: Option<u32>,
    tmdb_id: u64,
    title: UseState<String>,
    description: UseState<String>,
    tags: UseState<BTreeSet<u32>>,
    platforms: UseState<BTreeSet<Platform>>,
    poster: UseState<Option<String>>,
    release_date: UseState<NaiveDate>,
    runtime: UseState<Duration>,
    score: f64,
) -> Element {
    render! {
        MovieEditor {
            create_tag_dialog_open: create_tag_dialog_open.clone(),
            tmdb_id: *tmdb_id,
            imdb_id: *imdb_id,
            title: title.clone(),
            description: description.clone(),
            tags: tags.clone(),
            platforms: platforms.clone(),
            poster: poster.clone(),
            release_date: release_date.clone(),
            runtime: runtime.clone(),
            score: *score
        }
        span { slot: ActionType::Primary.as_str(), onclick: move |_| onsubmit.call(()), MatButton { label: "add movie" } }
    }
}
