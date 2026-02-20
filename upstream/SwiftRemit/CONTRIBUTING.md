# Contributing to SwiftRemit

Thank you for your interest in contributing to SwiftRemit! We're building a production-ready Soroban smart contract for USDC remittance on the Stellar blockchain, and we welcome contributions from developers of all experience levels.

This guide will help you understand how to contribute effectively to the project. Whether you're fixing a bug, adding a feature, improving documentation, or helping with tests, your contributions make SwiftRemit better for everyone.

---

## Table of Contents

- [Welcome](#welcome)
- [Types of Contributions](#types-of-contributions)
- [Getting Help](#getting-help)
- [Quick Start for First-Time Contributors](#quick-start-for-first-time-contributors)
- [Development Environment Setup](#development-environment-setup)
  - [Prerequisites](#prerequisites)
  - [Automated Setup](#automated-setup-recommended)
  - [Manual Setup](#manual-setup)
  - [Verifying Your Installation](#verifying-your-installation)
  - [Recommended Development Tools](#recommended-development-tools)
  - [Troubleshooting](#troubleshooting)
- [Understanding the Project](#understanding-the-project)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Security Considerations](#security-considerations)
- [Pull Request Process](#pull-request-process)
- [Contribution Guidelines by Type](#contribution-guidelines-by-type)
- [Documentation Standards](#documentation-standards)
- [Community and Communication](#community-and-communication)

---

## Welcome

> **Note**: This document contains placeholder URLs (e.g., `https://github.com/yourusername/swiftremit`). These should be updated with the actual repository URL once the project is hosted on GitHub.

SwiftRemit is an escrow-based remittance system that enables secure cross-border money transfers using USDC stablecoin. As a financial smart contract on the Stellar blockchain, security, correctness, and code quality are paramount.

We believe in:
- **Open collaboration** - Everyone's contributions are valued
- **Quality over quantity** - Well-tested, secure code is essential
- **Clear communication** - Ask questions, share ideas, and help others
- **Continuous improvement** - Learn, iterate, and grow together

Whether you're new to Rust, Soroban, or blockchain development, we're here to help you succeed.

---

## Types of Contributions

We welcome various types of contributions:

### Code Contributions
- **Bug fixes** - Help us identify and fix issues in the contract logic
- **New features** - Implement enhancements from our roadmap or propose new ideas
- **Performance optimizations** - Improve gas efficiency and contract performance
- **Refactoring** - Improve code structure and maintainability

### Documentation
- **Code documentation** - Add or improve doc comments for functions and modules
- **Guide improvements** - Enhance README, DEPLOYMENT, or other documentation files
- **Examples** - Create usage examples and tutorials
- **Translations** - Help make documentation accessible in other languages

### Testing
- **Test coverage** - Add tests for uncovered code paths
- **Edge case testing** - Identify and test boundary conditions
- **Integration tests** - Test multi-function workflows
- **Security testing** - Test authorization, validation, and error handling

### Bug Reports
- Report issues you discover in the contract or documentation
- Provide clear reproduction steps and expected vs actual behavior
- Include relevant error messages and logs

### Feature Requests
- Propose new features or enhancements
- Explain the use case and expected benefits
- Discuss implementation approaches

---

## Quick Start for First-Time Contributors

Welcome, first-time contributor! 🎉 We're excited to have you here. This section will guide you through making your first contribution to SwiftRemit.

### Step 1: Fork and Clone the Repository

1. **Fork the repository**
   - Visit the [SwiftRemit repository](https://github.com/yourusername/swiftremit)
   - Click the "Fork" button in the top-right corner
   - This creates a copy of the repository under your GitHub account

2. **Clone your fork**
   ```bash
   # Clone your forked repository
   git clone https://github.com/YOUR_USERNAME/swiftremit.git
   
   # Navigate into the project directory
   cd swiftremit
   
   # Add the original repository as an upstream remote
   git remote add upstream https://github.com/yourusername/swiftremit.git
   ```

3. **Verify your setup**
   ```bash
   # Check your remotes
   git remote -v
   
   # You should see:
   # origin    https://github.com/YOUR_USERNAME/swiftremit.git (fetch)
   # origin    https://github.com/YOUR_USERNAME/swiftremit.git (push)
   # upstream  https://github.com/yourusername/swiftremit.git (fetch)
   # upstream  https://github.com/yourusername/swiftremit.git (push)
   ```

### Step 2: Set Up Your Development Environment

Run the automated setup script for your operating system:

**Linux/macOS:**
```bash
chmod +x setup.sh
./setup.sh
```

**Windows (PowerShell):**
```powershell
.\setup.ps1
```

The setup script will install:
- Rust and Cargo
- wasm32-unknown-unknown target
- Soroban CLI
- Development tools (rustfmt, clippy)

**Verify your installation:**
```bash
# Check Rust version
rustc --version

# Check Cargo version
cargo --version

# Check Soroban CLI
soroban --version

# Build the project
cargo build --target wasm32-unknown-unknown --release
```

If everything builds successfully, you're ready to contribute! 🚀

### Step 3: Find a Good First Issue

We label beginner-friendly issues to help you get started:

- **Look for "good first issue" labels** - These are specifically chosen for newcomers
- **Look for "documentation" labels** - Great for learning the codebase
- **Look for "help wanted" labels** - We'd especially appreciate help with these

**Browse issues here**: [Good First Issues](https://github.com/yourusername/swiftremit/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

**Suggested areas for first contributions:**
- **Documentation improvements** - Fix typos, clarify instructions, add examples
- **Test additions** - Add test cases for existing functionality
- **Code comments** - Improve inline documentation
- **Small bug fixes** - Fix minor issues with clear reproduction steps

### Step 4: Make Your Changes

Here's a simple workflow for making your first contribution:

1. **Create a new branch**
   ```bash
   # Create and switch to a new branch
   git checkout -b fix/my-first-contribution
   
   # Branch naming conventions:
   # - fix/description - for bug fixes
   # - docs/description - for documentation
   # - test/description - for test additions
   ```

2. **Make your changes**
   - Edit the relevant files
   - Keep changes focused and small for your first PR
   - Follow the existing code style

3. **Test your changes**
   ```bash
   # Run the test suite
   cargo test
   
   # Format your code
   cargo fmt
   
   # Run the linter
   cargo clippy
   ```

4. **Commit your changes**
   ```bash
   # Stage your changes
   git add .
   
   # Commit with a clear message
   git commit -m "Fix: Correct typo in README example"
   
   # Use clear commit messages:
   # - "Fix: [description]" for bug fixes
   # - "Docs: [description]" for documentation
   # - "Test: [description]" for tests
   # - "Feat: [description]" for new features
   ```

5. **Push to your fork**
   ```bash
   # Push your branch to your fork
   git push origin fix/my-first-contribution
   ```

### Step 5: Submit a Pull Request

1. **Create the PR**
   - Go to your fork on GitHub
   - Click "Compare & pull request" button
   - Fill out the PR template with:
     - **Title**: Clear, concise description of your change
     - **Description**: What you changed and why
     - **Testing**: How you tested your changes
     - **Checklist**: Complete all applicable items

2. **PR Template Checklist**
   ```markdown
   - [ ] Tests pass locally (`cargo test`)
   - [ ] Code is formatted (`cargo fmt`)
   - [ ] Code is linted (`cargo clippy`)
   - [ ] Documentation updated (if applicable)
   - [ ] Tests added for new functionality (if applicable)
   ```

3. **Wait for review**
   - A maintainer will review your PR within 2-3 business days
   - Be responsive to feedback and questions
   - Make requested changes by pushing new commits to your branch

4. **Celebrate! 🎉**
   - Once approved, your PR will be merged
   - You're now an official SwiftRemit contributor!
   - Your contribution will be acknowledged in the release notes

### Example: Your First Contribution

Here's a complete example of fixing a documentation typo:

```bash
# 1. Fork and clone (done once)
git clone https://github.com/YOUR_USERNAME/swiftremit.git
cd swiftremit

# 2. Create a branch
git checkout -b docs/fix-readme-typo

# 3. Make your change
# Edit README.md to fix a typo

# 4. Test and format
cargo test
cargo fmt

# 5. Commit
git add README.md
git commit -m "Docs: Fix typo in installation instructions"

# 6. Push
git push origin docs/fix-readme-typo

# 7. Create PR on GitHub
# Go to your fork and click "Compare & pull request"
```

### Tips for Success

- **Start small** - Your first PR doesn't need to be complex
- **Ask questions** - Use GitHub Discussions or Discord if you're stuck
- **Read existing code** - Learn from the patterns already in the codebase
- **Be patient** - Code review takes time, and that's okay
- **Have fun** - Contributing to open source should be enjoyable!

### First-Time Contributor Recognition

We celebrate all first-time contributors! When your first PR is merged:
- You'll be added to our contributors list
- Your contribution will be highlighted in release notes
- You'll receive a welcome message from the maintainers
- You're officially part of the SwiftRemit community! 🌟

**Thank you for contributing to SwiftRemit!** Every contribution, no matter how small, helps make this project better. We're grateful for your time and effort.

---

## Getting Help

We're here to support you! Here's how to get help:

### Communication Channels

- **GitHub Issues** - For bug reports, feature requests, and technical discussions
  - [View existing issues](https://github.com/yourusername/swiftremit/issues)
  - [Create a new issue](https://github.com/yourusername/swiftremit/issues/new)

- **GitHub Discussions** - For questions, ideas, and general discussions
  - [Join the conversation](https://github.com/yourusername/swiftremit/discussions)

- **Stellar Discord** - For real-time chat and community support
  - [Join Stellar Discord](https://discord.gg/stellar)
  - Find us in the #soroban channel

### When to Use Each Channel

- **GitHub Issues**: Bug reports, feature requests, specific technical problems
- **GitHub Discussions**: Questions about usage, design discussions, brainstorming
- **Discord**: Quick questions, real-time help, community chat

### Getting Effective Help

When asking for help:
1. **Search first** - Check existing issues and documentation
2. **Provide context** - Explain what you're trying to do
3. **Include details** - Share error messages, logs, and code snippets
4. **Be specific** - Clearly describe the problem and what you've tried
5. **Be patient** - Maintainers and community members volunteer their time

### Response Times

- **Issues and PRs**: We aim to respond within 2-3 business days
- **Discord**: Response times vary; community members often help quickly
- **Security issues**: Report privately and expect a response within 24-48 hours

---

## Development Environment Setup

This section provides detailed instructions for setting up your development environment to work on SwiftRemit. We support Linux, macOS, and Windows.

### Prerequisites

Before you begin, ensure you have the following installed:

#### Required Dependencies

1. **Rust and Cargo** (latest stable version)
   - The Rust programming language and its package manager
   - Required for building and testing the smart contract

2. **wasm32-unknown-unknown target**
   - WebAssembly compilation target for Soroban contracts
   - Required for building contracts that run on Stellar

3. **Soroban CLI**
   - Command-line tool for Soroban smart contract development
   - Used for building, testing, deploying, and interacting with contracts

4. **Git**
   - Version control system
   - Required for cloning the repository and managing contributions

### Automated Setup (Recommended)

We provide automated setup scripts that install all required dependencies and build the project. This is the fastest way to get started.

#### Linux and macOS

```bash
# Make the script executable
chmod +x setup.sh

# Run the setup script
./setup.sh
```

The script will:
- Install Rust and Cargo (if not already installed)
- Add the wasm32-unknown-unknown target
- Install Soroban CLI
- Configure Stellar testnet
- Build and optimize the contract

#### Windows

```powershell
# Run the setup script in PowerShell
.\setup.ps1
```

**Note for Windows users**: If Rust is not installed, the script will prompt you to install it from [rustup.rs](https://rustup.rs/). After installation, restart PowerShell and run the script again.

### Manual Setup

If you prefer to install dependencies manually or the automated script doesn't work for your system, follow these platform-specific instructions.

#### Linux

1. **Install Rust and Cargo**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **Add wasm32 target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Soroban CLI**
   ```bash
   cargo install --locked soroban-cli --features opt
   ```

4. **Configure Stellar testnet**
   ```bash
   soroban network add --global testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

5. **Build the contract**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

#### macOS

1. **Install Rust and Cargo**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source "$HOME/.cargo/env"
   ```

2. **Add wasm32 target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Soroban CLI**
   ```bash
   cargo install --locked soroban-cli --features opt
   ```

4. **Configure Stellar testnet**
   ```bash
   soroban network add --global testnet \
     --rpc-url https://soroban-testnet.stellar.org:443 \
     --network-passphrase "Test SDF Network ; September 2015"
   ```

5. **Build the contract**
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

#### Windows

1. **Install Rust and Cargo**
   - Download and run the installer from [rustup.rs](https://rustup.rs/)
   - Follow the installation wizard
   - Restart your terminal after installation

2. **Add wasm32 target**
   ```powershell
   rustup target add wasm32-unknown-unknown
   ```

3. **Install Soroban CLI**
   ```powershell
   cargo install --locked soroban-cli --features opt
   ```

4. **Configure Stellar testnet**
   ```powershell
   soroban network add --global testnet `
     --rpc-url https://soroban-testnet.stellar.org:443 `
     --network-passphrase "Test SDF Network ; September 2015"
   ```

5. **Build the contract**
   ```powershell
   cargo build --target wasm32-unknown-unknown --release
   ```

### Verifying Your Installation

After installation (automated or manual), verify that everything is set up correctly:

```bash
# Check Rust version (should be 1.70 or higher)
rustc --version

# Check Cargo version
cargo --version

# Check that wasm32 target is installed
rustup target list --installed | grep wasm32-unknown-unknown

# Check Soroban CLI version
soroban --version

# Build the project to ensure everything works
cargo build --target wasm32-unknown-unknown --release

# Run the test suite
cargo test
```

**Expected output:**
- All version commands should display version numbers
- The wasm32 target should be listed
- The build should complete without errors
- All tests should pass

If any command fails, refer to the troubleshooting section below or ask for help in our [communication channels](#getting-help).

### Recommended Development Tools

These tools are not required but will significantly improve your development experience:

#### Essential Tools

1. **rustfmt** - Code formatter
   ```bash
   rustup component add rustfmt
   ```
   - Automatically formats Rust code according to style guidelines
   - Run with: `cargo fmt`

2. **clippy** - Linter
   ```bash
   rustup component add clippy
   ```
   - Catches common mistakes and suggests improvements
   - Run with: `cargo clippy`

3. **rust-analyzer** - Language server
   - Provides IDE features like autocomplete, go-to-definition, and inline errors
   - Installation varies by IDE (see IDE configuration below)

#### IDE Configuration

##### Visual Studio Code (Recommended)

VS Code provides excellent Rust support with the right extensions:

1. **Install VS Code**
   - Download from [code.visualstudio.com](https://code.visualstudio.com/)

2. **Install recommended extensions**
   - **rust-analyzer** - Essential for Rust development
     - Provides code completion, inline errors, and refactoring
     - Install from: [marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   
   - **CodeLLDB** - Debugger support
     - Enables debugging Rust code
     - Install from: [marketplace](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
   
   - **Even Better TOML** - TOML file support
     - Syntax highlighting for Cargo.toml
     - Install from: [marketplace](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
   
   - **Error Lens** - Inline error display
     - Shows errors and warnings inline in the editor
     - Install from: [marketplace](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

3. **Configure workspace settings**
   
   Create or update `.vscode/settings.json` in the project root:
   ```json
   {
     "rust-analyzer.checkOnSave.command": "clippy",
     "rust-analyzer.cargo.target": "wasm32-unknown-unknown",
     "editor.formatOnSave": true,
     "[rust]": {
       "editor.defaultFormatter": "rust-lang.rust-analyzer"
     }
   }
   ```

4. **Keyboard shortcuts**
   - `Ctrl+Shift+B` (Cmd+Shift+B on macOS) - Build
   - `F5` - Start debugging
   - `Shift+Alt+F` (Shift+Option+F on macOS) - Format document

##### IntelliJ IDEA / CLion

JetBrains IDEs also provide strong Rust support:

1. **Install IntelliJ IDEA or CLion**
   - Download from [jetbrains.com](https://www.jetbrains.com/)
   - CLion has better Rust support but requires a license
   - IntelliJ IDEA Community Edition is free

2. **Install the Rust plugin**
   - Go to Settings/Preferences → Plugins
   - Search for "Rust"
   - Install the official Rust plugin by JetBrains
   - Restart the IDE

3. **Configure the plugin**
   - Go to Settings/Preferences → Languages & Frameworks → Rust
   - Ensure the toolchain is detected correctly
   - Enable "Use rustfmt instead of built-in formatter"
   - Enable "Run clippy on save"

4. **Configure run configurations**
   - Add a Cargo command configuration for building
   - Add test configurations for running tests
   - Set the target to `wasm32-unknown-unknown` for builds

##### Other Editors

- **Vim/Neovim**: Use [rust.vim](https://github.com/rust-lang/rust.vim) and configure rust-analyzer with your LSP client
- **Emacs**: Use [rust-mode](https://github.com/rust-lang/rust-mode) and configure rust-analyzer with lsp-mode
- **Sublime Text**: Install the Rust Enhanced package and LSP package for rust-analyzer

### Troubleshooting

#### Common Issues

**Issue: "rustc: command not found" after installation**
- **Solution**: Restart your terminal or run `source "$HOME/.cargo/env"` (Linux/macOS)

**Issue: "error: linker `cc` not found" on Linux**
- **Solution**: Install build essentials: `sudo apt-get install build-essential` (Ubuntu/Debian)

**Issue: Soroban CLI installation fails**
- **Solution**: Ensure you have the latest Rust version: `rustup update`

**Issue: Build fails with "error: could not compile" on Windows**
- **Solution**: Install Visual Studio Build Tools from [visualstudio.microsoft.com](https://visualstudio.microsoft.com/downloads/)

**Issue: Tests fail with network errors**
- **Solution**: Check your internet connection; some tests may require network access

#### Getting Help

If you encounter issues not covered here:
1. Check the [Soroban documentation](https://soroban.stellar.org/docs)
2. Search existing [GitHub issues](https://github.com/yourusername/swiftremit/issues)
3. Ask in [GitHub Discussions](https://github.com/yourusername/swiftremit/discussions)
4. Join the [Stellar Discord](https://discord.gg/stellar) #soroban channel

### Next Steps

Once your environment is set up:
1. Read the [Understanding the Project](#understanding-the-project) section to learn about the codebase
2. Review the [Development Workflow](#development-workflow) to understand the build-test cycle
3. Check out [good first issues](https://github.com/yourusername/swiftremit/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) to start contributing

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors.

### Our Standards

- **Be respectful** - Treat everyone with respect and kindness
- **Be inclusive** - Welcome newcomers and diverse perspectives
- **Be constructive** - Provide helpful feedback and suggestions
- **Be patient** - Remember that everyone is learning
- **Assume good intentions** - Give others the benefit of the doubt

### Unacceptable Behavior

- Harassment, discrimination, or offensive comments
- Personal attacks or insults
- Trolling or deliberately disruptive behavior
- Sharing others' private information without permission

### Reporting Issues

If you experience or witness unacceptable behavior, please report it to the project maintainers via email or private message. All reports will be handled confidentially.

---

## Understanding the Project

This section provides a comprehensive overview of the SwiftRemit smart contract architecture, helping you navigate the codebase and understand how different components work together.

### High-Level Architecture

SwiftRemit is a Soroban smart contract that implements an escrow-based remittance system for USDC transfers on the Stellar blockchain. The contract acts as a trusted intermediary that:

1. **Holds funds in escrow** - Accepts USDC from senders and holds it securely
2. **Manages agent network** - Maintains a registry of authorized payout agents
3. **Facilitates payouts** - Releases funds to agents upon confirmation
4. **Collects platform fees** - Deducts and accumulates fees from each transaction
5. **Enables cancellations** - Allows senders to cancel pending remittances

**Key Design Principles:**
- **Security First** - All state-changing operations require proper authorization
- **Fail-Safe** - Comprehensive error handling and validation
- **Gas Efficient** - Optimized storage patterns and minimal operations
- **Event-Driven** - Emits events for all important state changes
- **Immutable Core Logic** - Once deployed, the contract logic cannot be changed

**Architecture Diagram:**
```
┌─────────────┐
│   Sender    │
└──────┬──────┘
       │ create_remittance(agent, amount)
       ▼
┌─────────────────────────────────────┐
│     SwiftRemit Contract             │
│  ┌───────────────────────────────┐  │
│  │  Instance Storage             │  │
│  │  - Admin                      │  │
│  │  - USDC Token Address         │  │
│  │  - Platform Fee (BPS)         │  │
│  │  - Remittance Counter         │  │
│  │  - Accumulated Fees           │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │  Persistent Storage           │  │
│  │  - Remittances (by ID)        │  │
│  │  - Agent Registry             │  │
│  └───────────────────────────────┘  │
└──────────┬──────────────────────────┘
           │ confirm_payout(id)
           ▼
    ┌─────────────┐
    │    Agent    │
    └─────────────┘
```

### Source File Structure

The codebase is organized into focused modules, each with a specific responsibility:

#### `lib.rs` - Main Contract Implementation

**Purpose**: The entry point and core business logic of the smart contract.

**Key Responsibilities:**
- Defines the `SwiftRemitContract` struct and implements all public contract functions
- Orchestrates interactions between different modules (storage, validation, events, errors)
- Implements the main workflows: initialize, create remittance, confirm payout, cancel, etc.
- Enforces authorization requirements using `require_auth()`
- Handles USDC token transfers via the Soroban token interface

**Important Functions:**
- `initialize()` - Sets up the contract with admin, USDC token, and fee configuration
- `register_agent()` / `remove_agent()` - Admin functions to manage the agent network
- `update_fee()` - Admin function to adjust platform fees
- `create_remittance()` - Creates a new remittance and transfers USDC to escrow
- `confirm_payout()` - Agent confirms payout and receives funds minus fees
- `cancel_remittance()` - Sender cancels pending remittance and receives refund
- `withdraw_fees()` - Admin withdraws accumulated platform fees
- Query functions for reading contract state

**Code Pattern:**
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // 1. Authorization check
    sender.require_auth();
    
    // 2. Input validation
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    
    // 3. Business logic validation
    if !is_agent_registered(&env, &agent) {
        return Err(ContractError::AgentNotRegistered);
    }
    
    // 4. Fee calculation
    let fee = calculate_fee(amount, fee_bps)?;
    
    // 5. Token transfer
    token_client.transfer(&sender, &contract_address, &amount);
    
    // 6. State updates
    set_remittance(&env, id, &remittance);
    
    // 7. Event emission
    emit_remittance_created(&env, ...);
    
    // 8. Return result
    Ok(remittance_id)
}
```

#### `types.rs` - Data Structures

**Purpose**: Defines the core data types used throughout the contract.

**Key Types:**

1. **`RemittanceStatus` Enum**
   ```rust
   pub enum RemittanceStatus {
       Pending,    // Remittance created, awaiting payout
       Completed,  // Payout confirmed, funds transferred
       Cancelled,  // Sender cancelled, funds refunded
   }
   ```
   - Tracks the lifecycle state of each remittance
   - Prevents invalid state transitions (e.g., completing a cancelled remittance)

2. **`Remittance` Struct**
   ```rust
   pub struct Remittance {
       pub id: u64,              // Unique identifier
       pub sender: Address,      // Who sent the money
       pub agent: Address,       // Who will receive the payout
       pub amount: i128,         // Total amount in USDC stroops
       pub fee: i128,            // Platform fee in USDC stroops
       pub status: RemittanceStatus,  // Current state
   }
   ```
   - Represents a single remittance transaction
   - Stored in persistent storage, indexed by ID
   - Immutable once created (except status field)

**Design Notes:**
- Uses `#[contracttype]` macro for Soroban serialization
- All types implement `Clone`, `Debug`, `Eq`, `PartialEq` for testing and comparison
- Amounts are in stroops (1 USDC = 10,000,000 stroops) for precision

#### `storage.rs` - Storage Management

**Purpose**: Abstracts all storage operations and defines storage keys.

**Storage Model:**

SwiftRemit uses two types of Soroban storage, each optimized for different use cases:

1. **Instance Storage** - For contract configuration and frequently accessed data
   - **Characteristics**: Lower cost, faster access, tied to contract instance
   - **Lifetime**: Persists as long as the contract instance exists
   - **Use Cases**: Configuration that rarely changes
   
   **Stored Data:**
   - `Admin` - Contract administrator address
   - `UsdcToken` - USDC token contract address
   - `PlatformFeeBps` - Fee percentage in basis points (1 bps = 0.01%)
   - `RemittanceCounter` - Auto-incrementing ID for new remittances
   - `AccumulatedFees` - Total fees collected, awaiting withdrawal

2. **Persistent Storage** - For transaction data and user-specific information
   - **Characteristics**: Higher cost, permanent storage, survives contract upgrades
   - **Lifetime**: Persists indefinitely (with rent payments)
   - **Use Cases**: Critical data that must never be lost
   
   **Stored Data:**
   - `Remittance(u64)` - Individual remittance records, keyed by ID
   - `AgentRegistered(Address)` - Agent registration status, keyed by address

**Storage Key Design:**
```rust
enum DataKey {
    Admin,                      // Instance: single admin
    UsdcToken,                  // Instance: single token
    PlatformFeeBps,             // Instance: single fee config
    RemittanceCounter,          // Instance: single counter
    AccumulatedFees,            // Instance: single fee total
    Remittance(u64),            // Persistent: many remittances
    AgentRegistered(Address),   // Persistent: many agents
}
```

**Key Functions:**
- `set_*()` / `get_*()` - Setter and getter pairs for each storage key
- `has_*()` - Check existence without retrieving value
- `is_agent_registered()` - Special getter that returns `false` if not found (instead of error)

**Storage Best Practices:**
- Use instance storage for singleton configuration values
- Use persistent storage for user data and transaction records
- Always handle missing keys gracefully with `Result<T, ContractError>`
- Minimize storage operations to reduce gas costs

#### `errors.rs` - Error Definitions

**Purpose**: Defines all possible error conditions in the contract.

**Error Types:**
```rust
pub enum ContractError {
    AlreadyInitialized = 1,   // Contract already initialized
    NotInitialized = 2,       // Contract not yet initialized
    InvalidAmount = 3,        // Amount is zero or negative
    InvalidFeeBps = 4,        // Fee exceeds 100% (10000 bps)
    AgentNotRegistered = 5,   // Agent not in registry
    RemittanceNotFound = 6,   // Remittance ID doesn't exist
    InvalidStatus = 7,        // Operation not allowed in current status
    Overflow = 8,             // Arithmetic overflow detected
    NoFeesToWithdraw = 9,     // No accumulated fees available
    InvalidAddress = 10,      // Address validation failed
}
```

**Error Handling Philosophy:**
- **Explicit over implicit** - Every error condition has a specific error code
- **Fail fast** - Validate inputs early and return errors immediately
- **No panics** - Never use `unwrap()` or `expect()` in production code
- **Checked arithmetic** - Use `checked_add()`, `checked_mul()`, etc. to prevent overflows
- **Descriptive codes** - Error numbers are stable and documented

**Usage Pattern:**
```rust
// Check for error condition
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}

// Use checked arithmetic
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?
    .checked_div(10000)
    .ok_or(ContractError::Overflow)?;
```

#### `events.rs` - Event Emission

**Purpose**: Defines and emits events for all important state changes.

**Why Events Matter:**
- **Transparency** - External observers can track contract activity
- **Indexing** - Off-chain services can build transaction history
- **Notifications** - Users and agents can be notified of state changes
- **Auditing** - Complete audit trail of all operations

**Event Functions:**
```rust
emit_remittance_created(env, id, sender, agent, amount, fee)
emit_remittance_completed(env, id, agent, payout_amount)
emit_remittance_cancelled(env, id, sender, refund_amount)
emit_agent_registered(env, agent)
emit_agent_removed(env, agent)
emit_fee_updated(env, fee_bps)
emit_fees_withdrawn(env, to, amount)
```

**Event Structure:**
- **Topic**: Short symbol identifying the event type (e.g., `"created"`, `"completed"`)
- **Data**: Tuple of relevant parameters (IDs, addresses, amounts)
- **Emission**: Called immediately after state change, before function returns

**Example:**
```rust
pub fn emit_remittance_created(
    env: &Env,
    remittance_id: u64,
    sender: Address,
    agent: Address,
    amount: i128,
    fee: i128,
) {
    env.events().publish(
        (symbol_short!("created"),),           // Topic
        (remittance_id, sender, agent, amount, fee),  // Data
    );
}
```

**Best Practices:**
- Emit events after successful state changes, not before
- Include all relevant information in event data
- Use consistent naming conventions for event topics
- Keep event data minimal to reduce gas costs

#### `validation.rs` - Input Validation

**Purpose**: Centralizes validation logic for inputs and addresses.

**Current Implementation:**
```rust
pub fn validate_address(address: &Address) -> Result<(), ContractError> {
    // Soroban SDK already validates Address type at runtime
    // This function serves as a placeholder for future validation logic
    // and makes validation requirements explicit in the code
    Ok(())
}
```

**Design Rationale:**
- **Explicit validation** - Makes it clear where validation is required
- **Future-proof** - Easy to add additional checks without changing call sites
- **Centralized logic** - Single place to update validation rules
- **Type safety** - Soroban's `Address` type already provides strong guarantees

**Future Enhancements:**
- Blacklist/whitelist checking
- Address format validation
- Sanity checks for specific address types (contract vs account)

**Usage in Contract:**
```rust
// Validate before critical operations
validate_address(&to)?;
token_client.transfer(&contract_address, &to, &amount);
```

#### `test.rs` - Test Suite

**Purpose**: Comprehensive test coverage for all contract functionality.

**Test Categories:**

1. **Initialization Tests**
   - `test_initialize()` - Successful initialization
   - `test_initialize_twice()` - Prevents double initialization
   - `test_initialize_invalid_fee()` - Rejects invalid fee configuration

2. **Agent Management Tests**
   - `test_register_agent()` - Register new agent
   - `test_remove_agent()` - Remove existing agent
   - Authorization checks for admin-only operations

3. **Remittance Lifecycle Tests**
   - `test_create_remittance()` - Create new remittance
   - `test_confirm_payout()` - Agent confirms payout
   - `test_cancel_remittance()` - Sender cancels remittance
   - Status transition validation

4. **Error Condition Tests**
   - Invalid amounts (zero, negative)
   - Unregistered agents
   - Invalid status transitions
   - Double confirmation/cancellation

5. **Fee Calculation Tests**
   - `test_fee_calculation()` - Verify fee math
   - `test_update_fee()` - Admin updates fee
   - Fee accumulation and withdrawal

6. **Authorization Tests**
   - `test_authorization_enforcement()` - Verify auth checks
   - Ensures only authorized parties can perform operations

7. **Event Tests**
   - `test_events_emitted()` - Verify events are published
   - Check event topics and data

8. **Integration Tests**
   - `test_multiple_remittances()` - Multiple concurrent remittances
   - `test_multiple_settlements_with_address_validation()` - Complex workflows

**Testing Utilities:**
```rust
// Create test token contract
fn create_token_contract(env: &Env, admin: &Address) -> StellarAssetClient

// Create SwiftRemit contract instance
fn create_swiftremit_contract(env: &Env) -> SwiftRemitContractClient

// Mock all authorizations for testing
env.mock_all_auths();
```

**Test Patterns:**
```rust
#[test]
fn test_happy_path() {
    let env = Env::default();
    env.mock_all_auths();  // Skip auth checks for testing
    
    // Setup
    let admin = Address::generate(&env);
    let contract = create_swiftremit_contract(&env);
    
    // Execute
    contract.initialize(&admin, &token.address, &250);
    
    // Assert
    assert_eq!(contract.get_platform_fee_bps(), 250);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_error_condition() {
    // Test that specific error is raised
    contract.create_remittance(&sender, &agent, &0);
}
```

### Storage Model Deep Dive

Understanding the storage model is crucial for working with Soroban contracts:

#### Instance Storage vs Persistent Storage

| Aspect | Instance Storage | Persistent Storage |
|--------|------------------|-------------------|
| **Cost** | Lower | Higher |
| **Access Speed** | Faster | Slower |
| **Lifetime** | Contract instance | Indefinite (with rent) |
| **Use Case** | Configuration | User data |
| **Examples** | Admin, fee config | Remittances, agents |

#### Storage Cost Optimization

**Best Practices:**
1. **Minimize writes** - Storage writes are expensive; batch updates when possible
2. **Use appropriate storage type** - Don't use persistent for temporary data
3. **Compact data structures** - Smaller types cost less to store
4. **Avoid redundant storage** - Don't store data that can be computed

**Example:**
```rust
// GOOD: Single storage write
let remittance = Remittance { /* ... */ };
set_remittance(&env, id, &remittance);

// BAD: Multiple storage writes for same data
set_remittance_sender(&env, id, &sender);
set_remittance_agent(&env, id, &agent);
set_remittance_amount(&env, id, &amount);
```

### Core Data Flows

Understanding how data flows through the contract helps you reason about the system:

#### 1. Create Remittance Flow

```
Sender → create_remittance(agent, amount)
  ↓
1. Authorization: sender.require_auth()
  ↓
2. Validation: amount > 0, agent registered
  ↓
3. Fee Calculation: fee = amount * fee_bps / 10000
  ↓
4. Token Transfer: USDC from sender to contract
  ↓
5. Generate ID: counter + 1
  ↓
6. Create Remittance: status = Pending
  ↓
7. Storage: Save remittance, update counter
  ↓
8. Event: emit_remittance_created()
  ↓
Return: remittance_id
```

**State Changes:**
- USDC balance: Sender ↓, Contract ↑
- Storage: New remittance record created
- Counter: Incremented by 1

#### 2. Confirm Payout Flow

```
Agent → confirm_payout(remittance_id)
  ↓
1. Load Remittance: get_remittance(id)
  ↓
2. Authorization: agent.require_auth()
  ↓
3. Status Check: status == Pending
  ↓
4. Address Validation: validate_address(agent)
  ↓
5. Calculate Payout: amount - fee
  ↓
6. Token Transfer: USDC from contract to agent
  ↓
7. Update Fees: accumulated_fees += fee
  ↓
8. Update Status: status = Completed
  ↓
9. Storage: Save updated remittance and fees
  ↓
10. Event: emit_remittance_completed()
  ↓
Return: Ok(())
```

**State Changes:**
- USDC balance: Contract ↓, Agent ↑
- Remittance status: Pending → Completed
- Accumulated fees: Increased by fee amount
- Storage: Remittance and fees updated

#### 3. Cancel Remittance Flow

```
Sender → cancel_remittance(remittance_id)
  ↓
1. Load Remittance: get_remittance(id)
  ↓
2. Authorization: sender.require_auth()
  ↓
3. Status Check: status == Pending
  ↓
4. Token Transfer: USDC from contract to sender (full amount)
  ↓
5. Update Status: status = Cancelled
  ↓
6. Storage: Save updated remittance
  ↓
7. Event: emit_remittance_cancelled()
  ↓
Return: Ok(())
```

**State Changes:**
- USDC balance: Contract ↓, Sender ↑
- Remittance status: Pending → Cancelled
- Storage: Remittance updated
- Note: No fees collected on cancellation

### Authorization Patterns

SwiftRemit implements role-based authorization with three roles:

#### 1. Admin Role

**Capabilities:**
- Initialize the contract
- Register and remove agents
- Update platform fees
- Withdraw accumulated fees

**Authorization Pattern:**
```rust
pub fn admin_function(env: Env, ...) -> Result<(), ContractError> {
    let admin = get_admin(&env)?;
    admin.require_auth();  // Verify caller is admin
    
    // Perform admin operation
    Ok(())
}
```

**Protected Functions:**
- `register_agent()`
- `remove_agent()`
- `update_fee()`
- `withdraw_fees()`

#### 2. Sender Role

**Capabilities:**
- Create remittances
- Cancel their own pending remittances

**Authorization Pattern:**
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    ...
) -> Result<u64, ContractError> {
    sender.require_auth();  // Verify caller is sender
    
    // Create remittance for sender
    Ok(remittance_id)
}
```

**Protected Functions:**
- `create_remittance()` - Sender must authorize
- `cancel_remittance()` - Only original sender can cancel

#### 3. Agent Role

**Capabilities:**
- Confirm payouts for remittances assigned to them

**Authorization Pattern:**
```rust
pub fn confirm_payout(
    env: Env,
    remittance_id: u64,
) -> Result<(), ContractError> {
    let remittance = get_remittance(&env, remittance_id)?;
    remittance.agent.require_auth();  // Verify caller is assigned agent
    
    // Process payout
    Ok(())
}
```

**Protected Functions:**
- `confirm_payout()` - Only assigned agent can confirm

#### Authorization Security

**Key Principles:**
1. **Always require auth** - Every state-changing function must check authorization
2. **Check early** - Verify authorization before expensive operations
3. **Use correct identity** - Verify the right party for each operation
4. **No bypasses** - Never skip authorization checks, even in "safe" scenarios

**Testing Authorization:**
```rust
#[test]
fn test_authorization_enforcement() {
    let env = Env::default();
    // Don't mock auths - test real authorization
    
    let result = contract.admin_function(&non_admin);
    // Should fail with authorization error
}
```

### Fee Calculation Mechanism

SwiftRemit uses a basis point (BPS) system for fee calculation:

#### Basis Points Explained

- **1 basis point (bps) = 0.01%**
- **100 bps = 1%**
- **10,000 bps = 100%**

**Examples:**
- 250 bps = 2.5% fee
- 500 bps = 5% fee
- 1000 bps = 10% fee

#### Fee Calculation Formula

```rust
fee = (amount * fee_bps) / 10000
```

**Example Calculation:**
```
Amount: 10,000 USDC (100,000,000 stroops)
Fee BPS: 250 (2.5%)

fee = (100,000,000 * 250) / 10,000
    = 25,000,000,000 / 10,000
    = 2,500,000 stroops
    = 250 USDC

Payout to agent = 10,000 - 250 = 9,750 USDC
```

#### Fee Calculation Implementation

```rust
let fee_bps = get_platform_fee_bps(&env)?;

// Use checked arithmetic to prevent overflow
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?
    .checked_div(10000)
    .ok_or(ContractError::Overflow)?;

let payout_amount = amount
    .checked_sub(fee)
    .ok_or(ContractError::Overflow)?;
```

#### Fee Constraints

**Validation:**
```rust
if fee_bps > 10000 {
    return Err(ContractError::InvalidFeeBps);
}
```

- **Minimum**: 0 bps (0% - no fees)
- **Maximum**: 10,000 bps (100% - all fees, effectively blocking transactions)
- **Typical**: 100-500 bps (1-5%)

#### Fee Accumulation and Withdrawal

**Accumulation:**
- Fees are accumulated in instance storage (`AccumulatedFees`)
- Updated on each successful `confirm_payout()`
- Tracked separately from contract's USDC balance

**Withdrawal:**
```rust
pub fn withdraw_fees(env: Env, to: Address) -> Result<(), ContractError> {
    let admin = get_admin(&env)?;
    admin.require_auth();
    
    let fees = get_accumulated_fees(&env)?;
    
    if fees <= 0 {
        return Err(ContractError::NoFeesToWithdraw);
    }
    
    // Transfer fees to recipient
    token_client.transfer(&contract_address, &to, &fees);
    
    // Reset accumulated fees to zero
    set_accumulated_fees(&env, 0);
    
    emit_fees_withdrawn(&env, to, fees);
    Ok(())
}
```

**Fee Lifecycle:**
1. Remittance created with calculated fee
2. Payout confirmed → fee added to accumulated total
3. Admin withdraws fees → accumulated total reset to zero
4. Cycle repeats

### Working with the Codebase

#### Adding a New Feature

When adding new functionality, follow this pattern:

1. **Define error types** (if needed) in `errors.rs`
2. **Add data structures** (if needed) in `types.rs`
3. **Create storage functions** (if needed) in `storage.rs`
4. **Implement validation** (if needed) in `validation.rs`
5. **Add event emission** (if needed) in `events.rs`
6. **Implement business logic** in `lib.rs`
7. **Write comprehensive tests** in `test.rs`

#### Modifying Existing Features

1. **Understand the current flow** - Trace through the code
2. **Check authorization** - Ensure changes don't bypass security
3. **Update tests** - Modify or add tests for new behavior
4. **Update events** - Add new event data if state changes
5. **Consider storage** - Minimize new storage operations
6. **Handle errors** - Add new error types if needed

#### Common Patterns

**Reading from storage:**
```rust
let value = get_something(&env)?;  // Returns Result
```

**Writing to storage:**
```rust
set_something(&env, &value);  // No return value
```

**Requiring authorization:**
```rust
address.require_auth();  // Panics if not authorized
```

**Checked arithmetic:**
```rust
let result = a.checked_add(b).ok_or(ContractError::Overflow)?;
```

**Emitting events:**
```rust
emit_something_happened(&env, param1, param2);
```

### Next Steps

Now that you understand the project architecture:

1. **Explore the code** - Read through the source files in detail
2. **Run the tests** - See the contract in action: `cargo test`
3. **Review the workflow** - Learn how to build and deploy in the next section
4. **Start contributing** - Check out [good first issues](https://github.com/yourusername/swiftremit/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

---

## Development Workflow

This section covers the day-to-day development workflow for building, testing, and validating your changes to SwiftRemit. Understanding these processes is essential for effective contribution.

### Overview

The typical development cycle follows these steps:

1. **Build** - Compile the contract to WebAssembly
2. **Test** - Run the test suite to verify functionality
3. **Format** - Apply consistent code formatting
4. **Lint** - Check for common issues and style violations
5. **Optimize** - Create production-ready WASM binary
6. **Test on Testnet** - Validate changes in a live environment (for contract changes)

We provide both **Makefile commands** (recommended for convenience) and **manual cargo commands** (for understanding and customization).

### Building the Contract

Building compiles the Rust code into a WebAssembly (WASM) binary that can be deployed to the Stellar blockchain.

#### Using Makefile (Recommended)

```bash
make build
```

This command:
- Compiles the contract for the `wasm32-unknown-unknown` target
- Creates a release build with optimizations
- Outputs the WASM file to `target/wasm32-unknown-unknown/release/swiftremit.wasm`

#### Manual Cargo Command

```bash
cargo build --target wasm32-unknown-unknown --release
```

**Explanation:**
- `--target wasm32-unknown-unknown` - Compiles to WebAssembly instead of native code
- `--release` - Enables optimizations for smaller, faster code

#### Build Output

After a successful build, you'll find the WASM binary at:
```
target/wasm32-unknown-unknown/release/swiftremit.wasm
```

**Typical build time:** 10-30 seconds (first build may take longer)

#### Checking Code Without Building

For faster feedback during development, you can check for compilation errors without producing a binary:

```bash
# Using Makefile
make check

# Using Cargo
cargo check --target wasm32-unknown-unknown
```

This is much faster than a full build and useful for quick iteration.

### Running Tests

Testing is critical for ensuring correctness and preventing regressions. Always run tests before submitting a PR.

#### Running All Tests

```bash
# Using Makefile
make test

# Using Cargo
cargo test
```

This runs the entire test suite defined in `src/test.rs` and any other test files.

**Expected output:**
```
Running tests...
running 25 tests
test test_initialize ... ok
test test_create_remittance ... ok
test test_confirm_payout ... ok
...
test result: ok. 25 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
✅ Tests passed
```

#### Running Specific Tests

To run a single test or a subset of tests:

```bash
# Run a specific test by name
cargo test test_create_remittance

# Run all tests matching a pattern
cargo test remittance

# Run tests in a specific module
cargo test storage::
```

#### Running Tests with Output

By default, `cargo test` captures output from passing tests. To see `println!` statements and other output:

```bash
# Using Makefile
make test-verbose

# Using Cargo
cargo test -- --nocapture
```

This is useful for debugging test failures or understanding test behavior.

#### Understanding Test Output

**Successful test:**
```
test test_create_remittance ... ok
```

**Failed test:**
```
test test_invalid_amount ... FAILED

failures:

---- test_invalid_amount stdout ----
thread 'test_invalid_amount' panicked at 'assertion failed: result.is_err()'
```

**Test with expected panic:**
```
test test_double_initialization ... ok
```
(Tests marked with `#[should_panic]` are expected to panic)

#### Test Best Practices

- **Run tests frequently** - After every significant change
- **Fix failing tests immediately** - Don't let them accumulate
- **Add tests for new features** - Every new function should have tests
- **Test error conditions** - Don't just test the happy path
- **Keep tests fast** - Slow tests discourage frequent testing

### Code Formatting

Consistent code formatting makes the codebase easier to read and maintain. We use `rustfmt` to automatically format code according to Rust style guidelines.

#### Formatting Your Code

```bash
# Using Makefile
make fmt

# Using Cargo
cargo fmt
```

This command:
- Formats all Rust files in the project
- Applies consistent indentation, spacing, and line breaks
- Follows the official Rust style guide
- Modifies files in place

**Always format your code before committing!**

#### Checking Format Without Modifying

To check if your code is properly formatted without making changes:

```bash
cargo fmt -- --check
```

This is useful in CI/CD pipelines to enforce formatting standards.

#### What Gets Formatted

`rustfmt` handles:
- Indentation (4 spaces)
- Line length (default 100 characters)
- Spacing around operators and punctuation
- Alignment of struct fields and function parameters
- Import organization

**Example:**

Before formatting:
```rust
pub fn create_remittance(env:Env,sender:Address,agent:Address,amount:i128)->Result<u64,ContractError>{
sender.require_auth();if amount<=0{return Err(ContractError::InvalidAmount);}
// ...
}
```

After formatting:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    sender.require_auth();
    
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    // ...
}
```

### Linting with Clippy

Clippy is Rust's official linter that catches common mistakes, suggests improvements, and enforces best practices.

#### Running Clippy

```bash
# Using Makefile
make lint

# Using Cargo
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

**Important:** Our Makefile configuration treats all warnings as errors (`-D warnings`). This ensures high code quality and prevents warnings from accumulating.

#### What Clippy Checks

Clippy analyzes your code for:
- **Common mistakes** - Incorrect patterns, logic errors
- **Performance issues** - Inefficient code that can be optimized
- **Style violations** - Non-idiomatic Rust code
- **Complexity** - Overly complex functions that should be simplified
- **Correctness** - Potential bugs and edge cases

**Example warnings:**

```
warning: this expression creates a reference which is immediately dereferenced
  --> src/lib.rs:45:20
   |
45 |     let value = &(*data);
   |                 ^^^^^^^^ help: try this: `data`

warning: you seem to be trying to use `match` for destructuring a single pattern
  --> src/lib.rs:67:5
   |
67 | /     match status {
68 | |         RemittanceStatus::Pending => { /* ... */ }
69 | |         _ => return Err(ContractError::InvalidStatus),
70 | |     }
   | |_____^ help: try this: `if let RemittanceStatus::Pending = status { /* ... */ }`
```

#### Fixing Clippy Warnings

1. **Read the warning carefully** - Clippy usually explains the issue and suggests a fix
2. **Apply the suggested fix** - Most suggestions improve code quality
3. **Understand why** - Learn from each warning to avoid similar issues
4. **Re-run clippy** - Verify the warning is resolved

#### Allowing Specific Warnings

In rare cases, you may need to allow a specific warning:

```rust
#[allow(clippy::too_many_arguments)]
pub fn complex_function(/* many parameters */) {
    // ...
}
```

**Use sparingly!** Only allow warnings when you have a good reason and document why.

### Contract Optimization

Optimization reduces the size of the WASM binary, which is important for:
- **Lower deployment costs** - Smaller contracts cost less to deploy
- **Faster execution** - Optimized code runs more efficiently
- **Gas savings** - Smaller contracts use less gas

#### Optimizing the Contract

```bash
# Using Makefile
make optimize

# Using Cargo (manual process)
cargo build --target wasm32-unknown-unknown --release
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/swiftremit.wasm
```

The `make optimize` command:
1. Builds the contract in release mode
2. Runs the Soroban optimizer on the WASM binary
3. Creates an optimized binary: `swiftremit.optimized.wasm`

#### Optimization Results

**Before optimization:**
```
swiftremit.wasm: ~150 KB
```

**After optimization:**
```
swiftremit.optimized.wasm: ~50 KB (typical reduction: 60-70%)
```

#### When to Optimize

- **Before deployment** - Always deploy the optimized version
- **For size checks** - When monitoring contract size
- **Not during development** - Optimization is slow; use unoptimized builds for testing

#### Checking Contract Size

To see the size of your optimized contract:

```bash
make size
```

This displays the file size of the optimized WASM binary.

### Testing on Testnet

Before submitting a PR with contract changes, you should test your changes on the Stellar testnet to ensure they work in a real blockchain environment.

#### Why Test on Testnet?

- **Real environment** - Tests actual blockchain interactions
- **Network conditions** - Validates behavior under real network conditions
- **Integration testing** - Ensures compatibility with other contracts and tools
- **Confidence** - Provides assurance before mainnet deployment

#### Testnet Testing Workflow

##### 1. Set Up Testnet (One-Time Setup)

```bash
make setup-testnet
```

This configures the Soroban CLI to connect to the Stellar testnet.

##### 2. Create a Deployer Identity (One-Time Setup)

```bash
make create-identity
```

This generates a keypair for deploying contracts. The address will be displayed:
```
Deployer address: GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

##### 3. Fund Your Deployer Account (One-Time Setup)

```bash
make fund-deployer
```

This requests testnet XLM from the Stellar friendbot to pay for deployment and transaction fees.

##### 4. Deploy Your Contract

```bash
make deploy
```

This:
- Optimizes the contract
- Deploys it to testnet
- Saves the contract ID to `.contract-id`

**Output:**
```
Deploying contract to testnet...
Contract deployed at: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
✅ Deployment complete
```

##### 5. Initialize the Contract

```bash
make initialize CONTRACT_ID=CXXX... USDC_TOKEN=CYYY... FEE_BPS=250
```

Replace:
- `CONTRACT_ID` - Your deployed contract ID (from `.contract-id`)
- `USDC_TOKEN` - Testnet USDC token contract address
- `FEE_BPS` - Platform fee in basis points (e.g., 250 = 2.5%)

##### 6. Test Contract Functions

Use the Soroban CLI to invoke contract functions:

```bash
# Register an agent
soroban contract invoke \
  --id YOUR_CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  register_agent \
  --agent GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

# Create a remittance
soroban contract invoke \
  --id YOUR_CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  create_remittance \
  --sender GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --agent GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX \
  --amount 1000000000

# Query remittance
soroban contract invoke \
  --id YOUR_CONTRACT_ID \
  --network testnet \
  -- \
  get_remittance \
  --remittance_id 1
```

##### 7. Verify Results

- Check that functions execute without errors
- Verify state changes are correct
- Test error conditions
- Confirm events are emitted

#### Full Deployment Flow

For a complete deployment from scratch:

```bash
make deploy-full
```

This runs all setup steps and deploys the contract in one command.

#### Testnet Best Practices

- **Test all changes** - Don't skip testnet testing for "small" changes
- **Test error cases** - Try invalid inputs and unauthorized operations
- **Clean up** - Remove test data when done (if applicable)
- **Document issues** - Note any unexpected behavior for discussion
- **Share contract ID** - Include testnet contract ID in PR for reviewer testing

#### Testnet Resources

- **Testnet Explorer**: [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet)
- **Friendbot** (for funding): [laboratory.stellar.org](https://laboratory.stellar.org/#account-creator?network=test)
- **Testnet RPC**: `https://soroban-testnet.stellar.org:443`

### Writing New Tests

When adding new features or fixing bugs, you should write tests to verify the behavior.

#### Test Structure

Tests in SwiftRemit follow this pattern:

```rust
#[test]
fn test_descriptive_name() {
    // 1. Setup - Create test environment and contracts
    let env = Env::default();
    env.mock_all_auths();  // Skip authorization checks for testing
    
    let admin = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    
    // 2. Initialize - Set up contract state
    contract.initialize(&admin, &token.address, &250);
    
    // 3. Execute - Perform the action being tested
    let result = contract.some_function(&param1, &param2);
    
    // 4. Assert - Verify the expected outcome
    assert_eq!(result, expected_value);
    assert_eq!(contract.get_state(), expected_state);
}
```

#### Types of Tests to Write

##### 1. Happy Path Tests

Test that functions work correctly with valid inputs:

```rust
#[test]
fn test_create_remittance_success() {
    // Setup
    let env = Env::default();
    env.mock_all_auths();
    
    // ... initialize contract ...
    
    // Execute
    let remittance_id = contract.create_remittance(
        &sender,
        &agent,
        &1000000000,  // 100 USDC
    );
    
    // Assert
    assert_eq!(remittance_id, 1);
    
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.sender, sender);
    assert_eq!(remittance.agent, agent);
    assert_eq!(remittance.amount, 1000000000);
    assert_eq!(remittance.status, RemittanceStatus::Pending);
}
```

##### 2. Error Condition Tests

Test that functions properly reject invalid inputs:

```rust
#[test]
#[should_panic(expected = "Error(Contract, #3)")]  // InvalidAmount error
fn test_create_remittance_zero_amount() {
    // Setup
    let env = Env::default();
    env.mock_all_auths();
    
    // ... initialize contract ...
    
    // Execute - should panic with InvalidAmount error
    contract.create_remittance(&sender, &agent, &0);
}
```

##### 3. State Transition Tests

Test that state changes occur correctly:

```rust
#[test]
fn test_remittance_lifecycle() {
    // Setup
    let env = Env::default();
    env.mock_all_auths();
    
    // ... initialize contract ...
    
    // Create remittance
    let id = contract.create_remittance(&sender, &agent, &1000000000);
    assert_eq!(contract.get_remittance(&id).status, RemittanceStatus::Pending);
    
    // Confirm payout
    contract.confirm_payout(&id);
    assert_eq!(contract.get_remittance(&id).status, RemittanceStatus::Completed);
    
    // Verify cannot confirm again
    let result = std::panic::catch_unwind(|| {
        contract.confirm_payout(&id);
    });
    assert!(result.is_err());
}
```

##### 4. Authorization Tests

Test that authorization is properly enforced:

```rust
#[test]
fn test_only_admin_can_register_agent() {
    let env = Env::default();
    // Don't mock auths - test real authorization
    
    let admin = Address::generate(&env);
    let non_admin = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // ... initialize contract with admin ...
    
    // This should work (admin calling)
    contract.register_agent(&agent);
    
    // This should fail (non-admin calling)
    // Note: In real tests, this would require proper auth setup
}
```

##### 5. Edge Case Tests

Test boundary conditions and unusual scenarios:

```rust
#[test]
fn test_maximum_fee_calculation() {
    // Test with maximum allowed fee (10000 bps = 100%)
    // Test with very large amounts
    // Test with minimum amounts (1 stroop)
}

#[test]
fn test_multiple_concurrent_remittances() {
    // Create multiple remittances
    // Verify they don't interfere with each other
    // Check that IDs are unique and sequential
}
```

#### Test Utilities

Use the provided test utilities in `test.rs`:

```rust
// Create a test token contract
let token = create_token_contract(&env, &admin);

// Create SwiftRemit contract
let contract = create_swiftremit_contract(&env);

// Mock all authorizations (for happy path tests)
env.mock_all_auths();

// Generate test addresses
let address = Address::generate(&env);
```

#### Running Your New Tests

After writing tests:

```bash
# Run all tests
make test

# Run your specific test
cargo test test_your_new_test

# Run with output to debug
cargo test test_your_new_test -- --nocapture
```

#### Test Coverage Guidelines

When adding new functionality:
- **Minimum**: Test the happy path and at least one error condition
- **Recommended**: Test all error conditions and edge cases
- **Ideal**: Test all code paths, including authorization and state transitions

### Pre-Commit Checklist

Before committing your changes, run through this checklist:

```bash
# 1. Build the contract
make build

# 2. Run all tests
make test

# 3. Format code
make fmt

# 4. Lint code
make lint

# 5. (Optional) Optimize and check size
make optimize
make size

# 6. (For contract changes) Test on testnet
make deploy
# ... test contract functions ...
```

**All checks must pass before submitting a PR!**

### Continuous Integration

Our CI/CD pipeline automatically runs these checks on every PR:
- ✅ Build succeeds
- ✅ All tests pass
- ✅ Code is formatted (`cargo fmt --check`)
- ✅ No clippy warnings (`cargo clippy -- -D warnings`)
- ✅ Contract optimizes successfully

If any check fails, the PR cannot be merged. Fix issues locally and push updates.

### Development Tips

#### Fast Iteration

For quick development cycles:

```bash
# Check for errors (fast)
make check

# Run specific test (fast)
cargo test test_name

# Format and lint (before commit)
make fmt && make lint
```

#### Debugging Tests

To debug failing tests:

```bash
# Run with output
cargo test test_name -- --nocapture

# Add println! statements in your test
println!("Debug: value = {:?}", value);

# Use the Rust debugger (with IDE support)
# Set breakpoints in your IDE and run tests in debug mode
```

#### Watching for Changes

For automatic rebuilds on file changes:

```bash
make watch
```

This requires `cargo-watch`:
```bash
cargo install cargo-watch
```

#### Generating Documentation

To generate and view Rust documentation:

```bash
make docs
```

This opens the generated documentation in your browser.

### Common Issues and Solutions

#### Issue: Build fails with "error: linker `cc` not found"

**Solution**: Install build tools
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS (install Xcode Command Line Tools)
xcode-select --install
```

#### Issue: Tests fail with "error: test failed, to rerun pass '--lib'"

**Solution**: Run tests with verbose output to see the actual error
```bash
cargo test -- --nocapture
```

#### Issue: Clippy warnings about unused variables

**Solution**: Prefix unused variables with underscore
```rust
// Before
let result = some_function();

// After
let _result = some_function();
```

#### Issue: Contract size is too large

**Solution**: 
1. Run `make optimize` to ensure optimization is applied
2. Review code for unnecessary dependencies
3. Consider refactoring to reduce code size

#### Issue: Testnet deployment fails with "account not found"

**Solution**: Fund your deployer account
```bash
make fund-deployer
```

### Next Steps

Now that you understand the development workflow:
1. Review [Coding Standards](#coding-standards) to learn our code quality expectations
2. Read [Testing Guidelines](#testing-guidelines) for comprehensive testing practices
3. Check [Security Considerations](#security-considerations) for smart contract security
4. Start contributing with a [good first issue](https://github.com/yourusername/swiftremit/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

---

## Coding Standards

This section defines the coding standards and best practices for contributing to SwiftRemit. Following these standards ensures code consistency, maintainability, and quality across the project.

### Rust Coding Standards

#### Naming Conventions

Follow Rust's standard naming conventions:

**Functions and Variables** - Use `snake_case`:
```rust
// Good
fn create_remittance(sender: Address, amount: i128) -> Result<u64, ContractError> { }
let remittance_id = 1;
let platform_fee = calculate_fee(amount);

// Bad
fn CreateRemittance(Sender: Address, Amount: i128) -> Result<u64, ContractError> { }
let RemittanceID = 1;
let platformFee = calculate_fee(amount);
```

**Types, Structs, and Enums** - Use `PascalCase`:
```rust
// Good
pub struct Remittance { }
pub enum RemittanceStatus { }
pub type ContractResult<T> = Result<T, ContractError>;

// Bad
pub struct remittance { }
pub enum remittance_status { }
pub type contract_result<T> = Result<T, ContractError>;
```

**Constants** - Use `SCREAMING_SNAKE_CASE`:
```rust
// Good
const MAX_FEE_BPS: i128 = 10000;
const MIN_REMITTANCE_AMOUNT: i128 = 1;

// Bad
const maxFeeBps: i128 = 10000;
const min_remittance_amount: i128 = 1;
```

**Module Names** - Use `snake_case`:
```rust
// Good
mod storage;
mod validation;
mod error_handling;

// Bad
mod Storage;
mod Validation;
mod ErrorHandling;
```

#### Descriptive Variable Names

Use clear, descriptive names that convey meaning:

```rust
// Good
let remittance_counter = get_remittance_counter(&env)?;
let accumulated_fees = get_accumulated_fees(&env)?;
let payout_amount = amount.checked_sub(fee).ok_or(ContractError::Overflow)?;

// Bad
let rc = get_remittance_counter(&env)?;
let af = get_accumulated_fees(&env)?;
let pa = amount.checked_sub(fee).ok_or(ContractError::Overflow)?;
```

**Exceptions**: Short names are acceptable for:
- Loop iterators: `i`, `j`, `idx`
- Generic type parameters: `T`, `E`, `K`, `V`
- Very short scopes (1-2 lines)

#### Type Annotations

**Public APIs** - Always use explicit types:
```rust
// Good
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // ...
}

// Bad - implicit return type
pub fn create_remittance(env: Env, sender: Address, agent: Address, amount: i128) {
    // ...
}
```

**Internal Functions** - Prefer explicit types, but type inference is acceptable for obvious cases:
```rust
// Good - explicit
let remittance_id: u64 = counter + 1;

// Acceptable - inference is clear
let remittance_id = counter + 1;  // counter is u64, so result is u64

// Bad - unclear inference
let result = some_complex_function();  // What type is this?
```

#### Error Handling

**Use `Result<T, E>` for fallible operations**:
```rust
// Good
pub fn get_remittance(env: &Env, id: u64) -> Result<Remittance, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Remittance(id))
        .ok_or(ContractError::RemittanceNotFound)
}

// Bad - using Option when errors should be explicit
pub fn get_remittance(env: &Env, id: u64) -> Option<Remittance> {
    env.storage().persistent().get(&DataKey::Remittance(id))
}
```

**Never use `unwrap()` or `expect()` in production code**:
```rust
// Good
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?;

// Bad - will panic on overflow
let fee = amount.checked_mul(fee_bps as i128).unwrap();

// Bad - even with a message, panics are not acceptable
let fee = amount.checked_mul(fee_bps as i128).expect("Fee calculation overflow");
```

**Exception**: `unwrap()` is acceptable in test code where panics are expected:
```rust
#[test]
fn test_something() {
    let value = some_function().unwrap();  // OK in tests
    assert_eq!(value, expected);
}
```

**Use the `?` operator for error propagation**:
```rust
// Good
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let remittance = get_remittance(&env, remittance_id)?;
    let admin = get_admin(&env)?;
    // ...
    Ok(())
}

// Bad - verbose and error-prone
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let remittance = match get_remittance(&env, remittance_id) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    // ...
}
```

**Provide descriptive error messages through error types**:
```rust
// Good - specific error types
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}
if !is_agent_registered(&env, &agent) {
    return Err(ContractError::AgentNotRegistered);
}

// Bad - generic errors
if amount <= 0 {
    return Err(ContractError::InvalidInput);
}
```

#### Code Organization

**Keep functions focused and single-purpose**:
```rust
// Good - single responsibility
fn calculate_fee(amount: i128, fee_bps: i128) -> Result<i128, ContractError> {
    amount
        .checked_mul(fee_bps)
        .ok_or(ContractError::Overflow)?
        .checked_div(10000)
        .ok_or(ContractError::Overflow)
}

fn validate_remittance_amount(amount: i128) -> Result<(), ContractError> {
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    Ok(())
}

// Bad - doing too much
fn process_remittance(env: Env, sender: Address, agent: Address, amount: i128) -> Result<u64, ContractError> {
    // Validates, calculates fees, transfers tokens, updates storage, emits events
    // This should be broken into smaller functions
}
```

**Extract complex logic into helper functions**:
```rust
// Good
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    sender.require_auth();
    
    validate_remittance_creation(&env, &agent, amount)?;
    
    let fee = calculate_fee(amount, get_platform_fee_bps(&env)?)?;
    let remittance_id = generate_remittance_id(&env)?;
    
    transfer_to_escrow(&env, &sender, amount)?;
    store_remittance(&env, remittance_id, &sender, &agent, amount, fee)?;
    emit_remittance_created(&env, remittance_id, sender, agent, amount, fee);
    
    Ok(remittance_id)
}

// Bad - all logic inline
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    sender.require_auth();
    
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    
    if !env.storage().persistent().has(&DataKey::AgentRegistered(agent.clone())) {
        return Err(ContractError::AgentNotRegistered);
    }
    
    let fee_bps = env.storage().instance().get(&DataKey::PlatformFeeBps).ok_or(ContractError::NotInitialized)?;
    let fee = amount.checked_mul(fee_bps as i128).ok_or(ContractError::Overflow)?.checked_div(10000).ok_or(ContractError::Overflow)?;
    
    // ... more inline logic ...
}
```

**Group related functionality**:
```rust
// Good - related functions grouped together
// Agent management functions
pub fn register_agent(env: Env, agent: Address) -> Result<(), ContractError> { }
pub fn remove_agent(env: Env, agent: Address) -> Result<(), ContractError> { }
pub fn is_agent_registered(env: &Env, agent: &Address) -> bool { }

// Remittance lifecycle functions
pub fn create_remittance(...) -> Result<u64, ContractError> { }
pub fn confirm_payout(...) -> Result<(), ContractError> { }
pub fn cancel_remittance(...) -> Result<(), ContractError> { }
```

**Maintain separation of concerns**:
- `lib.rs` - Business logic and contract interface
- `storage.rs` - Storage operations only
- `validation.rs` - Input validation only
- `errors.rs` - Error definitions only
- `events.rs` - Event emission only
- `types.rs` - Data structures only

### Soroban-Specific Best Practices

#### Storage Patterns

**Use appropriate storage types**:

```rust
// Good - instance storage for configuration
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

// Good - persistent storage for user data
pub fn set_remittance(env: &Env, id: u64, remittance: &Remittance) {
    env.storage().persistent().set(&DataKey::Remittance(id), remittance);
}

// Bad - persistent storage for configuration (unnecessary cost)
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&DataKey::Admin, admin);
}
```

**Storage Type Guidelines**:
- **Instance Storage**: Configuration, counters, singleton values
- **Persistent Storage**: User data, transactions, registrations

**Minimize storage operations**:
```rust
// Good - single write
let remittance = Remittance {
    id,
    sender: sender.clone(),
    agent: agent.clone(),
    amount,
    fee,
    status: RemittanceStatus::Pending,
};
set_remittance(&env, id, &remittance);

// Bad - multiple writes for same data
set_remittance_id(&env, id);
set_remittance_sender(&env, id, &sender);
set_remittance_agent(&env, id, &agent);
set_remittance_amount(&env, id, amount);
set_remittance_fee(&env, id, fee);
set_remittance_status(&env, id, RemittanceStatus::Pending);
```

**Batch related updates**:
```rust
// Good - update related data together
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let mut remittance = get_remittance(&env, remittance_id)?;
    remittance.status = RemittanceStatus::Completed;
    set_remittance(&env, remittance_id, &remittance);
    
    let accumulated = get_accumulated_fees(&env)?;
    set_accumulated_fees(&env, accumulated + remittance.fee);
    
    Ok(())
}
```

#### Authorization

**Always require authorization for state-changing operations**:
```rust
// Good
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    sender.require_auth();  // Authorization check first
    
    // ... rest of function ...
}

// Bad - missing authorization
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // No authorization check!
    // ... function logic ...
}
```

**Verify the correct party is authorized**:
```rust
// Good - verify the specific party
pub fn cancel_remittance(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let remittance = get_remittance(&env, remittance_id)?;
    remittance.sender.require_auth();  // Only the sender can cancel
    
    // ... cancellation logic ...
}

// Bad - wrong party authorized
pub fn cancel_remittance(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let admin = get_admin(&env)?;
    admin.require_auth();  // Wrong! Admin shouldn't cancel user remittances
    
    // ... cancellation logic ...
}
```

**Check authorization early**:
```rust
// Good - auth check before expensive operations
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let remittance = get_remittance(&env, remittance_id)?;
    remittance.agent.require_auth();  // Check auth early
    
    // ... expensive operations ...
}

// Bad - auth check after expensive operations
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    let remittance = get_remittance(&env, remittance_id)?;
    
    // ... expensive operations ...
    
    remittance.agent.require_auth();  // Too late!
}
```

#### Gas Optimization

**Use checked arithmetic to prevent overflows**:
```rust
// Good - checked arithmetic
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?
    .checked_div(10000)
    .ok_or(ContractError::Overflow)?;

// Bad - unchecked arithmetic (can panic)
let fee = (amount * fee_bps as i128) / 10000;
```

**Avoid unnecessary clones**:
```rust
// Good - borrow when possible
pub fn validate_agent(env: &Env, agent: &Address) -> Result<(), ContractError> {
    if !is_agent_registered(env, agent) {
        return Err(ContractError::AgentNotRegistered);
    }
    Ok(())
}

// Bad - unnecessary clone
pub fn validate_agent(env: &Env, agent: &Address) -> Result<(), ContractError> {
    let agent_clone = agent.clone();  // Unnecessary
    if !is_agent_registered(env, &agent_clone) {
        return Err(ContractError::AgentNotRegistered);
    }
    Ok(())
}
```

**Minimize storage reads**:
```rust
// Good - read once, use multiple times
pub fn process_remittance(env: &Env, id: u64) -> Result<(), ContractError> {
    let remittance = get_remittance(env, id)?;
    
    validate_remittance_status(&remittance)?;
    calculate_payout(&remittance)?;
    emit_event(env, &remittance);
    
    Ok(())
}

// Bad - multiple reads of same data
pub fn process_remittance(env: &Env, id: u64) -> Result<(), ContractError> {
    validate_remittance_status(&get_remittance(env, id)?)?;
    calculate_payout(&get_remittance(env, id)?)?;
    emit_event(env, &get_remittance(env, id)?);
    
    Ok(())
}
```

**Use events for important state changes**:
```rust
// Good - emit events for transparency
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    // ... payout logic ...
    
    emit_remittance_completed(&env, remittance_id, agent, payout_amount);
    
    Ok(())
}

// Bad - no event emission
pub fn confirm_payout(env: Env, remittance_id: u64) -> Result<(), ContractError> {
    // ... payout logic ...
    // No event! External observers can't track this
    Ok(())
}
```

**Follow the contract's existing error handling patterns**:
```rust
// Good - consistent with existing patterns
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}

if !is_agent_registered(&env, &agent) {
    return Err(ContractError::AgentNotRegistered);
}

// Bad - inconsistent error handling
if amount <= 0 {
    panic!("Invalid amount");  // Don't panic!
}

if !is_agent_registered(&env, &agent) {
    return Err(ContractError::InvalidInput);  // Too generic!
}
```

### Code Formatting

**Requirement**: All code must be formatted with `rustfmt` before committing.

```bash
# Format your code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

**What rustfmt handles**:
- Indentation (4 spaces)
- Line length (100 characters default)
- Spacing around operators
- Alignment of struct fields
- Import organization

**Configuration**: We use the default `rustfmt` configuration. Do not override formatting rules without team discussion.

### Linting

**Requirement**: All code must pass `clippy` with zero warnings.

```bash
# Run clippy
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

**Our standard**: We treat all clippy warnings as errors (`-D warnings`). This ensures:
- High code quality
- Consistent style
- Early detection of potential bugs
- No accumulation of technical debt

**Addressing clippy warnings**:
1. Read the warning and understand the issue
2. Apply the suggested fix (most suggestions improve code)
3. If you must allow a warning, document why:
   ```rust
   #[allow(clippy::too_many_arguments)]  // Complex function requires many params
   pub fn complex_function(...) { }
   ```

**Common clippy warnings to avoid**:
- Unnecessary clones
- Inefficient string operations
- Overly complex expressions
- Non-idiomatic patterns
- Potential performance issues

### Documentation Standards

**Requirement**: All public functions and modules must have doc comments.

#### Public Functions

```rust
/// Creates a new remittance and transfers USDC to escrow.
///
/// The sender must authorize this transaction. The specified agent must be
/// registered before creating a remittance. A platform fee is calculated
/// based on the configured fee percentage and deducted upon payout.
///
/// # Arguments
///
/// * `sender` - The address sending the remittance
/// * `agent` - The registered agent who will handle the payout
/// * `amount` - The amount in USDC stroops (1 USDC = 10,000,000 stroops)
///
/// # Returns
///
/// The unique remittance ID that can be used to query or manage the remittance
///
/// # Errors
///
/// * `InvalidAmount` - If amount is zero or negative
/// * `AgentNotRegistered` - If the specified agent is not registered
/// * `NotInitialized` - If the contract has not been initialized
/// * `Overflow` - If fee calculation causes arithmetic overflow
///
/// # Examples
///
/// ```ignore
/// let remittance_id = contract.create_remittance(
///     &sender_address,
///     &agent_address,
///     &100_000_000,  // 10 USDC
/// );
/// ```
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // Implementation
}
```

**Required elements**:
- Brief description (first line)
- Detailed explanation (if needed)
- `# Arguments` - Description of each parameter
- `# Returns` - Description of return value
- `# Errors` - All possible error conditions
- `# Examples` - Usage example for complex functions

#### Structs and Enums

```rust
/// Represents a remittance transaction in the system.
///
/// A remittance is created when a sender transfers USDC to the contract
/// for payout by a registered agent. The remittance tracks the sender,
/// agent, amount, fee, and current status.
#[contracttype]
pub struct Remittance {
    /// Unique identifier for this remittance
    pub id: u64,
    
    /// Address of the sender who created the remittance
    pub sender: Address,
    
    /// Address of the agent responsible for payout
    pub agent: Address,
    
    /// Total amount in USDC stroops
    pub amount: i128,
    
    /// Platform fee in USDC stroops (deducted on payout)
    pub fee: i128,
    
    /// Current status of the remittance
    pub status: RemittanceStatus,
}

/// Represents the lifecycle status of a remittance.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    /// Remittance created, awaiting payout confirmation
    Pending,
    
    /// Payout confirmed, funds transferred to agent
    Completed,
    
    /// Remittance cancelled, funds refunded to sender
    Cancelled,
}
```

#### Modules

```rust
//! Storage management for the SwiftRemit contract.
//!
//! This module provides functions for reading and writing contract state
//! to Soroban storage. It uses both instance storage (for configuration)
//! and persistent storage (for user data).
//!
//! # Storage Keys
//!
//! - Instance: Admin, USDC token, fee config, counters, accumulated fees
//! - Persistent: Remittances (by ID), agent registrations (by address)
```

#### Inline Comments

Use inline comments sparingly for complex logic:

```rust
// Calculate fee using basis points (1 bps = 0.01%)
// Example: 10,000 USDC * 250 bps / 10,000 = 250 USDC (2.5%)
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?
    .checked_div(10000)
    .ok_or(ContractError::Overflow)?;
```

**When to use inline comments**:
- Complex algorithms or calculations
- Non-obvious business logic
- Workarounds for known issues
- Important security considerations

**When NOT to use inline comments**:
- Obvious code (let the code speak for itself)
- Redundant explanations
- Commented-out code (delete it instead)

### Test Coverage Expectations

**Requirement**: All new code must include appropriate test coverage.

#### Minimum Coverage

For new functions:
- At least one happy path test
- At least one error condition test

For new features:
- Happy path tests for all main workflows
- Error condition tests for all error types
- Edge case tests for boundary conditions

#### Recommended Coverage

- All code paths tested
- All error conditions tested
- Authorization tests for protected functions
- State transition tests for stateful operations
- Integration tests for multi-function workflows

#### Test Quality

Tests should be:
- **Deterministic** - Same input always produces same output
- **Independent** - Tests don't depend on each other
- **Fast** - Tests run quickly to encourage frequent testing
- **Clear** - Test names and assertions are self-documenting
- **Comprehensive** - Cover both success and failure cases

**Example test coverage for a new function**:

```rust
// Happy path
#[test]
fn test_create_remittance_success() { }

// Error conditions
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_create_remittance_zero_amount() { }

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_create_remittance_unregistered_agent() { }

// Edge cases
#[test]
fn test_create_remittance_minimum_amount() { }

#[test]
fn test_create_remittance_maximum_amount() { }

// Integration
#[test]
fn test_create_and_confirm_remittance() { }
```

### Pre-Commit Checklist

Before committing your code, ensure:

- [ ] Code builds successfully: `cargo build --target wasm32-unknown-unknown --release`
- [ ] All tests pass: `cargo test`
- [ ] Code is formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy --target wasm32-unknown-unknown -- -D warnings`
- [ ] Public functions have doc comments
- [ ] New functionality has tests
- [ ] Error handling follows project patterns
- [ ] Authorization checks are in place
- [ ] Storage operations are optimized

### Code Review Standards

When reviewing code, check for:

**Correctness**:
- Logic is correct and handles all cases
- Error handling is comprehensive
- Authorization is properly enforced

**Quality**:
- Code follows naming conventions
- Functions are focused and well-organized
- Documentation is clear and complete

**Performance**:
- Storage operations are minimized
- Unnecessary clones are avoided
- Checked arithmetic is used

**Security**:
- No `unwrap()` or `expect()` in production code
- All inputs are validated
- Authorization checks are present and correct

**Testing**:
- Tests cover happy path and error conditions
- Tests are clear and maintainable
- Edge cases are tested

### Common Patterns

#### Pattern: Validated State Change

```rust
pub fn state_changing_function(env: Env, ...) -> Result<(), ContractError> {
    // 1. Authorization
    address.require_auth();
    
    // 2. Load state
    let state = get_state(&env)?;
    
    // 3. Validate
    validate_state(&state)?;
    validate_inputs(...)?;
    
    // 4. Perform operation
    let new_state = perform_operation(state)?;
    
    // 5. Save state
    set_state(&env, &new_state);
    
    // 6. Emit event
    emit_event(&env, ...);
    
    Ok(())
}
```

#### Pattern: Fee Calculation

```rust
pub fn calculate_fee(amount: i128, fee_bps: i128) -> Result<i128, ContractError> {
    amount
        .checked_mul(fee_bps)
        .ok_or(ContractError::Overflow)?
        .checked_div(10000)
        .ok_or(ContractError::Overflow)
}
```

#### Pattern: Storage Access

```rust
// Getter with error handling
pub fn get_something(env: &Env, key: &SomeKey) -> Result<SomeValue, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Something(key.clone()))
        .ok_or(ContractError::NotFound)
}

// Setter
pub fn set_something(env: &Env, key: &SomeKey, value: &SomeValue) {
    env.storage()
        .persistent()
        .set(&DataKey::Something(key.clone()), value);
}
```

### Resources

- [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Clippy Lints](https://rust-lang.github.io/rust-clippy/master/)

### Questions?

If you have questions about coding standards:
- Check existing code for examples
- Ask in [GitHub Discussions](https://github.com/yourusername/swiftremit/discussions)
- Reach out in [Stellar Discord](https://discord.gg/stellar) #soroban channel

---

## Testing Guidelines

Comprehensive testing is essential for SwiftRemit as a financial smart contract. This section defines our testing requirements, best practices, and guidelines for writing effective tests using the Soroban SDK.

### Testing Philosophy

Our testing approach is built on these principles:

- **Correctness First** - Tests verify that the contract behaves correctly under all conditions
- **Comprehensive Coverage** - All code paths, error conditions, and edge cases must be tested
- **Deterministic Tests** - Tests must produce consistent results every time they run
- **Fast Feedback** - Tests should run quickly to encourage frequent testing
- **Clear Intent** - Test names and assertions should clearly communicate what is being tested

### Testing Requirements

#### Unit Tests for All New Functions

**Requirement**: Every new function must have unit tests that verify its behavior.

**What to test**:
- Happy path (expected successful execution)
- Error conditions (all possible error returns)
- Edge cases (boundary values, empty inputs, maximum values)
- Authorization (if the function requires auth)

**Example**:
```rust
#[test]
fn test_calculate_fee_success() {
    let amount = 100_000_000;  // 10 USDC
    let fee_bps = 250;         // 2.5%
    
    let fee = calculate_fee(amount, fee_bps).unwrap();
    
    assert_eq!(fee, 2_500_000);  // 0.25 USDC
}

#[test]
fn test_calculate_fee_overflow() {
    let amount = i128::MAX;
    let fee_bps = 250;
    
    let result = calculate_fee(amount, fee_bps);
    
    assert_eq!(result, Err(ContractError::Overflow));
}

#[test]
fn test_calculate_fee_zero_amount() {
    let amount = 0;
    let fee_bps = 250;
    
    let fee = calculate_fee(amount, fee_bps).unwrap();
    
    assert_eq!(fee, 0);
}
```

#### Integration Tests for Multi-Function Workflows

**Requirement**: All multi-function workflows must have integration tests that verify end-to-end behavior.

**What to test**:
- Complete user workflows (create → confirm → complete)
- State transitions across multiple operations
- Interactions between different contract functions
- Token transfers and balance changes

**Example**:
```rust
#[test]
fn test_complete_remittance_workflow() {
    let env = Env::default();
    env.mock_all_auths();
    
    // Setup
    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    
    // Initialize contract
    contract.initialize(&admin, &token.address, &250);
    
    // Register agent
    contract.register_agent(&agent);
    
    // Mint tokens to sender
    token.mint(&sender, &100_000_000);
    
    // Create remittance
    let remittance_id = contract.create_remittance(
        &sender,
        &agent,
        &100_000_000,
    );
    
    // Verify remittance created
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, RemittanceStatus::Pending);
    assert_eq!(remittance.amount, 100_000_000);
    
    // Confirm payout
    contract.confirm_payout(&remittance_id);
    
    // Verify remittance completed
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, RemittanceStatus::Completed);
    
    // Verify agent received payout (amount - fee)
    let agent_balance = token.balance(&agent);
    assert_eq!(agent_balance, 97_500_000);  // 10 USDC - 2.5% fee
    
    // Verify fees accumulated
    let accumulated_fees = contract.get_accumulated_fees();
    assert_eq!(accumulated_fees, 2_500_000);
}
```

#### Test Coverage for Error Conditions

**Requirement**: All error conditions must be tested to ensure proper error handling.

**What to test**:
- Each error type defined in `errors.rs`
- Invalid inputs that should trigger errors
- Invalid state transitions
- Authorization failures
- Overflow conditions

**Example**:
```rust
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_create_remittance_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Should panic with InvalidAmount error
    contract.create_remittance(&sender, &agent, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_create_remittance_unregistered_agent() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract = setup_contract(&env);
    let sender = Address::generate(&env);
    let unregistered_agent = Address::generate(&env);
    
    // Should panic with AgentNotRegistered error
    contract.create_remittance(&sender, &unregistered_agent, &100_000_000);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_confirm_payout_invalid_status() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract = setup_contract(&env);
    let remittance_id = create_test_remittance(&env, &contract);
    
    // Confirm once (should succeed)
    contract.confirm_payout(&remittance_id);
    
    // Try to confirm again (should panic with InvalidStatus error)
    contract.confirm_payout(&remittance_id);
}
```

### Writing Tests with Soroban SDK Testutils

The Soroban SDK provides powerful testing utilities that make it easy to write comprehensive tests.

#### Setting Up the Test Environment

```rust
use soroban_sdk::{Env, Address, testutils::Address as _};

