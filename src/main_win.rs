use druid::{Widget, WidgetExt, ImageBuf, Application, WindowDesc};
use druid::widget::{Flex, Label, SizedBox, Image, FillStrat, ViewSwitcher, LineBreaking};
use druid::piet::InterpolationMode;
use crate::data::AppData;
use crate::file_handler::{get_path_to_pfp, get_path_to_illust};
use crate::pixiv_handler::{toggle_bookmark, toggle_follow};
use crate::settings_win::open_settings_window;
use crate::ui_globals::ui::{self, create_button};

/// Creates a widget that displays artist's profile picture
fn create_pfp_widget(width: u32, height: u32) -> impl Widget<AppData> {
    let image_widget = ViewSwitcher::new(
        |data: &AppData, _| data.pfp_image.clone(), move |image_option: &Option<ImageBuf>, _, _| {
            if image_option.is_some() {
                let mut image = Image::new(image_option.as_ref().unwrap().clone()).fill_mode(FillStrat::Cover);
                image.set_interpolation_mode(InterpolationMode::Bilinear);
                let image = image.on_click(|_, data: &mut AppData, _| { open_artist_pfp(data.artist.id) });
                Box::new(image)
            } else {
                Box::new(Label::new("Profile image").with_line_break_mode(LineBreaking::WordWrap).with_font(ui::FONT).with_text_size(20.0).background(ui::HOVER_COLOR))
            }
        }
    );

    SizedBox::new(image_widget)
        .fix_width(width as f64)
        .fix_height(height as f64)
}

/// Creates a widget that displays illustration. If `illust_index >= AppData.illust_images.len()` then the function panics
fn create_illust_widget(illust_index: usize, width: u32, height: u32) -> impl Widget<AppData> {
    let image_widget = ViewSwitcher::new(
        move |data: &AppData, _| data.illust_images[illust_index].clone(), move |image_option: &Option<ImageBuf>, _, _| {
            if image_option.is_some() {
                let mut image = Image::new(image_option.as_ref().unwrap().clone()).fill_mode(FillStrat::Cover);
                image.set_interpolation_mode(InterpolationMode::Bilinear);
                let image = image.on_click(move |_, data: &mut AppData, _| { open_illust(data.illusts[illust_index].id) });
                Box::new(image)
            } else {
                Box::new(Label::new("Illustration image").with_line_break_mode(LineBreaking::WordWrap).with_font(ui::FONT).with_text_size(20.0).background(ui::HOVER_COLOR))
            }
        }
    );

    SizedBox::new(image_widget)
        .fix_width(width as f64)
        .fix_height(height as f64)
}

/// Executes whatever using windows cmd. For an example, it can be used it to open images and urls externally
fn execute_process(something_to_execute: &str) {
    std::process::Command::new("cmd")
        .arg("/C")
        .arg("start")
        .arg(something_to_execute)
        .output()
        .expect(&format!("Failed to execute \"{}\"", something_to_execute));
}

fn open_artist_pfp(id: u32) {
    execute_process(&get_path_to_pfp(id));
}

fn open_illust(id: u32) {
    execute_process(&get_path_to_illust(id));
}

fn open_artist_url(id: u32) {
    execute_process(&format!("https://www.pixiv.net/en/users/{}", id));
}

fn open_illust_url(id: u32) {
    execute_process(&format!("https://www.pixiv.net/en/artworks/{}", id));
}

/// Creates the titlebar container
fn create_titlebar() -> Flex<AppData> {
    let bs = 40;  // button size
    let fs = 30;  // font size

    let button_settings = create_button(
        Label::new("S"),
        bs, bs, fs, true)
        .on_click(|ctx, data, _| { open_settings_window(ctx, data) });

    let label_window_info = create_button(
        Label::new(|data: &AppData, _: &_| { data.window_title.clone() }),
        bs, bs, fs, true);

    //let button_minimize = Button::new("-")
    //    .on_click(|ctx, _, _| { ctx.window().set_window_state(druid::WindowState::Minimized) });

    let button_close = create_button(
        Label::new("X"),
        bs, bs, fs, true)
        .on_click(|_, _, _| { Application::global().quit() });

    Flex::row()
        .with_child(button_settings)
        .with_flex_child(label_window_info.expand_width(), 1.0)
        .with_child(button_close)
}

