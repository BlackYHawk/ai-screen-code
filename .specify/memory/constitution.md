<!--
Sync Impact Report
==================
Version Change: 0.0.0 → 1.0.0 (NEW CONSTITUTION)
-------------------------------------------
Modified Principles:
  - (none, new constitution)

Added Sections:
  - I. Test Verification Gate (NON-NEGOTIABLE) - Core principle requiring automated test verification for feature completion
  - II. TDD Approach (REQUIRED) - Red-Green-Refactor cycle enforcement
  - III. Modularity & Library-First - Component architecture standards
  - IV. Code Quality Standards - Specific rules (function size, nesting, etc.)
  - V. Security & Secrets - Security checklist requirements
  - Additional Constraints - Technology stack + Quality gates
  - Development Workflow - Feature implementation process
  - Governance - Amendment procedure and versioning policy

Removed Sections:
  - (none, new constitution)

Templates Status:
  ✅ plan-template.md - No changes needed (already references "Constitution Check")
  ✅ spec-template.md - No changes needed (already has User Scenarios & Testing section)
  ✅ tasks-template.md - No changes needed (already has test phases)
  ⚠️ Command files - No command files found in .specify/templates/commands/

Follow-up TODOs:
  - None

Deferred Items:
  - None
-->
# AI Screen Code Constitution

## Core Principles

### I. Test Verification Gate (NON-NEGOTIABLE)

Every feature MUST pass automated test verification before being considered complete.

**Rationale**: Without automated verification, features cannot be reliably validated or safely deployed. This ensures all delivered functionality meets its specification and prevents regressions.

**Rules**:
- Implementation is NOT complete until all tests pass
- Each user story MUST have corresponding automated tests (unit, integration, or E2E as appropriate)
- Test coverage MUST meet the project minimum threshold (80%)
- CI/CD pipeline MUST block deployment on test failures
- Manual testing alone does NOT constitute completion

**Enforcement**: PRs without passing tests will not be merged. Deployment pipelines require green test status.

---

### II. TDD Approach (REQUIRED)

Write tests before implementation using Red-Green-Refactor cycle.

**Rules**:
- Tests MUST be written first and fail before implementation begins
- Implementation MUST only make failing tests pass
- Refactoring occurs only after tests are green
- 80%+ test coverage required for all new code

**Enforcement**: Code review verifies TDD discipline was followed.

---

### III. Modularity & Library-First

Every feature starts as a standalone, self-contained module.

**Rules**:
- Libraries MUST be independently testable
- Clear single responsibility - no organizational-only libraries
- Dependencies between modules MUST be explicit and minimal
- Frontend components MUST be reusable across pages

**Rationale**: AI Screen Code generates frontend code from UI designs; the codebase must itself demonstrate clean component architecture.

---

### IV. Code Quality Standards

**Rules**:
- Functions MUST be small (< 50 lines)
- Files MUST be focused (< 800 lines)
- Deep nesting MUST NOT exceed 4 levels
- Error handling MUST be comprehensive
- Console.log statements are prohibited in production code
- No hardcoded values - use configuration
- Immutability patterns MUST be used (no mutations)

**Enforcement**: Linting and code review gate.

---

### V. Security & Secrets

**Rules**:
- No hardcoded secrets (API keys, passwords, tokens)
- All user inputs MUST be validated
- SQL injection prevention via parameterized queries
- XSS prevention via sanitized HTML
- CSRF protection enabled
- Rate limiting on all API endpoints
- Error messages MUST NOT leak sensitive data

---

## Additional Constraints

### Technology Stack

| Component | Technology |
|-----------|------------|
| Frontend | React 19 + TypeScript |
| UI Styling | Tailwind CSS 4.x |
| Backend | Rust + Axum |
| Desktop | Tauri 2.x |
| State Management | Zustand |
| Build Tool | Vite 7.x |
| Testing | Vitest (frontend), cargo test (backend), Playwright (E2E) |

### Quality Gates

All PRs MUST pass:
- [ ] Unit tests (80%+ coverage)
- [ ] Integration tests
- [ ] E2E tests for critical user flows
- [ ] Type checking (TypeScript strict mode)
- [ ] Linting (no violations)
- [ ] Security scan (no vulnerabilities)

---

## Development Workflow

### Feature Implementation

1. **Plan First**: Create implementation plan with architecture review
2. **TDD Approach**: Write tests first (Red-Green-Refactor)
3. **Code Review**: Submit for review after implementation
4. **Verification**: Automated tests MUST pass
5. **Security Review**: Verify no security vulnerabilities
6. **Commit & Push**: Detailed commit messages

### Test Requirements

- **Unit Tests**: Individual functions, utilities, components
- **Integration Tests**: API endpoints, database operations
- **E2E Tests**: Critical user flows via Playwright

### Review Process

1. Author submits PR with test evidence
2. Reviewer verifies:
   - Tests pass
   - Coverage meets threshold
   - No security issues
   - Code follows quality standards
3. CI/CD validates automatically
4. Merge only after all gates green

---

## Governance

### Amendment Procedure

1. Propose change with rationale
2. Document migration plan if needed
3. Obtain approval from project lead
4. Update constitution version
5. Propagate changes to templates

### Versioning Policy

- **MAJOR**: Backward incompatible governance/principle removals or redefinitions
- **MINOR**: New principle/section added or materially expanded guidance
- **PATCH**: Clarifications, wording, typo fixes, non-semantic refinements

### Compliance Review

- All PRs/reviews MUST verify compliance with constitution
- Complexity MUST be justified when deviating from principles
- Use `.specify/templates/plan-template.md` for implementation guidance

---

**Version**: 1.0.0 | **Ratified**: 2026-03-21 | **Last Amended**: 2026-03-21
