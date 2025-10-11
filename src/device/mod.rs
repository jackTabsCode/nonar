use hidapi::HidApi;
use nova_7::Nova7;
use nova_pro_wireless::NovaProWireless;
use std::sync::{Arc, atomic::AtomicBool};
use strum::EnumIter;

use crate::error::DeviceError;

mod nova_7;
mod nova_pro_wireless;

pub trait Device {
    fn new(api: &HidApi) -> Result<Self, DeviceError>
    where
        Self: Sized;

    fn enable(&self) -> Result<(), DeviceError>;
    fn disable(&self) -> Result<(), DeviceError>;

    fn poll_volumes(&self) -> Result<Option<(u8, u8)>, DeviceError>;

    fn output_name(&self) -> &'static str;
    fn display_name(&self) -> String;

    fn close_handle(&self) -> Arc<AtomicBool>;
}

#[derive(Debug, EnumIter)]
pub enum DeviceKind {
    NovaProWireless,
    Nova7,
}

impl DeviceKind {
    pub fn probe(&self, api: &HidApi) -> Result<Box<dyn Device>, DeviceError> {
        match self {
            DeviceKind::NovaProWireless => Ok(Box::new(NovaProWireless::new(api)?)),
            DeviceKind::Nova7 => Ok(Box::new(Nova7::new(api)?)),
        }
    }
}
