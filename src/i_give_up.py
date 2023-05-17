from pixivpy3 import AppPixivAPI, PixivError
import json
import sys
from datetime import datetime, timedelta
from pytz import UnknownTimeZoneError, timezone
from os import listdir, path, remove

CURR_DIR = path.curdir

##### GENERAL FUNCTIONS #####

def initialize_client(refresh_token):
    pixiv_client = AppPixivAPI()
    pixiv_client.auth(refresh_token=refresh_token)
    return pixiv_client

def get_next_page(client, result):
    next_url = result.get("next_url")
    if next_url is None:
        return None

    r = client.no_auth_requests_call("GET", next_url)
    return client.parse_result(r)

def format_artist_json(original_json):
    return {
        "name": original_json["name"],
        "id": original_json["id"],
        "recent_count": 0,
        "is_followed": original_json["is_followed"]
    }

def format_illust_json(original_json, tz):
    return {
        "id": original_json["id"],
        "views": original_json["total_view"],
        "bookmarks": original_json["total_bookmarks"],
        "upload_date": str(
            datetime.strptime(original_json["create_date"], "%Y-%m-%dT%H:%M:%S%z").astimezone(tz).strftime("%Y/%m/%d %H:%M:%S")
        ),
        "is_bookmarked": original_json["is_bookmarked"]
    }

def create_empty_illust():
    return {
        "id": 0,
        "views": 0,
        "bookmarks": 0,
        "upload_date": "None",
        "is_bookmarked": False
    }

def save_result(result):
    with open(f"{CURR_DIR}\\jsons\\{result['artist']['id']}.json", "w") as fh:
        json.dump(result, fh)

def load_result(artist_id):
    with open(f"{CURR_DIR}\\jsons\\{artist_id}.json") as fh:
        return json.load(fh)

# converts whatever to json and returns it to Rust
def return_result(result):
    print(json.dumps(result), end="")

def delete_artist_info_if_exists(artist_id):
    json_path = f"{CURR_DIR}\\jsons\\{artist_id}.json"
    if not path.isfile(json_path):
        return

    with open(json_path) as fh:
        file = json.load(fh)

    for illust in file["illusts"]:
        if illust["id"] != 0:
            remove(f"{CURR_DIR}\\images\\i_{illust['id']}.jpeg")

    remove(f"{CURR_DIR}\\images\\u_{artist_id}.jpeg")
    remove(json_path)

def delete_all_artist_info_if_exists():
    json_path = f"{CURR_DIR}\\jsons"
    if path.exists(json_path):
        for file in listdir(json_path):
            if file.endswith(".json"):
                remove(f"{json_path}\\{file}")

    image_path = f"{CURR_DIR}\\images"
    if path.exists(image_path):
        for file in listdir(image_path):
            if file.endswith(".jpeg"):
                remove(f"{image_path}\\{file}")

##### DOWNLOAD ARTIST LIST #####

def download_artist_list(token, page_limit=0):
    # delete the limit later
    page_limit = 1
    delete_all_artist_info_if_exists()

    client = initialize_client(token)
    artist_list = []

    #artists = client.user_following(client.user_id, "private")
    artists = client.user_following(client.user_id)

    temp_count = 0
    while True:
        temp_count += 1
        if page_limit > 0 and temp_count > page_limit:
            break

        for artist in artists["user_previews"]:
            artist_list.append([artist["user"]["id"], False])

        try:
            artists = get_next_page(client, artists)
        except PixivError:
            artists = None

        if artists is None:
            break

    artist_list.reverse()
    with open(f"{CURR_DIR}\\artists.json", "w") as fh:
        json.dump(artist_list, fh)

##### DOWNLOAD ARTIST INFO #####

def prepare_dai(token, id, amount_to_search, tz):
    amount_to_search = int(amount_to_search)
    if len(tz) == 0:
        tz = timezone("Etc/GMT-9")
    else:
        tz = timezone(tz)

    delete_artist_info_if_exists(id)
    download_artist_info(token, id, amount_to_search, tz)

