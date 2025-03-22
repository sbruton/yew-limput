# Filtered Input Fields for Yew

## Example Usage

__General Text Input Constraints__
```rust
use yew::prelude::*;
use yew_limput::LimitedTextInput;

#[function_component]
fn Example() -> Html {
    let filter = Rc::new(|c: &char| c.is_uppercase()) as Rc<LimitedInputFilter>;

    html! {
        <div>
            <h1>{ "Uppercase-Only Input Example" }</h1>
            <LimitedTextInput {filter} max_len={12} />
        </div>
    }
}
```

__TOTP Code Input__

`LimitedNumericInput` is the same as using `LimitedTextInput` with the filter `|c: &char| c.is_ascii_digit()`

```rust
use yew::prelude::*;
use yew_limput::LimitedNumericInput;

#[function_component]
fn Example() -> Html {
    html! {
        <div>
            <h1>{ "Uppercase-Only Input Example" }</h1>
            <LimitedNumericInput class="totp" max_len={6} />
        </div>
    }
}

```