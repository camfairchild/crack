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
) -> Optional[bytes]:
    pass

def loop_over_replaced_words(
    dictionary: list[str],
    mnemonic: list[str],
    indices: list[int],
    target: bytes, # public key bytes
) -> Optional[list[str]]:
    pass

def to_pub_key_bytes(ss58_address: str) -> bytes:
    pass