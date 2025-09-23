# 📘 Jargon #0 – Mutability  

## 🔒 Immutable Variables  
Immutable variables represent values that **cannot be changed** once assigned.  

```rust
fn main() {
    let x: i32 = 1;
    x = 2; // ❌ Error: x is immutable
    println!("{}", x);
}
````

By default, **all variables in Rust are immutable**.
This design choice has two big advantages:

1. **Thread-safety**

   * Immutable data is inherently safe across multiple threads.
   * Since no thread can alter it, **no synchronization (locks, mutexes)** is needed when accessing it concurrently.

2. **Compiler optimization**

   * When the compiler knows data won’t change, it can optimize code better.

---

## 🔑 Mutable Variables

You can make variables mutable using the **`mut`** keyword:

```rust
fn main() {
    let mut x: i32 = 1;
    x = 2; // ✅ Works fine
    println!("{}", x);
}
```

---

## 💡 Notes

* `const` in **JavaScript** is **not the same** as Rust’s immutability.

  * In JS, you can still update the *contents* of `const` arrays and objects.

  ```js
  const arr = [1, 2];
  arr.push(3); // ✅ Allowed
  console.log(arr); // [1, 2, 3]
  ```

* JavaScript also has the concept of **immutability** through libraries like [immutable.js](https://www.npmjs.com/package/immutable).

---

