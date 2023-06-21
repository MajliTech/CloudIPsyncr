if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi
set -e
FILE=$HOME/.cargo/env
if test -f "$HOME/.cargo/env"; then
    source $HOME/.cargo/env
fi
command -v rustup >/dev/null 2>&1 ||

{ echo >&2 "rustup is not installed. installing";
DEPORT_RUST=1
curl https://sh.rustup.rs -sSf | sh -s -- -y
    source $HOME/.cargo/env

}
echo Welcome to this ClouDDNS installer!

echo Building ClouDDNS
cargo build --release
cp target/release/cloudipsyncr /usr/bin
chmod +x /usr/bin/cloudipsyncr
cp target/release/cipsyncr-setup /usr/bin
chmod +x /usr/bin/cipsyncr-setup
echo Compiled and installed successfully
echo running cloudipsyncr-setup
cloudipsyncr-setup
if [ $DEPORT_RUST -eq 1 ]
then
echo Uninstalling rustup...
rustup self uninstall -y

fi
