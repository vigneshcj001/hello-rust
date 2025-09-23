# 🦀 Memory Management in Rust  

Whenever you run a program (C++, Rust, JS), it allocates and deallocates memory on **RAM**.  

For example, in **JavaScript**:  

```js
function main() {
  runLoop();
}

function runLoop() {
  let x = [];
  for (let i = 0; i < 100000; i++) {
    x.push(1);
  }
  console.log(x);
}

main();
````

* As the `runLoop` function executes, a new array is created and pushed to RAM.
* Later, **Garbage Collection (GC)** removes it automatically.

---

## ⚡ Ways of Memory Management

There are 3 popular approaches:

1. **Manual memory management** – Example: C (malloc/free).
2. **Garbage collection (GC)** – Example: Java, JavaScript, Go.
3. **Ownership model** – Example: Rust (no GC, no manual free).

---

## 🚀 Memory Management in Rust

Rust ensures **safety and efficiency** *without a garbage collector*.
This is one of the main reasons why **Rust is so fast**.

Rust achieves this using:

### 🔑 1. Mutability

* By default, variables are **immutable** (`let`).
* You can make them mutable with `let mut`.
* This enforces safer code and prevents accidental changes.

### 🧩 2. Heap vs Stack

* **Stack**: Fast, stores fixed-size data (e.g., integers).
* **Heap**: Stores dynamic data (e.g., vectors, strings).
* Rust tracks where data lives and when it should be freed.

### 📦 3. Ownership Model

* Each value in Rust has a **single owner**.
* When the owner goes out of scope → memory is **freed automatically**.
* Prevents *double free* and *memory leaks*.

### 🔗 4. Borrowing & References

* Instead of copying values, you can **borrow** them using references (`&`).
* Borrowing ensures safe access without taking ownership.
* Supports **mutable (`&mut`)** and **immutable (`&`)** borrowing.

### ⏳ 5. Lifetimes

* Rust enforces that references are always valid.
* **Lifetimes** describe how long a reference is valid.
* Prevents dangling pointers and invalid memory access.

---

✅ **Summary:**
Rust eliminates the need for garbage collection by enforcing a strict ownership model, borrowing rules, and lifetimes. This gives **C++-level performance** with **memory safety guarantees**.

---

