use windows::{
    core::{HRESULT, PWSTR},
    Win32::Foundation::E_FAIL,
    Win32::Media::MediaFoundation::*,
    Win32::System::Com::CoTaskMemFree,
};

fn get_capture_device_name(device: &Option<IMFActivate>) -> Result<String, HRESULT> {
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
            let result = name.to_string().map_err(|_| E_FAIL)?;

            CoTaskMemFree(Some(name.0 as *mut _));

            result
        };

        return Ok(res_name);
    }

    Err(E_FAIL)
}

pub fn get_capture_device_id_by_name(name: &str) -> Result<i32, HRESULT> {
    let mut p_config: Option<IMFAttributes> = None;
    let mut pp_devices: *mut Option<IMFActivate> = std::ptr::null_mut();
    let mut count = 0;

    unsafe {
        let _ = MFCreateAttributes(&mut p_config, 1)?;

        _ = p_config.as_ref().unwrap().SetGUID(
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE,
            &MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID,
        )?;

        MFEnumDeviceSources(p_config.as_ref().unwrap(), &mut pp_devices, &mut count)?;
    }

    let devices = unsafe {
        let slice = std::slice::from_raw_parts(pp_devices, count as usize);
        slice.to_vec()
    };

    let mut id = -1;
    let mut i = 0;
    for device in &devices {
        if let Ok(device_name) = get_capture_device_name(&device) {
            if device_name == name {
                id = i;
            }
        }
        i += 1;
    }
    /*
    for device in &devices {
        if let Some(device) = device {
            unsafe {
                device.Release();
            }
        }
    }
    */
    unsafe {
        CoTaskMemFree(Some(pp_devices as *mut _));
    }

    println!("{:?}", id);

    match id < 0 {
        true => Err(E_FAIL),
        false => Ok(id),
    }
}
