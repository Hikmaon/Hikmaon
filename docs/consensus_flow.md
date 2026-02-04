# Consensus Flow (Hybrid PoS/PoW)

Status: Draft. This document explains the hybrid validator selection + PoW finalization flow.

1. Validators stake and register keys.
2. A deterministic PoS selection chooses the validator based on the previous block hash and
   staker set hash.
3. The selected validator signs the block payload and performs PoW mining to satisfy difficulty.
4. Peers validate PoS selection, signature, and PoW before accepting the block.
