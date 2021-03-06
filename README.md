# wasm2rs
WASM to Rust Decompiler.

## Usage Example
Suppose you compile a Rust program that looks something like this to WASM:

```rs
#![no_main]

struct Coordinates {
    x: f32,
    y: f32
}

impl Coordinates {
    #[no_mangle]
    fn distance(&self, other: &Self) -> f32 {
        ((other.y - self.y).powi(2) + (other.x - self.x).powi(2)).sqrt()
    }

    #[no_mangle]
    unsafe fn call_ff(&self) -> f32 {
        do_stuff(self.x, self.y)
    }
}

#[link(wasm_import_module = "foo")]
extern {
    #[link_name="do_things"]
    pub fn do_stuff(x: f32, y: f32) -> f32;
}
```

It will look something like this (in WAT):
```
(module
  (type $t0 (func (param f32 f32) (result f32)))
  (type $t1 (func (param i32 i32) (result f32)))
  (type $t2 (func (param i32) (result f32)))
  (import "foo" "do_things" (func $do_stuff (type $t0)))
  (func $distance (type $t1) (param $p0 i32) (param $p1 i32) (result f32)
    (local $l2 f32)
    local.get $p1
    f32.load offset=4
    local.get $p0
    f32.load offset=4
    f32.sub
    local.tee $l2
    local.get $l2
    f32.mul
    local.get $p1
    f32.load
    local.get $p0
    f32.load
    f32.sub
    local.tee $l2
    local.get $l2
    f32.mul
    f32.add
    f32.sqrt)
  (func $call_ff (type $t2) (param $p0 i32) (result f32)
    local.get $p0
    f32.load
    local.get $p0
    f32.load offset=4
    call $do_stuff)
  (export "memory" (memory 0))
  (export "distance" (func $distance))
  (export "call_ff" (func $call_ff))
```

You can then decompile this to Rust using wasm2rs (from source):

```
cargo run --bin decompile -- /path/to/file.wasm > decompiled.rs
```

This will create `decompiled.rs` which may look something like this:
```rs
#![no_main]

#[link(wasm_import_module="foo")]
extern {
    #[link_name="do_things"]
    fn __w2r_f0(p0: f32, p1: f32) -> f32;
}

#[no_mangle]
unsafe fn call_ff(mut p0: i32) -> f32 {
    (__w2r_f0((((p0) as *const f32).read()),(((p0) as *const f32).cast::<u8>().add(4).cast::<f32>().read())))
}

#[no_mangle]
unsafe fn distance(mut p0: i32, mut p1: i32) -> f32 {
    let (mut p2): (f32);
    ((({ p2 = ((((p1) as *const f32).cast::<u8>().add(4).cast::<f32>().read()) - (((p0) as *const f32).cast::<u8>().add(4).cast::<f32>().read())); p2 } * (p2)) + ({ p2 = ((((p1) as *const f32).read()) - (((p0) as *const f32).read())); p2 } * (p2))).sqrt())
}
```

To check that `decompiled.rs` emits mostly the same WASM you can then recompile it like so:

```
rustc decompiled.rs --target wasm32-unknown-unknown -o decompiled.wasm -O
```

## Limitations
Only a subset of all WASM opcodes are supported for decompilation. Some common WASM opcodes not yet supported include:
- loop
- if else
- global get/set

If a binary contains an unsupported opcode then wasm2rs will not be able to decompile the binary. Contributions to this end are encouraged.

Additionally, handling of globals and table elements is not yet supported.

Data sections are supported, albeit in a convoluted way. If one or more data sections is present (or if the number of initial pages in the memory section is greater than 16), then the decompiler will emit an exported `setup` function. This function handles cases where static pointers to data are embedded into function code. Without the `setup` function, calling such pointer-using functions will result in an out of bounds memory access, in the best case. Using the `setup` function approach is a workaround of the fact that it is not possible to set the address a of static value in Rust.

For an example of this behavior, follow the steps below:

1. Compile this program
```rs
#![no_main]

#[no_mangle]
fn pow(x: f32, y: f32) -> f32 {
    x.powf(y)
}
```
2. Decompile the resulting WASM binary
3. Compile the resulting Rust source file to WASM
4. Import the module into your favorite WASM runtime
5. Call `pow` before calling `setup`
6. Call `pow` after calling `setup`

Step 5 should result in a trap.