#[test]
fn test_something() {
    // Create a test environment
    let env = Env::default();
    
    // Mock all authorizations (skip auth checks for testing)
    env.mock_all_auths();
    
    // Generate test addresses
    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    
    // Create contract instances
    let contract = create_contract(&env);
    
    // Run your test...
}
```

#### Key Testutils Features

**1. Address Generation**
```rust
use soroban_sdk::testutils::Address as _;

let address = Address::generate(&env);
```

**2. Authorization Mocking**
```rust
// Mock all authorizations (most common for unit tests)
env.mock_all_auths();

// Mock specific authorization
env.mock_auths(&[
    MockAuth {
        address: &user,
        invoke: &MockAuthInvoke {
            contract: &contract.address,
            fn_name: "create_remittance",
            args: vec![&env, user.to_val(), agent.to_val(), 100_000_000.into_val(&env)],
            sub_invokes: &[],
        },
    },
]);
```

**3. Event Verification**
```rust
// Get all events emitted during test
let events = env.events().all();

// Verify specific event was emitted
assert!(events.iter().any(|e| {
    e.topics.contains(&symbol_short!("created"))
}));
```

**4. Storage Inspection**
```rust
// Access storage directly in tests (for verification)
let value: Option<Remittance> = env.storage()
    .persistent()
    .get(&DataKey::Remittance(1));

