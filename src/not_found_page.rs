use leptos::*;

#[component]
pub fn NotFoundPage(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div>
            <h1>
                "404 Page not found."
            </h1>
            <a href="/reaction-time-test">
                "Return"
            </a>
        </div>
    }
}
