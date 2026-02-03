# Hikmalayer Core

## What is Hikmalayer core?
Hikmalayer Core is a hybrid Layer‑1 blockchain prototype that combines Proof‑of‑Stake (validator
selection) with Proof‑of‑Work (block finalization). It provides:

- PoS validator selection, staking, and validator signature verification.
- PoW mining and PoW validation for every block.
- Governance and slashing configuration for validator accountability.
- Persistence of chain state to disk for safe restarts.
- P2P gossip and peer discovery for block propagation.
- A React dashboard for local interaction and testing workflows.

This repository is a production‑focused hybrid L1 foundation that implements the core consensus
mechanics and operational services needed for industrial‑grade deployments. Phase 4 hardening is
in progress to complete the production readiness checklist.

For the official whitepaper, see `docs/Whitepaper.md`.

## Licence
Refer to the repository’s license policy (add or update a LICENSE file as needed for distribution).

## Development process
Hikmalayer Core is developed in phases:

- **Phase 1**: Core PoW and chain primitives.
- **Phase 2**: PoS validator selection, staking, and validator identities.
- **Phase 3**: Persistence, P2P gossip, governance, slashing, and async‑safe services.
- **Phase 4 (upcoming)**: Production‑grade key management, hardened network authentication,
  finalized‑state tracking, and operational tooling for multi‑node deployments.

Phase 3 capabilities are implemented in this repository. Phase 4 is planned and will focus on
industrial‑scale security, operations, and scaling.

## Testing
Run the Rust test suite:

```bash
cargo test
```

## Automated testing
Automated tests are currently run via `cargo test` and include unit coverage for chain validation,
transactions, and PoS selection.

## Manual Quality Assurance testing
Manual QA can be performed using the API and dashboard:

- Start the backend (`cargo run`) and the dashboard (`npm run dev` in `dashboard/`).
- Verify mining, staking, transfers, and validation flows.
- Validate P2P peer registration and block gossip by running two nodes with different ports.

For secured environments, set `P2P_TOKEN` and `ADMIN_TOKEN` to require `x-p2p-token` and
`x-admin-token` headers for P2P and governance/slashing endpoints.

## Translations
No translations are included yet. If you want to add documentation translations, create locale‑
specific README files (for example `README.es.md`, `README.fr.md`).

## Project directory
```
hikmalayer/
├── dashboard/
│   ├── node_modules/
│   ├── public/
│   ├── src/
│   │   ├── assets/
│   │   │   └── react.svg
│   │   ├── components/
│   │   │   ├── BlockchainViewer.jsx
│   │   │   ├── CertificateManager.jsx
│   │   │   ├── MiningActions.jsx
│   │   │   ├── ProtectedAction.jsx
│   │   │   ├── StatsGrid.jsx
│   │   │   ├── TokenManager.jsx
│   │   │   ├── WalletAuth.css
│   │   │   └── WalletAuth.jsx
│   │   ├── hooks/
│   │   │   ├── useAuthenticatedApi.js
│   │   │   └── useWallet.jsx
│   │   ├── api.js
│   │   ├── App.css
│   │   ├── App.jsx
│   │   ├── index.css
│   │   └── main.jsx
│   ├── .gitignore
│   ├── eslint.config.js
│   ├── index.html
│   ├── package-lock.json
│   ├── package.json
│   ├── postcss.config.js
│   ├── README.md
│   ├── tailwind.config.js
│   └── vite.config.js
├── docs/
├── node_modules/
├── src/
│   ├── api/
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── auth/
│   │   ├── middleware.rs
│   │   ├── mod.rs
│   │   ├── routes.rs
│   │   └── signature.rs
│   ├── blockchain/
│   │   ├── block.rs
│   │   ├── chain.rs
│   │   ├── mod.rs
│   │   └── transaction.rs
│   ├── consensus/
│   │   ├── mod.rs
│   │   ├── pos.rs
│   │   └── pow.rs
│   ├── contract/
│   │   ├── contract.rs
│   │   └── mod.rs
│   ├── governance.rs
│   ├── persistence.rs
│   ├── token/
│   │   ├── fungible.rs
│   │   └── mod.rs
│   ├── utils/
│   └── main.rs
├── data/
├── target/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── package-lock.json
├── package.json
└── README.md
```
