use std::path::Path;
use std::process::{Command, Output};

pub fn concat(filelist_path: &Path, output_path: &Path) -> Result<(), &'static str>{
    let mut command = Command::new("ffmpeg");
    command
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(filelist_path.to_str().unwrap())
        .arg("-c")
        .arg("copy")
        .arg(output_path.to_str().unwrap());

    // println!("{:?}", &command);
    let output: Output = command.output().unwrap();

    if output.status.success() {
        return Ok(())
    } else {
        return Err("can not concat")
    }
    // println!("{:?}", &output.stderr);
    // println!("{:?}", &output.stdout);
}

