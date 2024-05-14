#[allow(unused_extern_crates)]
use clipboard_win::{formats, get_clipboard, set_clipboard};
use regex::Regex;
use std::borrow::Borrow;
use std::env;
use std::io::Error;
use std::path::Path;
use std::slice;
use windows::{core::Result, Win32::NetworkManagement::WNet};

fn main() {
    // Debugging only :P
    // set_clipboard(formats::Unicode, "\"S:\\SMA Tasks\\boeing_ti_testing_tie_rods\\1. Test Plan\\MAA7-72472-5 Rev New_2024-05-10_INITIAL_DRAFT.docx\"").expect("To set clipboard");

    // Get the input from the user
    let input: String = get_clipboard(formats::Unicode).expect("To get clipboard");
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // Strip quotes at the start and end (copy as path appends these)
    let re = Regex::new("(^\"|\"$)").unwrap();
    let input = re.replace_all(&input, "");

    // Create a path object
    let path = Path::new::<str>(input.borrow());

    // Check if the path is valid
    // todo: move these to a function
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

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn get_connection(input: &Path) -> Result<()> {
    let root: &str = input
        .components()
        .map(|comp| comp.as_os_str().to_str().unwrap())
        .next()
        .unwrap();

    // [in] `lplocalname`: Pointer to a constant null-terminated string that specifies the name of the
    // local device to get the network name for
    let mut v = root.encode_utf16().chain([0u16]).collect::<Vec<u16>>();
    let root_ptr = windows::core::PWSTR(v.as_mut_ptr());

    // [out] `lpremotename`: Pointer to a null-terminated string that receives the remote name used
    // to make the connection
    static BUFFER_SIZE: u32 = 255;
    let mut remote_name = vec![0_u16; BUFFER_SIZE as usize];
    let remote_name_ptr = windows::core::PWSTR(remote_name.as_mut_ptr());

    // [in, out] `lpnlength`: Pointer to a variable that specivies the size of the buffer pointed
    // to by the `lpremotename` parameter, in characters.  If the function fails because the buffer
    // is not large enough, this parameter returns the required buffer size.
    let mut length: u32 = 1;

    // I'm lazy, I could either find the actual length, or just call this twice and be given
    // the length.
    let result = unsafe { WNet::WNetGetConnectionW(root_ptr, remote_name_ptr, &mut length) };
    let result = unsafe { WNet::WNetGetConnectionW(root_ptr, remote_name_ptr, &mut length) };

    if result.is_err() {
        println!("Not OK:\n{:?}", result.to_hresult().message());
        return Err(result.into());
    }

    // `remote_name` should have been updated with the remote name via the `lpremotename` pointer
    // `length` should have been updated with the length of `remote_name` if buffer is not large enough
    length -= 1; // Avoid the null ternmination
    let output = String::from_utf16(&remote_name[..length as usize])
        .expect("Our bytes should be valid utf16");

    let universal_path = &input
        .to_str()
        .expect("I already know this is a good path")
        .replace(&root, &output);

    set_clipboard(formats::Unicode, universal_path.clone()).expect("To set clipboard");

    // let asdf = universal_path
    //     .encode_utf16()
    //     .chain([0u16])
    //     .collect::<Vec<u16>>();
    // let asdf = String::from_utf16(&asdf);
    // println!("{:?}", asdf);

    // let blah = asdf.iter().map(|x| char::from_digit(x).unwrap());

    println!("{}", universal_path);

    Ok(())
}
