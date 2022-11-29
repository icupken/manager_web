use super::{AppData, Drawable};
pub use crate::App;
use eframe::egui;
use egui_extras::{Size, TableBuilder};

pub struct ProfileWin {
    name: String,
}

impl ProfileWin {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Drawable for ProfileWin {
    fn name<'a>(&'a self) -> &'a str {
        &self.name
    }

    fn redraw(&mut self, ctx: &egui::Context, data: &mut AppData) {
        egui::Area::new("profile")
            .fixed_pos(egui::pos2(200.0, 10.0))
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Size::exact(120.0))
                    .column(Size::exact(120.0))
                    .body(|mut body| {
                        body.row(200.0, |mut row| {
                            row.col(|ui| {
                                ui.label("Имя");
                                ui.label("Фамилия");
                                ui.label("email");
                                ui.label("Телефон");
                                ui.label("is_admin?");
                                ui.label("Пароль");
                            });
                            row.col(|ui| {
                                ui.label(format!("{}", data.user.as_ref().unwrap().name));
                                ui.label(format!("{}", data.user.as_ref().unwrap().last_name));
                                ui.label(format!("{}", data.user.as_ref().unwrap().email));
                                ui.label(format!(
                                    "{}",
                                    data.user.as_ref().unwrap().phone.as_ref().unwrap()
                                ));
                                ui.label(format!(
                                    "{}",
                                    data.user.as_ref().unwrap().is_admin.as_ref().unwrap()
                                ));
                                if ui.button("change pass").clicked() {}
                            });
                        });
                    });
            });
    }
    fn request(&mut self, _data: &mut AppData) {}
}
