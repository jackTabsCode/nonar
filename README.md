# nonar

ChatMix for Linux and macOS, without SteelSeries Sonar!

## Supported Devices

- SteelSeries Arctis Nova Pro Wireless

## Installation

First, you'll need [the Rust toolchain](https://rustup.rs/) because you're compiling this yourself.

Compile the program and install it into your Cargo bin:

```bash
cargo install --path .
```

### Linux

Figure out how to install these on your distro:

- PipeWire
- PulseAudio utilities

Copy `50-nonar.rules` to `/etc/udev/rules.d` and reload udev rules:

```bash
sudo cp linux/50-nonar.rules /etc/udev/rules.d/

sudo udevadm control --reload-rules
sudo udevadm trigger
```

Add and enable the systemd service:

```bash
	mkdir -p ~/.config/systemd/user
	cp linux/nonar.service ~/.config/systemd/user/

	systemctl --user daemon-reload
	systemctl --user enable nonar.service --now
```

### macOS

Nonar works on macOS, but it's more involved. You'll need to create two virtual devices that support volume control.

The easiest way to do this is with [Loopback](https://rogueamoeba.com/loopback/), but you'll need to fork over $100. Sadge. I don't know of any other way at the moment.

Once you've installed and set up Loopback, create two new virtual devices: `NonarGame` and `NonarChat`. Add your headset as a monitor for each.

Then, add and enable the launchd service:

```bash
cp macos/nonar.plist ~/Library/LaunchAgents/

launchctl load nonar
launchctl start nonar
```

## Troubleshooting

There's some basic logging implemented, so check it out to see if there's any obvious issues.

### Linux

```bash
journalctl --user -u nonar.service -f
```

### macOS

```bash
tail -f /tmp/nonar.log /tmp/nonar.err
```

## Attributions

[nova-chatmix-linux by Dymstro](https://github.com/Dymstro/nova-chatmix-linux) provided the initial implementation in a Python script for the Arctis Nova Pro Wireless on Linux.
