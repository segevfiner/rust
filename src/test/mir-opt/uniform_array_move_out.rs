// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_syntax, slice_patterns, advanced_slice_patterns)]

fn move_out_from_end() {
    let a = [box 1, box 2];
    let [.., _y] = a;
}

fn move_out_by_subslice() {
    let a = [box 1, box 2];
    let [_y..] = a;
}

fn main() {
    move_out_by_subslice();
    move_out_from_end();
}

// END RUST SOURCE

// START rustc.move_out_from_end.UniformArrayMoveOut.before.mir
//     StorageLive(_6);
//      _6 = move _1[-1 of 1];
//      _0 = ();
// END rustc.move_out_from_end.UniformArrayMoveOut.before.mir

// START rustc.move_out_from_end.UniformArrayMoveOut.after.mir
//     StorageLive(_6);
//      _6 = move _1[1 of 2];
//      nop;
//      _0 = ();
// END rustc.move_out_from_end.UniformArrayMoveOut.after.mir

// START rustc.move_out_by_subslice.UniformArrayMoveOut.before.mir
//     StorageLive(_6);
//      _6 = move _1[0:];
// END rustc.move_out_by_subslice.UniformArrayMoveOut.before.mir

// START rustc.move_out_by_subslice.UniformArrayMoveOut.after.mir
//     StorageLive(_6);
//     StorageLive(_7);
//     _7 = move _1[0 of 2];
//     StorageLive(_8);
//     _8 = move _1[1 of 2];
//     _6 = [move _7, move _8];
//     StorageDead(_7);
//     StorageDead(_8);
//     nop;
//     _0 = ();
// END rustc.move_out_by_subslice.UniformArrayMoveOut.after.mir

// START rustc.move_out_by_subslice.RestoreSubsliceArrayMoveOut.before.mir
//     StorageLive(_6);
//     StorageLive(_7);
//     _7 = move _1[0 of 2];
//     StorageLive(_8);
//     _8 = move _1[1 of 2];
//     _6 = [move _7, move _8];
//     StorageDead(_7);
//     StorageDead(_8);
//     _0 = ();
// END rustc.move_out_by_subslice.RestoreSubsliceArrayMoveOut.before.mir

// START rustc.move_out_by_subslice.RestoreSubsliceArrayMoveOut.after.mir
//     StorageLive(_6);
//     nop;
//     nop;
//     nop;
//     nop;
//     _6 = move _1[0:];
//     nop;
//     nop;
//     nop;
//     _0 = ();
// END rustc.move_out_by_subslice.RestoreSubsliceArrayMoveOut.after.mir
