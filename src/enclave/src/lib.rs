#![crate_name = "sworkerenclave"]
#![crate_type = "staticlib"]
#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]
#![feature(vec_into_raw_parts)]

extern crate sgx_types;
extern crate sgx_tseal;
extern crate sgx_tcrypto;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_rand;
#[macro_use]
extern crate lazy_static;

use sgx_types::{sgx_status_t};

use std::thread;
use std::time::Duration;

mod srd;
use srd::*;

#[no_mangle]
pub extern "C" fn ecall_main_loop() {
    loop {
        // Do srd
        srd_change();

        // Sleep for a while
        thread::sleep(Duration::from_millis(10000));
    }
}

#[no_mangle]
pub extern "C" fn ecall_srd_increase() -> sgx_status_t {
    return srd_increase();
}

#[no_mangle]
pub extern "C" fn ecall_srd_decrease(num: u32) -> sgx_status_t {
    return srd_decrease(num as usize);
}

#[no_mangle]
pub extern "C" fn ecall_change_srd_task(num: i32) -> sgx_status_t {
    return srd_change_task(num);
}
