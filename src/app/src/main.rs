extern crate sgx_types;
extern crate sgx_urts;
extern crate threadpool;
extern crate actix_web;
extern crate serde;
extern crate futures;

use sgx_types::*;
use sgx_urts::SgxEnclave;

mod webserver;
use webserver::*;

mod utils;
use utils::*;

mod srd;
use srd::*;

const SRD_PATH: &'static str = "/opt/crust/data/srd";
const ENCLAVE_FILE: &'static str = "/opt/crust/crust-sworker/etc/enclave.signed.so";
static mut ENCLAVE_EID: sgx_enclave_id_t = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum FileType {
    SrdType,
    SealType,
}

extern {
    fn ecall_main_loop(eid: sgx_enclave_id_t) -> sgx_status_t;
}

fn init_enclave() -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {secs_attr: sgx_attributes_t { flags:0, xfrm:0}, misc_select:0};
    SgxEnclave::create(ENCLAVE_FILE,
                       debug,
                       &mut launch_token,
                       &mut launch_token_updated,
                       &mut misc_attr)
}

fn main() {
    println!("[?] initializing enclave...");
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return ;
        },
    };

    // Start webservice
    match start_webserver() {
        Ok(()) => {},
        Err(x) => {
            println!("[-] Start webserver failed {}", x);
        },
    }

    unsafe {
        ENCLAVE_EID = enclave.geteid().clone(); 
        ecall_main_loop(enclave.geteid());
    };
    enclave.destroy();
}

#[no_mangle]
pub extern "C" fn ocall_store_file(path: *const u8, path_sz: usize, data: *const u8, data_sz: usize, ftype: FileType) -> sgx_status_t {
    let path_r = get_real_path(path, path_sz, ftype);
    match store_file(path_r.as_ptr(), path_r.len(), data, data_sz) {
        Ok(())  => {},
        _       => {
            println!("[-] Store m file failed!");
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        },
    };
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn ocall_create_dir(path: *const u8, path_sz: usize, ftype: FileType) -> sgx_status_t {
    let path_r = get_real_path(path, path_sz, ftype);
    match create_dir(path_r.as_ptr(), path_r.len()) {
        Ok(())  => {},
        Err(r)  => {
            println!("[-] Create directory failed {}", r);
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        },
    }
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn ocall_rename_file(src: *const u8, src_sz: usize, dst: *const u8, dst_sz: usize, ftype: FileType) -> sgx_status_t {
    let src_r = get_real_path(src, src_sz, ftype);
    let dst_r = get_real_path(dst, dst_sz, ftype);
    match rename_file(src_r.as_ptr(), src_r.len(), dst_r.as_ptr(), dst_r.len()) {
        Ok(())  => {},
        _       => {
            println!("[-] Rename directory failed!");
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        },
    }
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn ocall_delete_file(path: *const u8, path_sz: usize, ftype: FileType) -> sgx_status_t {
    let path_r = get_real_path(path, path_sz, ftype);
    match delete_file(path_r.as_ptr(), path_r.len()) {
        Ok(())  => {},
        Err(x)  => {
            println!("[-] Delete file failed {}", x);
            return sgx_status_t::SGX_ERROR_UNEXPECTED;
        },
    }
    sgx_status_t::SGX_SUCCESS
}

#[no_mangle]
pub extern "C" fn ocall_srd_change(num: i32) -> sgx_status_t {
    return srd_change(num);
}
