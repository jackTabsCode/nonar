use std::process::{Child, Command};

use tracing::{info, trace};

const CMD_PACTL: &str = "pactl";
const CMD_PWLOOPBACK: &str = "pw-loopback";

#[derive(Debug)]
pub struct SinkNames {
    pub output: &'static str,
    pub game: &'static str,
    pub chat: &'static str,
}

#[derive(Debug)]
pub struct ChatMix {
    names: SinkNames,
    game_proc: Child,
    chat_proc: Child,
}

impl ChatMix {
    pub fn new(names: SinkNames) -> anyhow::Result<Self> {
        info!("Creating sinks: {:?}", names);

        let game_proc = Command::new(CMD_PWLOOPBACK)
            .args([
                "-P",
                names.output,
                "--capture-props=media.class=Audio/Sink",
                "-n",
                names.game,
            ])
            .spawn()?;

        let chat_proc = Command::new(CMD_PWLOOPBACK)
            .args([
                "-P",
                names.output,
                "--capture-props=media.class=Audio/Sink",
                "-n",
                names.chat,
            ])
            .spawn()?;

        Ok(Self {
            names,
            game_proc,
            chat_proc,
        })
    }

    pub fn set_volumes(&self, game: u8, chat: u8) -> anyhow::Result<()> {
        trace!("Setting volumes: game={}, chat={}", game, chat);

        self.set_sink_volume(self.names.game, game)?;
        self.set_sink_volume(self.names.chat, chat)?;

        Ok(())
    }

    fn set_sink_volume(&self, sink: &str, vol: u8) -> anyhow::Result<()> {
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

impl Drop for ChatMix {
    fn drop(&mut self) {
        let _ = self.game_proc.kill();
        let _ = self.chat_proc.kill();
    }
}
