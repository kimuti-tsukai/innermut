# innermut

A Rust crate providing unified traits for accessing inner values from various container types like `RefCell`, `Mutex`, and `RwLock`.

## Overview

The `innermut` crate defines two core traits:

- `Inner` - For immutable access to wrapped values
- `InnerMut` - For mutable access to wrapped values

These traits provide a consistent interface across different container types, making it easier to write generic code that works with various synchronization primitives and interior mutability patterns.

## Features

- **Unified Interface**: Access inner values through consistent trait methods regardless of container type
- **Zero Dependencies**: Pure Rust implementation with no external dependencies
- **Generic Guards**: Supports any guard type that implements `Deref`/`DerefMut`
- **Error Handling**: Proper error propagation for failed access attempts

## Supported Types

- `RefCell<T>` - Single-threaded interior mutability
- `Mutex<T>` - Thread-safe mutual exclusion
- `RwLock<T>` - Multiple readers, single writer lock

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
innermut = "0.1.0"
```

### Basic Example

```rust
use innermut::{Inner, InnerMut};
use std::cell::RefCell;
use std::sync::{Mutex, RwLock};

fn read_inner<T: Inner<Inner = i32>>(container: &T) -> Result<i32, T::Error<'_>> {
    let guard = container.inner()?;
    Ok(*guard)
}

fn main() {
    let refcell = RefCell::new(42);
    let mutex = Mutex::new(42);
    let rwlock = RwLock::new(42);

    // All containers can be used with the same interface
    println!("{}", read_inner(&refcell).unwrap()); // 42
    println!("{}", read_inner(&mutex).unwrap());   // 42
    println!("{}", read_inner(&rwlock).unwrap());  // 42
}
```

### Mutable Access

```rust
use innermut::{Inner, InnerMut};
use std::cell::RefCell;

fn modify_inner<T: InnerMut<Inner = i32>>(container: &mut T) -> Result<(), T::Error<'_>> {
    let mut guard = container.inner_mut()?;
    *guard = 100;
    Ok(())
}

fn main() {
    let mut refcell = RefCell::new(42);
    modify_inner(&mut refcell).unwrap();
    
    let guard = refcell.inner().unwrap();
    println!("{}", *guard); // 100
}
```

## Trait Details

### `Inner` Trait

```rust
pub trait Inner {
    type Inner;
    type Guard<'a>: Deref<Target = Self::Inner>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner(&self) -> Result<Self::Guard<'_>, Self::Error<'_>>;
}
```

### `InnerMut` Trait

```rust
pub trait InnerMut: Inner {
    type GuardMut<'a>: DerefMut<Target = Self::Inner>
    where
        Self: 'a,
        Self::Inner: 'a;
    type Error<'a>
    where
        Self: 'a,
        Self::Inner: 'a;

    fn inner_mut(&mut self) -> Result<Self::GuardMut<'_>, <Self as InnerMut>::Error<'_>>;
}
```

## Error Types

Each container type has its own error type:

- `RefCell<T>`: `BorrowError` for `inner()`, `BorrowMutError` for `inner_mut()`
- `Mutex<T>`: `TryLockError<MutexGuard<T>>` for both methods
- `RwLock<T>`: `TryLockError<RwLockReadGuard<T>>` for `inner()`, `TryLockError<RwLockWriteGuard<T>>` for `inner_mut()`

## Use Cases

- **Generic Container Handling**: Write functions that work with any container type
- **Testing**: Mock different container types in tests
- **Library Design**: Create APIs that accept various synchronization primitives
- **Abstraction**: Hide implementation details of interior mutability from consumers

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.