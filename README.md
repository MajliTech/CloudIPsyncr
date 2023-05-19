# ClouDDNS
#### A simple tool for changing your CloudFlare powered (sub)domain into a DDNS (sub)domain
## What does this do?
This simple program checks your IP every minute, and updates it in CloudFlare. This is very useful when you have a rotating IP address, such when setting up a home server available to the clearnet. 
## How to install it?
### (Soon) using your distro's package manager
I will soon make some packages for popular distros such as Debian (yes, that also means Ubuntu), Arch and Fedora.
### Downloading binaries
<!-- 0. Elevate to root
    - run `sudo bash`
    - run `cd ~` -->
1. `wget` or `curl` the latest release and move it's contents to /usr/bin/. 
    - (optional) create a new dir: mkdir clouddns-binaries && cd clouddns-binaries
    - run `wget https://github.com/MajliTech/clouddns/releases/download/v1.1.1/linux-x64.zip`
    - unzip it: `unzip linux-x64.zip`
    - move the files: `sudo mv clouddns* /usr/bin`
2. Make the files executable.
    - run `sudo chmod +x /usr/bin/clouddns /usr/bin/clouddns-setup`
3. Write a simple systemd service (optional)
   - Run `systemctl edit --full --force clouddns`
   - Paste the following contents
```ini
[Unit]
Description=ClouDDNS
After=network-online.target
Wants=network-online.target systemd-networkd-wait-online.service

[Service]
ExecStart=/usr/bin/clouddns
DynamicUser=yes
After=network-online.target
Wants=network-online.target systemd-networkd-wait-online.service

[Install]
WantedBy=multi-user.target
```
4. Run `clouddns-setup`
    - Follow the instructions on screen
### Build it yourself
Since this is open source, you can build it yourself.
1. Check if you have `cargo` installed
    - If yes, continue
    - If not, install it using instructions on https://rustup.rs/
2. Clone this repo
    - Run `git clone https://github.com/MajliTech/clouddns.git`
    - Cd into the dir: `cd clouddns`
3. Build it
    - Run `cargo build --release`
    - The binaries are at `./target/release/clouddns` and `./target/release/clouddns-setup`
4. Move the binaries
    - Run `mv ./target/release/clouddns /usr/bin/`
    - Run `mv ./target/release/clouddns-setup /usr/bin/`
5. Continue from the `Downloading binaries` step 2
## Why was this created?
I just had too much free time left. That's why.
## Any way to sponsor?
Not yet!