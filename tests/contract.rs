//! The contract's own witness: every canonical value survives the wire, and
//! every canonical Datom line is the codec's own text for one of them.

use signal_criome::{
    ExpiryAction, InterceptPolicy, InterceptPolicyCancellation, InterceptPolicyProposal,
    InterceptPolicyWindow, InterceptTargetSelector, ParkedRequestAnswer, ParkedRequestDecision,
    ParkedRequestQuery, PolicyOverlapMode,
};
use signal_mentci::{
    ActivitySource, AnswerProposal, AnswerProposalAdmitted, ApprovalDecision, ApprovalQuestion,
    ApprovalSource, ApprovalVerdict, ByteViewable, ConversationCursor, ConversationEntry,
    ConversationObservation, ConversationSnapshot, CorrelationStatus, CriomeAccess, FlowSnapshot,
    FlowState, InterceptPolicyObservation, InterfaceInterest, InterfaceMutation,
    InterfaceObservationOpened, InterfaceObservationRetracted, InterfaceProjection, InterfaceState,
    InterfaceStateObservation, InterfaceUpdate, NotificationSlice, OperationFailure, PaneContent,
    ProjectedInterfaceState, Provenance, PsycheSubmission, Query, QuestionContext,
    QuestionPresented, QuestionProposal, ReceiptGrade, Rejection, RejectionReason, Response,
    Restorable, RosterObservation, RosterSnapshot, Signal, Signalizable, SourceKind, SourceStatus,
    SubmissionDisposition, SubmissionReceipt, Unavailability, UpdateAccepted, VerdictAccepted,
};

fn question_proposal() -> QuestionProposal {
    QuestionProposal {
        approval_source: ApprovalSource::CriomeEscalation(signal_criome::AuthorizationRequestSlot(
            String::from("slot-1"),
        )),
        prompt_text: signal_mentci::PromptText(String::from("approve the release?")),
        answer_text: signal_mentci::AnswerText(String::from("yes")),
        explanation_text: signal_mentci::ExplanationText(String::from(
            "the quorum is short by one",
        )),
        question_context: QuestionContext {
            context_label: signal_mentci::ContextLabel(String::from("release")),
            context_body: signal_mentci::ContextBody(String::from("criome 1.0.0")),
        },
    }
}

fn pane_content() -> PaneContent {
    PaneContent {
        pane_label: signal_mentci::PaneLabel(String::from("main")),
        context_body: signal_mentci::ContextBody(String::from("waiting on a verdict")),
    }
}

fn policy() -> InterceptPolicy {
    InterceptPolicy {
        intercept_policy_identifier: signal_criome::InterceptPolicyIdentifier(String::from(
            "policy-1",
        )),
        mentci_session_slot: signal_criome::MentciSessionSlot(String::from("session-1")),
        intercept_target_selector: InterceptTargetSelector {
            spirit_process_key: signal_criome::SpiritProcessKey(String::from("spirit-1")),
        },
        spirit_operation_names: signal_criome::SpiritOperationNames(vec![
            signal_criome::SpiritOperationName(String::from("Deploy")),
            signal_criome::SpiritOperationName(String::from("Retire")),
        ]),
        intercept_policy_window: InterceptPolicyWindow {
            first_timestamp_nanos: signal_criome::TimestampNanos(1_700_000_000_000_000_000),
            second_timestamp_nanos: signal_criome::TimestampNanos(1_700_000_060_000_000_000),
        },
        expiry_action: ExpiryAction::LeaveParked,
        policy_priority: signal_criome::PolicyPriority(3),
    }
}

