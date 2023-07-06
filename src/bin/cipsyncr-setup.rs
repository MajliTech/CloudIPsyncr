use colored::*;
use input_macro::*;
use std::io::{stdout, Write};
use termion::input::TermRead;

use is_root::*;
use reqwest::header::{self, HeaderValue};
use termion::raw::IntoRawMode;
fn wait_for_keypress(message: &str) {
    println!("{}", message);

    let stdin = std::io::stdin();

    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();

    for event in stdin.keys() {
        if let Ok(_key) = event {
            print!("\n");
            break;
        }
    }
}
fn check_ip_version(version_type: String) {
    if version_type=="A"{
        match reqwest::blocking::get("https://v4.ident.me/") {
            Ok(_) => {return;},
            Err(_) => {println!("{}\n{}: try changing the CloudFlare record from A to AAAA or\n      check if you have internet connection.","Couldn't get your IP (version 4). Does your software really support it?\n".red(),"hint".yellow().bold())}
        };
    } else if version_type=="AAAA"{
        match reqwest::blocking::get("https://v6.ident.me/") {
            Ok(_) => {return;},
            Err(_) => {println!("{}\n{}: try changing the CloudFlare record from AAAA to A or\n      check if you have internet connection.","Couldn't get your IP (version 6). Does your software really support it?\n".red(),"hint".yellow().bold())}
        };
    }
}
fn main() {
    let client = reqwest::blocking::Client::new();

    let mut config = json::JsonValue::new_object();
    if !is_root() {
        println!(
            "{}",
            "Hey my freind, you should be running me as root.".red()
        );
        std::process::exit(1)
    }
    println!("{}", "Welcome to CloudIPsyncr setup!".green());
    println!("Before we begin, head over to https://dash.cloudflare.com/profile/api-tokens and generate an API token for this program.");
    let key = input!("After acquiring the key, paste it here: ");
    config["token"] = json::JsonValue::String(key);
    println!("Ok, now enter the email address that is associated with this account.");
    let email = input!("Email address: ");
    let mut headers = header::HeaderMap::new();
    let auth = format!("Bearer {}", config["token"].as_str().unwrap());
    let auth = header::HeaderValue::from_str(&auth);
    let auth = match auth {
        Ok(auth) =>auth,
        Err(_) => panic!("Something went horribly wrong while parsing the API token. Please inform the devs about this.")
    };
    headers.insert(header::AUTHORIZATION, auth);
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );
    config["email"] = json::JsonValue::String(email);
    // config["type"] = json::JsonValue::String("A".to_string());
    let response = match client
        .get("https://api.cloudflare.com/client/v4/user/tokens/verify")
        .headers(headers.clone())
        .send()
    {
        Ok(resp) => {
            let text = match resp.text(){
                Ok(text) => text,
                Err(_) => panic!("Something went wrong while parsing the response from CloudFlare. (report to devs)")
            };
            match json::parse(&text) {
                Ok(object) => object,
                Err(_) => panic!("Couldnt parse JSON from CloudFlare. Report to devs."),
            }
        }
        Err(_err) => {
            println!("Couldn't connect. (is the gateway down?)");
            std::process::exit(1)
        }
    };
    if response["success"].as_bool().unwrap() {
        if response["result"]["status"].as_str().unwrap() == "active" {
            println!(
                "{}",
                "CloudFlare has confirmed your API key, let's continue.".green()
            )
        }
    } else {
        println!(
            "{}",
            "CloudFlare didn't return a success, either couldn't verify API key or has deemed your key invalid (report to devs if this is the first case).".red()
        );
        std::process::exit(1)
    }
    headers.insert(
        "X-Auth-Email",
        match HeaderValue::from_str(config["email"].clone().as_str().unwrap()) {
            Ok(header) => header,
            Err(_) => {
                panic!("Something went wrong when creating an email header. (report to devs)")
            }
        },
    );
    println!("I need to know, which zone do you want to update.\nOpen up the domain panel, and look for the Zone ID.");
    config["zoneid"] = json::JsonValue::String(input!("Once you found it, paste it here: "));
    headers.insert(
        "X-Auth-Email",
        match HeaderValue::from_str(config["zoneid"].clone().as_str().unwrap()) {
            Ok(header) => header,
            Err(_) => {
                panic!("Something went wrong when creating a zoneid header. (report to devs)")
            }
        },
    );
    print!("\n");
    wait_for_keypress("Ok, now, create the subdomain you want to update (for now fill it with 0.0.0.0 for an A record or :: for an AAAA).\nIf it's alredy created, give it a comment containing exactly `cloudipsyncr` (no quotation marks).\nIf you have done that, press any key to continue.");
    let response = match client
        .get(format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?comment=cloudipsyncr",
            config["zoneid"]
        ))
        .headers(headers.clone())
        .send()
    {
        Ok(resp) => {
            let text = match resp.text(){
                Ok(text) => text,
                Err(_) => panic!("Something went wrong while parsing the response from CloudFlare. (report to devs)")
            };
            match json::parse(&text) {
                Ok(object) => object,
                Err(_) => panic!("Couldnt parse JSON from CloudFlare. Report to devs."),
            }
        }
        Err(_err) => {
            println!("Couldn't connect. (is the gateway down?)");
            std::process::exit(1)
        }
    };
    if response["success"].as_bool().unwrap() {
        if response["result"].members().len() == 1 {
            let url = response["result"][0]["name"].as_str().unwrap();
            println!("Autodetected {}, will use that.", url.green());
            config["record"] = json::JsonValue::String(url.to_string());
            config["recordid"] =
                json::JsonValue::String(response["result"][0]["id"].as_str().unwrap().to_string());
            println!("{}",response);
            if response["result"][0]["type"].as_str().unwrap().to_string()!="A" || response["result"][0]["type"].as_str().unwrap().to_string()!="AAAA" {
                check_ip_version(response["result"][0]["type"].as_str().unwrap().to_string());
                config["type"]=json::JsonValue::String(response["result"][0]["id"].as_str().unwrap().to_string());
            }
        } else {
            println!(
                "Didn't detect anything or detected too much. Did you configure your DNS properly?"
            );
            std::process::exit(1)
        }
    } else {
        println!("{}","Something went wrone while listing DNS records. (Does the API key have the correct permissions?)".red());
        std::process::exit(1)
    }
    let biding = &config.dump();
    let config_dump = biding.as_bytes();
    match std::fs::write("/etc/cloudipsyncr.json", config_dump.clone()) {
        Err(_) => panic!(
            "Even with root, something went wrong while writing the file. Check your system."
        ),
        Ok(_) => {}
    };
    println!("{} ClouDDNS has been now set up. If you have installed this using your package manager:\nTo make this start on system boot, run:\n   sudo systemctl enable cloudipsyncr\nTo start now AND on system boot:\n   sudo systemctl enable --now clouddns","Hooray!".yellow());
}
