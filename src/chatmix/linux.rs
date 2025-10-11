use tracing::info;

use crate::chatmix::ChatMixBackend;
use crate::error::ChatMixError;
use crate::{CHAT_SINK_NAME, GAME_SINK_NAME};
use std::process::{Child, Command, Stdio};

const CMD_PACTL: &str = "pactl";
const CMD_PWLOOPBACK: &str = "pw-loopback";

#[derive(Debug)]
pub struct ChatMix {
    game_proc: Child,
    chat_proc: Child,
}

impl ChatMix {
    fn set_sink_volume(&self, sink: &str, vol: u8) -> Result<(), ChatMixError> {
        Command::new(CMD_PACTL)
            .args([
                "set-sink-volume",
                &format!("input.{sink}"),
                &format!("{vol}%"),
            ])
            .status()?;
        Ok(())
    }
}

impl ChatMixBackend for ChatMix {
    fn new(output_name: &'static str) -> Result<Self, ChatMixError> {
        // Get all audio sinks
        let pactl_child = Command::new(CMD_PACTL)
            .args(["list", "sinks", "short"])
            .stdout(Stdio::piped())
            .spawn()?;
        // Only the ones with our output_name
        let grep_child = Command::new("grep")
            .arg(output_name)
            .stdin(Stdio::from(pactl_child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        // If there are multiple, sort newest connected to the top
        let sort_child = Command::new("sort")
            .args(["-h", "-r"])
            .stdin(Stdio::from(grep_child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        // Only the actual name of the sink
        let cut_child = Command::new("cut")
            .arg("-f2")
            .stdin(Stdio::from(sort_child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        // Just the top (newest) one
        let head_child = Command::new("head")
            .arg("-n1")
            .stdin(Stdio::from(cut_child.stdout.unwrap()))
            .stdout(Stdio::piped())
            .spawn()?;
        let output = head_child.wait_with_output()?;
        let playback_sink = str::from_utf8(&output.stdout).unwrap().trim();
        info!("Mapping ChatMix to \"{playback_sink}\"");

        let game_proc = Command::new(CMD_PWLOOPBACK)
            .args([
                "-P",
                playback_sink,
                "--capture-props=media.class=Audio/Sink",
                "-n",
                GAME_SINK_NAME,
            ])
            .spawn()?;

        let chat_proc = Command::new(CMD_PWLOOPBACK)
            .args([
                "-P",
                playback_sink,
                "--capture-props=media.class=Audio/Sink",
                "-n",
                CHAT_SINK_NAME,
            ])
            .spawn()?;

        Ok(Self {
            game_proc,
            chat_proc,
        })
    }

    fn set_volumes(&self, game: u8, chat: u8) -> Result<(), ChatMixError> {
        self.set_sink_volume(GAME_SINK_NAME, game)?;
        self.set_sink_volume(CHAT_SINK_NAME, chat)?;

        Ok(())
    }
}

impl Drop for ChatMix {
    fn drop(&mut self) {
        let _ = self.game_proc.kill();
        let _ = self.chat_proc.kill();
    }
}
