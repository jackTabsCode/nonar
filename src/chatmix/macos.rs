use crate::chatmix::ChatMixBackend;
use crate::{CHAT_SINK_NAME, GAME_SINK_NAME};
use anyhow::{Context, Result, bail};
use core_foundation::string::CFString;
use coreaudio_sys::{
    AudioObjectGetPropertyData, AudioObjectGetPropertyDataSize, AudioObjectID,
    AudioObjectPropertyAddress, AudioObjectSetPropertyData, kAudioDevicePropertyScopeOutput,
    kAudioDevicePropertyVolumeScalar, kAudioHardwarePropertyDevices,
    kAudioObjectPropertyElementMain, kAudioObjectPropertyName, kAudioObjectPropertyScopeGlobal,
    kAudioObjectSystemObject,
};
use std::{mem, ptr};
use tracing::{info, trace};

#[derive(Debug)]
pub struct ChatMix {
    game_id: AudioObjectID,
    chat_id: AudioObjectID,
}

impl ChatMixBackend for ChatMix {
    fn new(_output_name: &'static str) -> Result<Self> {
        let devices = list_audio_devices()?;

        let game_id =
            find_device_by_name(&devices, GAME_SINK_NAME).context("Game sink not found")?;
        let chat_id =
            find_device_by_name(&devices, CHAT_SINK_NAME).context("Chat sink not found")?;

        info!("Found game sink: {game_id}, chat sink: {chat_id}");

        Ok(Self { game_id, chat_id })
    }

    fn set_volumes(&self, game: u8, chat: u8) -> Result<()> {
        trace!("Setting volumes: game={game}, chat={chat}");

        set_device_volume(self.game_id, game)?;
        set_device_volume(self.chat_id, chat)?;

        Ok(())
    }
}

fn list_audio_devices() -> Result<Vec<AudioObjectID>> {
    get_property_data::<AudioObjectID>(
        kAudioObjectSystemObject,
        &addr(
            kAudioHardwarePropertyDevices,
            kAudioObjectPropertyScopeGlobal,
            kAudioObjectPropertyElementMain,
        ),
    )
}

fn find_device_by_name(devices: &[AudioObjectID], target: &str) -> Option<AudioObjectID> {
    let name_addr = addr(
        kAudioObjectPropertyName,
        kAudioObjectPropertyScopeGlobal,
        kAudioObjectPropertyElementMain,
    );

    devices.iter().copied().find(|&device| {
        match get_property_data::<CFString>(device, &name_addr) {
            Ok(names) => names[0].to_string().contains(target),
            _ => false,
        }
    })
}

fn set_device_volume(device: AudioObjectID, vol: u8) -> Result<()> {
    let volume = vol as f32 / 100.0;

    let addr = addr(
        kAudioDevicePropertyVolumeScalar,
        kAudioDevicePropertyScopeOutput,
        kAudioObjectPropertyElementMain,
    );

    set_property_data(device, &addr, &volume)
}

fn addr(selector: u32, scope: u32, element: u32) -> AudioObjectPropertyAddress {
    AudioObjectPropertyAddress {
        mSelector: selector,
        mScope: scope,
        mElement: element,
    }
}

fn get_property_data<T>(
    object: AudioObjectID,
    addr: &AudioObjectPropertyAddress,
) -> Result<Vec<T>> {
    let mut size = 0;
    let status = unsafe { AudioObjectGetPropertyDataSize(object, addr, 0, ptr::null(), &mut size) };

    if status != 0 {
        bail!("Failed to get property data size: {status}");
    }

    let count = size as usize / mem::size_of::<T>();
    let mut data: Vec<T> = Vec::with_capacity(count);

    let status = unsafe {
        AudioObjectGetPropertyData(
            object,
            addr,
            0,
            ptr::null(),
            &mut size,
            data.as_mut_ptr() as *mut _,
        )
    };
    if status != 0 {
        bail!("Failed to get property data: {status}");
    }

    unsafe { data.set_len(count) };
    Ok(data)
}

fn set_property_data<T>(
    object: AudioObjectID,
    addr: &AudioObjectPropertyAddress,
    value: &T,
) -> Result<()> {
    let status = unsafe {
        AudioObjectSetPropertyData(
            object,
            addr,
            0,
            ptr::null(),
            mem::size_of::<T>() as u32,
            value as *const _ as *const _,
        )
    };

    if status != 0 {
        bail!("Failed to set property data: {status}");
    }

    Ok(())
}
