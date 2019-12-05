use std::os::raw::c_void;

pub type Callback =
    unsafe extern "C" fn(user_data: *mut c_void, arg: i32) -> i32;

/// An arbitrary function which executes the provided function, using a pointer
/// to some `user_data` for context.
pub unsafe extern "C" fn execute_a_closure(
    arg: i32,
    cb: Callback,
    user_data: *mut c_void,
) -> i32 {
    cb(user_data, arg)
}

/// Get a function pointer which can be used as a [`Callback`] that accepts a
/// pointer to the closure as its `user_data`.
pub fn raw_callback<F>(_closure: &F) -> Callback
where
    F: FnMut(i32) -> i32,
{
    unsafe extern "C" fn wrapper<P>(user_data: *mut c_void, arg: i32) -> i32
    where
        P: FnMut(i32) -> i32,
    {
        let cb = &mut *(user_data as *mut P);

        cb(arg)
    }

    wrapper::<F>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut calls = 0;
        let mut closure = |arg: i32| {
            calls += 1;
            arg
        };

        unsafe {
            let func = raw_callback(&closure);

            let got = execute_a_closure(
                42,
                func,
                &mut closure as *mut _ as *mut c_void,
            );

            assert_eq!(got, 42);
            assert_eq!(calls, 1);
        }
    }
}
