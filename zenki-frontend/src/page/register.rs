use leptos::{prelude::*, task::spawn_local};
use leptos_meta::Title;

use crate::auth::register;

#[component]
pub fn Register() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let password_confirm = RwSignal::new(String::new());

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let _ = register(username.get(), password.get(), password_confirm.get()).await;
        });
    };

    view! {
        <Title text="Sign Up"/>
        <h1>"Sign Up"</h1>
        <form on:submit=on_submit>
            <div>
                <label for="username">"Username:"</label>
                <input
                    id="username"
                    type="text"
                    bind:value=username
                />
            </div>
            <div>
                <label for="password">"Password:"</label>
                <input
                    id="password"
                    type="password"
                    bind:value=password
                />
            </div>
            <div>
                <label for="password">"Confirm Password:"</label>
                <input
                    id="password"
                    type="password"
                    bind:value=password_confirm
                />
            </div>
            <div>
                <button type="submit">"Register"</button>
            </div>
        </form>
    }
}
