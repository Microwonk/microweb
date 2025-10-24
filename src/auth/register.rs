use leptos::{prelude::*, task::spawn_local};
use leptos_meta::*;
use leptos_router::hooks::use_query;
use regex::Regex;

use crate::{auth::ReturnUrlQuery, models::*};

#[server(RegisterAction, "/api", endpoint = "register")]
#[tracing::instrument]
pub async fn register(register: RegisterRequest, return_url: String) -> Result<(), ServerFnError> {
    use crate::auth::encode_jwt;
    use bcrypt::{DEFAULT_COST, hash};
    use leptos_axum::{ResponseOptions, redirect};

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(register.email.as_str())
        .fetch_one(crate::database::db())
        .await;

    if user.is_ok() {
        return Err(ServerFnError::new("User already exists."));
    }

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email, admin, passwordhash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, email, admin, passwordhash, created_at
        "#,
    )
    .bind(register.name)
    .bind(register.email)
    .bind(false)
    .bind(
        hash(register.password.as_str(), DEFAULT_COST)
            .map_err(|_| ServerFnError::new("Error creating User."))?,
    )
    .fetch_one(crate::database::db())
    .await
    .map_err(|_| ServerFnError::new("Error authenticating User."))?;

    let token =
        encode_jwt(user.email.clone()).map_err(|_| ServerFnError::new("Error encoding jwt."))?;

    let response = expect_context::<ResponseOptions>();

    let expires = (chrono::Utc::now() + chrono::Duration::days(crate::auth::EXPIRATION_DAYS))
        .format("%a, %d %b %Y %H:%M:%S GMT");

    crate::auth::set_auth_cookie(response, &token, &expires.to_string());

    redirect(&return_url);

    Ok(())
}

#[component]
pub fn RegisterPage() -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (username, set_username) = signal(String::new());
    let (email_error, set_email_error) = signal(None::<String>);
    let (password_error, set_password_error) = signal(None::<String>);
    let (username_error, set_username_error) = signal(None::<String>);

    let back_icon = icondata::IoArrowBackOutline;

    let query = use_query::<ReturnUrlQuery>();

    view! {
        <Title text="Register" />
        <div class="mx-auto max-w-screen-xl px-4 py-16 sm:px-6 lg:px-8">
            <div class="mx-auto max-w-lg">
                <a href=move || {
                    query
                        .with(|q| {
                            q.as_ref()
                                .map(|r| r.return_url.clone())
                                .ok()
                                .unwrap_or("/profile".into())
                        })
                }>
                    <svg
                        x=back_icon.x
                        y=back_icon.y
                        width=48
                        height=48
                        viewBox=back_icon.view_box
                        stroke-linecap=back_icon.stroke_linecap
                        stroke-linejoin=back_icon.stroke_linejoin
                        stroke-width=back_icon.stroke_width
                        stroke=back_icon.stroke
                        fill=back_icon.fill.unwrap_or("currentColor")
                        inner_html=back_icon.data
                    ></svg>
                </a>

                <h1 class="text-center text-2xl font-bold text-black sm:text-3xl">
                    Create an account
                </h1>

                <p class="mx-auto mt-4 max-w-md text-center text-gray-500">
                    Creating an account lets you post comments!
                </p>

                <div class="mb-0 mt-6 space-y-4 rounded-lg p-4 shadow-lg sm:p-6 lg:p-8">

                    <div>
                        <label for="username" class="sr-only">
                            Username
                        </label>

                        <div class="relative">
                            <input
                                class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                                style="width: 85%;"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_username(value);
                                    set_username_error(None);
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
                        {move || {
                            username_error
                                .get()
                                .map(|error| {
                                    view! { <p class="text-red-500 text-sm mt-2">{error}</p> }
                                })
                        }}
                    </div>

                    <div>
                        <label for="email" class="sr-only">
                            Email
                        </label>

                        <div class="relative">
                            <input
                                type="email"
                                class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                                style="width: 85%;"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_email(value);
                                    set_email_error(None);
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
                        {move || {
                            email_error
                                .get()
                                .map(|error| {
                                    view! { <p class="text-red-500 text-sm mt-2">{error}</p> }
                                })
                        }}
                    </div>

                    <div>
                        <label for="password" class="sr-only">
                            Password
                        </label>

                        <div class="relative">
                            <input
                                type="password"
                                class="rounded-lg border-gray-200 p-4 pe-12 text-sm shadow-sm"
                                style="width: 85%;"
                                on:input=move |ev| {
                                    let value = event_target_value(&ev);
                                    set_password(value);
                                    set_password_error(None);
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
                        {move || {
                            password_error
                                .get()
                                .map(|error| {
                                    view! { <p class="text-red-500 text-sm mt-2">{error}</p> }
                                })
                        }}
                    </div>

                    <button
                        class="block w-full rounded-lg bg-black px-5 py-3 text-sm font-medium text-white"
                        on:click=move |_| {
                            let username_value = username.get();
                            let email_value = email.get();
                            let password_value = password.get();
                            let mut valid = true;
                            if !Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
                                .unwrap()
                                .is_match(email_value.as_str())
                            {
                                set_email_error(
                                    Some("Please enter a valid email address.".to_string()),
                                );
                                valid = false;
                            }
                            if password_value.len() < 8 {
                                set_password_error(
                                    Some("Password must be at least 8 characters long.".to_string()),
                                );
                                valid = false;
                            }
                            if username_value.is_empty() {
                                set_username_error(Some("Please enter a Username.".to_string()));
                            }
                            if valid {
                                spawn_local(async move {
                                    if let Err(e) = register(
                                            RegisterRequest {
                                                email: email_value,
                                                password: password_value,
                                                name: username_value,
                                            },
                                            query
                                                .with(|q| {
                                                    q.as_ref()
                                                        .map(|r| r.return_url.clone())
                                                        .ok()
                                                        .unwrap_or("/profile".into())
                                                }),
                                        )
                                        .await
                                    {
                                        set_password_error(
                                            Some(
                                                e
                                                    .to_string()
                                                    .split(": ")
                                                    .last()
                                                    .unwrap_or_default()
                                                    .to_owned(),
                                            ),
                                        );
                                    }
                                });
                            }
                        }
                    >
                        Sign up
                    </button>

                    <p class="text-center text-sm text-gray-500">
                        Already have an account?
                        <a
                            class="underline text-black"
                            href=move || {
                                format!(
                                    "/login?return_url={}",
                                    query
                                        .with(|q| {
                                            q.as_ref()
                                                .map(|r| r.return_url.clone())
                                                .ok()
                                                .unwrap_or("/profile".into())
                                        }),
                                )
                            }
                        >
                            Sign in
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}
