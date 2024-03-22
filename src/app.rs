use leptonic::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::*;
use leptos_use::{storage::use_session_storage, utils::FromToStringCodec};
use tracing::info;

use crate::{
    error_template::{AppError, ErrorTemplate},
    pages::{page::Page, welcome::Welcome},
};

#[derive(Copy, Clone, Debug, Default)]
pub struct GlobalState {
    pub auth: bool,
}

fn not_authorized() -> bool {
    let (flag, _set_flag, _remove_flag) = use_session_storage::<bool, FromToStringCodec>("my-flag");
    info!("{}", flag.get());
    flag.get()
}

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_rw_signal(GlobalState::default()));
    provide_meta_context();
    view! {
        <Meta name="charset" content="UTF-8"/>
        <Meta name="description" content="Leptonic CSR template"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Meta name="theme-color" content="#e66956"/>

        <Stylesheet id="leptos" href="/pkg/leptonic-template-ssr.css"/>
        <Stylesheet href="https://fonts.googleapis.com/css?family=Roboto&display=swap"/>

        <Title text="Leptonic CSR template"/>

        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! { <ErrorTemplate outside_errors/> }
            }>
                <Routes>
                    <Route
                        path="/"
                        view=move || {
                            view! {
                                // only show the outlet if data have loaded
                                <Show when=|| not_authorized() fallback=|| view! { <Welcome/> }>
                                    <Outlet/>
                                </Show>
                            }
                        }
                    >

                        // nested child route
                        <Route path="/" view=Page/>
                    </Route>
                </Routes>
            </Router>
        </Root>
    }
}
