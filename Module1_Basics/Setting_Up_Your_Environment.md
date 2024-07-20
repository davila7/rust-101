# Installing Rust

## Step-by-Step Guide (MAC OS)

1. **Open your terminal.**

2. **Run the following command to download and install Rust:**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
3. Follow the on-screen instructions:

    - You will see a welcome message and information about the installation.
    - You will be prompted with installation options:
    - Press Enter to proceed with the standard installation.
    - Alternatively, you can customize the installation or cancel it.

4. Wait for the installation to complete.

   - The installer will download and install the Rust compiler (rustc), the package manager (Cargo), and other components.

5. Configure your shell:
   - After installation, you may need to restart your terminal or run the following command to configure your current shell:
    ```bash
        source "$HOME/.cargo/env"
    ```
    - For fish shell users, use:
    ```bash
        source "$HOME/.cargo/env.fish"
    ```

6. Add Cargo's bin directory to your PATH:
    - If you are using zsh, run:
    ```bash
        echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc && source ~/.zshrc
    ```

7. Verify the installation:
    - Check the version of rustup:
    ```bash
    rustup --version
    ```
    - Check the version of rustc:
    ```bash
    rustc --version
    ```
    - Check the version of Cargo:
    ```bash
    cargo --version
    ```
