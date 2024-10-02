use crate::components::*;
use crate::sections::features::{Project, ProjectData};
use gloo_net::http::Request;
use leptos::prelude::*;

use leptos::svg::Svg;
use leptos::*;
use leptos_use::use_element_hover;

pub async fn get_project_by_slug(project_slug: String) -> Result<Option<Project>, ()> {
    // Perform a GET request to fetch the JSON data
    let response = Request::get("/resources/projects.json")
        .send()
        .await
        .map_err(|_| ())?;

    // Parse the response body as JSON
    let project_data: ProjectData = response.json().await.map_err(|_| ())?;

    // Find the project with the matching slug
    let project = project_data
        .data
        .into_iter()
        .find(|p| p.slug == project_slug);

    Ok(project)
}

#[component]
pub fn UseCase(slug: String) -> impl IntoView {
    let (project, write_project) = create_signal(None::<Project>);

    create_effect(move |_| {
        let slug = slug.clone();
        spawn_local(async move {
            if let Ok(value) = get_project_by_slug(slug).await {
                write_project(value);
            }
        });
    });

    let el = create_node_ref::<Svg>();
    let is_hovered = use_element_hover(el);

    view! {
        <Close el=el/>
        <main class={
            let base_class = "flex delay-75 duration-1000 mb-40 ease-out";
            move || {
                if is_hovered.get() {
                    format!("{} {}", base_class, "usecase-in")
                } else {
                    format!("{} {}", base_class, "usecase-out")
                }
            }
        }>
            <Layout id="project".to_string() aria_label="Usecase" class_name="flex-col".to_string()>
                {move || match project.get() {
                    None => view! {}.into_view(),
                    Some(data) => {
                        let timeline = data.information.timeline.clone();
                        let role = data.information.role.clone();
                        let vec_names: Vec<String> = data
                            .name
                            .split_whitespace()
                            .map(|s| s.to_string())
                            .collect();
                        view! {
                            <h1 class="text-7xl sm:text-8xl xl:text-10xl tracking-normal text-nf-white mb-4 mt-8 md:mb-10 md:mt-20 sm:leading-mediumheading xl:leading-heading tracking-smallheading sm:tracking-heading">
                                {vec_names
                                    .iter()
                                    .map(|name| {
                                        view! {
                                            <div class="animated-title">
                                                <span class="animated-title-element text-nf-white font-bold">
                                                    {name}
                                                </span>
                                            </div>
                                            {' '}
                                        }
                                    })
                                    .collect_view()}

                            </h1>
                            <div class="flex flex-col lg:flex-row gap-8 lg:gap-10 lg:gap-20 fade-y-trans">
                                <div class="w-full lg:w-2/6 flex flex-col gap-8">
                                    <p class="font-montserrat text-xl lg:text-2xl lg:text-3xl lg:leading-relaxed leading-relaxed text-nf-white">
                                        {data.description}
                                    </p>

                                    <div class="flex flex-row flex-wrap gap-2 lg:gap-4 overflow-x-scroll lg:overflow-x-hidden">
                                        <button class="pill pill-cta" role="button">
                                            <a target="_blank" href=data.link.url>
                                                <span class="pill-cta-border"></span>
                                                <span class="pill-cta-ripple">
                                                    <span></span>
                                                </span>
                                                <span class="pill-cta-title">
                                                    <span data-text="visit">visit</span>
                                                </span>
                                            </a>
                                        </button>

                                        {move || {
                                            data.tags
                                                .iter()
                                                .map(|tag| {
                                                    view! {
                                                        <div class="bg-nf-color rounded-full px-6 py-2">
                                                            <span class="font-[400] text-nf-white text-md">{tag}</span>
                                                        </div>
                                                    }
                                                })
                                                .collect_view()
                                        }}

                                    </div>

                                    {move || {
                                        if let Some(github_repo) = data.information.github.clone() {
                                            let (owner, repo) = github_repo.split_once('/').unwrap_or_default();
                                            view! {
                                                <div>
                                                    <GithubRepository owner=owner.to_string() repo=repo.to_string() />
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div/>
                                            }
                                        }
                                    }}
                                </div>
                                <div class="w-full md:w-4/6 flex flex-col">
                                    <Show when=move || data.information.role.is_some()>
                                        <p class="text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-nf-white font-bold">
                                            {role.clone()}
                                        </p>
                                    </Show>
                                    <Show when=move || data.information.timeline.is_some()>
                                        <p class="font-montserrat text-xs text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-nf-white font-light">
                                            <b>Timeline:</b>
                                            {' '}
                                            {timeline.clone()}
                                        </p>
                                    </Show>
                                    <p
                                        inner_html=ammonia::Builder::new()
                                            .clean(&data.information.responsibility.clone())
                                            .to_string()
                                        class="font-montserrat text-md md:text-lg lg:text-xl lg:leading-relaxed leading-relaxed text-nf-white mt-2 md:mt-8"
                                    ></p>
                                    {move || {
                                        if let Some(itch_embed) = data.information.itch.clone() {
                                            view! {
                                                <div class="hidden aspect-video xl:block">  // Hides on small screens
                                                    <ItchEmbed link=itch_embed.link embed_link=itch_embed.embed_link title=itch_embed.title/>
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div/>
                                            }
                                        }
                                    }}
                                </div>
                            </div>

                        }
                            .into_view()
                    }
                }}
            </Layout>
        </main>
    }
}
