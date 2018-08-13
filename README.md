# funfun

[![Build Status](https://travis-ci.org/DominicBurkart/funfun.svg?branch=master)](https://travis-ci.org/DominicBurkart/funfun)
[![Coverage Status](https://coveralls.io/repos/github/DominicBurkart/funfun/badge.svg?branch=master)](https://coveralls.io/github/DominicBurkart/funfun?branch=master)
[![Codecov Coverage Status](https://codecov.io/gh/DominicBurkart/funfun/branch/master/graphs/badge.svg)](https://codecov.io/gh/DominicBurkart/funfun)
[![Crates.io](https://img.shields.io/crates/v/funfun.svg)](https://crates.io/crates/funfun)
[![Rust Documentation](https://docs.rs/funfun/badge.svg)](https://docs.rs/funfun)


### spawn_fn!
```spawn_fn!``` Takes a closure or function and its arguments, runs the
closure or function with the passed arguments in a new thread, and
returns the thread's hook.

``` rust
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
```

### box_fn!
```box_fn!``` Boxes a closure and returns an Rc pointer.
```rust
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
```

### arc_fn!
```arc_fn!``` Boxes a closure and returns an Arc pointer. Slower than
an Rc pointer, but allows derivation of traits like Clone.
```rust
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
```