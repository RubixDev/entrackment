use std::marker::PhantomData;

use chrono::NaiveDate;
use dioxus::prelude::*;
use material_dioxus::{text_inputs::TextFieldType, MatTextField, StaticCallback};

#[derive(Props)]
pub struct DateInputProps<'a> {
    #[props(default = "Date")]
    label: &'a str,
    #[props(default)]
    value: NaiveDate,
    #[props(into)]
    _onchange: Option<StaticCallback<NaiveDate>>,
    _lifetime: Option<PhantomData<&'a ()>>,
}

pub fn DateInput<'a>(cx: Scope<'a, DateInputProps<'a>>) -> Element<'a> {
    if let Some(callback) = cx.props._onchange.clone() {
        render! {
            MatTextField {
                label: "{cx.props.label}",
                field_type: TextFieldType::Date,
                value: r#"{cx.props.value.format("%Y-%m-%d")}"#,
                webkit_date_picker: true,
                icon: "event",
                _oninput: move |new_value: String| {
                    callback
                        .call(
                            NaiveDate::parse_from_str(&new_value, "%Y-%m-%d")
                                .expect("invalid date from date input"),
                        )
                }
            }
        }
    } else {
        render! {
            MatTextField {
                label: "{cx.props.label}",
                field_type: TextFieldType::Date,
                value: r#"{cx.props.value.format("%Y-%m-%d")}"#,
                webkit_date_picker: true,
                icon: "event"
            }
        }
    }
}
