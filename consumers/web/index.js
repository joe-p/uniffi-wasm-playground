import init, { add } from "./pkg/arithmetical.js";

export async function main() {
    await init();
    alert(add(1n, 2n))
}
