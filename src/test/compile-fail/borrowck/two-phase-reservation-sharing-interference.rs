// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// ignore-tidy-linelength

// revisions: lxl_beyond nll_beyond nll_target

//[lxl_beyond]compile-flags: -Z borrowck=mir -Z two-phase-borrows -Z two-phase-beyond-autoref
//[nll_beyond]compile-flags: -Z borrowck=mir -Z two-phase-borrows -Z two-phase-beyond-autoref -Z nll
//[nll_target]compile-flags: -Z borrowck=mir -Z two-phase-borrows -Z nll

// This is a corner case that the current implementation is (probably)
// treating more conservatively than is necessary. But it also does
// not seem like a terribly important use case to cover.
//
// So this test is just making a note of the current behavior, with
// the caveat that in the future, the rules may be loosened, at which
// point this test might be thrown out.
//
// The convention for the listed revisions: "lxl" means lexical
// lifetimes (which can be easier to reason about). "nll" means
// non-lexical lifetimes. "nll_target" means the initial conservative
// two-phase borrows that only applies to autoref-introduced borrows.
// "nll_beyond" means the generalization of two-phase borrows to all
// `&mut`-borrows (doing so makes it easier to write code for specific
// corner cases).
//
// FIXME: in "nll_target", we currently see the same error reported
// twice. This is injected by `-Z two-phase-borrows`; not sure why as
// of yet.

fn main() {
    let mut vec = vec![0, 1];
    let delay: &mut Vec<_>;
    {
        let shared = &vec;

        // we reserve here, which could (on its own) be compatible
        // with the shared borrow. But in the current implementation,
        // its an error.
        delay = &mut vec;
        //[lxl_beyond]~^   ERROR cannot borrow `vec` as mutable because it is also borrowed as immutable
        //[nll_beyond]~^^  ERROR cannot borrow `vec` as mutable because it is also borrowed as immutable
        //[nll_target]~^^^ ERROR cannot borrow `vec` as mutable because it is also borrowed as immutable
        //[nll_target]~|   ERROR cannot borrow `vec` as mutable because it is also borrowed as immutable

        shared[0];
    }

    // the &mut-borrow only becomes active way down here.
    //
    // (At least in theory; part of the reason this test fails is that
    // the constructed MIR throws in extra &mut reborrows which
    // flummoxes our attmpt to delay the activation point here.)
    delay.push(2);
}

