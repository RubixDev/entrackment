use dioxus::prelude::*;
use dioxus_free_icons::IconShape;
use sir::css;

/// Icon component Props
#[derive(Props)]
pub struct IconProps<'a> {
    /// The icon shape to use.
    pub icon: &'a dyn IconShape,
    /// The height of the `<svg>` element. Defaults to 1em.
    #[props(default = "1em")]
    pub height: &'a str,
    /// The width of the `<svg>` element. Defaults to 1em.
    #[props(default = "1em")]
    pub width: &'a str,
    /// The color to use for filling the icon. Defaults to "currentColor".
    #[props(default = "currentColor")]
    pub fill: &'a str,
    /// A class for the `<svg>` element.
    #[props(default = "")]
    pub class: &'a str,
    /// Additional styling for the `<svg>` element.
    #[props(default = "")]
    pub style: &'a str,
    /// An `onclick` event handler.
    #[props(default)]
    pub onclick: EventHandler<'a, MouseEvent>,
}

/// Icon component which generates SVG elements
pub fn Icon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element<'a> {
    let icon = css!(
        "
        pointer-events: none;
    "
    );
    render! {
        svg {
            stroke: "currentColor",
            stroke_width: 0,
            class: "{icon} {cx.props.class}",
            style: "{cx.props.style}",
            height: "{cx.props.height}",
            width: "{cx.props.width}",
            view_box: "{cx.props.icon.view_box()}",
            xmlns: "{cx.props.icon.xmlns()}",
            fill: "{cx.props.fill}",
            onclick: move |event| cx.props.onclick.call(event),
            cx.props.icon.child_elements()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MdMovieEdit;
impl IconShape for MdMovieEdit {
    fn view_box(&self) -> String {
        String::from("0 0 24 24")
    }

    fn xmlns(&self) -> String {
        String::from("http://www.w3.org/2000/svg")
    }

    fn child_elements(&self) -> LazyNodes {
        rsx! {
            g {
                path { d: "M4 10h18V6c0-1.1-.9-2-2-2h-3l2 4h-3l-2-4h-2l2 4h-3L9 4H7l2 4H6L4 4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h8v-2H4v-8z" }
            }
            g { polygon { points: "14,18.88 14,21 16.12,21 21.29,15.83 19.17,13.71" } }
            g {
                path { d: "m22.71 13-.71-.71c-.39-.39-1.02-.39-1.41 0l-.71.71L22 15.12l.71-.71c.39-.39.39-1.02 0-1.41z" }
            }
        }
    }
}
