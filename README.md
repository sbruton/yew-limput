# Filtered Input Fields for Yew

## Example Usage

__General Text Input Constraints__
```rust
use std::rc::Rc;
use yew::prelude::*;
use yew_limput::{LimitedTextInput, input_filter};

#[function_component]
fn Example() -> Html {
    let filter = input_filter!(|c: &char| c.is_uppercase());
    html! { <LimitedTextInput {filter} max_len={12} /> }
}
```

__TOTP Code Input__

`LimitedNumericInput` is the same as using `LimitedTextInput` with the filter `|c: &char| c.is_ascii_digit()`

```rust
use std::rc::Rc;
use yew::prelude::*;
use yew_limput::LimitedNumericInput;

#[function_component]
fn Example() -> Html {
    html! { <LimitedNumericInput class="totp" max_len={6} /> }
}
```