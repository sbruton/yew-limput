use std::rc::Rc;

use yew::prelude::*;
use yew_limput::{LimitedInputFilter, LimitedTextInput};

fn main() {
    yew::Renderer::<Example>::new().render();
}

#[function_component]
fn Example() -> Html {
    let max_len = Some(6);
    let filter = Rc::new(|c: &char| c.is_uppercase()) as Rc<LimitedInputFilter>;

    html! {
        <div>
            <h1>{ "Uppercase-Only Input Example" }</h1>
            <LimitedTextInput {filter} {max_len} />
        </div>
    }
}
