use dioxus::prelude::*;
use dioxus_free_icons::IconShape;
use sir::css;

use crate::components::general::Icon;

#[derive(Props)]
pub struct RatingInputProps<'a> {
    #[props(default)]
    value: u8,
    #[props(default = true)]
    interactive: bool,
    #[props(default = "2rem")]
    height: &'a str,
    #[props(default)]
    onchange: EventHandler<'a, u8>,
    #[props(default = "")]
    class: &'a str,
    #[props(default = "")]
    style: &'a str,
}

pub fn RatingInput<'a>(cx: Scope<'a, RatingInputProps<'a>>) -> Element<'a> {
    let div = css!(
        "
        display: flex;
        filter: drop-shadow(0 0.125rem 0.0625rem #0003);
    "
    );

    render! {
        div { class: "{div} {cx.props.class}", style: "{cx.props.style}", gap: if cx.props.interactive { "0.25rem" } else { "0" },
            for i in 1..=10 {
                Icon {
                    style: if cx.props.interactive { "cursor: pointer; pointer-events: auto;" } else { "" },
                    width: "calc({cx.props.height} / 2)",
                    height: "{cx.props.height}",
                    fill: if i <= cx.props.value {
                        "var(--clr-yellow)"
                    } else {
                        "var(--mdc-theme-text-hint-on-background)"
                    },
                    icon: if i % 2 != 0 { &StarHalfLeft } else { &StarHalfRight },
                    onclick: move |_| cx.props.onchange.call(i)
                }
            }
        }
    }
}

struct StarHalfLeft;
impl IconShape for StarHalfLeft {
    fn view_box(&self) -> String {
        String::from("0 0 12 24")
    }

    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }

    fn child_elements(&self) -> LazyNodes {
        rsx! { path { d: "M12 2 9.19 8.63 2 9.24l5.46 4.73L5.82 21L12 17.27z" } }
    }
}

struct StarHalfRight;
impl IconShape for StarHalfRight {
    fn view_box(&self) -> String {
        String::from("12 0 12 24")
    }

    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }

    fn child_elements(&self) -> LazyNodes {
        rsx! { path { d: "M12 17.27L18.18 21l-1.64-7.03L22 9.24l-7.19-.61L12 2z" } }
    }
}
