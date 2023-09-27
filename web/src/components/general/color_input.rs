use dioxus::prelude::*;
use material_dioxus::{palette, theming, MatIcon};
use schema::Color;
use sir::css;

#[derive(Props)]
pub struct ColorInputProps<'a> {
    #[props(default = "colorize")]
    icon: &'a str,
    #[props(default)]
    value: Color,
    #[props(default = "36px")]
    size: &'a str,
    #[props(default)]
    onchange: EventHandler<'a, Color>,
}

pub fn ColorInput<'a>(cx: Scope<'a, ColorInputProps<'a>>) -> Element<'a> {
    let picker_css = css!(
        "
        width: var(--size);
        height: var(--size);
        position: relative;

        input {
            background: none;
            border: none;
            padding: 0;
            cursor: pointer;
            width: 100%;
            height: 100%;

            &::-webkit-color-swatch-wrapper {
                padding: 0;
            }

            &::-webkit-color-swatch {
                border: none;
                border-radius: var(--mdc-shape-small, 4px);
            }
            &::-moz-color-swatch {
                border: none;
                border-radius: var(--mdc-shape-small, 4px);
            }
        }

        .icon {
            position: absolute;
            --mdc-icon-size: calc(var(--size) - 12px);
            color: var(--icon-color);
            top: 50%;
            left: 50%;
            transform: translate(-50%, -50%);
            pointer-events: none;
        }
    "
    );
    let (r, g, b) = cx.props.value;
    let (icon_r, icon_g, icon_b, _) =
        palette::with_alpha(theming::contrast_text(palette::Color::new(r, g, b, 1.)), 1.)
            .into_components();

    render! {
        span {
            class: "{picker_css}",
            style: "--size: {cx.props.size}; --icon: '{cx.props.icon}'; --icon-color: rgb({icon_r}, {icon_g}, {icon_b});",
            MatIcon { class: "icon", "{cx.props.icon}" }
            input {
                r#type: "color",
                value: "#{r:02x}{g:02x}{b:02x}",
                oninput: move |event| {
                    cx.props.onchange.call({
                        let r = u8::from_str_radix(&event.value[1..3], 16)
                            .expect("color input type should return valid hex color");
                        let g = u8::from_str_radix(&event.value[3..5], 16)
                            .expect("color input type should return valid hex color");
                        let b = u8::from_str_radix(&event.value[5..7], 16)
                            .expect("color input type should return valid hex color");
                        (r, g, b)
                    })
                },
            }
        }
    }
}
