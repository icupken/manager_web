use ehttp::Request;
use leptonic::prelude::*;
use leptos::*;
use leptos_use::{storage::use_session_storage, utils::FromToStringCodec};
use tracing::info;
use uuid::Uuid;

#[component]
pub fn Welcome() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (login, set_login) = create_signal("".to_owned());
    let (passwd, set_passwd) = create_signal("".to_owned());
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
                    get=login
                    set=set_login
                    style="width: 337px; height: 48px"
                    placeholder="Логин"
                />
                <PasswordInput
                    get=passwd
                    set=set_passwd
                    style="width: 337px; height: 48px"
                    placeholder="Пароль"
                />
                <Button
                    class="btn"
                    on_click=move |_| {
                        let login_str = login.get();
                        let passwd_str = passwd.get();
                        let sha_str = login_str + &passwd_str;
                        let token = sha256::digest(sha_str);
                        let request = Request {
                            headers: ehttp::Headers::new(&[("token", &token)]),
                            ..Request::get("https://mso.aquanter.ru:8000/auth")
                        };
                        ehttp::fetch(
                            request,
                            move |result: ehttp::Result<ehttp::Response>| {
                                let toasts = expect_context::<Toasts>();
                                if let Ok(answ) = result {
                                    if answ.status == 200 {
                                        set_flag.update(|f| *f = true);
                                    } else {
                                        toasts
                                            .push(Toast {
                                                id: Uuid::new_v4(),
                                                created_at: time::OffsetDateTime::now_utc(),
                                                variant: ToastVariant::Error,
                                                header: "Ошибка авторизации!".into_view(),
                                                body: "Неверный логин или пароль."
                                                    .into_view(),
                                                timeout: ToastTimeout::DefaultDelay,
                                            });
                                    }
                                } else {
                                    toasts
                                        .push(Toast {
                                            id: Uuid::new_v4(),
                                            created_at: time::OffsetDateTime::now_utc(),
                                            variant: ToastVariant::Error,
                                            header: "Ошибка авторизации!".into_view(),
                                            body: "Сервер недоступен!".into_view(),
                                            timeout: ToastTimeout::DefaultDelay,
                                        });
                                }
                            },
                        );
                    }
                >

                    "Войти"
                </Button>
            </Box>
        </Box>
    }
}
