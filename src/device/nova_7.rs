use crate::{device::Device, error::DeviceError};
use hidapi::{HidApi, HidDevice};
use std::sync::{Arc, atomic::AtomicBool};
use tracing::debug;

pub struct Nova7 {
    dev: HidDevice,
    close: Arc<AtomicBool>,
}

impl Nova7 {
    const VID: u16 = 0x1038;
    const PIDS: [u16; 6] = [
        0x2202, // Arctis Nova 7
        0x2206, // Arctis Nova 7X
        0x2258, // Arctis Nova 7X v2
        0x220a, // Arctis Nova 7P
        0x223a, // Arctis Nova 7 Diablo IV
        0x227a, // Arctis Nova 7 WOW Edition
    ];
    const INTERFACE: i32 = 0x5;

    const MSGLEN: usize = 8;
    const READ_TIMEOUT: i32 = 1000;

    const OPT_CHATMIX: u8 = 0x45;
}

impl Device for Nova7 {
    fn new(api: &HidApi) -> Result<Self, DeviceError> {
        let dev = api
            .device_list()
            .find(|d| {
                d.vendor_id() == Self::VID
                    && Self::PIDS.contains(&d.product_id())
                    && d.interface_number() == Self::INTERFACE
            })
            .ok_or(DeviceError::NotFound)?
            .open_device(api)?;

        Ok(Self {
            dev,
            close: Arc::new(AtomicBool::new(false)),
        })
    }

    fn enable(&self) -> Result<(), DeviceError> {
        Ok(())
    }

    fn disable(&self) -> Result<(), DeviceError> {
        Ok(())
    }

    fn poll_volumes(&self) -> Result<Option<(u8, u8)>, DeviceError> {
        let mut buf = [0u8; Self::MSGLEN];
        let n = self.dev.read_timeout(&mut buf, Self::READ_TIMEOUT)?;
        if n == 0 || buf[0] != Self::OPT_CHATMIX {
            return Ok(None);
        }

        let gamevol = buf[1];
        let chatvol = buf[2];

        debug!("Received volumes: game={}, chat={}", gamevol, chatvol);

        Ok(Some((gamevol, chatvol)))
    }

    fn output_name(&self) -> &'static str {
        if let Ok(device_info) = self.dev.get_device_info() {
            match device_info.product_id() {
                0x2206 | 0x2258 => return "SteelSeries_Arctis_Nova_7X",
                0x220a => return "SteelSeries_Arctis_Nova_7P",
                _ => (),
            }
        }
        "SteelSeries_Arctis_Nova_7"
    }

    fn output_name_pretty(&self) -> String {
        self.output_name()
            .replace("SteelSeries_", "")
            .replace("_", " ")
    }

    fn close_handle(&self) -> Arc<AtomicBool> {
        self.close.clone()
    }
}

impl Drop for Nova7 {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}
