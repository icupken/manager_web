use super::Drawable;
use rmp_serde as rmps;
use eframe::egui;
use reqwest_wasm::{header::AUTHORIZATION};

pub struct PointEditWin {
    name: String,
}

impl PointEditWin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drawable for PointEditWin {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn request(&mut self, _data: &mut super::AppData) {}

    fn redraw(&mut self, ctx: &eframe::egui::Context, data: &mut super::AppData) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("prof_edit")
                .min_col_width(200.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("id");
                    ui.add(egui::TextEdit::singleline(&mut data.points[0].id.to_string()));
                    ui.end_row();

                    ui.label("Имя");
                    ui.add(egui::TextEdit::singleline(&mut data.points[0].name));
                    ui.end_row();

                    egui::CollapsingHeader::new("Location")
                    .default_open(true)
                    .show(ui, |ui| {
                    egui::Grid::new("location_edit")
                        .min_col_width(150.0)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Страна");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].location.country,
                            ));
                            ui.end_row();
                            ui.label("Город");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].location.city,
                            ));
                            ui.end_row();
                            ui.label("Улица");

                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].location.street,
                            ));
                            ui.end_row();
                            ui.label("Строение");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].location.building,
                            ));
                            ui.end_row();
                            ui.label("Широта");
                            ui.add(egui::DragValue::new(&mut data.points[0].location.latitude));
                            ui.end_row();
                            ui.label("Долгота");
                            ui.add(egui::DragValue::new(&mut data.points[0].location.longitude));
                            ui.end_row();
                        });
                    });
                    ui.label(" ");
                    ui.end_row();
                    


                    egui::CollapsingHeader::new("Contacts")
                    .default_open(true)
                    .show(ui, |ui| {
                    egui::Grid::new("conctact_edit")
                        .min_col_width(150.0)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Фамилия");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].contact.last_name,
                            ));
                            ui.end_row();

                            ui.label("Имя");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].contact.name,
                            ));
                            ui.end_row();

                            ui.label("Отчество");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].contact.mid_name,
                            ));
                            ui.end_row();


                            ui.label("Телефон");
                            ui.add(egui::TextEdit::singleline(
                                &mut data.points[0].contact.phone,
                            ));
                            ui.end_row();
                        });
                    });
                    ui.end_row();
                    ui.label("частота обновления");
                    ui.add(egui::DragValue::new(&mut data.points[0].poll_period));
                    ui.end_row();

                    ui.label("time_zone");
                    ui.add(egui::DragValue::new(&mut data.points[0].time_zone));
                    ui.end_row();

                    ui.label("bin_path");
                    ui.add(egui::TextEdit::singleline(&mut data.points[0].bin_path));
                    ui.end_row();

                    ui.label("ipc_dir");
                    ui.add(egui::TextEdit::singleline(&mut data.points[0].ipc_dir));
                    ui.end_row();

                    if ui.button("update").clicked() {
                        let client = reqwest_wasm::blocking::Client::new();
                        let _req = client
                            .put("http://srv04.elpi-tech.ru:25000/pointUpd/{0}")
                            .body(rmps::encode::to_vec(&data.points[0]).unwrap())
                            .header(AUTHORIZATION, &data.auth_token)
                            .send();
                    }
                });
        });
    }
}
