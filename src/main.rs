use sha1::Digest;
use std::io::{BufRead, BufReader};

const SHA1_HASH_LENGTH: usize = 40;

fn hash_crack() -> Result<(), Box<dyn std::error::Error>> {
    let mut wordlist: String = String::new();
    let mut hash: String = String::new();

    println!("Enter the Wordlist File Path: ");
    let _usize: usize = std::io::stdin().read_line(&mut wordlist).unwrap();

    let wordlist_file: std::fs::File = std::fs::File::open(wordlist.trim())?;
    let buff_reader: BufReader<std::fs::File> = BufReader::new(wordlist_file);

    println!("Enter SHA 1 Hash: ");
    let _usize: usize = std::io::stdin().read_line(&mut hash).unwrap();

    if hash.trim().len() != SHA1_HASH_LENGTH {
        return Err("This is not a Valid SHA-1 Hash".into());
    }

    for line in buff_reader.lines() {
        let password: String = line?.trim().to_string();

        let mut sha1_hasher = sha1::Sha1::new();
        sha1_hasher.update(password.as_bytes());

        println!("{}", hash.to_uppercase().to_owned().as_str());
        println!("{}", (hex::encode(sha1_hasher.finalize())).to_owned().as_str());

        // if hash.to_uppercase().to_owned() == (hex::encode(sha1_hasher.finalize())).to_owned() {
        //     println!("Password of the given Hash is : {}", &password);
        //     return Ok(());
        // } else {
        //     println!("Password Not Found in theses Wordlist");
        // }
    }

    return Ok(());
}

fn main() {
    println!("SHA-1 Hash Cracker");

    let _result: Result<(), Box<dyn std::error::Error>> = hash_crack();

    return;
}
