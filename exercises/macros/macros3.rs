// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    macro_rules! another_macro {
      () => {
        let ten: usize = 10;
        println!("The number is: {}", ten)
      }
    }
}

fn main() {
    my_macro!();
    another_macro!();
}