assert!(value.is_some());
```

#### Test Helper Functions

Create helper functions to reduce boilerplate and improve test readability:

```rust
// Helper to create and initialize a token contract
fn create_token_contract(env: &Env, admin: &Address) -> StellarAssetClient {
    let token = StellarAssetClient::new(env, &env.register_stellar_asset_contract(admin.clone()));
    token
}

// Helper to create and initialize SwiftRemit contract
fn create_swiftremit_contract(env: &Env) -> SwiftRemitContractClient {
    let contract_id = env.register_contract(None, SwiftRemitContract);
    SwiftRemitContractClient::new(env, &contract_id)
}

// Helper to setup a fully initialized contract
fn setup_contract(env: &Env) -> (SwiftRemitContractClient, StellarAssetClient, Address) {
    let admin = Address::generate(env);
    let token = create_token_contract(env, &admin);
    let contract = create_swiftremit_contract(env);
    
    contract.initialize(&admin, &token.address, &250);
    
    (contract, token, admin)
}

// Helper to create a test remittance
fn create_test_remittance(
    env: &Env,
    contract: &SwiftRemitContractClient,
    token: &StellarAssetClient,
    sender: &Address,
    agent: &Address,
    amount: i128,
) -> u64 {
    token.mint(sender, &amount);
    contract.register_agent(agent);
    contract.create_remittance(sender, agent, &amount)
}
```

### Deterministic, Non-Flaky Tests

**Requirement**: All tests must be deterministic and produce consistent results.

#### What Makes Tests Flaky

**Avoid**:
- Random values without seeding
- Time-dependent logic without mocking
- External dependencies (network, filesystem)
- Race conditions in concurrent tests
- Uninitialized state
- Tests that depend on execution order

#### Writing Deterministic Tests

**Good Practices**:

```rust
// Good - deterministic values
#[test]
fn test_fee_calculation() {
    let amount = 100_000_000;
    let fee_bps = 250;
    let expected_fee = 2_500_000;
    
    let fee = calculate_fee(amount, fee_bps).unwrap();
    
    assert_eq!(fee, expected_fee);
}

