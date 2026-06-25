use leptos::*;
use serde::Deserialize;

// We redefine or import the struct structure here so this module knows how to parse JSON
#[derive(Clone, Debug, Deserialize, Default)]
struct User {
    id: i32,
    name: String,
}

#[component]
pub fn FetchButton(
    // We pass the signal setter as a "prop" so this sub-button can change the main App's state
    set_user_data: WriteSignal<crate::User> 
) -> impl IntoView {
    
    // The asynchronous action that triggers when the button is clicked
    let fetch_user_action = create_action(move |_| async move {
        logging::log!("Fetch button clicked! Sending request to API...");
        
        let client = reqwest::Client::new();
        match client.get("http://127.0.0.1:3000/api/user").send().await {
            Ok(res) => {
                logging::log!("API responded with status: {}", res.status());
                match res.json::<User>().await {
                    Ok(user) => {
                        logging::log!("Successfully parsed user data: {:?}", user);
                        
                        // Map the local module User struct data to the parent main.rs User struct
                        let app_user = crate::User { id: user.id, name: user.name };
                        set_user_data.set(app_user); 
                    }
                    Err(e) => logging::error!("Failed to parse JSON: {:?}", e),
                }
            }
            Err(e) => logging::error!("Network request failed: {:?}", e),
        }
    });

    view! {
        <button 
            style="padding: 10px 20px; font-size: 16px; cursor: pointer; background-color: #df5b2a; color: white; border: none; border-radius: 4px; margin-right: 10px;"
            disabled=move || fetch_user_action.pending().get()
            on:click=move |_| fetch_user_action.dispatch(())
        >
            {move || if fetch_user_action.pending().get() { "Fetching..." } else { "Fetch Profile from DB" }}
        </button>
    }
}