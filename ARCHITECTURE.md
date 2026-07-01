# signal-mentci - architecture

`signal-mentci` is the wire contract for Mentci's programmable UI. It is a pure
schema-derived contract crate: generated rkyv records, optional NOTA projection,
and a `signal-frame` request/reply/stream envelope.

## Direction

Mentci is a state-bearing programmable UI component: the daemon owns the canonical UI state, and clients — CLI, egui shell, TUI, editor pane, status bar, agentic flows — are thin producers/subscribers over that daemon-owned state. A UI change exists because the daemon state changed.

The criome escalation slot (`ApprovalSource::CriomeEscalation`) is cross-imported from `signal_criome::schema::lib`, not redefined here. signal-mentci owns the slot's placement in the approval question; criome owns the type. Clients answer by sending `AnswerQuestion` to the Mentci daemon, and the daemon delivers the verdict to criome by that slot — clients never open a criome socket directly.

The daemon holds its criome connection in read-only or write mode and mirrors that access level to clients via `InterfaceState::criome_access`. Clients present answer controls only when the daemon has write access; they never elevate the access level themselves.

The verdict is closed: `ApproveSuggestedAnswer`, `Reject`, or `Defer`. When the psyche edits a suggestion, Mentci creates an `AnswerProposal` object that goes through the normal authorization path; there is no verdict side channel.

## Owned

- `Input` / `Output` operation roots for the Mentci UI surface, including
  `Input::AnswerQuestion(ApprovalVerdict)` — the single seam a client uses to
  answer, criome-sourced or not.
- `QuestionProposal`, `ApprovalQuestion`, `ApprovalVerdict`, `ApprovalDecision`,
  `ApprovalSource`, and `AnswerProposal`.
- Intercept-policy control requests that thin Mentci clients send to the
  Mentci daemon for forwarding to criome's meta socket:
  `CreateInterceptPolicy`, `ReplaceInterceptPolicy`, `CancelInterceptPolicy`,
  `ListInterceptPolicies`, `FetchParkedRequests`, and `AnswerParkedRequest`.
- `InterfaceState`, `ProjectedInterfaceState`, `InterfaceInterest`, and the
  interface-state subscription.
- Daemon-minted identity records such as `QuestionIdentifier`,
  `ProposalIdentifier`, and `SubscriptionToken`.

## Cross-imported, not owned

- `AuthorizationRequestSlot` is re-exported from `signal_criome::schema::lib`
  and carried inside `ApprovalSource::CriomeEscalation(AuthorizationRequestSlot)`.
  signal-mentci owns the slot's placement in the approval question; criome owns
  the type. This is the seam the daemon routes verdicts on — a client emits
  `AnswerQuestion` and the daemon delivers the verdict to criome by the parked
  slot, so the client never opens a criome socket.
- Intercept policy and parked Spirit request records are imported from
  `signal-criome`. Mentci owns only the client-facing control roots and UI
  placement; criome owns policy state, target keys, parked request identity, and
  answer audit records.

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
- A criome-sourced question carries its `AuthorizationRequestSlot` in
  `ApprovalSource::CriomeEscalation`; that slot is the daemon's routing key,
  never a client's.
- A criome-intercepted Spirit operation carries its `ParkedRequestIdentifier`
  in `ApprovalSource::CriomeInterception`; the raw operation payload itself
  remains in the imported `ParkedSpiritRequest`/question context for rendering.
- `InterfaceState` carries the daemon's criome access mode as
  `criome_access: CriomeAccess` (`CriomeAccess [ReadOnly ReadWrite]`); clients
  read it through `ProjectedInterfaceState::criome_access` and present answer
  controls only when the daemon has write access.
- This crate stays wire-only: no actors, storage, daemon clients, or sockets.
