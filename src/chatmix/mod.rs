pub trait ChatMixBackend {
    fn new(output_name: &'static str) -> Result<Self, ChatMixError>
    where
        Self: Sized;

    fn set_volumes(&self, game: u8, chat: u8) -> Result<(), ChatMixError>;
}

#[cfg(target_os = "linux")]
mod linux;

use crate::error::ChatMixError;

#[cfg(target_os = "linux")]
pub use self::linux::ChatMix;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use self::macos::ChatMix;
