extern crate sgx_types;
use sgx_types::*;

use super::ENCLAVE_EID;
use threadpool::ThreadPool;

extern {
    fn ecall_srd_increase(eid: sgx_enclave_id_t, retval: *mut sgx_status_t) -> sgx_status_t;
    fn ecall_srd_decrease(eid: sgx_enclave_id_t, retval: *mut sgx_status_t, num: u32) -> sgx_status_t;
}

static mut SRD_SUCCESS_NUM: i32 = 0;

pub fn srd_change(num: i32) -> sgx_status_t {
    if num == 0 {
        return sgx_status_t::SGX_SUCCESS;
    }

    unsafe {
        let thread_num = 8;
        let pool = ThreadPool::new(thread_num);
        println!("[+] Starting to srd {}GB space...", num);
        if num > 0 {
            SRD_SUCCESS_NUM = 0;
            for _ in 0..num {
                pool.execute(|| {
                    let mut sgx_status = sgx_status_t::SGX_SUCCESS;
                    let retval = ecall_srd_increase(ENCLAVE_EID, &mut sgx_status);
                    match retval {
                        sgx_status_t::SGX_SUCCESS => {
                            match sgx_status {
                                sgx_status_t::SGX_SUCCESS => SRD_SUCCESS_NUM += 1,
                                _ => println!("[-] Srd increase failed {}", sgx_status.as_str()),
                            }
                        },
                        _ => println!("[-] Invoke SGX API failed {}", retval.as_str()),
                    }
                });
            }
            pool.join();
            if num == SRD_SUCCESS_NUM {
                println!("[+] Increase {}GB srd space successfully", num);
            } else {
                println!("[+] Increase {}GB srd space finish, success:{}GB, failed:{}GB", num, SRD_SUCCESS_NUM, num - SRD_SUCCESS_NUM);
            }
        } else {
            let mut sgx_status = sgx_status_t::SGX_SUCCESS;
            ecall_srd_decrease(ENCLAVE_EID, &mut sgx_status, (-num).abs() as u32);
            println!("[+] Decrease {}GB srd space successfully", -num);
        }
        sgx_status_t::SGX_SUCCESS
    }
}
