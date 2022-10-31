//! # Escrow
//!
//! This implements an Escrow contract.
//!
//! ## Warning
//!
//! This contract is an *example*. It is neither audited nor endorsed for production use.
//! Do **not** rely on it to keep anything of value secure.
//!
//! ## Overview
//!
//! Escrow is the third party which holds the asset (asset can be money, bond, stocks)
//! on the presence of two parties. Escrow will release the fund when certain conditions are met.
//!
//! In this contract, there are two parties, a buyer, and a seller. The buyer wants
//! to buy some goods from the seller, and use this smart contract instance as trusted entity for
//! depositing funds. The contract is instantiated by the buyer.
//!
//! This contract can end up in two ways:
//! * either delivery of the goods is confirmed by the buyer -> a deposit is transferred to the seller, or
//! * delivery is cancelled by the seller -> a deposit is refunded to the buyer.
//! In each way contract terminates itself and any remaining funds are transferred to contract
//! constructor account (the buyer)
//!
//! Implementation note: due to how things are implemented in Substrate, a storage deposit for this
//! contract is returned to message caller. This means that the deposit made by contract instantiation
//! account, which is the buyer, can be acquired by the seller by calling `refund()`. This value
//! can be non-negligible on some chains.

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
pub mod challenge {
    use scale::{Decode, Encode};

    #[ink(storage)]
    pub struct Escrow {
        /// Buyer's account
        buyer: AccountId,

        /// Sellers's account
        seller: AccountId,

        /// Deposit value
        deposit: Balance,
    }

    /// Error scenarios in escrow contract
    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Someone else than buyer tries to confirm
        ConfirmNotAsBuyer,

        /// Someone else than seller tries to refund
        RefundNotAsSeller,

