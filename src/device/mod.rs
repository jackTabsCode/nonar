use derive_more::Display;
use hidapi::HidApi;
use nova_pro_wireless::NovaProWireless;
use std::sync::{Arc, atomic::AtomicBool};

use crate::error::DeviceError;

mod nova_pro_wireless;

pub trait Device {
    fn new(api: &HidApi) -> Result<Self, DeviceError>
    where
        Self: Sized;

    fn enable(&self) -> Result<(), DeviceError>;
    fn disable(&self) -> Result<(), DeviceError>;

    fn poll_volumes(&self) -> Result<Option<(u8, u8)>, DeviceError>;

    fn output_name(&self) -> &'static str;

    fn close_handle(&self) -> Arc<AtomicBool>;
}

#[derive(Debug, Display)]
pub enum DeviceKind {
    #[display("Nova Pro Wireless")]
    NovaProWireless,
}

impl DeviceKind {
    pub fn probe(&self, api: &HidApi) -> Result<Box<dyn Device>, DeviceError> {
        match self {
            DeviceKind::NovaProWireless => Ok(Box::new(NovaProWireless::new(api)?)),
        }
    }
}
