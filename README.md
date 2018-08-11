# funfun

[![Build Status](https://travis-ci.org/DominicBurkart/funfun.svg?branch=master)](https://travis-ci.org/DominicBurkart/funfun)
[![Coverage Status](https://coveralls.io/repos/github/DominicBurkart/funfun/badge.svg?branch=master)](https://coveralls.io/github/DominicBurkart/funfun?branch=master)
[![Codecov Coverage Status](https://codecov.io/gh/DominicBurkart/funfun/branch/master/graphs/badge.svg)](https://codecov.io/gh/DominicBurkart/funfun)
[![Crates.io](https://img.shields.io/crates/v/funfun.svg)](https://crates.io/crates/funfun)

### heap_fn!
macro for allocating closures to the heap. Heap-allocated closures can
be convenient when (re)assigning closures to structure fields, though
heap allocation comes at the expense of inline optimization.

 Usage:
```rust
let closure = heap_fn!(
    || {
        println!("This closure lives in the heap now!")
    }
);

closure.c()(); // "This closure lives in the heap now!"

let closure_identifier = || {println!("Named closure!")};

heap_fn!(closure_identifier).c()(); // "Named closure!"

```