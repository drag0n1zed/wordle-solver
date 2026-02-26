use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::grid::Grid;

#[component]
fn NumberCounter(
    value: Signal<usize>,
    #[prop(into)] on_change: Callback<usize>,
    #[prop(default = 1)] min: usize,
) -> impl IntoView {
    view! {
      <div class="flex items-center border-2 border-black">
        <button
          class="px-3 py-1 font-bold bg-white hover:bg-black hover:text-white transition-colors select-none active:scale-95"
          on:click=move |e| {
            let v = value.get();
            if v > min {
              on_change.run(v - 1);
            }
            let _ = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().blur();
          }
        >
          "-"
        </button>
        <span class="w-8 text-center font-bold">{move || value.get()}</span>
        <button
          class="px-3 py-1 font-bold bg-white hover:bg-black hover:text-white transition-colors select-none active:scale-95"
          on:click=move |e| {
            on_change.run(value.get() + 1);
            let _ = e.target().unwrap().unchecked_into::<web_sys::HtmlElement>().blur();
          }
        >
          "+"
        </button>
      </div>
    }
}

#[component]
pub(super) fn Settings(
    grid: RwSignal<Grid>,
    #[prop(into)] resize_grid: Callback<(Option<usize>, Option<usize>)>,
) -> impl IntoView {
    view! {
      <div class="flex items-center gap-6">
        <div class="flex items-center gap-2">
          "Guess count: "
          <NumberCounter
            value=Signal::derive(move || grid.read().rows)
            on_change=move |v| resize_grid.run((Some(v), None))
          />
        </div>
        <div class="flex items-center gap-2">
          "Word length: "
          <NumberCounter
            value=Signal::derive(move || grid.read().cols)
            on_change=move |v| resize_grid.run((None, Some(v)))
          />
        </div>
      </div>
    }
}
