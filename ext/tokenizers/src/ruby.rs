use std::ffi::c_void;
use std::ptr::null_mut;

use magnus::Ruby;
use rb_sys::rb_thread_call_without_gvl;

pub trait GvlExt {
    fn detach<T, F>(&self, func: F) -> T
    where
        F: Send + FnOnce() -> T,
        T: Send;
}

impl GvlExt for Ruby {
    fn detach<T, F>(&self, func: F) -> T
    where
        F: Send + FnOnce() -> T,
        T: Send,
    {
        let mut data = CallbackData {
            func: Some(func),
            result: None,
        };

        unsafe {
            rb_thread_call_without_gvl(
                Some(call_without_gvl::<F, T>),
                &mut data as *mut _ as *mut c_void,
                None,
                null_mut(),
            );
        }

        data.result.unwrap()
    }
}

struct CallbackData<F, T> {
    func: Option<F>,
    result: Option<T>,
}

extern "C" fn call_without_gvl<F, T>(data: *mut c_void) -> *mut c_void
where
    F: FnOnce() -> T,
{
    let data = unsafe { &mut *(data as *mut CallbackData<F, T>) };
    let func = data.func.take().unwrap();
    data.result = Some(func());
    null_mut()
}
