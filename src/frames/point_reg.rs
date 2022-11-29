use super::{AppData, Drawable};
use crate::frames::point_reg;
pub use crate::App;
use eframe::egui::{self, Color32};
use egui_extras::{Size, TableBuilder};
use reqwest_wasm::{header::AUTHORIZATION, StatusCode};
use rmp_serde as rmps;

#[derive(Debug, Serialize, Deserialize)]
pub struct PointRegReq {
    id: i32,
    name: String,
    firm: Option<String>,
    ip: String,
    closed: bool,
    dt_last_req: i64,
    point_id: Option<i32>,
}

#[derive(Deserialize)]
pub struct QueryOpt {
    limit: Option<i32>,
    offset: Option<i32>,
    closed: Option<bool>,
}

pub struct PointRegWin {
    name: String,
}

impl PointRegWin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drawable for PointRegWin {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn redraw(&mut self, ctx: &egui::Context, data: &mut AppData) {
        egui::Area::new("profile")
            .fixed_pos(egui::pos2(200.0, 10.0))
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Size::exact(20.0))
                    .column(Size::exact(150.0))
                    .column(Size::exact(150.0))
                    .column(Size::exact(150.0))
                    .column(Size::exact(150.0))
                    .column(Size::exact(150.0))
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
                            ui.heading("Имя фирмы");
                        });
                        header.col(|ui| {
                            ui.heading("Статус заявки");
                        });
                        header.col(|ui| {
                            ui.heading("Point_id");
                        });
                        header.col(|ui| {
                            ui.heading("");
                        });
                    })
                    .body(|mut body| {
                        for p in &mut data.points_reg {
                            body.row(80.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(format!("{}", p.id));
                                });
                                row.col(|ui| {
                                    ui.text_edit_singleline(&mut p.name);
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", p.firm.as_ref().unwrap()));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", p.closed));
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", p.point_id.as_ref().unwrap()));
                                });
                                // row.col(|ui| {
                                //     //todo indicate!
                                // });
                                row.col(|ui| {
                                    ui.horizontal(|ui| {
                                        if ui
                                            .add(egui::Button::new("Принять").fill(Color32::GREEN))
                                            .clicked()
                                        {
                                            let client = reqwest_wasm::blocking::Client::new();

                                            // let mut point_data = Vec::new();
                                            //     point_data.push(PointRegReq {
                                            //         id: p.id,
                                            //         name: p.name,
                                            //         firm: p.firm,
                                            //         ip: p.ip,
                                            //         closed: p.closed,
                                            //         dt_last_req: p.dt_last_req,
                                            //         point_id: p.point_id,
                                            //     });

                                            let resp = client
                                            .get(format!("http://srv04.elpi-tech.ru:25000/points_reg/{}/accept",
                                                p.id
                                            ))
                                            .header(AUTHORIZATION, &data.auth_token)
                                            //.body(rmps::encode::to_vec(&point_data).unwrap())
                                            .send();
                                        match resp {
                                            Ok(resp) => match resp.status() {
                                                StatusCode::OK => {
                                                    println!("point registered successfully!");
                                                }
                                                other_status => {
                                                    println!(
                                                        "accept fail, status_code: {:?}",
                                                        other_status
                                                    );
                                                }
                                            },
                                            Err(e) => println!("{}", e),
                                        }

                                        }
                                        if ui
                                            .add(egui::Button::new("Отклонить").fill(Color32::RED))
                                            .clicked()
                                        {
                                            //
                                        }
                                    });
                                });
                            });
                        }
                    });
            });
    }

    fn request(&mut self, data: &mut AppData) {
        let client = reqwest_wasm::blocking::Client::new();
        let resp = client
            .get("http://srv04.elpi-tech.ru:25000/points_reg")
            .header(AUTHORIZATION, &data.auth_token)
            .send();
        match resp {
            Ok(resp) => match resp.status() {
                StatusCode::OK => {
                    let point_reg_raw = resp.bytes().unwrap().to_vec();
                    let point_reg_info: Vec<point_reg::PointRegReq> =
                        rmps::decode::from_slice(&point_reg_raw).expect("error with decode");
                    data.points_reg = point_reg_info;
                }
                other_status => {
                    println!("request fail, status_code: {:?}", other_status);
                }
            },
            Err(e) => println!("{}", e),
        }
    }
}
