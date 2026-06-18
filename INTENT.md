# INTENT - signal-mentci

`signal-mentci` is the ordinary working signal contract for the Mentci
programmable-UI daemon.

Mentci is a state-bearing programmable user-interface component. The daemon owns
the canonical UI state; clients such as the TUI, CLI, editor pane, status bar,
notification surface, and agentic flows are thin producers/subscribers over that
daemon-owned state. A UI change exists because the daemon state changed.

This crate owns the typed wire vocabulary for that surface:

- presenting approval questions without caller-supplied local identifiers;
- pushing interface updates into the daemon;
- observing projected interface state through filtered subscriptions;
- answering questions with the closed `ApprovalDecision` set;
- proposing an edited answer as a new typed proposal object, not as a verdict
  payload;
- retracting interface observations by daemon-minted subscription token.

Criome authorizes submitted content-addressed objects. It does not mint answer
objects from a verdict side channel. Therefore Mentci's verdict is closed:
`ApproveSuggestedAnswer`, `Reject`, or `Defer`. If the psyche edits a suggestion,
Mentci creates an `AnswerProposal` object and that object goes through the normal
authorization path.

This crate is a pure contract crate. It does not own the Mentci daemon, kameo
actors, durable SEMA state, Unix sockets, notification integrations, or criome
verdict signing. Those live in the daemon repo and in criome.
