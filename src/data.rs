use serde::{Deserialize, Serialize};
use druid::{Data, ImageBuf, Lens};
use crate::file_handler::{get_image_buf, get_path_to_pfp, get_path_to_illust, get_small_artist_info, load_artist_info, get_last_checked_artist_index, mark_as_checked_up_to_index, does_path_exist};
use crate::pixiv_handler::{download_artist_info, check_token_validity, check_amount_to_search_validity, check_timezone_validity, reset_artist_list};
use crate::ui_globals::ui;

#[derive(Debug, Serialize, Deserialize, Clone, Data)]
pub struct UserData {
    /// pixiv's refresh token
    pub token: String,
    /// Max amount of illustrations to search for last bookmarked illustration and for recent illustration count. min=0, max=1020. It is more optimal for it to be divisible by 30
    pub amount_to_search: String,
    /// All dates will get converted to this timezone. Set to "" to use pixiv's timezone. For formatting check python's pytz module
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
/// Struct used for retrieving artist info from json files
pub struct ArtistResult {
    pub artist: Artist,
    pub last_bookmarked: Illustration,
    pub illusts: [Illustration; 4],
}

#[derive(Debug, Clone, Data, Lens)]
pub struct AppData {
    pub requires_initialization: bool,
    pub window_title: String,
    pub settings_window_is_open: bool,
    pub info_window_is_open: bool,
    /// Artist index to jump to when using jump to artist
    pub jump_index: String,
    /// Current user settings for the app
    pub settings: UserData,
    // Temp settings to allow cancelling functionality when the user is changing settings
    pub temp_token: String,
    pub temp_amount_to_search: String,
    pub temp_timezone: String,
    /// Whether the artist has been checked or not (aka whether their info was downloaded for the first time or not)
    pub has_been_checked: bool,
    /// Total artist count
    pub total_artists: u16,
    /// Current artist index
    pub artist_index: u16,
    pub pfp_image: Option<ImageBuf>,
    pub illust_images: [Option<ImageBuf>; 4],
    pub artist: Artist,
    /// Latest bookmarked illustration
    pub last_bookmarked: Illustration,
    /// 4 most recent illustrations
    pub illusts: [Illustration; 4],
}

#[derive(Debug, Serialize, Deserialize, Clone, Data)]
pub struct Artist {
    pub name: String,
    pub id: u32,
    /// Illustrations uploaded in the last 6 months
    pub recent_count: u16,
    pub is_followed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Data)]
pub struct Illustration {
    pub id: u32,
    pub views: u32,
    pub bookmarks: u32,
    pub upload_date: String,
    pub is_bookmarked: bool,
}

impl UserData {
    fn load() -> UserData {
        let file_cont = std::fs::read_to_string("settings.json")
            .expect("Could not read settings.json");
        serde_json::from_str(&file_cont)
            .expect("Could not parse settings.json")
    }

    pub fn save(&self) {
        let file_cont = serde_json::to_string(self)
            .expect("Could not convert data to json");
        std::fs::write("settings.json", file_cont)
            .expect("Could not write to settings.json");
    }

    pub fn load_default() -> UserData {
        UserData {
            token: "None".to_string(),
            amount_to_search: ui::DEFAULT_AMOUNT_TO_SEARCH.to_string(),
            timezone: ui::DEFAULT_TIMEZONE.to_string(),
        }
    }
}

fn make_window_title(artist_name: &str, artist_index: u16, total_artists: u16) -> String {
    format!("PAR - {} - {}/{}", artist_name, artist_index+1, total_artists)
}

impl AppData {
    pub fn load_window_title(&mut self) {
        self.window_title = make_window_title(&self.artist.name, self.artist_index, self.total_artists);
    }

    pub fn load_images(&mut self) {
        if self.artist.id != 0 {
            let image_buf_pfp = get_image_buf(&get_path_to_pfp(self.artist.id));
            self.pfp_image = Some(image_buf_pfp);
        } else {
            self.pfp_image = None;
        }
        
        for index in 0..4 {
            if self.illusts[index].id != 0 {
                let image_buf_illust = get_image_buf(&get_path_to_illust(self.illusts[index].id));
                self.illust_images[index] = Some(image_buf_illust);
            } else {
                self.illust_images[index] = None;
            }
        }
    }

    /// Load all artist related information into the data
    pub fn change_artist(&mut self, artist_index: u16, redownload_if_exists: bool) {
        if self.requires_initialization {
            return
        }

        let (artist_id, has_been_checked, total_artists) = match get_small_artist_info(artist_index) {
            Some((i, s, l)) => (i, s, l),
            None => return,
        };

        let artist_result = {
            if redownload_if_exists || !does_path_exist(&format!("./jsons/{}.json", artist_id)) {
                download_artist_info(&self.settings, artist_id)
            } else {
                load_artist_info(artist_id)
            }
        };

        self.artist = artist_result.artist;
        self.last_bookmarked = artist_result.last_bookmarked;
        self.illusts = artist_result.illusts;

        self.artist_index = artist_index;
        self.total_artists = total_artists;
        self.has_been_checked = has_been_checked;

        self.load_window_title();
        self.load_images();
    }

