#![allow(missing_docs)]
//! Module for supervillains and their related stuff
use std::time::Duration;

use anyhow::anyhow;
#[cfg(test)]
use mockall::automock;

#[cfg(not(test))]
use crate::sidekick::Sidekick;
use crate::{Cipher, Gadget, Henchman};
#[cfg(test)]
use tests::doubles::Sidekick;

/// Type that represents supervillains.
#[derive(Default)]
pub struct Supervillain<'a> {
    pub first_name: String,
    pub last_name: String,
    pub sidekick: Option<Sidekick<'a>>,
    pub shared_key: String,
}

#[cfg_attr(test, automock)]
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

    pub fn start_world_domination_stage2<H: Henchman>(&self, henchman: H) {
        henchman.do_hard_things();
        henchman.fight_enemies();
    }

    pub fn tell_plans<C: Cipher>(&self, secret: &str, cipher: &C) {
        if let Some(ref sidekick) = self.sidekick {
            let ciphered_msg = cipher.transform(secret, &self.shared_key);
            sidekick.tell(ciphered_msg);
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
                ..Default::default()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use test_context::{test_context, AsyncTestContext, TestContext};

    use super::*;

    use crate::test_common;

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
        let mut weapon = MockMegaweapon::new();
        weapon.expect_shoot().once().return_const(());
        // Act
        ctx.sut.attack(&weapon);
        // Assert: automatic verification of the mock on drop
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
        let mut hm_spy = HenchmanDouble::default();
        let mut sk_double = doubles::Sidekick::new();
        sk_double.targets = test_common::TARGETS.map(String::from).to_vec();
        ctx.sut.sidekick = Some(sk_double);

        ctx.sut.start_world_domination_stage1(&mut hm_spy, &gdummy);

        assert_eq!(
            hm_spy.hq_location,
            Some(test_common::FIRST_TARGET.to_string())
        );
    }
    #[test_context(Context)]
    #[test]
    fn world_domination_stage2_tells_henchman_to_do_hard_things_and_fight_with_enemies(
        ctx: &mut Context,
    ) {
        let mut henchman = HenchmanDouble::default();
        henchman.assertions = vec![Box::new(move |h| h.verify_two_things_done())];

        ctx.sut.start_world_domination_stage2(henchman);
    }
    #[test_context(Context)]
    #[test]
    fn tell_plans_sends_ciphered_message(ctx: &mut Context) {
        let mut sk_double = doubles::Sidekick::new();
        sk_double.assertions = vec![Box::new(move |s| {
            s.verify_received_msg(test_common::MAIN_CIPHERED_MESSAGE)
        })];
        ctx.sut.sidekick = Some(sk_double);
        let fake_cipher = CipherDouble {};

        ctx.sut
            .tell_plans(test_common::MAIN_SECRET_MESSAGE, &fake_cipher);
    }
    pub(crate) mod doubles {
        use std::cell::RefCell;
        use std::marker::PhantomData;

        use crate::Gadget;

        pub struct Sidekick<'a> {
            phantom: PhantomData<&'a ()>,
            pub agree_answer: bool,
            pub targets: Vec<String>,
            pub received_msg: RefCell<String>,
            pub assertions: Vec<Box<dyn Fn(&Sidekick) -> () + Send>>,
        }

        impl<'a> Sidekick<'a> {
            pub fn new() -> Sidekick<'a> {
                Sidekick {
                    phantom: PhantomData,
                    agree_answer: false,
                    targets: vec![],
                    received_msg: RefCell::new(String::from("")),
                    assertions: vec![],
                }
            }
            pub fn agree(&self) -> bool {
                self.agree_answer
            }
            pub fn get_weak_targets<G: Gadget>(&self, _gadget: &G) -> Vec<String> {
                self.targets.clone()
            }
            pub fn tell(&self, ciphered_msg: String) {
                *self.received_msg.borrow_mut() = ciphered_msg;
            }
            pub fn verify_received_msg(&self, expected_msg: &str) {
                assert_eq!(*self.received_msg.borrow(), expected_msg);
            }
        }

        impl Drop for Sidekick<'_> {
            fn drop(&mut self) {
                for a in &self.assertions {
                    a(self);
                }
            }
        }
    }

    struct CipherDouble;

    impl Cipher for CipherDouble {
        fn transform(&self, secret: &str, _key: &str) -> String {
            String::from("+") + secret + "+"
        }
    }

    #[derive(Default)]
    struct HenchmanDouble {
        hq_location: Option<String>,
        done_hard_things: RefCell<bool>,
        fought_enemies: RefCell<bool>,
        pub assertions: Vec<Box<dyn Fn(&HenchmanDouble) -> () + Send>>,
    }

    impl HenchmanDouble {
        fn verify_two_things_done(&self) {
            assert!(*self.done_hard_things.borrow() && *self.fought_enemies.borrow());
        }
    }

    impl Henchman for HenchmanDouble {
        fn build_secret_hq(&mut self, location: String) {
            self.hq_location = Some(location);
        }
        fn do_hard_things(&self) {
            *self.done_hard_things.borrow_mut() = true;
        }
        fn fight_enemies(&self) {
            *self.fought_enemies.borrow_mut() = true;
        }
    }

    impl Drop for HenchmanDouble {
        fn drop(&mut self) {
            for a in &self.assertions {
                a(self);
            }
        }
    }

    struct GadgetDummy;

    impl Gadget for GadgetDummy {
        fn do_stuff(&self) {}
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
