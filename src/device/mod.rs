use crate::chatmix::SinkNames;
use hidapi::HidApi;
use nova_pro_wireless::NovaProWireless;
use std::sync::{Arc, atomic::AtomicBool};

mod nova_pro_wireless;

pub trait Device {
    fn new(api: &HidApi) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn enable(&self) -> anyhow::Result<()>;
    fn disable(&self) -> anyhow::Result<()>;

    fn poll_volumes(&self) -> anyhow::Result<Option<(u8, u8)>>;

    fn sink_names(&self) -> SinkNames;

    fn close_handle(&self) -> Arc<AtomicBool>;
}

pub enum DeviceKind {
    NovaProWireless,
}

impl DeviceKind {
    pub fn probe(&self, api: &HidApi) -> anyhow::Result<Box<dyn Device>> {
        match self {
            DeviceKind::NovaProWireless => Ok(Box::new(NovaProWireless::new(api)?)),
        }
    }
}
