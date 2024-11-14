#[cfg(any(target_os = "solaris", target_os = "illumos"))]
use libc::___errno as errno_location;
#[cfg(any(target_os = "netbsd", target_os = "openbsd", target_os = "android"))]
use libc::__errno as errno_location;
#[cfg(any(target_os = "linux", target_os = "emscripten", target_os = "redox",))]
use libc::__errno_location as errno_location;
#[cfg(any(target_os = "macos", target_os = "freebsd"))]
use libc::__error as errno_location;
#[cfg(target_os = "haiku")]
use libc::_errnop as errno_location;

type Errno = libc::c_int;
pub(crate) fn get_last_error() -> Errno {
    // SAFETY:
    //  The only way to safely access the referenced errno is to use either
    //  `get_last_error` or `set_last_error`, ensuring that no one currently
    //  holds a mutable reference to the underlying value.
    unsafe { *errno_location() }
}

pub(crate) fn set_last_error(code: Errno) {
    // SAFETY:
    //  The only way to safely access the referenced errno is to use either
    //  `set_last_error` or `get_last_error`, ensuring that no one currently
    //  holds any reference to the underlying value.
    unsafe { *errno_location() = code };
}
