use crate::{
    chatmix::{ChatMix, ChatMixBackend},
    device::{Device, DeviceKind},
    error::{DeviceError, Error},
};
use hidapi::HidApi;
use notify_rust::Notification;
use std::{sync::atomic::Ordering, thread, time::Duration};
use strum::IntoEnumIterator;
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

pub const CHAT_SINK_NAME: &str = "NonarChat";
pub const GAME_SINK_NAME: &str = "NonarGame";

mod chatmix;
mod device;
mod error;

fn main() -> Result<(), Error> {
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

    let mut api = HidApi::new().map_err(DeviceError::Hid)?;

    loop {
        match try_probe(&api) {
            Some((dev, _)) => {
                info!("{} found", dev.display_name());

                let close = dev.close_handle();

                if let Err(e) = run_device(&*dev) {
                    error!("run_device failed: {e:?}");

                    if let Error::ChatMix(_) = e {
                        std::process::exit(1);
                    }
                }

                info!("{} disconnected", dev.display_name());
                notify(&format!("{} disconnected", dev.display_name()));

                close.store(true, Ordering::SeqCst);
            }
            None => {
                info!("No supported device found, retrying in 5s");
                thread::sleep(Duration::from_secs(5));

                api.refresh_devices().map_err(DeviceError::Hid)?;
            }
        }
    }
}

fn try_probe(api: &HidApi) -> Option<(Box<dyn Device>, DeviceKind)> {
    for kind in DeviceKind::iter() {
        trace!("Probing device: {kind:?}");

        match kind.probe(api) {
            Ok(dev) => return Some((dev, kind)),
            Err(e) => debug!("Failed to probe device: {e:?}"),
        }
    }

    None
}

fn run_device(dev: &dyn Device) -> Result<(), Error> {
    let chatmix = ChatMix::new(dev.output_name())?;
    dev.enable()?;

    notify(&format!("{} connected", dev.display_name()));

    let close = dev.close_handle();

    loop {
        if close.load(Ordering::SeqCst) {
            break;
        }

        match dev.poll_volumes() {
            Ok(Some((game, chat))) => {
                if let Err(e) = chatmix.set_volumes(game, chat) {
                    warn!("set_volumes failed: {e:?}");
                    break;
                }
                trace!("Volumes set: game={game}, chat={chat}");
            }
            Ok(None) => {}
            Err(e) => {
                warn!("poll_volumes failed: {e:?}");
                break;
            }
        }
    }

    dev.disable()?;

    Ok(())
}

fn notify(body: &str) {
    let _ = Notification::new().summary("Nonar").body(body).show();
}
