use sgx_types::{sgx_status_t, sgx_sealed_data_t};
use sgx_types::marker::ContiguousMemory;
use sgx_tseal::{SgxSealedData};
use sgx_rand::{Rng, StdRng};
use sgx_tcrypto::{rsgx_sha256_slice};

use std::vec::Vec;
use std::string::{String, ToString};
use std::borrow::ToOwned;
use std::sync::SgxMutex;

const SRD_MAX_PER_TURN: i32 = 64;
static mut SRD_TASK_NUM: i32 = 0;
static mut SRD_RANDOM_BUFFER: [u8; 1048576] = [0_u8; 1048576];
static mut SRD_RANDOM_BUFFER_RANDOM: bool = false;
static mut SRD_HASHS: Vec<String> = Vec::new();

#[repr(C)]
pub enum FileType {
    SrdType,
}

lazy_static! {
    static ref SRD_HASHS_MUTEX: SgxMutex<()> = SgxMutex::new(());
    static ref SRD_TASK_NUM_MUTEX: SgxMutex<()> = SgxMutex::new(());
}

extern "C" {
    pub fn ocall_store_file(retval: *mut sgx_status_t, path: *const u8, 
                              path_sz: usize, data: *const u8, data_sz: usize, ftype: FileType) -> sgx_status_t;

    pub fn ocall_create_dir(retval: *mut sgx_status_t, path: 
                            *const u8, path_sz: usize, ftype: FileType) -> sgx_status_t;

    pub fn ocall_delete_file(retval: *mut sgx_status_t, path: 
                            *const u8, path_sz: usize, ftype: FileType) -> sgx_status_t;

    pub fn ocall_rename_file(retval: *mut sgx_status_t, src: *const u8, 
                             src_sz: usize, dst: *const u8, dst_sz: usize, ftype: FileType) -> sgx_status_t;

    pub fn ocall_srd_change(retval: *mut sgx_status_t, num: i32) -> sgx_status_t;
}

pub fn srd_change_task(num: i32) -> sgx_status_t {
    unsafe {
        SRD_TASK_NUM_MUTEX.lock().unwrap();
        SRD_TASK_NUM += num;
    }
    sgx_status_t::SGX_SUCCESS
}

pub fn srd_change() {
    unsafe {
        let mutex = SRD_TASK_NUM_MUTEX.lock().unwrap();
        let mut sgx_status = sgx_status_t::SGX_SUCCESS;
        let srd_change_num;
        if SRD_TASK_NUM > SRD_MAX_PER_TURN {
            srd_change_num = SRD_MAX_PER_TURN;
        } else {
            srd_change_num = SRD_TASK_NUM
        }
        SRD_TASK_NUM -= srd_change_num;
        drop(mutex);

        if srd_change_num != 0 {
            ocall_srd_change(&mut sgx_status, srd_change_num);
        }
    }
}

