//! Detect if the current process is running as a translated binary under [`Rosetta`].
//!
//! [`Rosetta`]: https://developer.apple.com/documentation/apple-silicon/about-the-rosetta-translation-environment

#[cfg(not(any(doc, all(target_os = "macos", target_arch = "x86_64"))))]
pub fn is_proc_translated() -> bool {
    false
}

/// Detects Rosetta emulation on macOS using [`sysctl.proc_translated`].
///
/// [`sysctl.proc_translated`]: https://developer.apple.com/documentation/apple-silicon/about-the-rosetta-translation-environment#Determine-Whether-Your-App-Is-Running-as-a-Translated-Binary
#[cfg(any(doc, all(target_os = "macos", target_arch = "x86_64")))]
pub fn is_proc_translated() -> bool {
    use std::ffi::CStr;

    let mut value: libc::c_int = 0;
    let mut value_size = std::mem::size_of_val(&value);

    let sysctl_name = CStr::from_bytes_with_nul(b"sysctl.proc_translated\0").unwrap();

    let err = unsafe {
        libc::sysctlbyname(
            sysctl_name.as_ptr(),
            &mut value as *mut _ as *mut _,
            &mut value_size,
            std::ptr::null_mut(),
            0,
        )
    };

    err == 0 && value != 0
}