    pub fn load() -> AppData {
        let user_data = UserData::load();
        let requires_initialization = !check_token_validity(&user_data.token);

        let mut data = AppData {
            requires_initialization: requires_initialization,
            window_title: "<- Click on the S and change your token".to_string(),
            settings_window_is_open: false,
            info_window_is_open: false,
            jump_index: "0".to_string(),
            settings: user_data.clone(),
            temp_token: user_data.token,
            temp_amount_to_search: user_data.amount_to_search,
            temp_timezone: user_data.timezone,
            has_been_checked: false,
            total_artists: 0,
            artist_index: get_last_checked_artist_index(),
            pfp_image: None,
            illust_images: [None, None, None, None],
            artist: Artist::load_empty(),
            last_bookmarked: Illustration::load_empty(),
            illusts: Illustration::load_empty_list(),
        };

        data.change_artist(data.artist_index, false);
        data
    }

    pub fn apply_settings(&mut self) {
        let mut changes_made = false;
        let mut do_reset_artist_list = false;

        self.temp_token = self.temp_token.trim().to_string();
        self.temp_amount_to_search = self.temp_amount_to_search.trim().to_string();
        self.temp_timezone = self.temp_timezone.trim().to_string();

        if self.settings.token != self.temp_token {
            let is_valid = check_token_validity(&self.temp_token);
            if is_valid {
                self.settings.token = self.temp_token.clone();
                self.requires_initialization = false;
                changes_made = true;
                do_reset_artist_list = true;
            } else {
                self.temp_token = self.settings.token.clone();
            }
        }

        if self.settings.amount_to_search != self.temp_amount_to_search {
            if self.temp_amount_to_search.len() == 0 {
                self.temp_amount_to_search = ui::DEFAULT_AMOUNT_TO_SEARCH.to_string();
                self.settings.amount_to_search = ui::DEFAULT_AMOUNT_TO_SEARCH.to_string();
                changes_made = true;
            } else {
                let is_valid = check_amount_to_search_validity(&self.temp_amount_to_search);
                if is_valid {
                    self.settings.amount_to_search = self.temp_amount_to_search.clone();
                    changes_made = true;
                } else {
                    self.temp_amount_to_search = self.settings.amount_to_search.clone();
                }
            }
        }

        if self.settings.timezone != self.temp_timezone {
            if self.temp_timezone.len() == 0 {
                self.temp_timezone = ui::DEFAULT_TIMEZONE.to_string();
                self.settings.timezone = ui::DEFAULT_TIMEZONE.to_string();
                changes_made = true;
            } else {
                let is_valid = check_timezone_validity(&self.temp_timezone);
                if is_valid {
                    self.settings.timezone = self.temp_timezone.clone();
                    changes_made = true;
                } else {
                    self.temp_timezone = self.settings.timezone.clone();
                }
            }
        }

        if changes_made {
            self.settings.save();
        }

        if do_reset_artist_list {
            reset_artist_list(self);
        }
    }

    pub fn cancel_settings(&mut self) {
        self.jump_index = "0".to_string();
        self.temp_token = self.settings.token.clone();
        self.temp_amount_to_search = self.settings.amount_to_search.clone();
        self.temp_timezone = self.settings.timezone.clone();
    }

    pub fn temp_token_has_changed(&self) -> bool {
        if self.settings.token != self.temp_token { true }
        else { false }
    }

    /// Checks if jump_index is a valid artist_index
    pub fn jump_index_is_valid(&self) -> bool {
        match self.jump_index.parse::<u16>() {
            Ok(num) => {
                if num == 0 || num > self.total_artists { false }
                else { true }
            },
            Err(_) => false,
        }
    }

    /// Returns ((jump_index as u16) - 1) if jump_index is valid. Otherwise returns None
    pub fn get_jump_index(&self) -> Option<u16> {
        if self.jump_index_is_valid() {
            Some(self.jump_index.parse::<u16>().unwrap() - 1)
        } else {
            None
        }
    }

    /// Jumps to artist at jump_index
    pub fn jump_to_jump_index(&mut self) {
        let new_index = self.get_jump_index().unwrap();
        mark_as_checked_up_to_index(new_index);
        self.change_artist(new_index, false);
    }
}

impl Artist {
    fn load_empty() -> Artist {
        Artist {
            name: "Artist name".to_string(),
            id: 0,
            recent_count: 0,
            is_followed: false,
        }
    }
}

impl Illustration {
    fn load_empty() -> Illustration {
        Illustration {
            id: 0,
            views: 0,
            bookmarks: 0,
            upload_date: "Upload date".to_string(),
            is_bookmarked: false,
        }
    }

    fn load_empty_list() -> [Illustration; 4] {
        [
            Illustration::load_empty(),
            Illustration::load_empty(),
            Illustration::load_empty(),
            Illustration::load_empty(),
        ]
    }
}
