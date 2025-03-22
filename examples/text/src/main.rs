use std::rc::Rc;

use yew::prelude::*;
use yew_limput::{LimitedTextInput, input_filter};

fn main() {
    tracing_wasm::set_as_global_default();
    yew::Renderer::<Example>::new().render();
}

#[function_component]
fn Example() -> Html {
    let filter = input_filter!(|c: &char| c.is_uppercase());

    let on_max_len =
        Callback::from(|s: String| tracing::info!("Max length reached: value is {s:?}"));

    html! {
        <div>
            <h1>{ "Uppercase-Only Input Example" }</h1>
            <LimitedTextInput class="my-class" {filter} max_len={12} {on_max_len} />
        </div>
    }
}