fn canonical_queries() -> Vec<Query> {
    vec![
        Query::ListInterceptPolicies(InterceptPolicyObservation {}),
        Query::RetractInterfaceObservation(signal_mentci::SubscriptionToken(String::from(
            "token-1",
        ))),
        Query::ObserveInterfaceState(InterfaceStateObservation {
            subscriber_name: signal_mentci::SubscriberName(String::from("operator")),
            interface_interest: InterfaceInterest::FullInterfaceState,
        }),
        Query::PushUpdate(InterfaceUpdate {
            update_identifier: signal_mentci::UpdateIdentifier(String::from("update-1")),
            interface_mutation: InterfaceMutation::SetPaneContent(pane_content()),
        }),
        Query::PresentQuestion(question_proposal()),
        Query::ProposeEditedAnswer(AnswerProposal {
            question_identifier: signal_mentci::QuestionIdentifier(String::from("question-1")),
            answer_text: signal_mentci::AnswerText(String::from("yes, with the second signature")),
            subscriber_name: signal_mentci::SubscriberName(String::from("operator")),
        }),
        Query::AnswerQuestion(ApprovalVerdict {
            question_identifier: signal_mentci::QuestionIdentifier(String::from("question-1")),
            approval_decision: ApprovalDecision::ApproveSuggestedAnswer,
            subscriber_name: signal_mentci::SubscriberName(String::from("operator")),
        }),
        Query::CreateInterceptPolicy(InterceptPolicyProposal {
            mentci_session_slot: signal_criome::MentciSessionSlot(String::from("session-1")),
            intercept_target_selector: InterceptTargetSelector {
                spirit_process_key: signal_criome::SpiritProcessKey(String::from("spirit-1")),
            },
            spirit_operation_names: signal_criome::SpiritOperationNames(vec![
                signal_criome::SpiritOperationName(String::from("Deploy")),
            ]),
            policy_duration_nanos: signal_criome::PolicyDurationNanos(60_000_000_000),
            expiry_action: ExpiryAction::AutoReject,
            policy_priority: signal_criome::PolicyPriority(3),
            policy_overlap_mode: PolicyOverlapMode::RejectSamePriorityOverlap,
        }),
        Query::CancelInterceptPolicy(InterceptPolicyCancellation {
            intercept_policy_identifier: signal_criome::InterceptPolicyIdentifier(String::from(
                "policy-1",
            )),
        }),
        Query::FetchParkedRequests(ParkedRequestQuery {
            mentci_session_slot_option: Some(signal_criome::MentciSessionSlot(String::from(
                "session-1",
            ))),
            intercept_target_selector_option: None,
        }),
        Query::AnswerParkedRequest(ParkedRequestAnswer {
            parked_request_identifier: signal_criome::ParkedRequestIdentifier(String::from(
                "parked-1",
            )),
            parked_request_decision: ParkedRequestDecision::Approve,
        }),
        Query::ObserveRoster(RosterObservation {
            request_identifier: signal_mentci::RequestIdentifier(String::from("request-roster-1")),
        }),
        Query::ObserveConversation(ConversationObservation {
            request_identifier: signal_mentci::RequestIdentifier(String::from(
                "request-conversation-1",
            )),
            flow_identifier: signal_mentci::FlowIdentifier(String::from("flow-1")),
            conversation_cursor_option: None,
        }),
        Query::ObserveConversation(ConversationObservation {
            request_identifier: signal_mentci::RequestIdentifier(String::from(
                "request-conversation-older",
            )),
            flow_identifier: signal_mentci::FlowIdentifier(String::from("flow-1")),
            conversation_cursor_option: Some(ConversationCursor {
                native_session_identifier: signal_mentci::NativeSessionIdentifier(String::from(
                    "11111111-2222-3333-4444-555555555555",
                )),
                file_device: signal_mentci::FileDevice(1),
                file_inode: signal_mentci::FileInode(2),
                snapshot_bytes: signal_mentci::SnapshotBytes(900),
                before_byte: signal_mentci::BeforeByte(450),
            }),
        }),
        Query::SubmitPsyche(PsycheSubmission {
            request_identifier: signal_mentci::RequestIdentifier(String::from("request-send-1")),
            flow_identifier: signal_mentci::FlowIdentifier(String::from("flow-1")),
            psyche_text: signal_mentci::PsycheText(String::from("  preserve this text exactly  ")),
        }),
    ]
}