// Good - each test is independent
#[test]
fn test_create_remittance() {
    let env = Env::default();  // Fresh environment
    env.mock_all_auths();
    
    let contract = setup_contract(&env);
    // Test logic...
}

// Good - explicit state setup
#[test]
fn test_cancel_pending_remittance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Explicitly create the state we need
    let remittance_id = create_test_remittance(
        &env,
        &contract,
        &token,
        &sender,
        &agent,
        100_000_000,
    );
    
    // Now test cancellation
    contract.cancel_remittance(&remittance_id);
    
    let remittance = contract.get_remittance(&remittance_id);
    assert_eq!(remittance.status, RemittanceStatus::Cancelled);
}
```

**Bad Practices**:

```rust
// Bad - random values
#[test]
fn test_something() {
    let amount = rand::random::<i128>();  // Non-deterministic!
    // ...
}

// Bad - depends on previous test
static mut GLOBAL_STATE: Option<Contract> = None;

#[test]
fn test_first() {
    unsafe { GLOBAL_STATE = Some(create_contract()); }
}

#[test]
fn test_second() {
    let contract = unsafe { GLOBAL_STATE.as_ref().unwrap() };  // Depends on test_first!
    // ...
}

// Bad - time-dependent
#[test]
fn test_expiration() {
    let now = SystemTime::now();  // Non-deterministic!
    // ...
}
```

### Test Structure and Patterns

#### Standard Test Structure

Follow the **Arrange-Act-Assert** pattern:

```rust
#[test]
fn test_descriptive_name() {
    // Arrange - Set up test environment and data
    let env = Env::default();
    env.mock_all_auths();
    let contract = setup_contract(&env);
    let sender = Address::generate(&env);
    
    // Act - Perform the operation being tested
    let result = contract.some_function(&sender, &param);
    
    // Assert - Verify the expected outcome
    assert_eq!(result, expected_value);
}
```

#### Test Naming Conventions

Use descriptive names that explain what is being tested:

**Pattern**: `test_<function>_<scenario>_<expected_result>`

**Examples**:
```rust
#[test]
fn test_create_remittance_success() { }

#[test]
fn test_create_remittance_zero_amount_fails() { }

#[test]
fn test_create_remittance_unregistered_agent_fails() { }

#[test]
fn test_confirm_payout_updates_status() { }

#[test]
fn test_confirm_payout_transfers_correct_amount() { }

#[test]
fn test_cancel_remittance_refunds_sender() { }
```

#### Testing Error Conditions

Use `#[should_panic]` for expected errors:

```rust
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_invalid_amount_error() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract = setup_contract(&env);
    contract.create_remittance(&sender, &agent, &0);  // Should panic
}
```

Or use `Result` for more control:

```rust
#[test]
fn test_invalid_amount_returns_error() {
    let env = Env::default();
    
    let result = validate_amount(0);
    
    assert_eq!(result, Err(ContractError::InvalidAmount));
}
```

#### Testing State Transitions

Verify state changes explicitly:

```rust
#[test]
fn test_confirm_payout_state_transition() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Create remittance
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    // Verify initial state
    let remittance = contract.get_remittance(&id);
    assert_eq!(remittance.status, RemittanceStatus::Pending);
    
    // Perform state transition
    contract.confirm_payout(&id);
    
    // Verify new state
    let remittance = contract.get_remittance(&id);
    assert_eq!(remittance.status, RemittanceStatus::Completed);
}
```

#### Testing Authorization

Test both authorized and unauthorized access:

```rust
#[test]
fn test_admin_can_register_agent() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, admin) = setup_contract(&env);
    let agent = Address::generate(&env);
    
    // Should succeed
    contract.register_agent(&agent);
    
    assert!(contract.is_agent_registered(&agent));
}

#[test]
#[should_panic]
fn test_non_admin_cannot_register_agent() {
    let env = Env::default();
    // Don't mock auths - test real authorization
    
    let (contract, _, _) = setup_contract(&env);
    let non_admin = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Should panic with authorization error
    contract.register_agent(&agent);
}
```

### Test Categories and Examples

#### 1. Initialization Tests

```rust
#[test]
fn test_initialize_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    
    contract.initialize(&admin, &token.address, &250);
    
    assert_eq!(contract.get_admin(), admin);
    assert_eq!(contract.get_usdc_token(), token.address);
    assert_eq!(contract.get_platform_fee_bps(), 250);
}

#[test]
#[should_panic(expected = "Error(Contract, #1)")]
fn test_initialize_twice_fails() {
    let env = Env::default();
    env.mock_all_auths();
    
    let admin = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    
    contract.initialize(&admin, &token.address, &250);
    contract.initialize(&admin, &token.address, &250);  // Should panic
}
```

#### 2. Agent Management Tests

```rust
#[test]
fn test_register_agent_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, _) = setup_contract(&env);
    let agent = Address::generate(&env);
    
    contract.register_agent(&agent);
    
    assert!(contract.is_agent_registered(&agent));
}

#[test]
fn test_remove_agent_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, _) = setup_contract(&env);
    let agent = Address::generate(&env);
    
    contract.register_agent(&agent);
    assert!(contract.is_agent_registered(&agent));
    
    contract.remove_agent(&agent);
    assert!(!contract.is_agent_registered(&agent));
}
```

#### 3. Remittance Lifecycle Tests

```rust
#[test]
fn test_create_remittance_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    token.mint(&sender, &100_000_000);
    contract.register_agent(&agent);
    
    let id = contract.create_remittance(&sender, &agent, &100_000_000);
    
    let remittance = contract.get_remittance(&id);
    assert_eq!(remittance.sender, sender);
    assert_eq!(remittance.agent, agent);
    assert_eq!(remittance.amount, 100_000_000);
    assert_eq!(remittance.status, RemittanceStatus::Pending);
}

#[test]
fn test_confirm_payout_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    contract.confirm_payout(&id);
    
    let remittance = contract.get_remittance(&id);
    assert_eq!(remittance.status, RemittanceStatus::Completed);
    
    // Verify agent received payout
    let agent_balance = token.balance(&agent);
    assert_eq!(agent_balance, 97_500_000);  // 100M - 2.5% fee
}

#[test]
fn test_cancel_remittance_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    contract.cancel_remittance(&id);
    
    let remittance = contract.get_remittance(&id);
    assert_eq!(remittance.status, RemittanceStatus::Cancelled);
    
    // Verify sender received refund
    let sender_balance = token.balance(&sender);
    assert_eq!(sender_balance, 100_000_000);
}
```

#### 4. Fee Calculation Tests

```rust
#[test]
fn test_fee_calculation_correct() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    let remittance = contract.get_remittance(&id);
    
    // Fee should be 2.5% of 100M = 2.5M
    assert_eq!(remittance.fee, 2_500_000);
}

#[test]
fn test_update_fee_success() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, _) = setup_contract(&env);
    
    contract.update_fee(&500);  // Change to 5%
    
    assert_eq!(contract.get_platform_fee_bps(), 500);
}
```

#### 5. Edge Case Tests

```rust
#[test]
fn test_minimum_amount_remittance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 1);
    
    let remittance = contract.get_remittance(&id);
    assert_eq!(remittance.amount, 1);
}

#[test]
fn test_multiple_concurrent_remittances() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    token.mint(&sender, &300_000_000);
    contract.register_agent(&agent);
    
    // Create multiple remittances
    let id1 = contract.create_remittance(&sender, &agent, &100_000_000);
    let id2 = contract.create_remittance(&sender, &agent, &100_000_000);
    let id3 = contract.create_remittance(&sender, &agent, &100_000_000);
    
    // Verify all are independent
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);
    
    let r1 = contract.get_remittance(&id1);
    let r2 = contract.get_remittance(&id2);
    let r3 = contract.get_remittance(&id3);
    
    assert_eq!(r1.status, RemittanceStatus::Pending);
    assert_eq!(r2.status, RemittanceStatus::Pending);
    assert_eq!(r3.status, RemittanceStatus::Pending);
}
```

