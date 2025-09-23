# 📘 Jargon #1 – Stack vs Heap  

Rust has clear and strict rules about **stack** and **heap** memory management.  

---

## ⚡ Stack  
- **Fast allocation and deallocation**.  
- Stores data with **fixed, known size at compile time**.  
- Commonly used for **primitive types**.  

✅ Examples of **stack data**:  
- Numbers (`i32`, `i64`, `f64`, …)  
- Booleans (`true`, `false`)  
- Fixed-size arrays  

---

## 🧩 Heap  
- Used for **data that can grow at runtime**.  
- Allocation is slower than stack, but allows flexibility.  
- Accessed via **pointers/references** from the stack.  

✅ Examples of **heap data**:  
- Strings (`String`)  
- Vectors (`Vec<T>`)  
- Any data whose size is not known at compile time  

---

## 🔬 Examples  

### 📍 Hello World with Numbers (Stack)  
```rust
fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}
````

---

### 📍 Hello World with Strings (Heap)

```rust
fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}
```

---

### 📍 Updating Strings at Runtime (Heap Growth)

```rust
fn update_string() {
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    s.push_str(" and some additional text");
    println!("After update: {}", s);
}
```

---

## 🖥 Full Program – Memory in Action

```rust
fn main() {
    stack_fn();       // Uses stack memory
    heap_fn();        // Uses heap memory
    update_string();  // Shows heap growth at runtime
}
```

---

✅ **Summary:**

* **Stack:** fast, simple, fixed-size data.
* **Heap:** flexible, resizable data at runtime.
* Rust ensures memory safety while giving you control over both.

---

