use colored::*;
use json::JsonValue;
use reqwest::header;
use reqwest::blocking::Client;
use serde_json::json;
use std::fs;
use std::process;
use std::thread::sleep;
use std::time::{Duration};
fn get_external_ip_address(config: &JsonValue) -> &'static str{
    if config["type"].as_str() == Some("A") {
        println!("{}", "IPv4 set.".green());
        return "http://v4.ident.me/";
    } else if config["type"].as_str() == Some("A") {
        println!("{}", "IPv6 set.".green());
        return "http://v6.ident.me/";
    } else {
        panic!(
            "Something is wrong with the config, couldn't set check IP address. Have you touched it? Try running clouddns-setup"
        )
    }
}

fn main() {
    let client = Client::new();
    println!("{} is starting...", "ClouDDNS".yellow());
    println!("{}", "Trying to read the config...".blue());
    let config_file_result = fs::read_to_string("/etc/cloudipsyncr.json");
    let config_file = match config_file_result {
        Ok(config_file) => config_file,
        Err(_error) => {
            println!(
                "{}",
                "Couldn't open file /etc/cloudipsyncr.json, have your run clouddns-setup?".red()
            );
            process::exit(1)
            
        }
    };
    let config_reader = json::parse(&config_file);
    let config = match config_reader {
        Ok(config) => config,
        Err(_) => {
            println!(
                "{}",
                "Couldn't parse config. Try running clouddns-setup.".red()
            );
            process::exit(1)
        }
    };
    let ipchecker=get_external_ip_address(&config);
    println!(
        "{}",
        "Read and parsed the config, entering the loop.".green(),
    );
    let mut headers = header::HeaderMap::new();
    let auth = format!("Bearer {}", config["token"].as_str().unwrap());
    let auth = header::HeaderValue::from_str(&auth);
    let auth = match auth {
        Ok(auth) =>auth,
        Err(_) => panic!("Something went horribly wrong while parsing the API token. Please inform the devs about this.")
    };
    let email = config["email"].as_str().unwrap();
    let email = header::HeaderValue::from_str(&email);
    let email = match email {
        Ok(email) =>email,
        Err(_) => panic!("Something went horribly wrong while parsing the email. Please inform the devs about this.")
    };
    headers.insert(header::AUTHORIZATION, auth);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert("X-Auth-Email", email);
    loop {
        let ip = match reqwest::blocking::get(&*ipchecker){
            Ok(mut resp) => {
                let mut body = String::new();
                if let Err(err) = std::io::Read::read_to_string(&mut resp, &mut body) {
                    println!("Failed to read response body: {}", err);
                    sleep(Duration::new(60, 0));
                    continue;
                }
                body
            }
            Err(err) => {
                eprintln!("Failed to send request: {}", err);
                sleep(Duration::new(60, 0));
                continue;
            }
        };
        let body = json!({
            "type": config["type"].as_str(),
            "name": config["record"].as_str(),
            "content": ip,
            "ttl": 1,
            "proxied": false
        })
        .to_string();
        let cloud =  client.put(format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",config["zoneid"],config["recordid"])).headers(headers.clone()).body(body).send() ;
        match cloud{
                Ok(response) => {
                    println!("API Hit successfully, here's the response: {}",response.text().unwrap());
                }
                Err(_) => {                    println!("Couldn't connect to cloudflare api. (is the gateway down?) Waiting one minute and continuing");
                sleep(Duration::new(60, 0));
                continue;}
            };
        sleep(Duration::new(60, 0));
    }
}
