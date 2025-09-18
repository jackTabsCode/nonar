use tracing::{info, trace};

use crate::chatmix::{ChatMixBackend, SinkNames};
use std::process::{Child, Command};

#[derive(Debug)]
pub struct ChatMix {}

impl ChatMix {}

impl ChatMixBackend for ChatMix {
    fn new(names: SinkNames) -> anyhow::Result<Self> {
        unimplemented!()
    }

    fn set_volumes(&self, game: u8, chat: u8) -> anyhow::Result<()> {
        unimplemented!()
    }
}
