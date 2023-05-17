use druid::commands::CLOSE_WINDOW;
use druid::widget::{Label, Flex, LineBreaking};
use druid::{WidgetExt, Widget, WindowDesc, EventCtx};
use std::cmp::Ordering;
use crate::data::AppData;
use crate::pixiv_handler::reset_artist_list;
use crate::settings_win::close_settings_window;
use crate::ui_globals::ui::{self, create_button};

#[derive(Debug, Clone)]
/// Info window type
pub enum InfoWindowType {
    ConfirmTokenChange,
    ConfirmJump,
    ConfirmListReload,
}

fn create_info_label(bw: u32, bh: u32, fs: u32, win_type: InfoWindowType) -> impl Widget<AppData> {
    let label = match win_type {
        InfoWindowType::ConfirmTokenChange => {
            Label::new("You are about to change the token, that will erase all currently downloaded artist data. Are you sure you want to continue?")
        },
        InfoWindowType::ConfirmJump => {
            Label::new(|data: &AppData, _: &_| { match data.get_jump_index() {
                Some(new_index) => match new_index.cmp(&data.artist_index) {
                    Ordering::Greater => format!("You are about to jump from artist {} to artist {}, that will mark all artists before {} as reviewed. Are you sure you want to continue?", data.artist_index+1, new_index+1, new_index+1),
                    Ordering::Less => format!("You are about to jump from artist {} to artist {}, that will mark all artists after {} as not reviewed. Are you sure you want to continue?", data.artist_index+1, new_index+1, new_index+1),
                    Ordering::Equal => "Your jump number is the same as the current artist number. Clicking yes will not do much.".to_string(),
                },
                None => "You broke the jump index, congrats. As a reward, clicking yes will not do anything.".to_string(),
            }})
        },
        InfoWindowType::ConfirmListReload => {
            Label::new("You are about to reload the artist list, that will erase all currently downloaded artist data. Are you sure you want to continue?")
        },
    };

    create_button(
        label.with_line_break_mode(LineBreaking::WordWrap),
        bw, bh*5, fs, false)
        .on_added(|_, ctx, _, _| {
            unsafe { ui::info_window_id = Some(ctx.window_id()) }
        })
}

fn create_no_button(bw: u32, bh: u32, fs: u32) -> impl Widget<AppData> {
    create_button(
        Label::new("No"),
        bw/2, bh, fs, true)
        .on_click(|ctx, data, _| {
            close_info_window(ctx, data);
        })
}

fn create_yes_button(bw: u32, bh: u32, fs: u32, win_type: InfoWindowType) -> impl Widget<AppData> {
    create_button(
        Label::new("Yes"),
        bw/2, bh, fs, true)
        .on_click(move |ctx, data, _| {
            match win_type {
                InfoWindowType::ConfirmTokenChange => {
                    data.apply_settings();
                    close_settings_window(ctx, data);
                },
                InfoWindowType::ConfirmJump => match data.get_jump_index() {
                    Some(new_index) => match new_index.cmp(&data.artist_index) {
                        Ordering::Equal => {},
                        _ => {
                            data.jump_to_jump_index();
                            close_settings_window(ctx, data);
                        }
                    },
                    None => {},
                },
                InfoWindowType::ConfirmListReload => {
                    reset_artist_list(data);
                    close_settings_window(ctx, data);
                },
            };
        })
}

fn build_ui(win_type: InfoWindowType) -> impl Widget<AppData> {
    let bw = (ui::INFO_WINDOW_WIDTH as u32) - 10;
    let bh = 30;
    let fs = 20;

    let label_info = create_info_label(bw, bh, fs, win_type.clone());

    let button_no = create_no_button(bw, bh, fs);

    let button_yes = create_yes_button(bw, bh, fs, win_type.clone());

    Flex::column()
        .with_child(label_info)
        .with_spacer(5.0)
        .with_child(
            Flex::row()
                .with_child(button_no)
                .with_child(button_yes)
        )
        .padding(5.0)
}

fn create_info_window(win_type: InfoWindowType) -> WindowDesc<AppData> {
    WindowDesc::new(build_ui(win_type))
        .window_size((ui::INFO_WINDOW_WIDTH, ui::INFO_WINDOW_HEIGHT))
        .set_position(unsafe { ui::INFO_WINDOW_POS })
        .resizable(false)
        .show_titlebar(false)
        .title("PAR - Confirm")
}

pub fn close_info_window(ctx: &mut EventCtx, data: &mut AppData) {
    if data.info_window_is_open {
        unsafe {
            ctx.submit_command(CLOSE_WINDOW.to(ui::info_window_id.unwrap()));
            data.info_window_is_open = false;
            ui::info_window_id = None;
        }
    }
}

pub fn open_info_window(ctx: &mut EventCtx, data: &mut AppData, win_type: InfoWindowType) {
    close_info_window(ctx, data);
    ctx.new_window(create_info_window(win_type));
    data.info_window_is_open = true;
}
