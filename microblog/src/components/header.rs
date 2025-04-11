use leptos::{prelude::*, task::spawn_local};
use leptos_router::components::A;
use reactive_stores::Store;

use crate::{
    app::{GlobalState, GlobalStateStoreFields},
    models::Profile,
};

#[component]
pub fn Header() -> impl IntoView {
    let user = RwSignal::new(use_context::<Option<Profile>>().unwrap_or_default());
    let header = {
        move || {
            if let Some(u) = user.get() {
                view! {
                    <LogoutButton user />

                    <div class="group relative inline-block w-1/6">
                        <div class="experience experience-cta">
                            <span class="experience-cta-border"></span>
                            <span class="experience-cta-ripple">
                                <span></span>
                            </span>
                            <span class="experience-cta-title">
                                <span
                                    data-text=u.email.clone()
                                    class="justify-between flex-row w-full"
                                >
                                    {u.email.clone()}
                                </span>
                            </span>
                        </div>
                    </div>
                }
                .into_any()
            } else {
                view! {
                    <div class="group relative inline-block text-sm sm:text-lg font-medium text-black focus:outline-none focus:ring active:text-nf-color">
                        <A href="/login">
                            <span class="absolute inset-0 border border-current"></span>
                            <span class="block border border-current bg-nf-black px-12 py-3 transition-transform group-hover:-translate-x-1 group-hover:-translate-y-1 group-hover:backdrop-blur">
                                login
                            </span>
                        </A>
                    </div>

                    <div class="group relative inline-block text-sm sm:text-lg font-medium text-black focus:outline-none focus:ring active:text-nf-color">
                        <A href="/register">
                            <span class="absolute inset-0 border border-current"></span>
                            <span class="block border border-current bg-nf-black px-12 py-3 transition-transform group-hover:-translate-x-1 group-hover:-translate-y-1 group-hover:backdrop-blur">
                                register
                            </span>
                        </A>
                    </div>
                }.into_any()
            }
        }
    };

    view! {
        <header
            id="header"
            class="sticky top-0 mx-auto max-w-full selection:bg-nf-white selection:text-nf-dark relative backdrop-invert-0 z-10"
            style="backdrop-filter: blur(5px)"
        >
            <nav
                class="w-full bg-nf-dark py-2 md:py-3 border-b-nf-white/[0.35] border-b-2 px-3 md:px-5"
                aria-label="Global"
            >
                <ul class="gap-4 flex-row flex items-center justify-between">
                    <li class="text-nf-white font-rosmatika text-md md:text-lg flex uppercase gap-1">
                        <a href="/">
                            <span>"Nicolas'"</span>
                            <span class="block sm:hidden font-rosmatika">Blog</span>
                        </a>
                    </li>
                </ul>
            </nav>
            <nav
                class="w-full py-2 md:py-3 bg-transparent border-b-nf-white/[0.35] border-b-2 px-3 md:px-5"
                aria-label="Global"
            >
                <ul class="gap-4 flex-row flex justify-between">
                    <li class="font-rosmatika hidden sm:block text-nf-dark text-md md:text-lg flex uppercase hover:text-nf-color transition">
                        <a href="/">Blog</a>
                    </li>

                    <li class="font-montserrat flex gap-4 md:gap-8 items-center w-full md:justify-end justify-center">
                        <a
                            href="https://www.nicolas-frey.com"
                            class="text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color"
                            target="_blank"
                        >
                            about
                        </a>
                        <a
                            href="/feed"
                            class="font-montserrat text-sm sm:text-lg text-nf-dark flex items-center gap-1 hover:animate-pulse hover:text-nf-color "
                        >
                            feed
                        </a>
                        {header}
                    </li>
                </ul>
            </nav>
        </header>
    }
}

#[server(LogoutAction, "/api")]
#[tracing::instrument]
pub async fn logout() -> Result<(), ServerFnError> {
    use axum::http::{header, HeaderValue};
    use leptos_axum::ResponseOptions;

    let response = expect_context::<ResponseOptions>();

    response.append_header(
        header::SET_COOKIE,
        HeaderValue::from_str(
            "auth_token=deleted; Path=/; SameSite=Strict; Secure; expires=Thu, 01 Jan 1970 00:00:00 GMT;",
        )?,
    );

    Ok(())
}

#[component]
fn LogoutButton(user: RwSignal<Option<Profile>>) -> impl IntoView {
    let state = expect_context::<Store<GlobalState>>();

    let on_click = {
        move |_| {
            spawn_local(async move {
                if logout().await.is_ok() {
                    user.set(None);
                    state.logged_in().set(false);
                };
            });
        }
    };

    view! {
        <button
            class="group relative inline-block text-sm sm:text-lg font-medium text-black focus:outline-none focus:ring active:text-nf-color"
            on:click=on_click
        >
            <span class="pointer-events-none absolute inset-0 border border-current"></span>
            <span class="pointer-events-none block border border-current bg-nf-black px-12 py-3 transition-transform group-hover:-translate-x-1 group-hover:-translate-y-1 group-hover:backdrop-blur">
                "logout"
            </span>
        </button>
    }
}
