use eframe::egui;
pub mod auth;
pub mod profile;
pub mod firm;
pub mod point;
pub mod point_reg;
pub mod point_edit;

pub struct Frame {
	pub is_open: bool,
	pub frame: Box<dyn Drawable>
}

pub struct AppData {
	pub win_req: Option<String>,
    pub login: String,
    pub passw: String,
    pub user: Option<auth::User>,
    pub auth_token: String,
    pub firms: Vec<firm::Firm>,
    pub points: Vec<point::Point>,
    pub points_reg: Vec<point_reg::PointRegReq>,
    pub authorization: bool,
}

impl AppData {
	pub fn req_win(&mut self, name: &str) {
		self.win_req = Some(String::from(name))
	}
}


impl Frame {
	pub fn new(frame: Box<dyn Drawable>) -> Self {
		Self {
			is_open: false,
			frame: frame
		}
	}

	pub fn open(&mut self, data: &mut AppData) {
		self.frame.request(data);
		self.is_open = true;
	}

	pub fn close(&mut self) {
		self.is_open = false;
	}

	pub fn is_open(&self) -> bool {
		self.is_open
	}

	// and any generic control functions
}

pub trait Drawable {
    /// `&'a` so we can also use it as a key to store open/close state.
	fn name<'a>(&'a self) -> &'a str;

	/// init window - load and handle data for view
	fn request(&mut self, data: &mut AppData);

	/// Show window, etc
	fn redraw(&mut self, ctx: &egui::Context, data: &mut AppData);
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Contact {
    pub name: String,
    pub last_name: String,
    pub mid_name: String,
    pub phone: String,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Privileges {
	manage_firm: bool,
	manage_points: bool
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Location {
	id: i32,
	country: String,
	city: String,
	street: String,
	building: String,
	latitude: f64,
	longitude: f64
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum PointStatus {
	Ok,
	Warning,
	Error,
	Disconnect
}