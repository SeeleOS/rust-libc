use crate::prelude::*;

pub type sigset_t = c_ulonglong;

unsafe extern "C" {
    pub fn pthread_atfork(
        prepare: Option<unsafe extern "C" fn()>,
        parent: Option<unsafe extern "C" fn()>,
        child: Option<unsafe extern "C" fn()>,
    ) -> c_int;

    pub fn pthread_sigmask(
        how: c_int,
        set: *const crate::sigset_t,
        oldset: *mut crate::sigset_t,
    ) -> c_int;
}
