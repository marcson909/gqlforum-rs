pub mod graphql;

use serde::{Deserialize, Serialize};
use sycamore::{prelude::*, suspense::Suspense};
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

use crate::graphql::GraphQLClient;

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/topic/<id>/<page>")]
    Topic { id: i64, page: usize },
    #[to("/user/<id>")]
    User { id: i64 },
    #[to("/test")]
    Test,
    #[not_found]
    NotFound,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    role: String,
}

#[component]
async fn TestGql<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let resp1 = client
        .query_raw(
            r#"
    query {
        user(by: {id: 1}) 
        {
            id 
            name 
            role 
        } 
    }
    "#,
        )
        .await
        .unwrap();
    let resp2 = client.query_raw("{ asdfdasf }").await.unwrap();
    view! { cx,
        p {
            "Response: " (format!("{:?}",resp1))
        }
        p {
            "Error: " (format!("{:?}", resp2))
        }
    }
}

#[component]
async fn TestAsync<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        p { "Hello from async!" }
    }
}

#[component]
fn TestApp<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        p { "Hello, World!" }
        Suspense {
            fallback: view! { cx, "Async..." },
            TestAsync {}
        }
        Suspense {
            fallback: view! { cx, "Loading..." },
            TestGql {}
        }
    }
}

#[component]
fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = GraphQLClient::new("/graphql");
    provide_context(cx, client);
    view! { cx,
        Router {
            integration: HistoryIntegration::new(),
            view: |cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="app") {
                        (match route.get().as_ref() {
                            AppRoutes::Index => view! { cx, "Stub index"},
                            AppRoutes::Topic{ .. } => view! { cx, "Stub topic"},
                            AppRoutes::User{ .. } => view! {cx, "Stub user"},
                            AppRoutes::Test => view! { cx, TestApp {}},
                            AppRoutes::NotFound => view! { cx, "404 Not Found"}
                        })
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| {
        view! { cx, App {} }
    });
}
