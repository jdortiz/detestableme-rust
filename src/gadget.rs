#![allow(dead_code)]
//! Module for gadgets and all the related functionality

/// Trait that represents a gadget.
pub trait Gadget: Send {
    fn do_stuff(&self);
}
