# Benchmark Results – Soplang vs. Rust vs. Python

> **Host**   HP EliteBook x360 830 G8   |   **OS** Kali Linux Rolling (kernel 6.12.20‑amd64)   |   **CPU** Intel i7‑1185G7   |   **RAM** 32 GB   |   **Shell** zsh 5.9  
> **Display res.** 1920 × 1080 + 1920 × 1080   |   **DE / WM** GNOME 48 / Mutter

---

## Test Programs ("Hello, world!")

| Language           | Build cmd                                   | Run cmd            |
| ------------------ | ------------------------------------------- | ------------------ |
| Python 3           | —                                           | `python3 hello.py` |
| Sopnac (native)    | `sopnac hello.sop`                          |`./hello`           |
| Rust 1.78 (stable) | `rustc -O hello.rs -o hello_rust`           | `./hello_rust`     |

All binaries compiled with full optimisation (`-O` or `--release`).

---

## 1. Hyperfine (1 000+ runs, sub‑ms precision)

| Program              | Mean run‑time ± σ       | Speed‑up vs Python |
| -------------------- | ----------------------- | ------------------ |
| `python3 hello.py`   | **9.3 ms ± 0.8 ms**     |  1×                |
| `./hello` *(Sopnac)* | **0.274 ms ± 0.114 ms** | **≈ 34× faster**   |
| `./hello_rust`       | **0.416 ms ± 0.260 ms** | ≈ 22× faster       |

*Sopnac narrowly beats the equivalent Rust build on this micro‑benchmark; both crush Python.*

---

## 2. GNU `time` (coarse, 1 run each)

```text
python3 hello.py   →  real 0.01 s   user 0.00 s   sys 0.00 s
./hello            →  real 0.00 s   user 0.00 s   sys 0.00 s
./hello_rust       →  real 0.00 s   user 0.00 s   sys 0.00 s
```

`time` rounds to centiseconds, so both native binaries register 0.00 s.

---

## Take‑aways

* **Sopnac (native)** delivers instant startup and native‑level speed: \~34× faster than the interpreted Python version.
* **Parity with Rust**: On this trivial workload Sopnac is marginally faster than the Rust binary—differences are within the noise floor.
* **Python overhead**: Even printing a single line shows the cost of interpreter startup.

---

### Re‑running

```bash
hyperfine 'python3 hello.py' './hello' './hello_rust'
/usr/bin/time -p python3 hello.py
/usr/bin/time -p ./hello
/usr/bin/time -p ./hello_rust
```

> *Updated 2025‑05‑04.*
