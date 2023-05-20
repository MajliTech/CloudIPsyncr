sudo mv /etc/clouddns.json /etc/cloudipsyncr.json
sudo rm /usr/bin/cloudipsyncr
sudo rm /usr/bin/cipsyncr-setup
echo "You will now need to fetch the latest release of cloudipsyncr"
echo "This script will download v1.1.3"
cd ~; mkdir cloudipsyncr; cd cloudipsyncr; wget https://github.com/MajliTech/CloudIPsyncr/releases/download/v1.1.3/linux-x64.zip ; unzip linux-x64; sudo mv cloudipsyncr /usr/bin/; sudo mv cipsyncr-setup /usr/bin; sudo chmod +x /usr/bin/cloudipsyncr; sudo chmod +x /usr/bincipsyncr-setup;
echo "If you use the old systemd service, you need to update it."
echo "Run sudo systemctl edit --force --full cloudipsyncr and paste the following contents: " 
echo "[Unit]"
echo "Description=CloudIPsyncr"
echo "After=network-online.target"
echo "Wants=network-online.target systemd-networkd-wait-online.service"
echo ""
echo "[Service]"
echo "ExecStart=/usr/bin/cloudipsyncr"
echo "DynamicUser=yes"
echo "After=network-online.target"
echo "Wants=network-online.target systemd-networkd-wait-online.service"
echo ""
echo "[Install]"
echo "WantedBy=multi-user.target"
echo "You might need to remove the the old service: rm /etc/systemd/system/clouddns.service"