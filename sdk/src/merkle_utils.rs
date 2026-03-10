// Copyright 2023 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.

// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// the specific license governing permissions and limitations under
// each license.

//! C2PA Merkle tree helpers for computing roots and inclusion proofs from pre-hashed leaves.

use crate::utils::merkle::{C2PAMerkleTree, MerkleNode};
use crate::{Error, Result};

/// Compute the Merkle root from pre-hashed leaves using the C2PA Merkle tree algorithm.
/// Returns the root hash bytes, or an error if leaves is empty.
pub fn compute_merkle_root(leaf_hashes: Vec<Vec<u8>>, alg: &str) -> Result<Vec<u8>> {
    let leaves: Vec<MerkleNode> = leaf_hashes.into_iter().map(MerkleNode).collect();
    let tree = C2PAMerkleTree::from_leaves(leaves, alg, false);
    tree.get_root()
        .cloned()
        .ok_or_else(|| Error::BadParam("empty merkle tree".into()))
}

/// Compute the Merkle inclusion proof for a given leaf index.
/// Returns the sibling hashes from leaf to root.
pub fn compute_merkle_proof(
    leaf_hashes: Vec<Vec<u8>>,
    leaf_index: usize,
    alg: &str,
) -> Result<Vec<Vec<u8>>> {
    let leaves: Vec<MerkleNode> = leaf_hashes.into_iter().map(MerkleNode).collect();
    let tree = C2PAMerkleTree::from_leaves(leaves, alg, false);
    tree.get_proof_by_index(leaf_index, tree.layers.len())
}
