use std::process::exit;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (random_word, set_random_word) = signal(String::new());

    let list_of_words = vec![
        "Konterkarierend",
        "Famos",
    ];

    let random_index = match getrandom::u32()  {
        Ok(index) => index % list_of_words.len() as u32,
        Err(e) => {
            eprintln!("Error generating random index: {}", e);
            exit(1);
        }
    };

    set_random_word.set(list_of_words[random_index as usize].to_string());

    view! {
        <main class="container">
            <p>{ move || random_word.get() }</p>
        </main>
    }
}
