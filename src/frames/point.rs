use super::{Drawable, AppData};
use super::{Contact, Location, PointStatus};
pub use crate::App;
use chrono::NaiveDateTime;
use eframe::egui::{self, RichText};
use eframe::epaint::Color32;
use egui_extras::{Size, TableBuilder};


#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Point {
	pub id: i32,
	firm_id: i32,
	pub name: String,
	platform: String,
	ptype: i16,
	pub contact: Contact,
	pub location: Location,
	pub poll_period: i64,
	pub time_zone: i16,
	dt_last_req: i64,
	pub bin_path: String,
	pub ipc_dir: String,
	status: PointStatus
}

pub struct PointWin {
    name: String,
}

impl PointWin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drawable for PointWin {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn redraw(&mut self, ctx: &egui::Context, data: &mut AppData) {
        egui::Area::new("profile")
            .fixed_pos(egui::pos2(200.0, 10.0))
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Size::exact(100.0))
                    .column(Size::exact(250.0))
                    .column(Size::exact(150.0))
                    .column(Size::exact(150.0))
                    .striped(true)
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("point_id");
                        });
                        header.col(|ui| {
                            ui.heading("Название");
                        });
                        header.col(|ui| {
                            ui.heading("Статус");
                        });
                        header.col(|ui| {
                            ui.heading("last_req");
                        });
                    })
                    .body(|mut body| {
                        for p in &data.points {
                            body.row(80.0, |mut row| {
                                row.col(|ui| {
                                    if ui.button(format!("{}", p.id)).clicked() {
                                        data.win_req = Some(String::from("point_edit"));
                                    }
                                });
                                row.col(|ui| {
                                    ui.label(format!("{}", p.name));
                                });
                                row.col(|ui| {
                                    match p.status {
                                        PointStatus::Ok => {
                                            ui.label(RichText::new("Ok").color(Color32::GREEN).size(26.0));
                                        },
                                        PointStatus::Warning => {
                                            ui.label(RichText::new("Warning").color(Color32::YELLOW).size(26.0));
                                        },
                                        PointStatus::Disconnect => {
                                            ui.label(RichText::new("Disconnect").color(Color32::BLACK).size(26.0));
                                        }
                                        PointStatus::Error => {
                                            ui.label(RichText::new("ERROR!").color(Color32::RED).size(26.0));
                                        }
                                    }
                                });
                                row.col(|ui| {
                                    let dt = NaiveDateTime::from_timestamp_opt(p.dt_last_req, 0);
                                    ui.label(format!("{}", dt.unwrap().time()));
                                    ui.label(format!("{}", dt.unwrap().date()));
                                });
                            });
                        }
                    });
            });
    }

    fn request(&mut self, _data: &mut AppData) {
    }
}
