#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()>{



    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "EazyBanking",
        native_options,
        Box::new(|cc| Box::new(easy_banking::EazyBanking::new(cc))),
    )

}


