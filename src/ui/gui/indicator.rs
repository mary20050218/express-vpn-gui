use libappindicator::{AppIndicator, AppIndicatorStatus};
use std::path::PathBuf;
use gtk::*;

pub struct Indicator {
    app_indicator: AppIndicator,
    path_to_images: PathBuf,
}

impl Indicator {
    pub fn new() -> Self {
        let mut image_path = dirs::data_dir().unwrap();
        image_path.push("express-vpn-gui/logo.png");

        let mut app_indicator = AppIndicator::new(
            "express-vpn-gui",
            image_path.to_str().unwrap()
        );

        app_indicator.set_status(AppIndicatorStatus::APP_INDICATOR_STATUS_ACTIVE);

        let mut path_to_images = dirs::data_dir().expect("System data dir couldn't be fetched.");
        path_to_images.push("express-vpn-gui/");

        Indicator{app_indicator, path_to_images}
    }

    pub fn append_menu(&mut self, mut menu: Menu) {
        menu.show_all();
        &self.app_indicator.set_menu(&mut menu);
    }

    pub fn change_icon(&mut self, image_name: &str) {
        let mut path_to_images = self.path_to_images.clone();
        path_to_images.push(image_name);

        self.app_indicator.set_icon_full(
            path_to_images.to_str().expect("Path couldn't be converted to string."),
            ""
        );
    }
}