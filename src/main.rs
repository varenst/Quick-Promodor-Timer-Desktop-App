// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();

    let mut is_timer_counting = false;

    ui.on_start_timer(move || {
        let ui = ui_handle.unwrap();
        
        is_timer_counting = !is_timer_counting;

        ui.set_istimerrunning(is_timer_counting);
    });

    ui.on_get_formatedTime(move |seconds| {
       let formattedseconds = seconds % 60;
       let formattedminutes = (seconds / 60) % 60;

       let final_string = format!("{:0>2}:{:0>2}", formattedminutes, formattedseconds);

       return final_string.into();
    });
    
    ui.run()?;

    Ok(())
}
