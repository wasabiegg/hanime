use m3u8_rs::playlist::{MediaPlaylist, Playlist};
use regex::Regex;
use reqwest::Client;
use reqwest::Response;
use serde_json::Value;
use std::error::Error;
use std::process;
use std::time::Duration;
use std::path::PathBuf;
use std::sync::Arc;


// local lib
mod clean;
use clean::clean_folder;

mod ffmpeg;
use ffmpeg::concat;

mod uuid_tool;

mod download;
use download::download_playlist;


mod decrypt;

// type Aes128Cbc = Cbc<Aes128, Pkcs7>;
pub async fn run(url: &Vec<&str>, save_path: PathBuf, tmp_path: PathBuf) -> Result<(), Box<dyn Error>> {
    // build the client
    let client = reqwest::Client::builder()
        .timeout(Duration::new(9, 0))
        .build()
        .unwrap();

    let save_path = Arc::new(save_path);
    let tmp_path = Arc::new(tmp_path);

    for hanime_url in url.iter() {
        let c_client = client.clone();
        download_hanime(hanime_url, c_client, Arc::clone(&save_path), Arc::clone(&tmp_path)).await?;
    }

    Ok(())
}

async fn download_hanime(url: &str, client: Client, save_path: Arc<PathBuf>, tmp_path: Arc<PathBuf>) -> Result<(), Box<dyn Error>> {

    let body = client
        .get(url)
        .send()
        .await
        .expect(&format!("can't get url {}", url));

    let body_text = &body.text().await.expect("can't parse body to text");

    // find info text
    let re = Regex::new(r"<script>window.__NUXT__=(.*?);</script>").unwrap();

    let result = re.captures(body_text);

    let result = match result {
        Some(result_captures) => result_captures,
        None => {
            eprintln!("hanime api may changed!");
            process::exit(1);
        }
    };

    let content = result.get(1).unwrap().as_str();

    let v: Value = serde_json::from_str(content).unwrap();
    let hentai_name = &v["state"]["data"]["video"]["hentai_video"]["name"].to_string()[..];
    // let hentai_name = hentai_name.trim_matches(&['"'] as &[_]);
    let hentai_name = hentai_name.trim_matches('"');

    let best_m3u8_url =
        &v["state"]["data"]["video"]["videos_manifest"]["servers"][0]["streams"][1]["url"];

    // println!("hentai_name: {:?}", hentai_name.as_str());
    // println!("m3u8 url: {:?}", best_m3u8_url.as_str());

    println!("[DOWNLOADING] {} {}", hentai_name, url);

    let best_m3u8_url = best_m3u8_url.as_str().unwrap();

    // create tmp path
    let new_uuid = uuid_tool::get_uuid_v4();
    // let tmp_path = Path::new(&new_uuid);
    let tmp_path = tmp_path.join(&new_uuid);

    // out path
    let out: String = format!("{}.mp4", &hentai_name);
    let out_path = save_path.join(&out);
    if out_path.is_file() {
        eprintln!("file: {:?} already exists", &out_path);
        process::exit(1); // if hentai already exist, don't download, just exit
    }

    // create tmp path if not exists
    if !tmp_path.is_dir() {
        tokio::fs::create_dir(&tmp_path).await?;
    }

    let m3u8_playlist = get_m3u8_playlist(best_m3u8_url, client.clone()).await?;

    // println!("{:#?}", m3u8_playlist);
    download_playlist(&m3u8_playlist, client.clone(), &tmp_path).await?;

    // concat
    print!("[CONCATING]...");
    let filelist_path = &tmp_path.join("filelist.txt");
    if concat(&filelist_path, &out_path).is_ok() {
        println!(" SUCCESS!");
    } else {
        eprintln!(" FAILED!");
    }

    // clean work
    print!("[CLEANING]...");
    clean_folder(&tmp_path).await.expect("can not clean tmp folder");
    println!(" SUCCESS!");

    Ok(())
}

async fn get_m3u8_playlist(
    m3u8_url: &str,
    client: Client,
) -> Result<MediaPlaylist, Box<dyn std::error::Error>> {
    let resp: Response = client.get(m3u8_url).send().await?;

    let resp_bytes = resp.bytes().await?;
    let media_playlist: MediaPlaylist = parse_m3u8(&resp_bytes)?;

    Ok(media_playlist)
}

pub fn parse_m3u8(bytes: &[u8]) -> Result<MediaPlaylist, &'static str> {
    // let mut bytes: &[u8] = content.as_bytes();

    match m3u8_rs::parse_playlist_res(bytes) {
        Ok(Playlist::MasterPlaylist(_pl)) => {
            return Err("can't handle media playlist");
        }
        Ok(Playlist::MediaPlaylist(pl)) => Ok(pl),
        Err(_e) => Err("can't parse m3u8"),
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_s() {}
}
