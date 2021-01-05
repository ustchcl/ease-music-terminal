use crate::api_type::api_type::*;
use crate::app::App;
use crate::downloader::Downloader;
use anyhow::Result;
use reqwest::{self};
use serde_json;

fn base_url() -> String {
    "http://49.234.74.97:3000".to_string()
}

pub fn login(app: &mut App) -> Result<()> {
    let url = format!(
        "{}/login/cellphone?phone=18500975410&password=s1s2s3",
        base_url()
    );
    let res = app.client.get(&url).send()?;
    let account = res.json::<AccountDetail>()?;
    app.userinfo = Option::Some(account);
    Ok(())
}

pub fn playlists(app: &mut App) -> Result<()> {
    let user_id = app.userinfo.as_ref().map(|a| a.account.id).unwrap_or(1);
    let url = format!("{}/user/playlist?uid={}", base_url(), user_id);
    let res = app.client.get(&url).send()?;
    let user_playlist = res.json::<UserPlaylist>()?;
    app.set_playlists(user_playlist.playlist);
    Ok(())
}

pub fn get_playlist_detail(app: &mut App) -> Result<()> {
    let id = app.playlists_state.items[app.playlists_state.state.selected().unwrap_or(0)].id;
    let url = format!("{}/playlist/detail?id={}", base_url(), id);
    let res = app.client.get(&url).send()?;
    let playlist_detail = res.json::<PlaylistDetailRep>()?;
    app.set_current_playlist(playlist_detail.playlist.tracks);
    Ok(())
}

pub fn get_like_list(app: &mut App) -> Result<()> {
    let user_id = app.userinfo.as_ref().map(|a| a.account.id).unwrap_or(1);
    let url = format!("{}/likelist?uid={}", base_url(), user_id);
    let res = app.client.get(&url).send()?;
    let like_list = res.json::<LikeListRep>()?;
    app.likelist = like_list.ids;
    Ok(())
}

pub fn get_music_detail(ids: Vec<i64>, app: &App) -> Result<Vec<MusicDetail>> {
    let url = format!(
        "{}/song/url?id={}",
        base_url(),
        ids.iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );
    let res = app.client.get(&url).send()?;
    let musics = res.json::<Musics>()?;
    Ok(musics.data)
}

pub fn download_music(url: &str, music_name: &str) -> Option<String> {
    let filepath = dirs::audio_dir().map(|p| p.join(music_name)).unwrap();
    let str = filepath.to_str().map(|str| str.to_string());
    if !filepath.exists() {
        let _ = Downloader::download(url, filepath);
    }
    str
}
