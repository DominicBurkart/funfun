use std::sync::Arc;
use std::marker::PhantomData;


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct HeapFn<T, A, B> where T: Fn(A) -> B {
    arc: Arc<Box<T>>,
    a: PhantomData<A>,
    b: PhantomData<B>
}

impl<T, A, B> HeapFn<T, A, B>  where T: Fn(A) -> B {
    pub fn new(arc: Box<T>) -> Self {
        HeapFn {
            arc: Arc::new(arc),
            a: PhantomData,
            b: PhantomData
        }
    }

    pub fn f(&self) -> & Fn(A) -> B {
        self.arc
    }
}

//impl<T> Fn<()> for HeapFn<T> {
//    extern "rust-call" fn call(&self, _args: ()) {
//        println!("Call (Fn) for Foo");
//    }
//}

//impl FnMut<()> for HeapFn {
//    extern "rust-call" fn call_mut(&mut self, _args: ()) {
//        println!("Call (FnMut) for Foo");
//    }
//}
//
//impl FnOnce<()> for HeapFn {
//    type Output = ();
//
//    extern "rust-call" fn call_once(self, _args: ()) {
//        println!("Call (FnOnce) for Foo");
//    }
//}

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

        heap_fn!(||{println!("Neat!")}).f();
//        (heap_fn!(||{wrapped_prim(10)}).f)();
//        (heap_fn!(||{wrapped_vec(vec!["nice", "cool"])}).f)();
    }

}
