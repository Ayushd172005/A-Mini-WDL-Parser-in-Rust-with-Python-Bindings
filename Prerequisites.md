# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install maturin (the PyO3 build tool)
pip install maturin

# Create project
mkdir wdl-lite-py && cd wdl-lite-py
maturin init --bindings pyo3
# choose "pyo3" when prompted
