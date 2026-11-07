# Upgrades

## 1.0.0 → 1.1.0

First Datom/Protos 0.33 adoption: generated scalar aliases become tuple
structs with Datom derives; Rust fixtures now construct those wrappers and
use checked ReaderBudget construction. Authored Ethos, library, build script
and binary archive configuration are unchanged.

The coherent producer cohort is Datom eabead4 / Protos 3c7ade3 (0.33.0),
Signal 7afb68b (10.3.0), signal-criome e30cada (2.1.0), and Ethos Zero
5a5daca (19.2.0). Registry dependency versions remain unchanged.

The canonical comparison and explicit rewrite use compact() to retain the
one-line fixture format despite the current vertical printer. A pre-existing
fixture completeness defect is corrected: baseline ed750b constructs 15
queries and 13 responses, while its committed file contained only 11 queries
and 9 responses. The eight missing existing roster/conversation/psyche values
are added; the original 20 lines remain byte-identical and in their original
relative order. The added text comes from the historical producer's actual
witness output. The unchanged exact comparison verifies current compact
output against all 28 lines. This adds test coverage, not contract behavior.
Historical and pre-correction candidate full-test failures are retained as
evidence; successful witness-only tests do not erase those failures.

Historical ed750b with its original source, lock and tools and the candidate
produce equal archive/frame bytes for 28 samples (15 Query and 13 Response).
Each local frame restores and both directions of binary/text typed reads
succeed. Raw printed text is equal for 18/28; balanced-string/token inspection
shows layout-only differences for the remaining ten samples. Binary equality
and text parsing are independent observations. These are canonical samples,
not exhaustive nested-value, source-API, storage or runtime guarantees.

Checks cover x86_64-linux only; the three other declared systems are omitted.
The deliberate source-rewriting test stays ignored in normal checks.
