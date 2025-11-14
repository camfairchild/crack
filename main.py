from typing import List, Optional

import concurrent.futures
import math
import bt_crack
import math
import substrateinterface
from tqdm import tqdm

first: list[str] = """
""".replace("\n", " ").split(" ")[1:-1]
target = "5DS8yTBRZJ8DRCkosBss2ELfSDWBy2ZkWmjMLYciswZX3Rad"

DICT = [
    "node-subtensor",
    "tao",
    "Tao",
    "TAO",
    "bittensor",
    "Bittensor",
    "wallet",
    "Wallet",
    "WALLET",
    "node",
    "NODE",
    "sub",
    "SUB",
    "subtensor",
    "Subtensor",
    "SUBTENSOR",
    "nodesubtensor",
    "hardware",
    "Hardware",
    "HARDWARE",
    "Path",
    "path",
    "PATH",
    "bt",
    "bit",
    "BT",
]

with open("words.txt", "r") as f:
    WORDS = f.read().split("\n")

target_as_bytes = substrateinterface.keypair.Keypair(ss58_address=target).public_key
target_as_bytes, len(target_as_bytes)

BATCH_SIZE = 100
outer_batch = 100
MAX_WORKERS = 4 * 2

START = 0 * BATCH_SIZE

num_batches = math.factorial(len(first))


# def run_batch(START: int, BATCH_SIZE: int) -> Optional[str]:
#     result = bt_crack.crack([], first, target_as_bytes, START, BATCH_SIZE)
#     if result is not None:
#         return result

#     return None

def run_batch(dictionary: List[str], START: int, BATCH_SIZE: int, DERIVE_LENGTH: int) -> Optional[str]:
    result = bt_crack.derive(dictionary, first, target_as_bytes, START, BATCH_SIZE, DERIVE_LENGTH)
    if result is not None:
        return result

    return None

def run_(dictionary: List[str], x, DERIVE_LENGTH: int):
    return run_batch(dictionary, x, BATCH_SIZE, DERIVE_LENGTH)


# def main():
#     batches = iter(range(START, num_batches, BATCH_SIZE))
#     len_batches = (num_batches - START) / BATCH_SIZE
#     outer_batches = math.ceil((num_batches - START) / (outer_batch * BATCH_SIZE))
#     dictionary = DICT

#     with tqdm(total=len_batches) as pbar:
#         with concurrent.futures.ProcessPoolExecutor(
#             max_workers=MAX_WORKERS
#         ) as executor:
#             for length in range(1, 10):
#                 for _ in range(outer_batches):
#                     this_batch = [next(batches) for _ in range(outer_batch)]
#                     futures = {executor.submit(run_, dictionary, batch, length): batch for batch in this_batch}

#                     for future in concurrent.futures.as_completed(futures):
#                         result = future.result()
#                         if result is not None:
#                             print(f"Found result: {result}")
#                             raise Exception("Found result")
#                             return
#                         pbar.update(1)

def main():
    k = 2
    result = bt_crack.try_k_replacements(
        WORDS, first, target_as_bytes, k
    )
    if result is not None:
        print(result)
    else:
        print("No result found")


if __name__ == "__main__":
    main()
