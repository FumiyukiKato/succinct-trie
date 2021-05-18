use crate::config::*;
use crate::louds_dense::LoudsDense;
use crate::louds_sparse::LoudsSparse;
use crate::builder;

pub struct Trie {
    louds_dense: LoudsDense,
    louds_sparse: LoudsSparse,
    suffixes: Vec<Suffix>,
}

// 生ポインタを使えばもっと速くなる
// ベクタofベクタだとキャッシュにも乗らない
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Suffix {
    contents: Vec<u8>,
}

impl Trie {
    pub fn new(keys: &Vec<&fsa_key_t>) -> Self {
        let include_dense = K_INCLUDE_DENSE;
        let sparse_dense = K_SPARSE_DENSE_RATIO;

        let mut builder = builder::Builder::new(include_dense, sparse_dense);
        builder.build(&keys);
        let louds_dense = LoudsDense::new(&builder);
        let louds_sparse = LoudsSparse::new(&builder);

        let mut num_keys = 0;
        for level in 0..louds_sparse.get_height() {
            num_keys += builder.get_suffix_counts()[level];
        }

        let mut suffix_builder: Vec<Suffix> = vec![
            Suffix {
                contents: Vec::new(),
            };
            num_keys
        ];
        for i in 0..keys.len() {
            if i != 0 && keys[i] == keys[i - 1] {
                continue;
            }

            let (key_id, level) = Trie::traverse(&louds_dense, &louds_sparse, keys[i]);
            assert!(key_id < num_keys);
            let contents = keys[i][level..].to_vec();
            suffix_builder[key_id] = Suffix { contents };
        }
        // suffix_builder.sort();
        // let mut suffix_ptrs: Vec<usize> = vec![0; num_keys];
        // let mut suffixes = vec![];
        // let mut prev_suffix = Suffix {
        //     contents: Vec::new(),
        //     key_id: kNotFound,
        // };

        // for i in 0..num_keys {
        //     let curr_suffix = suffix_builder[num_keys - i - 1];
        //     if curr_suffix.contents.len() == 0 {
        //         suffix_ptrs[curr_suffix.key_id] = 0;
        //         continue;
        //     }
        //     let mut num_match = 0;
        //     while num_match < curr_suffix.contents.len()
        //         && num_match < prev_suffix.contents.len()
        //         && prev_suffix.contents[num_match] == curr_suffix.contents[num_match]
        //     {
        //         num_match += 1;
        //     }

        //     if num_match == curr_suffix.contents.len() && prev_suffix.contents.len() != 0 {
        //         suffix_ptrs[curr_suffix.key_id] = suffix_ptrs[prev_suffix.key_id] + (prev_suffix.contents.len() - num_match)
        //     } else {
        //         suffix_ptrs[curr_suffix.key_id] = suffixes.len();
        //         suffixes.push(curr_suffix);
        //     }
        //     prev_suffix = curr_suffix;
        // }

        // let mut suf_bits = 0;
        // let mut max_ptr = suffixes.len();

        // suf_bits += 1;
        // max_ptr >>= 1;
        // while max_ptr != 0 {
        //     suf_bits += 1;
        //     max_ptr >>= 1;
        // }
        // let suffix_ptrs = 

        return Trie {
            louds_dense,
            louds_sparse,
            suffixes: suffix_builder,
        }
    }

    fn traverse(
        louds_dense: &LoudsDense,
        louds_sparse: &LoudsSparse,
        key: &fsa_key_t,
    ) -> (position_t, level_t) {
        let ret = louds_dense.find_key(key);
        if ret.0 != K_NOT_FOUND {
            return (ret.0, ret.1);
        }
        if ret.2 != K_NOT_FOUND {
            return louds_sparse.find_key(key, ret.2);
        }
        return (ret.0, ret.1);
    }

    pub fn exact_search(&self, key: &fsa_key_t) -> position_t {
        let (key_id, level) = Trie::traverse(&self.louds_dense, &self.louds_sparse, key);
        if key_id == K_NOT_FOUND {
            return K_NOT_FOUND
        }

        let suffix = &self.suffixes[key_id].contents;
        let length = key.len() - level;
        if length != suffix.len() {
            return K_NOT_FOUND
        }
        for (cur_key, cur_suf) in key[level..].iter().zip(suffix.iter()) {
            if cur_key != cur_suf {
                return K_NOT_FOUND
            }
        }
        return key_id
    }
}
