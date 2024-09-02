#![allow(dead_code)]
//! Module for sideckicks and all the related functionality

#[cfg(test)]
use mockall::mock;

use crate::Gadget;

/// Type that represents a sidekick.
pub struct Sidekick<'a> {
    gadget: Box<dyn Gadget + 'a>,
}

impl<'a> Sidekick<'a> {
    pub fn new<G: Gadget + 'a>(gadget: G) -> Sidekick<'a> {
        Sidekick {
            gadget: Box::new(gadget),
        }
    }

    pub fn agree(&self) -> bool {
        true
    }

    pub fn get_weak_targets<G: Gadget>(&self, _gadget: &G) -> Vec<String> {
        vec![]
    }

    pub fn tell(&self, _ciphered_msg: String) {}
}

#[cfg(test)]
mock! {
    pub Sidekick<'a> {
        pub fn agree(&self) -> bool;
        pub fn get_weak_targets(&self, _gadget: &'a dyn Gadget) -> Vec<String>;
        pub fn tell(&self, _ciphered_msg: String);
    }
}