### Running Tests

#### Run All Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (faster)
cargo test --release
```

#### Run Specific Tests

```bash
# Run a specific test
cargo test test_create_remittance_success

# Run all tests matching a pattern
cargo test create_remittance

# Run tests in a specific module
cargo test test::remittance_tests
```

#### Test Output

```bash
running 15 tests
test test_initialize_success ... ok
test test_create_remittance_success ... ok
test test_confirm_payout_success ... ok
test test_cancel_remittance_success ... ok
test test_create_remittance_zero_amount_fails ... ok
test test_create_remittance_unregistered_agent_fails ... ok
...

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Test Maintenance

#### Keeping Tests Up to Date

- Update tests when changing functionality
- Add tests for new features before implementing them (TDD)
- Remove tests for removed functionality
- Refactor tests to reduce duplication

#### Test Code Quality

Tests should be:
- **Readable** - Clear and easy to understand
- **Maintainable** - Easy to update when code changes
- **Reliable** - Consistently pass or fail
- **Fast** - Run quickly to encourage frequent testing

#### Refactoring Tests

Extract common setup into helper functions:

```rust
// Before - duplicated setup
#[test]
fn test_a() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    // Test logic...
}

#[test]
fn test_b() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let token = create_token_contract(&env, &admin);
    let contract = create_swiftremit_contract(&env);
    contract.initialize(&admin, &token.address, &250);
    // Test logic...
}

// After - extracted helper
fn setup_test_environment(env: &Env) -> (SwiftRemitContractClient, StellarAssetClient, Address) {
    env.mock_all_auths();
    let admin = Address::generate(env);
    let token = create_token_contract(env, &admin);
    let contract = create_swiftremit_contract(env);
    contract.initialize(&admin, &token.address, &250);
    (contract, token, admin)
}

#[test]
fn test_a() {
    let env = Env::default();
    let (contract, token, admin) = setup_test_environment(&env);
    // Test logic...
}

#[test]
fn test_b() {
    let env = Env::default();
    let (contract, token, admin) = setup_test_environment(&env);
    // Test logic...
}
```

### Testing Checklist

Before submitting a PR, ensure:

- [ ] All new functions have unit tests
- [ ] All error conditions are tested
- [ ] Integration tests cover multi-function workflows
- [ ] Tests are deterministic and non-flaky
- [ ] Test names clearly describe what is being tested
- [ ] All tests pass: `cargo test`
- [ ] Tests follow the Arrange-Act-Assert pattern
- [ ] Helper functions reduce test duplication
- [ ] Authorization is tested where applicable
- [ ] State transitions are verified
- [ ] Edge cases are covered

### Common Testing Mistakes

**Avoid**:
- Testing implementation details instead of behavior
- Writing tests that are too complex
- Mocking too much (test real behavior when possible)
- Not testing error conditions
- Flaky tests that sometimes fail
- Tests that depend on execution order
- Overly broad assertions that don't catch bugs

**Instead**:
- Test public behavior and contracts
- Keep tests simple and focused
- Use real implementations when practical
- Test all error paths
- Write deterministic tests
- Make each test independent
- Use specific assertions

### Resources

- [Soroban Testing Documentation](https://soroban.stellar.org/docs/how-to-guides/testing)
- [Rust Testing Guide](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Test-Driven Development](https://en.wikipedia.org/wiki/Test-driven_development)

### Questions?

If you have questions about testing:
- Review existing tests in `test.rs` for examples
- Ask in [GitHub Discussions](https://github.com/yourusername/swiftremit/discussions)
- Reach out in [Stellar Discord](https://discord.gg/stellar) #soroban channel

---

## Security Considerations

Security is paramount for SwiftRemit as a financial smart contract handling real value on the Stellar blockchain. This section outlines our security requirements, best practices, and prohibited patterns to ensure the contract remains secure and trustworthy.

### Security Philosophy

Our security approach is built on these core principles:

- **Assume Malicious Input** - Treat all external inputs as potentially malicious
- **Fail Securely** - When errors occur, fail in a way that preserves security
- **Defense in Depth** - Use multiple layers of security controls
- **Minimize Attack Surface** - Keep the contract simple and focused
- **Explicit Authorization** - Always verify who is performing an action
- **Validate Everything** - Never trust data without validation

### Security Best Practices for Soroban Contracts

#### 1. Authorization Checks

**Requirement**: All state-changing functions MUST require proper authorization.

**Why**: Without authorization checks, anyone could manipulate contract state, steal funds, or disrupt operations.

**Implementation**:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // REQUIRED: Verify the caller is authorized
    sender.require_auth();
    
    // Rest of function logic...
}
```

**Best Practices**:
- Call `require_auth()` early in the function, before expensive operations
- Verify the correct party for each operation (sender, agent, admin)
- Never skip authorization checks, even for "read-only" functions that might change state
- Test authorization failures explicitly

**Example - Admin-Only Function**:
```rust
pub fn register_agent(env: Env, agent: Address) -> Result<(), ContractError> {
    // Only admin can register agents
    let admin = get_admin(&env)?;
    admin.require_auth();
    
    set_agent_registered(&env, &agent, true);
    emit_agent_registered(&env, agent);
    
    Ok(())
}
```

**Example - Owner-Only Function**:
```rust
pub fn cancel_remittance(
    env: Env,
    remittance_id: u64,
) -> Result<(), ContractError> {
    let remittance = get_remittance(&env, remittance_id)?;
    
    // Only the original sender can cancel
    remittance.sender.require_auth();
    
    // Verify status allows cancellation
    if remittance.status != RemittanceStatus::Pending {
        return Err(ContractError::InvalidStatus);
    }
    
    // Process cancellation...
    Ok(())
}
```

#### 2. Input Validation

**Requirement**: ALL user inputs MUST be validated before use.

**Why**: Invalid inputs can cause arithmetic errors, state corruption, or unexpected behavior.

**Validation Checklist**:
- [ ] Amounts are positive (> 0)
- [ ] Addresses are valid
- [ ] IDs exist in storage
- [ ] Enum values are valid
- [ ] Percentages/basis points are within valid ranges
- [ ] String lengths are reasonable (if applicable)

**Implementation**:
```rust
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    sender.require_auth();
    
    // Validate amount
    if amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    
    // Validate agent is registered
    if !is_agent_registered(&env, &agent) {
        return Err(ContractError::AgentNotRegistered);
    }
    
    // Validate addresses (if additional checks needed)
    validate_address(&sender)?;
    validate_address(&agent)?;
    
    // Proceed with validated inputs...
}
```

**Example - Fee Validation**:
```rust
pub fn update_fee(env: Env, fee_bps: u32) -> Result<(), ContractError> {
    let admin = get_admin(&env)?;
    admin.require_auth();
    
    // Validate fee is not greater than 100% (10000 basis points)
    if fee_bps > 10000 {
        return Err(ContractError::InvalidFeeBps);
    }
    
    set_platform_fee_bps(&env, fee_bps);
    emit_fee_updated(&env, fee_bps);
    
    Ok(())
}
```

**Example - State Validation**:
```rust
pub fn confirm_payout(
    env: Env,
    remittance_id: u64,
) -> Result<(), ContractError> {
    let mut remittance = get_remittance(&env, remittance_id)?;
    
    remittance.agent.require_auth();
    
    // Validate current state allows this operation
    if remittance.status != RemittanceStatus::Pending {
        return Err(ContractError::InvalidStatus);
    }
    
    // Proceed with payout...
}
```

#### 3. Overflow Protection

**Requirement**: ALL arithmetic operations MUST use checked arithmetic to prevent overflows.

**Why**: Integer overflow can lead to incorrect calculations, loss of funds, or contract exploitation.

**Use Checked Arithmetic**:
- `checked_add()` instead of `+`
- `checked_sub()` instead of `-`
- `checked_mul()` instead of `*`
- `checked_div()` instead of `/`

**Implementation**:
```rust
pub fn calculate_fee(amount: i128, fee_bps: u32) -> Result<i128, ContractError> {
    // GOOD: Use checked_mul to prevent overflow
    let fee = amount
        .checked_mul(fee_bps as i128)
        .ok_or(ContractError::Overflow)?
        .checked_div(10000)
        .ok_or(ContractError::Overflow)?;
    
    Ok(fee)
}

// BAD: Direct arithmetic can overflow
pub fn calculate_fee_unsafe(amount: i128, fee_bps: u32) -> i128 {
    (amount * fee_bps as i128) / 10000  // ❌ Can overflow!
}
```

**Example - Fee Calculation with Overflow Protection**:
```rust
let fee_bps = get_platform_fee_bps(&env)?;

// Calculate fee with overflow protection
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?
    .checked_div(10000)
    .ok_or(ContractError::Overflow)?;

// Calculate payout with overflow protection
let payout_amount = amount
    .checked_sub(fee)
    .ok_or(ContractError::Overflow)?;
```

**Example - Counter Increment**:
```rust
let counter = get_remittance_counter(&env)?;

// Safe increment
let new_counter = counter
    .checked_add(1)
    .ok_or(ContractError::Overflow)?;

set_remittance_counter(&env, new_counter);
```

#### 4. Error Handling

**Requirement**: Use proper error handling; NEVER use `unwrap()` or `expect()` in production code.

**Why**: Panics in production can lock funds, corrupt state, or make the contract unusable.

**Use Result Types**:
```rust
// GOOD: Return Result and handle errors
pub fn get_remittance(
    env: &Env,
    remittance_id: u64,
) -> Result<Remittance, ContractError> {
    env.storage()
        .persistent()
        .get(&DataKey::Remittance(remittance_id))
        .ok_or(ContractError::RemittanceNotFound)
}

// BAD: Using unwrap can panic
pub fn get_remittance_unsafe(env: &Env, remittance_id: u64) -> Remittance {
    env.storage()
        .persistent()
        .get(&DataKey::Remittance(remittance_id))
        .unwrap()  // ❌ Will panic if not found!
}
```

**Propagate Errors with `?`**:
```rust
pub fn confirm_payout(
    env: Env,
    remittance_id: u64,
) -> Result<(), ContractError> {
    // Use ? to propagate errors up the call stack
    let mut remittance = get_remittance(&env, remittance_id)?;
    let fee_bps = get_platform_fee_bps(&env)?;
    let token_address = get_usdc_token(&env)?;
    
    // Continue with logic...
    Ok(())
}
```

**Handle All Error Cases**:
```rust
match operation_result {
    Ok(value) => {
        // Handle success
        process_value(value)
    }
    Err(ContractError::NotFound) => {
        // Handle specific error
        return Err(ContractError::NotFound);
    }
    Err(e) => {
        // Handle other errors
        return Err(e);
    }
}
```

#### 5. State Validation

**Requirement**: Validate state before and after critical operations.

**Why**: Ensures the contract maintains consistent, valid state throughout its lifecycle.

**Pre-Condition Checks**:
```rust
pub fn confirm_payout(
    env: Env,
    remittance_id: u64,
) -> Result<(), ContractError> {
    let mut remittance = get_remittance(&env, remittance_id)?;
    
    // Validate pre-conditions
    if remittance.status != RemittanceStatus::Pending {
        return Err(ContractError::InvalidStatus);
    }
    
    if remittance.amount <= 0 {
        return Err(ContractError::InvalidAmount);
    }
    
    // Proceed with operation...
}
```

**Post-Condition Checks**:
```rust
// After state change, verify it's correct
remittance.status = RemittanceStatus::Completed;
set_remittance(&env, remittance_id, &remittance);

// Verify the change was persisted correctly
let stored_remittance = get_remittance(&env, remittance_id)?;
if stored_remittance.status != RemittanceStatus::Completed {
    // This should never happen, but if it does, we catch it
    return Err(ContractError::InvalidStatus);
}
```

**Invariant Checks**:
```rust
// Ensure invariants hold after operations
let accumulated_fees = get_accumulated_fees(&env)?;

// Fees should never be negative
if accumulated_fees < 0 {
    return Err(ContractError::InvalidAmount);
}
```

### Prohibited Practices

These practices are **PROHIBITED** in SwiftRemit code unless explicitly justified and approved by maintainers:

#### 1. No `unsafe` Rust

**Rule**: Do NOT use `unsafe` blocks without explicit justification and security review.

```rust
// ❌ PROHIBITED without justification
unsafe {
    // Unsafe operations
}
```

**Why**: `unsafe` code bypasses Rust's safety guarantees and can introduce vulnerabilities.

**Exception**: If `unsafe` is absolutely necessary:
1. Document why it's needed
2. Explain why it's safe
3. Get explicit approval in code review
4. Add extensive tests

#### 2. No `unwrap()` or `expect()` in Production

**Rule**: NEVER use `unwrap()` or `expect()` in production code paths.

```rust
// ❌ PROHIBITED in production code
let value = storage.get(&key).unwrap();
let amount = calculate_fee(x).expect("fee calculation failed");
```

**Why**: These functions panic on error, which can lock funds or corrupt state.

**Allowed**: Only in test code where panics are acceptable:
```rust
#[test]
fn test_something() {
    let value = storage.get(&key).unwrap();  // ✅ OK in tests
}
```

**Use Instead**:
```rust
// ✅ CORRECT: Handle errors properly
let value = storage.get(&key).ok_or(ContractError::NotFound)?;
let amount = calculate_fee(x)?;
```

#### 3. No Unchecked Arithmetic

**Rule**: NEVER use direct arithmetic operators for financial calculations.

```rust
// ❌ PROHIBITED
let total = amount + fee;
let result = value * multiplier;
let difference = balance - withdrawal;
```

**Why**: Can overflow and produce incorrect results.

**Use Instead**:
```rust
// ✅ CORRECT: Use checked arithmetic
let total = amount.checked_add(fee).ok_or(ContractError::Overflow)?;
let result = value.checked_mul(multiplier).ok_or(ContractError::Overflow)?;
let difference = balance.checked_sub(withdrawal).ok_or(ContractError::Overflow)?;
```

#### 4. No Hardcoded Addresses or Secrets

**Rule**: NEVER hardcode addresses, private keys, or secrets in the contract.

```rust
// ❌ PROHIBITED
const ADMIN_ADDRESS: &str = "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
const SECRET_KEY: &str = "SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
```

**Why**: Hardcoded values are inflexible, visible on-chain, and can't be changed.

**Use Instead**:
```rust
// ✅ CORRECT: Store in contract storage, set during initialization
pub fn initialize(env: Env, admin: Address, ...) -> Result<(), ContractError> {
    set_admin(&env, &admin);
    Ok(())
}
```

#### 5. No Panics in Production

**Rule**: Production code must NEVER panic intentionally.

```rust
// ❌ PROHIBITED
if amount <= 0 {
    panic!("Invalid amount");
}

assert!(balance >= withdrawal, "Insufficient balance");
```

**Why**: Panics can lock funds and make the contract unusable.

**Use Instead**:
```rust
// ✅ CORRECT: Return errors
if amount <= 0 {
    return Err(ContractError::InvalidAmount);
}

if balance < withdrawal {
    return Err(ContractError::InsufficientBalance);
}
```

### Security-Focused Testing

**Requirement**: All security-sensitive operations MUST have dedicated security tests.

#### Test Authorization Failures

```rust
#[test]
#[should_panic]
fn test_unauthorized_agent_registration() {
    let env = Env::default();
    // Don't mock auths - test real authorization
    
    let (contract, _, _) = setup_contract(&env);
    let non_admin = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Should panic - non-admin trying to register agent
    contract.register_agent(&agent);
}

#[test]
#[should_panic]
fn test_unauthorized_cancellation() {
    let env = Env::default();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    let attacker = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    // Should panic - attacker trying to cancel someone else's remittance
    contract.cancel_remittance(&id);
}
```

#### Test Invalid Inputs

```rust
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_zero_amount_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Should panic with InvalidAmount error
    contract.create_remittance(&sender, &agent, &0);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_negative_amount_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    // Should panic with InvalidAmount error
    contract.create_remittance(&sender, &agent, &-100);
}
```

#### Test Overflow Conditions

```rust
#[test]
fn test_overflow_protection_in_fee_calculation() {
    let env = Env::default();
    
    // Test with maximum i128 value
    let result = calculate_fee(i128::MAX, 250);
    
    // Should return Overflow error, not panic
    assert_eq!(result, Err(ContractError::Overflow));
}

#[test]
fn test_overflow_protection_in_counter() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, _, _) = setup_contract(&env);
    
    // Set counter to maximum value
    set_remittance_counter(&env, u64::MAX);
    
    // Attempting to create remittance should fail gracefully
    let result = contract.create_remittance(&sender, &agent, &100_000_000);
    
    assert!(result.is_err());
}
```

#### Test State Transitions

```rust
#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_cannot_confirm_completed_remittance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    // Confirm once (should succeed)
    contract.confirm_payout(&id);
    
    // Try to confirm again (should panic with InvalidStatus)
    contract.confirm_payout(&id);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_cannot_cancel_completed_remittance() {
    let env = Env::default();
    env.mock_all_auths();
    
    let (contract, token, _) = setup_contract(&env);
    let sender = Address::generate(&env);
    let agent = Address::generate(&env);
    
    let id = create_test_remittance(&env, &contract, &token, &sender, &agent, 100_000_000);
    
    // Confirm payout
    contract.confirm_payout(&id);
    
    // Try to cancel (should panic with InvalidStatus)
    contract.cancel_remittance(&id);
}
```

#### Test Reentrancy (if applicable)

```rust
#[test]
fn test_no_reentrancy_vulnerability() {
    // Test that the contract is not vulnerable to reentrancy attacks
    // This is less common in Soroban but should be tested if callbacks are used
}
```

### Security Review Process

All security-sensitive changes require thorough review:

#### What Requires Security Review

- Changes to authorization logic
- Changes to token transfers
- Changes to fee calculations
- Changes to state transitions
- Addition of new state-changing functions
- Changes to storage patterns
- Use of `unsafe` code (requires explicit justification)

#### Security Review Checklist

Reviewers should verify:

- [ ] All state-changing functions have authorization checks
- [ ] All inputs are validated
- [ ] All arithmetic uses checked operations
- [ ] No `unwrap()` or `expect()` in production code
- [ ] No `unsafe` code (or justified and documented)
- [ ] Error handling is comprehensive
- [ ] State transitions are validated
- [ ] Security tests cover the changes
- [ ] No hardcoded addresses or secrets
- [ ] Token transfers are correct and safe

### Security Testing Checklist

Before submitting a PR with security-sensitive changes:

- [ ] Authorization tests for all new state-changing functions
- [ ] Invalid input tests for all new parameters
- [ ] Overflow tests for all arithmetic operations
- [ ] State transition tests for all status changes
- [ ] Error handling tests for all error paths
- [ ] Integration tests for complete workflows
- [ ] Edge case tests (zero, negative, maximum values)
- [ ] Unauthorized access tests (wrong user, wrong role)

### Reporting Security Issues

If you discover a security vulnerability:

1. **DO NOT** open a public GitHub issue
2. **DO NOT** discuss it publicly
3. **DO** report it privately to the maintainers via email
4. **DO** provide detailed information:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if you have one)

**Contact**: Please report security issues privately to the project maintainers via GitHub Security Advisories or by contacting the maintainers directly.

We will:
- Acknowledge receipt within 24-48 hours
- Investigate and validate the issue
- Develop and test a fix
- Coordinate disclosure timing with you
- Credit you in the security advisory (if desired)

### Security Resources

- [Soroban Security Best Practices](https://soroban.stellar.org/docs/learn/security)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Smart Contract Security](https://github.com/crytic/building-secure-contracts)
- [OWASP Smart Contract Security](https://owasp.org/www-project-smart-contract-top-10/)

### Questions About Security?

If you have questions about security practices:
- Review existing code for examples
- Ask in [GitHub Discussions](https://github.com/yourusername/swiftremit/discussions) (for general questions)
- Reach out in [Stellar Discord](https://discord.gg/stellar) #soroban channel
- Contact maintainers privately for sensitive issues

---

## Pull Request Process

This section guides you through the process of submitting, reviewing, and merging pull requests (PRs) to SwiftRemit. Following these guidelines ensures a smooth review process and helps maintain code quality.

### Before Submitting a Pull Request

Before you create a PR, complete these steps to ensure your contribution is ready for review:

#### 1. Discuss Non-Trivial Changes

For significant changes (new features, major refactors, breaking changes), create an issue first to discuss:
- The problem you're solving
- Your proposed approach
- Alternative solutions considered
- Impact on existing functionality

**When to create an issue first:**
- Adding new contract functions
- Changing existing behavior
- Modifying storage structures
- Updating fee calculations
- Refactoring core logic

**When you can skip the issue:**
- Fixing typos in documentation
- Adding tests for existing functionality
- Fixing obvious bugs with clear solutions
- Improving code comments

#### 2. Fork and Branch

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/swiftremit.git
cd swiftremit

# Add upstream remote
git remote add upstream https://github.com/yourusername/swiftremit.git

# Create a feature branch from main
git checkout -b feature/your-feature-name
```

#### 3. Make Your Changes

