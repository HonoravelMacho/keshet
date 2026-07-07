# קֶשֶׁת - Keshet (Polynomial Hunter) 🏹

**Keshet** (קֶשֶׁת - Rainbow/Bow) is a high-performance analytical brute-force engine written in Rust. It is designed to find polynomials of arbitrary degrees whose roots coincide with provided irrational constants (transcendental or algebraic).

## 🚀 Features

- **Exotic Algebraic Composition:** Searches through combinations of $\pi$, $e$, $\Phi$ (Golden Ratio), Catalan's Constant ($G$), Euler-Mascheroni Constant ($\gamma$), square roots, and natural logarithms.
- **High Precision:** Powered by the `rug` crate (GMP), using 128-bit floating-point arithmetic to ensure scientific accuracy.
- **Massive Parallelism:** Fully utilizes all CPU cores via the `rayon` crate.
- **Book-Style Rendering:** Beautifully renders polynomials with vertical fractions in the terminal.
- **Interactive & CLI Modes:** Use it as an interactive wizard or as a fast CLI tool with flags.
- **Archivist Engine:** Automatically logs every discovery to `keshet_descobertas.txt`.

## 🛠 Installation (Linux/Pop!_OS)

Ensure you have the GMP development libraries installed:

```bash
sudo apt update
sudo apt install build-essential libgmp-dev -y

Clone and build:
code
Bash
git clone https://github.com/HonoravelMacho/keshet.git
cd keshet
cargo build --release
sudo cp target/release/keshet /usr/local/bin/

🎯 Usage
Interactive Mode
Simply run:
code
Bash
keshet
CLI Flag Mode
code
Bash
keshet -g 2 -a 10 -n 1.6180339

📜 License
Licensed under the Apache License, Version 2.0.
✍️ Author
Tiago Rabelo - GitHub HonoravelMacho