fn create_artist_container() -> Flex<AppData> {
    let is = 120;  // image size
    let bh = is/2;  // button height
    let bw = ((ui::WINDOW_WIDTH as u32) - 10 - is) / 2;  // button width
    let fs = 20;

    let image_pfp = create_pfp_widget(is, is);

    let label_artist_name = create_button(
        Label::new(|data: &AppData, _: &_| { data.artist.name.clone() }).with_line_break_mode(LineBreaking::WordWrap),
        bw+1, bh, fs+10, false)
        .on_click(|_, data, _| {
            if data.artist.id != 0 { open_artist_url(data.artist.id) }
        });

    let label_recent_uploads = create_button(
        Label::new(|data: &AppData, _: &_| { format!("Illustrations in the last 6\nmonths: {}", data.artist.recent_count) }),
        bw+1, bh, fs, false);

    let label_last_bookmark = create_button(
        Label::new(|data: &AppData, _: &_| {
            if data.last_bookmarked.id == 0 { format!("No bookmarks in the last {}\nillustrations", data.settings.amount_to_search) }
            else { format!("Last bookmarked illustration:\n{}", data.last_bookmarked.upload_date) }
        }), bw, bh, fs, false)
        .on_click(|_, data, _| {
            if data.last_bookmarked.id != 0 { open_illust_url(data.last_bookmarked.id) }
        });

    let label_check_status = create_button(
        Label::new(|data: &AppData, _: &_| {
            if data.has_been_checked { "You have already reviewed\nthis artist" }
            else { "You haven't reviewed this\nartist yet" }
        }), bw, bh, fs, false);

    Flex::row()
        .with_child(image_pfp)
        .with_child(
            Flex::column()
                .with_child(label_artist_name)
                .with_child(label_recent_uploads)
        )
        .with_child(
            Flex::column()
                .with_child(label_last_bookmark)
                .with_child(label_check_status)
        )
}

fn create_illust(illust_index: usize) -> impl Widget<AppData> {
    let is = 230;
    let bs = 30;
    let fs = 20;

    let image_illust = create_illust_widget(illust_index, is, is);

    let label_views = create_button(
        Label::new(move |data: &AppData, _: &_| { format!("V: {}", data.illusts[illust_index].views) }),
        is/2, bs, fs, true);
    let label_bookmarks = create_button(
        Label::new(move |data: &AppData, _: &_| { format!("B: {}", data.illusts[illust_index].bookmarks) }),
        is/2, bs, fs, true);

    let label_upload_date = create_button(
        Label::new(move |data: &AppData, _: &_| { data.illusts[illust_index].upload_date.clone() }),
        is, bs, fs, true);

    let button_open = create_button(
        Label::new("Open"),
        is/2, bs, fs, true)
        .on_click(move |_, data, _| {
            if data.illusts[illust_index].id != 0 { open_illust_url(data.illusts[illust_index].id) }
        });
    let button_bookmark = create_button(
        Label::new(move |data: &AppData, _: &_| {
            if data.illusts[illust_index].is_bookmarked { "Unbookmark" }
            else { "Bookmark" }
        }), is/2, bs, fs, true)
        .on_click(move |_, data, _| {
            if data.illusts[illust_index].id != 0 { toggle_bookmark(data, illust_index) }
        });

    Flex::column()
        .with_child(image_illust)
        .with_child(
            Flex::row()
                .with_child(label_views)
                .with_child(label_bookmarks)
        )
        .with_child(label_upload_date)
        .with_child(
            Flex::row()
                .with_child(button_open)
                .with_child(button_bookmark)
        )
}

fn create_illust_container() -> Flex<AppData> {
    let mut cont = Flex::row();
    for index in 0..4 {
        let illust = create_illust(index);
        cont = cont.with_child(illust).with_spacer(5.0);
    }
    cont
}

fn create_footer() -> Flex<AppData> {
    let bh = 40;
    let bw = 160;
    let fs = 30;
    let filler_width = (ui::WINDOW_WIDTH as u32) - 10 - (bw * 4);

    let button_reload = create_button(
        Label::new("Reload"),
        bw, bh, fs, true)
        .on_click(|_, data, _| {
            data.change_artist(data.artist_index, true);
        });
    let button_follow = create_button(
        Label::new(|data: &AppData, _: &_| {
            if data.artist.is_followed { "Unfollow" }
            else { "Follow" }
        }), bw, bh, fs, true)
        .on_click(|_, data, _| {
            toggle_follow(data);
        });

    let label_filler = create_button(
        Label::new(""),
        filler_width, bh, fs, true);

    let button_prev = create_button(
        Label::new("<"),
        bw, bh, fs, true)
        .on_click(|_, data, _| {
            if data.artist_index != 0 { data.change_artist(data.artist_index-1, false) }
        });
    let button_next = create_button(
        Label::new(">"),
        bw, bh, fs, true)
        .on_click(|_, data, _| {
            data.change_artist(data.artist_index+1, false);
        });

    Flex::row()
        .with_child(button_reload)
        .with_child(button_follow)
        .with_child(label_filler)
        .with_child(button_prev)
        .with_child(button_next)
}

fn build_ui() -> impl Widget<AppData> {
    let titlebar = create_titlebar();
    let artist_container = create_artist_container();
    let illust_container = create_illust_container();
    let footer = create_footer();

    Flex::column()
        .with_child(titlebar)
        .with_spacer(5.0)
        .with_child(artist_container)
        .with_spacer(5.0)
        .with_child(illust_container)
        .with_spacer(5.0)
        .with_child(footer)
        .padding(5.0)
}

pub fn create_main_window() -> WindowDesc<AppData> {
    WindowDesc::new(build_ui())
        .window_size((ui::WINDOW_WIDTH, ui::WINDOW_HEIGHT))
        .set_position(unsafe {ui::WINDOW_POS})
        .resizable(false)
        .show_titlebar(false)
        .title(|data: &AppData, _: &_| { data.window_title.clone() })
}
