// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use std::{error::Error, fmt::format, string};

use std::{error::Error};

slint::include_modules!();

const TAXPER: f64 = 0.10;
const NEEDPER: f64 = 0.40;
const WANTPER: f64 = 0.30;
const INVESTPER: f64 = 0.20;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move | string | {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let tax: f64 = num * TAXPER;
            let needs: f64 = num * NEEDPER;
            let wants: f64 = num * WANTPER;
            let invest: f64 = num * INVESTPER;
            let result = format!("Taxes: {:.2}\nNeeds: {:.2}\nWants: {:.2}\nInvest: {:.2}", {tax}, {needs}, {wants}, {invest});
            ui.set_results(result.into());
        }
    });

    ui.run()?;

    Ok(())
}
