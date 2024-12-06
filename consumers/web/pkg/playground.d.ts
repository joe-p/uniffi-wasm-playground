/* tslint:disable */
/* eslint-disable */
export function add(a: bigint, b: bigint): bigint;
export function sub(a: bigint, b: bigint): bigint;
export function div(dividend: bigint, divisor: bigint): bigint;
export function equal(a: bigint, b: bigint): boolean;
export function say_after(ms: bigint, who: string): Promise<string>;
export function http_get(url: string): Promise<string>;
export function genkey(): Uint8Array;
export function falcon_genkey(seed: Uint8Array): FalconKeyPair;
export interface FalconKeyPair {
    public_key: number[];
    private_key: number[];
}


export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly add: (a: bigint, b: bigint) => [bigint, number, number];
  readonly sub: (a: bigint, b: bigint) => [bigint, number, number];
  readonly div: (a: bigint, b: bigint) => bigint;
  readonly equal: (a: bigint, b: bigint) => number;
  readonly say_after: (a: bigint, b: number, c: number) => any;
  readonly http_get: (a: number, b: number) => any;
  readonly genkey: () => [number, number];
  readonly falcon_genkey: (a: number, b: number) => [number, number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly closure261_externref_shim: (a: number, b: number, c: any) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h382eebc988c99998: (a: number, b: number) => void;
  readonly closure290_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
