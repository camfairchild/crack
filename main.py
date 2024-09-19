from typing import List, Optional

import concurrent.futures
import math
import bt_crack
import math
import substrateinterface
from tqdm import tqdm

first: List[str] = "".split(" ")
target = "5DS8yTBRZJ8DRCkosBss2ELfSDWBy2ZkWmjMLYciswZX3Rad"

target_as_bytes = substrateinterface.keypair.Keypair(ss58_address=target).public_key
target_as_bytes, len(target_as_bytes)

BATCH_SIZE = 100
outer_batch = 100
MAX_WORKERS = 64 * 2

START = 0 * BATCH_SIZE

num_batches = math.factorial(len(first))


def run_batch(START: int, BATCH_SIZE: int) -> Optional[str]:
    result = bt_crack.crack([], first, target_as_bytes, START, BATCH_SIZE)
    if result is not None:
        return result

    return None


def run_(x):
    return run_batch(x, BATCH_SIZE)


def main():
    batches = iter(range(START, num_batches, BATCH_SIZE))
    len_batches = (num_batches - START) / BATCH_SIZE
    outer_batches = math.ceil((num_batches - START) / (outer_batch * BATCH_SIZE))

    with tqdm(total=len_batches) as pbar:
        with concurrent.futures.ProcessPoolExecutor(
            max_workers=MAX_WORKERS
        ) as executor:
            for _ in range(outer_batches):
                this_batch = [next(batches) for _ in range(outer_batch)]
                futures = {executor.submit(run_, batch): batch for batch in this_batch}

                for future in concurrent.futures.as_completed(futures):
                    result = future.result()
                    if result is not None:
                        print(result)
                        return
                    pbar.update(1)


if __name__ == "__main__":
    main()
