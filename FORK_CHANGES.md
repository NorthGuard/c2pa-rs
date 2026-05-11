# Changes in this fork

This document lists all changes made in this fork compared to upstream [contentauth/c2pa-rs](https://github.com/contentauth/c2pa-rs), and the reason for each.

---

## Certificate trust type fix

**Where:** `sdk/src/crypto/raw_signature/openssl/check_certificate_trust.rs` (line 52)

**Change:** Replaced `verify_param.set_time(st);` with `verify_param.set_time(st.try_into().unwrap());`.

**Why:** Bug fix for Rust type mismatch when building for `i686-linux-android`.  
Commit message: *"Bug fix: Rust type mismatch in c2pa for i686-linux-android."*

---

## Expose Merkle API

**What was added:**

- New public module `merkle_utils` with:
  - `compute_merkle_root(leaf_hashes, alg)` — returns the Merkle root from pre-hashed leaves.
  - `compute_merkle_proof(leaf_hashes, leaf_index, alg)` — returns the Merkle inclusion proof for a given leaf index.
- Re-exported types at crate root: `MerkleNode`, `C2PAMerkleTree`.
- Re-exported functions at crate root: `compute_merkle_root`, `compute_merkle_proof`.

**Why:** To allow downstream code to compute C2PA Merkle roots and inclusion proofs from pre-hashed leaves using the same algorithm as the SDK, without reimplementing it.

---

## Expose BMFF hash utilities

**What was added:**

- `compute_bmff_flat_hash(reader, alg)` — thin wrapper around `BmffHash::new` + `set_default_exclusions` + `gen_hash_from_stream`. Returns the same flat hash stored in `c2pa.hash.bmff.v3.data.hash`. Must be called on the signed output file.
- `compute_bmff_mdat_merkle_roots(reader, chunk_size_kb, alg)` — thin wrapper that calls `create_merkle_tree_for_merkle_map` for each mdat box and returns the root. Returns the same roots the signer computes for `c2pa.hash.bmff.v3.data.merkle`. Must be called on the pre-sign input file.
- Both re-exported at crate root via `assertions::mod.rs`.

**Why:** To allow downstream code (e.g. DID/TEE signing of the Merkle root before calling `sign_file`) to obtain the exact values that the C2PA signer will embed, without reimplementing the hashing or tree construction.

---

## Open follow-ups

### Bump the fork to current upstream

**Status:** TODO (no fixed deadline).

**What:** Rebase the patches above onto a recent `contentauth/c2pa-rs` `main`. The fork currently pins SDK v0.77.0 while upstream has moved on (see crates.io for the latest tag). Most of the upstream delta is hardening fixes in BMFF / Merkle / claim-parsing paths plus general clippy / workflow updates — landing them is straightforward maintenance work, not a redesign.

**Why this matters:**

- Several upstream PRs since the fork point harden code we now actively call (`reality-backend/crates/attestation/src/sdk_check.rs` runs `Reader::from_stream` on every L1 PUT). Examples in the upstream log: BMFF Merkle integer-underflow guards, BMFF chunk-mapping bounds checks, claim-malformed parser hardening, multi-rendition validation fixes.
- The fork's reason-for-existing is to support this project (per `reality-backend/architecture.md` §17.2). Drift from upstream is acceptable when nothing in this project bumps into it; the moment the drift causes a problem here — a missing bug fix, a missing assertion shape we need to validate, a CVE — we **uplift the fork immediately** rather than work around it. This rule is repeated in `reality-backend/architecture.md` §17.2 so the policy lives next to the dependency analysis as well.

**What it costs to defer:** as long as no concrete bug in this project traces back to a missing upstream fix, deferring is fine. The check is part of the wider dependency-freshness work tracked in `reality-backend/architecture.md` §17.3.

**How to refresh once it's time:**

1. `git remote add upstream git@github.com:contentauth/c2pa-rs.git` and `git fetch upstream`.
2. Rebase the fork's commits (Merkle exposure, BMFF utility exposure, the `i686-linux-android` type-mismatch fix) onto a tagged upstream release.
3. Update `version` in `sdk/Cargo.toml` and `forks/c2pa-rs/Cargo.toml` to the new upstream version.
4. Run `cargo test -p c2pa` inside the fork plus `just test-e2e` at the repo root to confirm nothing regressed.
5. Update this section with the new sync point.
