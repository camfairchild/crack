use codec::{Decode, Encode};
use sp_core::{sr25519, Pair};
use itertools::Itertools;
use permutations;

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
    use sp_core::crypto::Ss58Codec;

    use super::*;

    fn to_pub_key(mnemonic: Vec<String>) -> Option<sr25519::Public> {
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
    fn to_ss58_addr(mnemonic: Vec<String>) -> Option<String> {
        let public = to_pub_key(mnemonic)?;

        Some(public.to_ss58check())
    }

    #[pyfunction(name = "crack")]
    pub fn py_crack(_dictionary: Vec<String>, mnemonic: Vec<String>, target: [u8; 32], start: u128, batch_size: u128) -> PyResult<Option<Vec<String>>> {
        let mut result = None;

        let perms = Permutations::new(mnemonic.len());

        for perm in perms.iter().skip(start as usize).take(batch_size as usize) {
            let combo = perm.inner.permute(&mnemonic);
            let pub_key = to_pub_key(combo.clone());
            if pub_key.clone().is_some_and(|x| x.0 == target) {
                result = Some(combo);
                break;
            }
        }

        Ok(result)
    }

    #[pyfunction(name = "try_pair_permutations")]
    pub fn py_try_pair_permutations(mnemonic: Vec<String>, target: [u8; 32], start: u128, batch_size: u128) -> PyResult<Option<Vec<String>>> {
        let mut result = None;
        let indices = (1..mnemonic.len()).collect::<Vec<usize>>();

        for perm in indices.iter().permutations(2).skip(start as usize).take(batch_size as usize) {
            let mut combo = mnemonic.clone();
            let (first, second) = (*perm[0], *perm[1]);
            let temp = combo[first].clone();
            combo[first] = combo[second].clone();
            combo[second] = temp;

            let pub_key = to_pub_key(combo.clone());
            if pub_key.clone().is_some_and(|x| x.0 == target) {
                result = Some(combo);
                break;
            }
        }

        Ok(result)
    }
}
