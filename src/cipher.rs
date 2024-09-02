#![allow(dead_code)]
//! Module for sideckicks and all the related functionality

#[cfg(test)]
use mockall::automock;

/// Type that represents a sidekick.
#[cfg_attr(test, automock)]
pub trait Cipher {
    fn transform(&self, secret: &str, key: &str) -> String;
}
