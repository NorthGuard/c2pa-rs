# Changes in this fork

This document lists all changes made in this fork compared to upstream [contentauth/c2pa-rs](https://github.com/contentauth/c2pa-rs), and the reason for each.

---

## Certificate trust type fix

**Where:** `sdk/src/crypto/raw_signature/openssl/check_certificate_trust.rs` (line 52)

**Change:** Replaced `verify_param.set_time(st);` with `verify_param.set_time(st.try_into().unwrap());`.

**Why:** Bug fix for Rust type mismatch when building for `i686-linux-android`.  
Commit message: *"Bug fix: Rust type mismatch in c2pa for i686-linux-android."*

