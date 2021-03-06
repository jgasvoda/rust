// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-x86
// ignore-arm
// ignore-emscripten
// ignore 32-bit platforms (test output is different)

use std::mem;

unsafe fn foo() -> (i32, *const (), Option<fn()>) {
    let i = mem::transmute(bar);
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    let p = mem::transmute(foo);
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    let of = mem::transmute(main);
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    (i, p, of)
}

unsafe fn bar() {
    // Error as usual if the resulting type is not pointer-sized.
    mem::transmute::<_, u8>(main);
    //~^ ERROR transmute called with types of different sizes
    //~^^ NOTE transmuting between fn() {main} and u8

    mem::transmute::<_, *mut ()>(foo);
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    mem::transmute::<_, fn()>(bar);
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    // No error if a coercion would otherwise occur.
    mem::transmute::<fn(), u32>(main);
}

unsafe fn baz() {
    mem::transmute::<_, *mut ()>(Some(foo));
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    mem::transmute::<_, fn()>(Some(bar));
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    mem::transmute::<_, Option<fn()>>(Some(baz));
    //~^ ERROR is zero-sized and can't be transmuted
    //~^^ NOTE cast with `as` to a pointer instead

    // No error if a coercion would otherwise occur.
    mem::transmute::<Option<fn()>, u32>(Some(main));
}

fn main() {
    unsafe {
        foo();
        bar();
        baz();
    }
}
