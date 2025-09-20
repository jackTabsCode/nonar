use hidapi::HidError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("chatmix error: {0}")]
    ChatMix(#[from] ChatMixError),

    #[error("device error: {0}")]
    Device(#[from] DeviceError),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Debug, Error)]
pub enum ChatMixError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[cfg(target_os = "macos")]
    #[error("sink not found: {sink}")]
    SinkNotFound { sink: String },

    #[cfg(target_os = "macos")]
    #[error("CoreAudio error: {0}")]
    CoreAudioError(String),
}

#[derive(Debug, Error)]
pub enum DeviceError {
    #[error("device not found")]
    NotFound,

    #[error("hidapi error: {0}")]
    Hid(#[from] HidError),
}
