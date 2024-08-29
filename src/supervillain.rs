#![allow(missing_docs)]
//! Module for supervillains and their related stuff
use std::time::Duration;

use anyhow::anyhow;

#[cfg(not(test))]
use crate::sidekick::Sidekick;
use crate::{Gadget, Henchman};
#[cfg(test)]
use tests::doubles::Sidekick;

/// Type that represents supervillains.
#[derive(Default)]
pub struct Supervillain<'a> {
    pub first_name: String,
    pub last_name: String,
    pub sidekick: Option<Sidekick<'a>>,
}

pub trait Megaweapon {
    fn shoot(&self);
}

impl Supervillain<'_> {
    /// Return the value of the full name as a single string.
    ///
    /// Full name is produced concatenating first name, a single space, and the last name.
    ///
    /// # Examples
    /// ```
    /// # use evil::supervillain::Supervillain;
    /// let lex = Supervillain {
    ///     first_name: "Lex".to_string(),
    ///     last_name: "Luthor".to_string(),
    /// };
    /// assert_eq!(lex.full_name(), "Lex Luthor");
    /// ```
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn set_full_name(&mut self, name: &str) {
        let components = name.split(" ").collect::<Vec<_>>();
        println!("Received {} components.", components.len());
        if components.len() != 2 {
            panic!("Name must have first and last name");
        }
        self.first_name = components[0].to_string();
        self.last_name = components[1].to_string();
    }
    pub fn attack(&self, weapon: &impl Megaweapon) {
        weapon.shoot();
    }
    pub async fn come_up_with_plan(&self) -> String {
        tokio::time::sleep(Duration::from_millis(100)).await;
        String::from("Take over the world!")
    }
    pub fn conspire(&mut self) {
        if let Some(ref sidekick) = self.sidekick {
            if !sidekick.agree() {
                self.sidekick = None;
            }
        }
    }

    pub fn start_world_domination_stage1<H: Henchman, G: Gadget>(
        &self,
        henchman: &mut H,
        gadget: &G,
    ) {
        if let Some(ref sidekick) = self.sidekick {
            let targets = sidekick.get_weak_targets(gadget);
            if !targets.is_empty() {
                henchman.build_secret_hq(targets[0].clone());
            }
        }
    }
}

