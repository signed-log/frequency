use frame_support::traits::{LockIdentifier, WithdrawReasons};
use crate::msa::MessageSourceId;
use frame_support::traits::tokens::Balance;
use sp_core::Get;
use sp_runtime::DispatchError;

/// A trait for checking that a target MSA can be staked to.
pub trait TargetValidator {
	/// Checks if an MSA is a valid target.
	fn validate(target: MessageSourceId) -> bool;
}

/// A blanket implementation
impl TargetValidator for () {
	fn validate(_target: MessageSourceId) -> bool {
		false
	}
}

/// A trait for Non-transferable asset.
pub trait Nontransferable {
	/// Scalar type for representing balance of an account.
	type Balance: Balance;

	/// The balance Capacity for an MSA account.
	fn balance(msa_id: MessageSourceId) -> Self::Balance;

	/// Reduce Capacity of an MSA account by amount.
	fn deduct(msa_id: MessageSourceId, amount: Self::Balance) -> Result<(), DispatchError>;

	/// Increase Capacity of an MSA account by an amount.
	fn deposit(msa_id: MessageSourceId, amount: Self::Balance) -> Result<(), DispatchError>;
}

/// A trait for replenishing Capacity.
pub trait Replenishable {
	/// Scalar type for representing balance of an account.
	type Balance: Balance;

	/// Replenish an MSA's Capacity by an amount.
	fn replenish_by_amount(
		msa_id: MessageSourceId,
		amount: Self::Balance,
	) -> Result<(), DispatchError>;

	/// Replenish all Capacity balance for an MSA.
	fn replenish_all_for(msa_id: MessageSourceId) -> Result<(), DispatchError>;

	/// Checks if an account can be replenished.
	fn can_replenish(msa_id: MessageSourceId) -> bool;
}

/// A fungible token whose accounts can have liquidity restrictions.
pub trait LockableFungible<AccountId> {
	/// Scalar type for representing balance of an account.
	type Balance: Balance;

	/// The quantity used to denote time; usually just a `BlockNumber`.
	type Moment;

	/// The maximum number of locks a user should have on their account.
	type MaxLocks: Get<u32>;

	/// Create a new balance lock on account `who`.
	///
	/// If the new lock is valid (i.e. not already expired), it will push the struct to
	/// the `Locks` vec in storage. Note that you can lock more funds than a user has.
	///
	/// If the lock `id` already exists, this will update it.
	fn set_lock(
		id: LockIdentifier,
		who: &AccountId,
		amount: Self::Balance,
		reasons: WithdrawReasons,
	);

	/// Changes a balance lock (selected by `id`) so that it becomes less liquid in all
	/// parameters or creates a new one if it does not exist.
	///
	/// Calling `extend_lock` on an existing lock `id` differs from `set_lock` in that it
	/// applies the most severe constraints of the two, while `set_lock` replaces the lock
	/// with the new parameters. As in, `extend_lock` will set:
	/// - maximum `amount`
	/// - bitwise mask of all `reasons`
	fn extend_lock(
		id: LockIdentifier,
		who: &AccountId,
		amount: Self::Balance,
		reasons: WithdrawReasons,
	);

	/// Remove an existing lock.
	fn remove_lock(id: LockIdentifier, who: &AccountId);
}

/// A trait for freezing and thawing funds, preventing them from and releasing them for transfer
/// TODO: remove when we upgrade to version of substrate that includes fungible::freeze
/// and use that.
pub trait Freezable<AccountId> {
	/// Scalar type for representing balance of an account.
	type Balance: Balance;

	/// Create a new balance lock on account `who`.
	fn set_lock(
		id: LockIdentifier,
		who: &AccountId,
		amount: Self::Balance,
		reasons: WithdrawReasons,
	);

	/// Remove an existing lock.
	fn remove_lock(id: LockIdentifier, who: &AccountId);
}
