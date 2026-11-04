# Security policy

## Reporting a vulnerability

Please report vulnerabilities privately through GitHub: **Security → Report a vulnerability** on this repository. Don't open a public issue.

Include what you found, how to reproduce it, and what an attacker could do with it. You'll get a reply within 3 working days.

## What runs automatically

Every pull request must pass these checks before it can merge into `main`. They also run on every push to `main`, and the dependency, malware and CodeQL scans run again every Monday.

| Check | What it catches |
| --- | --- |
| CI: Format, Clippy, Build and test | Broken builds, failing tests, lint errors |
| Dependency audit (cargo-deny) | RustSec vulnerabilities, unsound or malicious crates, yanked versions, crates from anywhere but crates.io, wildcard versions, unapproved licenses |
| OSV scan | Known vulnerable and malicious packages (OSV `MAL-*` database) |
| Malware scan (ClamAV) | Malware in any file in the repository |
| Secret scan (gitleaks) | API keys, tokens and private keys committed by mistake |
| Dependency review | A pull request adding a vulnerable, malicious or badly licensed dependency |
| CodeQL | Security bugs in the Rust code and the GitHub workflows |

Dependabot opens weekly update PRs for crates and GitHub Actions. Minor and patch updates merge on their own once every check above passes. Major updates wait for review.

GitHub secret scanning with push protection is also on, so a push containing a known secret format is blocked.

## Known accepted advisory

`RUSTSEC-2023-0071` (rsa, Marvin attack) is ignored in `osv-scanner.toml`, `.cargo/audit.toml` and dependency review, which read `Cargo.lock`. `rsa` appears there only because sqlx records its MySQL driver; RampWatch uses SQLite and never compiles it (`cargo tree -i rsa` is empty, and cargo-deny, which reads the real build graph, doesn't see it).