def download_artist_info(token, artist_id, amount_to_search, tz):
    client = initialize_client(token)
    result = client.user_illusts(artist_id)

    # artist
    artist_pfp_url = result["user"]["profile_image_urls"]["medium"]
    artist = format_artist_json(result["user"])

    #print("Got artist")

    # illusts
    illust_urls = []
    illusts = []
    for i in range(min(4, len(result["illusts"]))):
        illust_urls.append(result["illusts"][i]["image_urls"]["square_medium"])
        illust = format_illust_json(result["illusts"][i], tz)
        illusts.append(illust)

    for i in range(len(illusts), 4):
        illusts.append(create_empty_illust())

    #print("Got illusts")

    # last bookmarked && recent count
    date_threshold = (datetime.now() - timedelta(days=30*6)).strftime("%Y-%m-%d")
    count_recent = True
    recent_count = 0

    found_bookmarked = False
    while_break = False
    total_count = 0

    while True:
        for illust in result["illusts"]:
            total_count += 1
            if total_count > amount_to_search:
                while_break = True
                break

            if count_recent:
                if illust["create_date"] < date_threshold:
                    count_recent = False
                    if found_bookmarked:
                        while_break = True
                        break
                else:
                    recent_count += 1

            if not found_bookmarked:
                if illust["is_bookmarked"]:
                    last_bookmarked = format_illust_json(illust, tz)
                    found_bookmarked = True
                    if not count_recent:
                        while_break = True
                        break

        if while_break:
            break

        result = get_next_page(client, result)
        if result is None:
            break

    if not found_bookmarked:
        last_bookmarked = create_empty_illust()

    artist["recent_count"] = recent_count

    #print("Got last bookmarked and recent count")

    # image download
    image_path = f"{CURR_DIR}\\images"

    filename = f"u_{artist['id']}.jpeg"
    client.download(
        artist_pfp_url,
        path = image_path,
        fname = filename,
        replace = True
    )

    for i in range(4):
        if illusts[i]["id"] == 0:
            break

        filename = f"i_{illusts[i]['id']}.jpeg"
        client.download(
            illust_urls[i],
            path = image_path,
            fname = filename,
            replace = True
        )

    #print("Downloaded all images")

    result = {
        "artist": artist,
        "last_bookmarked": last_bookmarked,
        "illusts": illusts
    }

    save_result(result)
    return_result(result)

##### TOGGLE BOOKMARK #####

def toggle_bookmark(token, artist_id, illust_index):
    illust_index = int(illust_index)

    artist_json = load_result(artist_id)
    illust_id = artist_json["illusts"][illust_index]["id"]
    is_bookmarked = artist_json["illusts"][illust_index]["is_bookmarked"]

    client = initialize_client(token)
    old_bookmark_status = client.illust_bookmark_detail(illust_id)["bookmark_detail"]["is_bookmarked"]

    success = False

    # in case there is desync and the bookmark has already been toggled
    if is_bookmarked != old_bookmark_status:
        success = True

    # otherwise, just bookmarking and checking if it was successfull
    else:
        if is_bookmarked:
            client.illust_bookmark_delete(illust_id)
        else:
            client.illust_bookmark_add(illust_id)

        new_bookmark_status = client.illust_bookmark_detail(illust_id)["bookmark_detail"]["is_bookmarked"]
        if old_bookmark_status != new_bookmark_status:
            success = True

    if success:
        artist_json["illusts"][illust_index]["is_bookmarked"] = not is_bookmarked
        save_result(artist_json)

    return_result(success)

##### TOGGLE FOLLOW #####

def toggle_follow(token, artist_id):
    artist_json = load_result(artist_id)
    is_followed = artist_json["artist"]["is_followed"]

    client = initialize_client(token)
    old_follow_status = client.user_detail(artist_id)["user"]["is_followed"]

    success = False

    # in case there is desync and the follow has already been toggled
    if is_followed != old_follow_status:
        success = True

    # otherwise, just following and checking if it was successfull
    else:
        if is_followed:
            client.user_follow_delete(artist_id)
        else:
            client.user_follow_add(artist_id)

        new_follow_status = client.user_detail(artist_id)["user"]["is_followed"]
        if old_follow_status != new_follow_status:
            success = True

    if success:
        artist_json["artist"]["is_followed"] = not is_followed
        save_result(artist_json)

    return_result(success)

##### VALIDATE TOKEN #####

def validate_token(token):
    try:
        initialize_client(token)
        result = True
    except PixivError:
        result = False

    return_result(result)

##### VALIDATE TIMEZONE #####

def validate_timezone(tz):
    try:
        timezone(tz)
        result = True
    except UnknownTimeZoneError:
        result = False

    return_result(result)

##### MAIN #####

def main():
    command = sys.argv[1]
    args = sys.argv[2:]

    if command == "download_artist_info":
        prepare_dai(*args)
    elif command == "download_artist_list":
        download_artist_list(*args)
    elif command == "toggle_bookmark":
        toggle_bookmark(*args)
    elif command == "toggle_follow":
        toggle_follow(*args)
    elif command == "validate_timezone":
        validate_timezone(*args)
    elif command == "validate_token":
        validate_token(*args)
    else:
        raise Exception("Unknown command:", command)

main()
