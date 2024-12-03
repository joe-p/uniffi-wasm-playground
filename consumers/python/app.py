from arithmetic import (
    http_get,
    genkey,
    add,
    div,
    ArithmeticError,
    InternalError,
    falcon_genkey,
)
import asyncio
import json

MAX_U64 = 18446744073709551615


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
    except ArithmeticError.IntegerOverflow as e:
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


if __name__ == "__main__":
    asyncio.run(main())
