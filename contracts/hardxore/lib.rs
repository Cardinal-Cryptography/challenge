#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod hardxore {
    use ink_env::hash::{HashOutput, Sha2x256};
    use ink_prelude::{string::String, vec::Vec};
    use ink_storage::{traits::SpreadAllocate, Mapping};
    use scale::{Decode, Encode};

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Hardxore {
        pub seed: Hash256,
        // Registered blocks
        pub randomness: Mapping<BlockNumber, Hash256>,
        // Badges awarded so far. Entry (acc, badge) -> num, means that `badge` was awarded to `acc` at block `num`.
        // Badges available in this challenge: `WARMUP`, `XOR-0`, `XOR-1`, `XOR-2`, `XOR-3`.
        pub badges: Mapping<(AccountId, String), BlockNumber>,
        // Holds solution hashes that have already been used for any challenge.
        pub used_solutions: Mapping<Hash256, ()>,
    }

    // Around 24h worth of blocks
    const MAX_BLOCKS_IN_THE_PAST: BlockNumber = 24 * 60 * 60;

    /// Error scenarios in hardxore contract
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        BlockAlreadyHasRandomness(BlockNumber),
        NoRandomnessRegistered(BlockNumber),
        BadgeAlreadyHeld(BlockNumber),
        SolutionHasBeenUsed,
        BlockTooFarInThePast(BlockNumber),
        CloseButNoCigar,
        SolutionCannotBeEmpty,
        DuplicateNumber(BlockNumber),
    }

    #[ink(event)]
    pub struct BadgeAwarded {
        acc: AccountId,
        badge: Badge,
    }

    #[ink(event)]
    pub struct RandomnessRegistered {
        num: BlockNumber,
        randomness: Hash256,
    }

    pub type Result<T> = core::result::Result<T, Error>;
    // Hash256 is simply [u8; 32]
    pub type Hash256 = <Sha2x256 as HashOutput>::Type;
    pub type Badge = String;

    impl Hardxore {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                // Initialize the seed pseudorandomly
                contract.seed = Self::env().hash_bytes::<Sha2x256>(&[68, 124, 213, 7, 140]);
            })
        }

        pub fn hash_solution(solution: &[BlockNumber]) -> Hash256 {
            let input = solution.encode();
            Self::env().hash_bytes::<Sha2x256>(&input)
        }

        fn award_badge(&mut self, badge: &str) -> Result<(Badge, BlockNumber)> {
            let caller = Self::env().caller();
            match self.badges.get((caller, String::from(badge))) {
                Some(num) => Err(Error::BadgeAlreadyHeld(num)),
                None => {
                    let current_num = Self::env().block_number();
                    self.badges
                        .insert((caller, String::from(badge)), &current_num);
                    Self::env().emit_event(BadgeAwarded {
                        acc: caller,
                        badge: String::from(badge),
                    });
                    Ok((String::from(badge), current_num))
                }
            }
        }

        // Outputs `Some(num)` if the badge has been awarded at block `num`, otherwise `None`
        #[ink(message)]
        pub fn has_badge(&self, acc: AccountId, badge: String) -> Option<BlockNumber> {
            self.badges.get((acc, badge))
        }

        #[ink(message, payable)]
        pub fn just_give_me_a_badge(&mut self) -> Result<(Badge, BlockNumber)> {
            if Self::env().transferred_value() == 0x1c90 {
                return self.award_badge("WARMUP");
            }
            Err(Error::CloseButNoCigar)
        }

        /// Checks that:
        /// 0) The solution is not empty,
        /// 1) The solution has not been used before,
        /// 2) There are no neighboring duplicate entries,
        /// 3) Every number corresponds to a block no further than `MAX_BLOCKS_IN_THE_PAST` blocks in the past
        /// 4) At every number some randomness was registered
        /// Outputs a `Hash256` being the bitwise XOR of the randomnesses corresponding to block numbers.
        fn prevalidate_and_xor(&self, solution: &[BlockNumber]) -> Result<Hash256> {
            if solution.is_empty() {
                return Err(Error::SolutionCannotBeEmpty);
            }
            let current_num = Self::env().block_number();
            let sol_hash = Self::hash_solution(solution);
            if self.used_solutions.get(sol_hash).is_some() {
                return Err(Error::SolutionHasBeenUsed);
            }
            let mut xored_hashes = [0u8; 32];
            let mut previous = None;
            for num in solution {
                if let Some(prev) = previous {
                    if num == prev {
                        return Err(Error::DuplicateNumber(*num));
                    }
                }
                previous = Some(num);
                if current_num.saturating_sub(*num) >= MAX_BLOCKS_IN_THE_PAST {
                    return Err(Error::BlockTooFarInThePast(*num));
                }
                if let Some(randomness) = self.randomness.get(num) {
                    // Below: xored_hashes = xored_hashes XOR randomness;
                    xored_hashes
                        .iter_mut()
                        .zip(randomness.iter())
                        .for_each(|(a, b)| *a ^= *b);
                } else {
                    return Err(Error::NoRandomnessRegistered(*num));
                }
            }
            Ok(xored_hashes)
        }

        fn save_solution_as_used(&mut self, solution: &[BlockNumber]) {
            self.used_solutions
                .insert(Self::hash_solution(solution), &());
        }

        #[ink(message)]
        pub fn attempt_xor_0(
            &mut self,
            solution: Vec<BlockNumber>,
        ) -> Result<(Badge, BlockNumber)> {
            let mut solution = solution;
            solution.sort_unstable();
            let xored_hashes = self.prevalidate_and_xor(&solution)?;
            if xored_hashes[0] == 0 {
                self.save_solution_as_used(&solution);
                return self.award_badge("XOR-0");
            }
            Err(Error::CloseButNoCigar)
        }

        #[ink(message)]
        pub fn attempt_xor_1(
            &mut self,
            solution: Vec<BlockNumber>,
        ) -> Result<(Badge, BlockNumber)> {
            let mut solution = solution;
            solution.sort_unstable();
            let xored_hashes = self.prevalidate_and_xor(&solution)?;
            if xored_hashes[0] == 0 && xored_hashes[1] == 0 && xored_hashes[2] == 0 {
                self.save_solution_as_used(&solution);
                return self.award_badge("XOR-1");
            }
            Err(Error::CloseButNoCigar)
        }

        #[ink(message)]
        pub fn attempt_xor_2(
            &mut self,
            solution: Vec<BlockNumber>,
        ) -> Result<(Badge, BlockNumber)> {
            let xored_hashes = self.prevalidate_and_xor(&solution)?;
            if xored_hashes == [0u8; 32] {
                self.save_solution_as_used(&solution);
                return self.award_badge("XOR-2");
            }
            Err(Error::CloseButNoCigar)
        }

        #[ink(message)]
        pub fn attempt_xor_3(
            &mut self,
            solution: Vec<BlockNumber>,
        ) -> Result<(Badge, BlockNumber)> {
            let mut solution = solution;
            solution.sort_unstable();
            let xored_hashes = self.prevalidate_and_xor(&solution)?;
            if xored_hashes == [0u8; 32] {
                self.save_solution_as_used(&solution);
                return self.award_badge("XOR-3");
            }
            Err(Error::CloseButNoCigar)
        }

        /// Generates a new seed as `Hash(old_seed, acc, num)` and outputs it
        fn generate_fresh_randomness(&mut self, acc: AccountId, num: BlockNumber) -> Hash256 {
            let mut input: Vec<u8> = self.seed.to_vec();
            input.extend_from_slice(acc.as_ref());
            input.extend_from_slice(&num.encode());
            self.seed = Self::env().hash_bytes::<Sha2x256>(&input);
            self.seed
        }

        #[ink(message)]
        pub fn get_randomness(&self, num: BlockNumber) -> Result<Hash256> {
            match self.randomness.get(num) {
                Some(randomness) => Ok(randomness),
                None => Err(Error::NoRandomnessRegistered(num)),
            }
        }

        #[ink(message)]
        pub fn register_randomness(&mut self) -> Result<(BlockNumber, Hash256)> {
            let current_num = Self::env().block_number();
            if self.randomness.get(current_num).is_some() {
                return Err(Error::BlockAlreadyHasRandomness(current_num));
            }
            let caller = Self::env().caller();
            let randomness = self.generate_fresh_randomness(caller, current_num);
            self.randomness.insert(current_num, &randomness);
            Self::env().emit_event(RandomnessRegistered {
                num: current_num,
                randomness,
            });
            Ok((current_num, randomness))
        }
    }
}
