use sha1::Digest;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

pub(crate) fn encode_sha1(text: &str) -> String {
    hex::encode(sha1::Sha1::digest(text.as_bytes()))
}

const SHA1_HEX_STRING_LENGTH: usize = 40;

pub(crate) fn decode_sha1_string(wordlist_path: &str, hash: &str) -> Result<(), Box<dyn Error>> {
    if hash.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file = File::open(wordlist_path)?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hash == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }

    return Err("sha1 hash could not be decoded".into());
}
