## String Interning

Fast and efficient string interning with a simple API.

### Features

- Intern strings and get a unique ID for each string.
- Initialize with a pre-allocated capacity.

### Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
string_interner = "0.1.0"
```

### Usage

```rust
use string_interner::Intern;

let mut interner = Intern::new();
let id = interner.intern("hello");
assert_eq!(interner.lookup(id), 0);
```

