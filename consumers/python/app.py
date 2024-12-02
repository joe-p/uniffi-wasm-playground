from arithmetic import (
    http_get,
    genkey,
    add,
    div,
    ArithmeticError,
    InternalError,
)
import asyncio
import json

MAX_U64 = 18446744073709551615


async def main():
    # "Heavy" computation like key generation
    print("ed25519 key:", genkey())

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

    async def delay(n):
        await http_get(f"https://httpbin.org/delay/{n}")
        print(f"Delay {n} finished")

    delay_2 = delay(2)
    delay_1 = delay(1)

    await asyncio.gather(delay_2, delay_1)


if __name__ == "__main__":
    asyncio.run(main())
