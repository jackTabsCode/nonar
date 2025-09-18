let rules_file = "50-nonar.rules"
let service_name = "nonar.service"
let service_path = $"($env.HOME)/.config/systemd/user/($service_name)"

print "Compiling"
cargo install --path .

print "Installing udev rule"
sudo cp $rules_file /etc/udev/rules.d/

print "Reloading udev rules"
sudo udevadm control --reload-rules
sudo udevadm trigger

print "Stopping old service if running"
systemctl --user stop $service_name | ignore
systemctl --user disable $service_name | ignore
systemctl --user reset-failed $service_name | ignore

print "Installing service"
mkdir ~/.config/systemd/user
cp $service_name $service_path

print "Reloading systemd"
systemctl --user daemon-reload
systemctl --user enable --now $service_name
