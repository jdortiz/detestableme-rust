#![allow(dead_code)]
//! Module to define henchmen.

/// Henchman trait.
pub trait Henchman {
    fn build_secret_hq(&mut self, location: String);
}
