//! Provides operations with capturing devices on Windows using Microsoft Media Foundation API.

use log::debug;
use windows::{
    core::*, Win32::Foundation::E_FAIL, Win32::Media::MediaFoundation::*,
    Win32::System::Com::CoTaskMemFree,
};

/// Gets names of capturing devices available in the system.
/// 
/// # Errors
/// 
/// Returns the corresponding Windows error in case of failure.
/// 
/// # Safety
/// Requires calling unsafe methods of the `windows` crate.
#[cfg(target_os = "windows")]
pub fn enumerate_capture_devices() -> Result<Vec<String>> {
    let mut p_config: Option<IMFAttributes> = None;
    let mut pp_devices: *mut Option<IMFActivate> = std::ptr::null_mut();
    let mut count = 0;

    unsafe {
        MFCreateAttributes(&mut p_config, 1)?;

        p_config.as_ref().unwrap().SetGUID(
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
        )?;

        MFEnumDeviceSources(p_config.as_ref().unwrap(), &mut pp_devices, &mut count)?;
    }

    let devices = unsafe { Array::<IMFActivate>::from_raw_parts(pp_devices as _, count) };

    let mut dev_names: Vec<String> = Vec::new();
    for device in devices.as_slice() {
        dev_names.push(get_capture_device_name(device)?);
    }

    debug!("available capture devices: {:#?}", dev_names);

    Ok(dev_names)
}

#[cfg(target_os = "windows")]
fn get_capture_device_name(device: &Option<IMFActivate>) -> Result<String> {
    if let Some(device) = device {
        let mut name = PWSTR::null();
        let mut name_len = 0;

        unsafe {
            device.GetAllocatedString(
                &MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME,
                &mut name,
                &mut name_len,
            )
        }?;

        let res_name = unsafe {
            let result = name.to_string()?;
            CoTaskMemFree(Some(name.0 as *mut _));
            result
        };

        return Ok(res_name);
    }
    Err(Error::new(E_FAIL, "cannot get capture device name"))
}