        /// requested transfer failed, this can be the case if the contract does not
        /// have sufficient free funds or if the transfer would have brought the
        /// contract's balance below minimum balance
        TransferFailed,
    }

    /// An event emitted when token transfer occurs
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Escrow {
        /// Instantiates new escrow contract with buyer as contract author
        #[ink(constructor, payable)]
        pub fn new(seller: AccountId) -> Self {
            let escrow = Self {
                buyer: Self::env().caller(),
                seller,
                deposit: Self::env().transferred_value(),
            };
            Self::env().emit_event(Transfer {
                to: Self::env().account_id(),
                value: escrow.deposit,
            });

            escrow
        }

        /// Returns currently stored deposit
        #[ink(message)]
        pub fn get_deposit(&self) -> Balance {
            self.deposit
        }

        /// Returns buyers account
        #[ink(message)]
        pub fn get_buyer(&self) -> AccountId {
            self.buyer
        }

        /// Returns buyers account
        #[ink(message)]
        pub fn get_seller(&self) -> AccountId {
            self.seller
        }

        /// The buyer confirms delivery. Contract transfers a deposit to the seller and terminates itself
        #[ink(message)]
        pub fn confirm(&mut self) -> Result<()> {
            let caller = Self::env().caller();
            if caller != self.buyer {
                return Err(Error::ConfirmNotAsBuyer);
            }
            self.make_transfer(self.seller, self.deposit)?;
            self.env().terminate_contract(caller)
        }

        /// The seller aborts transaction. Contract refunds a deposit to the buyer and terminates itself
        #[ink(message)]
        pub fn refund(&mut self) -> Result<()> {
            let caller = Self::env().caller();
            if caller != self.seller {
                return Err(Error::RefundNotAsSeller);
            }
            self.make_transfer(self.buyer, self.deposit)?;
            self.env().terminate_contract(caller)
        }

        fn make_transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            self.env()
                .transfer(to, value)
                .map_err(|_| Error::TransferFailed)?;
            self.env().emit_event(Transfer { to, value });
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;
        use ink_env::AccountId;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        fn get_default_test_accounts() -> ink_env::test::DefaultAccounts<ink_env::DefaultEnvironment>
        {
            ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
        }

        fn set_balance(account_id: AccountId, balance: Balance) {
            ink_env::test::set_account_balance::<ink_env::DefaultEnvironment>(account_id, balance)
        }

        fn get_balance(account_id: AccountId) -> Balance {
            ink_env::test::get_account_balance::<ink_env::DefaultEnvironment>(account_id)
                .expect("Cannot get account balance")
        }

        fn contract_id() -> AccountId {
            ink_env::test::callee::<ink_env::DefaultEnvironment>()
        }

        fn set_caller(caller: AccountId) {
            ink_env::test::set_caller::<ink_env::DefaultEnvironment>(caller);
        }

        fn create_contract(deposit: Balance) -> (Escrow, AccountId, AccountId) {
            let accounts = get_default_test_accounts();
            let buyer = accounts.eve;
            let seller = accounts.frank;

            set_caller(buyer);
            set_balance(contract_id(), deposit);
            let escrow = Escrow::new(seller);

            (escrow, buyer, seller)
        }

        fn confirm(escrow: &mut Escrow, buyer: AccountId) -> () {
            set_caller(buyer);
            escrow.confirm().expect("Confirm failed!")
        }

        fn refund(escrow: &mut Escrow, seller: AccountId) -> () {
            set_caller(seller);
            escrow.refund().expect("Refund failed!")
        }

        #[ink::test]
        fn when_constructor_is_called_then_contract_has_deposit() {
            const DEPOSIT: Balance = 123;
            let (escrow, buyer, seller) = create_contract(DEPOSIT);
            assert_eq!(escrow.buyer, buyer);
            assert_eq!(escrow.seller, seller);
            assert_eq!(get_balance(contract_id()), DEPOSIT);
        }

        #[ink::test]
        fn when_buyer_confirms_then_seller_receives_deposit() {
            const DEPOSIT: Balance = 10;
            let (mut escrow, buyer, _) = create_contract(DEPOSIT);

            let should_terminate = move || confirm(&mut escrow, buyer);
            // unfortunately we can't test much in this UT, since below test only outcome of
            // self.env().terminate_contract(); the confirm() panics in off-chain testing environment
            // so there's no easy way to test state directly of the escrow contract after confirm()
            ink_env::test::assert_contract_termination::<ink_env::DefaultEnvironment, _>(
                should_terminate,
                buyer,
                DEPOSIT,
            );
            // we can only indirectly check account balances
            assert_eq!(get_balance(contract_id()), 0);
            // below incorrectly returns account values (should be other way round)
            // assert_eq!(get_balance(seller), 0);
            // assert_eq!(get_balance(buyer), DEPOSIT);
        }

        #[ink::test]
        fn when_not_buyer_confirms_then_error_is_returned() {
            let (mut escrow, _, seller) = create_contract(0);
            set_caller(seller);
            assert_eq!(escrow.confirm(), Err(Error::ConfirmNotAsBuyer));
        }

        #[ink::test]
        fn when_seller_refunds_then_deposit_is_returned_to_buyer() {
            const DEPOSIT: Balance = 1123;
            let (mut escrow, _, seller) = create_contract(DEPOSIT);

            let should_terminate = move || refund(&mut escrow, seller);
            // unfortunately we can't test much in this UT, since below test only outcome of
            // self.env().terminate_contract(); the refund() panics in off-chain testing environment
            // so there's no easy way to test state directly of the escrow contract after refund()
            ink_env::test::assert_contract_termination::<ink_env::DefaultEnvironment, _>(
                should_terminate,
                seller,
                DEPOSIT,
            );
            // we can only indirectly check account balances
            assert_eq!(get_balance(contract_id()), 0);
            // below incorrectly returns account values (should be other way round)
            // assert_eq!(get_balance(seller), DEPOSIT);
            // assert_eq!(get_balance(buyer), 0);
        }

        #[ink::test]
        fn when_not_seller_refunds_then_error_is_returned() {
            let (mut escrow, buyer, _) = create_contract(0);
            set_caller(buyer);
            assert_eq!(escrow.refund(), Err(Error::RefundNotAsSeller));
        }
    }
}
