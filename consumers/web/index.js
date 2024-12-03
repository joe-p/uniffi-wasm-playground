import init, {
  add,
  http_get,
  div,
  genkey,
  falcon_genkey,
} from "./pkg/arithmetical.js";

const MAX_U64 = BigInt("18446744073709551615");

export async function main() {
  await init();

  // "Heavy" computation like key generation
  console.log("ed25519 key:", genkey());

  // C -> Rust -> WASM -> JS
  const keyPair = falcon_genkey(new Uint8Array());
  console.log("falcon keypair object", keyPair);
  console.log("falcon public key", keyPair.public_key);
  console.log("falcon private key", keyPair.private_key);

  // Error handling
  try {
    div(1n, 0n);
  } catch (e) {
    console.log(`We caught a panic! ${e.name}: ${e.message}`);
  }

  try {
    const sum = add(MAX_U64 + 100n, 0n);
    console.log(
      `There was no error thrown by the binding! ${
        MAX_U64 + 1n
      } + ${100n} == ${sum}`
    );
  } catch (e) {
    console.log(`We caught an error thrown by the binding! ${e}`);
  }

  try {
    add(MAX_U64, 1n);
  } catch (e) {
    console.log(`The error is just a string: ${e}`);
  }

  const status = http_get("https://testnet-api.4160.nodely.dev/v2/status");
  const lastRound = JSON.parse(await status)["last-round"];
  console.log(`Last round: ${lastRound}`);

  const round2 = http_get(
    `https://testnet-api.4160.nodely.dev/v2/status/wait-for-block-after/${
      lastRound + 2
    }`
  ).then(() => console.log(`Got to round ${lastRound + 2}`));

  const round1 = http_get(
    `https://testnet-api.4160.nodely.dev/v2/status/wait-for-block-after/${
      lastRound + 1
    }`
  ).then(() => console.log(`Got to round ${lastRound + 1}`));

  await Promise.all([round2, round1]);
}
