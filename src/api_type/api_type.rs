use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub nickname: String,
    pub user_id: i64,
    pub avatar_url: String,
    pub signature: String,
    pub playlist_count: i32,
    pub followeds: i32,
    pub follows: i32,
    pub event_count: i32,
    pub playlist_be_subscribed_count: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: i64,
    pub user_name: String,
    pub salt: String,
    pub vip_type: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountDetail {
    pub code: i32,
    pub account: Account,
    pub profile: Profile,
    pub token: String,
    pub cookie: String,
}

// -------- Playlist ---------
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OtherAccount {
    pub nickname: String,
    pub user_id: i64,
    pub avatar_url: String,
    pub signature: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    pub creator: OtherAccount,
    pub cover_img_url: String,
    pub special_type: i32,
    pub play_count: i32,
    pub track_count: i32,
    pub subscribed_count: i32,
    pub name: String,
    pub id: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserPlaylist {
    pub version: String,
    pub more: bool,
    pub playlist: Vec<Playlist>,
    pub code: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    pub id: i64,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub pic_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Track {
    pub id: i64,
    pub name: String,
    pub ar: Vec<Artist>,
    pub al: Album,
    pub dt: i32, // duration
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistDetail {
    pub creator: OtherAccount,
    pub cover_img_url: String,
    pub special_type: i32,
    pub play_count: i32,
    pub track_count: i32,
    pub subscribed_count: i32,
    pub name: String,
    pub id: i64,
    pub tracks: Vec<Track>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistDetailRep {
    pub code: i32,
    pub playlist: PlaylistDetail,
}

// ---------- Music ------------
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MusicDetail {
    pub id: i64,
    pub url: String,
    pub size: i32,
    pub br: i32,
    pub md5: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Musics {
    pub data: Vec<MusicDetail>,
    pub code: i32,
}
