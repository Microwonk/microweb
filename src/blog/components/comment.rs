use leptos::{prelude::*, task::spawn_local};
use reactive_stores::Store;

use crate::blog::app::{GlobalState, GlobalStateStoreFields};

use crate::models::*;

#[server(CommentAction, "/api", endpoint = "comment")]
#[tracing::instrument]
pub async fn comment(comment: NewComment, post_id: i32) -> Result<Vec<Comment>, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    let Extension(user) = extract::<Extension<User>>()
        .await
        .map_err(|_| ServerFnError::new("Unauthorized."))?;

    sqlx::query(
        r#"
        INSERT INTO comments (
            author, post, content, replying_to
        )
        VALUES (
            $1, $2, $3, $4
        )
        "#,
    )
    .bind(user.id)
    .bind(post_id)
    .bind(ammonia::clean(&comment.content))
    .bind(None::<i32>)
    .execute(crate::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while creating comment: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not create comment.")
    })?;

    sqlx::query_as(
        r#"
        SELECT 
            comments.id,
            users.name AS author_name,
            comments.author AS author_id,
            comments.content,
            comments.replying_to,
            comments.created_at
        FROM comments
        JOIN posts ON comments.post = posts.id
        LEFT JOIN users ON comments.author = users.id
        WHERE comments.post = $1
        ORDER BY comments.created_at DESC
    "#,
    )
    .bind(post_id)
    .fetch_all(crate::database::db())
    .await
    .map_err(|e| {
        let err = format!("Error while creating comment: {e:?}");
        tracing::error!("{err}");
        ServerFnError::new("Could not create comment.")
    })
}

#[server(DeleteCommentAction, "/api", endpoint = "delete_comment")]
#[tracing::instrument]
pub async fn delete_comment(comment: Comment) -> Result<u64, ServerFnError> {
    use axum::extract::Extension;
    use leptos_axum::extract;

    let Extension(user) = extract::<Extension<User>>()
        .await
        .map_err(|_| ServerFnError::new("Unauthorized."))?;

    if !comment
        .author_id
        .is_some_and(|id| id == user.id || user.admin)
    {
        return Err(ServerFnError::new("Could not delete comment."));
    }

    sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(comment.id)
        .execute(crate::database::db())
        .await
        .map_err(|e| {
            let err = format!("Error while deleting comment: {e:?}");
            tracing::error!("{err}");
            ServerFnError::new("Could not delete comment.")
        })
        .map(|r| r.rows_affected())
}

#[component]
pub fn CommentComponent(
    #[prop(into)] comment: Signal<Comment>,
    #[prop(into)] comments: RwSignal<Vec<Comment>>,
    #[prop(into)] delete_btn: bool,
) -> impl IntoView {
    let icon = icondata::IoPersonCircleOutline;
    let delete_icon = icondata::IoTrashBin;

    view! {
        <article class="p-6 text-base bg-nf-white rounded-lg">
            <footer class="flex justify-between items-center mb-2">
                <div class="flex items-center">
                    <p class="inline-flex items-center text-sm text-gray-900 font-semibold">
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
                    </p>
                    <div class=move || {
                        comment.get().author_id.map(|_| "mr-6").unwrap_or("text-red-500 mr-6")
                    }>{comment.get().author_name.unwrap_or("DELETED USER".into())}</div>
                    <p class="text-sm text-gray-600">
                        <time
                            prop:pubdate
                            datetime=comment.get().created_at.format("%Y-%m-%d").to_string()
                        >
                            {comment.get().created_at.format("%b. %d, %Y").to_string()}
                        </time>
                    </p>
                </div>
                // delete icon
                <Show when=move || delete_btn>
                    <div class="right-0 top-0">
                        <button on:click=move |_| {
                            spawn_local(async move {
                                let comment = comment.get();
                                if delete_comment(comment.clone()).await.is_ok() {
                                    comments
                                        .set(
                                            comments
                                                .get()
                                                .into_iter()
                                                .filter(|c| c.id != comment.id)
                                                .collect(),
                                        )
                                }
                            })
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
            <p class="text-nf-dark">{comment.get().content}</p>
        </article>
    }
}

#[component]
pub fn CommentSection(
    #[prop(into)] comments: Vec<Comment>,
    #[prop(into)] blog_post_id: Signal<i32>,
) -> impl IntoView {
    let (new_comment, set_new_comment) = signal(Default::default());
    let store = expect_context::<Store<GlobalState>>();

    let comments = RwSignal::new(comments);

    view! {
        <div class="bg-nf-dark p-4 pb-12">
            // heading for the commentsection
            <div id="comment_section" class="flex justify-between items-center mb-6">
                <h2 class="text-lg lg:text-2xl font-bold text-nf-white">
                    {move || format!("Comments ({})", comments.get().len())}
                </h2>
            </div>

            // post new comment or login/signup
            {move || {
                if store.user().get().is_some() {
                    view! {
                        // textarea for new comment
                        <div class="mb-6 min-w-full px-48">
                            <div class="py-2 px-4 m-4 bg-nf-white rounded-lg">
                                <label for="comment" class="sr-only">
                                    Your comment
                                </label>
                                <textarea
                                    rows="6"
                                    class="px-0 w-full text-sm text-nf-dark border-0 focus:ring-0 focus:outline-none placeholder-gray-900 bg-nf-white"
                                    placeholder="Write a (plaintext) comment..."
                                    required
                                    on:input=move |ev| {
                                        let new_value = event_target_value(&ev);
                                        set_new_comment(NewComment { content: new_value });
                                    }
                                ></textarea>
                            </div>
                            // post button
                            <button
                                class="ml-4 inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-nf-white bg-nf-color rounded-lg focus:ring-4 focus:ring-primary-200"
                                on:click=move |_| {
                                    spawn_local(async move {
                                        if let Ok(c) = comment(
                                                new_comment.get(),
                                                blog_post_id.get(),
                                            )
                                            .await
                                        {
                                            comments.set(c);
                                        }
                                    });
                                }
                            >
                                Post comment
                            </button>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <p class="text-lg lg:text-2xl text-center font-bold text-nf-white p-12">
                            You need an Account to post a comment.
                            <a href="/login" class="underline text-nf-color">
                                Login
                            </a>or <a class="underline text-nf-color" href="/register">
                                Register
                            </a>a new Account!
                        </p>
                    }
                        .into_any()
                }
            }}

            // actual comments
            {move || {
                if comments.get().is_empty() {
                    view! {
                        <h2 class="text-lg lg:text-2xl text-center font-bold text-nf-white mb-6">
                            No Comments yet . . .
                        </h2>
                    }
                        .into_any()
                } else {
                    view! {
                        <ul class="grid gap-4">
                            <For
                                each=move || comments.get()
                                key=|c| c.id
                                children=move |comment: Comment| {
                                    let delete_btn = store
                                        .user()
                                        .get()
                                        .is_some_and(|u| {
                                            u.id == comment.author_id.unwrap_or_default() || u.is_admin
                                        });
                                    view! {
                                        <li class="px-48 min-w-full">
                                            <CommentComponent comment comments delete_btn />
                                        </li>
                                    }
                                }
                            />
                        </ul>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
