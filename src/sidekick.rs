#![allow(dead_code)]
//! Module for sideckicks and all the related functionality

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
}
