#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error,  ensure,StorageMap,dispatch};
use frame_system::{self as system, ensure_signed};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	//type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		// Just a dummy storage item.
		// Here we are declaring a StorageValue, `Something` as a Option<u32>
		// `get(fn something)` is the default getter which returns either the stored `u32` or `None` if nothing stored
	//Something get(fn something): Option<u32>;
	Proofs: map hasher(blake2_128_concat) Vec<u8> => (T::AccountId, T::BlockNumber);
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		//SomethingStored(u32, AccountId),
		ClaimCreated(AccountId, Vec<u8>),
		ClaimRevoked(AccountId, Vec<u8>),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		//NoneValue,
		/// Value reached maximum and cannot be incremented further
	//	StorageOverflow,
		ProofAlreadyClaimed,
		NoSuchProof,
		NotProofOwner,
		NotClaimOwner,
		SameClaimOwner,
		ProofTooLong,
		ClaimNotExist,

	}
}

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		/* #[weight = 10_000]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		} */
		/* #[weight=10_000]
		fn create_claim(origin,proof:Vec<u8>){
			let sender=ensure_signed(origin)?;
			ensure!(!Proofs::<T>::constants_key(&proof),Error::<T>::ProofAlreadyClaimed);
			let current_block=<system::Module<T>>::block_number();
			Proofs::<T>::insert(&proof,(&sender,current_block));
			Self::deposit_event(RawEvent::ClaimCreated(sender,proof));
			Ok(());
		}

		#[weight=10_000]
		fn revoke_claim(origin,proof:Vec<u8>){
			let sender=ensure_signed(origin)?;
			ensure!(Proofs::<T>::constants_key(&proof),Erroe::<T>::NoSuchProof);
			let(owner,_)=Proofs::<T>::get(&proof);
			Proofs::<T>::remove(&proof);
			Self::deposit_event(RawEvent::ClaimRevoked(sender,proof));
			Ok(());
		} */
		#[weight = 10_000]
        fn create_claim(origin, proof: Vec<u8>) {
            // Verify that the incoming transaction is signed and store who the
            // caller of this function is.
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has not been claimed yet or error with the message
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            // Call the `system` pallet to get the current block number
            let current_block = <system::Module<T>>::block_number();

            // Store the proof with the sender and the current block number
            Proofs::<T>::insert(&proof, (&sender, current_block));

            // Emit an event that the claim was created
            Self::deposit_event(RawEvent::ClaimCreated(sender, proof));
        }

        /// Allow the owner to revoke their claim
        #[weight = 10_000]
        fn revoke_claim(origin, proof: Vec<u8>) {
            // Determine who is calling the function
            let sender = ensure_signed(origin)?;

            // Verify that the specified proof has been claimed
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            // Get owner of the claim
            let (owner, _) = Proofs::<T>::get(&proof);

            // Verify that sender of the current call is the claim owner
            ensure!(sender == owner, Error::<T>::NotProofOwner);

            // Remove claim from storage
            Proofs::<T>::remove(&proof);

            // Emit an event that the claim was erased
            Self::deposit_event(RawEvent::ClaimRevoked(sender, proof));
        }
		
		#[weight = 10_000]
		 fn transfer_claim(origin, send_to: T::AccountId ,claim: Vec<u8>) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(claim.len() <= 1, Error::<T>::ProofTooLong);
			ensure!(Proofs::<T>::contains_key(&claim),Error::<T>::ClaimNotExist);
			let (owner, _block_number) = Proofs::<T>::get(&claim);
			ensure!(owner == sender, Error::<T>::NotClaimOwner);
			ensure!(owner != send_to, Error::<T>::SameClaimOwner);
			Proofs::<T>::remove(&claim);
			Proofs::<T>::insert(&claim, (send_to.clone(), system::Module::<T>::block_number()));
			Self::deposit_event(RawEvent::ClaimRevoked(sender, claim));
			Ok(())
		}

	} 
}
