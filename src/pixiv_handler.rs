use std::process::Output;
use crate::data::{UserData, ArtistResult, AppData};

const PYTHON_SCRIPT_PATH: &str = "src/i_give_up.py";

/// Calls the python script with the specified arguments
fn call_python_script(mut script_args: Vec<&str>) -> Output {
    // inserting constant args at the beggining of the vector
    script_args.splice(0..0, ["/C", "python", PYTHON_SCRIPT_PATH]);

    let output = std::process::Command::new("cmd")
        .args(script_args)
        .output()
        .expect(&format!("Failed to execute python script"));

    let code = output.status.code().unwrap();
    if code != 0 {
        panic!("Python script did not succeed, exit code: {}\nstderr:\n{}", code, std::str::from_utf8(&output.stderr).unwrap());
    }

    output
}

/// Returns parse error message
fn parse_error(output: &Vec<u8>) -> String {
    format!("Could not parse python output\n{}", std::str::from_utf8(output).unwrap())
}

/// Downloads artist info from pixiv
pub fn download_artist_info(data: &UserData, artist_id: u32) -> ArtistResult {
    let artist_id = artist_id.to_string();
    let script_args = vec![
        "download_artist_info",
        &data.token,
        &artist_id,
        &data.amount_to_search,
        &data.timezone
    ];
    let output = call_python_script(script_args);

    let artist_result: ArtistResult = serde_json::from_slice(&output.stdout)
        .expect(&parse_error(&output.stdout));
    artist_result
}

pub fn download_artist_list(data: &AppData) {
    let script_args = vec![
        "download_artist_list",
        &data.settings.token,
    ];
    let _output = call_python_script(script_args);
}

/// Tries to create a pixiv client with the token and returns true on success, otherwise returns false
pub fn check_token_validity(token: &str) -> bool {
    let script_args = vec![
        "validate_token",
        token,
    ];
    let output = call_python_script(script_args);

    let result: bool = serde_json::from_slice(&output.stdout)
        .expect(&parse_error(&output.stdout));

    result
}

/// If amount_to_search is a valid i16 and is less than 1020, returns true, otherwise returns false
pub fn check_amount_to_search_validity(amount_to_search: &str) -> bool {
    match amount_to_search.parse::<i16>() {
        Ok(num) => {
            if num < 0 || num > 1020 { false }
            else { true }
        },
        Err(_) => false,
    }
}

/// Tries to create a timezone object with the provided timezone string and returns true on success, otherwise returns false
pub fn check_timezone_validity(timezone: &str) -> bool {
    let script_args = vec![
        "validate_timezone",
        timezone,
    ];
    let output = call_python_script(script_args);

    let result: bool = serde_json::from_slice(&output.stdout)
        .expect(&parse_error(&output.stdout));

    result
}

pub fn toggle_bookmark(data: &mut AppData, illust_index: usize) {
    let artist_id = data.artist.id.to_string();
    let str_illust_index = illust_index.to_string();
    let script_args = vec![
        "toggle_bookmark",
        &data.settings.token,
        &artist_id,
        &str_illust_index,
    ];
    let output = call_python_script(script_args);

    let success: bool = serde_json::from_slice(&output.stdout)
        .expect(&parse_error(&output.stdout));

    if success {
        data.illusts[illust_index].is_bookmarked = !data.illusts[illust_index].is_bookmarked;
    }
}

pub fn toggle_follow(data: &mut AppData) {
    if data.requires_initialization {
        return
    }

    let artist_id = data.artist.id.to_string();
    let script_args = vec![
        "toggle_follow",
        &data.settings.token,
        &artist_id,
    ];
    let output = call_python_script(script_args);

    let success: bool = serde_json::from_slice(&output.stdout)
        .expect(&parse_error(&output.stdout));

    if success {
        data.artist.is_followed = !data.artist.is_followed;
    }
}

pub fn reset_artist_list(data: &mut AppData) {
    download_artist_list(data);
    data.change_artist(0, false);
}
