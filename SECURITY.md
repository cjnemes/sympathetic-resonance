# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The Sympathetic Resonance team takes security vulnerabilities seriously. We appreciate your efforts to responsibly disclose your findings.

**Please do not report security vulnerabilities through public GitHub issues.**

### How to Report

Send an email to: [security@sympathetic-resonance.com] (replace with actual email)

Include the following information:
- Type of issue (e.g. buffer overflow, SQL injection, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

- **Acknowledgment**: We'll acknowledge receipt of your vulnerability report within 48 hours
- **Initial Assessment**: We'll provide an initial assessment within 5 business days
- **Updates**: We'll keep you informed of our progress throughout the process
- **Resolution**: We aim to resolve critical vulnerabilities within 30 days

### Security Considerations

#### Save File Security
- Save files are stored locally and may contain sensitive game state
- Save files use JSON serialization with validation
- No network transmission of save data in current version

#### Database Security
- Uses SQLite with parameterized queries to prevent injection
- Database files stored locally with standard file permissions
- No user-provided SQL execution

#### Input Validation
- All user input is validated and sanitized
- Command parsing includes input length limits
- No execution of user-provided code

#### Dependencies
- Regular security audits using `cargo audit`
- Dependencies updated regularly for security patches
- Minimal dependency footprint to reduce attack surface

### Scope

This security policy applies to:
- The core Sympathetic Resonance application
- Official plugins and extensions
- Documentation and configuration examples

Out of scope:
- Third-party modifications or forks
- Local system security (file permissions, etc.)
- Issues in dependencies (report to upstream projects)

### Safe Harbor

We support safe harbor for security researchers who:
- Make a good faith effort to avoid privacy violations and service disruption
- Only interact with accounts you own or with explicit permission
- Do not access or modify data belonging to others
- Contact us before sharing any information about the vulnerability

Thank you for helping keep Sympathetic Resonance and our users safe!