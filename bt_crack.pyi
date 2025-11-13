from typing import List, Optional

def crack(
    dictionary: List[str],
    mnemonic: List[str],
    target: bytes,
    start: int,
    batch_size: int,
) -> Optional[str]:
    pass

def try_pair_permutations(
    mnemonic: List[str], target: bytes, start: int, batch_size: int
) -> Optional[str]:
    pass

def try_k_replacements(
    dictionary: List[str],
    mnemonic: List[str],
    target: bytes,
    k: int
) -> Optional[str]:
    pass

def derive(
    dictionary: List[str],
    mnemonic: List[str],
    target: bytes,
    start: int,
    batch_size: int,
    derive_length: int
) -> Optional[str]:
    pass

def try_derive(
    mnemonic: List[str],
    derivation_path: str,
) -> Optional[str]:
    pass