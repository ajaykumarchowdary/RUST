use leptos::*;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Default)]
struct User {
    id: i32,
    name: String,
}

#[component]
fn App() -> impl IntoView {
    // Reactive signal to store our data
    let (user_data, set_user_data) = create_signal(User::default());

    // Asynchronous action to fetch data from our Axum API
    let fetch_user_action = create_action(move |_| async move {
        let client = reqwest::Client::new();
        if let Ok(res) = client.get("http://127.0.0.1:3000/api/user").send().await {
            if let Ok(user) = res.json::<User>().await {
                set_user_data.set(user);
            }
        }
    });

    view! {
        <div style="font-family: Arial, sans-serif; padding: 40px; text-align: center;">
            <h2>"Full-Stack Rust App (Wasm + Axum)"</h2>
            <button 
                style="padding: 10px 20px; font-size: 16px; cursor: pointer; background-color: #df5b2a; color: white; border: none; border-radius: 4px;"
                on:click=move |_| fetch_user_action.dispatch(())
            >
                "Fetch Profile from DB"
            </button>
            
            <div style="margin-top: 30px; border: 1px solid #ccc; display: inline-block; padding: 20px; border-radius: 8px;">
                <h3>"Database Record:"</h3>
                <p>"User ID: " {move || user_data.get().id}</p>
                <p>"User Name: " {move || user_data.get().name}</p>
            </div>
        </div>
    }
}

fn main() {
    // Mount the Leptos app component to Chrome's DOM body element
    leptos::mount_to_body(|| view! { <App /> });
}