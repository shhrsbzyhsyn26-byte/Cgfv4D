cat > README.md << 'EOF'
# Cgfv4D - CLI Game First Version 4D

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-4D_Game-blue?style=for-the-badge)

**A minimal 4D (four-dimensional) terminal game / simulation** written in Rust.

### Name Meaning
**Cgfv4D** = **C**LI **G**ame **F**irst **V**ersion **4**D

---

### How to Play

You move inside a **four-dimensional world** (x, y, z, w).

- **Starting position**: `(0, 0, 0, 0)`
- There are two **Particles** placed in the 4D space.

#### Movement Commands:
- `+x` / `-x` → Move along X axis
- `+y` / `-y` → Move along Y axis
- `+z` / `-z` → Move along Z axis
- `+w` / `-w` → Move along W axis (the 4th dimension)

After each move, the current position and block status will be shown:
- `Empty` → The current coordinate is empty
- `Particle` → There is a particle at this position

---

### Installation & Running

#### 1. From Source (Recommended):
```bash
git clone https://github.com/shhrsbzyhsyn26-byte/Cgfv4D.git
cd Cgfv4D
cargo build --release
./target/release/Cgfv4D
