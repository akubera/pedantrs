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

fn nesting(a: i32, b: i32, c: i32, d: i32) {
    if a > b {
        if b > c {
            if c > d {
                println!("Warn for this block");
            } else {
                println!("Also warn for this block");
            }
        }
    }

    if a > b {
        if b > c {
            println!("No warn for this block");
        }
    } 

    match a {
        1 => {
            if a > b {
                // The warning would normally take place at this level if we
                // were just counting opening parenthesis as a nesting, but for
                // match expressions we ignore the nesting introduced by the 
                // match expression itself and instead start counting from the 
                // arms.
                if c > d {
                    println!("Warn for this block");
                }
            } 
        },
        _ => println!("No warn this block")
    }
}

fn main() {
    lots_of_args(1, 2, 3, 4, 5, 6, 7);
    not_so_many_args(1, 2, 3, 4, 5, 6);

    // Warn
    let _ = |_: i32, _: i32, _: i32, _: i32, _: i32| {}; 
    // No warn
    let _ = |_: i32, _: i32, _: i32, _: i32| {}; 

    nesting(1, 2, 3, 4);
}
