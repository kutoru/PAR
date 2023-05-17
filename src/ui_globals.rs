pub mod ui {
    use druid::{Widget, WidgetExt, Color, RenderContext, FontDescriptor, FontFamily, Point, WindowId};
    use druid::widget::{Painter, Label, SizedBox};
    use crate::data::AppData;

    pub const WINDOW_WIDTH: f64 = 945.0;
    pub const WINDOW_HEIGHT: f64 = 545.0;
    pub const SETTINGS_WINDOW_WIDTH: f64 = 400.0;
    pub const SETTINGS_WINDOW_HEIGHT: f64 = 425.0;
    pub const INFO_WINDOW_WIDTH: f64 = 300.0;
    pub const INFO_WINDOW_HEIGHT: f64 = 195.0;
    
    pub static mut SCREEN_WIDTH: f64 = 0.0;
    pub static mut SCREEN_HEIGHT: f64 = 0.0;
    pub static mut WINDOW_POS: Point = Point { x: 0.0, y: 0.0 };
    pub static mut SETTINGS_WINDOW_POS: Point = Point { x: 0.0, y: 0.0 };
    pub static mut INFO_WINDOW_POS: Point = Point { x: 0.0, y: 0.0 };

    pub const FONT: FontDescriptor = FontDescriptor::new(FontFamily::MONOSPACE);
    pub const INACTIVE_COLOR: Color = Color::rgb8(0, 120, 200);
    pub const HOVER_COLOR: Color = Color::rgb8(0, 90, 160);
    pub const ACTIVE_COLOR: Color = Color::rgb8(0, 60, 120);

    pub const DEFAULT_AMOUNT_TO_SEARCH: &str = "210";
    pub const DEFAULT_TIMEZONE: &str = "Etc/GMT-9";

    // This allows closing a window by clicking on a button in foreign windows. Ideally I would store this in the AppData, but druid doesn't allow that
    #[allow(non_upper_case_globals)]
    pub static mut setting_window_id: Option<WindowId> = None;
    #[allow(non_upper_case_globals)]
    pub static mut info_window_id: Option<WindowId> = None;

    /// Pre-initialize positions for all windows that the app could open. If center_window is true, the main window will appear in the center of the screen. Otherwise, it will appear in the top right corner. TODO: make SCREEN_WIDTH and SCREEN_HEIGHT initialize based on an actual screen size
    pub fn initialize_window_positions(center_window: bool) {
        unsafe {
            SCREEN_WIDTH = 1920.0;
            SCREEN_HEIGHT = 1080.0;

            WINDOW_POS = {
                if center_window {
                    Point {
                        x: (SCREEN_WIDTH / 2.0) - (WINDOW_WIDTH / 2.0),
                        y: (SCREEN_HEIGHT / 2.0) - (WINDOW_HEIGHT / 2.0),
                    }
                } else {
                    Point {
                        x: SCREEN_WIDTH - WINDOW_WIDTH,
                        y: 0.0,
                    }
                }
            };

            SETTINGS_WINDOW_POS = Point {
                x: WINDOW_POS.x + (WINDOW_WIDTH / 2.0) - (SETTINGS_WINDOW_WIDTH / 2.0),
                y: WINDOW_POS.y + (WINDOW_HEIGHT / 2.0) - (SETTINGS_WINDOW_HEIGHT / 2.0),
            };

            INFO_WINDOW_POS = Point {
                x: SETTINGS_WINDOW_POS.x + (SETTINGS_WINDOW_WIDTH / 2.0) - (INFO_WINDOW_WIDTH / 2.0),
                y: SETTINGS_WINDOW_POS.y + (SETTINGS_WINDOW_HEIGHT / 2.0) - (INFO_WINDOW_HEIGHT / 2.0),
            };
        };
    }

    /// Creates a button-like widget from an existing label
    pub fn create_button(label: Label<AppData>, width: u32, height: u32, font_size: u32, center: bool) -> impl Widget<AppData> {
        let painter = Painter::new(|ctx, _, _| {
            let bounds = ctx.size().to_rect();

            ctx.fill(bounds, &INACTIVE_COLOR);
            
            if ctx.is_hot() {
                ctx.fill(bounds, &HOVER_COLOR);
            }

            if ctx.is_active() {
                ctx.fill(bounds, &ACTIVE_COLOR);
            }
        });

        let button = label
            .with_font(FONT)
            .with_text_size(font_size as f64);

        let aligned_button = {
            if center { button.center() }
            else { button.align_left() }
        };

        let button = aligned_button
            .background(painter)
            .on_click(|_, _, _| {});

        SizedBox::new(button)
            .width(width as f64)
            .height(height as f64)
    }
}
