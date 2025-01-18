// SPDX-License-Identifier: Apache-2.0

use crate::{build_solidity, SorobanEnv};
use soroban_sdk::{IntoVal, Val};

#[test]
fn counter() {
    let wasm = build_solidity(
        r#"contract counter {
            uint64 public count = 10;

            function increment() public returns (uint64){
                count += 1;
                return count;
            }

            function decrement() public returns (uint64){
                count -= 1;
                return count;
            }
        }"#,
    );
    let mut src = SorobanEnv::new();
    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&src.env);
    let address = src.register_contract(wasm, constructor_args);

    let res = src.invoke_contract(&address, "count", vec![]);
    let expected: Val = 10_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    src.invoke_contract(&address, "increment", vec![]);
    let res = src.invoke_contract(&address, "count", vec![]);
    let expected: Val = 11_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    src.invoke_contract(&address, "decrement", vec![]);
    let res = src.invoke_contract(&address, "count", vec![]);
    let expected: Val = 10_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));
}

#[test]
fn different_storage_types() {
    let wasm = build_solidity(
        r#"contract sesa {
            
    uint64 public temporary sesa = 1;
    uint64 public instance sesa1 = 1;
    uint64 public persistent sesa2 = 2;
    uint64 public sesa3 = 2;
    function inc() public {
        sesa++;
        sesa1++;
        sesa2++;
        sesa3++;
    }
    function dec() public {
        sesa--;
        sesa1--;
        sesa2--;
        sesa3--;
    }
}"#,
    );

    let mut src = SorobanEnv::new();
    // No constructor arguments
    let constructor_args: soroban_sdk::Vec<Val> = soroban_sdk::Vec::new(&src.env);
    let addr = src.register_contract(wasm, constructor_args);

    let res = src.invoke_contract(&addr, "sesa", vec![]);
    let expected: Val = 1_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa1", vec![]);
    let expected: Val = 1_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa2", vec![]);
    let expected: Val = 2_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa3", vec![]);
    let expected: Val = 2_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    src.invoke_contract(&addr, "inc", vec![]);
    let res = src.invoke_contract(&addr, "sesa", vec![]);
    let expected: Val = 2_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa1", vec![]);
    let expected: Val = 2_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa2", vec![]);
    let expected: Val = 3_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa3", vec![]);
    let expected: Val = 3_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    src.invoke_contract(&addr, "dec", vec![]);
    let res = src.invoke_contract(&addr, "sesa", vec![]);
    let expected: Val = 1_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa1", vec![]);
    let expected: Val = 1_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa2", vec![]);
    let expected: Val = 2_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let res = src.invoke_contract(&addr, "sesa3", vec![]);
    let expected: Val = 2_u64.into_val(&src.env);
    assert!(expected.shallow_eq(&res));

    let diags = src.compiler_diagnostics;

    assert!(diags
        .contains_message("storage type not specified for `sesa3`, defaulting to `persistent`"));
}
