use leptos::prelude::*;
use leptos_use::{UseClipboardOptions, UseClipboardReturn};

#[derive(Clone)]
pub(super) struct SolutionEntry {
    pub id: usize,
    pub name: String,
    pub value: Option<Vec<String>>,
}

#[component]
pub(super) fn SolveButton(#[prop(into)] solve: Callback<()>, is_solvable: Memo<bool>) -> impl IntoView {
    view! {
      <div>
        <button
          class="border-2 border-black p-2 px-6 font-bold uppercase bg-white hover:bg-black hover:text-white transition-colors h-[46px] disabled:opacity-50 active:scale-95"
          on:click=move |_| solve.run(())
          disabled=move || !is_solvable.get()
        >
          "Solve"
        </button>
      </div>
    }
}

#[component]
fn SolutionButton(word: String) -> impl IntoView {
    let UseClipboardReturn { copy, copied, .. } =
        leptos_use::use_clipboard_with_options(UseClipboardOptions::default().copied_reset_delay(750.0));

    let word_for_copy = StoredValue::new(word.to_ascii_uppercase());
    let word_for_show = StoredValue::new(word);

    view! {
      <button
        type="button"
        class="font-mono antialiased bg-gray-200 px-3 py-1 text-base font-semibold tracking-wider uppercase hover:brightness-90 active:scale-95"
        on:click=move |_| word_for_copy.with_value(|word| copy(word))
      >
        <Show when=move || copied.get() fallback=move || view! { <span>{word_for_show.get_value()}</span> }>
          <span class="text-green-700">"Copied!"</span>
        </Show>
      </button>
    }
}

#[component]
fn SolutionSet(name: String, words: Vec<String>) -> impl IntoView {
    let count = words.len();
    let buttons = words
        .into_iter()
        .map(|word| view! {<SolutionButton word=word />})
        .collect_view();

    view! {
        <div>
            <p class="text-sm font-semibold mb-2">
              {format!("{name}: {count} solution{}", if count == 1 { "" } else { "s" })}
            </p>
                {if count == 0 {
                  view! {
                    <p class="text-sm text-gray-500 italic">
                      "No matching words found. Check for contradictions."
                    </p>
                  }.into_any()
                } else {
                  view! {
                    <div class="overflow-y-auto max-h-[40vh] border-2 border-black bg-white">
                      <div class="flex flex-wrap gap-2 p-3 justify-center">
                        {buttons}
                      </div>
                    </div>
                  }.into_any()
                }}
              </div>
    }
}

#[component]
pub(super) fn Solutions(solved: RwSignal<bool>, solutions: RwSignal<Vec<SolutionEntry>>) -> impl IntoView {
    view! {
      <Show when=move || solved.get()>
        <div class="w-full max-w-2xl px-4 flex flex-col gap-6">
          <For
            each=move || solutions.get()
            key=|entry| (entry.name.clone(), entry.id)
            children=|entry| entry.value.map(|words| view! { <SolutionSet name=entry.name words=words />})
          />
        </div>
      </Show>
    }
}
