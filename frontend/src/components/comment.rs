use std::collections::HashMap;

use crate::types::Comment;
use leptos::*;

#[component]
pub fn Comment(#[prop(into)] comment: MaybeSignal<Comment>) -> impl IntoView {
    let icon = icondata::IoPersonCircleOutline;
    let comment = comment.get();
    view! {
        <article class="p-6 text-base bg-nf-dark border-2 border-nf-color rounded-md">
            <footer class="flex justify-between items-center mb-2">
                <div class="flex items-center">
                    <p class="inline-flex items-center mr-3 text-sm text-gray-900 dark:text-white font-semibold">
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
                    <p class="text-sm text-gray-600 dark:text-gray-400"><time pubdate datetime={comment.created_at.format("%Y-%m-%d").to_string()}
                    >{comment.created_at.format("%b. %d, %Y").to_string()}</time></p>
                </div>

            </footer>
            <p class="text-gray-500 dark:text-gray-400">{comment.content}</p>
            <div class="flex items-center mt-4 space-x-4">
                <button type="button"
                    class="flex items-center text-sm text-gray-500 hover:underline dark:text-gray-400 font-medium">
                    <svg class="mr-1.5 w-3.5 h-3.5" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 18">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5h5M5 8h2m6-3h2m-5 3h6m2-7H2a1 1 0 0 0-1 1v9a1 1 0 0 0 1 1h3v5l5-5h8a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1Z"/>
                    </svg>
                    Reply
                </button>
            </div>
        </article>
    }
}

#[component]
pub fn CommentSection(
    #[prop(into)] comments: MaybeSignal<HashMap<i32, Vec<Comment>>>,
) -> impl IntoView {
    view! {
        <ul class="grid gap-4 px-6">
            {move || {
                comments.get().values().map(|v| {
                    v.iter().map(|comment| {
                        view! {
                            <li>
                                <Comment comment=comment.clone()/>
                            </li>
                        }
                    }).collect_view()
                }).collect_view()
            }}
        </ul>
    }
}
