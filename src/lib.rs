use std::sync::Arc;
use std::marker::PhantomData;


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct HeapFn<T>{
    f: Arc<Box<T>>
}

impl<T> HeapFn<T> {
    pub fn new(b: Box<T>) -> Self {
        HeapFn {
            f: Arc::new(b),
        }
    }
}

#[macro_export]
macro_rules! heap_fn {
    ( $f:ident ) => { $crate::HeapFn::new(Box::new($f)) };
    ( $f:expr ) => { $crate::HeapFn::new(Box::new($f)) };
}

#[cfg(test)]
mod tests {

    #[test]
    fn zero_arity_heap_fn() {
        let wrapped_prim = |i: u32| {println!("{}", i + 10)};
        let wrapped_vec = |v: Vec<&str>| { println!("{:?}", v) };

        (heap_fn!(||{println!("Neat!")}).f)();
        (heap_fn!(||{wrapped_prim(10)}).f)();
        (heap_fn!(||{wrapped_vec(vec!["nice", "cool"])}).f)();
    }

}
