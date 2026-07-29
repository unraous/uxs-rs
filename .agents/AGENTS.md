# Project Rules & Engineering Principles

## 1. Audit Existing Architecture Before Invention (禁造重复轮子 & 结合已有设计)

- **Inspect Established Patterns First**:
  Before implementing any new feature, bug fix, or codegen logic, thoroughly audit the existing codebase for pre-existing utility macros, helper crates, and architectural patterns (e.g. `auto_handler` proc-macro).
- **Extend Rather Than Duplicate**:
  New capabilities must integrate into and extend existing pipelines (e.g., adding TypeScript generation into `auto_handler` alongside permissions JSON sync) rather than introducing standalone scripts, ad-hoc string parsers, or fragmented initialization boilerplate.
- **Strict Anti-Wheel Policy**:
  NEVER hand-roll custom implementations for problems solved by established compiler features or official crates (e.g., hand-rolled AST string-to-TS type mappers like `rust_type_to_ts` vs. official reflection with `tauri_specta`).

## 2. Compile-Time Codegen & Macro Pipeline Abstraction (宏与代码生成标准)

- **Unified Macro Side-Effect Pipeline**:
  In projects using proc-macros for code/handler registration, all compile-time side-effects (e.g. permission JSON sync, TypeScript bindings export, routing generation) MUST be structured as parallel, co-located tasks within a single AST traversal pipeline in the macro expansion.
- **Entrypoint Cleanliness Invariant**:
  Application entry points (`src/lib.rs`, `src/main.rs`) MUST remain 100% clean, native, and free of verbose generation/export boilerplate.
- **Preserve System Routing Integrities**:
  Sub-system helpers or macro expansions MUST NOT break or replace primary framework handlers (e.g., replacing Tauri's `auto_handler` with sub-crate handlers that break unannotated IPC commands).
