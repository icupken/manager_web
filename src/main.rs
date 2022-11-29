#![allow(dead_code)]

use eframe::egui;
use frames::{profile, Frame, AppData, auth, firm, point_reg, point, point_edit};
pub mod frames;

#[macro_use]
extern crate serde_derive;
extern crate rmp_serde;

#[cfg(target_arch = "wasm32")]
fn main() {
        // Make sure panics are logged using `console.error`.
        console_error_panic_hook::set_once();

        // Redirect tracing to console.log and friends:
        tracing_wasm::set_as_global_default();
    
        let web_options = eframe::WebOptions::default();
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(App::new(cc))),
        )
        .expect("failed to start eframe");
}

pub struct App {
    pub wins: Vec<Frame>,
    pub data: AppData,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            data: AppData {
                win_req: None,
                login: String::new(),
                passw: String::new(),
                user: None,
                firms: Vec::new(),
                points: Vec::new(),
                points_reg: Vec::new(),
                auth_token: String::new(),
                authorization: true,
            },
            wins: vec![
                Frame::new(Box::new(auth::AuthWin::new("auth"))),
                Frame::new(Box::new(profile::ProfileWin::new("profile"))),
                Frame::new(Box::new(firm::FirmWin::new("firms"))),
                Frame::new(Box::new(point_reg::PointRegWin::new("point_reg"))),
                Frame::new(Box::new(point::PointWin::new("points"))),
                Frame::new(Box::new(point_edit::PointEditWin::new("point_edit")))
            ],
        }
    }

    pub fn draw_content(&mut self, ctx: &egui::Context) {
        match self.data.win_req.take() {
            Some(req) => self.open_exact(&req),
            None => ()
        }

        for win in &mut self.wins {
            if win.is_open() {
                win.frame.redraw(ctx, &mut self.data);
            }
        }
    }

    /// Open specefied by name window, and close others
    pub fn open_exact(&mut self, frame: &str) {
        for win in &mut self.wins {
            if win.frame.name() == frame {
                win.open(&mut self.data);
            } else {
                win.close();
            }
        }
    }

    pub fn open(&mut self, frame: &str) {
        for win in &mut self.wins {
            if win.frame.name() == frame {
                win.open(&mut self.data);
                break;
            }
        }
    }

    pub fn close(&mut self, frame: &str) {
        for win in &mut self.wins {
            if win.frame.name() == frame {
                win.close();
                break;
            }
        }
    }
}


impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.data.authorization {
            self.open("auth");
        } else {
            self.close("auth");
            egui::CentralPanel::default().show(ctx, |_ui| {
                egui::SidePanel::left("left")
                    .resizable(false)
                    .show(ctx, |ui| {
                        if ui.button("Список фирм").clicked(){
                            self.open_exact("firms")
                        }
                        if ui.button("Регистрация точек").clicked() {
                            self.open_exact("point_reg")
                        }
                        if ui.button("Статус конкретной точки").clicked() {
                            //что-то вроде id_firm+ id_point
                        }
                        if ui.button("Добавление пользователя").clicked() {}
                        if ui.button("Просмотр списка программ").clicked() {}
                        if ui.button("Личный кабинет").clicked() {
                            self.open_exact("profile");
                        }
                    });
            });
        }
        self.draw_content(ctx);
    }
}
