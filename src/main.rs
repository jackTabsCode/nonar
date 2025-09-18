use crate::{
    chatmix::ChatMix,
    device::{Device, DeviceKind},
};
use anyhow::bail;
use hidapi::HidApi;
use std::sync::atomic::Ordering;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod chatmix;
mod device;

fn main() -> anyhow::Result<()> {
    let journald = tracing_journald::layer()?;

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(journald)
        .init();

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
        bail!("No supported device found");
    }

    Ok(())
}

fn run_device(dev: &dyn Device) -> anyhow::Result<()> {
    let chatmix = ChatMix::new(dev.sink_names())?;
    dev.enable()?;

    let close = dev.close_handle();

    loop {
        if close.load(Ordering::SeqCst) {
            break;
        }

        if let Some((game, chat)) = dev.poll_volumes()? {
            chatmix.set_volumes(game, chat)?;
        }
    }

    dev.disable()?;

    Ok(())
}
