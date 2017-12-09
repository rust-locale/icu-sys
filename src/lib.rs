#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/icu.rs"));

#[cfg(test)]
mod tests {
    use std::default::Default;
    use std::ffi::CStr;
    use std::os::raw::c_char;

    unsafe fn bptr(buf: &mut [u8]) -> *mut c_char { buf as *mut _ as *mut c_char }

    unsafe fn cstr(bytes: &[u8]) -> &CStr { CStr::from_bytes_with_nul_unchecked(bytes) }
    use std::str::from_utf8_unchecked as ustr;

    #[test]
    fn uloc_linked() {
        let loc = unsafe { cstr(b"en-IN\0") };
        let mut buf = [0 as u8; 12];
        let mut err = super::U_ZERO_ERROR;
        unsafe {
            let len = super::uloc_getLanguage(loc.as_ptr(), bptr(&mut buf), buf.len() as i32, &mut err as *mut _) as usize;
            assert_eq!("en", ustr(&buf[0..len]));
        }
        unsafe {
            let len = super::uloc_getScript(loc.as_ptr(), bptr(&mut buf), buf.len() as i32, &mut err as *mut _) as usize;
            assert_eq!("", ustr(&buf[0..len]));
        }
        unsafe {
            let len = super::uloc_getCountry(loc.as_ptr(), bptr(&mut buf), buf.len() as i32, &mut err as *mut _) as usize;
            assert_eq!("IN", ustr(&buf[0..len]));
        }
    }

    #[test]
    fn ucal_linked() {
        unsafe {
            super::ucal_getNow();
        }
    }

    #[test]
    fn udat_linked() {
        unsafe {
            let minute = super::udat_toCalendarDateField(super::UDAT_MINUTE_FIELD);
            assert_eq!(super::UCAL_MINUTE, minute);
        }
    }

    #[test]
    fn umsg_linked() {
        let fmt = "trivial".encode_utf16().collect::<Vec<u16>>();
        let loc = unsafe { cstr(b"en-IN\0") };
        let mut pe = super::UParseError::default();
        let mut sc = super::U_ZERO_ERROR;
        unsafe {
            let msg = super::umsg_open(fmt.as_slice().as_ptr(), fmt.len() as i32, loc.as_ptr(), &mut pe as *mut _, &mut sc as *mut _);
            super::umsg_close(msg);
        }
    }

    #[test]
    fn unum_linked() {
        let loc = unsafe { cstr(b"en-IN\0") };
        let mut pe = super::UParseError::default();
        let mut sc = super::U_ZERO_ERROR;
        unsafe {
            let num = super::unum_open(super::UNUM_DECIMAL, ::std::ptr::null(), 0, loc.as_ptr(), &mut pe as *mut _, &mut sc as *mut _);
            super::unum_close(num);
        }
    }

    #[test]
    fn ucol_linked() {
        let loc = unsafe { cstr(b"en-IN\0") };
        let mut sc = super::U_ZERO_ERROR;
        unsafe {
            let col = super::ucol_open(loc.as_ptr(), &mut sc as *mut _);
            super::ucol_close(col);
        }
    }
}
