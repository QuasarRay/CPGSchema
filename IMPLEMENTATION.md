# Implementation contract

This repository implements the original Code Property Graph schema at revision `d02d76ea5c9e3b398317d334228d7e8e78ab45aa` in two independently maintained packages: native Rust first, then the standalone JSON Schema dialect. Both retain the original Scala module spelling and source semantics. CPG means Code Property Graph; CGP means Context-Generic Programming.

This milestone is **UNVERIFIED**. Only compilation and compiler-driven fixes are allowed during implementation. Tests, formal proofs, source-equivalence comparisons, benchmarks, fuzzing, mutation campaigns, conformance campaigns, and correctness or completion evidence reports are outside this Goal. Existing prototype reference and verification files remain historical material.

Production validation is implemented functionality, not an instruction to execute a verification campaign. Neither implementation depends on the other. Native and JSON definitions, semantics, graph codecs, and exporters require explicit manual synchronization when changed.

The native Codex Goal owns continuation and lifecycle. This document is implementation guidance and does not control Goal status. At implementation completion, perform only native Goal completion bookkeeping, give the brief unverified status, and stop. Do not add a final compilation, inspection, review, or evidence phase.
