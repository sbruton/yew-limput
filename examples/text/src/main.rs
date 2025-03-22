use std::rc::Rc;

use yew::prelude::*;
use yew_limput::{LimitedInputFilter, LimitedTextInput};

fn main() {
    yew::Renderer::<Example>::new().render();
}

#[function_component]
fn Example() -> Html {
    let filter = Rc::new(|c: &char| c.is_ascii_digit()) as Rc<LimitedInputFilter>;
    let max_len = Some(6);

    html! {
        <div>
            <h1>{ "Limited Text Input Example" }</h1>
            <LimitedTextInput {filter} {max_len} />
        </div>
    }
}
