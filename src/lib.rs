use std::sync::Arc;
use std::fmt;

/// Boxes (heap-allocates) the given value and returns an Rc to the object.
#[macro_export]
macro_rules! rc {
    ( $f:ident ) => { Box::new($f) };
    ( $f:expr ) => { Box::new($f) };
}

/// Boxes (heap-allocates) the given value and returns an Arc to the object.
#[macro_export]
macro_rules! arc {
    ( $f:ident ) => { ::std::sync::Arc::new($f) };
    ( $f:expr ) => { ::std::sync::Arc::new($f) };
}

/// Boxes a closure and returns a reference.
#[macro_export]
macro_rules! box_fn {
    ( $f:ident ) => { rc!($f) };
    ( $f:expr ) => { rc!($f) };
}

/// Boxes a closure and returns an Arc reference. Slower than just a box, but can derive traits like
/// Clone.
#[macro_export]
macro_rules! arc_fn {
    ( $f:ident ) => { arc!($f) };
    ( $f:expr ) => { arc!($f) };
}

/// Starts a new thread and runs the passed closure with the passed arguments in it, returning the
/// new thread's hook.
#[macro_export]
macro_rules! spawn_fn {
    ( $f:expr ) => { ::std::thread::spawn($f)};
    ( $f:ident, $( $arg:ident ),* ) => { {let v = $f.clone(); ::std::thread::spawn(move || {v($($arg),*)})}};
    ( $f:expr, $( $arg:expr ),* ) => { ::std::thread::spawn(move || {$f($($arg),*)}) };
}

/// Box<T> aliased for clarity (and later trait implementation) when boxing structures that
/// implement Fn* traits.
pub type BoxFn<T> = Box<T>;

/// Arc<T> aliased for clarity (and later trait implementation) when boxing structures that
/// implement Fn* traits.
pub type ArcFn<T> = Arc<T>;

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    #[test]
    fn zero_arity_box_fn() {
        let wrapped_prim = |i: u32| { println!("{}", i + 10) };
        let wrapped_vec = |v: Vec<&str>| { println!("{:?}", v) };

        let _t = box_fn!(||{println!("Neat!")});
        let _c = box_fn!(||{println!("Neat!")});

        box_fn!(||{println!("Neat!")})();
        box_fn!(||{wrapped_prim(10)})();
        box_fn!(||{wrapped_vec(vec!["nice", "cool"])})();
    }

    #[test]
    fn one_arity_box_fn() {
        box_fn!(|a: &str| {println!("{}", a)})("Awesome!");
        box_fn!(|i: u32| {println!("{}", i + 10)})(10);
        box_fn!(|v: Vec<&str>| { println!("{:?}", v) })(vec!["nice", "cool"]);
    }

    #[test]
    fn pass_named_box_fn() {
        let wrapped_prim = |i: u32| { println!("{}", i + 10) };
        let wrapped_vec = |v: Vec<&str>| { println!("{:?}", v) };

        box_fn!(wrapped_prim)(10);
        box_fn!(wrapped_vec)(vec!["nice", "cool"])
    }

    #[test]
    fn pass_function_box_fn() {
        fn func(s: &str) { println!("{}", s)}

        box_fn!(func)("Wow!");
    }

    #[test]
    fn in_struct_box_fn() {
        type T = BoxFn<Fn(&str) -> String>;
        struct F {
            c: T
        }
        let c: T = box_fn!(|s: &str| -> String {s.to_string()});
        let mut f = F { c };
        f.c = box_fn!(
            |d: &str| -> String {"reassign once".to_string()}
        );
        f.c = box_fn!(
            |_: &str| {"and again".to_string()}
        );
    }


    #[test]
    fn in_struct_arc_fn() {
        type T = ArcFn<Fn(&str) -> String>;
        #[derive(Clone)]
        struct F {
            c: T
        }
        let c: T = arc_fn!(|s: &str| -> String {s.to_string()});
        let mut f = F { c };
        f.c = arc_fn!(
            |d: &str| -> String {"reassign once".to_string()}
        );
        f.c = arc_fn!(
            |_: &str| {"and again".to_string()}
        );
    }

    #[test]
    fn multithreading_box_fn() {
        let eg = box_fn!(|x: i32| -> i32 {x + 2});
        let mut v1 = Vec::new();
        for i in 0..1000 {
            let cl = eg.clone();
            v1.push(thread::spawn(move ||{cl(i)})); // move is necessary
            //v2.push(spawn_fn!(eg, i));
        }
        for res in v1.into_iter() {
            res.join();
        }
    }

    #[test]
    fn multithreading_spawn_fn() {
        let eg = box_fn!(|x: i32| -> i32 {x + 2});
        let also = box_fn!(|x: i32, y: i32| -> i32 {x + y});
        let mut v1 = Vec::new();
        for i1 in 0..10000 {
            let i2 = i1 + 10;
            v1.push(spawn_fn!(eg, i1));
            v1.push(spawn_fn!(also, i1, i2)); // woohoo multi-arity!
        }
        v1.push(spawn_fn!(||{println!("accepts closures to run in their own thread!"); 1}));
        for res in v1.into_iter() {
            res.join();
        }
    }
}
