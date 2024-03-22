use leptonic::prelude::*;
use leptos::*;
use leptos_use::{storage::use_session_storage, utils::FromToStringCodec};

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (text, set_text) = create_signal("".to_owned());
    let (flag, set_flag, remove_flag) = use_session_storage::<bool, FromToStringCodec>("my-flag");
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
                <H2>"Авторизация"</H2>
                <TextInput
                    get=text
                    set=set_text
                    style="width: 337px; height: 48px"
                    placeholder="Логин"
                />
                <PasswordInput
                    get=text
                    set=set_text
                    style="width: 337px; height: 48px"
                    placeholder="Пароль"
                />
                <Button class="btn" on_click=move |_| { set_flag.update(|f| *f = true) }>
                    "Войти"
                </Button>
            </Box>
        </Box>
    }
}
