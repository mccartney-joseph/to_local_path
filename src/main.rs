#![windows_subsystem = "windows"]
use clipboard_win::{formats, set_clipboard};
use std::borrow::Borrow;
use std::env;
use std::path::Path;
use windows::{core::Result, Win32::NetworkManagement::WNet};

#[derive(PartialEq)]
enum Flags {
    Quote,
    EscapeAndQuote,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1]; // First arg is the path to *this* program
    let mut flag: Option<Flags> = None;
    if args.len() > 2 {
        flag = match args[2].as_ref() {
            "--mode=Quote" => Some(Flags::Quote),
            "--mode=EscapeAndQuote" => Some(Flags::EscapeAndQuote),
            _ => None,
        };
    }

    // Create a path object
    let path = Path::new::<str>(input.borrow());

    let _ = get_connection(&path, &flag);
}

fn get_connection(input: &Path, flag: &Option<Flags>) -> Result<()> {
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

    let universal_path: String;
    if result.is_err() {
        // we're a local path, no need to convert
        universal_path = input
            .to_str()
            .expect("I already know this is a good path")
            .to_string();
    } else {
        // `remote_name` should have been updated with the remote name via the `remote_name_ptr` pointer
        // `length` should have been updated with the length of `remote_name` if buffer is not large enough
        // Convert to string
        let remote_name = String::from_utf16(&remote_name_buffer[..length as usize])
            .expect("Our bytes should be valid utf16");

        // Replace the root with the connection name
        universal_path = input
            .to_str()
            .expect("I already know this is a good path")
            .replace(&root, &remote_name);
    }

    let mut modified_universal_path = universal_path.clone();
    if flag.is_some() {
        let flag_value = flag.as_ref().unwrap();
        if *flag_value == Flags::Quote {
            modified_universal_path = format!("\"{}\"", modified_universal_path);
        }
        if *flag_value == Flags::EscapeAndQuote {
            modified_universal_path = universal_path.replace("\\", "\\\\");
            modified_universal_path = format!("\"{}\"", modified_universal_path);
        }
    }

    // finally, place the new path on the clipboard
    set_clipboard(formats::Unicode, modified_universal_path.clone()).expect("To set clipboard");

    Ok(())
}
