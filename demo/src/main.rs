#![feature(plugin)]
#![plugin(pedantrs)]

fn lots_of_args(_: i32, _: i32, _: i32, _: i32, _:i32, _: i32, _: i32) {
}

fn main() {
    lots_of_args(1, 2, 3, 4, 5, 6, 7);
    let _ = |_: i32, _: i32, _: i32, _: i32, _: i32| {}; 
}
