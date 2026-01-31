// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc, time::Duration};

use slint::VecModel;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    use slint::Model;
    let main_window = AppWindow::new()?;

    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    // duplicating the tiles
    tiles.extend(tiles.clone());

    use rand::seq::SliceRandom;
    let mut rng = rand::rng();
    tiles.shuffle(&mut rng);

    let tiles_model = Rc::new(VecModel::from(tiles));

    main_window.set_memory_tiles(tiles_model.clone().into());

    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let mut flipped_tiles = tiles_model
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let solved = t1.image == t2.image;
            if solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);

                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);

                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);

                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    main_window.run()?;

    Ok(())
}
