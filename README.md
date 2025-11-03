# 🌡️ Temperature Converter — Rust Mini Project

---

## 📘 Overview
This project converts temperatures between **Fahrenheit** and **Celsius** using simple mathematical formulas.  
It’s one of the beginner challenges from *The Rust Programming Language* book, designed to reinforce **function usage**, **constants**, and **floating-point arithmetic** in Rust.

---

## 🧩 Features
- 🔄 Convert Fahrenheit → Celsius  
- 🔁 Convert Celsius → Fahrenheit  
- 📏 Uses constants for conversion factors  
- 🦀 Demonstrates functions and variable shadowing in Rust  

---

## 🗂️ Project Structure


Temperature_Converter/
├── src/
│ └── main.rs
├── target/
├── .gitignore
├── Cargo.toml
├── Cargo.lock
└── README.md


---

## ⚙️ Prerequisites
Install Rust using **rustup** (if you haven’t already):

```bash
curl https://sh.rustup.rs -sSf | sh
```

Then verify installation:

```bash
rustc --version
cargo --version
```

---

## 🚀 Setup and Run

Clone the repository:

```bash
git clone https://github.com/skipajenkins/Temperature_Converter.git
```

Navigate into the project folder:

```bash
cd Temperature_Converter
```

Build the project:

```bash
cargo build
```

Run the program:

```bash
cargo run
```

---

## 🧠 Code Overview
### 🔹 Main Function

```bash
fn main() {
    let _a = convert_to_celsius(0.0);
    let _b = convert_to_fahrenheit(0.0);
}
```

### 🔹 Convert Fahrenheit → Celsius

```bash
fn convert_to_celsius(temp: f32) {
    const CONST_A: f32 = 32.0;
    const CONST_B: f32 = 0.55555555555;

    let temp: f32 = (temp - CONST_A) * CONST_B;
    println!("Temperature in celsius: {temp}");
}
```

### 🔹 Convert Celsius → Fahrenheit

```bash
fn convert_to_fahrenheit(temp: f32) {
    const CONST_A: f32 = 32.0;
    const CONST_C: f32 = 1.8;

    let temp: f32 = temp * CONST_C + CONST_A;
    println!("Temperature in fahrenheit: {temp}");
}
```

---

## 🧮 Example Output
Temperature in celsius: -17.77778
Temperature in fahrenheit: 32

---

## 🧰 Concepts Practiced
Concept	Description
### 🔢 Constants	Fixed values like 32.0 and 1.8 for conversion formulas
### 🔁 Functions	Code reuse with convert_to_celsius() and convert_to_fahrenheit()
### 🧮 Floating Points	Using f32 for fractional arithmetic
### 🧱 Statements vs Expressions	Practicing Rust’s expression-oriented syntax
### 🧠 Challenge Context

This is part of the Rust Book’s end-of-chapter exercises:

“Try building programs to do the following:

Convert temperatures between Fahrenheit and Celsius.

Generate the nth Fibonacci number.

Print the lyrics to ‘The Twelve Days of Christmas.’”

---

## 🪪 License

This project is licensed under the MIT License
.

---

## 💡 Author

Created by @skipajenkins
Part of the Rust Learning Series — beginner projects from The Rust Programming Language book.
