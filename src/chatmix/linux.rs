use crate::chatmix::ChatMixBackend;
use crate::{CHAT_SINK_NAME, GAME_SINK_NAME};
use std::process::{Child, Command};
use tracing::{info, trace};

const CMD_PACTL: &str = "pactl";
const CMD_PWLOOPBACK: &str = "pw-loopback";

#[derive(Debug)]
pub struct ChatMix {
    game_proc: Child,
    chat_proc: Child,
}

impl ChatMix {
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

impl ChatMixBackend for ChatMix {
    fn new(output_name: &'static str) -> anyhow::Result<Self> {
        let game_proc = Command::new(CMD_PWLOOPBACK)
            .args([
                "-P",
                output_name,
                "--capture-props=media.class=Audio/Sink",
                "-n",
                GAME_SINK_NAME,
            ])
            .spawn()?;

        let chat_proc = Command::new(CMD_PWLOOPBACK)
            .args([
                "-P",
                output_name,
                "--capture-props=media.class=Audio/Sink",
                "-n",
                CHAT_SINK_NAME,
            ])
            .spawn()?;

        info!("Created sinks");

        Ok(Self {
            game_proc,
            chat_proc,
        })
    }

    fn set_volumes(&self, game: u8, chat: u8) -> anyhow::Result<()> {
        trace!("Setting volumes: game={game}, chat={chat}");

        self.set_sink_volume(GAME_SINK_NAME, game)?;
        self.set_sink_volume(CHAT_SINK_NAME, chat)?;

        Ok(())
    }
}
