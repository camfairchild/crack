use codec::{Decode, Encode};
use itertools::Itertools;
use sp_core::{sr25519, Pair};
use std::iter::once;

use pyo3::prelude::*;

// Implements ToPyObject for Compact<T> where T is an unsigned integer.
macro_rules! impl_UnsignedCompactIntoPy {
    ( $($type:ty),* $(,)? ) => {
        $(
            impl IntoPy<PyObject> for Compact<$type> {
                fn into_py(self, py: Python<'_>) -> PyObject {
                    let value: $type = self.0.into();

                    value.into_py(py)
                }
            }
        )*
    };
}

#[derive(Clone, Encode, Decode, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Compact<T>(pub codec::Compact<T>);
impl_UnsignedCompactIntoPy!(u8, u16, u32, u64, u128);

#[derive(Clone, Debug)]
pub struct Permutations {
    inner: permutations::Permutations,
}
impl Permutations {
    pub fn new(n: usize) -> Permutations {
        Permutations {
            inner: permutations::Permutations::new(n),
        }
    }
    /// Returns the permutation at a given index.
    pub fn get(&self, index: usize) -> Option<Permutation> {
        self.inner.get(index).map(|inner| Permutation { inner })
    }
    /// Returns the number of permutations in the sequence.
    ///
    /// The sequence is never empty.
    /// Even for `n = 0` it contains the empty permutation.
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> Iter {
        Iter::new(self.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Permutation {
    inner: permutations::Permutation,
}

impl IntoIterator for Permutations {
    type Item = Permutation;
    type IntoIter = Iter;
    fn into_iter(self) -> Iter {
        Iter::new(self)
    }
}
impl<'a> IntoIterator for &'a Permutations {
    type Item = Permutation;
    type IntoIter = Iter;
    fn into_iter(self) -> Iter {
        self.iter()
    }
}
#[derive(Clone, Debug)]
pub struct Iter {
    permutations: Permutations,
    next_index: usize,
}
impl Iter {
    fn new(permutations: Permutations) -> Iter {
        let next_index = 0;
        Iter {
            permutations,
            next_index,
        }
    }
}
impl Iterator for Iter {
    type Item = Permutation;
    fn next(&mut self) -> Option<Permutation> {
        if let Some(result) = self.permutations.get(self.next_index) {
            self.next_index += 1;
            Some(result)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.permutations.inner.iter().size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.next_index += n;
        self.next()
    }
}
impl ExactSizeIterator for Iter {}

#[pymodule(name = "bt_crack")]
mod bt_crack {
    use indicatif::ProgressBar;
    use sp_core::crypto::Ss58Codec;

    use super::*;

    const MNEMONIC_SIZE: usize = 12;
    const DICT_SIZE: usize = 2048;

    fn to_pub_key(mnemonic: &[&str; MNEMONIC_SIZE]) -> Option<sr25519::Public> {
        let seed = mnemonic.join(" ");
        let maybe_pair = sr25519::Pair::from_string(&seed, None);
        let pair = match maybe_pair {
            Ok(pair) => pair,
            Err(_) => return None,
        };
        let public = pair.public();

        Some(public)
    }

    #[allow(dead_code)]
    fn to_ss58_addr(mnemonic: &[&str; MNEMONIC_SIZE]) -> Option<String> {
        let public = to_pub_key(mnemonic)?;

        Some(public.to_ss58check())
    }

    #[pyfunction(name = "crack")]
    pub fn py_crack(
        _dictionary: [String; DICT_SIZE],
        mnemonic: [String; MNEMONIC_SIZE],
        target: [u8; 32],
        start: u128,
        batch_size: u128,
    ) -> PyResult<Option<Vec<String>>> {
        let mnemonic: &[&str; MNEMONIC_SIZE] = &mnemonic
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        let perms = Permutations::new(mnemonic.len());

        for perm in perms.iter().skip(start as usize).take(batch_size as usize) {
            let combo = perm.inner.permute(mnemonic).try_into().unwrap();
            let pub_key = to_pub_key(&combo);
            if pub_key.is_some_and(|x| x.0 == target) {
                return Ok(Some(
                    combo
                        .iter()
                        .map(|&x| x.to_string())
                        .collect::<Vec<String>>(),
                ));
            }
        }

        Ok(None)
    }

    #[pyfunction(name = "try_pair_permutations")]
    pub fn py_try_pair_permutations(
        mnemonic: [String; MNEMONIC_SIZE],
        target: [u8; 32],
        start: u128,
        batch_size: u128,
    ) -> PyResult<Option<Vec<String>>> {
        let mnemonic: [&str; MNEMONIC_SIZE] = mnemonic
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let indices = (1..mnemonic.len()).collect::<Vec<usize>>();

        for perm in indices
            .iter()
            .permutations(2)
            .skip(start as usize)
            .take(batch_size as usize)
        {
            let mut combo = mnemonic;
            let (first, second) = (*perm[0], *perm[1]);
            combo.swap(first, second);

            let pub_key = to_pub_key(&combo);
            if pub_key.is_some_and(|x| x.0 == target) {
                return Ok(Some(
                    combo
                        .iter()
                        .map(|&x| x.to_string())
                        .collect::<Vec<String>>(),
                ));
            }
        }

        Ok(None)
    }

    fn loop_over_replaced_word<'a>(
        dictionary: &[&'a str; 2048],
        mnemonic: &[&'a str; MNEMONIC_SIZE],
        &index: &usize,
        target: [u8; 32],
        pb: Option<&ProgressBar>,
    ) -> Option<Vec<&'a str>> {
        let replaced_word = mnemonic[index];
        let left = &mnemonic[..index];
        let right = &mnemonic[index + 1..];

        for &word in dictionary {
            if pb.is_some() {
                pb?.inc(DICT_SIZE as u64);
            }
            if *word == *replaced_word {
                continue;
            }

            let mut joined = left.iter().chain(once(&word)).chain(right.iter());
            let joined = std::array::from_fn(|_| *joined.next().unwrap());
            let pub_key = to_pub_key(&joined);
            if pub_key.is_some_and(|x| x.0 == target) {
                return Some(joined.to_vec());
            }
        }

        None
    }

    fn loop_over_replaced_words<'a>(
        dictionary: &[&'a str; 2048],
        mnemonic: &[&'a str; MNEMONIC_SIZE],
        indices: &[&usize],
        target: [u8; 32],
        pb: Option<&ProgressBar>,
    ) -> Option<Vec<&'a str>> {
        if indices.len() == 1 {
            return loop_over_replaced_word(dictionary, mnemonic, indices[0], target, pb);
        }

        let &next_ind = indices[0];
        let new_indices = &indices[1..];

        let &replaced_word = &mnemonic[next_ind];

        let left = &mnemonic[..next_ind];
        let right = &mnemonic[next_ind + 1..];

        for &word in dictionary {
            if *word == *replaced_word {
                continue;
            }

            let mut joined = left.iter().chain(once(&word)).chain(right.iter());
            let joined = std::array::from_fn(|_| *joined.next().unwrap());
            let result = loop_over_replaced_words(dictionary, &joined, new_indices, target, pb);
            if result.is_some() {
                return result;
            }
        }

        None
    }

    fn factorial(n: u128) -> u128 {
        (1..=n).product()
    }

    fn count_combinations(n: u128, r: u128) -> u128 {
        factorial(n) / (factorial(r) * factorial(n - r))
    }

    #[pyfunction(name = "try_k_replacements")]
    pub fn py_try_k_replacements(
        dictionary: Vec<String>,
        mnemonic: Vec<String>,
        target: [u8; 32],
        k: usize,
    ) -> PyResult<Option<Vec<String>>> {
        let indices = (1..mnemonic.len()).collect::<Vec<usize>>();
        let dictionary = dictionary
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let mnemonic = mnemonic
            .iter()
            .map(|x| x.as_str())
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();

        let combos = count_combinations(MNEMONIC_SIZE as u128, k as u128)
            * u128::pow(DICT_SIZE as u128, k as u32);
        let pb = ProgressBar::new(combos as u64);
        for comb in indices.iter().combinations(k) {
            let result = loop_over_replaced_words(&dictionary, &mnemonic, &comb, target, Some(&pb));
            match result {
                Some(output) => {
                    return Ok(Some(
                        output
                            .iter()
                            .map(|&x| x.to_string())
                            .collect::<Vec<String>>(),
                    ))
                }
                _ => continue,
            }
        }

        Ok(None)
    }
}