fn canonical_responses() -> Vec<Response> {
    vec![
        Response::RequestRejected(Rejection {
            rejection_reason: RejectionReason::QuestionAlreadyAnswered,
        }),
        Response::UpdateApplied(UpdateAccepted {
            update_identifier: signal_mentci::UpdateIdentifier(String::from("update-1")),
            revision_counter: signal_mentci::RevisionCounter(12),
        }),
        Response::QuestionAccepted(QuestionPresented {
            question_identifier: signal_mentci::QuestionIdentifier(String::from("question-1")),
            revision_counter: signal_mentci::RevisionCounter(13),
            timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_000),
        }),
        Response::VerdictRecorded(VerdictAccepted {
            question_identifier: signal_mentci::QuestionIdentifier(String::from("question-1")),
            approval_decision: ApprovalDecision::Defer,
            timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_001),
        }),
        Response::AnswerProposalAccepted(AnswerProposalAdmitted {
            proposal_identifier: signal_mentci::ProposalIdentifier(String::from("proposal-1")),
            question_identifier: signal_mentci::QuestionIdentifier(String::from("question-1")),
            proposal_digest: signal_mentci::ProposalDigest(String::from("d1ge57")),
            revision_counter: signal_mentci::RevisionCounter(14),
        }),
        Response::InterfaceObservationStarted(InterfaceObservationOpened {
            subscription_token: signal_mentci::SubscriptionToken(String::from("token-1")),
            projected_interface_state: ProjectedInterfaceState {
                revision_counter: signal_mentci::RevisionCounter(15),
                interface_projection: InterfaceProjection::FullProjection(InterfaceState {
                    revision_counter: signal_mentci::RevisionCounter(15),
                    status_text: signal_mentci::StatusText(String::from("idle")),
                    notification_text: signal_mentci::NotificationText(String::from(
                        "one question waiting",
                    )),
                    pane_content: pane_content(),
                    approval_question: ApprovalQuestion {
                        question_identifier: signal_mentci::QuestionIdentifier(String::from(
                            "question-1",
                        )),
                        question_proposal: question_proposal(),
                    },
                    criome_access: CriomeAccess::ReadWrite,
                }),
            },
        }),
        Response::InterfaceObservationClosed(InterfaceObservationRetracted {
            subscription_token: signal_mentci::SubscriptionToken(String::from("token-1")),
        }),
        Response::InterceptPolicyCreated(policy()),
        Response::InterceptPolicyCancelled(signal_criome::InterceptPolicyIdentifier(String::from(
            "policy-1",
        ))),
        Response::RosterObserved(RosterSnapshot {
            request_identifier: signal_mentci::RequestIdentifier(String::from("request-roster-1")),
            timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_002),
            source_status: SourceStatus::Complete,
            flows: signal_mentci::Flows(vec![
                FlowSnapshot {
                    flow_identifier_option: Some(signal_mentci::FlowIdentifier(String::from(
                        "flow-1",
                    ))),
                    flow_name: signal_mentci::FlowName(String::from("Fixture Flow")),
                    seat_label: signal_mentci::SeatLabel(String::from("fixture")),
                    flow_state: FlowState::Idle,
                    timestamp_nanos_option: Some(signal_mentci::TimestampNanos(
                        1_700_000_000_000_000_001,
                    )),
                    activity_source: ActivitySource::Herdr,
                    pane_identifier: signal_mentci::PaneIdentifier(String::from("pane-1")),
                    terminal_identifier: signal_mentci::TerminalIdentifier(String::from(
                        "terminal-1",
                    )),
                    correlation_status: CorrelationStatus::Verified,
                },
                FlowSnapshot {
                    flow_identifier_option: None,
                    flow_name: signal_mentci::FlowName(String::from("Uncorrelated fixture")),
                    seat_label: signal_mentci::SeatLabel(String::from("fixture")),
                    flow_state: FlowState::Unknown,
                    timestamp_nanos_option: None,
                    activity_source: ActivitySource::Herdr,
                    pane_identifier: signal_mentci::PaneIdentifier(String::from("pane-2")),
                    terminal_identifier: signal_mentci::TerminalIdentifier(String::from(
                        "terminal-2",
                    )),
                    correlation_status: CorrelationStatus::Unknown,
                },
            ]),
        }),
        Response::ConversationObserved(ConversationSnapshot {
            request_identifier: signal_mentci::RequestIdentifier(String::from(
                "request-conversation-1",
            )),
            flow_identifier: signal_mentci::FlowIdentifier(String::from("flow-1")),
            timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_002),
            source_status: SourceStatus::Partial,
            snapshot_bytes: signal_mentci::SnapshotBytes(900),
            current_bytes: signal_mentci::CurrentBytes(950),
            window_start: signal_mentci::WindowStart(450),
            window_end: signal_mentci::WindowEnd(900),
            conversation_cursor_option: Some(ConversationCursor {
                native_session_identifier: signal_mentci::NativeSessionIdentifier(String::from(
                    "11111111-2222-3333-4444-555555555555",
                )),
                file_device: signal_mentci::FileDevice(1),
                file_inode: signal_mentci::FileInode(2),
                snapshot_bytes: signal_mentci::SnapshotBytes(900),
                before_byte: signal_mentci::BeforeByte(450),
            }),
            entries: signal_mentci::Entries(vec![
                ConversationEntry {
                    entry_identifier: signal_mentci::EntryIdentifier(String::from(
                        "synthetic:flow-1:1",
                    )),
                    sequence: signal_mentci::Sequence(1),
                    source_ordinal: signal_mentci::SourceOrdinal(1),
                    timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_000),
                    entry_text: signal_mentci::EntryText(String::from("Origin is unresolved.")),
                    source_kind: SourceKind::UserInput,
                    provenance: Provenance::Unknown,
                    attributed_actor_option: None,
                },
                ConversationEntry {
                    entry_identifier: signal_mentci::EntryIdentifier(String::from(
                        "synthetic:flow-1:2",
                    )),
                    sequence: signal_mentci::Sequence(2),
                    source_ordinal: signal_mentci::SourceOrdinal(2),
                    timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_001),
                    entry_text: signal_mentci::EntryText(String::from("A final machine response.")),
                    source_kind: SourceKind::FinalResponse,
                    provenance: Provenance::Machine,
                    attributed_actor_option: Some(signal_mentci::AttributedActor(String::from(
                        "Fixture Flow",
                    ))),
                },
            ]),
        }),
        Response::PsycheSubmitted(SubmissionReceipt {
            request_identifier: signal_mentci::RequestIdentifier(String::from("request-send-1")),
            submission_disposition: SubmissionDisposition::Accepted,
            submission_reason_option: None,
            relay_identifier_option: Some(signal_mentci::RelayIdentifier(String::from("relay-1"))),
            event_identifier_option: Some(signal_mentci::EventIdentifier(String::from("event-1"))),
            timestamp_nanos: signal_mentci::TimestampNanos(1_700_000_000_000_000_003),
            receipt_grade: ReceiptGrade::IngressAccepted,
        }),
        Response::OperationUnavailable(OperationFailure {
            request_identifier: signal_mentci::RequestIdentifier(String::from(
                "request-conversation-2",
            )),
            unavailability: Unavailability::PersonaUnavailable,
        }),
    ]
}

