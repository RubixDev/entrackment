use dioxus::prelude::*;
use sir::css;

#[derive(Props)]
pub struct CardProps<'a> {
    children: Element<'a>,
    #[props(default = "")]
    class: &'a str,
    #[props(default = "")]
    style: &'a str,
    #[props(default)]
    onclick: EventHandler<'a, MouseEvent>,
}

pub fn Card<'a>(cx: Scope<'a, CardProps<'a>>) -> Element<'a> {
    let card = css!(
        "
        border-radius: 0.25rem;
        background-color: var(--clr-bg-card);
        flex-grow: 0;
        flex-shrink: 0;
        padding: 1rem;
    "
    );

    render! {
        div {
            class: "{card} {cx.props.class}",
            style: "{cx.props.style}",
            onclick: move |event| cx.props.onclick.call(event),
            &cx.props.children
        }
    }
}
