# Viska fork of rs-matter

This fork exists to carry patches viska-edge needs that upstream does not
have. It is a **permanent arrangement**, not a staging area — patches are not
sent upstream. `rs-matter` from crates.io is therefore not an option.

The cost of a permanent fork is rebasing, so the fork is deliberately kept as
small as it can possibly be: **one patch**.

## The patch series

Branch `viska/main` = an upstream commit plus the commits below, in order.

| # | Commit | What | Why not upstream |
|---|--------|------|------------------|
| 1 | Add Generic Switch cluster handler (0x003B) | `dm/clusters/switch.rs` | Upstream generates `decl::switch` from the IDL (`client cluster Switch = 59`) but ships no server handler |

Current base: **336ca6a** (`Support for PICS and the TH tool (#541)`, 2026-09-02).

A specific commit, not a tag or a branch: the only tag upstream publishes is
`v0.2.0` (2026-06), while `Cargo.toml` already says 0.3.0 — there is no tag
matching what we build against. Pinning a branch would make each rebase's
starting point unrecoverable after the fact.

## Rebasing forward

```sh
git fetch upstream
git rebase --onto <new-upstream-commit> <old-base> viska/main
```

Then update the base commit above, and rebuild viska-edge with its real
feature set — `cargo check` on rs-matter alone does not exercise what edge
depends on:

```sh
cargo build -p rs-matter --features \
  async-io,groups,max-groups-per-fabric-12,max-group-keys-per-fabric-2,max-group-endpoints-per-fabric-3
```

Note `groups` is not in upstream's default feature set, and viska-edge uses
`GroupsHandler` throughout.

## Patches that were dropped, and why

Do not resurrect these. Upstream solved each one independently — sometimes
differently than we did, which is why the fork's version has to go rather than
be carried forward:

| Ours | Upstream's answer |
|---|---|
| CASE initiator | `sc/case/initiator.rs`, plus session resumption |
| SAI/SII operational mDNS TXT records | `transport/network/mdns.rs`, with SAI/SII fixed to u32 per spec |
| `ImClient::subscribe` behind a feature | `im.rs`, unconditional — the `im-client-subscribe` feature is gone |
| StatusReport(SessionNotFound) on unknown SID | `transport.rs` |
| Panic fix in `remove_exch` | the function no longer exists; session handling was rewritten |
| Stale CASE exchange blocking a new handshake | same rewrite |
| Make `spake2p` module public | the *types* are re-exported from `sc::pase` while the module stays `pub(crate)` — import from `sc::pase`, not `sc::pase::spake2p` |
| `commissioner/` (RCAC/ICAC/NOC minting) | `onboard::cac::{RcacGenerator, IcacGenerator}` + `onboard::noc::NocGenerator`. Upstream keeps these as free primitives returning `(privkey, cert)` so the caller decides what to retain; the stateful container that wrapped them is viska-edge's concern, not rs-matter's |