#[test]
fn every_canonical_query_restores_from_fresh_peer_bytes() {
    for query in canonical_queries() {
        let received = Signal::<Query>::from(query.signalize().expect("archive").bytes().to_vec());
        assert_eq!(received.restore().expect("restore"), query);
    }
}

#[test]
fn every_canonical_response_restores_from_fresh_peer_bytes() {
    for response in canonical_responses() {
        let received =
            Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
        assert_eq!(received.restore().expect("restore"), response);
    }
}

#[test]
fn a_malformed_archive_is_refused() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
    assert!(Signal::<Response>::from(vec![1, 2, 3]).restore().is_err());
}

#[test]
fn an_empty_notification_slice_is_a_bare_tag() {
    let response = Response::InterfaceObservationStarted(InterfaceObservationOpened {
        subscription_token: signal_mentci::SubscriptionToken(String::from("token-1")),
        projected_interface_state: ProjectedInterfaceState {
            revision_counter: signal_mentci::RevisionCounter(1),
            interface_projection: InterfaceProjection::NotificationProjection(
                NotificationSlice::Empty,
            ),
        },
    });
    let received =
        Signal::<Response>::from(response.signalize().expect("archive").bytes().to_vec());
    assert_eq!(received.restore().expect("restore"), response);
}

#[cfg(feature = "datom")]
mod canonical {
    use super::{canonical_queries, canonical_responses};
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Compactable, Protosizable, ReaderBudget};
    use signal_mentci::{Query, Response};

    const CANONICAL: &str = include_str!("../examples/canonical.datom");

    fn budget() -> Budget {
        Budget {
            remaining: 1 << 20,
            reader: ReaderBudget::try_from(1 << 20).expect("bounded reader budget"),
            depth: 0,
            maximum_depth: 256,
        }
    }

    fn lines() -> Vec<String> {
        CANONICAL
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with(';'))
            .map(str::to_string)
            .collect()
    }

    /// Rewrite `examples/canonical.datom` from the canonical values. The file
    /// is the codec's product, never text spelled by hand; run this whenever
    /// the canonical values change:
    ///
    /// ```text
    /// cargo test --features datom -- --ignored rewrite_the_canonical_file
    /// ```
    #[test]
    #[ignore = "writes the source tree; run deliberately when the values change"]
    fn rewrite_the_canonical_file() {
        let mut out = String::from("; Canonical Datom examples for signal-mentci.\n");
        out.push_str("; Written by `rewrite_the_canonical_file`; never spelled by hand.\n\n");
        for query in canonical_queries() {
            out.push_str(&query.datomize(vec![]).protosize().compact());
            out.push('\n');
        }
        for response in canonical_responses() {
            out.push_str(&response.datomize(vec![]).protosize().compact());
            out.push('\n');
        }
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/canonical.datom"),
            out,
        )
        .expect("write canonical");
    }

    // Fixture-completeness repair during the 0.33 migration: baseline ed750b
    // already constructs 15 queries + 13 responses, but its file contained
    // only the first 11 queries + first 9 responses. Preserve those 20 lines
    // in order and include the existing roster/conversation/psyche values.
    // The eight added lines were emitted by the historical ed750b producer;
    // exact current compact output remains asserted below, without a bypass.
    /// The canonical file is the codec's own text for the canonical values.
    #[test]
    fn the_canonical_file_is_what_the_codec_writes() {
        let mut written: Vec<String> = Vec::new();
        for query in canonical_queries() {
            written.push(query.datomize(vec![]).protosize().compact());
        }
        for response in canonical_responses() {
            written.push(response.datomize(vec![]).protosize().compact());
        }
        assert_eq!(lines(), written);
    }

    /// Every line the file carries actualizes — as a request or as a reply,
    /// never as neither and never as both.
    #[test]
    fn every_canonical_line_actualizes_into_exactly_one_root() {
        let lines = lines();
        assert!(!lines.is_empty());
        for line in lines {
            let query = Potential::<Query>::from(line.clone())
                .actualize(&mut budget())
                .ok();
            let response = Potential::<Response>::from(line.clone())
                .actualize(&mut budget())
                .ok();
            match (query, response) {
                (Some(query), None) => {
                    assert_eq!(query.datomize(vec![]).protosize().compact(), line)
                }
                (None, Some(response)) => {
                    assert_eq!(response.datomize(vec![]).protosize().compact(), line)
                }
                (Some(_), Some(_)) => panic!("ambiguous canonical line: {line}"),
                (None, None) => panic!("unreadable canonical line: {line}"),
            }
        }
    }
}
