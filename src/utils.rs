/* For each subdomain,
- generate a new ddns key (tsig-keygen -a hmac-md5 {{subdomain}}.dyn.commoninternet.net) and append it to /etc/bind/dyn.commoninternet.net.keys
- add a zone section to /etc/bind/named.conf.local, associating the key with the subdomain
- add a minimal zone file to /var/lib/bind/subdomain.dyn.commoninternet.net
- reload bind and return the secret key to the client
*/
use std::process::Command;
use std::io::Error;
use std::io::Write;
use std::string::FromUtf8Error;
use std::{fs::OpenOptions};
use std::fs::File;

const BASE_DOMAIN : &str = "dyn.commoninternet.net";


#[derive(Debug)]
pub enum PeachDynError {
    GenerateTsigIoError(std::io::Error),
    GenerateTsigParseError(std::string::FromUtf8Error),
}

impl From<std::io::Error> for PeachDynError {
    fn from(err: std::io::Error) -> PeachDynError {
        PeachDynError::GenerateTsigIoError(err)
    }
}

impl From<FromUtf8Error> for PeachDynError {
    fn from(err: std::string::FromUtf8Error) -> PeachDynError {
        PeachDynError::GenerateTsigParseError(err)
    }
}


/// helper function to generate a TSIG key file
pub fn generate_tsig_key(full_domain: &str) -> Result<String, PeachDynError> {
    let output = Command::new("/usr/sbin/tsig-keygen")
        .arg("-a")
        .arg("hmac-md5")
        .arg(full_domain)
        .output()?;
    let key_file_text = String::from_utf8(output.stdout)?;
    Ok(key_file_text)
}



fn generate_zone(subdomain: &str) {

    let full_domain=format!("{}.{}", subdomain, BASE_DOMAIN);
    println!("[generating zone for {}]", subdomain);

    // generate key_file_text
    let key_file_text = generate_tsig_key(&full_domain).unwrap();
    println!("key_file_text: {}", key_file_text);

    // write key_file_text to file
    let key_file_path = "/etc/bind/dyn.commoninternet.net.keys";
    let mut file = OpenOptions::new()
        .append(true)
        .open(key_file_path).unwrap();
    if let Err(e) = writeln!(file, "{}", key_file_text) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // append to named.local.conf
    let bind_conf_path = "/etc/bind/named.local.conf";
    let mut file = OpenOptions::new()
        .append(true)
        .open(bind_conf_path).unwrap();
    let zone_section_text = format!("\
        zone \"{full_domain}\" {{
            type master;
            file \"/var/lib/bind/{full_domain}\";
            update-policy {{
	            grant {full_domain} self {full_domain};
            }};
        }};
    ", full_domain=full_domain);
    if let Err(e) = writeln!(file, "{}", zone_section_text) {
        eprintln!("Couldn't write to file: {}", e);
    }



}



fn main() {
    generate_zone("blue");
}
