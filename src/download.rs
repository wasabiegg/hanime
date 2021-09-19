use super::decrypt::decrypt;
use bytes::{BufMut, BytesMut};
use futures::future;
use m3u8_rs::playlist::MediaPlaylist;
use reqwest::Client;
use std::error::Error;
use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use std::sync::{Arc, Mutex};
use pbr::ProgressBar;

// type locked_progressbar = Arc<Mutex<ProgressBar<u64>>>;

pub async fn download_playlist(
    m3u8_playlist: &MediaPlaylist,
    client: Client,
    tmp_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut jobs = Vec::new();

    // dump tp filelist.txt
    let filelist_path = tmp_path.join("filelist.txt");
    let file_handler: File = File::create(&filelist_path).await.unwrap();
    let mut buf_writer = BufWriter::new(file_handler);

    for idx in 0..m3u8_playlist.segments.len() {
        buf_writer
            .write(format!("file {}.ts\n", idx).as_bytes())
            .await?;
    }
    buf_writer.flush().await?;

    // init progress bar
    let progressbar = Arc::new(Mutex::new(ProgressBar::new(m3u8_playlist.segments.len() as u64)));

    for (idx, segment) in m3u8_playlist.segments.iter().enumerate() {
        // println!("{}", &segment.uri);

        let out_path = tmp_path.join(format!("{}.ts", idx));
        if out_path.is_file() {
            tokio::fs::remove_file(&out_path).await?;
        }

        let client = client.clone();
        let segment_url: String = segment.uri.clone();
        let shared_progressbar = Arc::clone(&progressbar);

        let task = tokio::spawn(async move {
            let result = download_segment(segment_url, out_path, client).await;
            match result {
                Ok(_) => {
                    let mut pb = shared_progressbar.lock().unwrap();
                    pb.inc();
                },
                Err(e) => eprintln!("{:?}", e),
            }
        });
        jobs.push(task);
    }
    future::join_all(jobs).await;

    Ok(())
}

pub async fn download_segment(
    segment_url: String,
    out_path: PathBuf,
    client: Client,
) -> Result<(), Box<dyn Error>> {
    loop {
        let resp = client.get(&segment_url).send().await;
        if resp.is_ok() {
            // let mut file = File::create(out_path).await.unwrap();
            // .unwrap_or_else(async |_| -> File { File::open(out_path).await.unwrap() });
            let resp_bytes = resp.unwrap().bytes().await;

            if resp_bytes.is_ok() {
                let resp_bytes = resp_bytes.unwrap();
                let mut resp_bytes_raw = BytesMut::with_capacity(resp_bytes.len());
                resp_bytes_raw.put(&resp_bytes[..]);

                let decrypted_bytes = decrypt(&mut resp_bytes_raw).unwrap();

                // create file, if not existed open it then override
                let mut file = File::create(&out_path).await.unwrap();

                match file.write_all(&decrypted_bytes).await {
                    Ok(_) => return Ok(()),
                    Err(e) => return Err(Box::new(e)),
                }
            }
        }
    }
}
