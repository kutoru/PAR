use druid::commands::CLOSE_WINDOW;
use druid::widget::{Label, Flex, TextBox};
use druid::{WidgetExt, Widget, WindowDesc, EventCtx};
use crate::data::AppData;
use crate::info_win::{open_info_window, close_info_window, InfoWindowType};
use crate::ui_globals::ui::{self, create_button};

fn create_titlebar() -> impl Widget<AppData> {
    let bw = (ui::SETTINGS_WINDOW_WIDTH as u32) - 10;
    let bh = 40;
    let fs = 30;

    let label_window_info = create_button(
        Label::new("Settings"),
        bw, bh, fs, true)
        .on_added(|_, ctx, _, _| {
            unsafe { ui::setting_window_id = Some(ctx.window_id()) }
        });

    label_window_info
}

fn create_settings_container(data: &AppData) -> impl Widget<AppData> {
    let bw = (ui::SETTINGS_WINDOW_WIDTH as u32) - 10;
    let bh = 40;
    let fs = 30;

    let label_token = create_button(
        Label::new("pixiv refresh token:"),
        bw, bh, fs, false);
    let text_token = TextBox::new()
        .with_placeholder(data.temp_token.clone())
        .with_font(ui::FONT)
        .with_text_size(fs as f64)
        .lens(AppData::temp_token)
        .fix_size(bw as f64, bh as f64);

    let label_amount_to_search = create_button(
        Label::new("Amount to search:"),
        bw, bh, fs, false);
    let text_amount_to_search = TextBox::new()
        .with_placeholder(data.temp_amount_to_search.clone())
        .with_font(ui::FONT)
        .with_text_size(fs as f64)
        .lens(AppData::temp_amount_to_search)
        .fix_size(bw as f64, bh as f64);

    let label_timezone = create_button(
        Label::new("Timezone:"),
        bw, bh, fs, false);
    let text_timezone = TextBox::new()
        .with_placeholder(data.temp_timezone.clone())
        .with_font(ui::FONT)
        .with_text_size(fs as f64)
        .lens(AppData::temp_timezone)
        .fix_size(bw as f64, bh as f64);

    Flex::column()
        .with_child(label_token)
        .with_child(text_token)
        .with_child(label_amount_to_search)
        .with_child(text_amount_to_search)
        .with_child(label_timezone)
        .with_child(text_timezone)
}

fn create_function_container(data: &AppData) -> impl Widget<AppData> {
    let bw = (ui::SETTINGS_WINDOW_WIDTH as u32) - 10;
    let bh = 40;
    let fs = 30;

    let button_jump_to_artist = create_button(
        Label::new(|data: &AppData, _: &_| { format!("Jump to artist {}", {
            if data.jump_index_is_valid() { &data.jump_index }
            else { "" }
        })}),
        bw - (bw / 5), bh, fs, false)
        .on_click(|ctx, data, _| {
            if data.jump_index_is_valid() { open_info_window(ctx, data, InfoWindowType::ConfirmJump) }
        });
    let text_jump_to_artist = TextBox::new()
        .with_placeholder(data.jump_index.clone())
        .with_font(ui::FONT)
        .with_text_size(fs as f64)
        .lens(AppData::jump_index)
        .fix_size((bw / 5) as f64, bh as f64);

    let button_reload_artist_list = create_button(
        Label::new("Reload the artist list"),
        bw, bh, fs, false)
        .on_click(|ctx, data, _| {
            if !data.requires_initialization {
                open_info_window(ctx, data, InfoWindowType::ConfirmListReload);
            }
        });

    Flex::column()
        .with_child(
            Flex::row()
                .with_child(button_jump_to_artist)
                .with_child(text_jump_to_artist)
        )
        .with_child(button_reload_artist_list)
}

fn create_footer() -> impl Widget<AppData> {
    let bw = ((ui::SETTINGS_WINDOW_WIDTH as u32) - 10) / 2;
    let bh = 40;
    let fs = 30;

    let button_cancel = create_button(
        Label::new("Cancel"),
        bw, bh, fs, true)
        .on_click(|ctx, data, _| {
            close_settings_window(ctx, data);
        });
    let button_save = create_button(
        Label::new("Save"),
        bw, bh, fs, true)
        .on_click(|ctx, data, _| {
            if data.temp_token_has_changed() { open_info_window(ctx, data, InfoWindowType::ConfirmTokenChange) }
            else { data.apply_settings(); close_settings_window(ctx, data); }
        });

    Flex::row()
        .with_child(button_cancel)
        .with_child(button_save)
}

fn build_ui(data: &AppData) -> impl Widget<AppData> {
    let titlebar = create_titlebar();
    let settings_container = create_settings_container(data);
    let function_container = create_function_container(data);
    let footer = create_footer();

    Flex::column()
        .with_child(titlebar)
        .with_spacer(5.0)
        .with_child(settings_container)
        .with_spacer(5.0)
        .with_child(function_container)
        .with_spacer(5.0)
        .with_child(footer)
        .padding(5.0)
}

fn create_settings_window(data: &AppData) -> WindowDesc<AppData> {
    WindowDesc::new(build_ui(data))
        .window_size((ui::SETTINGS_WINDOW_WIDTH, ui::SETTINGS_WINDOW_HEIGHT))
        .set_position(unsafe { ui::SETTINGS_WINDOW_POS })
        .resizable(false)
        .show_titlebar(false)
        .title("PAR - Settings")
}

pub fn close_settings_window(ctx: &mut EventCtx, data: &mut AppData) {
    if data.settings_window_is_open {
        unsafe {
            data.cancel_settings();
            close_info_window(ctx, data);

            ctx.submit_command(CLOSE_WINDOW.to(ui::setting_window_id.unwrap()));
            data.settings_window_is_open = false;
            ui::setting_window_id = None;
        }
    }
}

pub fn open_settings_window(ctx: &mut EventCtx, data: &mut AppData) {
    close_settings_window(ctx, data);
    data.jump_index = (data.artist_index+1).to_string();
    ctx.new_window(create_settings_window(data));
    data.settings_window_is_open = true;
}
