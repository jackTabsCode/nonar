use crate::{
    chatmix::{ChatMix, ChatMixBackend},
    device::{Device, DeviceKind},
};
use anyhow::bail;
use hidapi::HidApi;
use std::sync::atomic::Ordering;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub const CHAT_SINK_NAME: &str = "NonarChat";
pub const GAME_SINK_NAME: &str = "NonarGame";

mod chatmix;
mod device;

fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    let journald = tracing_journald::layer()?;

    let registry = tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")));

    #[cfg(target_os = "linux")]
    let registry = registry.with(journald);

    #[cfg(not(target_os = "linux"))]
    let registry = registry.with(tracing_subscriber::fmt::layer());

    registry.init();

    info!("Starting Nonar");

    let api = HidApi::new()?;

    let supported = [DeviceKind::NovaProWireless];

    let mut dev: Option<Box<dyn Device>> = None;
    for kind in supported {
        debug!("Probing device: {:?}", kind);
        if let Ok(d) = kind.probe(&api) {
            info!("Found device: {:?}", kind);
            dev = Some(d);
            break;
        }
    }

    if let Some(dev) = dev {
        let close = dev.close_handle();
        ctrlc::set_handler(move || {
            info!("Received ctrl-c");
            close.store(true, Ordering::SeqCst);
        })?;

        run_device(&*dev)?;
    } else {
        send_notification("No supported device found");
        bail!("No supported device found");
    }

    Ok(())
}

fn run_device(dev: &dyn Device) -> anyhow::Result<()> {
    let chatmix = ChatMix::new(dev.output_name())?;
    dev.enable()?;

    send_notification("ChatMix is now running!");

    let close = dev.close_handle();

    loop {
        if close.load(Ordering::SeqCst) {
            break;
        }

        if let Some((game, chat)) = dev.poll_volumes()?
            && let Err(err) = chatmix.set_volumes(game, chat)
        {
            send_notification("Failed to set ChatMix volumes!");
            return Err(err);
        }
    }

    dev.disable()?;

    Ok(())
}

fn send_notification(body: &str) {
    let _ = notify_rust::Notification::new()
        .summary("Nonar")
        .body(body)
        .show();
}
