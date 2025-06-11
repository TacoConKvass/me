use leptos::prelude::*;
use leptos::mount::mount_to_renderer;
use std::vec::Vec;
use miniserde::{json, Deserialize};

const standalone_list_json: &str = include_str!("standalone.json");

fn main() {
    console_error_panic_hook::set_once();
    
    let projects = document().get_element_by_id("projects").expect("projects not found");
    let projects_owner = mount_to_renderer(&projects, Projects);
    projects_owner.forget();
}

#[allow(non_snake_case)]
fn Projects() -> impl IntoView {
    let projects = json::from_str::<Vec<Project>>(standalone_list_json).unwrap();
    
    view! {
        {move || {
            let mut output = Vec::new();

            for project in projects.clone() {
                output.push(view! {
                    <div class="project" tabindex=0>
                        <img class="title" src={ project.image.clone() }/>
                        <p class="title">{ project.name.clone() }</p>
                        <div class="description">{ project.description.clone() }</div>
                        <div class="links">
                            { move || {
                                if project.github.clone() != "" {
                                    return view! {
                                        <a href={project.github.clone()}>
                                            <img src="https://upload.wikimedia.org/wikipedia/commons/9/95/Font_Awesome_5_brands_github.svg" width="25px" style="filter: invert(100%);"/>
                                        </a>
                                    }.into_any();
                                }

                                return view! { }.into_any();
                            }}
                            { move || {
                                if !project.website.clone().is_empty() {
                                    return view! {
                                        <a href={project.website.clone()}>
                                            <img src="https://www.svgrepo.com/show/478273/internet-3.svg" width="25px" style="filter: invert(100%);"/>
                                        </a>
                                    }.into_any();
                                }
                                
                                return view! { }.into_any();
                            }}
                            { move || {
                                if project.steam.clone() != "" {
                                    return view! {
                                        <a href={project.steam.clone()}>
                                            <img src="https://www.svgrepo.com/show/314707/steam.svg" width="30px" style="filter: invert(100%);"/>
                                        </a>
                                    }.into_any();
                                }

                                return view! { }.into_any();
                            }}
                            { move|| {
                                let mut out = Vec::new();
                                
                                if let Some(icons) = project.icons.clone() {
                                    for element in &icons {
                                        out.push(view! { <img src={element} width="25px"/> }.into_any())
                                    }
                                }

                                out
                            }}
                        </div>
                    </div>
                }.into_any())
            }

            output
        }}
    }
}

#[derive(Deserialize, Clone)]
struct Project {
    name: String,
    description: String,
    github: String,
    website: String,
    steam: String,
    image: String,
    icons: Option<Vec<String>>,
}
