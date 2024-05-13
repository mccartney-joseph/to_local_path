#[allow(unused_extern_crates)]
// extern crate libc;
use clipboard_win::{formats, get_clipboard, set_clipboard};
// use std::mem;
use std::slice;
use windows::{core::Result, Win32::NetworkManagement::WNet};

fn main() {
    let input: String = get_clipboard(formats::Unicode).expect("To get clipboard");
    println!("{}", input);
    let _ = get_connection("W:");
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn get_connection(input: &str) -> Result<()> {
    // [in] `lplocalname`: Pointer to a constant null-terminated string that specifies the name of the
    // local device to get the network name for
    let mut v = input.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
    let input_ptr = windows::core::PWSTR(v.as_mut_ptr());
    println!("{}", String::from_utf16(&v).expect("blarg"));

    // [out] `lpremotename`: Pointer to a null-terminated string that receives the remote name used
    // to make the connection
    static BUFFER_SIZE: u32 = 256;
    let mut remote_name = vec![0_u16; BUFFER_SIZE as usize];
    let remote_name_ptr = windows::core::PWSTR(remote_name.as_mut_ptr());

    // [in, out] `lpnlength`: Pointer to a variable that specivies the size of the buffer pointed
    // to by the `lpremotename` parameter, in characters.  If the function fails because the buffer
    // is not large enough, this parameter returns the required buffer size.
    let mut length: u32 = BUFFER_SIZE;

    let result = unsafe { WNet::WNetGetConnectionW(input_ptr, remote_name_ptr, &mut length) };

    if result.is_err() {
        println!("Not OK:\n{:?}", result.to_hresult().message());
        // return Ok(());
        return Err(result.into());
    }

    let buffer = unsafe { slice::from_raw_parts(remote_name_ptr.0, length as usize - 1) };
    let output = String::from_utf16_lossy(buffer);

    // `remote_name` should have been updated with the remote name via the `lpremotename` pointer
    // `length` should have been updated with the length of `remote_name`
    // let output = String::from_utf16(&remote_name[..length as usize])
    //     .expect("Our bytes should be valid utf16");
    println!("Length: {}, Path: {}", length, output);

    set_clipboard(formats::Unicode, output).expect("To set clipboard");

    Ok(())
}
