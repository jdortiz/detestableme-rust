#![allow(missing_docs)]
//! Module for supervillains and their related stuff
use std::time::Duration;

use anyhow::anyhow;
#[cfg(test)]
use mockall::{automock, predicate::eq};
#[cfg(test)]
use mockall_double::double;

#[cfg_attr(test, double)]
use crate::sidekick::Sidekick;
use crate::{Cipher, Gadget, Henchman};

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
    use mockall::Sequence;
    use test_context::{test_context, AsyncTestContext, TestContext};

    use super::*;

    use crate::cipher::MockCipher;
    use crate::gadget::MockGadget;
    use crate::henchman::MockHenchman;
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
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick.expect_agree().once().return_const(false);
        ctx.sut.sidekick = Some(mock_sidekick);

        ctx.sut.conspire();
        assert!(
            ctx.sut.sidekick.is_none(),
            "Sidekick not fired unexpectedly"
        );
    }
    #[test_context(Context)]
    #[test]
    fn keep_sidekick_if_agrees_with_conspiracy(ctx: &mut Context) {
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick.expect_agree().once().return_const(true);
        ctx.sut.sidekick = Some(mock_sidekick);

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
        let gdummy = MockGadget::new();
        let mut mock_henchman = MockHenchman::new();
        mock_henchman
            .expect_build_secret_hq()
            .with(eq(String::from(test_common::FIRST_TARGET)))
            .return_const(());
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick
            .expect_get_weak_targets()
            .once()
            .returning(|_| test_common::TARGETS.map(String::from).to_vec());
        ctx.sut.sidekick = Some(mock_sidekick);

        ctx.sut
            .start_world_domination_stage1(&mut mock_henchman, &gdummy);
    }
    #[test_context(Context)]
    #[test]
    fn world_domination_stage2_tells_henchman_to_do_hard_things_and_fight_with_enemies(
        ctx: &mut Context,
    ) {
        let mut mock_henchman = MockHenchman::new();
        let mut sequence = Sequence::new();
        mock_henchman
            .expect_do_hard_things()
            .once()
            .in_sequence(&mut sequence)
            .return_const(());
        mock_henchman
            .expect_fight_enemies()
            .once()
            .in_sequence(&mut sequence)
            .return_const(());

        ctx.sut.start_world_domination_stage2(mock_henchman);
    }
    #[test_context(Context)]
    #[test]
    fn tell_plans_sends_ciphered_message(ctx: &mut Context) {
        let mut mock_sidekick = Sidekick::new();
        mock_sidekick
            .expect_tell()
            .with(eq(String::from(test_common::MAIN_CIPHERED_MESSAGE)))
            .once()
            .return_const(());
        ctx.sut.sidekick = Some(mock_sidekick);
        let mut mock_cipher = MockCipher::new();
        mock_cipher
            .expect_transform()
            .returning(|secret, _| String::from("+") + secret + "+");

        ctx.sut
            .tell_plans(test_common::MAIN_SECRET_MESSAGE, &mock_cipher);
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
