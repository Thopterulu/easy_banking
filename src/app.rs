use eframe::{App, egui::{CentralPanel, Ui}};

pub struct EazyBanking {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member

    value: f32,
}


impl Default for EazyBanking {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}


impl EazyBanking {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl App for EazyBanking {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Personnal Finances");
            // if ui.button("check Account").clicked() { take_action(ui) }
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }

}

// fn take_action(ui: |ui|) {
//     ui.add(egui::github_link_file!(
//         "https://github.com/emilk/eframe_template/blob/master/",
//         "Source code."
//     ));
// }