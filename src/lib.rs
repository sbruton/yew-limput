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
    input_ref: NodeRef,
    class: AttrValue,
    id: AttrValue,
    name: AttrValue,
    autocomplete: bool,
    append_only: bool,
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
        let append_only = props.append_only;
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

            if append_only {
                let len = filtered_value.len() as u32;
                if let Err(err) = input.set_selection_range(len, len) {
                    error!("failed to set selection range: {:?}", err);
                }
            }

            if filtered_value.len() >= max_len {
                on_max_len.emit(filtered_value);
            }
        }
    };

    let input_ref = &props.input_ref;
    let id = &props.id;
    let name = &props.name;
    let autocomplete = if props.autocomplete { "on" } else { "off" };

    html! {
        <input
            ref={input_ref}
            {id}
            {name}
            {autocomplete}
            type={props.input_type}
            class={&props.class}
            {oninput}
        />
    }
}

#[derive(Properties)]
pub struct LimitedTextInputProps {
    pub input_ref: Option<NodeRef>,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or(true)]
    pub autocomplete: bool,
    #[prop_or(false)]
    pub append_only: bool,
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
    let input_ref = props.input_ref.clone().unwrap_or_default();
    let class = &props.class;
    let id = &props.id;
    let name = &props.name;
    let autocomplete = props.autocomplete;
    let append_only = props.append_only;
    let filter = props.filter.clone();
    let max_len = props.max_len;
    let on_max_len = props.on_max_len.clone();

    html! {
        <LimitedInput
            {input_type}
            {input_ref}
            {class}
            {id}
            {name}
            {autocomplete}
            {append_only}
            {filter}
            {max_len}
            {on_max_len}
        />
    }
}

#[derive(PartialEq, Properties)]
pub struct LimitedNumericInputProps {
    pub input_ref: Option<NodeRef>,
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or(true)]
    pub autocomplete: bool,
    #[prop_or(false)]
    pub append_only: bool,
    pub max_len: Option<usize>,
    #[prop_or_else(|| Callback::noop())]
    pub on_max_len: Callback<String>,
}

#[function_component]
pub fn LimitedNumericInput(props: &LimitedNumericInputProps) -> Html {
    let input_ref = props.input_ref.clone().unwrap_or_default();
    let class = &props.class;
    let id = &props.id;
    let name = &props.name;
    let autocomplete = props.autocomplete;
    let append_only = props.append_only;
    let filter = Rc::new(|c: &char| c.is_ascii_digit()) as Rc<LimitedInputFilter>;
    let max_len = props.max_len;
    let on_max_len = props.on_max_len.clone();

    html! {
        <LimitedTextInput
            {input_ref}
            {class}
            {id}
            {name}
            {autocomplete}
            {append_only}
            {filter}
            {max_len}
            {on_max_len}
            />
    }
}
