from playground import (
    http_get,
    genkey,
    add,
    div,
    PlaygroundError,
    InternalError,
    falcon_genkey,
    FavoriteNumbers,
)
import asyncio
import json
import time
import random

MAX_U64 = 18446744073709551615


class NativeFavoriteNumbers:
    def __init__(self):
        self.numbers = []
        self.max_number = 0

    def add_number(self, number: int):
        self.numbers.append(number)
        self.max_number = max(self.max_number, number)

    def find_min(self):
        return min(self.numbers)

    def quick_sort(self, numbers: list[int] = None):
        if numbers is None:
            numbers = self.numbers

        # Base case: arrays with 0 or 1 element are already sorted
        if len(numbers) <= 1:
            return numbers

        # Choose a pivot (e.g., the last element)
        pivot = numbers[-1]
        # Partition the array
        less = [
            x for x in numbers[:-1] if x <= pivot
        ]  # Elements less than or equal to pivot
        greater = [x for x in numbers[:-1] if x > pivot]  # Elements greater than pivot
        # Recursively sort the partitions and combine with the pivot
        return self.quick_sort(less) + [pivot] + self.quick_sort(greater)


def bench():
    favorite_numbers = FavoriteNumbers()
    native_favorite_numbers = NativeFavoriteNumbers()
    iterations = 10_000

    # numbers is a list of random numbers
    numbers = [random.randint(0, iterations) for _ in range(iterations)]

    start = time.time()
    for n in numbers:
        native_favorite_numbers.add_number(n)
    end = time.time()
    print(f"Python push time taken: {end - start} seconds")

    start = time.time()
    for n in numbers:
        favorite_numbers.add_number(n)
    end = time.time()
    print(f"FFI push time taken: {end - start} seconds")

    start = time.time()
    for _ in range(iterations):
        native_favorite_numbers.find_min()
    end = time.time()
    print(f"Python find min time taken: {end - start} seconds")

    start = time.time()
    for _ in range(iterations):
        favorite_numbers.find_min()
    end = time.time()
    print(f"FFI find min time taken: {end - start} seconds")

    start = time.time()
    native_favorite_numbers.quick_sort()
    end = time.time()
    print(f"Python quicksort time taken: {end - start} seconds")

    start = time.time()
    favorite_numbers.quick_sort(None)
    end = time.time()
    print(f"FFI quicksort time taken: {end - start} seconds")


async def main():
    # "Heavy" computation like key generation
    print("ed25519 key:", genkey())

    # Py -> Rust -> C for falcon
    print("falcon key:", falcon_genkey(b"").public_key)

    # Error handling
    try:
        div(1, 0)
    except InternalError as e:
        print(f"We caught a panic! {e.__class__.__name__}: {e}")

    try:
        add(MAX_U64, 1)
    except PlaygroundError.IntegerOverflow as e:
        print(f"We caught an error! {e.__class__.__name__}: {e}")

    try:
        add(MAX_U64 + 1, 0)
    except ValueError as e:
        print(f"We caught an error thrown by the binding! {e.__class__.__name__}: {e}")

    bench()

    # Async HTTP requests
    status = http_get("https://testnet-api.4160.nodely.dev/v2/status")
    last_round = json.loads(await status)["last-round"]
    print(f"Last round: {last_round}")

    async def wait_for_round(n):
        await http_get(
            f"https://testnet-api.4160.nodely.dev/v2/status/wait-for-block-after/{n}"
        )
        print(f"Got to round {n}!")

    round_2 = wait_for_round(last_round + 2)
    round_1 = wait_for_round(last_round + 1)

    await asyncio.gather(round_2, round_1)


if __name__ == "__main__":
    asyncio.run(main())
