use leptos::*;
use serde::Deserialize;

// 1. Declare the components module
mod components;
// 2. Bring the buttons into scope
use components::clear_button::ClearButton;
use components::fetch_button::FetchButton;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[component]
fn App() -> impl IntoView {
    // Shared state between the components
    let (user_data, set_user_data) = create_signal(User::default());

    view! {
        <div style="font-family: Arial, sans-serif; padding: 40px; text-align: center;">
            <h2>"Full-Stack Rust App (Wasm + Axum)"</h2>
            
            // Render the Fetch Button file component
            <FetchButton set_user_data=set_user_data />

            // Render the Clear Button file component
            <ClearButton on_clear=move || set_user_data.set(User::default()) />
            
            <div style="margin-top: 30px; border: 1px solid #ccc; display: inline-block; padding: 20px; border-radius: 8px;">
                <h3>"Database Record:"</h3>
                <p>"User ID: " {move || user_data.get().id}</p>
                <p>"User Name: " {move || user_data.get().name}</p>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    leptos::mount_to_body(|| view! { <App /> });
}