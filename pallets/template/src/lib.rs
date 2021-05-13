#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::traits::Vec;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    // must import this if you want to use T::Hashing::hash or hash_of
    // use sp_runtime::traits::Hash;

    // sp_core::hashing can't be imported
    // docs recommend using sp_io::hashing which is made available
    // through the magic of macros
    use sp_io::hashing::blake2_128;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[derive(Encode, Decode, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "std", derive(Debug))]
    pub struct Kitty {
        pub id: u64,
        pub dna: [u8; 16],
    }

    // The pallet's runtime storage items.
    // https://substrate.dev/docs/en/knowledgebase/runtime/storage
    #[pallet::storage]
    #[pallet::getter(fn nonce)]
    pub type Nonce<T: Config> = StorageValue<_, u64>;

    #[pallet::storage]
    #[pallet::getter(fn owner_to_kitties)]
    pub type OwnerToKitties<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Option<Kitty>>>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_to_owner)]
    pub type KittyToOwner<T: Config> = StorageMap<_, Blake2_128Concat, u64, T::AccountId>;

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// parameters. [kitty_id, who]
        KittyCreated(u64, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        NonceFailed,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[pallet::weight(10_000 + T::DbWeight::get().writes(3))]
        pub fn create_kitty(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let who = ensure_signed(origin)?;

            let nonce = Self::encode_and_update_nonce()?;

            <Nonce<T>>::put(nonce);

            // twox_128 and blake2_128 seem to be interchangeable
            // let dna = twox_128(&nonce.to_be_bytes());
            let dna = blake2_128(&nonce.to_be_bytes());

            // example of 256 bit hash using the Hash type configured for the runtime
            // let dna = T::Hashing::hash_of(&nonce.to_be_bytes()).as_ref();
            debug::info!("{:?}", dna);

            let kitty = Kitty { id: nonce, dna };

            OwnerToKitties::<T>::append(&who, Some(&kitty));
            KittyToOwner::<T>::insert(kitty.id, &who);

            Self::deposit_event(Event::KittyCreated(nonce, who));

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        fn encode_and_update_nonce() -> Result<u64, Error<T>> {
            let current_nonce = <Nonce<T>>::get().unwrap_or(0);

            // return an error to fail the transaction if we can't increment the nonce
            match current_nonce.checked_add(1) {
                Some(nonce) => Ok(nonce),
                None => Err(Error::NonceFailed),
            }
        }
    }
}
