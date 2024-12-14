import init, {
  add,
  http_get,
  div,
  genkey,
  falcon_genkey,
  WasmFavoriteNumbers,
  wasm_user_object_from_record,
  no_op,
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

function timeIt(name, func, iterations) {
  const startTime = performance.now();
  let result;
  result = func();

  const endTime = performance.now();
  const totalTime = endTime - startTime;

  console.log(
    `${name.padEnd(35)} ${
      totalTime / iterations
    } ms/iter (${totalTime} ms/${iterations})`
  );
  return result;
}

function nativeNoOp() {
  // No-op
}

function bench() {
  // Anything larger than 10_000 will cause JavaScript impl to crash/leak
  const iterations = 10_000;
  const userIterations = 200;
  const wasmFavoriteNumbers = new WasmFavoriteNumbers();
  const nativeFavoriteNumbers = new NativeFavoriteNumbers();

  const colors = [
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
  ];

  const userRecords = [];
  const randomFavoriteNumbers = [];
  const randomFavoriteColors = [];

  for (let i = 0; i < userIterations; i++) {
    for (let j = 0; j < 100; j++) {
      randomFavoriteNumbers.push(
        BigInt(Math.floor(Math.random() * iterations))
      );
    }
    for (let j = 0; j < 5; j++) {
      randomFavoriteColors.push(
        colors[Math.floor(Math.random() * colors.length)]
      );
    }
    userRecords.push({
      id: BigInt(i),
      favorite_numbers: randomFavoriteNumbers,
      favorite_colors: randomFavoriteColors,
    });
  }

  console.log("Starting benchmarks...");

  const userObjects = timeIt(
    "JS Object to WASM struct",
    () => {
      return userRecords.map((record) => wasm_user_object_from_record(record));
    },
    userIterations
  );

  timeIt(
    "WASM struct to JS object",
    () => {
      return userObjects.map((user) => user.to_record());
    },
    userIterations
  );

  timeIt(
    "push (native)",
    () => {
      for (let i = 0; i < iterations; i++) {
        nativeFavoriteNumbers.add_number(BigInt(i));
      }
    },
    iterations
  );

  timeIt(
    "push (WASM)",
    () => {
      for (let i = 0; i < iterations; i++) {
        wasmFavoriteNumbers.add_number(BigInt(i));
      }
    },
    iterations
  );

  timeIt(
    "find_min (native)",
    () => {
      nativeFavoriteNumbers.find_min();
    },
    1
  );

  timeIt(
    "find_min (WASM)",
    () => {
      wasmFavoriteNumbers.find_min();
    },
    1
  );

  timeIt(
    "quick_sort (native)",
    () => {
      nativeFavoriteNumbers.quick_sort();
    },
    1
  );

  timeIt(
    "quick_sort (WASM)",
    () => {
      wasmFavoriteNumbers.quick_sort();
    },
    1
  );

  const noOpIterations = 1_000_000_000;

  timeIt(
    "no_op",
    () => {
      for (let i = 0; i < noOpIterations; i++) {
        no_op();
      }
    },
    noOpIterations
  );

  timeIt(
    "no_op (native)",
    () => {
      for (let i = 0; i < noOpIterations; i++) {
        nativeNoOp();
      }
    },
    noOpIterations
  );
}

async function demo() {
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

export async function main() {
  const startTime = performance.now();
  await init();
  const endTime = performance.now();
  const totalTime = endTime - startTime;
  console.log(`Initialization time: ${totalTime} ms`);
  await demo();
  bench();
}
