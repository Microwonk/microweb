use leptos::*;

use crate::{
    types::{self, Comment, NewComment},
    util::Api,
};

#[component]
pub fn CommentComponent(#[prop(into)] comment: MaybeSignal<types::Comment>) -> impl IntoView {
    let icon = icondata::IoPersonCircleOutline;
    let comment = comment.get();
    view! {
        <article class="p-6 text-base bg-nf-white rounded-lg">
            <footer class="flex justify-between items-center mb-2">
                <div class="flex items-center">
                    <p class="inline-flex items-center mr-3 text-sm text-gray-900 font-semibold">
                    <svg
                        x=icon.x
                        y=icon.y
                        width=64
                        height=64
                        viewBox=icon.view_box
                        stroke-linecap=icon.stroke_linecap
                        stroke-linejoin=icon.stroke_linejoin
                        stroke-width=icon.stroke_width
                        stroke=icon.stroke
                        fill=icon.fill.unwrap_or("currentColor")
                        inner_html=icon.data
                    ></svg>
                        {comment.author_name}
                    </p>
                    <p class="text-sm text-gray-600"><time pubdate datetime={comment.created_at.format("%Y-%m-%d").to_string()}
                    >{comment.created_at.format("%b. %d, %Y").to_string()}</time></p>
                </div>

            </footer>
            <p class="text-gray-500">{comment.content}</p>
        </article>
    }
}

#[component]
pub fn CommentSection(
    #[prop(into)] comments: ReadSignal<Option<Vec<Comment>>>,
    #[prop(into)] post_id: i32,
) -> impl IntoView {
    view! {
        <div class="bg-nf-dark p-4">
            // heading for the commentsection
            <div class="flex justify-between items-center mb-6">
                <h2 class="text-lg lg:text-2xl font-bold text-nf-white">{move || format!("Comments ({})", comments.get().unwrap_or_default().len())}</h2>
            </div>
            // textarea for new comment
            <div class="mb-6">
            <div class="py-2 px-4 m-4 bg-nf-white rounded-lg">
                <label for="comment" class="sr-only">Your comment</label>
                <textarea id="comment" rows="6"
                    class="px-0 w-full text-sm text-nf-dark border-0 focus:ring-0 focus:outline-none placeholder-gray-900 bg-nf-white"
                    placeholder="Write a comment..." required></textarea>
            </div>
            // post button
            <button
                class="ml-4 inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-nf-white bg-nf-color rounded-lg focus:ring-4 focus:ring-primary-200"
                on:click=move |_| {
                    spawn_local(async move {
                        if (Api::create_comment(post_id, NewComment { content: "Some Test Content".into()}).await).is_ok() {
                            // refresh
                            web_sys::window().unwrap().location().reload().unwrap();
                        };
                    });
                }>
                Post comment
            </button>
            </div>
            // actual comments
            <Show when=move || comments.get().is_some_and(|comments| !comments.is_empty()) fallback=move || view! {
                <h2 class="text-lg lg:text-2xl text-center font-bold text-nf-white">No Comments yet...</h2>
            }>
                <ul class="grid gap-4 p-12">
                    <For
                        each=move || comments.get().unwrap_or_default()
                        key=|c| c.id
                        children=move |comment: Comment| {
                            view! {
                                <li>
                                    <CommentComponent comment/>
                                </li>
                            }
                        }
                    />
                </ul>
            </Show>
        </div>
    }
}
