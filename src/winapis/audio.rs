use windows::{
    core::{Error, Result, HRESULT, HSTRING, PCSTR},
    Win32::Media::Multimedia::*,
};

pub fn open_audio(path: String) -> Result<MCI_OPEN_PARMSA> {
    let mut params = MCI_OPEN_PARMSA::default();
    params.lpstrDeviceType = PCSTR("mpegvideo\0".as_ptr());
    params.lpstrElementName = PCSTR(path.as_ptr());
    let result = unsafe {
        mciSendCommandA(
            0,
            MCI_OPEN,
            (MCI_OPEN_TYPE | MCI_OPEN_ELEMENT) as usize,
            (&mut params) as *mut _ as usize,
        )
    };
    if result != 0 {
        return Err(Error::new(
            HRESULT(0x80004005u32 as i32),
            HSTRING::from(result.to_string() + " : Failed to open audio."),
        ));
    }
    Ok(params)
}
pub fn play_audio(params: &MCI_OPEN_PARMSA) -> Result<()> {
    let play = MCI_PLAY_PARMS::default();
    let result = unsafe {
        mciSendCommandA(
            params.wDeviceID,
            MCI_PLAY,
            MCI_NOTIFY as usize,
            (&play) as *const _ as usize,
        )
    };
    if result != 0 {
        return Err(Error::new(
            HRESULT(0x80004005u32 as i32),
            HSTRING::from(result.to_string() + " : Something is wrong with playing audio."),
        ));
    }
    Ok(())
}
pub fn seek_start(params: &MCI_OPEN_PARMSA) -> Result<()> {
    let result =
        unsafe { mciSendCommandA(params.wDeviceID, MCI_SEEK, MCI_SEEK_TO_START as usize, 0) };
    if result != 0 {
        return Err(Error::new(
            HRESULT(0x80004005u32 as i32),
            HSTRING::from(result.to_string() + " : Something is wrong with seek to start."),
        ));
    }
    Ok(())
}
