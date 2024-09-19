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

batch_size = 100
outer_batch = 100

start = 0 * batch_size

num_batches = math.factorial(len(first))

def run_batch(start: int, batch_size: int) -> Optional[str]:
    result = bt_crack.crack([], first, target_as_bytes, start, batch_size)
    if result is not None:
        return result
    
    return None

def run_(x):
    return run_batch(x, batch_size)

def main():
    batches = iter(range(start, num_batches, batch_size))
    len_batches = (num_batches - start) / batch_size
    outer_batches = math.ceil((num_batches - start) / (outer_batch * batch_size))

    
    with tqdm(total=len_batches) as pbar:
        with concurrent.futures.ProcessPoolExecutor(max_workers=64*2) as executor:    
            for _ in range(outer_batches):
                this_batch = [next(batches) for _ in range(outer_batch)]
                futures = {executor.submit(run_, batch): batch for batch in this_batch}

                for future in concurrent.futures.as_completed(futures):
                    result = future.result()
                    if result is not None:
                        print(result)
                        return
                    pbar.update(1)

if __name__ == '__main__':
    main()