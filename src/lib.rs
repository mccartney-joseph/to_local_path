mod clipboard {
    use winapi::um::winuser::{
        CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData, CF_TEXT,
    };
    use winapi::ctypes::c_void;
    use winapi::shared::minwindef::{HGLOBAL, UINT};
    use winapi::shared::ntdef::NULL;
    use std::ptr::{null_mut, null};
    use std::ffi::CString;
}
