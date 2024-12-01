from arithmetic import add, div, say_after, ArithmeticError, InternalError
import asyncio

MAX_U64 = 18446744073709551615


async def main():
    print(await say_after(50, "Alice"))
    bob = say_after(100, "Bob")
    chuck = say_after(50, "Chuck")
    await asyncio.gather(bob, chuck)


if __name__ == "__main__":
    try:
        div(1, 0)
    except InternalError as e:
        print(f"We caught a panic! {e.__class__.__name__}: {e}")

    try:
        add(MAX_U64, 1)
    except ArithmeticError.IntegerOverflow as e:
        print(f"We caught an error! {e.__class__.__name__}: {e}")

    try:
        add(MAX_U64 + 1, 0)
    except ValueError as e:
        print(f"We caught an error thrown by the binding! {e.__class__.__name__}: {e}")

    asyncio.run(main())
