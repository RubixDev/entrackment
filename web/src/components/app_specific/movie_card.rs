use std::rc::Rc;

use chrono::Local;
use dioxus::prelude::*;
use dioxus_free_icons::icons::{
    md_action_icons::MdLabelOutline, md_image_icons::MdImageNotSupported,
};
use material_dioxus::{
    dialog::{ActionType, MatDialogAction},
    MatButton, MatCircularProgress, MatDialog,
};
use schema::{Movie, Rating, Tag};
use sir::css;

use crate::{
    api,
    components::{
        app_specific::{MovieEditor, PlatformChip},
        general::{Card, Chip, DateInput, DialogErrorPage, DialogLoadingPage, Icon, RatingInput},
    },
};

#[inline_props]
pub fn MovieCard(cx: Scope, movie: Movie, create_tag_dialog_open: UseState<bool>) -> Element {
    let Movie {
        imdb_id,
        tmdb_id,
        title,
        description,
        ratings,
        tags: movie_tags,
        platforms,
        poster,
        release_date,
        runtime,
        score,
    } = movie;
    let app_state = crate::use_app_state(cx);
    let small = use_state(cx, || true);
    let add_watch_dialog_open = use_state(cx, || false);
    let edit_dialog_open = use_state(cx, || false);

    let card_css = css!(
        "
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        gap: 1rem;
        min-height: var(--poster-height);
        position: relative;
        isolation: isolate;
        overflow: hidden;

        --poster-aspect-ratio: 185 / 278;
        --poster-height: 12rem;
        --poster-width: calc(var(--poster-aspect-ratio) * var(--poster-height));

        &.expanded {
            --poster-height: 15rem;

            .top-grid {
                grid-template-columns: 1fr 2fr;
            }
        }

        .img {
            height: var(--poster-height);
            width: var(--poster-width);
            border-radius: 0.5rem;
        }

        .top-grid {
            display: grid;
            gap: 1rem;
            grid-template-columns: auto 1fr;
        }

        .bottom-grid {
            display: grid;
            grid-template-columns: 1fr 2fr;
            gap: 0.25rem;
            justify-items: start;

            & > * {
                min-height: 1.5rem;
            }
        }

        @media screen and (max-width: 30rem) {
            .top-grid {
                grid-template-columns: 1fr !important;
                .img { order: 1 }
            }

            &:not(.expanded) {
                // &.bg-poster {
                //     background-image: var(--poster-url);
                //     background-origin: border-box;
                //     background-clip: border-box;
                //     background-repeat: repeat-y;
                //     background-size: 100vw auto;
                //     background-attachment: fixed;
                //     background-position-x: center;
                //
                //     &::before {
                //         content: '';
                //         position: absolute;
                //         inset: 0;
                //         z-index: -1;
                //         background-color: var(--clr-bg);
                //         opacity: 80%;
                //     }
                // }

                .img {
                    display: none !important;
                }
            }
        }
    "
    );
    let expanded_class = if !small { " expanded" } else { "" };
    let bg_poster_class = if poster.is_some() { " bg-poster" } else { "" };
    let small_grid_css = css!(
        "
        display: flex;
        gap: 0.5rem;
        flex-direction: column;
        align-items: flex-start;

        & > * {
            min-height: 1.25rem;
        }
    "
    );

    let poster_url: Rc<str> = poster
        .as_ref()
        .map(|path| format!("{0}/api/posters/big{path}", &*crate::BASE_URL))
        .unwrap_or_default()
        .into();
    let poster = match poster {
        Some(_) => {
            to_owned![poster_url];
            rsx! {
                img {
                    src: "{poster_url}",
                    class: "img{expanded_class}",
                    "loading": "lazy",
                }
            }
        }
        None => rsx! {
            div {
                class: "img{expanded_class}",
                background_color: "var(--clr-bg-img)",
                display: "grid",
                align_items: "center",
                justify_content: "center",
                Icon {
                    class: "md-icon",
                    height: "3.5rem",
                    width: "3.5rem",
                    fill: "var(--clr-bg-card)",
                    icon: &MdImageNotSupported,
                }
            }
        },
    };

    let tags = movie_tags.iter().map(|id| {
        let tag = app_state
            .read()
            .tags
            .get(id)
            .cloned()
            .unwrap_or_else(|| Tag {
                id: 0,
                name: "Unknown".into(),
                color: (50, 50, 50),
                icon: Some("question_mark".into()),
            });
        let chip = rsx! {
            Chip {
                key: "{id}",
                label: "{tag.name}",
                color: tag.color,
                icon_left: &MdLabelOutline,
                icon_right: tag.icon,
                icon_size: "1.3em"
            }
        };
        chip
    });
    let tags = rsx! {
        span {
            class: "spaced-list",
            tags
            if movie_tags.is_empty() {
                rsx! { "no tags" }
            }
        }
    };

    let platforms = rsx! {
        span {
            class: "spaced-list",
            for platform in platforms {
                PlatformChip {
                    key: "{platform}",
                    platform: *platform,
                }
            }
            if platforms.is_empty() {
                rsx! { "no platforms" }
            }
        }
    };

    let release_date = rsx! { span { release_date.format("%d.%m.%Y").to_string() } };
    let runtime =
        rsx! { span { humantime::format_duration(*runtime).to_string().replace(' ', "\u{a0}") } };

    let rating = ratings
        .last()
        .map(|w| {
            rsx! {
                RatingInput {
                    interactive: false,
                    height: "1.5rem",
                    style: "display: inline-flex;",
                    value: w.rating
                }
                span { class: "hint", r#"({w.date.format("%d.%m.%Y")})"# }
            }
        })
        .unwrap_or_else(|| rsx! { "no ratings" });
    let rating = rsx! {
        span {
            class: "spaced-list",
            rating,
            if !small {
                rsx! {
                    span {
                        onclick: move |_| add_watch_dialog_open.set(true),
                        MatButton {
                            label: "details",
                            dense: true,
                        }
                    }
                }
            }
        }
    };

    let bottom_buttons = rsx! {
        span {
            class: "spaced-list",
            span {
                onclick: move |_| small.set(!small),
                MatButton {
                    label: if !small { "minimize" } else { "expand" },
                    icon: if !small { "expand_less" } else { "expand_more" },
                    trailing_icon: true,
                    outlined: true,
                }
            }
            span {
                onclick: move |_| edit_dialog_open.set(true),
                MatButton {
                    label: "edit",
                    icon: "edit",
                    trailing_icon: true,
                    outlined: true,
                }
            }
        }
    };

    let (next_to_poster, below_poster) = if !small {
        (
            rsx! {
                div {
                    h3 { "{title}" }
                    p { "{description}" }
                }
            },
            Some(rsx! {
                div { class: "bottom-grid",
                    span { class: "hint", "Tags:" }
                    tags

                    span { class: "hint", "Platforms:" }
                    platforms

                    span { class: "hint", "Release Date:" }
                    release_date

                    span { class: "hint", "Runtime:" }
                    runtime

                    span { class: "hint", "Personal Rating:" }
                    rating

                    span { class: "hint", "TMDB Score:" }
                    span {
                        class: "spaced-list",
                        MatCircularProgress { progress: (score / 10.) as f32, density: -4 }
                        format_args!("{:.2}%", score * 10.)
                    }

                    span { class: "hint", "Links:" }
                    span {
                        class: "spaced-list",
                        imdb_id.map(|id| rsx! {
                            a {
                                target: "_blank",
                                href: "https://www.imdb.com/title/tt{id:07}/",
                                MatButton { label: "IMDb", icon: "open_in_new", trailing_icon: true }
                            }
                        }),
                        a {
                            target: "_blank",
                            href: "https://www.themoviedb.org/movie/{tmdb_id}",
                            MatButton { label: "TMDB", icon: "open_in_new", trailing_icon: true }
                        }
                    }
                }
                bottom_buttons
            }),
        )
    } else {
        (
            rsx! {
                div {
                    class: "{small_grid_css}",
                    h3 { margin: "0 0 0.5rem", "{title}" }
                    div { class: "hint", tags }
                    div { class: "hint", platforms }
                    div {
                        class: "hint",
                        display: "flex",
                        gap: "1.5rem",
                        release_date
                        runtime
                    }
                    div { class: "hint", rating }
                    bottom_buttons
                }
            },
            None,
        )
    };

    render! {
        RatingsInfoDialog { open: add_watch_dialog_open.clone(), movie_id: *tmdb_id, ratings: ratings }
        EditDialog { open: edit_dialog_open.clone(), movie: movie, create_tag_dialog_open: create_tag_dialog_open.clone() }

        Card {
            style: "--poster-url: url('{poster_url}');",
            class: "{card_css}{expanded_class}{bg_poster_class} elevation-4",
            div {
                class: "top-grid",
                poster
                next_to_poster
            }
            below_poster
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RatingsInfoDialogPage {
    List,
    Input,
    Loading,
    Error,
}

#[inline_props]
fn RatingsInfoDialog<'a>(
    cx: Scope,
    open: UseState<bool>,
    movie_id: u64,
    ratings: &'a [Rating],
) -> Element {
    use RatingsInfoDialogPage as Page;

    let page = use_state(cx, || Page::List);
    let app_state = crate::use_app_state(cx);

    // hooks for Page::Input
    let date = use_state(cx, || Local::now().date_naive());
    let rating = use_state(cx, || 6);

    // hooks for Page::Error
    let error = use_state(cx, String::new);

    let onsubmit = move || {
        page.set(Page::Loading);
        let new_rating = Rating {
            rating: *rating.get(),
            date: *date.get(),
        };
        let req = crate::CLIENT
            .put(format!(
                "{0}/api/movie/{movie_id}/rating",
                &*crate::BASE_URL
            ))
            .json(&new_rating);
        to_owned![page, error, app_state, movie_id];
        async move {
            if let Err(err) = api::send_request(req, api::no_body).await {
                error.set(err.to_string());
                page.set(Page::Error);
            } else {
                page.set(Page::List);
                let insert_index = app_state.read().movies[&movie_id]
                    .ratings
                    .binary_search_by_key(&new_rating.date, |w| w.date)
                    .expect_err("backend should error on conflicting ratings");
                app_state
                    .write()
                    .movies
                    .get_mut(&movie_id)
                    .expect("movie should exist")
                    .ratings
                    .insert(insert_index, new_rating);
            }
        }
    };
    let average =
        ratings.iter().map(|r| r.rating as u16).sum::<u16>() as f32 / ratings.len() as f32;

    let body = match page.get() {
        Page::List => render! {
            div { display: "grid", grid_template_columns: "1fr 2fr", gap: "0.5rem",

                span { "Average:" }
                div {
                    if !ratings.is_empty() {
                        rsx! {
                            RatingInput {
                                interactive: false,
                                height: "1.3rem",
                                style: "display: inline-flex;",
                                value: average.round() as u8,
                            }
                            "({average:.2}\u{a0}/\u{a0}10)"
                        }
                    } else {
                        rsx! { "N/A" }
                    }
                }
                for rating in ratings.iter().rev() {
                    span { key: "{rating.date}", rating.date.format("%d.%m.%Y:").to_string() }
                    RatingInput { key: "{rating.date}", interactive: false, height: "1rem", value: rating.rating }
                }
            }
            span {
                onclick: move |_| page.set(Page::Input),
                MatButton { label: "add new", icon: "add", outlined: true }
            }
        },
        Page::Input => render! {
            div { display: "grid", gap: "0.5rem", grid_template_columns: "1fr 2fr",
                span { "Date:" }
                DateInput {
                    value: **date,
                    _onchange: {
                        to_owned![date];
                        move |d| date.set(d)
                    }
                }

                span { "Rating:" }
                RatingInput { value: *rating.get(), onchange: move |r| rating.set(r) }
            }
            span {
                slot: ActionType::Primary.as_str(),
                onclick: move |_| cx.spawn(onsubmit()),
                MatButton { label: "add" }
            }
        },
        Page::Loading => render! { DialogLoadingPage {} },
        Page::Error => render! { DialogErrorPage { error: error.get() } },
    };

    render! {
        MatDialog {
            open: **open,
            _onclosed: {
                to_owned![open];
                move |_| open.set(false)
            },
            heading: "Personal Movie Ratings",
            body
            MatDialogAction {
                action_type: ActionType::Secondary,
                MatButton {
                    label: "back",
                    disabled: matches!(page.get(), Page::List | Page::Loading),
                    _onclick: {
                        to_owned![page];
                        move |_| {
                            if let Page::Input = page.get() {
                                page.set(Page::List);
                            } else {
                                page.set(Page::Input);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EditDialogPage {
    Input,
    Loading,
    Error,
}

#[inline_props]
fn EditDialog<'a>(
    cx: Scope,
    open: UseState<bool>,
    movie: &'a Movie,
    create_tag_dialog_open: UseState<bool>,
) -> Element {
    use EditDialogPage as Page;

    let page = use_state(cx, || Page::Input);
    let app_state = crate::use_app_state(cx);

    // hooks for Page::Input
    let tmdb_id = cx.use_hook(|| movie.tmdb_id);
    let title = use_state(cx, || movie.title.clone());
    let description = use_state(cx, || movie.description.clone());
    let tags = use_state(cx, || movie.tags.clone());
    let platforms = use_state(cx, || movie.platforms.clone());
    let poster = use_state(cx, || movie.poster.clone());
    let release_date = use_state(cx, || movie.release_date);
    let runtime = use_state(cx, || movie.runtime);
    // make sure to reset the states in case the dialog's movie changed
    if *tmdb_id != movie.tmdb_id {
        *tmdb_id = movie.tmdb_id;
        title.set(movie.title.clone());
        description.set(movie.description.clone());
        tags.set(movie.tags.clone());
        platforms.set(movie.platforms.clone());
        poster.set(movie.poster.clone());
        release_date.set(movie.release_date);
        runtime.set(movie.runtime);
    }

    // hooks for Page::Error
    let error = use_state(cx, String::new);

    let onsubmit = move || {
        page.set(Page::Loading);
        let new_movie = Movie {
            imdb_id: movie.imdb_id,
            tmdb_id: movie.tmdb_id,
            title: title.get().clone(),
            description: description.get().clone(),
            ratings: movie.ratings.clone(),
            tags: tags.get().clone(),
            platforms: platforms.get().clone(),
            poster: poster.get().clone(),
            release_date: *release_date.get(),
            runtime: *runtime.get(),
            score: movie.score,
        };
        let req = crate::CLIENT
            .patch(format!("{0}/api/movie", &*crate::BASE_URL))
            .json(&new_movie);
        to_owned![page, error, open, app_state];
        async move {
            if let Err(err) = api::send_request(req, api::no_body).await {
                error.set(err.to_string());
                page.set(Page::Error);
            } else {
                open.set(false);
                app_state
                    .write()
                    .movies
                    .insert(new_movie.tmdb_id, new_movie);
            }
        }
    };
    let ondelete = move || {
        page.set(Page::Loading);
        let id = movie.tmdb_id;
        let req = crate::CLIENT.delete(format!("{0}/api/movie/{id}", &*crate::BASE_URL));
        to_owned![page, error, open, app_state];
        async move {
            if let Err(err) = api::send_request(req, api::no_body).await {
                error.set(err.to_string());
                page.set(Page::Error);
            } else {
                open.set(false);
                app_state.write().movies.remove(&id);
            }
        }
    };

    let body = match page.get() {
        Page::Input => rsx! {
            MovieEditor {
                ondelete: move |_| cx.spawn(ondelete()),
                create_tag_dialog_open: create_tag_dialog_open.clone(),
                imdb_id: movie.imdb_id,
                tmdb_id: movie.tmdb_id,
                title: title.clone(),
                description: description.clone(),
                tags: tags.clone(),
                platforms: platforms.clone(),
                poster: poster.clone(),
                release_date: release_date.clone(),
                runtime: runtime.clone(),
                score: movie.score,
            }
            span {
                slot: ActionType::Primary.as_str(),
                onclick: move |_| cx.spawn(onsubmit()),
                MatButton { label: "submit" }
            }
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
                    page.set(Page::Input);
                }
            },
            heading: "Edit Movie \"{movie.title}\"",
            body
            MatDialogAction {
                action_type: ActionType::Secondary,
                MatButton {
                    label: "back",
                    disabled: matches!(page.get(), Page::Input | Page::Loading),
                    _onclick: {
                        to_owned![page];
                        move |_| page.set(Page::Input)
                    }
                }
            }
        }
    }
}
