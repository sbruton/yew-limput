use std::rc::Rc;

use tracing::error;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

pub type LimitedInputFilter = dyn for<'a> Fn(&'a char) -> bool;

#[macro_export]
macro_rules! input_filter {
    ($filter:expr) => {
        Rc::new($filter) as Rc<$crate::LimitedInputFilter>
    };
}

#[derive(Properties)]
struct LimitedInputProps {
    input_type: &'static str,
    class: AttrValue,
    filter: Rc<LimitedInputFilter>,
    max_len: Option<usize>,
    on_max_len: Callback<String>,
}

impl PartialEq for LimitedInputProps {
    fn eq(&self, other: &Self) -> bool {
        self.input_type == other.input_type
            && self.class == other.class
            && Rc::ptr_eq(&self.filter, &other.filter)
            && self.max_len == other.max_len
    }
}

#[function_component]
fn LimitedInput(props: &LimitedInputProps) -> Html {
    let oninput = {
        let filter = props.filter.clone();
        let max_len = props.max_len.unwrap_or(usize::MAX);
        let on_max_len = props.on_max_len.clone();

        move |event: InputEvent| {
            let Some(target) = event.target() else {
                error!("input event has no target");
                return;
            };

            let Ok(input) = target.dyn_into::<HtmlInputElement>() else {
                error!("input event target is not an input element");
                return;
            };

            let value = input.value();

            let filtered_value: String = value
                .chars()
                .filter(|c| (filter)(c))
                .take(max_len)
                .collect();

            input.set_value(&filtered_value);

            if filtered_value.len() >= max_len {
                on_max_len.emit(filtered_value);
            }
        }
    };

    html! {
        <input
            type={props.input_type}
            class={&props.class}
            {oninput}
        />
    }
}

#[derive(Properties)]
pub struct LimitedTextInputProps {
    #[prop_or_default]
    pub class: AttrValue,
    pub filter: Rc<LimitedInputFilter>,
    pub max_len: Option<usize>,
    #[prop_or_else(|| Callback::noop())]
    pub on_max_len: Callback<String>,
}

impl PartialEq for LimitedTextInputProps {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class
            && Rc::ptr_eq(&self.filter, &other.filter)
            && self.max_len == other.max_len
    }
}

#[function_component]
pub fn LimitedTextInput(props: &LimitedTextInputProps) -> Html {
    let input_type = "text";
    let class = &props.class;
    let filter = props.filter.clone();
    let max_len = props.max_len;
    let on_max_len = props.on_max_len.clone();

    html! {
        <LimitedInput {input_type} {class} {filter} {max_len} {on_max_len} />
    }
}

#[derive(PartialEq, Properties)]
pub struct LimitedNumericInputProps {
    #[prop_or_default]
    pub class: AttrValue,
    pub max_len: Option<usize>,
    #[prop_or_else(|| Callback::noop())]
    pub on_max_len: Callback<String>,
}

#[function_component]
pub fn LimitedNumericInput(props: &LimitedNumericInputProps) -> Html {
    let class = &props.class;
    let filter = Rc::new(|c: &char| c.is_ascii_digit()) as Rc<LimitedInputFilter>;
    let max_len = props.max_len;
    let on_max_len = props.on_max_len.clone();

    html! {
        <LimitedTextInput {class} {filter} {max_len} {on_max_len} />
    }
}
