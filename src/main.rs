use crate::{
    chatmix::{ChatMix, ChatMixBackend},
    device::{Device, DeviceKind},
};
use hidapi::HidApi;
use notify_rust::Notification;
use std::{sync::atomic::Ordering, thread, time::Duration};
use tracing::{debug, info, trace, warn};
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

    loop {
        match try_probe(&api) {
            Some((dev, kind)) => {
                info!("Found device: {kind:?}");

                let close = dev.close_handle();
                if let Err(e) = run_device(&*dev, &kind) {
                    debug!("run_device failed: {e:?}");
                }

                notify(&format!("{kind} connected"));

                info!("Device disconnected, waiting before retry");
                thread::sleep(Duration::from_secs(2));

                close.store(true, Ordering::SeqCst);
            }
            None => {
                debug!("No supported device found, retrying in 5s");
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

fn try_probe(api: &HidApi) -> Option<(Box<dyn Device>, DeviceKind)> {
    let supported = [DeviceKind::NovaProWireless];

    for kind in supported {
        trace!("Probing device: {kind:?}");

        if let Ok(dev) = kind.probe(api) {
            return Some((dev, kind));
        }
    }

    None
}

fn run_device(dev: &dyn Device, kind: &DeviceKind) -> anyhow::Result<()> {
    let chatmix = ChatMix::new(dev.output_name())?;
    dev.enable()?;

    notify(&format!("{kind} connected"));

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
