use crate::{
    components::{blog_card::BlogCard, header::Header},
    pages::loading::LoadingPage,
    types::Post,
    util::Api,
};
use leptos::*;
use leptos_meta::*;
// use leptos_use::*;

#[component]
pub fn HomePage(logged_in: ReadSignal<bool>) -> impl IntoView {
    let (blog_posts, set_blog_posts) = create_signal(Vec::new());
    let (loaded, set_loaded) = create_signal(false);
    spawn_local(async move {
        set_blog_posts(Api::all_blog_posts().await.unwrap_or(Vec::new()));
        set_loaded(true);
    });
    // let UseColorModeReturn { set_mode, .. } =
    //     use_color_mode_with_options(UseColorModeOptions::default());

    view! {
        <Title text="Blogs"/>

        <Header logged_in=logged_in/>
        <Show when=move || loaded.get() fallback=LoadingPage>
            <div class="mx-auto max-w-screen-xl px-4 pb-8 lg:pb-12 pt-8 lg:pt-12">
                <ul class="grid grid-cols-2 gap-4 md:grid-cols-3 lg:grid-cols-4 list-none">
                    <For
                        // a function that returns the items we're iterating over; a signal is fine
                        each=move || blog_posts.get()
                        // a unique key for each item
                        key=|b| b.id
                        // renders each item to a view
                        children=move |post: Post| {
                        view! {
                            <li>
                                <BlogCard title={post.title} description={post.description} link={post.slug}/>
                            </li>
                        }
                        }
                    />
                </ul>
            </div>
        </Show>

        // <ul>
        //     <li><button on:click=move |_| set_mode.set(ColorMode::Light)>Change to Light</button></li>
        //     <li><button on:click=move |_| set_mode.set(ColorMode::Dark)>Change to Dark</button></li>
        //     <li><button on:click=move |_| set_mode.set(ColorMode::Auto)>Change to Auto</button></li>
        // </ul>
        // <img src="https://microblog.shuttleapp.rs/upload/3" alt="My Image"/>
        // <video controls>
        //     <source type="video/mp4" src="https://microblog.shuttleapp.rs/upload/2"/>
        // </video>
    }
}
