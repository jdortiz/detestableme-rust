#![allow(dead_code)]
//! Module to define henchmen.

#[cfg(test)]
use mockall::automock;

/// Henchman trait.
#[cfg_attr(test, automock)]
pub trait Henchman {
    fn build_secret_hq(&mut self, location: String);
    fn do_hard_things(&self);
    fn fight_enemies(&self);
}
