use super::{point::Point, AppData, Contact, Drawable, Privileges};
pub use crate::App;
use eframe::egui;
use egui_extras::{Size, TableBuilder};
use reqwest_wasm::{header::AUTHORIZATION, StatusCode};
use rmp_serde as rmps;

#[derive(Debug, Deserialize)]
pub struct Firm {
    id: i32,
    name: String,
    contact: Contact,
    owner: Contact,
    privileges: Privileges,
}
pub struct FirmWin {
    name: String,
}
impl FirmWin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drawable for FirmWin {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn redraw(&mut self, ctx: &egui::Context, data: &mut AppData) {
        egui::Area::new(self.name())
            .fixed_pos(egui::pos2(200.0, 10.0))
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Size::exact(20.0))
                    .column(Size::exact(120.0))
                    .column(Size::exact(200.0))
                    .column(Size::exact(200.0))
                    .column(Size::exact(120.0))
                    .striped(true)
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("id");
                        });
                        header.col(|ui| {
                            ui.heading("Название");
                        });
                        header.col(|ui| {
                            ui.heading("Контакты");
                        });
                        header.col(|ui| {
                            ui.heading("Владелец");
                        });
                        header.col(|ui| {
                            ui.heading("Доступ");
                        });
                    })
                    .body(|mut body| {
                        for f in &data.firms {
                            body.row(80.0, |mut row| {
                                row.col(|ui| {
                                    if ui.button(format!("{}", f.id)).clicked() {
                                        let client = reqwest_wasm::blocking::Client::new();
                                        let resp = client
                                            .get(format!("http://srv04.elpi-tech.ru:25000/firms/{}/points",
                                                f.id
                                            ))
                                            .header(AUTHORIZATION, &data.auth_token)
                                            .send();
                                        match resp {
                                            Ok(resp) => match resp.status() {
                                                StatusCode::OK => {
                                                    let point_raw = resp.bytes().unwrap().to_vec();
                                                    let point_info: Vec<Point> =
                                                        rmps::decode::from_slice(&point_raw)
                                                            .expect("error with decode");
                                                    data.points = point_info;
                                                    data.win_req = Some(String::from("points"));
                                                }
                                                other_status => {
                                                    println!(
                                                        "request fail, status_code: {:?}",
                                                        other_status
                                                    );
                                                    println!("{}", std::str::from_utf8(&resp.bytes().unwrap().to_vec()).unwrap());
                                                }
                                            },
                                            Err(e) => println!("{}", e),
                                        }
                                    }
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", f.name));
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{} {} : {}",
                                        f.contact.name, f.contact.last_name, f.contact.phone
                                    ));
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "{} {} : {}",
                                        f.contact.name, f.contact.last_name, f.contact.phone
                                    ));
                                });
                                row.col(|ui| {
                                    ui.label(format!(
                                        "manage_firm: {}, manage_points: {}",
                                        f.privileges.manage_firm, f.privileges.manage_points
                                    ));
                                });
                            });
                        }
                    });
            });
    }

    fn request(&mut self, data: &mut AppData) {
        let client = reqwest_wasm::blocking::Client::new();
        let resp = client
            .get("http://srv04.elpi-tech.ru:25000/firms")
            .header(AUTHORIZATION, &data.auth_token)
            .send();
        match resp {
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let firm_raw = resp.bytes().unwrap().to_vec();
                    let firm_info: Vec<Firm> =
                        rmps::decode::from_slice(&firm_raw).expect("error with decode");
                    data.firms = firm_info;
                }
                other_status => {
                    println!("request fail, status_code: {:?}", other_status);
                }
            },
            Err(e) => println!("{}", e),
        }
    }
}
