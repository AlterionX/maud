#![feature(proc_macro_hygiene)]

extern crate maud_macros;

use maud_macros::html;

fn main() {
    let markup = html!{ 
        if {}
    };
}
