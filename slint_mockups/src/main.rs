// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};
use rand::prelude::*;
use slint::VecModel;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let main_window = AppWindow::new()?;

    let mut rng = rand::rng();
    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    let model = Rc::new(VecModel::from(nums));
    main_window.set_amplitudes(model.clone().into());

    main_window.run()?;

    Ok(())
}
