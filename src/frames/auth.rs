use eframe::{egui, egui::RichText, emath::Align2, epaint::FontId};
use rmp_serde as rmps;
use reqwest::{header::AUTHORIZATION, StatusCode};
use super::{AppData, Drawable};

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: String,
    pub mid_name: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub is_admin: Option<bool>,
}

pub struct AuthWin {
    name: String,
}

impl AuthWin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drawable for AuthWin {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn redraw(&mut self, ctx: &egui::Context, data: &mut AppData) {
        egui::Window::new(self.name())
            .collapsible(false)
            .resizable(false)
            .title_bar(false)
            //.open(&mut false)
            .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    data.login = "icupken@ya.ru".to_owned();
                    ui.label(RichText::new("Логин").font(FontId::proportional(26.0)));
                    ui.add(egui::TextEdit::singleline(&mut data.login));
                    ui.label(RichText::new("Пароль").font(FontId::proportional(26.0)));
                    ui.add(egui::TextEdit::singleline(&mut data.passw).password(true));
                    if ui.button("Авторизоваться").clicked() {

                        //crutch
                        let str_for_hash = format!("{}:{}", data.login, data.passw);
                        data.auth_token = sha256::digest(str_for_hash);
                        let client = reqwest::blocking::Client::new();
                        let resp = client
                        .get("http://srv04.elpi-tech.ru:25000/self")
                        .header(AUTHORIZATION, &data.auth_token)
                        .send();
                        match resp {
                            Ok(resp) => match resp.status() {
                                StatusCode::OK => {
                                    let user_raw = resp.bytes().unwrap().to_vec();
                                    let user_info: User = rmps::decode::from_slice(&user_raw)
                                        .expect("error with decode");
                                    data.user = Some(user_info);
                                    data.authorization = false;
                                }
                                other_status => {
                                    // other
                                    println!("request fail, status_code: {:?}", other_status);
                                }
                            },
                            //errors with connect
                            Err(e) => println!("{}", e),
                        }
                        //
                    }
                });
            });
    }

    fn request(&mut self, _data: &mut AppData) {}
}
