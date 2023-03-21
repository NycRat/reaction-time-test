use leptos::*;
use leptos_router::*;

pub mod home_page;
pub mod not_found_page;
pub mod settings_page;

use home_page::*;
use not_found_page::*;
use settings_page::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <Router>
            <Routes>
                <Route path="/reaction-time-test" view=move |cx|
                    view! {
                        cx,
                        <HomePage />
                    }
                />
                <Route path="/reaction-time-test/settings" view=move |cx|
                    view! {
                        cx,
                        <SettingsPage/>
                    }
                />
                <Route path="*" view=move |cx|
                    view! {
                        cx,
                        <NotFoundPage />
                    }
                />
            </Routes>
        </Router>
    }
}
