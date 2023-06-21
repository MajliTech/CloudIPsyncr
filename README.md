# CloudIPsyncr 

[![Build executables](https://github.com/MajliTech/clouddns/actions/workflows/build.yml/badge.svg)](https://github.com/MajliTech/clouddns/actions/workflows/build.yml)
#### A simple tool for changing your CloudFlare powered (sub)domain into a DDNS (sub)domain
---

## Changed name to avoid legal issues 

Next releases will have their name changed to CloudIPsyncr.

## Migrating from ClouDDNS

To migrate, run `curl https://raw.githubusercontent.com/MajliTech/CloudIPsyncr/master/migrate.sh | sh`

---

## What does this do?
This simple program checks your IP every minute, and updates it in CloudFlare. This is very useful when you have a rotating IP address, such when setting up a home server available to the clearnet. 
## How to install it?
### (Soon) using your distro's package manager

---
## Looking for maintainers!
Are you willing to help create those packages? If yes, please reach me at hello@majlitech.pl

---
### Using quick start
In every release, there is a Quick Start command. Just paste it into the terminal and you are done!

### Downloading binaries
1. `wget` or `curl` the latest release and move it's contents to /usr/bin/. 
    - (optional) create a new dir: mkdir cloudipsyncr-binaries && cd cloudipsyncr-binaries
    - run `wget https://github.com/MajliTech/CloudIPsyncr/releases/download/v1.1.1/linux-x64.zip`
    - unzip it: `unzip linux-x64.zip`
    - move the files: `sudo mv cipsyncr-setup /usr/bin`
    - move the files: `sudo mv cloudipsyncr /usr/bin`
2. Make the files executable.
    - run `sudo chmod +x /usr/bin/cloudipsyncr /usr/bin/cipsyncr-setup`
3. Write a simple systemd service (optional)
   - Run `systemctl edit --full --force cloudipsyncr`
   - Paste the following contents
```ini
[Unit]
Description=CloudIPsyncr
After=network-online.target
Wants=network-online.target systemd-networkd-wait-online.service

[Service]
ExecStart=/usr/bin/cloudipsyncr
DynamicUser=yes
After=network-online.target
Wants=network-online.target systemd-networkd-wait-online.service

[Install]
WantedBy=multi-user.target
```
4. Run `cipsyncr-setup`  
    - Follow the instructions on screen
### Build it yourself
#### Manually
Since this is open source, you can build it yourself.
1. Check if you have `cargo` installed
    - If yes, continue
    - If not, install it using instructions on https://rustup.rs/
2. Clone this repo
    - Run `git clone https://github.com/MajliTech/CloudIPsyncr.git`
    - Cd into the dir: `cd CloudIPsyncr`
3. Build it
    - Run `cargo build --release`
    - The binaries are at `./target/release/cipsyncr-setup` and `./target/release/cloudipsyncr`
4. Move the binaries
    - Run `mv ./target/release/cloudipsyncr /usr/bin/`
    - Run `mv ./target/release/cipsyncr-setup /usr/bin/` 
5. Continue from the `Downloading binaries` step 2
#### Automatically
1. run: sudo ./build.sh
    - the script will do everything for you.
## Why was this created?
I just had too much free time left. That's why.
## Any way to sponsor?
Not yet!
