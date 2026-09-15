# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| Testnet (latest) | ✅ |
| Mainnet | Not yet deployed |

## Reporting a Vulnerability

**Please do NOT report security vulnerabilities in public GitHub Issues.**

If you discover a security vulnerability in PRISM:

1. **Email:** Open a private [GitHub Security Advisory](https://github.com/Glorious21/prism/security/advisories/new)
2. **Include:**
   - A description of the vulnerability
   - Steps to reproduce it
   - The potential impact
   - Any suggested fixes (optional)

We will acknowledge your report within **48 hours** and aim to release a fix within **7 days** for critical issues.

## Scope

The following are in scope for security reports:

- Smart contract logic vulnerabilities (reentrancy, integer overflow, unauthorized access)
- Frontend vulnerabilities that could expose private keys or secret keys
- Authentication/authorization bypass in wallet connection flows

## Out of Scope

- Issues in third-party dependencies (report to their maintainers)
- Social engineering attacks
- Denial of service against the Stellar network itself
