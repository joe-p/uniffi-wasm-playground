import init, { add, say_after, http_get, div } from "./pkg/arithmetical.js";

const MAX_U64 = BigInt("18446744073709551615")

export async function main() {
    await init();
    console.log(add(1n, 2n))
    const promises = [
        say_after(1000n, "Alice"),
        say_after(500n, "Bob")
    ]
    promises.forEach(p => p.then(console.log))

    const result = await http_get("https://testnet-api.4160.nodely.dev/v2/status")

    console.log(result)

    http_get("https://httpbin.org/delay/3").then(() => console.log("3"))
    http_get("https://httpbin.org/delay/2").then(() => console.log("2"))
    http_get("https://httpbin.org/delay/1").then(() => console.log("1"))

    try {
        div(100n, 0n)
    } catch (e) {
        console.log('We caught a panic!', e)
    }

    try {
        add(MAX_U64, 1n)
    } catch (e) {
        console.log('We caught an error!', e)
    }

    // Note that the wasm binding does NOT check for overflows in args
    const sum = add(MAX_U64 + 10n, 0n)
    console.log('Says the sum is:', sum)
}
