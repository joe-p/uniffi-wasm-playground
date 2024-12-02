import init, { add, http_get, div, genkey } from "./pkg/arithmetical.js";

const MAX_U64 = BigInt("18446744073709551615");

export async function main() {
  await init();

  // "Heavy" computation like key generation
  print("ed25519 key:", genkey());

  // Error handling
  try {
    div(1n, 0n);
  } catch (e) {
    console.log(`We caught a panic! ${e.name}: ${e.message}`);
  }

  try {
    const sum = add(MAX_U64 + 1n, 0n);
    console.log(
      `There was no error thrown by the binding! ${
        MAX_U64 + 1n
      } + ${0n} -- ${sum}`
    );
  } catch (e) {
    console.log(
      `We caught an error thrown by the binding! ${e.name}: ${e.message}`
    );
  }

  try {
    add(MAX_U64, 1n);
  } catch (e) {
    console.log(`We caught an error! ${e.name}: ${e.message}`);
    console.log(`The error is just a string: ${e}`);
  }

  // Async HTTP requests
  //     status = http_get("https://testnet-api.4160.nodely.dev/v2/status")
  //     last_round = json.loads(await status)["last-round"]
  //     print(f"Last round: {last_round}")

  //     async def delay(n):
  //         await http_get(f"https://httpbin.org/delay/{n}")
  //         print(f"Delay {n} finished")

  //     delay_2 = delay(2)
  //     delay_1 = delay(1)

  //     await asyncio.gather(delay_2, delay_1)
  const status = http_get("https://testnet-api.4160.nodely.dev/v2/status");
  const lastRound = JSON.parse(await status)["last-round"];
  console.log(`Last round: ${lastRound}`);

  const delay2 = http_get("https://httpbin.org/delay/2").then(() =>
    console.log("Delay 2 finished")
  );
  const delay1 = http_get("https://httpbin.org/delay/1").then(() =>
    console.log("Delay 1 finished")
  );

  await Promise.all([delay2, delay1]);
}
