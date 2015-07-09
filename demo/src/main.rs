#![feature(plugin)]
#![plugin(pedantrs)]

// Warn
pub const UNDOCUMENTED_CONST: i32 = 6;
// No warn
/// Documentation
pub const DOCUMENTED_CONST: i32 = 6;

// Warn
fn lots_of_args(_: i32, _: i32, _: i32, _: i32, _:i32, _: i32, _: i32) {
}
// No warn
fn not_so_many_args(_: i32, _: i32, _: i32, _: i32, _:i32, _:i32) {
}

fn main() {
    lots_of_args(1, 2, 3, 4, 5, 6, 7);
    not_so_many_args(1, 2, 3, 4, 5, 6);

    // Warn
    let _ = |_: i32, _: i32, _: i32, _: i32, _: i32| {}; 
    // No warn
    let _ = |_: i32, _: i32, _: i32, _: i32| {}; 
}
