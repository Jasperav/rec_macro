use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use mac::re;

macro_rules! t {
    ($e:expr) => {
        e
    }
}

fn main() {
    re!(t(1));
}