# INTENT - signal-mentci

`signal-mentci` is the ordinary working signal contract for the Mentci
programmable-UI daemon.

Mentci is a state-bearing programmable user-interface component. The daemon owns
the canonical UI state; clients such as the CLI, egui shell, TUI, editor pane,
status bar, notification surface, and agentic flows are thin
producers/subscribers over that daemon-owned state. A UI change exists because
the daemon state changed.

This crate owns the typed wire vocabulary for that surface:

- presenting approval questions without caller-supplied local identifiers;
- pushing interface updates into the daemon;
- observing projected interface state through filtered subscriptions;
- answering questions with the closed `ApprovalDecision` set;
- proposing an edited answer as a new typed proposal object, not as a verdict
  payload;
- retracting interface observations by daemon-minted subscription token.

## The criome escalation slot is cross-imported, not redefined

A Mentci approval question may originate as a criome escalation. When it does,
`ApprovalSource::CriomeEscalation` carries the `signal-criome`
`AuthorizationRequestSlot` — re-exported from `signal_criome::schema::lib`, not
a duplicate type — that identifies the authorization criome parked. This is the
seam the daemon routes on: a client answers by sending `AnswerQuestion` to the
daemon, and the daemon hands criome the verdict keyed by that slot (the
daemon-routing decision of 2026-06-21; clients never open a criome socket).
signal-mentci owns the slot's *placement in the approval question*; criome owns
the slot type itself.

## Daemon-mirrored criome access mode

The daemon holds its criome connection in read-only or write mode and mirrors
that access level to clients. `InterfaceState` carries the daemon's criome
access mode as a `criome_access: CriomeAccess` field (`CriomeAccess`
`[ReadOnly ReadWrite]`); a client folding the projected interface state reads
it through `ProjectedInterfaceState::criome_access` and knows whether to
present answer controls — observation-only for a read-only daemon, answerable
for a write daemon. The access level is the daemon's to set and the client's to
reflect; the client never elevates it.

## Closed verdicts

Criome authorizes submitted content-addressed objects. It does not mint answer
objects from a verdict side channel. Therefore Mentci's verdict is closed:
`ApproveSuggestedAnswer`, `Reject`, or `Defer`. If the psyche edits a suggestion,
Mentci creates an `AnswerProposal` object and that object goes through the normal
authorization path.

This crate is a pure contract crate. It does not own the Mentci daemon, kameo
actors, durable SEMA state, Unix sockets, notification integrations, or criome
verdict signing. Those live in the daemon repo and in criome.
