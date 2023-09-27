use dioxus::prelude::*;
use material_dioxus::{
    dialog::{ActionType, MatDialogAction},
    MatButton, MatDialog,
};
use reqwest::Response;
use schema::{Color, Tag};

use crate::{
    api,
    components::{
        app_specific::TagEditor,
        general::{DialogErrorPage, DialogLoadingPage},
    },
};

#[derive(Debug, Clone, Copy)]
enum Page {
    Create,
    Loading,
    Error,
}

#[inline_props]
pub fn CreateTagDialog(cx: Scope, open: UseState<bool>) -> Element {
    let page = use_state(cx, || Page::Create);
    let app_state = crate::use_app_state(cx);

    // hooks for Page::Create
    let name = use_state(cx, || String::from("New Tag"));
    let color = use_state(cx, rand::random::<Color>);
    let icon = use_state(cx, || None);

    // hooks for Page::Error
    let error = use_state(cx, String::new);

    let create_tag = move || {
        page.set(Page::Loading);
        let new_tag = Tag {
            id: 0,
            name: name.get().clone(),
            color: *color.get(),
            icon: icon.get().clone(),
        };
        let req = crate::CLIENT
            .post(format!("{0}/api/tag", &*crate::BASE_URL))
            .json(&new_tag);
        to_owned![page, error, app_state, open];
        async move {
            match api::send_request(req, Response::json::<Tag>).await {
                Ok(new_tag) => {
                    open.set(false);
                    page.set(Page::Create);
                    app_state.write().tags.insert(new_tag.id, new_tag);
                }
                Err(err) => {
                    error.set(err.to_string());
                    page.set(Page::Error);
                }
            }
        }
    };

    let body = match page.get() {
        Page::Create => rsx! {
            TagEditor { id: 0, name: name.clone(), color: color.clone(), icon: icon.clone() }
            span { slot: ActionType::Primary.as_str(), onclick: move |_| cx.spawn(create_tag()), MatButton { label: "create" } }
        },
        Page::Loading => rsx! { DialogLoadingPage {} },
        Page::Error => rsx! { DialogErrorPage { error: error.get() } },
    };

    render! {
        MatDialog {
            open: **open,
            _onclosed: {
                to_owned![open];
                move |_| open.set(false)
            },
            heading: "Create Tag",
            body,
            MatDialogAction { action_type: ActionType::Secondary,
                MatButton {
                    label: "back",
                    disabled: matches!(page.get(), Page::Create | Page::Loading),
                    _onclick: {
                        to_owned![page];
                        move |_| page.set(Page::Create)
                    }
                }
            }
        }
    }
}
