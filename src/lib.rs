// TODO; sgx mode
// #![cfg_attr(not(target_env = "sgx"), no_std)]
// #[macro_use]
// extern crate sgx_tstd as std;
#![feature(core_intrinsics)]
#[warn(non_camel_case_types)]

mod bitvector;
mod builder;
pub mod config;
mod label_vector;
mod louds_dense;
mod louds_sparse;
mod popcount;
mod rank;
mod select;
mod suffix;
pub mod trie;

#[cfg(test)]
mod tests {
    use crate::{config::K_NOT_FOUND, trie::Trie};

    #[test]
    fn contains_check() {
        let a = vec![48, 49];
        let b = vec![49, 49];
        let c = vec![49, 50, 54];
        let d = vec![50, 50, 54, 55, 56, 57];
        let keys: Vec<&[u8]> = vec![a.as_slice(), b.as_slice(), c.as_slice(), d.as_slice()];
        let fsa = Trie::new(&keys);
        for key in keys.iter() {
            let key_id = fsa.exact_search(key);
            assert_ne!(key_id, K_NOT_FOUND);
        }

        let not_exist_item_a = vec![48, 49, 50];
        let not_exist_item_b = vec![100, 55, 0];
        let not_exist_item_c = vec![0, 0, 0, 0, 0, 0, 0];
        let not_exist_item_d = vec![255, 255, 255, 255, 255, 255, 255];
        let not_exist_keys: Vec<&[u8]> = vec![
            not_exist_item_a.as_slice(),
            not_exist_item_b.as_slice(),
            not_exist_item_c.as_slice(),
            not_exist_item_d.as_slice(),
        ];
        for key in not_exist_keys.iter() {
            let key_id = fsa.exact_search(key);
            assert_eq!(key_id, K_NOT_FOUND);
        }
    }
}
