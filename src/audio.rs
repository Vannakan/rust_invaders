use std::{fs::read_dir, error::Error};

use rusty_audio::Audio;

const AUDIO_DIR: &str = "audio";

pub fn get_file_name(file_name: &String) -> &str {
    if let Some((first, _)) = file_name.split_once('.') {
        first
    } else {
        ""
    }
}

pub fn register_audio(audio: &mut Audio) -> Result<(), Box<dyn Error>> {
    let entries = read_dir(AUDIO_DIR)?; 
    for entry in entries {
        let file = entry?;

        let file_name = file.file_name().into_string().unwrap();

        let first = get_file_name(&file_name);

        println!("{:?}", first);
        println!("{:?}", file.path());
        audio.add(first, file.path())
    }

    Ok(())
}
