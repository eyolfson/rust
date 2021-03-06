// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test that borrowed pointers are not sendable unless 'static.

fn assert_send<T:Send>() { }

// lifetime pointers with 'static lifetime are ok
fn test01() { assert_send::<&'static isize>(); }
fn test02() { assert_send::<&'static str>(); }
fn test03() { assert_send::<&'static [isize]>(); }

// whether or not they are mutable
fn test10() { assert_send::<&'static mut isize>(); }

// otherwise lifetime pointers are not ok
fn test20<'a>(_: &'a isize) {
    assert_send::<&'a isize>(); //~ ERROR declared lifetime bound not satisfied
}
fn test21<'a>(_: &'a isize) {
    assert_send::<&'a str>(); //~ ERROR declared lifetime bound not satisfied
}
fn test22<'a>(_: &'a isize) {
    assert_send::<&'a [isize]>(); //~ ERROR declared lifetime bound not satisfied
}

fn main() { }