- Write clean, well-documented code
- Follow the [Coding Standards](#coding-standards)
- Keep changes focused on a single concern
- Break large changes into smaller, reviewable PRs

#### 4. Write Tests

**Required**: All new functionality must have tests.

- Add unit tests for new functions
- Add integration tests for workflows
- Test error conditions and edge cases
- Ensure all tests pass: `cargo test`

See [Testing Guidelines](#testing-guidelines) for details.

#### 5. Run Quality Checks

Before committing, run these checks locally:

```bash
# Run all tests
make test

# Format code
make fmt

# Run linter (must pass with zero warnings)
make lint

# Build the contract
make build
```

**All checks must pass before submitting your PR.**

#### 6. Test on Testnet (for Contract Changes)

If your changes affect contract logic, test on Stellar testnet:

```bash
# Deploy to testnet
make deploy

# Test your changes
# (Use soroban CLI to invoke functions and verify behavior)

# Document testnet contract ID in PR description
```

See [Testing on Testnet](#testing-on-testnet) for detailed instructions.

#### 7. Commit Your Changes

Write clear, descriptive commit messages:

```bash
# Stage your changes
git add .

# Commit with a descriptive message
git commit -m "Add agent payout confirmation feature

- Implement confirm_payout function
- Add validation for pending status
- Include tests for success and error cases
- Update documentation"
```

**Commit Message Guidelines:**
- Use present tense ("Add feature" not "Added feature")
- Use imperative mood ("Move cursor to..." not "Moves cursor to...")
- First line is a brief summary (50 chars or less)
- Add detailed description after a blank line if needed
- Reference issue numbers when applicable: "Fixes #123"

#### 8. Push to Your Fork

```bash
# Push your branch to your fork
git push origin feature/your-feature-name
```

### Branch Naming Conventions

Use descriptive branch names that indicate the type and purpose of your changes:

**Format**: `<type>/<short-description>`

**Types:**

- **`feature/`** - New features or enhancements
  - Example: `feature/multi-currency-support`
  - Example: `feature/agent-reputation-system`

- **`fix/`** - Bug fixes
  - Example: `fix/fee-calculation-overflow`
  - Example: `fix/authorization-check-missing`

- **`docs/`** - Documentation updates
  - Example: `docs/update-deployment-guide`
  - Example: `docs/add-api-examples`

- **`test/`** - Test additions or improvements
  - Example: `test/add-integration-tests`
  - Example: `test/improve-coverage`

- **`refactor/`** - Code refactoring (no behavior change)
  - Example: `refactor/extract-validation-logic`
  - Example: `refactor/simplify-storage-keys`

- **`perf/`** - Performance improvements
  - Example: `perf/optimize-storage-access`
  - Example: `perf/reduce-gas-costs`

- **`chore/`** - Maintenance tasks, dependency updates
  - Example: `chore/update-soroban-sdk`
  - Example: `chore/improve-makefile`

**Best Practices:**
- Use lowercase with hyphens (kebab-case)
- Keep descriptions short but meaningful
- Be specific: `fix/overflow-in-fee-calc` not `fix/bug`
- Avoid generic names: `feature/improvements` is too vague

### Creating the Pull Request

Once your changes are pushed to your fork, create a PR on GitHub:

#### 1. Open a Pull Request

1. Go to the [SwiftRemit repository](https://github.com/yourusername/swiftremit)
2. Click "Pull requests" → "New pull request"
3. Click "compare across forks"
4. Select your fork and branch
5. Click "Create pull request"

#### 2. Fill Out the PR Template

Provide complete information using our PR template:

**Title**: Clear, concise description of the change
- Good: "Add overflow protection to fee calculation"
- Good: "Fix authorization check in cancel_remittance"
- Bad: "Update code"
- Bad: "Fixes"

**Description**: Explain what and why

```markdown
## Description

Brief summary of the changes and the problem they solve.

## Motivation

Why is this change needed? What problem does it solve?

## Changes Made

- Bullet point list of specific changes
- Be detailed but concise
- Mention any breaking changes

## Related Issues

Fixes #123
Relates to #456
```

**Testing**: Describe how you tested the changes

```markdown
## Testing

### Unit Tests
- Added test_fee_calculation_overflow
- Added test_fee_calculation_edge_cases
- All existing tests pass

### Integration Tests
- Tested complete remittance workflow
- Verified fee accumulation

### Testnet Testing
- Deployed to testnet: CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
- Created test remittances
- Confirmed payouts work correctly
- Verified events are emitted

### Manual Testing
- Tested with zero amounts (correctly rejected)
- Tested with maximum values (overflow handled)
- Tested authorization failures
```

**Checklist**: Complete all applicable items

```markdown
## Checklist

- [ ] Tests pass locally (`make test`)
- [ ] Code is formatted (`make fmt`)
- [ ] Code is linted with zero warnings (`make lint`)
- [ ] Documentation updated (if applicable)
- [ ] Tests added for new functionality
- [ ] Tested on testnet (for contract changes)
- [ ] Breaking changes documented (if applicable)
- [ ] Security considerations addressed (if applicable)
```

**Screenshots/Logs** (if applicable):
- Include relevant output, error messages, or test results
- Add screenshots for documentation changes

### PR Template Example

Here's a complete example:

```markdown
# Add Overflow Protection to Fee Calculation

## Description

This PR adds overflow protection to the fee calculation logic to prevent arithmetic overflow when calculating fees for large remittance amounts.

## Motivation

The current fee calculation uses unchecked arithmetic, which can overflow with large amounts. This could lead to incorrect fee calculations or contract panics. This change ensures the contract fails gracefully with a clear error when overflow would occur.

## Changes Made

- Updated `calculate_fee()` to use `checked_mul()` and `checked_div()`
- Added `ContractError::Overflow` error type
- Added comprehensive tests for overflow scenarios
- Updated documentation to reflect overflow handling

## Related Issues

Fixes #234

## Testing

### Unit Tests
- Added `test_fee_calculation_overflow` - verifies overflow is detected
- Added `test_fee_calculation_max_safe_value` - tests boundary conditions
- Added `test_fee_calculation_normal_values` - ensures normal operation unchanged
- All existing tests pass (25/25)

### Integration Tests
- Tested complete remittance workflow with various amounts
- Verified overflow error propagates correctly

### Testnet Testing
- Deployed to testnet: CBQHNBFQHNBFQHNBFQHNBFQHNBFQHNBFQHNBFQHNBFQHNBFQHNBFQH
- Created remittances with normal amounts (successful)
- Attempted remittance with i128::MAX (correctly rejected with Overflow error)
- Verified existing functionality unaffected

### Manual Testing
- Tested with amounts: 1, 1000, 1000000, i128::MAX
- Verified error messages are clear
- Confirmed no panics occur

## Checklist

- [x] Tests pass locally (`make test`)
- [x] Code is formatted (`make fmt`)
- [x] Code is linted with zero warnings (`make lint`)
- [x] Documentation updated
- [x] Tests added for new functionality
- [x] Tested on testnet
- [ ] Breaking changes documented (N/A - no breaking changes)
- [x] Security considerations addressed

## Security Considerations

This change improves security by preventing arithmetic overflow, which could lead to incorrect fee calculations or loss of funds. All arithmetic operations now use checked variants that return errors instead of overflowing silently.

## Screenshots

```
$ cargo test test_fee_calculation_overflow
running 1 test
test test_fee_calculation_overflow ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 24 filtered out
```
```

### Code Review Process

After submitting your PR, it will go through our review process:

#### 1. Automated Checks

GitHub Actions will automatically run:
- **Tests** - All tests must pass
- **Formatting** - Code must be formatted with `rustfmt`
- **Linting** - Code must pass `clippy` with zero warnings
- **Build** - Contract must build successfully

**If automated checks fail:**
1. Review the error messages in the GitHub Actions logs
2. Fix the issues locally
3. Push the fixes to your branch
4. Checks will run automatically on the new commits

#### 2. Maintainer Review

A project maintainer will review your PR within **2-3 business days**.

**What reviewers look for:**
- **Correctness** - Does the code work as intended?
- **Security** - Are there any security concerns?
- **Testing** - Is the code adequately tested?
- **Code Quality** - Is the code clean, readable, and maintainable?
- **Documentation** - Is the code and functionality documented?
- **Standards** - Does it follow our coding standards?
- **Scope** - Is the PR focused on a single concern?

**Review outcomes:**
- **Approved** - PR is ready to merge
- **Changes Requested** - Reviewer requests modifications
- **Comments** - Reviewer has questions or suggestions

#### 3. Responding to Feedback

When a reviewer requests changes or asks questions:

**Do:**
- Respond promptly (within a few days)
- Address all feedback points
- Ask for clarification if you don't understand
- Be open to suggestions and alternative approaches
- Make requested changes and push new commits
- Mark conversations as resolved when addressed
- Thank reviewers for their time and feedback

**Don't:**
- Take feedback personally - it's about the code, not you
- Ignore feedback or leave comments unaddressed
- Force-push and rewrite history (unless explicitly asked)
- Argue without understanding the reviewer's perspective
- Rush to merge without addressing concerns

**Example responses:**

Good:
```markdown
> Consider using checked_mul here to prevent overflow

Good catch! I've updated the code to use checked_mul and added a test for the overflow case. Thanks!
```

Good:
```markdown
> This function is getting complex. Can we extract some logic?

I agree it's getting hard to follow. I've extracted the validation logic into a separate helper function. Let me know if this is clearer.
```

Good:
```markdown
> Why did you choose this approach over X?

I chose this approach because it minimizes storage operations, which reduces gas costs. Approach X would require an additional storage read on each call. However, if you think the tradeoff isn't worth it, I'm happy to change it.
```

#### 4. Making Changes

To address review feedback:

```bash
# Make the requested changes
# Edit files as needed

# Run quality checks
make test
make fmt
make lint

# Commit the changes
git add .
git commit -m "Address review feedback

- Extract validation logic into helper function
- Add overflow protection to fee calculation
- Improve test coverage for edge cases"

# Push to your branch
git push origin feature/your-feature-name
```

**The PR will automatically update with your new commits.**

#### 5. Iterating

The review process may involve multiple rounds:
1. Reviewer provides feedback
2. You make changes
3. Reviewer reviews again
4. Repeat until approved

**This is normal and expected!** Multiple review rounds lead to better code quality.

### Merge Criteria

Your PR will be merged when it meets all of these criteria:

#### Required for All PRs

- [ ] **All automated checks pass**
  - Tests pass (`cargo test`)
  - Code is formatted (`cargo fmt --check`)
  - Linting passes with zero warnings (`cargo clippy`)
  - Build succeeds

- [ ] **Code review approved**
  - At least one maintainer has approved the PR
  - All requested changes have been addressed
  - All review conversations are resolved

- [ ] **No merge conflicts**
  - Branch is up to date with `main`
  - No conflicting changes with other merged PRs

- [ ] **Documentation updated**
  - Code changes are documented with doc comments
  - README or other docs updated if functionality changed
  - API changes documented

- [ ] **Follows coding standards**
  - Adheres to Rust and Soroban best practices
  - Follows project conventions and patterns
  - Code is clean, readable, and maintainable

#### Additional Requirements for Contract Changes

- [ ] **Comprehensive tests**
  - Unit tests for all new functions
  - Integration tests for workflows
  - Error condition tests
  - Security tests (authorization, validation, overflow)

- [ ] **Testnet validation**
  - Changes tested on Stellar testnet
  - Testnet contract ID provided in PR
  - Test results documented

- [ ] **Security review**
  - Security considerations addressed
  - No prohibited practices (unwrap, unsafe, unchecked arithmetic)
  - Authorization checks in place
  - Input validation implemented

#### Additional Requirements for Breaking Changes

- [ ] **Breaking changes documented**
  - Clear description of what breaks
  - Migration guide provided
  - Rationale explained

- [ ] **Discussed with maintainers**
  - Breaking changes discussed in issue before implementation
  - Consensus reached on approach

### Merge Process

Once all criteria are met:

1. **Maintainer merges the PR**
   - We use "Squash and merge" to keep history clean
   - Your commits will be squashed into a single commit
   - Commit message will be based on PR title and description

2. **Branch is deleted**
   - Your feature branch is automatically deleted after merge
   - Your fork remains unchanged

3. **PR is closed**
   - PR is marked as merged and closed
   - Linked issues are automatically closed (if using "Fixes #123")

4. **Contributor recognition**
   - You're added to the contributors list
   - Your contribution is noted in release notes
   - First-time contributors are celebrated! 🎉

### After Your PR is Merged

Congratulations! Your contribution is now part of SwiftRemit. Here's what happens next:

#### 1. Update Your Fork

Keep your fork in sync with the main repository:

```bash
# Switch to main branch
git checkout main

# Fetch upstream changes
git fetch upstream

# Merge upstream changes
git merge upstream/main

# Push to your fork
git push origin main

# Delete your feature branch (optional)
git branch -d feature/your-feature-name
git push origin --delete feature/your-feature-name
```

#### 2. Monitor for Issues

- Watch for any issues related to your changes
- Be available to answer questions or provide clarification
- Help fix any bugs that are discovered

#### 3. Contribute More!

- Look for more issues to work on
- Help review other contributors' PRs
- Share your experience with newcomers
- Suggest new features or improvements

### Before Submitting Checklist

Use this checklist before creating your PR:

#### Code Quality
- [ ] Code follows [Coding Standards](#coding-standards)
- [ ] All functions have doc comments
- [ ] Code is clean and readable
- [ ] No commented-out code or debug statements
- [ ] No `unwrap()`, `expect()`, or `panic!()` in production code
- [ ] All arithmetic uses checked operations

#### Testing
- [ ] All tests pass: `cargo test`
- [ ] New functionality has unit tests
- [ ] Error conditions are tested
- [ ] Integration tests added for workflows
- [ ] Security tests added for sensitive operations
- [ ] Tested on testnet (for contract changes)

#### Quality Checks
- [ ] Code is formatted: `cargo fmt`
- [ ] Linting passes: `cargo clippy` (zero warnings)
- [ ] Build succeeds: `cargo build --target wasm32-unknown-unknown --release`
- [ ] No compiler warnings

#### Documentation
- [ ] Public functions have doc comments
- [ ] Doc comments include parameter descriptions
- [ ] Doc comments include return value descriptions
- [ ] Doc comments include error conditions
- [ ] README updated (if functionality changed)
- [ ] Examples added for complex features

#### Security (for contract changes)
- [ ] Authorization checks in place
- [ ] Input validation implemented
- [ ] Overflow protection added
- [ ] Error handling is comprehensive
- [ ] No security vulnerabilities introduced
- [ ] Security tests added

#### PR Preparation
- [ ] Branch name follows conventions
- [ ] Commits have clear messages
- [ ] PR title is descriptive
- [ ] PR description is complete
- [ ] Testing section filled out
- [ ] Checklist completed
- [ ] Related issues linked

### Common PR Mistakes to Avoid

**Avoid these common pitfalls:**

1. **Too large** - PRs with hundreds of lines are hard to review
   - **Solution**: Break into smaller, focused PRs

2. **Missing tests** - New code without tests
   - **Solution**: Write tests before submitting

3. **Failing checks** - Submitting with failing tests or linting errors
   - **Solution**: Run all checks locally first

4. **Poor description** - Vague or missing PR description
   - **Solution**: Use the template and be thorough

5. **Unrelated changes** - Multiple unrelated changes in one PR
   - **Solution**: Keep PRs focused on a single concern

6. **Merge conflicts** - Not keeping branch up to date
   - **Solution**: Regularly sync with main branch

7. **Ignoring feedback** - Not responding to review comments
   - **Solution**: Address all feedback promptly

8. **Force pushing** - Rewriting history after review
   - **Solution**: Add new commits instead of force pushing

9. **No testnet testing** - Skipping testnet validation for contract changes
   - **Solution**: Always test contract changes on testnet

10. **Breaking changes without discussion** - Submitting breaking changes without prior discussion
    - **Solution**: Open an issue first to discuss breaking changes

### Getting Help with PRs

If you need help with the PR process:

- **Review the PR template** - It guides you through what's needed
- **Look at merged PRs** - See examples of good PRs
- **Ask in your PR** - Mention you're new and ask for guidance
- **GitHub Discussions** - Ask general questions about the process
- **Discord** - Get real-time help from the community

### PR Review Timeline

**Expected timelines:**

- **Initial review**: 2-3 business days
- **Follow-up reviews**: 1-2 business days
- **Merge after approval**: Usually within 1 day

**Factors that may delay review:**
- Holidays or weekends
- Large or complex PRs
- Breaking changes requiring discussion
- Security-sensitive changes requiring thorough review

**If your PR hasn't been reviewed:**
- Wait at least 3 business days
- Politely ping in a comment: "Friendly ping for review when you have time"
- Ensure all automated checks are passing
- Check that your PR is ready for review (not marked as draft)

### Draft Pull Requests

Use draft PRs for work in progress:

```markdown
# Create a draft PR for early feedback
1. Create PR as usual
2. Select "Create draft pull request" instead of "Create pull request"
3. Add [WIP] or [Draft] to the title
```

**When to use drafts:**
- You want early feedback on your approach
- You're working on a large feature incrementally
- You want to show progress but it's not ready for review
- You need help or guidance

**Converting to ready:**
- Click "Ready for review" when your PR is complete
- Ensure all checks pass before converting
- Complete the PR template

---

## Contribution Guidelines by Type

This section provides specific guidelines for different types of contributions. Understanding these guidelines helps ensure your contribution is well-structured, properly tested, and likely to be accepted.

---

### Bug Fixes

Bug fixes address defects in the contract logic, incorrect behavior, or issues that prevent the contract from working as intended.

#### Guidelines for Bug Fixes

**Before You Start:**
1. **Verify the bug** - Confirm the issue exists and is reproducible
2. **Check existing issues** - See if the bug is already reported
3. **Understand the root cause** - Don't just fix symptoms; fix the underlying problem
4. **Consider impact** - Assess how widespread the bug is and who it affects

**When to Open an Issue First:**
- **Always** for security-related bugs (report privately if critical)
- **Always** for bugs affecting fund safety or contract state
- **Usually** for complex bugs requiring discussion of the fix approach
- **Optional** for obvious bugs with clear, simple fixes (typos, missing validation, etc.)

**Implementation Requirements:**
- **Minimal changes** - Fix only what's broken; don't refactor unrelated code
- **Root cause fix** - Address the underlying issue, not just the symptom
- **Backward compatibility** - Avoid breaking changes unless absolutely necessary
- **Error handling** - Ensure the fix includes proper error handling

**Testing Requirements:**
- **Reproduction test** - Add a test that fails before your fix and passes after
- **Regression tests** - Ensure the fix doesn't break existing functionality
- **Edge cases** - Test boundary conditions related to the bug
- **Related scenarios** - Test similar code paths that might have the same issue

**Documentation Requirements:**
- **Code comments** - Explain why the fix is necessary (reference the bug)
- **Commit message** - Clearly describe the bug and the fix
- **PR description** - Include reproduction steps and how the fix resolves it

**Example Bug Fix PR:**
```markdown
# Fix: Prevent overflow in fee calculation for large amounts

## Bug Description
When creating a remittance with an amount close to i128::MAX, the fee 
calculation overflows, causing a panic instead of returning an error.

## Root Cause
The `calculate_fee()` function uses unchecked multiplication, which 
overflows with large amounts.

## Fix
Replace unchecked arithmetic with `checked_mul()` and `checked_div()`, 
returning `ContractError::Overflow` when overflow would occur.

## Testing
- Added `test_fee_calculation_overflow` - verifies overflow is caught
- Added `test_fee_calculation_large_amounts` - tests near-max values
- All existing tests pass

## Impact
- Prevents contract panics with large amounts
- Provides clear error message to users
- No breaking changes to existing functionality
```

**Common Bug Fix Patterns:**

1. **Missing Validation**
   ```rust
   // Before (bug)
   pub fn create_remittance(env: Env, amount: i128) -> Result<u64, ContractError> {
       // Missing amount validation
       let fee = calculate_fee(amount, fee_bps)?;
       // ...
   }
   
   // After (fixed)
   pub fn create_remittance(env: Env, amount: i128) -> Result<u64, ContractError> {
       if amount <= 0 {
           return Err(ContractError::InvalidAmount);
       }
       let fee = calculate_fee(amount, fee_bps)?;
       // ...
   }
   ```

2. **Missing Authorization**
   ```rust
   // Before (bug)
   pub fn cancel_remittance(env: Env, id: u64) -> Result<(), ContractError> {
       // Missing authorization check
       let remittance = get_remittance(&env, id)?;
       // ...
   }
   
   // After (fixed)
   pub fn cancel_remittance(env: Env, sender: Address, id: u64) -> Result<(), ContractError> {
       sender.require_auth();  // Added authorization
       let remittance = get_remittance(&env, id)?;
       // ...
   }
   ```

3. **Incorrect Error Handling**
   ```rust
   // Before (bug)
   pub fn get_remittance_status(env: Env, id: u64) -> RemittanceStatus {
       let remittance = get_remittance(&env, id).unwrap();  // Panics if not found
       remittance.status
   }
   
   // After (fixed)
   pub fn get_remittance_status(env: Env, id: u64) -> Result<RemittanceStatus, ContractError> {
       let remittance = get_remittance(&env, id)?;  // Returns error if not found
       Ok(remittance.status)
   }
   ```

---

### New Features

New features add functionality that doesn't currently exist in the contract. This includes new contract functions, enhanced capabilities, or additional workflows.

#### Guidelines for New Features

**Before You Start:**
1. **Open an issue first** - **Always** discuss new features before implementing
2. **Explain the use case** - Why is this feature needed? Who will use it?
3. **Consider alternatives** - Are there other ways to achieve the same goal?
4. **Assess impact** - How does this affect gas costs, storage, and existing functionality?
5. **Get consensus** - Ensure maintainers agree the feature should be added

**When to Open an Issue First:**
- **Always** - New features should always be discussed before implementation
- Explain the problem the feature solves
- Describe the proposed solution
- Discuss trade-offs and alternatives
- Get feedback on the approach before coding

**Design Considerations:**
- **Minimal scope** - Start with the simplest version that solves the problem
- **Backward compatibility** - Don't break existing functionality
- **Gas efficiency** - Minimize storage operations and computational complexity
- **Security** - Consider authorization, validation, and potential attack vectors
- **Consistency** - Follow existing patterns and conventions in the codebase

**Implementation Requirements:**
- **Clean code** - Well-structured, readable, and maintainable
- **Documentation** - Comprehensive doc comments for all new public functions
- **Error handling** - Proper error types and handling for all failure cases
- **Events** - Emit events for important state changes
- **Authorization** - Require proper authorization for state-changing operations
- **Validation** - Validate all inputs thoroughly

**Testing Requirements:**
- **Unit tests** - Test each new function independently
- **Integration tests** - Test the feature in complete workflows
- **Happy path tests** - Verify the feature works as intended
- **Error condition tests** - Test all error cases and edge conditions
- **Authorization tests** - Verify only authorized users can access the feature
- **Gas cost tests** - Measure and document gas costs
- **Testnet validation** - Deploy and test on Stellar testnet

**Documentation Requirements:**
- **Doc comments** - Complete documentation for all new functions
- **README updates** - Add the feature to the main README
- **API documentation** - Update API.md with new functions
- **Examples** - Provide usage examples for complex features
- **Migration guide** - If the feature changes existing behavior

**Example New Feature PR:**
```markdown
# Feature: Add batch remittance creation

## Motivation
Users often need to create multiple remittances at once (e.g., payroll). 
Creating them one-by-one is inefficient and expensive due to multiple 
transactions.

## Solution
Add a `create_remittances_batch()` function that accepts multiple 
remittance requests and creates them in a single transaction.

## Design Decisions
- Limit batch size to 10 to prevent excessive gas costs
- Use atomic execution: if any remittance fails, all fail
- Return array of remittance IDs in the same order as inputs
- Emit individual events for each remittance created

## Implementation
- Added `create_remittances_batch()` function in lib.rs
- Added `BatchRemittanceRequest` type in types.rs
- Added validation for batch size and individual requests
- Added comprehensive error handling

## Testing
- Unit tests for batch creation with various sizes
- Error tests for oversized batches, invalid requests
- Integration test for complete batch workflow
- Gas cost comparison: batch vs individual (40% savings)
- Testnet deployment and testing completed

## Documentation
- Added doc comments with examples
- Updated README with batch creation section
- Updated API.md with new function signature
- Added example script for batch usage

## Breaking Changes
None - this is a new function, existing functionality unchanged.
```

**Feature Checklist:**
- [ ] Issue created and discussed with maintainers
- [ ] Design approved before implementation
- [ ] Implementation follows coding standards
- [ ] All new functions have doc comments
- [ ] Comprehensive unit tests added
- [ ] Integration tests added
- [ ] Error conditions tested
- [ ] Authorization tested
- [ ] Tested on testnet
- [ ] Gas costs measured and documented
- [ ] README updated
- [ ] API documentation updated
- [ ] Examples provided
- [ ] No breaking changes (or documented if unavoidable)

---

### Documentation Improvements

Documentation improvements enhance the clarity, completeness, and accessibility of project documentation. This includes code comments, README files, guides, and examples.

#### Guidelines for Documentation Improvements

**Types of Documentation Contributions:**

1. **Code Documentation (Doc Comments)**
   - Adding missing doc comments to functions
   - Improving existing doc comments with better descriptions
   - Adding parameter and return value descriptions
   - Documenting error conditions
   - Adding usage examples

2. **Project Documentation**
   - Improving README.md clarity
   - Enhancing setup instructions
   - Adding or improving examples
   - Creating tutorials or guides
   - Updating deployment documentation

3. **API Documentation**
   - Documenting contract functions
   - Adding function signatures and parameters
   - Providing usage examples
   - Documenting error codes

4. **Corrections**
   - Fixing typos and grammar errors
   - Correcting outdated information
   - Fixing broken links
   - Updating version numbers

**When to Open an Issue First:**
- **Optional** for minor fixes (typos, grammar, formatting)
- **Recommended** for significant documentation additions or restructuring
- **Always** for new documentation files or major changes to existing structure

**Documentation Standards:**

**Code Documentation Format:**
```rust
/// Creates a new remittance and transfers USDC to escrow.
///
/// This function initiates a remittance by transferring the specified amount
/// of USDC from the sender to the contract's escrow. The contract holds the
/// funds until the agent confirms payout or the sender cancels.
///
/// # Arguments
/// * `sender` - The address sending the remittance (must authorize)
/// * `agent` - The registered agent who will handle the payout
/// * `amount` - The amount in USDC stroops (1 USDC = 10,000,000 stroops)
///
/// # Returns
/// The unique remittance ID that can be used to track and manage the remittance.
///
/// # Errors
/// * `InvalidAmount` - If amount is zero or negative
/// * `AgentNotRegistered` - If the specified agent is not registered
/// * `Overflow` - If fee calculation overflows
///
/// # Examples
/// ```
/// let remittance_id = contract.create_remittance(
///     &sender_address,
///     &agent_address,
///     &10_000_000  // 1 USDC
/// );
/// ```
///
/// # Authorization
/// Requires authorization from the `sender` address.
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // Implementation
}
```

**README Documentation Guidelines:**
- **Clear structure** - Use headings and sections logically
- **Step-by-step instructions** - Make setup and usage easy to follow
- **Code examples** - Provide working examples that users can copy
- **Prerequisites** - List all requirements upfront
- **Troubleshooting** - Include common issues and solutions
- **Links** - Provide links to related documentation

**Testing Documentation Changes:**
- **Build docs** - Run `cargo doc` to ensure doc comments compile
- **Check links** - Verify all links work correctly
- **Review formatting** - Ensure markdown renders correctly
- **Test examples** - Verify code examples actually work
- **Proofread** - Check for typos and grammar errors

**Example Documentation PR:**
```markdown
# Docs: Add comprehensive examples to README

## Changes
- Added "Quick Start" section with minimal example
- Added "Common Use Cases" section with 5 detailed examples
- Added "Error Handling" section with error code reference
- Fixed broken links to API documentation
- Updated testnet deployment instructions

## Motivation
New contributors reported difficulty understanding how to use the contract.
The README had function signatures but lacked practical examples.

## Testing
- Built documentation: `cargo doc --no-deps --open`
- Verified all links work
- Tested all code examples on testnet
- Proofread for typos and clarity
```

**Documentation Checklist:**
- [ ] Changes improve clarity and completeness
- [ ] All code examples are tested and work
- [ ] Links are valid and point to correct resources
- [ ] Markdown formatting is correct
- [ ] Spelling and grammar are correct
- [ ] Technical accuracy verified
- [ ] Consistent with existing documentation style

---

### Test Additions

Test additions improve code coverage, verify edge cases, and ensure contract reliability. Tests are critical for a financial smart contract where correctness is paramount.

#### Guidelines for Test Additions

**Types of Test Contributions:**

1. **Coverage Improvements**
   - Adding tests for uncovered code paths
   - Testing functions that lack adequate tests
   - Improving overall test coverage percentage

2. **Edge Case Testing**
   - Testing boundary conditions (zero, max values, etc.)
   - Testing unusual but valid inputs
   - Testing state transitions at boundaries

3. **Error Condition Testing**
   - Testing all error paths
   - Verifying correct error codes are returned
   - Testing error messages are helpful

4. **Integration Testing**
   - Testing multi-function workflows
   - Testing interactions between components
   - Testing realistic user scenarios

5. **Security Testing**
   - Testing authorization failures
   - Testing with malicious inputs
   - Testing reentrancy scenarios
   - Testing overflow conditions

**When to Open an Issue First:**
- **Optional** for straightforward test additions
- **Recommended** if you're adding a new test category or framework
- **Always** if you're proposing changes to the testing infrastructure

**Test Writing Guidelines:**

**Test Structure:**
```rust
#[test]
fn test_descriptive_name_of_what_is_tested() {
    // 1. Setup - Create test environment and dependencies
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract = create_swiftremit_contract(&env);
    let token = create_token_contract(&env, &admin);
    
    // 2. Initialize - Set up contract state
    contract.initialize(&admin, &token.address, &250);
    
    // 3. Execute - Perform the action being tested
    let result = contract.create_remittance(&sender, &agent, &amount);
    
    // 4. Assert - Verify the expected outcome
    assert_eq!(result, 1);
    assert_eq!(contract.get_remittance(1).status, RemittanceStatus::Pending);
}
```

**Test Naming Conventions:**
- Use descriptive names that explain what is being tested
- Format: `test_<function>_<scenario>_<expected_outcome>`
- Examples:
  - `test_create_remittance_with_valid_inputs_succeeds()`
  - `test_create_remittance_with_zero_amount_fails()`
  - `test_cancel_remittance_by_non_sender_fails()`

**Testing Best Practices:**
- **One assertion per test** - Test one thing at a time (when possible)
- **Descriptive names** - Test name should explain what's being tested
- **Independent tests** - Tests should not depend on each other
- **Deterministic** - Tests should always produce the same result
- **Fast** - Tests should run quickly
- **Clear assertions** - Use descriptive assertion messages

**Error Testing Pattern:**
```rust
#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_create_remittance_with_zero_amount_fails() {
    let env = Env::default();
    env.mock_all_auths();
    // Setup...
    
    // This should panic with InvalidAmount error (code 3)
    contract.create_remittance(&sender, &agent, &0);
}
```

**Integration Test Pattern:**
```rust
#[test]
fn test_complete_remittance_workflow() {
    let env = Env::default();
    env.mock_all_auths();
    // Setup...
    
    // Create remittance
    let id = contract.create_remittance(&sender, &agent, &1_000_000);
    assert_eq!(contract.get_remittance(id).status, RemittanceStatus::Pending);
    
    // Confirm payout
    contract.confirm_payout(&agent, id);
    assert_eq!(contract.get_remittance(id).status, RemittanceStatus::Completed);
    
    // Verify balances
    assert_eq!(token.balance(&agent), expected_payout);
    assert_eq!(contract.get_accumulated_fees(), expected_fee);
}
```

**Security Test Pattern:**
```rust
#[test]
#[should_panic(expected = "require_auth")]
fn test_confirm_payout_without_authorization_fails() {
    let env = Env::default();
    // Don't mock auths - test real authorization
    // Setup...
    
    // This should fail authorization check
    contract.confirm_payout(&unauthorized_address, id);
}
```

**Example Test Addition PR:**
```markdown
# Test: Add edge case tests for fee calculation

## Motivation
The fee calculation function lacks tests for edge cases like maximum values,
zero fees, and rounding behavior. This PR adds comprehensive edge case coverage.

## Tests Added
- `test_fee_calculation_with_zero_fee_bps` - Verifies 0% fee works
- `test_fee_calculation_with_max_fee_bps` - Tests 100% fee (10000 bps)
- `test_fee_calculation_rounding_down` - Verifies rounding behavior
- `test_fee_calculation_with_max_safe_amount` - Tests near i128::MAX
- `test_fee_calculation_with_one_stroop` - Tests minimum amount

## Coverage Impact
- Fee calculation coverage: 75% → 95%
- Overall contract coverage: 82% → 85%

## Test Results
All tests pass:
```
running 5 tests
test test_fee_calculation_with_zero_fee_bps ... ok
test test_fee_calculation_with_max_fee_bps ... ok
test test_fee_calculation_rounding_down ... ok
test test_fee_calculation_with_max_safe_amount ... ok
test test_fee_calculation_with_one_stroop ... ok

test result: ok. 5 passed; 0 failed
```
```

**Test Addition Checklist:**
- [ ] Tests follow naming conventions
- [ ] Tests are independent and deterministic
- [ ] Tests have clear setup, execute, assert structure
- [ ] Error tests use `#[should_panic]` with expected error
- [ ] Integration tests cover realistic workflows
- [ ] Security tests verify authorization and validation
- [ ] All tests pass locally
- [ ] Tests improve overall coverage
- [ ] Tests are well-documented with comments

---

### Performance Optimizations

Performance optimizations improve the contract's efficiency, reduce gas costs, or enhance execution speed. In Soroban contracts, gas optimization is critical for user experience and cost-effectiveness.

#### Guidelines for Performance Optimizations

**Types of Performance Optimizations:**

1. **Storage Optimization**
   - Reducing storage reads/writes
   - Using more efficient storage patterns
   - Caching frequently accessed data
   - Eliminating redundant storage operations

2. **Computational Optimization**
   - Simplifying calculations
   - Reducing loop iterations
   - Using more efficient algorithms
   - Eliminating unnecessary operations

3. **Gas Cost Reduction**
   - Minimizing contract size
   - Reducing instruction count
   - Optimizing data structures
   - Batching operations

**When to Open an Issue First:**
- **Always** - Performance optimizations should be discussed before implementation
- Explain the current performance issue
- Provide measurements showing the problem
- Describe the proposed optimization
- Discuss trade-offs (complexity vs performance)
- Get consensus on whether the optimization is worth the complexity

**Optimization Process:**

**1. Measure First**
- **Identify the bottleneck** - Don't optimize without data
- **Measure current performance** - Establish baseline metrics
- **Set optimization goals** - Define what "better" means
- **Profile the code** - Find the actual slow parts

**2. Optimize**
- **Make targeted changes** - Optimize the bottleneck, not everything
- **Keep it simple** - Don't sacrifice readability for minor gains
- **Maintain correctness** - Never trade correctness for performance
- **Document trade-offs** - Explain why the optimization is worth it

**3. Measure Again**
- **Verify improvement** - Measure the optimized version
- **Compare results** - Show before/after metrics
- **Check for regressions** - Ensure nothing else got slower
- **Document results** - Include measurements in PR

**Optimization Guidelines:**

**Storage Optimization Example:**
```rust
// Before: 3 storage reads
pub fn confirm_payout(env: Env, agent: Address, id: u64) -> Result<(), ContractError> {
    agent.require_auth();
    
    let remittance = get_remittance(&env, id)?;  // Read 1
    let fee_bps = get_platform_fee_bps(&env)?;   // Read 2
    let fee = calculate_fee(remittance.amount, fee_bps)?;
    
    // ... transfer logic ...
    
    let remittance = get_remittance(&env, id)?;  // Read 3 (redundant!)
    remittance.status = RemittanceStatus::Completed;
    set_remittance(&env, id, &remittance);
    
    Ok(())
}

// After: 2 storage reads (eliminated redundant read)
pub fn confirm_payout(env: Env, agent: Address, id: u64) -> Result<(), ContractError> {
    agent.require_auth();
    
    let mut remittance = get_remittance(&env, id)?;  // Read 1
    let fee_bps = get_platform_fee_bps(&env)?;       // Read 2
    let fee = calculate_fee(remittance.amount, fee_bps)?;
    
    // ... transfer logic ...
    
    // Reuse the remittance we already loaded
    remittance.status = RemittanceStatus::Completed;
    set_remittance(&env, id, &remittance);
    
    Ok(())
}

// Gas savings: ~15% reduction in function cost
```

**Computational Optimization Example:**
```rust
// Before: Multiple divisions
pub fn calculate_fee(amount: i128, fee_bps: u32) -> Result<i128, ContractError> {
    let fee = amount
        .checked_mul(fee_bps as i128)?
        .checked_div(10000)?;
    
    let adjusted = fee
        .checked_div(100)?
        .checked_mul(100)?;  // Unnecessary operations
    
    Ok(adjusted)
}

// After: Simplified calculation
pub fn calculate_fee(amount: i128, fee_bps: u32) -> Result<i128, ContractError> {
    let fee = amount
        .checked_mul(fee_bps as i128)?
        .checked_div(10000)?;
    
    Ok(fee)
}

// Gas savings: ~8% reduction in fee calculation cost
```

**Testing Requirements for Optimizations:**
- **Correctness tests** - Verify the optimization doesn't change behavior
- **Performance tests** - Measure and document the improvement
- **Regression tests** - Ensure no other functionality is affected
- **Edge case tests** - Verify optimization works in all scenarios

**Documentation Requirements:**
- **Explain the optimization** - What was slow and why
- **Show measurements** - Before/after gas costs or execution time
- **Document trade-offs** - Any complexity added or limitations introduced
- **Update comments** - Explain why the code is structured this way

**Example Performance Optimization PR:**
```markdown
# Perf: Eliminate redundant storage read in confirm_payout

## Problem
The `confirm_payout` function reads the remittance from storage twice:
once for validation and once before updating the status. This redundant
read increases gas costs unnecessarily.

## Measurements (Before)
- Gas cost: ~45,000 units
- Storage reads: 3
- Storage writes: 2

## Solution
Reuse the remittance loaded during validation instead of reading it again.
Change the binding to `mut` to allow status update.

## Measurements (After)
- Gas cost: ~38,000 units (15% reduction)
- Storage reads: 2 (33% reduction)
- Storage writes: 2 (unchanged)

## Testing
- All existing tests pass (behavior unchanged)
- Added performance benchmark test
- Verified on testnet with identical results

## Trade-offs
- Minimal: Added `mut` keyword to variable binding
- No complexity increase
- No behavior changes
- Pure optimization with no downsides

## Code Changes
- Modified `confirm_payout` to reuse loaded remittance
- Updated variable binding from `let` to `let mut`
- Removed redundant `get_remittance` call
```

**Performance Optimization Checklist:**
- [ ] Issue created with performance measurements
- [ ] Bottleneck identified with profiling/measurement
- [ ] Baseline metrics documented
- [ ] Optimization implemented with minimal complexity
- [ ] Correctness verified with tests
- [ ] Performance improvement measured and documented
- [ ] No regressions in other areas
- [ ] Trade-offs clearly explained
- [ ] Code comments explain optimization rationale
- [ ] Tested on testnet

**Important Notes:**
- **Correctness first** - Never sacrifice correctness for performance
- **Measure, don't guess** - Always measure before and after
- **Significant improvements only** - Don't optimize for <5% gains
- **Readability matters** - Don't make code unreadable for minor gains
- **Document everything** - Future maintainers need to understand why

---

### When to Open an Issue Before Submitting a PR

Opening an issue before starting work helps ensure your contribution will be accepted and saves time for everyone involved.

#### Always Open an Issue First For:

**1. New Features**
- Any functionality that doesn't currently exist
- Changes to the contract's public API
- New storage structures or patterns
- Features that affect gas costs significantly

**2. Breaking Changes**
- Changes that affect existing functionality
- Modifications to function signatures
- Changes to data structures
- Alterations to contract behavior

**3. Major Refactoring**
- Large-scale code reorganization
- Architectural changes
- Changes affecting multiple files
- Performance optimizations with trade-offs

**4. Security-Related Changes**
- Changes to authorization logic
- Modifications to validation
- Updates to error handling
- Changes affecting fund safety

**5. Performance Optimizations**
- Gas cost reductions
- Storage optimizations
- Computational improvements
- Any optimization requiring code complexity

**6. Unclear Requirements**
- When you're not sure if a change is desired
- When multiple approaches are possible
- When trade-offs need discussion
- When you need guidance

#### You Can Skip the Issue For:

**1. Obvious Bug Fixes**
- Clear defects with obvious fixes
- Typos in code or comments
- Missing error handling for clear error cases

**2. Documentation Improvements**
- Typo corrections
- Grammar fixes
- Clarifying existing documentation
- Adding missing examples

**3. Test Additions**
- Adding tests for existing functionality
- Improving test coverage
- Adding edge case tests

**4. Minor Code Quality Improvements**
- Formatting fixes
- Removing unused code
- Simplifying obvious complexity
- Adding helpful comments

#### How to Open a Good Issue

**1. Use a descriptive title**
- Good: "Add support for multi-currency remittances"
- Bad: "New feature"

**2. Provide context**
- Explain the problem or need
- Describe the current behavior
- Explain the desired behavior

**3. Propose a solution** (optional)
- Suggest an implementation approach
- Discuss alternatives
- Note any concerns or trade-offs

**4. Add relevant labels**
- `enhancement` for new features
- `bug` for defects
- `documentation` for docs
- `performance` for optimizations
- `question` for discussions

**5. Be open to feedback**
- Maintainers may suggest different approaches
- Be willing to adjust your proposal
- Engage in constructive discussion

#### Issue Template Example

```markdown
## Problem Description
Clear description of the problem or need.

## Current Behavior
How the contract currently works (if applicable).

## Desired Behavior
How you'd like it to work.

## Proposed Solution
Your suggested implementation approach (optional).

## Alternatives Considered
Other approaches you thought about (optional).

## Additional Context
Any other relevant information, examples, or references.
```

#### Benefits of Opening Issues First

- **Avoid wasted effort** - Ensure your work will be accepted
- **Get early feedback** - Improve your approach before coding
- **Coordinate with others** - Avoid duplicate work
- **Build consensus** - Ensure stakeholders agree on the approach
- **Document decisions** - Create a record of why changes were made

---


## Documentation Standards

Comprehensive documentation is essential for SwiftRemit as a production-ready smart contract. This section defines our documentation requirements and standards for code documentation, project documentation, and examples.

Good documentation:
- **Helps contributors understand the codebase** - Clear explanations of what code does and why
- **Reduces onboarding time** - New contributors can get up to speed quickly
- **Prevents bugs** - Documented behavior is less likely to be misunderstood
- **Serves as a contract** - Documents the expected behavior and API
- **Improves maintainability** - Future developers understand design decisions

---

### Code Documentation Requirements

All public functions, structs, enums, and modules must have comprehensive doc comments. This is not optional - undocumented public items will not be accepted in pull requests.

#### Public Functions

**Requirement**: Every public function must have a doc comment that includes:
1. **Brief description** - One-line summary of what the function does
2. **Detailed explanation** - Additional context if the function is complex
3. **Arguments section** - Description of each parameter
4. **Returns section** - Description of the return value
5. **Errors section** - All possible error conditions
6. **Examples section** - Usage example for complex functions

**Format**:
```rust
/// Creates a new remittance and transfers USDC to escrow.
///
/// The sender must authorize this transaction. The specified agent must be
/// registered before creating a remittance. A platform fee is calculated
/// based on the configured fee percentage and deducted upon payout.
///
/// # Arguments
///
/// * `env` - The contract environment
/// * `sender` - The address sending the remittance
/// * `agent` - The registered agent who will handle the payout
/// * `amount` - The amount in USDC stroops (1 USDC = 10,000,000 stroops)
///
/// # Returns
///
/// The unique remittance ID that can be used to query or manage the remittance
///
/// # Errors
///
/// * `InvalidAmount` - If amount is zero or negative
/// * `AgentNotRegistered` - If the specified agent is not registered
/// * `NotInitialized` - If the contract has not been initialized
/// * `Overflow` - If fee calculation causes arithmetic overflow
///
/// # Examples
///
/// ```ignore
/// let remittance_id = contract.create_remittance(
///     env,
///     sender_address,
///     agent_address,
///     100_000_000,  // 10 USDC
/// )?;
/// ```
pub fn create_remittance(
    env: Env,
    sender: Address,
    agent: Address,
    amount: i128,
) -> Result<u64, ContractError> {
    // Implementation
}
```

**Key Points**:
- Use `///` for doc comments (not `//`)
- Start with a verb in present tense ("Creates", "Returns", "Validates")
- Be specific about units (stroops, basis points, etc.)
- Document all error conditions, not just common ones
- Include examples for functions with non-obvious usage
- Use `# Arguments`, `# Returns`, `# Errors`, `# Examples` sections

#### Structs and Enums

**Requirement**: All public structs and enums must have doc comments explaining their purpose and usage. Each field should also be documented.

**Format for Structs**:
```rust
/// Represents a remittance transaction in the system.
///
/// A remittance is created when a sender transfers USDC to the contract
/// for payout by a registered agent. The remittance tracks the sender,
/// agent, amount, fee, and current status.
///
/// # Storage
///
/// Remittances are stored in persistent storage, indexed by their unique ID.
/// They remain in storage even after completion or cancellation for audit purposes.
#[contracttype]
pub struct Remittance {
    /// Unique identifier for this remittance
    pub id: u64,
    
    /// Address of the sender who created the remittance
    pub sender: Address,
    
    /// Address of the agent responsible for payout
    pub agent: Address,
    
    /// Total amount in USDC stroops (1 USDC = 10,000,000 stroops)
    pub amount: i128,
    
    /// Platform fee in USDC stroops (deducted on payout)
    pub fee: i128,
    
    /// Current status of the remittance
    pub status: RemittanceStatus,
}
```

**Format for Enums**:
```rust
/// Represents the lifecycle status of a remittance.
///
/// A remittance progresses through these states:
/// - Created as `Pending`
/// - Transitions to `Completed` when agent confirms payout
/// - Transitions to `Cancelled` if sender cancels before payout
///
/// State transitions are one-way - a completed or cancelled remittance
/// cannot return to pending state.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RemittanceStatus {
    /// Remittance created, awaiting payout confirmation
    Pending,
    
    /// Payout confirmed, funds transferred to agent
    Completed,
    
    /// Remittance cancelled, funds refunded to sender
    Cancelled,
}
```

**Key Points**:
- Document the overall purpose of the type
- Explain when and how it's used
- Document each field or variant
- Mention storage implications if relevant
- Explain state transitions for enums
- Note any invariants or constraints

#### Modules

**Requirement**: Each module (file) should have a module-level doc comment explaining its purpose and contents.

**Format**:
```rust
//! Storage management for the SwiftRemit contract.
//!
//! This module provides functions for reading and writing contract state
//! to Soroban storage. It uses both instance storage (for configuration)
//! and persistent storage (for user data).
//!
//! # Storage Types
//!
//! ## Instance Storage
//! - Admin address
//! - USDC token address
//! - Platform fee configuration (basis points)
//! - Remittance counter (auto-incrementing ID)
//! - Accumulated fees (total fees collected)
//!
//! ## Persistent Storage
//! - Remittances (indexed by ID)
//! - Agent registrations (indexed by address)
//!
//! # Usage
//!
//! ```ignore
//! use crate::storage::{get_remittance, set_remittance};
//!
//! let remittance = get_remittance(&env, remittance_id)?;
//! set_remittance(&env, remittance_id, &updated_remittance);
//! ```
```

**Key Points**:
- Use `//!` for module-level comments (not `///`)
- Explain the module's purpose and scope
- List major components or functions
- Provide usage examples if helpful
- Explain design decisions or patterns used

#### Inline Comments

**When to Use Inline Comments**:
- Complex algorithms or calculations
- Non-obvious business logic
- Workarounds for known issues or limitations
- Important security considerations
- Performance optimizations that sacrifice clarity

**When NOT to Use Inline Comments**:
- Obvious code (let the code speak for itself)
- Redundant explanations that repeat what the code does
- Commented-out code (delete it instead - use version control)
- Apologies or TODOs without context

**Good Examples**:
```rust
// Calculate fee using basis points (1 bps = 0.01%)
// Example: 10,000 USDC * 250 bps / 10,000 = 250 USDC (2.5%)
let fee = amount
    .checked_mul(fee_bps as i128)
    .ok_or(ContractError::Overflow)?
    .checked_div(10000)
    .ok_or(ContractError::Overflow)?;

// Verify agent is registered before allowing remittance creation.
// This prevents funds from being locked with an invalid agent.
if !is_agent_registered(&env, &agent) {
    return Err(ContractError::AgentNotRegistered);
}

// Use checked arithmetic to prevent overflow with large amounts.
// Overflow could lead to incorrect fee calculations or loss of funds.
let total = amount.checked_add(fee)
    .ok_or(ContractError::Overflow)?;
```

**Bad Examples**:
```rust
// Increment counter
counter += 1;  // Don't state the obvious

// Get the remittance
let remittance = get_remittance(&env, id)?;  // Redundant

// TODO: Fix this later
// let result = broken_function();  // No context, commented code

// This is a hack but it works
let value = unsafe { ... };  // No explanation of why
```

---

### Project Documentation Requirements

In addition to code documentation, certain project-level documentation must be updated when making changes.

#### README.md Updates

**Requirement**: Update README.md when adding new features or changing existing functionality.

**When to Update**:
- Adding new public functions to the contract
- Changing function signatures or behavior
- Adding new features or capabilities
- Changing deployment procedures
- Updating dependencies or requirements

**What to Update**:
- Feature list - Add new capabilities
- Usage examples - Show how to use new functions
- API reference - Update function signatures
- Installation instructions - If dependencies change
- Quick start guide - If basic usage changes

**Example**:
```markdown
## Features

- ✅ Escrow-based remittance creation
- ✅ Agent registration and management
- ✅ Secure payout confirmation
- ✅ Remittance cancellation with refunds
- ✅ Platform fee collection
- ✅ **NEW: Batch remittance creation** ← Add new features

## Usage

### Creating a Remittance

\`\`\`rust
let remittance_id = contract.create_remittance(
    &env,
    &sender,
    &agent,
    &amount,
)?;
\`\`\`

### Creating Multiple Remittances (Batch)  ← Add examples for new features

\`\`\`rust
let remittance_ids = contract.create_remittances_batch(
    &env,
    &sender,
    &requests,
)?;
\`\`\`
```

#### API Documentation

**Requirement**: Maintain accurate API documentation for all public contract functions.

If your project has an API.md or similar file:
- Update function signatures
- Update parameter descriptions
- Update return values
- Update error conditions
- Add examples for new functions

#### CHANGELOG.md

**Requirement**: Document all changes in CHANGELOG.md following [Keep a Changelog](https://keepachangelog.com/) format.

**Format**:
```markdown
## [Unreleased]

### Added
- Batch remittance creation function for creating multiple remittances in one transaction
- New `BatchRemittanceRequest` type for batch operations

### Changed
- Improved error messages for validation failures
- Optimized storage access in `confirm_payout` function

### Fixed
- Fixed overflow in fee calculation for large amounts
- Fixed authorization check in `cancel_remittance`

### Security
- Added overflow protection to all arithmetic operations
- Enhanced input validation for agent addresses
```

**Categories**:
- **Added** - New features
- **Changed** - Changes to existing functionality
- **Deprecated** - Features that will be removed
- **Removed** - Removed features
- **Fixed** - Bug fixes
- **Security** - Security improvements or fixes

---

### Documentation for Complex Functions

**Requirement**: Functions with complex logic, multiple edge cases, or non-obvious behavior must include comprehensive examples.

#### What Makes a Function Complex?

A function is considered complex if it:
- Has multiple parameters with interdependencies
- Has multiple possible error conditions
- Performs calculations with specific units or formats
- Has non-obvious side effects
- Requires specific setup or state
- Has important edge cases or limitations

#### Example Documentation for Complex Function

```rust
/// Calculates the platform fee for a remittance amount.
///
/// The fee is calculated using basis points (bps), where 1 bps = 0.01%.
/// For example, a fee of 250 bps represents 2.5%.
///
/// # Arguments
///
/// * `amount` - The remittance amount in USDC stroops
/// * `fee_bps` - The fee percentage in basis points (0-10000)
///
/// # Returns
///
/// The calculated fee in USDC stroops
///
/// # Errors
///
/// * `Overflow` - If the multiplication or division causes arithmetic overflow
/// * `InvalidFeeBps` - If fee_bps is negative or greater than 10000
///
/// # Examples
///
/// ```ignore
/// // Calculate 2.5% fee on 100 USDC
/// let amount = 100_000_000;  // 100 USDC in stroops
/// let fee_bps = 250;         // 2.5% in basis points
/// let fee = calculate_fee(amount, fee_bps)?;
/// assert_eq!(fee, 2_500_000);  // 2.5 USDC in stroops
///
/// // Calculate 0.1% fee on 1000 USDC
/// let amount = 1_000_000_000;  // 1000 USDC
/// let fee_bps = 10;            // 0.1%
/// let fee = calculate_fee(amount, fee_bps)?;
/// assert_eq!(fee, 1_000_000);  // 1 USDC
///
/// // Edge case: Zero fee
/// let fee = calculate_fee(100_000_000, 0)?;
/// assert_eq!(fee, 0);
///
/// // Edge case: Maximum fee (100%)
/// let fee = calculate_fee(100_000_000, 10000)?;
/// assert_eq!(fee, 100_000_000);
/// ```
///
/// # Notes
///
/// - The calculation uses checked arithmetic to prevent overflow
/// - Rounding is performed using integer division (rounds down)
/// - For amounts less than 10000 stroops, fees may round to zero
pub fn calculate_fee(amount: i128, fee_bps: i128) -> Result<i128, ContractError> {
    // Validate fee_bps range
    if fee_bps < 0 || fee_bps > 10000 {
        return Err(ContractError::InvalidFeeBps);
    }
    
    // Calculate: (amount * fee_bps) / 10000
    amount
        .checked_mul(fee_bps)
        .ok_or(ContractError::Overflow)?
        .checked_div(10000)
        .ok_or(ContractError::Overflow)
}
```

**Key Elements**:
- Multiple examples showing different use cases
- Edge case examples (zero fee, maximum fee)
- Clear explanation of units and formats
- Notes section for important details
- Validation of inputs documented

---

### Documentation Quality Standards

#### Clarity

- **Use simple language** - Avoid jargon when possible
- **Be specific** - "Returns the remittance ID" not "Returns a number"
- **Define terms** - Explain stroops, basis points, etc.
- **Use examples** - Show, don't just tell

#### Accuracy

- **Keep docs in sync with code** - Update docs when code changes
- **Test examples** - Ensure example code actually works
- **Document actual behavior** - Not what you wish it did
- **Update error conditions** - When adding new error cases

#### Completeness

- **Document all parameters** - Don't skip any
- **Document all errors** - Include all possible error conditions
- **Document side effects** - State changes, events emitted, etc.
- **Document assumptions** - Prerequisites, required state, etc.

#### Consistency

- **Follow the format** - Use the standard sections (Arguments, Returns, Errors, Examples)
- **Use consistent terminology** - "remittance" not "transaction" or "transfer"
- **Match existing style** - Follow patterns in existing documentation
- **Use consistent units** - Always specify stroops, bps, etc.

---

### Generating and Viewing Documentation

Rust provides built-in tools for generating HTML documentation from doc comments.

#### Generate Documentation

```bash
# Generate documentation for the contract
cargo doc --no-deps

# Generate and open in browser
cargo doc --no-deps --open

# Generate with private items (for internal development)
cargo doc --no-deps --document-private-items
```

#### Review Your Documentation

Before submitting a PR:
1. Generate the documentation: `cargo doc --no-deps --open`
2. Navigate to your new or modified functions
3. Verify all sections are present and correctly formatted
4. Check that examples render correctly
5. Ensure links work (if you added any)

#### Documentation Tests

Rust can test code examples in documentation:

```rust
/// Calculates the sum of two numbers.
///
/// # Examples
///
/// ```
/// use swiftremit::math::add;
///
/// assert_eq!(add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run documentation tests:
```bash
cargo test --doc
```

**Note**: For Soroban contracts, use ````ignore` for examples that require the contract environment, as they can't be run as doc tests.

---

### Documentation Checklist

Before submitting your PR, verify:

**Code Documentation**:
- [ ] All new public functions have doc comments
- [ ] All parameters are documented in `# Arguments` section
- [ ] Return values are documented in `# Returns` section
- [ ] All error conditions are documented in `# Errors` section
- [ ] Complex functions include examples in `# Examples` section
- [ ] New structs/enums have doc comments
- [ ] All struct fields and enum variants are documented
- [ ] Module-level doc comments added for new files

**Project Documentation**:
- [ ] README.md updated for new features
- [ ] API documentation updated (if applicable)
- [ ] CHANGELOG.md updated with changes
- [ ] Examples added for new functionality
- [ ] Deployment docs updated (if applicable)

**Quality**:
- [ ] Documentation is clear and easy to understand
- [ ] Examples are accurate and tested
- [ ] Terminology is consistent with existing docs
- [ ] Units are specified (stroops, bps, etc.)
- [ ] Generated docs reviewed: `cargo doc --no-deps --open`

---

### Common Documentation Mistakes

#### Mistake 1: Missing Error Documentation

**Bad**:
```rust
/// Creates a remittance.
pub fn create_remittance(...) -> Result<u64, ContractError> {
```

**Good**:
```rust
/// Creates a remittance.
///
/// # Errors
///
/// * `InvalidAmount` - If amount is zero or negative
/// * `AgentNotRegistered` - If agent is not registered
/// * `NotInitialized` - If contract is not initialized
pub fn create_remittance(...) -> Result<u64, ContractError> {
```

#### Mistake 2: Vague Parameter Descriptions

**Bad**:
```rust
/// # Arguments
///
/// * `amount` - The amount
```

**Good**:
```rust
/// # Arguments
///
/// * `amount` - The remittance amount in USDC stroops (1 USDC = 10,000,000 stroops)
```

#### Mistake 3: No Examples for Complex Functions

**Bad**:
```rust
/// Calculates fee using basis points.
pub fn calculate_fee(amount: i128, fee_bps: i128) -> Result<i128, ContractError> {
```

**Good**:
```rust
/// Calculates fee using basis points.
///
/// # Examples
///
/// ```ignore
/// // 2.5% fee on 100 USDC
/// let fee = calculate_fee(100_000_000, 250)?;
/// assert_eq!(fee, 2_500_000);
/// ```
pub fn calculate_fee(amount: i128, fee_bps: i128) -> Result<i128, ContractError> {
```

#### Mistake 4: Outdated Documentation

**Problem**: Code changes but documentation doesn't.

**Solution**:
- Update docs in the same commit as code changes
- Review docs during code review
- Run `cargo doc` to catch broken links
- Test examples to ensure they still work

#### Mistake 5: Using `//` Instead of `///`

**Bad**:
```rust
// Creates a remittance
pub fn create_remittance(...) {
```

**Good**:
```rust
/// Creates a remittance
pub fn create_remittance(...) {
```

**Note**: `//` is for inline comments, `///` is for doc comments that appear in generated documentation.

---

### Documentation Review Process

During code review, reviewers will check:

**Completeness**:
- All public items are documented
- All required sections are present
- All parameters and errors are documented

**Quality**:
- Documentation is clear and accurate
- Examples are helpful and correct
- Terminology is consistent
- Units are specified

**Maintenance**:
- Existing docs are updated if behavior changed
- README and other project docs are updated
- CHANGELOG is updated

**Reviewers may request changes if**:
- Documentation is missing or incomplete
- Examples are unclear or incorrect
- Terminology is inconsistent
- Project documentation is not updated

---

### Getting Help with Documentation

If you're unsure about documentation:

- **Look at existing code** - Follow patterns in the codebase
- **Check Rust documentation guidelines** - [Rust Doc Book](https://doc.rust-lang.org/rustdoc/)
- **Ask in your PR** - Mention you'd like feedback on documentation
- **Ask in discussions** - Get help before submitting
- **Review generated docs** - Use `cargo doc --no-deps --open` to see how it looks

**Remember**: Good documentation is as important as good code. Take the time to document your work thoroughly, and future contributors (including yourself) will thank you!

---


## Community and Communication

SwiftRemit is built by a community of contributors, and we value open communication, collaboration, and mutual respect. This section explains how to communicate with maintainers and other contributors, get help, and participate in the community.

---

### Communication Channels

We use multiple channels for different types of communication:

#### GitHub Issues

**Purpose**: Bug reports, feature requests, and technical discussions

**When to use**:
- Reporting bugs or defects
- Proposing new features or enhancements
- Discussing technical implementation details
- Tracking work and progress

**How to use**:
- [View existing issues](https://github.com/yourusername/swiftremit/issues)
- [Create a new issue](https://github.com/yourusername/swiftremit/issues/new)
- Use appropriate labels (bug, enhancement, documentation, etc.)
- Provide clear descriptions and reproduction steps
- Search existing issues before creating new ones

**Best practices**:
- Use descriptive titles
- Provide context and details
- Include error messages and logs
- Be respectful and constructive
- Follow up on your issues

#### GitHub Discussions

**Purpose**: Questions, ideas, and general discussions

**When to use**:
- Asking questions about usage or implementation
- Brainstorming new ideas
- Discussing design decisions
- Sharing knowledge and experiences
- General community chat

**How to use**:
- [Join the conversation](https://github.com/yourusername/swiftremit/discussions)
- Browse existing discussions
- Start a new discussion with a clear topic
- Engage respectfully with other participants

**Discussion categories**:
- **Q&A** - Ask and answer questions
- **Ideas** - Propose and discuss new ideas
- **Show and Tell** - Share what you've built
- **General** - Everything else

#### Stellar Discord

**Purpose**: Real-time chat and community support

**When to use**:
- Quick questions that need fast answers
- Real-time collaboration
- Getting to know the community
- Casual conversation about Soroban development

**How to join**:
- [Join Stellar Discord](https://discord.gg/stellar)
- Find us in the **#soroban** channel
- Introduce yourself!

**Discord etiquette**:
- Be patient - responses may not be immediate
- Don't spam or cross-post excessively
- Use threads for longer discussions
- Be respectful of everyone's time

#### Email (For Security Issues Only)

**Purpose**: Private reporting of security vulnerabilities

**When to use**:
- Discovered a security vulnerability
- Found a critical bug that could be exploited
- Need to discuss security concerns privately

**How to report**:
- Use GitHub Security Advisories (preferred)
- Or contact project maintainers directly via GitHub
- Include detailed description of the vulnerability
- Provide steps to reproduce
- Do NOT disclose publicly until fixed

---

### Response Times

We're a community-driven project with volunteer maintainers. Here are our target response times:

#### GitHub Issues
- **Initial response**: 2-3 business days
- **Bug reports**: Triaged within 1 week
- **Feature requests**: Discussed within 1 week
- **Critical bugs**: Prioritized and addressed ASAP

#### Pull Requests
- **Initial review**: 2-3 business days
- **Follow-up reviews**: 1-2 business days after updates
- **Merge time**: Varies based on complexity and review feedback

#### GitHub Discussions
- **Questions**: Usually answered within 1-2 days by community or maintainers
- **Ideas**: Discussed as community interest develops

#### Discord
- **Response time**: Varies - community members often respond quickly
- **Maintainer availability**: Not guaranteed - use GitHub for important matters

**Note**: These are targets, not guarantees. Response times may be longer during holidays, busy periods, or for complex issues. We appreciate your patience!

---

### How to Ask Questions Effectively

Good questions get better answers faster. Here's how to ask effective questions:

#### 1. Search First

Before asking:
- Search existing GitHub issues and discussions
- Check the README and documentation
- Review the code and comments
- Search Discord history

You might find your answer immediately!

#### 2. Provide Context

When asking, include:
- **What you're trying to do** - Your goal or use case
- **What you've tried** - Steps you've already taken
- **What happened** - Actual behavior or error
- **What you expected** - Expected behavior
- **Your environment** - Rust version, OS, Soroban CLI version

#### 3. Include Details

Provide:
- **Error messages** - Full error text, not just "it doesn't work"
- **Code snippets** - Relevant code (use code blocks)
- **Logs** - Output from commands or tests
- **Screenshots** - If visual issues are involved

#### 4. Be Specific

- **Good**: "Getting `ContractError::AgentNotRegistered` when calling `create_remittance` with agent address GABC..., but agent is registered"
- **Bad**: "create_remittance doesn't work"

#### 5. Show Your Work

Demonstrate that you've tried to solve the problem:
- "I tried X but got error Y"
- "I read the documentation on Z but I'm still confused about..."
- "I looked at the code in file.rs but I don't understand why..."

#### Example of a Good Question

```markdown
## Question: How to handle remittance cancellation after partial payout?

### Context
I'm building a feature where agents can do partial payouts, and I need to 
understand how cancellation should work if only part of the remittance has 
been paid out.

### What I've Tried
- Reviewed the `cancel_remittance` function in lib.rs
- Looked at the RemittanceStatus enum
- Searched issues for "partial payout" and "cancellation"

### Specific Questions
1. Should cancellation be allowed after partial payout?
2. If yes, should the refund be the remaining amount or full amount?
3. Should we add a new status like `PartiallyCompleted`?

### Environment
- Rust 1.75.0
- Soroban CLI 20.0.0
- Testing on Stellar testnet
```

---

### Code of Conduct

We are committed to providing a welcoming, inclusive, and harassment-free environment for everyone.

#### Our Standards

**Positive behaviors we encourage**:
- **Be respectful** - Treat everyone with respect and kindness
- **Be inclusive** - Welcome newcomers and diverse perspectives
- **Be constructive** - Provide helpful feedback and suggestions
- **Be patient** - Remember that everyone is learning
- **Assume good intentions** - Give others the benefit of the doubt
- **Be collaborative** - Work together toward common goals
- **Be professional** - Keep discussions focused and productive

**Unacceptable behaviors**:
- Harassment, discrimination, or offensive comments
- Personal attacks, insults, or derogatory remarks
- Trolling or deliberately disruptive behavior
- Publishing others' private information without permission
- Unwelcome sexual attention or advances
- Other conduct that would be inappropriate in a professional setting

#### Scope

This Code of Conduct applies to:
- GitHub repositories (issues, PRs, discussions)
- Discord channels
- Email communications
- Any other SwiftRemit community spaces

#### Enforcement

**If you experience or witness unacceptable behavior**:
1. **Report it** - Contact project maintainers privately
2. **Provide details** - Describe what happened, when, and where
3. **Include evidence** - Screenshots, links, or other documentation

**How we handle reports**:
- All reports are taken seriously and reviewed promptly
- Reports are handled confidentially
- We will investigate and take appropriate action
- Actions may include warnings, temporary bans, or permanent bans
- Reporters will be updated on the outcome

**Contact for reporting**:
- Contact any project maintainer privately via GitHub
- Or email the project maintainers (contact information available in the repository)

#### Attribution

This Code of Conduct is adapted from the [Contributor Covenant](https://www.contributor-covenant.org/), version 2.1.

---

### Getting Help

#### For Technical Questions

1. **Check documentation first**
   - README.md
   - This CONTRIBUTING.md
   - Code comments and doc comments
   - Generated docs: `cargo doc --no-deps --open`

2. **Search existing resources**
   - GitHub issues
   - GitHub discussions
   - Discord history

3. **Ask in the right place**
   - **Quick questions**: Discord #soroban channel
   - **Detailed questions**: GitHub Discussions
   - **Bug reports**: GitHub Issues

4. **Provide context and details**
   - See "How to Ask Questions Effectively" above

#### For Contribution Questions

1. **Review this guide**
   - Read relevant sections thoroughly
   - Check examples and patterns

2. **Look at existing code**
   - Find similar functionality
   - Follow established patterns

3. **Ask before major work**
   - Open an issue to discuss
   - Get feedback on your approach
   - Avoid wasted effort

4. **Ask in your PR**
   - Request specific feedback
   - Ask questions in PR comments
   - Engage with reviewers

#### For Security Issues

1. **Do NOT disclose publicly**
   - Don't open a public issue
   - Don't discuss in Discord

2. **Report privately**
   - Use GitHub Security Advisories or contact maintainers directly
   - Provide detailed information
   - Include reproduction steps

3. **Wait for response**
   - We'll respond within 24-48 hours
   - We'll work with you on a fix
   - We'll coordinate disclosure

---

### Decision-Making Process

Understanding how decisions are made helps you participate effectively.

#### Types of Decisions

**Minor decisions** (made quickly by maintainers):
- Bug fixes
- Documentation improvements
- Code style and formatting
- Test additions
- Minor refactoring

**Major decisions** (require discussion):
- New features
- Breaking changes
- Architecture changes
- Security-related changes
- Changes affecting gas costs significantly

#### Decision Process for Major Changes

1. **Proposal**
   - Open a GitHub issue
   - Describe the problem and proposed solution
   - Explain benefits and trade-offs

2. **Discussion**
   - Community provides feedback
   - Maintainers ask questions
   - Alternative approaches are considered

3. **Consensus Building**
   - Address concerns and objections
   - Refine the proposal based on feedback
   - Seek agreement from maintainers

4. **Decision**
   - Maintainers make final decision
   - Decision is documented in the issue
   - Implementation can proceed

5. **Implementation**
   - PR is submitted
   - Code review process
   - Merge when approved

#### Who Makes Decisions?

- **Project maintainers** - Have final say on major decisions
- **Core contributors** - Significant input on technical decisions
- **Community** - Feedback and input on all decisions
- **You** - Your voice matters! Participate in discussions

#### Disagreements

If you disagree with a decision:
- **Express your concerns respectfully** - Explain your reasoning
- **Provide alternatives** - Suggest different approaches
- **Accept the outcome** - Maintainers have final say
- **Move forward constructively** - Work with the decision

---

### Recognition and Acknowledgment

We value and recognize all contributions to SwiftRemit.

#### How We Recognize Contributors

**In the repository**:
- Contributors list in README.md
- Acknowledgment in release notes
- Credit in CHANGELOG.md

**For significant contributions**:
- Highlighted in release announcements
- Mentioned in project updates
- Potential invitation to become a core contributor

**For first-time contributors**:
- Welcome message when first PR is merged
- Added to contributors list
- Celebrated in the community

#### Types of Contributions We Recognize

Not just code! We value:
- Code contributions (features, fixes, refactoring)
- Documentation improvements
- Test additions
- Bug reports and triage
- Code reviews
- Helping others in discussions
- Community building
- Design and UX feedback

**Every contribution matters**, no matter how small!

---

### Staying Updated

Stay informed about SwiftRemit development:

#### Watch the Repository

- Click "Watch" on GitHub
- Choose notification preferences
- Get notified of issues, PRs, and releases

#### Follow Releases

- Check the [Releases page](https://github.com/yourusername/swiftremit/releases)
- Read release notes for changes
- Update your local copy regularly

#### Participate in Discussions

- Join GitHub Discussions
- Engage in Discord
- Provide feedback on proposals

#### Subscribe to Announcements

- Watch for pinned issues
- Check for announcement discussions
- Follow project maintainers

---

### Building Community

We're not just building software - we're building a community.

#### Ways to Contribute to Community

**Help others**:
- Answer questions in discussions
- Review pull requests
- Share your knowledge and experience

**Share your work**:
- Write blog posts about using SwiftRemit
- Create tutorials or videos
- Share your projects built with SwiftRemit

**Improve the project**:
- Suggest improvements
- Report bugs
- Contribute code and documentation

**Welcome newcomers**:
- Be friendly and helpful
- Share resources and tips
- Encourage first-time contributors

#### Community Values

We strive to be:
- **Welcoming** - Everyone is welcome to contribute
- **Inclusive** - We value diverse perspectives
- **Supportive** - We help each other succeed
- **Collaborative** - We work together toward common goals
- **Respectful** - We treat everyone with respect and kindness

---

### Thank You!

Thank you for being part of the SwiftRemit community! Whether you're contributing code, documentation, ideas, or just learning, you're helping make this project better.

**Questions?** Don't hesitate to ask in [GitHub Discussions](https://github.com/yourusername/swiftremit/discussions) or [Discord](https://discord.gg/stellar).

**Ready to contribute?** Check out [good first issues](https://github.com/yourusername/swiftremit/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) to get started!

**Happy contributing!** 🚀

---
