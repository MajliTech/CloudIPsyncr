sudo mv /etc/clouddns.json /etc/cloudipsyncr.json
sudo rm /usr/bin/cloudipsyncr
sudo rm /usr/bin/cipsyncr-setup
echo "You will now need to fetch the latest release of cloudipsyncr"
echo "This script will download v1.1.3"
cd ~; mkdir cloudipsyncr; cd cloudipsyncr; wget https://github.com/MajliTech/CloudIPsyncr/releases/download/v1.1.3/linux-x64.zip ; unzip linux-x64; sudo mv cloudipsyncr /usr/bin/; sudo mv cipsyncr-setup /usr/bin; sudo chmod +x /usr/bin/cloudipsyncr; sudo chmod +x cipsyncr-setup;