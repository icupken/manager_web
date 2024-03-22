use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn NeedAuth() -> impl IntoView {
    view! {
        <Box class="main-container">
            <Box class="card">
                <picture>
                    <source
                        srcset="https://raw.githubusercontent.com/icupken/manager_web/main/logo%20(1).png"
                        media="(prefers-color-scheme: dark)"
                    />
                    <img
                        src="https://raw.githubusercontent.com/icupken/manager_web/main/logo%20(1).png"
                        alt="Leptos Logo"
                        height="150"
                        width="150"
                    />
                </picture>
                <H2>"Необходима авторизация!"</H2>
                <H2>"Доступ ограничен!"</H2>
                <LinkButton href="/auth">"Back"</LinkButton>
            </Box>
        </Box>
    }
}
