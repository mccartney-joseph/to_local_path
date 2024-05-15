#![windows_subsystem = "windows"]
use clipboard_win::{formats, set_clipboard};
use std::borrow::Borrow;
use std::env;
use std::path::Path;
use windows::{core::Result, Win32::NetworkManagement::WNet};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1]; // First arg is the path to *this* program

    // Create a path object
    let path = Path::new::<str>(input.borrow());

    // Check if the path is valid
    if !path.exists() {
        println!("Path does not exist");
        return;
    }

    if !path.has_root() {
        println!("Path does not have a root");
        return;
    }

    let _ = get_connection(&path);
}

fn get_connection(input: &Path) -> Result<()> {
    let root: &str = input
        .components()
        .map(|comp| comp.as_os_str().to_str().unwrap())
        .next()
        .unwrap();

    // [in] `root_ptr`: Pointer to a null-terminated string that specifies the name of the
    // local device to get the network name for. e.g. "S:", "W:", "D:"
    let mut root_u16_buffer = root.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
    let root_ptr = windows::core::PWSTR(root_u16_buffer.as_mut_ptr());

    // [out] `remote_name_ptr`: Pointer to a null-terminated string that receives the remote name used
    // to make the connection
    static BUFFER_SIZE: u32 = 257;
    let mut remote_name_buffer = vec![0_u16; BUFFER_SIZE as usize];
    let remote_name_ptr = windows::core::PWSTR(remote_name_buffer.as_mut_ptr());

    // [in, out] `lpnlength`: Pointer to a variable that specifies the size of the buffer pointed
    // to by the `lpremotename` parameter, in characters.  If the function fails because the buffer
    // is not large enough, this parameter returns the required buffer size.
    let mut length: u32 = BUFFER_SIZE;

    let result = unsafe { WNet::WNetGetConnectionW(root_ptr, remote_name_ptr, &mut length) };
    length = remote_name_buffer
        .iter()
        .position(|x| *x == 0u16)
        .unwrap_or(0) as u32;

    if result.is_err() {
        // we're *probably* a local path, just put the contents back on the clipboard
        set_clipboard(
            formats::Unicode,
            &input.to_str().expect("already a good path/string"),
        )
        .expect("To set clipboard");
        return Ok(());
    }

    // `remote_name` should have been updated with the remote name via the `remote_name_ptr` pointer
    // `length` should have been updated with the length of `remote_name` if buffer is not large enough
    // Convert to string
    length -= 1; // Avoid the null ternmination
    let remote_name = String::from_utf16(&remote_name_buffer[..length as usize])
        .expect("Our bytes should be valid utf16");

    // Replace the root with the connection name
    let universal_path = &input
        .to_str()
        .expect("I already know this is a good path")
        .replace(&root, &remote_name);

    // finally, place the new path on the clipboard
    set_clipboard(formats::Unicode, universal_path.clone()).expect("To set clipboard");

    Ok(())
}
