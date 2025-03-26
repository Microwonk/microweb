use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};
use regex::Regex;

use crate::models::Profile;

#[component]
pub fn RegisterPage(
    set_logged_in: WriteSignal<bool>,
    set_user: WriteSignal<Option<Profile>>,
) -> impl IntoView {
    let navigate = use_navigate();
    let (email, set_email) = signal("".to_string());
    let (password, set_password) = signal("".to_string());
    let (username, set_username) = signal("".to_string());
    let (email_error, set_email_error) = signal(None::<String>);
    let (password_error, set_password_error) = signal(None::<String>);
    let (username_error, set_username_error) = signal(None::<String>);

    view! {
        <Title text="Register"/>
        <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-lg">
                <h1 class="text-center text-2xl font-bold text-black sm:text-3xl">Create an account</h1>

                <p class="mx-auto mt-4 max-w-md text-center text-gray-500">
                Creating an account lets you post comments!
                </p>

                <div class="mb-0 mt-6 space-y-4 rounded-lg p-4 shadow-lg sm:p-6 lg:p-8">

                <div>
                    <label for="username" class="sr-only">Username</label>

                    <div class="relative">
                    <input
                        class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                        style="width: 85%;"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_username(value);
                            set_username_error(None); // Reset error on input
                        }
                        type="text"
                        prop:value=username
                        placeholder="Enter Username"
                    />

                    <span class="absolute inset-y-0 end-0 grid place-content-center px-4">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="size-4 text-gray-400"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor"
                        >
                            <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"
                            />
                        </svg>
                    </span>
                    </div>
                    // Display username error message
                    { move || username_error.get().map(|error| view! {
                        <p class="text-red-500 text-sm mt-2">{error}</p>
                    }) }
                </div>

                <div>
                    <label for="email" class="sr-only">Email</label>

                    <div class="relative">
                    <input
                        type="email"
                        class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                        style="width: 85%;"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_email(value);
                            set_email_error(None); // Reset error on input
                        }
                        prop:value=email
                        placeholder="Enter email"
                    />

                    <span class="absolute inset-y-0 end-0 grid place-content-center px-4">
                        <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="size-4 text-gray-400"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                        />
                        </svg>
                    </span>
                    </div>
                    // Display email error message
                    { move || email_error.get().map(|error| view! {
                        <p class="text-red-500 text-sm mt-2">{error}</p>
                    }) }
                </div>

                <div>
                    <label for="password" class="sr-only">Password</label>

                    <div class="relative">
                    <input
                        type="password"
                        class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                        style="width: 85%;"
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_password(value);
                            set_password_error(None); // Reset error on input
                        }
                        prop:value=password
                        placeholder="Enter password"
                    />

                    <span class="absolute inset-y-0 end-0 grid place-content-center px-4">
                        <svg
                        xmlns="http://www.w3.org/2000/svg"
                        class="size-4 text-gray-400"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                        >
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                        />
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                        />
                        </svg>
                    </span>
                    </div>
                    // Display password error message
                    { move || password_error.get().map(|error| view! {
                        <p class="text-red-500 text-sm mt-2">{error}</p>
                    }) }
                </div>

                <button
                    class="block w-full rounded-lg bg-black px-5 py-3 text-sm font-medium text-white"
                    on:click=move |_| {
                        let navigate = navigate.clone();
                        let username_value = username.get();
                        let email_value = email.get();
                        let password_value = password.get();

                        let mut valid = true;

                        if !Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
                                .unwrap()
                                .is_match(email_value.as_str())
                        {
                            set_email_error(Some("Please enter a valid email address.".to_string()));
                            valid = false;
                        }

                        if password_value.len() < 8 {
                            set_password_error(Some("Password must be at least 8 characters long.".to_string()));
                            valid = false;
                        }

                        if username_value.is_empty() {
                            set_username_error(Some("Please enter a Username.".to_string()));
                        }

                        // TODO
                        // spawn_local(async move {
                        //     if valid {
                        //         match Api::register(email_value, password_value, username_value).await {
                        //             Ok(_) => {
                        //                 set_logged_in(true);
                        //                 set_user(Api::get_profile().await.ok());
                        //                 navigate("/", NavigateOptions::default());
                        //             },
                        //             Err(_) => {
                        //                 set_email_error(None);
                        //                 set_password_error(Some("An error occurred, try again.".to_string()));
                        //             }
                        //         }
                        //     }
                        // });
                    }
                >
                    Sign up
                </button>

                <p class="text-center text-sm text-gray-500">
                    Already have an account?
                    <a class="underline text-black" href="/login">Sign in</a>
                </p>
                </div>
            </div>
        </div>
    }
}
