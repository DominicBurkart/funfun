use std::sync::Arc;


#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct HeapFn<T>{
    f: Arc<Box<T>>
}

impl<T> HeapFn<T> {
    pub fn new(b: Box<T>) -> Self {
        HeapFn {
            f: Arc::new(b),
        }
    }

    pub fn c(&self) -> Arc<Box<T>> {
        (self.f.clone())
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

        heap_fn!(||{println!("Neat!")}).c();
        heap_fn!(||{wrapped_prim(10)}).c();
        heap_fn!(||{wrapped_vec(vec!["nice", "cool"])}).c();
    }

    #[test]
    fn one_arity_heap_fn() {
        heap_fn!(|a: &str|{println!("{}", a)}).c()("Awesome!");
        heap_fn!(|i: u32| {println!("{}", i + 10)}).c()(10);
        heap_fn!(|v: Vec<&str>| { println!("{:?}", v) }).c()(vec!["nice", "cool"]);
    }

    #[test]
    fn pass_let_exp(){
        let wrapped_prim = |i: u32| {println!("{}", i + 10)};
        let wrapped_vec = |v: Vec<&str>| { println!("{:?}", v) };

        heap_fn!(wrapped_prim).c()(10);
        heap_fn!(wrapped_vec).c()(vec!["nice", "cool"])
    }
}
