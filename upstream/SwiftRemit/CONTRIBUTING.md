# Contributing to SwiftRemit

Thank you for your interest in contributing to SwiftRemit! This guide will help you get started with contributing to our Soroban smart contract project.

## Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Branch Naming Convention](#branch-naming-convention)
- [Commit Style](#commit-style)
- [Making Changes](#making-changes)
- [Running Tests](#running-tests)
- [Code Quality](#code-quality)
- [Submitting a Pull Request](#submitting-a-pull-request)
- [Code Review Process](#code-review-process)

## Getting Started

### Prerequisites

Before you begin, ensure you have the following installed:

- **Rust** (latest stable version)
- **Cargo** (comes with Rust)
- **Soroban CLI** (for contract deployment and testing)
- **Git** (for version control)

### Development Setup

1. **Fork the repository** on GitHub

2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/swiftremit.git
   cd swiftremit
   ```

3. **Add the upstream repository**:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/swiftremit.git
   ```

4. **Install dependencies**:
   ```bash
   make install
   ```
   
   This will:
   - Add the `wasm32-unknown-unknown` target
   - Install Soroban CLI

5. **Verify your setup**:
   ```bash
   make build
   make test
   ```

If all tests pass, you're ready to contribute!

## Branch Naming Convention

Use descriptive branch names that follow this pattern:

```
<type>/<short-description>
```

### Branch Types

- `feature/` - New features or enhancements
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring without changing functionality
- `test/` - Adding or updating tests
- `chore/` - Maintenance tasks, dependency updates

### Examples

```bash
feature/multi-currency-support
fix/overflow-in-fee-calculation
docs/update-api-examples
refactor/storage-optimization
test/add-cancellation-tests
chore/update-soroban-sdk
```

### Creating a Branch

```bash
git checkout -b feature/your-feature-name
```

## Commit Style

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification for clear and consistent commit messages.

### Commit Message Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Commit Types

- `feat` - A new feature
- `fix` - A bug fix
- `docs` - Documentation changes
- `style` - Code style changes (formatting, missing semicolons, etc.)
- `refactor` - Code refactoring
- `test` - Adding or updating tests
- `chore` - Maintenance tasks

### Scope (Optional)

The scope specifies what part of the codebase is affected:

- `contract` - Core contract logic
- `storage` - Storage-related changes
- `events` - Event emission
- `validation` - Input validation
- `tests` - Test suite
- `deps` - Dependencies

### Examples

```bash
feat(contract): add multi-currency support for remittances

fix(validation): prevent overflow in fee calculation for large amounts

docs(readme): update deployment instructions for mainnet

test(contract): add edge case tests for cancellation flow

refactor(storage): optimize remittance data structure

chore(deps): update soroban-sdk to 21.7.0
```

### Commit Message Guidelines

- Use the imperative mood ("add" not "added" or "adds")
- Keep the subject line under 72 characters
- Capitalize the subject line
- Don't end the subject line with a period
- Separate subject from body with a blank line
- Wrap the body at 72 characters
- Use the body to explain what and why, not how

### Example Commit

```
feat(contract): add batch remittance processing

Implement batch processing to allow multiple remittances
in a single transaction, reducing gas costs for high-volume
users. The batch size is limited to 10 remittances per call
to prevent transaction timeouts.

Closes #42
```

## Making Changes

### Before You Start

1. **Sync with upstream**:
   ```bash
   git fetch upstream
   git checkout main
   git merge upstream/main
   ```

2. **Create a new branch** (see [Branch Naming Convention](#branch-naming-convention))

### Development Workflow

1. **Make your changes** in the appropriate files:
   - `src/lib.rs` - Main contract logic
   - `src/types.rs` - Data structures
   - `src/storage.rs` - Storage operations
   - `src/errors.rs` - Error definitions
   - `src/events.rs` - Event emissions
   - `src/validation.rs` - Input validation
   - `src/test.rs` - Test cases

2. **Format your code**:
   ```bash
   make fmt
   ```

3. **Check for issues**:
   ```bash
   make check
   make lint
   ```

4. **Build the contract**:
   ```bash
   make build
   ```

5. **Run tests** (see [Running Tests](#running-tests))

## Running Tests

### Run All Tests

```bash
make test
```

### Run Tests with Verbose Output

```bash
make test-verbose
```

### Run Specific Tests

```bash
cargo test test_name
```

For example:
```bash
cargo test test_create_remittance
cargo test test_confirm_payout
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Writing Tests

When adding new features, always include tests:

1. **Unit tests** - Test individual functions
2. **Integration tests** - Test complete workflows
3. **Edge cases** - Test boundary conditions and error cases

Example test structure:

```rust
#[test]
fn test_your_feature() {
    let env = Env::default();
    env.mock_all_auths();
    
    // Setup
    let contract_id = env.register_contract(None, SwiftRemitContract);
    let client = SwiftRemitContractClient::new(&env, &contract_id);
    
    // Execute
    // ... your test logic
    
    // Assert
    assert_eq!(expected, actual);
}
```

## Code Quality

### Code Standards

- Follow Rust best practices and idioms
- Use meaningful variable and function names
- Add comments for complex logic
- Keep functions focused and concise
- Handle all error cases explicitly
- Avoid unwrap() in production code

### Linting

Run the linter before committing:

```bash
make lint
```

Fix any warnings or errors reported by Clippy.

### Formatting

Ensure code is properly formatted:

```bash
make fmt
```

### Security

- Never expose private keys or sensitive data
- Validate all inputs
- Use safe math operations to prevent overflows
- Follow the principle of least privilege
- Test authorization checks thoroughly

## Submitting a Pull Request

### Before Submitting

1. **Ensure all tests pass**:
   ```bash
   make test
   ```

2. **Lint your code**:
   ```bash
   make lint
   ```

3. **Format your code**:
   ```bash
   make fmt
   ```

4. **Update documentation** if needed:
   - Update README.md for user-facing changes
   - Update API.md for contract interface changes
   - Add inline code comments for complex logic

5. **Commit your changes** following the [Commit Style](#commit-style)

6. **Push to your fork**:
   ```bash
   git push origin your-branch-name
   ```

### Creating the Pull Request

1. Go to the original repository on GitHub
2. Click "New Pull Request"
3. Select your fork and branch
4. Fill out the PR template with:
   - **Title**: Brief description following commit style
   - **Description**: What changes you made and why
   - **Related Issues**: Link any related issues (e.g., "Closes #42")
   - **Testing**: Describe how you tested your changes
   - **Screenshots**: If applicable (for UI changes)

### PR Title Format

Follow the same format as commit messages:

```
feat(contract): add batch remittance processing
fix(validation): prevent overflow in fee calculation
docs(contributing): add branch naming guidelines
```

### PR Description Template

```markdown
## Description
Brief description of what this PR does.

## Motivation
Why is this change needed?

## Changes
- List of changes made
- Another change

## Testing
How did you test these changes?

## Related Issues
Closes #42
```

## Code Review Process

### What to Expect

1. **Automated checks** will run on your PR:
   - Build verification
   - Test suite execution
   - Linting checks

2. **Maintainer review**:
   - Code quality assessment
   - Security review
   - Documentation review
   - Test coverage check

3. **Feedback and iteration**:
   - Address reviewer comments
   - Make requested changes
   - Push updates to your branch

### Responding to Feedback

- Be open to suggestions and constructive criticism
- Ask questions if feedback is unclear
- Make requested changes promptly
- Push updates to the same branch (they'll appear in the PR automatically)

### After Approval

Once approved, a maintainer will merge your PR. Your contribution will be part of the next release!

## Additional Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Stellar Discord](https://discord.gg/stellar)
- [Project README](README.md)
- [API Documentation](API.md)
- [Deployment Guide](DEPLOYMENT.md)

## Getting Help

If you need help:

1. Check existing documentation
2. Search for similar issues on GitHub
3. Ask in the project's GitHub Discussions
4. Join the Stellar Discord community

## Code of Conduct

- Be respectful and inclusive
- Welcome newcomers
- Focus on constructive feedback
- Assume good intentions

Thank you for contributing to SwiftRemit! Your efforts help make cross-border remittances more accessible and affordable.
