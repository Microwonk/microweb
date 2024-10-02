use crate::components::*;
use gloo_net::http::Request;

use leptos::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub data: Vec<Project>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Project {
    pub slug: String,
    pub name: String,
    pub area: String,
    pub image: Option<String>,
    pub description: String,
    pub information: Information,
    pub tags: Vec<String>,
    pub link: Link,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Information {
    pub role: Option<String>,
    pub timeline: Option<String>,
    pub responsibility: String,
    pub github: Option<String>,
    pub itch: Option<Itch>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Itch {
    pub embed_link: String,
    pub link: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Link {
    pub source: String,
    pub url: String,
}

pub async fn get_projects() -> Result<Vec<Project>, ()> {
    let response = Request::get("/resources/projects.json")
        .send()
        .await
        .map_err(|_| ())?;

    // Parse the response body as JSON
    let project_data: ProjectData = response.json().await.map_err(|_| ())?;

    Ok(project_data.data)
}

#[component]
fn FeaturedCards() -> impl IntoView {
    let (projects, write_projects) = create_signal(Vec::<Project>::new());

    create_effect(move |_| {
        spawn_local(async move {
            if let Ok(value) = get_projects().await {
                write_projects(value);
            }
        });
    });

    view! {
        <div class="features mt-20 md:mt-40" id="projects">
            {move || {
                projects
                    .get()
                    .iter()
                    .map(|project| {
                        let project_clone = project.clone();
                        view! {
                            <Card
                                name=project_clone.slug.to_string()
                                style=""
                                class_name=project.area.to_string()
                            >
                                <div class="h-full flex justify-center items-center">
                                    {move || {
                                        if let Some(image) = &project_clone.image {
                                            view! {
                                                <div>
                                                    <img src=image />
                                                </div>
                                            }
                                        } else {
                                            view! {
                                                <div class="sm:text-xl md:text-2xl  xl:text-6xl">
                                                    {&project_clone.name}
                                                </div>
                                            }
                                        }
                                    }
                                    }
                                </div>
                            </Card>
                        }
                    })
                    .collect_view()
            }}

        </div>
    }
}

#[component]
pub fn Features() -> impl IntoView {
    view! {
        <Layout id="features".to_string() aria_label="Features" class_name="flex-col".to_string()>
            <h2 class="text-5xl xs:text-6xl sm:text-7xl lg:text-8xl leading-smallheading sm:leading-mediumheading tracking-smallheading sm:tracking-heading">
                <div class="animated-title">
                    <span class="animated-title-element text-nf-white font-bold uppercase ">
                        Featured
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element font-light text-nf-white uppercase">
                        work
                    </span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element text-nf-white font-bold uppercase">
                        experience
                    </span>
                </div>
                <br/>
                <div class="animated-title">
                    <span class="animated-title-element text-nf-white font-bold uppercase">
                        and
                    </span>
                </div>
                {' '}
                <div class="animated-title">
                    <span class="animated-title-element font-light text-nf-white uppercase">
                        projects
                    </span>
                </div>
            </h2>
            <FeaturedCards/>
        </Layout>
    }
}
