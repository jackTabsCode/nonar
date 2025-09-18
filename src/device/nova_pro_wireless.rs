use crate::{chatmix::SinkNames, device::Device};
use anyhow::Context;
use hidapi::{HidApi, HidDevice};
use std::sync::{Arc, atomic::AtomicBool};

pub struct NovaProWireless {
    dev: HidDevice,
    close: Arc<AtomicBool>,
}

impl NovaProWireless {
    const VID: u16 = 0x1038;
    const PID: u16 = 0x12E0;
    const INTERFACE: i32 = 0x4;

    const MSGLEN: usize = 63;
    const READ_TIMEOUT: i32 = 1000;

    const OPT_CHATMIX: u8 = 0x45;

    const TX: u8 = 0x6;
    const OPT_CHATMIX_ENABLE: u8 = 0x49;
    const OPT_SONAR_ICON: u8 = 0x8D;

    fn write_msg(&self, bytes: &[u8]) -> anyhow::Result<()> {
        let mut data = [0u8; Self::MSGLEN];
        let len = bytes.len().min(Self::MSGLEN);
        data[..len].copy_from_slice(bytes);

        self.dev.write(&data).context("Failed to write message")?;

        Ok(())
    }

    fn set_chatmix(&self, state: bool) -> anyhow::Result<()> {
        self.write_msg(&[Self::TX, Self::OPT_CHATMIX_ENABLE, state as u8])
    }

    fn set_sonar_icon(&self, state: bool) -> anyhow::Result<()> {
        self.write_msg(&[Self::TX, Self::OPT_SONAR_ICON, state as u8])
    }
}

impl Device for NovaProWireless {
    fn new(api: &HidApi) -> anyhow::Result<Self> {
        let dev = api
            .device_list()
            .find(|d| {
                d.vendor_id() == Self::VID
                    && d.product_id() == Self::PID
                    && d.interface_number() == Self::INTERFACE
            })
            .context("Device not found")?
            .open_device(api)
            .context("Failed to open device")?;

        Ok(Self {
            dev,
            close: Arc::new(AtomicBool::new(false)),
        })
    }

    fn enable(&self) -> anyhow::Result<()> {
        self.set_sonar_icon(true)?;
        self.set_chatmix(true)?;
        Ok(())
    }

    fn disable(&self) -> anyhow::Result<()> {
        self.set_sonar_icon(false)?;
        self.set_chatmix(false)?;
        Ok(())
    }

    fn poll_volumes(&self) -> anyhow::Result<Option<(u8, u8)>> {
        let mut buf = [0u8; Self::MSGLEN];
        let n = self.dev.read_timeout(&mut buf, Self::READ_TIMEOUT)?;
        if n == 0 || buf[1] != Self::OPT_CHATMIX {
            return Ok(None);
        }

        let gamevol = buf[2];
        let chatvol = buf[3];
        Ok(Some((gamevol, chatvol)))
    }

    fn sink_names(&self) -> crate::chatmix::SinkNames {
        SinkNames {
            output: "SteelSeries_Arctis_Nova_Pro_Wireless",
            game: "NovaProWirelessGame",
            chat: "NovaProWirelessChat",
        }
    }

    fn close_handle(&self) -> Arc<AtomicBool> {
        self.close.clone()
    }
}

impl Drop for NovaProWireless {
    fn drop(&mut self) {
        let _ = self.disable();
    }
}
