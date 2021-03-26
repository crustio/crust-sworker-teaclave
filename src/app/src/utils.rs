use std::io::prelude::*;
use std::fs::File;

use super::FileType;
use super::SRD_PATH;

pub fn rename_file(src: *const u8, src_sz: usize, dst: *const u8, dst_sz: usize) -> std::io::Result<()> {
    unsafe {
        let mut src_vec = vec![0_u8; src_sz];
        let mut dst_vec = vec![0_u8; dst_sz];
        std::ptr::copy(src, src_vec.as_mut_ptr(), src_sz);
        std::ptr::copy(dst, dst_vec.as_mut_ptr(), dst_sz);
        let src_path = String::from_utf8(src_vec).unwrap();
        let dst_path = String::from_utf8(dst_vec).unwrap();
        std::fs::rename(src_path, dst_path)?;
    }
    Ok(())
}

pub fn create_dir(path: *const u8, path_sz: usize) -> std::io::Result<()> {
    unsafe {
        let mut path_vec = vec![0_u8; path_sz];
        std::ptr::copy(path, path_vec.as_mut_ptr(), path_sz);
        let path_str = String::from_utf8(path_vec).unwrap();
        std::fs::create_dir_all(path_str)?;
    }
    Ok(())
}

pub fn delete_file(path: *const u8, path_sz: usize) -> std::io::Result<()> {
    unsafe {
        let mut path_vec = vec![0_u8; path_sz];
        std::ptr::copy(path, path_vec.as_mut_ptr(), path_sz);
        let path_str = String::from_utf8(path_vec).unwrap();
        let attr = std::fs::metadata(path_str.clone())?;
        if attr.is_dir() {
            std::fs::remove_dir_all(path_str)?;
        } else if attr.is_file() {
            std::fs::remove_file(path_str)?;
        } else {
            println!("[-] Unknown {} type", path_str);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Delete file error"));
        }
    }
    Ok(())
}

pub fn store_file(hash: *const u8, hash_sz: usize, data: *const u8, data_sz: usize) -> std::io::Result<()> {
    unsafe {
        let mut hash_vec = vec![0_u8; hash_sz];
        std::ptr::copy(hash, hash_vec.as_mut_ptr(), hash_sz);
        let m_hash = String::from_utf8(hash_vec).unwrap();
        let data_slice = std::slice::from_raw_parts(data, data_sz as usize);
        let mut file = File::create(m_hash)?;
        file.write_all(data_slice)?;
        file.sync_all()?;
    }
    Ok(())
}

pub fn get_real_path(path: *const u8, path_sz: usize, ftype: FileType) -> String {
    let path_str;
    unsafe {
        let mut tmp_vec = vec![0_u8; path_sz];
        std::ptr::copy(path, tmp_vec.as_mut_ptr(), path_sz);
        match ftype {
            FileType::SrdType => {
                path_str = SRD_PATH.to_owned() + "/" + &String::from_utf8(tmp_vec).unwrap();
            },
            _ => {
                path_str = String::from_utf8(tmp_vec).unwrap();
            },
        }
    }
    path_str
}
