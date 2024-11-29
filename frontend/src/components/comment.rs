use leptos::*;

use crate::{
    types::{self, Comment, NewComment, Post, Profile},
    util::Api,
};

#[component]
pub fn CommentComponent(
    #[prop(into)] comment: MaybeSignal<types::Comment>,
    #[prop(into)] delete_btn: ReadSignal<bool>,
) -> impl IntoView {
    let icon = icondata::IoPersonCircleOutline;
    let delete_icon = icondata::IoTrashBin;
    let comment = comment.get();
    view! {
        <article class="p-6 text-base bg-nf-white rounded-lg">
            <footer class="flex justify-between items-center mb-2">
                <div class="flex items-center">
                    <p class="inline-flex items-center mr-6 text-sm text-gray-900 font-semibold">
                    <svg
                        x=icon.x
                        y=icon.y
                        width=48
                        height=48
                        viewBox=icon.view_box
                        stroke-linecap=icon.stroke_linecap
                        stroke-linejoin=icon.stroke_linejoin
                        stroke-width=icon.stroke_width
                        stroke=icon.stroke
                        fill=icon.fill.unwrap_or("currentColor")
                        inner_html=icon.data
                    ></svg>
                        <div class=move || if comment.author_id.is_none() {"text-red-500"} else {""}>
                            {comment.author_name.unwrap_or("DELETED USER".into())}
                        </div>
                    </p>
                    <p class="text-sm text-gray-600"><time pubdate datetime={comment.created_at.format("%Y-%m-%d").to_string()}
                    >{comment.created_at.format("%b. %d, %Y").to_string()}</time></p>
                </div>
                // delete icon
                <Show when=move || delete_btn.get()>
                    <div class="right-0 top-0">
                        <button
                            on:click=move |_| {
                                spawn_local(async move {
                                    if (Api::delete_comment(comment.id).await).is_ok() {
                                        // refresh
                                        web_sys::window().unwrap().location().reload().unwrap();
                                    };
                                });
                            }>
                            <svg
                                x=delete_icon.x
                                y=delete_icon.y
                                width=32
                                height=32
                                viewBox=delete_icon.view_box
                                stroke-linecap=delete_icon.stroke_linecap
                                stroke-linejoin=delete_icon.stroke_linejoin
                                stroke-width=delete_icon.stroke_width
                                stroke=delete_icon.stroke
                                fill="red"
                                inner_html=delete_icon.data
                            ></svg>
                        </button>
                    </div>
                </Show>

            </footer>
            <p class="text-nf-dark">{comment.content}</p>
        </article>
    }
}

#[component]
pub fn CommentSection(
    #[prop(into)] user: ReadSignal<Option<Profile>>,
    #[prop(into)] is_admin: ReadSignal<bool>,
    #[prop(into)] comments: ReadSignal<Option<Vec<Comment>>>,
    #[prop(into)] blog_post: ReadSignal<Option<Post>>,
) -> impl IntoView {
    let (content, set_content) = create_signal("".to_string());
    view! {
        <div class="bg-nf-dark p-4 pb-12">
            // heading for the commentsection
            <div id="comment_section" class="flex justify-between items-center mb-6">
                <h2 class="text-lg lg:text-2xl font-bold text-nf-white">{move || format!("Comments ({})", comments.get().unwrap_or_default().len())}</h2>
            </div>

            // post new comment or login/signup
            <Show when=move || user.get().is_some() fallback=move || view! {
                <p class="text-lg lg:text-2xl text-center font-bold text-nf-white p-12">
                    You need an Account to post a comment. <a href="/login" class="underline text-nf-color">Login</a> or <a class="underline text-nf-color" href="/register">Register</a> a new Account!
                </p>
            }>
                // textarea for new comment
                <div class="mb-6 min-w-full px-48">
                    <div class="py-2 px-4 m-4 bg-nf-white rounded-lg">
                        <label for="comment" class="sr-only">Your comment</label>
                        <textarea rows="6"
                            class="px-0 w-full text-sm text-nf-dark border-0 focus:ring-0 focus:outline-none placeholder-gray-900 bg-nf-white"
                            placeholder="Write a (plaintext) comment..." required
                            on:input=move |ev| {
                                let new_value = event_target_value(&ev);
                                set_content(new_value);
                            }>
                        </textarea>
                    </div>
                    // post button
                    <button
                        class="ml-4 inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-nf-white bg-nf-color rounded-lg focus:ring-4 focus:ring-primary-200"
                        on:click=move |_| {
                            let content = ammonia::clean(&content.get());
                            spawn_local(async move {
                                if let Some(post) = blog_post.get() {
                                    if (Api::create_comment(post.id, NewComment { content }).await).is_ok() {
                                        // refresh
                                        web_sys::window().unwrap().location().reload().unwrap();
                                    };
                                }
                            });
                        }>
                        Post comment
                    </button>
                </div>
            </Show>

            // actual comments
            <Show when=move || comments.get().is_some_and(|comments| !comments.is_empty()) fallback=move || view! {
                <h2 class="text-lg lg:text-2xl text-center font-bold text-nf-white mb-6">No Comments yet . . .</h2>
            }>
                <ul class="grid gap-4">
                    <For
                        each=move || comments.get().unwrap_or_default()
                        key=|c| c.id
                        children=move |comment: Comment| {
                            let (delete_btn, set_delete_btn) = create_signal(false);
                            set_delete_btn(is_admin.get() || user.get().is_some_and(|u| u.id == comment.author_id.unwrap_or_default()));
                            view! {
                                <li class="px-48 min-w-full">
                                    <CommentComponent comment delete_btn/>
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}
