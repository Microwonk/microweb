use gloo_net::http::Request;
use leptos::*;
use serde::Deserialize;

// Define a struct to deserialize the GitHub API response
#[derive(Deserialize, Debug, Clone)]
struct Repository {
    full_name: String,
    description: Option<String>,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
}

#[component]
pub fn GithubRepository(owner: String, repo: String) -> impl IntoView {
    // Create a signal to hold the repository data
    let (repository, set_repository) = create_signal(None);

    // Create an effect to fetch the repository data when the component mounts
    create_effect(move |_| {
        let owner = owner.clone();
        let repo = repo.clone();

        spawn_local(async move {
            match fetch_repository(&owner, &repo).await {
                Ok(repo) => set_repository(Some(repo)),
                Err(_) => log::error!("Failed to fetch repository data"),
            }
        });
    });

    // Conditionally render the repository data or a loading message
    view! {
        <div class="text-nf-white font-montserrat mt-10 gap-8 md:gap-10 lg:gap-20 fade-y-trans">
            {move || {
                if let Some(repo) = repository.get() {
                    view! {
                        <a href=format!("https://github.com/{}", &repo.full_name) target="_blank"
                           class="flex flex-col hover:bg-nf-color hover:text-nf-dark rounded-md p-3 border-dashed border-2 border-nf-color">
                            <h2>{ &repo.full_name }</h2>
                            <p>{ repo.description.unwrap_or("No description available.".into()) }</p>
                            <p>"Stars: "{ repo.stargazers_count }</p>
                            <p>"Forks: "{ repo.forks_count }</p>
                            <p>"Open Issues: "{ repo.open_issues_count }</p>
                        </a>
                    }
                } else {
                    view! {
                        <a>
                            <p>"Loading repository data..."</p>
                        </a>
                    }
                }
            }}
        </div>
    }
}

// Function to fetch repository data from GitHub API
async fn fetch_repository(owner: &str, repo: &str) -> Result<Repository, ()> {
    let url = format!("https://api.github.com/repos/{}/{}", owner, repo);

    let response = Request::get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Leptos-GitHub-Component")
        .send()
        .await
        .map_err(|_| ())?;

    response.json::<Repository>().await.map_err(|_| ())
}
