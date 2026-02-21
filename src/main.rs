mod frontend;
mod logic;

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

use crate::frontend::app::App;

fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(move || {
        view! { <App /> }
    });

    Ok(())
}
