"These tools" == UniFFI and WASM bindgen

# Goals

- Get a rough of production-readyness of these tools
- Learn how to use these tools
- Learn what the dev experience is like using the bindings (without any hand-written wrappers)
- Get a rough idea of performance implications

# Findings

## Production Ready?

### UniFFI

UniFFI is still in v0, but it is used in production by Mozilla (core maintainers) in various Firefox products across multiple platforms. It is also used in production by Nord Security, who are the maintainers for some of the third-party binding generators as well.

I believe we could go into production with the current version of UniFFI with fairly high confidence, but there is some risk to breaking changes especially if we use any third-party bindings.

### WASM Bindgen

WASM Bindgen is also in v0, but it is a much more popular project. That being said, it is harder to determine how many product are actually using it in production. There are various popular Rust-based web frameworks that leverage WASM bindgen. Crate compatability with WASM bindgen is a mixed bag. For example, to get HTTP I needed to use a library with a downstream dependency that is no longer maintained: https://github.com/koute/stdweb/issues/427#issuecomment-2510242651. This specific issue is not a concern per se, but I think it is a reminder of the maturity of the ecosystem.

### Conclusion

Adoption of WASM bindgen and UniFFI will result in some growing pains, but I have fairly high confidence in the current state of these tools for production use.

## How hard is it to use?

### UniFFI

UniFFI is largely driven by proc macros within the Rust code and some small pieces of logic in the build script. Actually using it and generating the bindings is fairly straightforward, but the challenges come with the architecture of the functions you are exposing over the FFI. For example, all exposed objects must be thread-safe, meaning mutexes or atomic types (or some Rust magic/unsafe code) are required. This means it's not quite as simple as taking a rust lib, adding a proc macro on top and calling it a day. It will often require a specific implementation with UniFFI's thread-safety requirement in mind (assumiong the original implementation didn't need to be thread-safe).

### WASM Bindgen

WASM Bindgen is also fairly straightforward to use, especially with `wasm-pack`. It is entirely driven by proc macros within the Rust code and does not have the thread-safety requirement meaning it's more compatible with existing non-thread-safe code. The downside, however, is that certain crates have dependencies that are not compatible with WASM.

### Conclusion

The tools themselves are fairly easy to use, but they impose some constraints on the implementation of the code you are exposing over the FFI.

## Dev Experience

Both of these tools have roughly the same dev experience on the consumer side. The main difference is that some UniFFI bindings do not integrate into the garbage collection of the target language which can make it harder to use without hand-written wrappers. In both cases, there are two main impacts on the developer experience:

1. Developers cannot "look under the hood" of the FFI bindings if they are not familiar with Rust (and they might not even know where to look). Similarly they cannot easily "patch" the SDK (ie. just modify files in `node_modules`)

2. Rather than copying data often, both bindings often return pointers to the target language. This helps with performance, but it can make it harder to debug. For example, printing a Rust-owned object will just print out a pointer that is meaningless to the developer even if the rust object has relevant data

## Performance

Crossing the boundary (either FFI or WASM) does add some performance overhead. Performance gains are found when the Rust-side is doing heavy computations. Crossing complex data from the target language to Rust tends to be the most expensive operation.

### UniFFI

Below are the results of some simple, non-exhaustive benchmarks (from [./consumers/python/app.py](./consumers/python/app.py)):

```
Calling a no-op function:
no_op_ffi                           0.000915 ms/iter (0.009153 sec/10000)
no_op_native                        0.000026 ms/iter (0.000261 sec/10000)

Pushing to an array of numbers in a class/struct:
native_push                         0.000105 ms/iter (0.001046 sec/10000)
push_ffi                            0.002416 ms/iter (0.024158 sec/10000)

Finding the min value in an array of numbers in a class/struct:
find_min_native                     0.075549 ms/iter (0.755488 sec/10000)
find_min_ffi                        0.003922 ms/iter (0.039224 sec/10000)

Quick sort algo on an array of numbers in a class/struct:
quick_sort_native                   0.000884 ms/iter (0.008842 sec/10000)
quick_sort_ffi                      0.000476 ms/iter (0.004763 sec/10000)

Converting between Python class and Rust struct:
py_class_to_rust_struct             0.214266 ms/iter (2.142655 sec/10000)
rust_struct_to_py_class             0.036052 ms/iter (0.360518 sec/10000)

Serializing a class/struct to msgpack:
serialize_rust_struct               0.006251 ms/iter (0.062513 sec/10000)
serialize_python_class              0.002725 ms/iter (0.027248 sec/10000)
```

As noted, most tests show a performance hit for crossing the FFI boundary. The quick sort test, however, demonstrates where Rust can be edge out the native implementation. The performance gain is expected to grow proportionally with compute complexity.

It should be noted that UniFFI leaves some optimizations on the table. For example, UniFFI uses serializes and deserializes rust buffers for all data types, but C representations and pointers could be used for what would likely be better performance. The ADR for this decision can be found here: https://github.com/mozilla/uniffi-rs/blob/main/docs/adr/0002-serialize-complex-datatypes.md. An issue tracking related performance improvements can be found here: https://github.com/mozilla/uniffi-rs/issues/2156.

### WASM

The results of the WASM benchmarks are largely the same as the UniFFI benchmarks.

```
JS Object to WASM struct            6.445 ms/iter (1289 ms/200)
WASM struct to JS object            1.02 ms/iter (204 ms/200)
push (native)                       0 ms/iter (0 ms/10000)
push (WASM)                         0.0001 ms/iter (1 ms/10000)
find_min (native)                   0 ms/iter (0 ms/1)
find_min (WASM)                     0 ms/iter (0 ms/1)
quick_sort (native)                 1427 ms/iter (1427 ms/1)
quick_sort (WASM)                   188 ms/iter (188 ms/1)
no_op                               0.000003875 ms/iter (3875 ms/1000000000)
no_op (native)                      3.54e-7 ms/iter (354 ms/1000000000)
```
