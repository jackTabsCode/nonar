use derive_more::Display;
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

    fn output_name(&self) -> &'static str;

    fn close_handle(&self) -> Arc<AtomicBool>;
}

#[derive(Debug, Display)]
pub enum DeviceKind {
    #[display("Nova Pro Wireless")]
    NovaProWireless,
}

impl DeviceKind {
    pub fn probe(&self, api: &HidApi) -> anyhow::Result<Box<dyn Device>> {
        match self {
            DeviceKind::NovaProWireless => Ok(Box::new(NovaProWireless::new(api)?)),
        }
    }
}
