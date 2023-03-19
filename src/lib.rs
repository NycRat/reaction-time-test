use leptos::*;

pub mod home_page;

use home_page::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <HomePage />
    }
}