pub fn srd_increase() -> sgx_status_t {
    let aad: [u8; 0] = [0_u8; 0];
    let mut rand = match StdRng::new() {
        Ok(rng) => rng,
        Err(_) => { return sgx_status_t::SGX_ERROR_UNEXPECTED; },
    };
    unsafe {
        if !SRD_RANDOM_BUFFER_RANDOM {
            rand.fill_bytes(&mut SRD_RANDOM_BUFFER);
            SRD_RANDOM_BUFFER_RANDOM = true;
        }
    }

    let sealed_buffer_sz = unsafe { SgxSealedData::<[u8]>::calc_raw_sealed_data_size(0, SRD_RANDOM_BUFFER.len() as u32) as usize };
    let mut sealed_buffer = vec![0_u8; sealed_buffer_sz];

    let mut sgx_status = sgx_status_t::SGX_SUCCESS;

    // Create srd directory
    let mut dir_slice = [0_u8; 16];
    rand.fill_bytes(&mut dir_slice);
    let dir_path = &to_hex_string(dir_slice.to_vec());
    unsafe { ocall_create_dir(&mut sgx_status, dir_path.as_ptr(), dir_path.len(), FileType::SrdType) };

    // Generate and store m file and hashs
    let m_num = 1024;
    let mut i = 0;
    let mut m_hashs = vec![0_u8; m_num*32];
    while i < m_num {
        let result = unsafe { SgxSealedData::<[u8]>::seal_data(&aad, &SRD_RANDOM_BUFFER) };
        let sealed_data = match result {
            Ok(x) => x,
            Err(ret) => { return ret; },
        };
        let sealed_buffer_ptr = sealed_buffer.as_mut_ptr();
        let opt = to_sealed_log_for_slice(&sealed_data, sealed_buffer_ptr, sealed_buffer_sz as u32);
        if opt.is_none() {
            return sgx_status_t::SGX_ERROR_INVALID_PARAMETER;
        }
        unsafe { 
            let m_hash = match rsgx_sha256_slice(&sealed_buffer) {
                Ok(r)  => r,
                Err(x) => {
                    println!("[-] get m hash failed {}!", x.as_str());
                    return sgx_status_t::SGX_ERROR_UNEXPECTED;
                },
            };
            let mut file_path: String = dir_path.clone();
            file_path.push_str(&("/".to_owned() + &i.to_string() + "_"));
            file_path.push_str(&to_hex_string(m_hash.to_vec()));
            let (m_path, m_path_len, _) = file_path.into_raw_parts();
            ocall_store_file(&mut sgx_status, m_path, m_path_len, sealed_buffer_ptr, sealed_buffer_sz, FileType::SrdType); 
            if sgx_status != sgx_status_t::SGX_SUCCESS {
                return sgx_status;
            }

            m_hashs.append(&mut m_hash.to_vec());
        }
        i += 1;
    }

    // ----- Calculate m_hashs ----- //
    unsafe {
        let m_hashs_sha256 = match rsgx_sha256_slice(&m_hashs) {
            Ok(r)  => r,
            Err(x) => {
                println!("[-] Get m_hashs failed {}!", x.as_str());
                return sgx_status_t::SGX_ERROR_UNEXPECTED;
            },
        };

        // Store m_hashs to file
        let mut m_hashs_path: String = dir_path.clone();
        m_hashs_path.push_str("/m-hashs");
        let (hashs_path, hashs_path_len, _) = m_hashs_path.into_raw_parts();
        ocall_store_file(&mut sgx_status, hashs_path, hashs_path_len, m_hashs.as_ptr(), m_hashs.len(), FileType::SrdType); 
        if sgx_status != sgx_status_t::SGX_SUCCESS {
            return sgx_status;
        }

        // Rename srd directory
        let g_hash = to_hex_string(m_hashs_sha256.to_vec());
        let new_dir = &(g_hash.clone());
        ocall_rename_file(&mut sgx_status, dir_path.as_ptr(), dir_path.len(), new_dir.as_ptr(), new_dir.len(), FileType::SrdType);

        SRD_HASHS_MUTEX.lock().unwrap();
        SRD_HASHS.push(g_hash.clone());
        println!("[+] Seal random data -> {} , {}G success", g_hash, SRD_HASHS.len());
    }

    sgx_status_t::SGX_SUCCESS
}

pub fn srd_decrease(num: usize) -> sgx_status_t {
    unsafe {
        SRD_HASHS_MUTEX.lock().unwrap();
        let srd_hashs_len = SRD_HASHS.len();
        let mut nsize = 0;
        if num < srd_hashs_len {
            nsize = srd_hashs_len - num;
        }
        for i in (nsize..srd_hashs_len).rev() {
            let dir_path = &SRD_HASHS[i];
            let mut sgx_status = sgx_status_t::SGX_SUCCESS;
            let retval = ocall_delete_file(&mut sgx_status, dir_path.as_ptr(), dir_path.len(), FileType::SrdType);
            if retval == sgx_status_t::SGX_SUCCESS && sgx_status == sgx_status_t::SGX_SUCCESS {
                SRD_HASHS.remove(i);
            }
        }
    }
    sgx_status_t::SGX_SUCCESS
}

fn to_sealed_log_for_slice<T: Copy + ContiguousMemory>(sealed_data: &SgxSealedData<[T]>, sealed_log: * mut u8, sealed_log_size: u32) -> Option<* mut sgx_sealed_data_t> {
    unsafe {
        sealed_data.to_raw_sealed_data_t(sealed_log as * mut sgx_sealed_data_t, sealed_log_size)
    }
}

pub fn to_hex_string(bytes: Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02x}", b))
                               .collect();
    strs.join("")
}
