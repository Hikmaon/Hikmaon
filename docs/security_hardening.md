# Security Hardening Checklist (Phase 4)

Status: In progress. This document tracks production hardening work and highlights gaps that still
require implementation and validation.

## Validator / Node Authentication
- [ ] Key rotation procedures implemented and tested
- [ ] Secure keystore integration (HSM or vault-backed)
- [ ] Permissioned validator onboarding enforced (no anonymous validators)

## RPC & API Protection
- [ ] Rate limiting configured for public endpoints
- [ ] DDoS protection in place (reverse proxy/WAF)
- [ ] Debug endpoints removed or access-restricted

## P2P Hardening
- [ ] Peer allowlist/denylist enforcement
- [ ] Peer reputation/banning logic
- [ ] Secure handshakes and authentication

## Secrets Management
- [x] Tokens provided via environment variables
- [ ] Vault-backed secrets for production

## Threat Model & Audits
- [ ] Threat model finalized (see `docs/threat_model.md`)
- [ ] Static code scan and dependency audit completed
- [ ] Consensus edge cases / reentrancy review completed
