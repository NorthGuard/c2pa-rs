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
