# funfun

[![Build Status](https://travis-ci.org/DominicBurkart/funfun.svg?branch=master)](https://travis-ci.org/DominicBurkart/funfun)
[![Coverage Status](https://coveralls.io/repos/github/DominicBurkart/funfun/badge.svg?branch=master)](https://coveralls.io/github/DominicBurkart/funfun?branch=master)
[![Codecov Coverage Status](https://codecov.io/gh/DominicBurkart/funfun/branch/master/graphs/badge.svg)](https://codecov.io/gh/DominicBurkart/funfun)
[![Crates.io](https://img.shields.io/crates/v/funfun.svg)](https://crates.io/crates/funfun)

### heap_fn!
```heap_fn!``` allocates a given closure to the heap, returning an
asynchronous reference to it. Heap-allocated closures can be convenient
when (re)assigning closures to structure fields, though heap allocation
comes at the expense of closure inline optimization. I don't see a use
for hiding a function behind an Arc in the heap, but the macro also
works with ```fn``` objects.

Notes:
- A HeapFn type is an Arc pointing towards a boxed value (e.g. a
closure). In many instances where you need a heap-allocated closure,
just boxing it (without worrying about asynchronous reference
management) is enough. Make sure that you actually want the overhead of
asynchronous pointers before you use the heap_fn macro!
- Closures were boxed in earlier versions of Rust, but the advantages
of inline optimization on performance drove the development of inline
closures.
- In this implementation, the closure is stored on the heap as long as
it is being referenced. The closure's space in memory is released as
soon as the last reference to it (including clones and across all
threads) is destroyed.

 Usage:
```rust
let closure = heap_fn!(
    || {
        println!("This closure lives in the heap now!")
    }
);

closure(); // "This closure lives in the heap now!"

let closure_identifier = || {println!("Named closure!")};

heap_fn!(closure_identifier)(); // "Named closure!"


// example use in structs:

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

```