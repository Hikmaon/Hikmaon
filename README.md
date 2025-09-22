## PROJECT DIRECTORY STRUCTURE

```
hybridchain/
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
│   ├── token/
│   │   ├── fungible.rs
│   │   └── mod.rs
│   ├── utils/
│   └── main.rs
├── target/
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── package-lock.json
├── package.json
└── README.md
```
