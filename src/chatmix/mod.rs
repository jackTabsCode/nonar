pub trait ChatMixBackend {
    fn new(names: SinkNames) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn set_volumes(&self, game: u8, chat: u8) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub struct SinkNames {
    pub output: &'static str,
    pub game: &'static str,
    pub chat: &'static str,
}

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub type ChatMix = linux::ChatMix;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub type ChatMix = macos::ChatMix;
