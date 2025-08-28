use leptos::prelude::*;
use leptos_use::use_interval_fn;
use rand::random;

#[component]
pub fn Skills() -> impl IntoView {
    let technologies = [
        "Rust",
        "Axum",
        "Leptos",
        "Bevy",
        "PostgreSQL",
        "Java",
        "Spring",
        "Vue",
        "Angular",
        "PHP",
        "Javascript",
        "C",
        "C++",
        "OpenGL",
        "Vulkan",
        "Linux",
        "MySQL",
        "C#",
        ".NET",
        "Git",
        "CI/CD",
        "Docker",
    ];

    let (highlighted, write_highlighted) = signal(vec![false; technologies.len()]);

    let _ = use_interval_fn(
        move || {
            let new_highlighted = (0..highlighted.get_untracked().len())
                .map(|index| {
                    if index % 2 == 0 {
                        random::<bool>()
                    } else {
                        false
                    }
                })
                .collect::<Vec<_>>();

            write_highlighted(new_highlighted);
        },
        750,
    );

    view! {
        <div class="grid lg:grid-cols-9 lg:grid-flow-col gap-x-12 lg:gap-y-0 fade-in w-full">
            <div class="lg:col-span-2">
                <div class="text-md lg:text-lg leading-about text-nf-white uppercase">
                    <span class="uppercase">"Skills —"</span>
                </div>

            </div>
            <div class="lg:col-span-5 py-5 lg:py-10 border-solid border-t border-b border-nf-white flex flex-wrap gap-x-4 gap-y-2 lg:gap-x-10 lg:gap-y-4 min-w-full text-xl lg:text-5xl leading-p lg:leading-largep font-[400]">
                {move || {
                    technologies
                        .iter()
                        .enumerate()
                        .map(|(index, tech)| {
                            let is_highlighted = highlighted.get()[index];
                            let class = if is_highlighted {
                                "text-nf-color drop-shadow-[0_0_5px_#047857] drop-shadow-[0_0_15px_#047857] drop-shadow-[0_0_30px_#047857]"
                            } else {
                                "text-nf-white"
                            };
                            view! { <span class=move || class>{tech.to_string()}</span> }
                        })
                        .collect::<Vec<_>>()
                }}

            </div>
        </div>
    }
}
