# signal-mentci architecture

## Center

This repository owns what Mentci says and is told: the interface state it
projects, the questions it presents, the verdicts it records, and the Criome
views it surfaces to a human.

## Authority and projection

`ethos/signal.ethos` is the sole textual source — a `Signal` root holding the
import list, the request variants, the reply variants, and the type
declarations. `ethos-zero` projects it into `src/generated/signal.rs`, which
is committed; `build.rs` generates afresh and asserts equality, so a build
cannot succeed while source and projection differ.

`src/lib.rs` re-exports the projection, and re-exports the portable frame from
`signal` so a consumer speaks this contract without naming `signal` itself.
The request and reply roots are named `Query` and `Response` — `ethos-zero`
names them, not this contract.

## What is imported, and why

Eleven names come from `signal-criome`: `ParkedRequestQuery`,
`ParkedRequestAnswer`, `ParkedRequestSnapshot`, `ParkedRequestResolution`,
`ParkedRequestIdentifier`, `InterceptPolicy`, `InterceptPolicyProposal`,
`InterceptPolicyCancellation`, `InterceptPolicyIdentifier`,
`ActiveInterceptPolicies` and `AuthorizationRequestSlot`.

They are Criome's facts. Mentci is the surface a human meets them through, and
a surface that redeclared them would be a second wire for the same thing — two
shapes to keep in step, and a silent divergence the first time one moved. The
import is what makes `MentciReply::ParkedRequestAnswered` carry the very value
Criome produced.

The portable rkyv frame comes from `signal`, imported and never copied: one
frame type across the estate is what lets a router carry every contract
through one generic path.

## Shape

Nothing in this contract reaches itself; `InterfaceProjection` nests
`InterfaceState`, which holds an `ApprovalQuestion` and a `PaneContent`, and
the nesting bottoms out in text and counters. The whole contract therefore
fits the rkyv archive the Signal frame carries.

`NotificationSlice` is `Empty` or `Present`, not an `Option<NotificationText>`:
the absence of a notification is a state the interface displays, not a missing
field.

## The producer cut this contract is generated against

`ethos-zero` 9.0.0 `b232d35e`, whose projection derives
`datom_codec::Composing` from the split composing kind that `datom-codec`
0.27.0 `6dccc76b` reintroduced arity into; `signal` 5.0.0 `7bcb0949`,
generated against the same pair; `protos` 0.30.1 `171b21f6`; and
`signal-criome` 2.0.0 `5bfa61b5`, generated against all four.

These move together and are spelled identically everywhere, without a `.git`
suffix: cargo source identity is the pin string, not the commit, so one commit
under two spellings is two packages — and `signal` carries `links = "signal"`,
which admits exactly one per graph.

## Boundaries

`signal` for the frame and the framing, `signal-criome` for Criome's shapes,
`rkyv` for the archive, and under the optional `datom` feature `datom-codec`
and `protos` for the Datom text projection. Nothing else, and nothing here
reaches a filesystem, a socket, or a clock.
