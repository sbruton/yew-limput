use std::rc::Rc;

use yew::prelude::*;
use yew_limput::{LimitedTextInput, input_filter};

fn main() {
    yew::Renderer::<Example>::new().render();
}

#[function_component]
fn Example() -> Html {
    let filter = input_filter!(|c: &char| c.is_uppercase());

    html! {
        <div>
            <h1>{ "Uppercase-Only Input Example" }</h1>
            <LimitedTextInput class="my-class" {filter} max_len={12} />
        </div>
    }
}
