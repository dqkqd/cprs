#[macro_export]
macro_rules! rfn {
    (|$self_arg:ident $(, $arg_name:ident : $arg_type:ty)* $(,)? | -> $ret_type:ty
        $body:block) => {{
            trait HideFn {
                fn call(&mut self, $($arg_name : $arg_type ,)*) -> $ret_type;
            }
            struct HideFnImpl<F: FnMut(&mut dyn HideFn, $($arg_type ,)*) -> $ret_type> (std::cell::UnsafeCell<F>);
            impl<F: FnMut(&mut dyn HideFn, $($arg_type ,)*) -> $ret_type> HideFn for HideFnImpl<F> {
                #[inline]
                fn call(&mut self, $($arg_name : $arg_type ,)*) -> $ret_type {
                    unsafe { (*self.0.get())(self, $($arg_name ,)*) }
                }
            }
            let mut inner = HideFnImpl(
                std::cell::UnsafeCell::new(
                #[inline]
                |$self_arg: &mut dyn HideFn, $($arg_name : $arg_type ,)*| -> $ret_type {
                    let mut $self_arg = |$($arg_name : $arg_type ),*| $self_arg.call($($arg_name ,)*);
                    {
                        $body
                    }
                })
            );
            #[inline]
            move |$($arg_name : $arg_type),*| -> $ret_type {
                inner.call($($arg_name),*)
            }
        }
    };
    (|$self_arg:ident $(, $arg_name:ident : $arg_type:ty)* $(,)? |
        $body:block) => {{
            trait HideFn {
                fn call(&mut self, $($arg_name : $arg_type ,)*);
            }
            struct HideFnImpl<F: FnMut(&mut dyn HideFn, $($arg_type ,)*)> (std::cell::UnsafeCell<F>);
            impl<F: FnMut(&mut dyn HideFn, $($arg_type ,)*)> HideFn for HideFnImpl<F> {
                #[inline]
                fn call(&mut self, $($arg_name : $arg_type ,)*) -> () {
                    unsafe { (*self.0.get())(self, $($arg_name ,)*) }
                }
            }
            let mut inner = HideFnImpl(
                std::cell::UnsafeCell::new(
                #[inline]
                |$self_arg: &mut dyn HideFn, $($arg_name : $arg_type ,)*| -> () {
                    let mut $self_arg = |$($arg_name : $arg_type ),*| $self_arg.call($($arg_name ,)*);
                    {
                        $body
                    }
                })
            );
            #[inline]
            move |$($arg_name : $arg_type),*| {
                inner.call($($arg_name),*)
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn fibonacci() {
        let mut fib = rfn!(|f, i: i32| -> i32 {
            if i <= 1 {
                i
            } else {
                f(i - 1) + f(i - 2)
            }
        });
        assert_eq!(fib(10), 55);
    }

    #[test]
    fn factorial() {
        let mut num_calls = 0;
        let mut fact = rfn!(|f, i: i32| -> i32 {
            num_calls += 1;
            if i <= 1 {
                1
            } else {
                i * f(i - 1)
            }
        });
        assert_eq!(fact(5), 120);
        assert_eq!(num_calls, 5);
    }

    #[test]
    fn double_closure() {
        let mut sum1 = rfn!(|f, i: i32| -> i32 {
            if i <= 1 {
                i
            } else {
                i + f(i - 1)
            }
        });
        let mut sum2 = rfn!(|f, i: i32| -> i32 {
            if i <= 1 {
                i
            } else {
                i + f(i - 1)
            }
        });
        assert_eq!(sum1(5), 15);
        assert_eq!(sum2(6), 21);
    }

    #[test]
    fn no_return_type() {
        let mut outer = 0;
        let mut modify_outer = rfn!(|f, i: i32| {
            if i > 0 {
                outer += 1;
                f(i - 1);
            }
        });
        modify_outer(10);
        assert_eq!(outer, 10);
    }
}
