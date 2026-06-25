use leptos::*;

#[component]
pub fn ClearButton<F>(
    // We pass an anonymous function (closure) as a prop
    on_clear: F
) -> impl IntoView 
where
    F: Fn() + 'static + Copy,
{
    view! {
        <button 
            style="padding: 10px 20px; font-size: 16px; cursor: pointer; background-color: #6c757d; color: white; border: none; border-radius: 4px;"
            on:click=move |_| {
                logging::log!("Clear button clicked! Resetting profile data...");
                on_clear(); // Executes the reset logic passed from main.rs
            }
        >
            "Clear Profile"
        </button>
    }
}