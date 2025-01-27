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
    });

    let wasm = build_solidity(
        r#"
        import "soroban";


        contract counter {
            /// Variable to track the count. Stored in persistent storage
            uint64 public persistent count = 11;

            /// Set value of count to 0
            function reset() public {
                count += 1;
            }

            /// Extends the TTL for the `count` persistent key to 5000 ledgers
            /// if the current TTL is smaller than 1000 ledgers
            function extend() public {
                // Encode the key into a `bytes` format for the built-in function
                bytes memory encodedKey = abi.encode("count");

                extendPersistentTtl(encodedKey, 1000, 5000);
            }
        }"#,
    );

    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&env.env);
    let address = env.register_contract(wasm, constructor_args);

    // initial TTL
    env.env.as_contract(&address, || {
        let key = env.env.storage().persistent().all().keys().first().unwrap();
        // FIXME: we still have to figure out how to encode the key so for now
        //        we will just use the key directly
        assert_eq!(env.env.storage().persistent().get_ttl(&key), 499);
    });

    // FIXME: This is getting stuck?
    env.invoke_contract(&address, "reset", vec![]);

    use soroban_sdk::IntoVal;
    env.env.as_contract(&address, || {
        let pres_storage = env.env.storage().persistent().all();
        println!("Pres storage: {:?}", pres_storage);

        let key = env.env.storage().persistent().all().keys().first().unwrap();
        // Get value
        let res: Val = env.env.storage().persistent().get(&key).unwrap();
        let expected: Val = 11_u64.into_val(&env.env);

        assert!(
            expected.shallow_eq(&res),
            "expected: {:?}, got: {:?}",
            expected,
            res
        );
    });

    // env.invoke_contract(&address, "reset", vec![]);

    // // env.invoke_contract(&address, "extend", vec![]);

    // // TTL should now be updated to 5000
    // env.env.as_contract(&address, || {
    //     // print all keys
    //     let keys = env.env.storage().persistent().all().keys();
    //     for key in keys {
    //         println!("key: {:?}", key);
    //     }

    //     let key = env.env.storage().persistent().all().keys().first().unwrap();
    //     env.env.storage().persistent().extend_ttl(&key, 1000, 5000);
    //     assert_eq!(env.env.storage().persistent().get_ttl(&key), 5000);
    // });

    // env.env.ledger().with_mut(|li| {
    //     li.sequence_number += 5000;
    // });

    // // TTL should now be reduced to 0
    // env.env.as_contract(&address, || {
    //     let key = env.env.storage().persistent().all().keys().first().unwrap();
    //     assert_eq!(env.env.storage().persistent().get_ttl(&key), 0);
    // });

    // // Extend the TTL again
    // env.invoke_contract(&address, "extend", vec![]);

    // // TTL is updated back to 5000
    // env.env.as_contract(&address, || {
    //     let key = env.env.storage().persistent().all().keys().first().unwrap();
    //     assert_eq!(env.env.storage().persistent().get_ttl(&key), 5000);
    // });
}
