#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate eth_pairings;
extern crate eth_pairings_go_2537;
extern crate hex;


fuzz_target!(|data: &[u8]| {
    if data.len() < 1 {
        return;
    }

    use eth_pairings::public_interface::eip2537::EIP2537Executor;
    use eth_pairings_go_2537::{OperationType, perform_operation};
    
    let op = OperationType::from_u8(data[0]);
    if op.is_none() {
        return;
    }
    let op = op.unwrap();

    let true_input = &data[1..];
    let native_result = match op {
        OperationType::G1ADD => {
            EIP2537Executor::g1_add(true_input).map(|el| el[..].to_vec())
        },
        OperationType::G1MUL => {
            EIP2537Executor::g1_mul(true_input).map(|el| el[..].to_vec())
        },
        OperationType::G1MULTIEXP => {
            EIP2537Executor::g1_multiexp(true_input).map(|el| el[..].to_vec())
        },
        OperationType::G2ADD => {
            EIP2537Executor::g2_add(true_input).map(|el| el[..].to_vec())
        },
        OperationType::G2MUL => {
            EIP2537Executor::g2_mul(true_input).map(|el| el[..].to_vec())
        },
        OperationType::G2MULTIEXP => {
            EIP2537Executor::g2_multiexp(true_input).map(|el| el[..].to_vec())
        },
        OperationType::PAIR => {
            EIP2537Executor::pair(true_input).map(|el| el[..].to_vec())
        },
        OperationType::MAPFPTOG1 => {
            EIP2537Executor::map_fp_to_g1(true_input).map(|el| el[..].to_vec())
        },
        OperationType::MAPFP2TOG2 => {
            EIP2537Executor::map_fp2_to_g2(true_input).map(|el| el[..].to_vec())
        },
    };

    let go_result = perform_operation(op, true_input);

    match (native_result, go_result) {
        (Ok(n), Ok(c)) => {
            if n != c {
                // println!("Native result = {}, C++ result = {}", hex::encode(&n), hex::encode(&c));
                panic!("Native result = {}, Go result = {}", hex::encode(&n), hex::encode(&c));
            } else {
                // println!("Native and C++ results coincide on {}", hex::encode(&n));
            }
        },
        (Err(_n), Err(_c)) => {
            return;
            // println!("Native and C++ results coincide on error: {:?}, {:?}", n, c);
        },
        (Ok(n), Err(c)) => {
            // println!("Input = {}", hex::encode(&data));
            // println!("Native result = {}, while C++ returned error {:?}", hex::encode(&n), c);
            panic!("Native result = {}, while Go returned error {:?}", hex::encode(&n), c);
        },
        (Err(n), Ok(c)) => {
            // println!("Input = {}", hex::encode(&data));
            // println!("Native result returned error {:?}, while C++ returned {}", n, hex::encode(&c));
            panic!("Native result returned error {:?}, while Go returned {}", n, hex::encode(&c));
        }
    }
});
