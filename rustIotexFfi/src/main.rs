#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("../DeviceConnect_Core.rs");

use std::os::raw::{c_int, c_uint};

/* fn main() {
    println!("Hello, world!");
    let mut time: tm = tm {
        tm_sec: 1,
        tm_min: 1,
        tm_hour: 1,
        tm_mday: 1,
        tm_mon: 1,
        tm_year: 1,
        tm_wday: 1,
        tm_yday: 1,
        tm_isdst: -1,
        tm_gmtoff: 1,
        tm_zone: std::ptr::null_mut(),
    };
    unsafe {
        let time_mut: *mut tm = &mut time;
        let time_long: c_long;

        let char_ptr = asctime(time_mut);
        let c_str = CStr::from_ptr(char_ptr);
        let date_str = c_str.to_str().unwrap();
        println!("date_str: {:?}", date_str);

        time_long = mktime(time_mut);
        println!("time_long: {:?}", time_long);
    }
} */

fn main() {
    let jwk: JWK;
    let mut myKeyId: c_uint = 2 as c_uint;
    let myKeyIdPtr: *mut c_uint = &mut myKeyId;
    unsafe {
        let jwk_ptr = iotex_jwk_generate(
            JWKType_JWKTYPE_EC,
            JWKSupportKeyAlg_JWK_SUPPORT_KEY_ALG_P256,
            0x01 as c_int,
            0x00001000 | 0x00002000 | 0x00000001 as c_uint,
            0x06000600 | (0x02000009 & 0x000000ff) as c_uint,
            myKeyIdPtr,
        );
        println!("jwk: {:?}", jwk_ptr);
    }
}
