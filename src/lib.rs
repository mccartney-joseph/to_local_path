// mod clipboard {
//     use std::ffi::CString;
//     use std::ptr::{null, null_mut};
//     use winapi::ctypes::c_void;
//     use winapi::shared::minwindef::{HGLOBAL, UINT};
//     use winapi::shared::ntdef::NULL;
//     use winapi::um::winuser::{
//         CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData, CF_TEXT,
//     };

//     fn open_clipboard() -> Result<(), &'static str> {
//         let result = unsafe { OpenClipboard(null_mut()) };
//         if result == 0 {
//             Err("Failed to open the clipboard")
//         } else {
//             Ok(())
//         }
//     }

//     fn close_clipboard() {
//         unsafe {
//             CloseClipboard();
//         }
//     }

//     fn get_clipboard_text() -> Result<String, &'static str> {
//         let clipboard_data = unsafe { GetClipboardData(CF_TEXT) as *mut u8 };
//         if clipboard_data.is_null() {
//             return Err("Failed to get clipboard data");
//         }

//         let mut text = String::new();
//         let mut index = 0;
//         loop {
//             let ch = unsafe { *clipboard_data.offset(index) } as char;
//             if ch == '\0' {
//                 break;
//             }
//             text.push(ch);
//             index += 1;
//         }

//         Ok(text)
//     }

//     fn set_clipboard_text(text: &str) -> Result<(), &'static str> {
//         let text_len = text.len() + 1;
//         let text_cstring = CString::new(text).unwrap();
//         let h_mem = unsafe { GlobalAlloc(GMEM_MOVEABLE, text_len) } as *mut c_void;

//         if h_mem.is_null() {
//             return Err("Failed to allocate memory for clipboard data");
//         }

//         let h_mem_text = unsafe { GlobalLock(h_mem) as *mut u8 };
//         for (i, ch) in text_cstring.as_bytes_with_nul().iter().enumerate() {
//             unsafe {
//                 *h_mem_text.offset(i as isize) = *ch;
//             }
//         }

//         unsafe {
//             GlobalUnlock(h_mem);
//             EmptyClipboard();
//             if SetClipboardData(CF_TEXT, h_mem as HGLOBAL) == NULL {
//                 GlobalFree(h_mem);
//                 return Err("Failed to set clipboard data");
//             }
//         }

//         Ok(())
//     }
// }
