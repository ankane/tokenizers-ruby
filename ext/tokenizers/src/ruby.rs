use std::borrow::Cow;
use std::ffi::c_void;
use std::ptr::null_mut;

use magnus::{Error, Ruby};
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

macro_rules! create_ruby_exception {
    ($type:ident, $cls:ident) => {
        pub struct $type {}

        impl $type {
            pub fn new_err<T>(message: T) -> Error
            where
                T: Into<Cow<'static, str>>,
            {
                let cls = Ruby::get().unwrap().$cls();
                Error::new(cls, message)
            }
        }
    };
}

create_ruby_exception!(RbException, exception_type_error);
