# lifetime — compile-time lifetime annotation checker

`lifetime` is a zero cost compile time lifetime annotation system for Rust. It uses a build-time static analysis to detect use-after-free, scope escape, and reverse-order lifetime violations without any runtime overhead.

The `lt!` macro is an identity macro that expands to its argument at compile time. A companion CLI tool scans source files for `lt!` annotations and reports violations as clear error messages with file:line locations.

`cargo lifetime check` automatically traverses all modules reachable from the entry file (`src/main.rs` or `src/lib.rs`) by following `mod` declarations recursively. Use `--file` to check a single standalone file.

## Quick start

### 1. Add library dependency

```toml
[dependencies]
lifetime = { git = "https://github.com/fuji-184/R_LT-Rust-User-Space-Lifetime-Library" }
```

### 2. Install CLI tool

Installs only the CLI binary — no test code included:

```bash
cargo install --git https://github.com/fuji-184/R_LT-Rust-User-Space-Lifetime-Library
```

Or from a local checkout:

```bash
cargo install --path cargo-lifetime
```

### 3. Annotate your code

Wrap owners and borrows with matching labels:

```rust
use lifetime::lt;

fn main() {
    let val = lt!(vec![1, 2, 3], "my_data");
    let ptr = lt!(val.as_ptr(), "my_data");
    drop(val);
    unsafe { println!("{}", *ptr); }
}
```

### 4. Check for violations

Auto-detect `src/main.rs` or `src/lib.rs`:

```bash
cargo lifetime check
```

Check a specific file:

```bash
cargo lifetime check --file tests/my_code.rs
```

## Usage

### Annotate your code

Wrap owned values and their borrows with `lt!(expr, "label")`:

```rust
use lifetime::lt;

fn main() {
    let val = lt!(vec![1, 2, 3], "my_data");
    let ptr = lt!(val.as_ptr(), "my_data");
    drop(val);
    unsafe { println!("{}", *ptr); }
}
```

The label (`"my_data"`) pairs each owner with its borrows. Both must use the same label.

### Check for violations

```bash
cargo lifetime check
```

### Violations detected

| violation | example |
|-----------|---------|
| **explicit drop** | `drop(val)` while `ptr` has active borrow |
| **scope escape** | borrow outlives the scope where owner was created |
| **reverse order** | borrow declared before owner in same scope |
| **function move** | owner passed to function while borrow active |

## Syntax reference

### `lt!` macro

```rust
lt!(expr, "label")
```

| argument | description |
|----------|-------------|
| `expr` | Any Rust expression (value, reference, pointer, function call) |
| `"label"` | A string literal lifetime label |

- **Owners**: expressions that don't look like pointers (e.g. `vec![1,2,3]`, `Box::new(42)`, `String::from("hello")`, `Rc::new(...)`, `Arc::new(...)`)
- **Borrows**: expressions that look like pointers/references (e.g. `&val`, `val.as_ptr()`, `val.as_ref()`, `val.as_bytes()`, `Box::into_raw(box)`, `Rc::as_ptr(&rc)`, `Arc::as_ptr(&arc)`, `NonNull::new(&val)`, `NonNull::new_unchecked(ptr)`)

### Detected pointer expressions

`&expr`, `&mut expr`, `&raw const expr`, `&raw mut expr`, `expr.as_ptr()`, `expr.as_mut_ptr()`, `expr.as_ref()`, `expr.as_mut()`, `expr.as_bytes()`, `expr.as_bytes_mut()`, `expr.as_slice()`, `expr.as_mut_slice()`, `expr.as_str()`, `expr.as_mut_str()`, `expr.as_deref()`, `expr.as_deref_mut()`, `expr.borrow()`, `expr.borrow_mut()`, `Box::into_raw(...)`, `Rc::as_ptr(...)`, `Arc::as_ptr(...)`, `NonNull::new(...)`, `NonNull::new_unchecked(...)`

## How it works

1. **Build time scanning**: The CLI reads the entry file (`src/main.rs` or `src/lib.rs`) and recursively follows all `mod` declarations to collect every module in the crate.
2. **Scope tracking**: It tracks `{ }` nesting depth, variable declarations, and owner/borrow pairs by label.
3. **Violation detection**: Checks for drops, scope exits, and function calls that would invalidate active borrows.
4. **Error reporting**: Reports violations as `file.rs:line: [label] description` with declaration locations.

The `lt!` macro itself is a zero-cost identity — the single-argument form produces a compile error as a reminder, the two-argument form compiles away to nothing:

```rust
macro_rules! lt {
    ($e:expr, $l:expr) => { $e };
    ($e:expr) => {
        compile_error!("missing lifetime label in lt!() — use lt!(expr, \"label\")");
    };
}
```

All checking happens at analysis time, zero runtime cost.

## Custom configuration

Extend the built-in safe function and pointer expression lists using either a config file or the `Config` API.

### Config file (`.lifetime.toml`)

Create `.lifetime.toml` in your project root:

```toml
# Mark functions as safe (borrow check skipped for their arguments)
safe_fn: my_custom_safe_fn
safe_fn: another_safe_fn

# Mark expressions by prefix as pointer borrows (e.g. custom pointer wrappers)
prefix: MyPtr::new(
prefix: CustomHandle::from_raw(

# Mark expressions by suffix as pointer borrows
suffix: .get_ref()
suffix: .raw_handle()
```

Config supports three keys:

| key | description |
|-----|-------------|
| `safe_fn` | Function name to skip borrow check on its arguments |
| `prefix` | Expression prefix to treat as a pointer/borrow |
| `suffix` | Expression suffix to treat as a pointer/borrow |

Lines starting with `#` are ignored.

### Using the API (library users)

```rust
use lifetime_cli::{Config, check_source_with_config};

let config = Config::new()
    .add_safe_fn("my_safe_fn")
    .add_pointer_prefix("CustomPtr::new(")
    .add_pointer_suffix(".as_raw()");

let errors = check_source_with_config(src, &config);
```

The default config is used when calling `check_source(&src)`.

## CLI

```bash
cargo lifetime                                Show usage info
cargo lifetime check                          Auto-detect & traverse modules from src/main.rs or src/lib.rs
cargo lifetime check --file <path>            Check a specific file only (no module traversal)
cargo lifetime check --config <path>          Use custom config file (e.g. .lifetime.toml)
cargo lifetime check <path>                   Check a specific file by path
cargo lifetime --help                         Show detailed usage
```

## Development

The test runner lives in its own crate so CLI users never pull in test code.

```bash
cargo run -p lifetime-test-runner                          Run test suite (54 tests)
cargo run -p lifetime-cli -- check                         Check project (traverse all modules)
cargo run -p lifetime-cli -- check --file <path>           Check a single file
cargo run -p lifetime-cli                                  Show usage info
```

## Project structure

```
Cargo.toml           Workspace root
lifetime/            Library crate: just the lt! identity macro (zero-cost, no analysis)
  src/lib.rs
cargo-lifetime/      CLI crate: analysis engine + cargo-lifetime binary
  src/lib.rs         Library: analysis engine + shared utilities
  src/main.rs        CLI binary: check subcommand only
test-runner/         Standalone test runner (separate from CLI)
  src/main.rs        Runs all 54 test fixtures
tests/               Test fixtures (valid_*.rs → zero violations, invalid_*.rs → violations expected)
```
