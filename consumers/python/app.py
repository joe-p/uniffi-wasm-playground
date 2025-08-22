from playground import (
    AsyncAdder,
    call_async_adder,
    http_get,
    genkey,
    add,
    div,
    PlaygroundError,
    InternalError,
    falcon_genkey,
    FavoriteNumbers,
    UserRecord,
    user_object_from_record,
    no_op,
)
import asyncio
import json
import time
import random
import msgpack

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


def native_no_op():
    pass


def time_it(func, iterations):
    start = time.time()
    ret_val = func()
    end = time.time()

    seconds = end - start
    ms_per_iter = 1000 * (seconds) / iterations

    print(
        f"{func.__name__:<35} {ms_per_iter:.6f} ms/iter ({seconds:.6f} sec/{iterations})"
    )
    return ret_val


def bench():
    favorite_numbers = FavoriteNumbers()
    native_favorite_numbers = NativeFavoriteNumbers()
    iterations = 10_000

    colors = [
        "red",
        "blue",
        "green",
        "yellow",
        "purple",
        "orange",
        "pink",
        "brown",
        "gray",
        "black",
        "white",
        "teal",
        "navy",
        "maroon",
        "violet",
    ]
    # Generate fake users with random favorite numbers and colors
    user_records = [
        UserRecord(
            id=i,
            favorite_numbers=[random.randint(0, iterations) for _ in range(100)],
            favorite_colors=[random.choice(colors) for _ in range(5)],
        )
        for i in range(iterations)
    ]

    print("\nCalling a no-op function:")

    def no_op_ffi():
        for _ in range(iterations):
            no_op()

    time_it(no_op_ffi, iterations)

    def no_op_native():
        for _ in range(iterations):
            native_no_op()

    time_it(no_op_native, iterations)

    numbers = [random.randint(0, iterations) for _ in range(iterations)]

    print("\nPushing to an array of numbers in a class/struct:")

    def native_push():
        for n in numbers:
            native_favorite_numbers.add_number(n)

    time_it(native_push, iterations)

    def push_ffi():
        for n in numbers:
            favorite_numbers.add_number(n)

    time_it(push_ffi, iterations)

    print("\nFinding the min value in an array of numbers in a class/struct:")

    def find_min_native():
        for _ in range(iterations):
            native_favorite_numbers.find_min()

    time_it(find_min_native, iterations)

    def find_min_ffi():
        for _ in range(iterations):
            favorite_numbers.find_min()

    time_it(find_min_ffi, iterations)

    print("\nQuick sort algo on an array of numbers in a class/struct:")

    def quick_sort_native():
        return native_favorite_numbers.quick_sort()

    time_it(quick_sort_native, iterations)

    def quick_sort_ffi():
        return favorite_numbers.quick_sort(None)

    time_it(quick_sort_ffi, iterations)

    print("\nConverting between Python class and Rust struct:")

    def py_class_to_rust_struct():
        return [user_object_from_record(user_record) for user_record in user_records]

    user_objects = time_it(py_class_to_rust_struct, iterations)

    def rust_struct_to_py_class():
        return [user_object.to_record() for user_object in user_objects]

    user_records_from_objects = time_it(rust_struct_to_py_class, iterations)

    print(user_records[0].__dict__)
    print(user_objects[0])

    print("\nSerializing a class/struct to msgpack:")

    def serialize_rust_struct():
        return [user_object.serialize() for user_object in user_objects]

    time_it(serialize_rust_struct, iterations)

    def serialize_python_class():
        return [
            msgpack.packb(user_record.__dict__)
            for user_record in user_records_from_objects
        ]

    time_it(serialize_python_class, iterations)


class AsyncAdderImpl(AsyncAdder):
    async def add_async(self, a: int, b: int) -> int:
        return a + b


async def demo():
    res = await call_async_adder(AsyncAdderImpl(), 1, 2)
    assert res == 3, f"Expected 3, got {res}"
    print(f"Async adder result: {res}")

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


async def main():
    await demo()
    bench()


if __name__ == "__main__":
    asyncio.run(main())
