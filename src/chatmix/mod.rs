pub trait ChatMixBackend {
    fn new(output_name: &'static str) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn set_volumes(&self, game: u8, chat: u8) -> anyhow::Result<()>;
}

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::ChatMix;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use self::macos::ChatMix;
