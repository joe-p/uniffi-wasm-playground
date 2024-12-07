import init, {
  add,
  http_get,
  div,
  genkey,
  falcon_genkey,
  WasmFavoriteNumbers,
} from "./pkg/playground.js";

const MAX_U64 = BigInt("18446744073709551615");

class NativeFavoriteNumbers {
  numbers = [];
  max_number = 0;

  add_number(number) {
    this.numbers.push(number);
    this.max_number = this.max_number > number ? this.max_number : number;
  }

  find_min() {
    if (this.numbers.length === 0) return null;
    return this.numbers.reduce((min, current) =>
      current < min ? current : min
    );
  }

  quick_sort(numbers = null) {
    if (numbers === null) {
      numbers = [...this.numbers];
    }

    if (numbers.length <= 1) {
      return numbers;
    }

    const pivot = numbers[numbers.length - 1];
    const less = numbers.slice(0, -1).filter((x) => x <= pivot);
    const greater = numbers.slice(0, -1).filter((x) => x > pivot);

    return [...this.quick_sort(less), pivot, ...this.quick_sort(greater)];
  }
}

function bench() {
  // Anything larger than 10_000 will cause JavaScript impl to crash/leak
  const iterations = 10_000;
  const wasmFavoriteNumbers = new WasmFavoriteNumbers();
  const nativeFavoriteNumbers = new NativeFavoriteNumbers();

  let startTime = performance.now();
  for (let i = 0; i < iterations; i++) {
    nativeFavoriteNumbers.add_number(BigInt(i));
  }
  let endTime = performance.now();
  console.log(`JS push time taken: ${endTime - startTime} milliseconds`);

  startTime = performance.now();
  for (let i = 0; i < iterations; i++) {
    wasmFavoriteNumbers.add_number(BigInt(i));
  }
  endTime = performance.now();
  console.log(`WASM push time taken: ${endTime - startTime} milliseconds`);

  startTime = performance.now();
  nativeFavoriteNumbers.find_min();
  endTime = performance.now();
  console.log(`JS find_min time taken: ${endTime - startTime} milliseconds`);

  startTime = performance.now();
  wasmFavoriteNumbers.find_min();
  endTime = performance.now();
  console.log(`WASM find_min time taken: ${endTime - startTime} milliseconds`);

  startTime = performance.now();
  nativeFavoriteNumbers.quick_sort();
  endTime = performance.now();
  console.log(`JS quick_sort time taken: ${endTime - startTime} milliseconds`);

  startTime = performance.now();
  wasmFavoriteNumbers.quick_sort();
  endTime = performance.now();
  console.log(
    `WASM quick_sort time taken: ${endTime - startTime} milliseconds`
  );
}

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

  bench();

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
