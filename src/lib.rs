#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/icu.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CStr;
    use std::os::raw::c_char;
    #[test]
    fn uloc_linked() {
        let loc = unsafe { CStr::from_bytes_with_nul_unchecked(b"en-IN\0") };
        let mut buf = [0 as u8; 12];
        let mut err = super::UErrorCode_U_ZERO_ERROR;
        unsafe {
            let len = super::uloc_getLanguage_57(loc.as_ptr(), &mut buf as *mut _ as *mut c_char, buf.len() as i32, &mut err as *mut _) as usize;
            assert_eq!("en".as_bytes(), &buf[0..len]);
        }
        unsafe {
            let len = super::uloc_getScript_57(loc.as_ptr(), &mut buf as *mut _ as *mut c_char, buf.len() as i32, &mut err as *mut _) as usize;
            assert_eq!("".as_bytes(), &buf[0..len]);
        }
        unsafe {
            let len = super::uloc_getCountry_57(loc.as_ptr(), &mut buf as *mut _ as *mut c_char, buf.len() as i32, &mut err as *mut _) as usize;
            assert_eq!("IN".as_bytes(), &buf[0..len]);
        }
    }
}
