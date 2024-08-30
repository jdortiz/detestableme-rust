#![allow(dead_code)]
//! Module for sideckicks and all the related functionality

/// Type that represents a sidekick.
pub trait Cipher {
    fn transform(&self, secret: &str, key: &str) -> String;
}
