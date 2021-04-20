#![allow(dead_code)]
#![no_std]
#![feature(new_uninit)]

#[cfg(test)]
#[macro_use]
extern crate std;

extern crate alloc;
pub mod list;
pub mod tree;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
