use std::{marker::PhantomData, time::Duration};

use dioxus::prelude::*;
use material_dioxus::{MatTextField, StaticCallback};

#[derive(Props)]
pub struct DurationInputProps<'a> {
    label: &'a str,
    #[props(default)]
    value: Duration,
    #[props(into)]
    _onchange: Option<StaticCallback<Duration>>,
    _lifetime: Option<PhantomData<&'a ()>>,
}

/// Uses a slightly modified version of [html-duration-picker](https://nadchif.github.io/html-duration-picker.js/)
pub fn DurationInput<'a>(cx: Scope<'a, DurationInputProps<'a>>) -> Element<'a> {
    let create_eval = use_eval(cx);

    let hours = cx.props.value.as_secs() / 3600;
    let minutes = (cx.props.value.as_secs() % 3600) / 60;

    if let Some(callback) = cx.props._onchange.clone() {
        render! {
            pre {
                hidden: true,
                onmounted: move |_| {
                    create_eval("setTimeout(() => { HtmlDurationPicker.refresh() }, 100)").unwrap();
                }
            }
            MatTextField {
                label: cx.props.label,
                class: "html-duration-picker",
                value: "{hours:02}:{minutes:02}",
                icon: "schedule",
                _onchange: move |new_value: String| {
                    let (hours, minutes) = new_value.split_once(':').unwrap();
                    let hours: u64 = hours.parse().unwrap();
                    let minutes: u64 = minutes.parse().unwrap();
                    callback.call(Duration::from_secs(hours * 3600 + minutes * 60))
                }
            }
        }
    } else {
        render! {
            pre {
                hidden: true,
                onmounted: move |_| {
                    create_eval("setTimeout(() => { HtmlDurationPicker.refresh() }, 100)").unwrap();
                }
            }
            MatTextField {
                label: cx.props.label,
                class: "html-duration-picker",
                value: "{hours:02}:{minutes:02}",
                icon: "schedule"
            }
        }
    }
}
