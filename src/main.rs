use druid::{AppLauncher, theme};
use ui_globals::ui;
use main_win::create_main_window;
use crate::file_handler::{prepare_settings, prepare_artists, prepare_folders};

mod data;
mod ui_globals;
mod main_win;
mod settings_win;
mod info_win;
mod pixiv_handler;
mod file_handler;

fn main() {
    println!("Start");

    let mut initial_data = prepare_settings();
    prepare_artists(&mut initial_data);
    prepare_folders();

    ui::initialize_window_positions(false);
    let main_window = create_main_window();

    AppLauncher::with_window(main_window)
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, ui::ACTIVE_COLOR)
        })
        //.log_to_console()
        .launch(initial_data)
        .expect("Failed to launch the app");

    println!("End");
}
