use crate::{
    chatmix::ChatMix,
    device::{Device, DeviceKind},
};
use anyhow::bail;
use hidapi::HidApi;
use std::sync::atomic::Ordering;

mod chatmix;
mod device;

fn main() -> anyhow::Result<()> {
    let api = HidApi::new()?;

    let supported = [DeviceKind::NovaProWireless];

    let mut dev: Option<Box<dyn Device>> = None;
    for kind in supported {
        if let Ok(d) = kind.probe(&api) {
            dev = Some(d);
            break;
        }
    }

    if let Some(dev) = dev {
        let close = dev.close_handle();
        ctrlc::set_handler(move || {
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
