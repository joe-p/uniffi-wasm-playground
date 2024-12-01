import init, { add, say_after } from "./pkg/arithmetical.js";

export async function main() {
    await init();
    console.log(add(1n, 2n))
    const promises = [
        say_after(1000n, "Alice"),
        say_after(500n, "Bob")
    ]
    promises.forEach(p => p.then(console.log))
}
