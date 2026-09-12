# signal-mentci

The ordinary Signal contract for Mentci: the human interface surface.
Projected interface state, approval questions and their verdicts, notification
and pane mutation, and the parked-request and intercept-policy views Mentci
presents on Criome's behalf.

`ethos/signal.ethos` is the schema authority. `build.rs` regenerates the
projection with `ethos-zero` and asserts it against the committed
`src/generated/signal.rs`, so the two can never drift. The crate's public
surface is that projection, re-exported from `src/lib.rs` under the names the
ethos declares.

The Criome-owned shapes this contract carries — parked requests, intercept
policies, authorization slots — are imported from `signal-criome`, never
restated. Mentci shows Criome's facts; it does not own a second copy of them.

`examples/canonical.datom` holds one canonical Datom value per line, written
by the codec and never spelled by hand; `tests/contract.rs` asserts every line
is exactly what the codec writes and that each actualizes back into exactly
one of `Query` and `Response`.

The crate owns wire vocabulary. It owns no daemon, no interface, and no
policy.
