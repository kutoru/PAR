use druid::ImageBuf;
use std::path::Path;
use std::fs::create_dir;
use crate::data::{ArtistResult, AppData, UserData};
use crate::pixiv_handler::reset_artist_list;

/// Checks if settings.json exists. Starts an initializing sequence if it doesn't
pub fn prepare_settings() -> AppData {
    if !does_path_exist("./settings.json") {
        UserData::load_default().save();
    }
    AppData::load()
}

/// Checks if artits.json exists. Calls download_artist_list if it doesn't. It assumes that the AppData.token is valid
pub fn prepare_artists(data: &mut AppData) {
    if !data.requires_initialization {
        if !does_path_exist("./artists.json") {
            reset_artist_list(data);
        }
    }
}

/// Checks if images and jsons folders exist. Creates them if they don't
pub fn prepare_folders() {
    if !does_path_exist("./jsons") {
        create_dir("./jsons").expect("Could not create the jsons folder");
    }

    if !does_path_exist("./images") {
        create_dir("./images").expect("Could not create the images folder");
    }
}

pub fn does_path_exist(path: &str) -> bool {
    Path::new(path).exists()
}

/// Reads artists.json and returns the contents on successfull read. Returns None if an error occurs
fn get_artist_list() -> Option<Vec<(u32, bool)>> {
    let file_contents = std::fs::read_to_string("artists.json").ok()?;
    serde_json::from_str(&file_contents).ok()
}

fn save_artist_list(list: &Vec<(u32, bool)>) {
    let file_cont = serde_json::to_string(list)
        .expect("Could not convert data to json");
    std::fs::write("artists.json", file_cont)
        .expect("Could not write to artists.json");
}

/// Returns [index - 1] of the first occurence of false in artists.json. Returns 0 if artists.json is invalid
pub fn get_last_checked_artist_index() -> u16 {
    let artist_list = match get_artist_list() {
        Some(al) => al,
        None => return 0,
    };

    for index in 1..artist_list.len() {
        if !artist_list[index].1 {
            return (index-1) as u16;
        }
    }

    (artist_list.len()-1) as u16
}

/// Returns artist id, whether the artist has been checked or not, and artist.json length. Returns None if the artist_index is out of range or if artist.json is invalid. Also marks the artist as checked
pub fn get_small_artist_info(artist_index: u16) -> Option<(u32, bool, u16)> {
    let mut artist_list = match get_artist_list() {
        Some(al) => al,
        None => return None,
    };

    let count = artist_list.len();
    let index = artist_index as usize;

    if index >= count {
        return None;
    }

    let (id, status) = artist_list[index];

    if !artist_list[index].1 {
        artist_list[index].1 = true;
        save_artist_list(&artist_list);
    };

    Some((id, status, count as u16))
}

/// load artist info from disk
pub fn load_artist_info(artist_id: u32) -> ArtistResult {
    let path = format!("./jsons/{}.json", artist_id);
    let file_cont = std::fs::read_to_string(&path)
        .expect(&format!("Could not read {}", path));
    let artist_result: ArtistResult = serde_json::from_str(&file_cont)
        .expect(&format!("Could not parse {}", path));

    artist_result
}

/// Marks artists in range 0..artist_index as checked, and then marks all artists after that as not checked
pub fn mark_as_checked_up_to_index(artist_index: u16) {
    let artist_index = artist_index as usize;
    let mut list = get_artist_list().unwrap();
    for index in 0..list.len() {
        if index < artist_index {
            list[index].1 = true;
        } else {
            list[index].1 = false;
        }
    }
    save_artist_list(&list);
}

pub fn get_image_buf(path: &str) -> ImageBuf {
    let bytes = std::fs::read(&path)
        .expect(&format!("Could not open an image at \"{}\"", path));
    let image_data = ImageBuf::from_data(&bytes)
        .expect(&format!("Could not load the image at \"{}\"", path));
    image_data
}

pub fn get_path_to_pfp(id: u32) -> String {
    format!("./images/u_{}.jpeg", id)
}

pub fn get_path_to_illust(id: u32) -> String {
    format!("./images/i_{}.jpeg", id)
}
