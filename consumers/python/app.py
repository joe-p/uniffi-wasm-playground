from arithmetic import add, div, say_after, http_get, ArithmeticError, InternalError
import asyncio
import json

MAX_U64 = 18446744073709551615


async def main():
    print(await say_after(50, "Alice"))
    bob = say_after(100, "Bob")
    chuck = say_after(50, "Chuck")
    await asyncio.gather(bob, chuck)
    status = http_get("https://testnet-api.4160.nodely.dev/v2/status")
    print("HERE!")
    last_round = json.loads(await status)["last-round"]
    print(f"Last round: {last_round}")

    round_5 = http_get(
        f"https://testnet-api.4160.nodely.dev/v2/status/wait-for-block-after/{last_round + 5}"
    )

    round_3 = http_get(
        f"https://testnet-api.4160.nodely.dev/v2/status/wait-for-block-after/{last_round + 3}"
    )

    round_1 = http_get(
        f"https://testnet-api.4160.nodely.dev/v2/status/wait-for-block-after/{last_round + 1}"
    )

    await asyncio.gather(round_1, round_3, round_5)


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
