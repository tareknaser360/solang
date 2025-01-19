// SPDX-License-Identifier: Apache-2.0

use crate::{build_solidity, SorobanEnv};
use soroban_sdk::testutils::storage::Persistent;
use soroban_sdk::{testutils::Ledger, Val};

/// This test is adapted from
/// [Stellar Soroban Examples](https://github.com/stellar/soroban-examples/blob/f595fb5df06058ec0b9b829e9e4d0fe0513e0aa8/ttl).
///
/// It shows testing the TTL extension for persistent storage keys using the `extendPersistentTTL` built-in function
#[test]
fn ttl_basic() {
    // Create a new environment
    let mut env = SorobanEnv::new();

    env.env.ledger().with_mut(|li| {
        // Current ledger sequence - the TTL is the number of
        // ledgers from the `sequence_number` (exclusive) until
        // the last ledger sequence where entry is still considered
        // alive.
        li.sequence_number = 100_000;
        // Minimum TTL for persistent entries - new persistent (and instance)
        // entries will have this TTL when created.
        li.min_persistent_entry_ttl = 500;
        // Minimum TTL for temporary entries - new temporary
        // entries will have this TTL when created.
        li.min_temp_entry_ttl = 100;
        // Maximum TTL of any entry. Note, that entries can have their TTL
        // extended indefinitely, but each extension can be at most
        // `max_entry_ttl` ledger from the current `sequence_number`.
        li.max_entry_ttl = 15000;
    });

    let wasm = build_solidity(
        r#"contract counter {
            /// Variable to track the count. Stored in persistent storage
            uint64 public persistent count = 11;


            /// Extends the TTL for the `count` persistent key to 5000 ledgers
            /// if the current TTL is smaller than 1000 ledgers
            function extend() public {
                // Encode the key into a `bytes` format for the built-in function
                bytes memory encodedKey = abi.encode("count");

                extendPersistentTTL(encodedKey, 1000, 5000);
            }
        }"#,
    );

    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&env.env);
    let address = env.register_contract(wasm, constructor_args);

    env.env.as_contract(&address, || {
        let key = env.env.storage().persistent().all().keys().first().unwrap();
        // FIXME: we still have to figure out how to encode the key so for now
        //        we will just use the key directly
        assert_eq!(env.env.storage().persistent().get_ttl(&key), 499);
    });
}
