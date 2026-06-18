# signal-mentci - architecture

`signal-mentci` is the wire contract for Mentci's programmable UI. It is a pure
schema-derived contract crate: generated rkyv records, optional NOTA projection,
and a `signal-frame` request/reply/stream envelope.

## Owned

- `Input` / `Output` operation roots for the Mentci UI surface.
- `QuestionProposal`, `ApprovalQuestion`, `ApprovalVerdict`, and
  `AnswerProposal`.
- `InterfaceState`, `ProjectedInterfaceState`, `InterfaceInterest`, and the
  `InterfaceStateStream` subscription.
- Daemon-minted identity records such as `QuestionIdentifier`,
  `ProposalIdentifier`, and `SubscriptionToken`.

## Not Owned

- The Mentci daemon and CLI runtime.
- Durable state and SEMA tables.
- Kameo actors and socket listeners.
- Criome key custody and verdict signing.
- UI rendering policy in individual clients.

## Invariants

- `PresentQuestion` carries a `QuestionProposal`; the daemon mints the
  `QuestionIdentifier`.
- `ObserveInterfaceState` carries an `InterfaceInterest`; the daemon mints the
  `SubscriptionToken`.
- Subscriptions receive `ProjectedInterfaceState`, not necessarily the full
  `InterfaceState`.
- `ApprovalDecision` is closed: `ApproveSuggestedAnswer`, `Reject`, or `Defer`.
  Authored answer bodies are `AnswerProposal` objects admitted separately.
- This crate stays wire-only: no actors, storage, daemon clients, or sockets.
