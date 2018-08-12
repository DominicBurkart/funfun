use std::sync::Arc;

pub type HeapFn<T> = Arc<Box<T>>;

#[macro_export]
macro_rules! heap_fn {
    ( $f:ident ) => { $crate::Arc::new(Box::new($f)) };
    ( $f:expr ) => { $crate::Arc::new(Box::new($f)) };
}


#[cfg(test)]
mod tests {
    use HeapFn;

    #[test]
    fn zero_arity_heap_fn() {
        let wrapped_prim = |i: u32| { println!("{}", i + 10) };
        let wrapped_vec = |v: Vec<&str>| { println!("{:?}", v) };

        let _t = heap_fn!(||{println!("Neat!")});
        let _c = heap_fn!(||{println!("Neat!")});

        heap_fn!(||{println!("Neat!")})();
        heap_fn!(||{wrapped_prim(10)})();
        heap_fn!(||{wrapped_vec(vec!["nice", "cool"])})();
    }

    #[test]
    fn one_arity_heap_fn() {
        heap_fn!(|a: &str|{println!("{}", a)})("Awesome!");
        heap_fn!(|i: u32| {println!("{}", i + 10)})(10);
        heap_fn!(|v: Vec<&str>| { println!("{:?}", v) })(vec!["nice", "cool"]);
    }

    #[test]
    fn pass_let_exp_heap_fn() {
        let wrapped_prim = |i: u32| { println!("{}", i + 10) };
        let wrapped_vec = |v: Vec<&str>| { println!("{:?}", v) };

        heap_fn!(wrapped_prim)(10);
        heap_fn!(wrapped_vec)(vec!["nice", "cool"])
    }

    #[test]
    fn pass_function_heap_fn() {
        fn func(s: &str) { println!("{}", s)}

        heap_fn!(func)("Wow!");
    }

    #[test]
    fn in_struct_heap_fn() {
        type T = HeapFn<Fn(&str) -> String>;

        struct F {
            c: T
        }

        let c: T = heap_fn!(|s: &str| -> String {s.to_string()});

        let mut f = F { c };

        f.c = heap_fn!(
            |d: &str| -> String {"reassign once".to_string()}
        );

        f.c = heap_fn!(
            |d: &str| -> String {"and again".to_string()}
        )
    }
}
