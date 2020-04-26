use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use mac::re;

macro_rules! t {
    ($e:expr) => {
        e.to_string + " I am a macro"
    }
}

fn main() {
    re!(t!("hi"));
}