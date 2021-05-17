/* For each subdomain,
- generate a new ddns key (tsig-keygen -a hmac-md5 {{subdomain}}.dyn.commoninternet.net) and append it to /etc/bind/dyn.commoninternet.net.keys
- add a zone section to /etc/bind/named.conf.local, associating the key with the subdomain
- add a minimal zone file to /var/lib/bind/subdomain.dyn.commoninternet.net
- reload bind and return the secret key to the client
*/
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::string::FromUtf8Error;
use tera::{Tera, Context};


const BASE_DOMAIN: &str = "dyn.commoninternet.net";

#[derive(Debug)]
pub enum PeachDynError {
    GenerateTsigIoError(std::io::Error),
    GenerateTsigParseError(std::string::FromUtf8Error),
    DomainAlreadyExistsError(String),
    BindConfigurationError(String),
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

/// helper function to generate the text of a TSIG key file
pub fn generate_tsig_key(full_domain: &str) -> Result<String, PeachDynError> {
    let output = Command::new("/usr/sbin/tsig-keygen")
        .arg("-a")
        .arg("hmac-md5")
        .arg(full_domain)
        .output()?;
    let key_file_text = String::from_utf8(output.stdout)?;
    Ok(key_file_text)
}

/// helper function which returns true if all three conditions are true
/// - no already extant tsig key for the given domain
/// - no zone file for the given domain in /var/lib/bind
/// - no zone section for the given domain in named.conf.local
pub fn check_domain_available(full_domain: &str) -> bool {
    let status1 = Command::new("/bin/grep")
        .arg(full_domain)
        .arg("/etc/bind/named.conf.local")
        .status().expect("error running grep on /etc/bind/named.conf.local");
    let code1 = status1.code().expect("error getting code from grep");
    let status2 = Command::new("/bin/grep")
        .arg(full_domain)
        .arg("/etc/bind/dyn.commoninternet.net.keys")
        .status().expect("error running grep on /etc/bind/dyn.commoninternet.net.keys");
    let code2 = status2.code().expect("error getting code from grep");
    let condition3 = std::path::Path::new(&format!("/var/lib/bind/{}", full_domain)).exists();

    // domain is only available if domain does not exist in either named.conf.local or dyn.commoninternet.netkeys
    // and a file with that name is not found in /var/lib/bind/
    let domain_available = (code1 == 1) & (code2 == 1) & (!condition3);

    // return
    domain_available

}

/// helper function which generates all necessary bind configuration to serve the given
/// subdomain using dynamic DNS authenticated via a new TSIG key which is unique to that subdomain
/// and thus only the possessor of that key can use nsupdate to modify the records
/// for that subodmain
pub fn generate_zone(subdomain: &str) -> Result<String, PeachDynError> {

    // construct full_domain
    let full_domain = format!("{}.{}", subdomain, BASE_DOMAIN);
    println!("[generating zone for {}]", full_domain);

    // first safety check if the domain is available
    let is_available = check_domain_available(&full_domain);
    if !is_available {
        return Err(PeachDynError::DomainAlreadyExistsError(full_domain));
    }

    // generate string with text for TSIG key file
    let key_file_text = generate_tsig_key(&full_domain).expect("failed to generate tsig key");

    // append key_file_text to /etc/bind/dyn.commoninternet.net.keys
    let key_file_path = "/etc/bind/dyn.commoninternet.net.keys";
    let mut file = OpenOptions::new().append(true).open(key_file_path)
        .expect(&format!("failed to open {}", key_file_path));
    if let Err(e) = writeln!(file, "{}", key_file_text) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // append zone section to /etc/bind/named.conf.local
    let bind_conf_path = "/etc/bind/named.conf.local";
    let mut file = OpenOptions::new()
        .append(true)
        .open(bind_conf_path)
        .expect(&format!("failed to open {}", bind_conf_path));
    let zone_section_text = format!(
        "\
        zone \"{full_domain}\" {{
            type master;
            file \"/var/lib/bind/{full_domain}\";
            update-policy {{
	            grant {full_domain} self {full_domain};
            }};
        }};
    ",
        full_domain = full_domain
    );
    if let Err(e) = writeln!(file, "{}", zone_section_text) {
        eprintln!("Couldn't write to file: {}", e);
    }

    // use tera to render the zone file
    let tera = match Tera::new("templates/*.tera") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut context = Context::new();
    context.insert("full_domain", &full_domain);
    let result = tera.render("zonefile.tera", &context).expect("error loading zonefile.tera");

    // write new zone file to /var/lib/bind
    let zone_file_path = format!("/var/lib/bind/{}", full_domain);
    let mut file = File::create(&zone_file_path)
        .expect(&format!("failed to create {}", zone_file_path));
    if let Err(e) = writeln!(file, "{}", result) {
        eprintln!("Couldn't write to file: {}", e)
    };

    // restart bind
    // we use the /etc/sudoers.d/bindctl to allow peach-dyndns user to restart bind as sudo without entering a password
    // using a binary at /bin/reloadbind which runs 'systemctl reload bind9'
    let status = Command::new("sudo")
        .arg("/bin/reloadbind")
        .status().expect("error restarting bind9");
    if !status.success() {
        return Err(PeachDynError::BindConfigurationError("There was an error in the bind configuration".to_string()));
        // TODO: for extra safety consider to revert bind configurations to whatever they were before
    }

    // return success
    Ok(key_file_text)
}

// this main function is only used for the purpose of testing
fn main() {

    let result = generate_zone("purple");
    match result {
        Ok(key_text) => {
            println!("successfully created zone");
            println!("{}", key_text);
        }
        Err(err) => {
            println!("error creating zone {:?}", err);
        }
    }
}
