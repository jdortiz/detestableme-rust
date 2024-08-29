#![allow(dead_code)]
//! Module to define henchmen.

/// Henchman trait.
pub trait Henchman {
    fn build_secret_hq(&mut self, location: String);
    fn do_hard_things(&self);
    fn fight_enemies(&self);
}