impl TryFrom<&str> for Supervillain<'_> {
    type Error = anyhow::Error;
    fn try_from(name: &str) -> Result<Self, Self::Error> {
        let components = name.split(" ").collect::<Vec<_>>();
        if components.len() < 2 {
            Err(anyhow!("Too few arguments"))
        } else {
            Ok(Supervillain {
                first_name: components[0].to_string(),
                last_name: components[1].to_string(),
                sidekick: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::test_common;
    use test_context::{test_context, AsyncTestContext, TestContext};

    #[test_context(Context)]
    #[test]
    fn full_name_returns_first_name_space_last_name(ctx: &mut Context) {
        // Act
        let full_name = ctx.sut.full_name();
        // Assert
        assert_eq!(full_name, test_common::PRIMARY_FULL_NAME);
    }
    #[test_context(Context)]
    #[test]
    fn set_full_name_sets_first_and_last_name(ctx: &mut Context) {
        // Act
        ctx.sut.set_full_name(test_common::SECONDARY_FULL_NAME);
        // Assert
        assert_eq!(ctx.sut.first_name, test_common::SECONDARY_FIRST_NAME);
        assert_eq!(ctx.sut.last_name, test_common::SECONDARY_LAST_NAME);
    }
    #[test_context(Context)]
    // #[ignore]
    #[test]
    #[should_panic(expected = "Name must have first and last name")]
    fn set_full_name_panics_with_empty_name(ctx: &mut Context) {
        // Arrange

        // Act
        ctx.sut.set_full_name("");
        // Assert
    }
    #[test]
    fn from_str_slice_produces_supervillain_with_first_and_last_name() -> Result<(), anyhow::Error>
    {
        // Act
        let sut = Supervillain::try_from(test_common::SECONDARY_FULL_NAME)?;
        // Assert
        assert_eq!(sut.first_name, test_common::SECONDARY_FIRST_NAME);
        assert_eq!(sut.last_name, test_common::SECONDARY_LAST_NAME);
        Ok(())
    }
    #[test]
    // #[ignore]
    fn from_str_slice_produces_error_with_less_than_two_substrings() {
        // Act
        let result = Supervillain::try_from("");
        // Assert
        let Err(_) = result else {
            panic!("Unexpected value returned by try_from");
        };
    }
    #[test_context(Context)]
    #[test]
    fn attack_shoots_weapon(ctx: &mut Context) {
        // Arrange
        let weapon = WeaponDouble::new();
        // Act
        ctx.sut.attack(&weapon);
        // Assert
        assert!(*weapon.is_shot.borrow());
    }
    #[test_context(AsyncContext)]
    #[tokio::test]
    async fn plan_is_sadly_expected(ctx: &mut AsyncContext<'static>) {
        assert_eq!(ctx.sut.come_up_with_plan().await, "Take over the world!");
    }
    #[test_context(Context)]
    #[test]
    fn fire_sidekick_if_doesnt_agree_with_conspiracy(ctx: &mut Context) {
        let mut sk_double = doubles::Sidekick::new();
        sk_double.agree_answer = false;
        ctx.sut.sidekick = Some(sk_double);
        ctx.sut.conspire();
        assert!(
            ctx.sut.sidekick.is_none(),
            "Sidekick not fired unexpectedly"
        );
    }
    #[test_context(Context)]
    #[test]
    fn keep_sidekick_if_agrees_with_conspiracy(ctx: &mut Context) {
        let mut sk_double = doubles::Sidekick::new();
        sk_double.agree_answer = true;
        ctx.sut.sidekick = Some(sk_double);
        ctx.sut.conspire();
        assert!(ctx.sut.sidekick.is_some(), "Sidekick fired unexpectedly");
    }
    #[test_context(Context)]
    #[test]
    fn conspiracy_without_sidekick_doesnt_fail(ctx: &mut Context) {
        ctx.sut.conspire();
        assert!(ctx.sut.sidekick.is_none(), "Unexpected sidekick");
    }
    #[test_context(Context)]
    #[test]
    fn world_domination_stage1_builds_hq_in_first_weak_target(ctx: &mut Context) {
        let gdummy = GadgetDummy {};
        let mut hm_spy = HenchmanSpy { hq_location: None };
        let mut sk_double = doubles::Sidekick::new();
        sk_double.targets = test_common::TARGETS.map(String::from).to_vec();
        ctx.sut.sidekick = Some(sk_double);

        ctx.sut.start_world_domination_stage1(&mut hm_spy, &gdummy);

        assert_eq!(
            hm_spy.hq_location,
            Some(test_common::FIRST_TARGET.to_string())
        );
    }

    pub(crate) mod doubles {
        use std::marker::PhantomData;

        use crate::Gadget;

        pub struct Sidekick<'a> {
            phantom: PhantomData<&'a ()>,
            pub agree_answer: bool,
            pub targets: Vec<String>,
        }

        impl<'a> Sidekick<'a> {
            pub fn new() -> Sidekick<'a> {
                Sidekick {
                    phantom: PhantomData,
                    agree_answer: false,
                    targets: vec![],
                }
            }
            pub fn agree(&self) -> bool {
                self.agree_answer
            }
            pub fn get_weak_targets<G: Gadget>(&self, _gadget: &G) -> Vec<String> {
                self.targets.clone()
            }
        }
    }

    struct HenchmanSpy {
        hq_location: Option<String>,
    }

    impl Henchman for HenchmanSpy {
        fn build_secret_hq(&mut self, location: String) {
            self.hq_location = Some(location);
        }
    }

    struct GadgetDummy;

    impl Gadget for GadgetDummy {
        fn do_stuff(&self) {}
    }

    struct WeaponDouble {
        pub is_shot: RefCell<bool>,
    }
    impl WeaponDouble {
        fn new() -> WeaponDouble {
            WeaponDouble {
                is_shot: RefCell::new(false),
            }
        }
    }
    impl Megaweapon for WeaponDouble {
        fn shoot(&self) {
            *self.is_shot.borrow_mut() = true;
        }
    }
    struct Context<'a> {
        sut: Supervillain<'a>,
    }
    impl<'a> TestContext for Context<'a> {
        fn setup() -> Context<'a> {
            Context {
                sut: Supervillain {
                    first_name: test_common::PRIMARY_FIRST_NAME.to_string(),
                    last_name: test_common::PRIMARY_LAST_NAME.to_string(),
                    ..Default::default()
                },
            }
        }
        fn teardown(self) {}
    }
    struct AsyncContext<'a> {
        sut: Supervillain<'a>,
    }
    #[async_trait::async_trait]
    impl<'a> AsyncTestContext for AsyncContext<'a> {
        async fn setup() -> AsyncContext<'a> {
            AsyncContext {
                sut: Supervillain {
                    first_name: test_common::PRIMARY_FIRST_NAME.to_string(),
                    last_name: test_common::PRIMARY_LAST_NAME.to_string(),
                    ..Default::default()
                },
            }
        }
        async fn teardown(self) {}
    }
}
