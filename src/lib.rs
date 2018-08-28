#![recursion_limit="1024"]

#[macro_use(c)]
extern crate cute;
#[macro_use]
extern crate tt_call;

use std::sync::Arc;
//use std::fmt;

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
    ( $f:ident ) => { $crate::BoxFn::new($f) };
    ( $f:expr ) => { $crate::BoxFn::new($f) };
}

/// Boxes a closure and returns an Arc reference. Slower than just a box, but can derive traits like
/// Clone.
#[macro_export]
macro_rules! arc_fn {
    ( $f:ident ) => { $crate::ArcFn::new($f) };
    ( $f:expr ) => { $crate::ArcFn::new($f) };
}

/// Starts a new thread and runs the passed closure with the passed arguments in it, returning the
/// new thread's hook.
#[macro_export]
macro_rules! spawn_fn {
    ( $f:expr ) => { ::std::thread::spawn($f)};
    ( $f:ident, $( $arg:ident ),* ) => {{
        let v = $f.clone();
        ::std::thread::spawn(move || {v( $($arg),* )})
    }};
    ( $f:expr, $( $arg:expr ),* ) => { ::std::thread::spawn(move || {$f( $($arg),* )}) };
}

#[macro_export]
macro_rules! call {
    ( $f:ident, ($( $arg:expr ),*) ) => {$f($($arg),*)};
    ( $f:ident, $( $arg:expr ),* ) => {$f($($arg),*)};
}

macro_rules! tt_fn_wrapper {
    {
        $caller:tt
        input = [{ $f:ident, ($( $arg:expr ),*) }]
    } => {
        tt_return! {
            $caller
            is = [{ $f($( $arg ),* )}]
            // is = [{ call!($f, ($( $arg ),* ))}]
        }
    };
    {
        $caller:tt
        input = [{ $f:ident, $( $arg:expr ),*  }]
    } => {
        tt_return! {
            $caller
            is = [{ $f($($arg),*) }]
            // is = [{ call!($f,$( $arg ),* ) }]
        }
    }
}

macro_rules! dup_next {
    {
        $caller:tt
        input = [{ $arity:ident, $f:ident, $rep:expr }]
    } => {
        tt_return! {
            $caller
            is = [{ $f, $rep, $rep, $rep, $rep}]
        }
    }
}

#[macro_export]
macro_rules! vcall {
    ($function:expr, $args:expr) => {{
        let f = $function;
        let arity = $args.len();
        let mut it = $args.into_iter();

        tt_call! {
            macro = [{ dup_next }]
            input = [{ arity, f, it.next().unwrap() }]
            ~~> tt_fn_wrapper
        }
    }}
}



//fn vcall<A, B, C>(f: A, args:Vec<B>) -> C where A: Fn(B, B, B, B) -> C { // number of Bs == 4
//    let arity = args.len();
//    let mut it = args.into_iter();
//    unpack!(f, it.next().unwrap(), arity)
//}

//fn vcall<A, B, C>(f: A, args:Vec<B>) -> C where A: Fn(B, B, B, B) -> C {
//    let l = args.len();
//    let mut it = args.into_iter();
//    match l {
//        4 => f(it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap()),
//        _ => unimplemented!()
//    }
//}


/// Box<T> aliased for clarity (and later trait implementation) when boxing structures that
/// implement Fn* traits.
pub type BoxFn<T> = Box<T>;

/// Arc<T> aliased for clarity (and later trait implementation) when boxing structures that
/// implement Fn* traits.
pub type ArcFn<T> = Arc<T>;

#[cfg(test)]
mod tests {

    use std::collections::HashSet;
    use std::iter::FromIterator;
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
        fn func(s: &str) { println!("{}", s) }

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
        for i in 0..100 {
            let cl = eg.clone();
            v1.push(thread::spawn(move || { cl(i) }));
        }
        for res in v1.into_iter() {
            res.join().unwrap();
        }
    }

    #[test]
    fn multithreading_spawn_fn() {
        let eg = box_fn!(|x: i32| -> i32 {x + 2});
        let also = box_fn!(|x: i32, y: i32| -> i32 { 0 - (x + y)});
        let mut v1 = Vec::new();
        for i1 in 0..100 {
            let i2 = i1 + 10;
            v1.push(spawn_fn!(eg, i1));
            v1.push(spawn_fn!(also, i1, i2)); // woohoo multi-arity!
        }
        v1.push(spawn_fn!(||{println!("accepts closures to run in their own thread!"); 0}));
        let results: Vec<i32> = c![res.join().unwrap(), for res in v1.into_iter()];
        let res_set: HashSet<i32> = HashSet::from_iter(results.clone());
        assert_eq!(res_set.len(), results.len()); // no duplicates
        assert_eq!(201, results.len()); // no missing values
    }

    #[test]
    fn test_paren_call() {
        fn lgbt(l: &str, g: &str, b: &str, t: &str) -> bool {
            println!("LGBT stands for: {} {} {} {}", l, g, b, t);
            true
        }
        fn mixed(l: &str, g: &str, b: &str, t: i32) -> bool {
            println!("Params: {} {} {} {}", l, g, b, t);
            true
        }
        fn no_params() -> bool {
            println!("nice");
            true
        }

        assert!(call!(lgbt, ("let's", "go", "beach", "to the"))); // remember: this isn't actually
                                                                  // a tuple. it's parsed by the
                                                                  // macro before it can be tupled.
        let g = "go";
        assert!(call!(lgbt, ("let's", g, "beach", "to the"))); // identifiers work
        assert!(call!(mixed, ("hungry", g, "hippo", 2))); // mixed types work
        assert!(call!(no_params, ())) // no parameters
    }

    #[test]
    fn test_no_paren_call() {
        fn p(s1:&str, s2:&str) {
            println!("{}", s1);
            println!("{}", s2);
        }
        let n = "neato";
        call!(p, n, "hello!")
    }

    #[test]
    fn test_vec_call() {
        fn lgbt(l: &str, g: &str, b: &str, t: &str) -> bool {
            println!("Vector says LGBT stands for: {} {} {} {}", l, g, b, t);
            true
        }
        let v = vec!["let's", "go", "beach", "to the"];
        assert!{vcall!{lgbt, v}};
    }
}
