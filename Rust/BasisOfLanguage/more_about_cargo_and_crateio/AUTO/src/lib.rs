//! # Rust Tutorial Art Lib
//! 
//! A library for modeling artistc concepts.


pub use self::{
    utils::mix,
    kinds::PrimaryColor
};


pub mod
kinds {
    /// The primary colors according to the RYB color model.
    pub enum
    PrimaryColor {
        Red, Yellow, Blue
    }

    /// The secondary colors according to the RYB color model.
    pub enum
    SecondaryColor {
        Orange, Green, Purple
    }
}

pub mod
utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// secondary color.
    pub fn
    mix(first_color: PrimaryColor, second_color: PrimaryColor) {
        unimplemented!();
    }
}