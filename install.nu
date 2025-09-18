print "Compiling"
cargo install --path .

let rules_file = "50-nonar.rules"
let service_file = "nonar.service"

print "Installing udev rule"
sudo cp $rules_file /etc/udev/rules.d/

print "Reloading udev rules"
sudo udevadm control --reload-rules
sudo udevadm trigger

print "Installing service"
mkdir ~/.config/systemd/user
cp $service_file ~/.config/systemd/user/

print "Reloading systemd"
systemctl --user daemon-reload
systemctl --user enable $service_file --now
