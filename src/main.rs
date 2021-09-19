use clap::{App, Arg};
use hanime::run;
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse args
    let matches = App::new("Hanime command line download tool")
        .version("1.0.0")
        .author("nobody")
        .subcommand(
            App::new("get")
                .about("control download function")
                .version("1.0.0")
                .arg(
                    Arg::new("url")
                        .long("url")
                        .about("specify url")
                        .multiple_values(true)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("path")
                        .long("path")
                        .short('p')
                        .about("specify outpath")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::new("tmp")
                        .long("tmp")
                        .short('t')
                        .about("specify tmppath")
                        .takes_value(true)
                        .required(false),
                )
        )
        .get_matches();

    println!("");
    println!("{:#<39}", "");
    println!("{:^39}", "HANIME DOWNLOADER COMMAND LINE TOOL");
    println!("{:#<39}", "");
    println!("{:^39}", "AUTHOR: NOBODY");
    println!("{:^39}", "VERSION 1.0.0");
    println!("{:#<39}", "");
    println!("{:^39}", "ENJOY :D");
    println!("{:#<39}", "");

    println!("");
    println!("{:-<39}", "");

    // get hanime video
    if let Some(sub_matches) = matches.subcommand_matches("get") {
        let url: Vec<&str> = sub_matches.values_of("url").unwrap().collect();
        let path = sub_matches.value_of("path").unwrap_or(".");
        let tmp = sub_matches.value_of("tmp").unwrap_or(".");

        // build config 
        let save_path: PathBuf = Path::new(path).to_path_buf();
        let tmp_path: PathBuf = Path::new(tmp).join("tmp").to_path_buf();

        if !tmp_path.is_dir() {
            tokio::fs::create_dir(&tmp_path).await?;
        }

        run(&url, save_path, tmp_path).await?;
        // run(config).await?;
    } else {
        eprintln!("no command specify");
        println!("For more information try --help");
    }

    Ok(())
}
