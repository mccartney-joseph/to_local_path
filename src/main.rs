#[allow(unused_extern_crates)]
extern crate libc;
use std::mem;
use std::slice;
use windows::{core::Result, Win32::NetworkManagement::WNet};

fn main() {
    println!("Hello, world!");
    get_connection();
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_connection() -> Result<()> {
    let input = "S:";
    let mut v: Vec<u16> = input.encode_utf16().collect();
    v.push(0);
    let input_ptr = windows::core::PWSTR(v.as_mut_ptr());

    let mut cch_buffer = 80_u32;
    let mut remote_name = Vec::<u16>::with_capacity(cch_buffer as usize);
    let remote_name_ptr = windows::core::PWSTR(remote_name.as_mut_ptr());

    unsafe {
        let length: *mut u32 = libc::malloc(mem::size_of::<u32>()) as *mut u32;
        let result = WNet::WNetGetConnectionW(input_ptr, remote_name_ptr, length);
        let result = WNet::WNetGetConnectionW(input_ptr, remote_name_ptr, length);
        if result.is_ok() {
            println!("OK");
        } else {
            println!("Not OK");
            let hresult = result.to_hresult();
            println!("{:?}", hresult);
        }
        // println!("{:?}", *length);
        let length_slice = unsafe { slice::from_raw_parts(length, 1) };
        println!("{}", length_slice[0]);
        let length: usize = length_slice[0].try_into().unwrap();
        print_type_of(&length);

        let slice = unsafe { slice::from_raw_parts(remote_name.as_ptr(), length) };
        let s = String::from_utf16(&slice).expect("Our bytes should be valid utf16");
        // println!("{:?}", slice[0]);
        println!("{}", s);
    }

    // let s = String::from_utf16(&remote_name).expect("Our bytes should be valid utf16");
    // println!("{}", s);

    // let ss = String::from_utf16_lossy(&remote_name);
    // println!("{}", ss);

    Ok(())
}
