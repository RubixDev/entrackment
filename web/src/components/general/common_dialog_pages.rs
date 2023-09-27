use dioxus::prelude::*;
use material_dioxus::MatCircularProgress;

pub fn DialogLoadingPage(cx: Scope) -> Element {
    render! {
        MatCircularProgress { indeterminate: true }
        "loading..."
    }
}

#[inline_props]
pub fn DialogErrorPage<'a>(cx: Scope, error: &'a str) -> Element {
    render! { div { color: "var(--mdc-theme-error)", "{error}" } }
}
