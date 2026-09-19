//! The contract's own witness: every canonical value survives the wire, and
//! every canonical Datom line is the codec's own text for one of them.

use signal_criome::{
    ExpiryAction, InterceptPolicy, InterceptPolicyCancellation, InterceptPolicyProposal,
    InterceptPolicyWindow, InterceptTargetSelector, ParkedRequestAnswer, ParkedRequestDecision,
    ParkedRequestQuery, PolicyOverlapMode,
};
use signal_mentci::{
    AnswerProposal, AnswerProposalAdmitted, ApprovalDecision, ApprovalQuestion, ApprovalSource,
    ApprovalVerdict, ByteViewable, CriomeAccess, InterceptPolicyObservation, InterfaceInterest,
    InterfaceMutation, InterfaceObservationOpened, InterfaceObservationRetracted,
    InterfaceProjection, InterfaceState, InterfaceStateObservation, InterfaceUpdate,
    NotificationSlice, PaneContent, ProjectedInterfaceState, Query, QuestionContext,
    QuestionPresented, QuestionProposal, Rejection, RejectionReason, Response, Restorable, Signal,
    Signalizable, UpdateAccepted, VerdictAccepted, RosterObservation, ConversationObservation,
    PsycheSubmission, RosterSnapshot, FlowSnapshot, FlowState, ActivitySource, SourceStatus,
    ConversationSnapshot, ConversationEntry, SourceKind, Provenance, SubmissionReceipt,
    SubmissionDisposition, ReceiptGrade, OperationFailure, Unavailability,
};

fn question_proposal() -> QuestionProposal {
    QuestionProposal {
        approval_source: ApprovalSource::CriomeEscalation(String::from("slot-1")),
        prompt_text: String::from("approve the release?"),
        answer_text: String::from("yes"),
        explanation_text: String::from("the quorum is short by one"),
        question_context: QuestionContext {
            context_label: String::from("release"),
            context_body: String::from("criome 1.0.0"),
        },
    }
}

fn pane_content() -> PaneContent {
    PaneContent {
        pane_label: String::from("main"),
        context_body: String::from("waiting on a verdict"),
    }
}

fn policy() -> InterceptPolicy {
    InterceptPolicy {
        intercept_policy_identifier: String::from("policy-1"),
        mentci_session_slot: String::from("session-1"),
        intercept_target_selector: InterceptTargetSelector {
            spirit_process_key: String::from("spirit-1"),
        },
        spirit_operation_names: vec![String::from("Deploy"), String::from("Retire")],
        intercept_policy_window: InterceptPolicyWindow {
            first_timestamp_nanos: 1_700_000_000_000_000_000,
            second_timestamp_nanos: 1_700_000_060_000_000_000,
        },
        expiry_action: ExpiryAction::LeaveParked,
        policy_priority: 3,
    }
}

fn canonical_queries() -> Vec<Query> {
    vec![
        Query::ListInterceptPolicies(InterceptPolicyObservation {}),
        Query::RetractInterfaceObservation(String::from("token-1")),
        Query::ObserveInterfaceState(InterfaceStateObservation {
            subscriber_name: String::from("operator"),
            interface_interest: InterfaceInterest::FullInterfaceState,
        }),
        Query::PushUpdate(InterfaceUpdate {
            update_identifier: String::from("update-1"),
            interface_mutation: InterfaceMutation::SetPaneContent(pane_content()),
        }),
        Query::PresentQuestion(question_proposal()),
        Query::ProposeEditedAnswer(AnswerProposal {
            question_identifier: String::from("question-1"),
            answer_text: String::from("yes, with the second signature"),
            subscriber_name: String::from("operator"),
        }),
        Query::AnswerQuestion(ApprovalVerdict {
            question_identifier: String::from("question-1"),
            approval_decision: ApprovalDecision::ApproveSuggestedAnswer,
            subscriber_name: String::from("operator"),
        }),
        Query::CreateInterceptPolicy(InterceptPolicyProposal {
            mentci_session_slot: String::from("session-1"),
            intercept_target_selector: InterceptTargetSelector {
                spirit_process_key: String::from("spirit-1"),
            },
            spirit_operation_names: vec![String::from("Deploy")],
            policy_duration_nanos: 60_000_000_000,
            expiry_action: ExpiryAction::AutoReject,
            policy_priority: 3,
            policy_overlap_mode: PolicyOverlapMode::RejectSamePriorityOverlap,
        }),
        Query::CancelInterceptPolicy(InterceptPolicyCancellation {
            intercept_policy_identifier: String::from("policy-1"),
        }),
        Query::FetchParkedRequests(ParkedRequestQuery {
            mentci_session_slot_option: Some(String::from("session-1")),
            intercept_target_selector_option: None,
        }),
        Query::AnswerParkedRequest(ParkedRequestAnswer {
            parked_request_identifier: String::from("parked-1"),
            parked_request_decision: ParkedRequestDecision::Approve,
        }),
        Query::ObserveRoster(RosterObservation {
            request_identifier: String::from("request-roster-1"),
        }),
        Query::ObserveConversation(ConversationObservation {
            request_identifier: String::from("request-conversation-1"),
            flow_identifier: String::from("flow-1"),
        }),
        Query::SubmitPsyche(PsycheSubmission {
            request_identifier: String::from("request-send-1"),
            flow_identifier: String::from("flow-1"),
            psyche_text: String::from("  preserve this text exactly  "),
        }),
    ]
}

fn canonical_responses() -> Vec<Response> {
    vec![
        Response::RequestRejected(Rejection {
            rejection_reason: RejectionReason::QuestionAlreadyAnswered,
        }),
        Response::UpdateApplied(UpdateAccepted {
            update_identifier: String::from("update-1"),
            revision_counter: 12,
        }),
        Response::QuestionAccepted(QuestionPresented {
            question_identifier: String::from("question-1"),
            revision_counter: 13,
            timestamp_nanos: 1_700_000_000_000_000_000,
        }),
        Response::VerdictRecorded(VerdictAccepted {
            question_identifier: String::from("question-1"),
            approval_decision: ApprovalDecision::Defer,
            timestamp_nanos: 1_700_000_000_000_000_001,
        }),
        Response::AnswerProposalAccepted(AnswerProposalAdmitted {
            proposal_identifier: String::from("proposal-1"),
            question_identifier: String::from("question-1"),
            proposal_digest: String::from("d1ge57"),
            revision_counter: 14,
        }),
        Response::InterfaceObservationStarted(InterfaceObservationOpened {
            subscription_token: String::from("token-1"),
            projected_interface_state: ProjectedInterfaceState {
                revision_counter: 15,
                interface_projection: InterfaceProjection::FullProjection(InterfaceState {
                    revision_counter: 15,
                    status_text: String::from("idle"),
                    notification_text: String::from("one question waiting"),
                    pane_content: pane_content(),
                    approval_question: ApprovalQuestion {
                        question_identifier: String::from("question-1"),
                        question_proposal: question_proposal(),
                    },
                    criome_access: CriomeAccess::ReadWrite,
                }),
            },
        }),
        Response::InterfaceObservationClosed(InterfaceObservationRetracted {
            subscription_token: String::from("token-1"),
        }),
        Response::InterceptPolicyCreated(policy()),
        Response::InterceptPolicyCancelled(String::from("policy-1")),
        Response::RosterObserved(RosterSnapshot {
            request_identifier: String::from("request-roster-1"),
            timestamp_nanos: 1_700_000_000_000_000_002,
            source_status: SourceStatus::Observed,
            flows: vec![FlowSnapshot {
                flow_identifier: String::from("flow-1"),
                flow_name: String::from("Fixture Flow"),
                seat_label: String::from("fixture"),
                flow_state: FlowState::Idle,
                timestamp_nanos_option: Some(1_700_000_000_000_000_001),
                activity_source: ActivitySource::Herdr,
            }],
        }),
        Response::ConversationObserved(ConversationSnapshot {
            request_identifier: String::from("request-conversation-1"),
            flow_identifier: String::from("flow-1"),
            timestamp_nanos: 1_700_000_000_000_000_002,
            source_status: SourceStatus::Partial,
            entries: vec![ConversationEntry {
                entry_identifier: String::from("synthetic:flow-1:1"),
                sequence: 1,
                source_ordinal: 1,
                timestamp_nanos: 1_700_000_000_000_000_000,
                entry_text: String::from("Origin is unresolved."),
                source_kind: SourceKind::UserInput,
                provenance: Provenance::Unknown,
                attributed_actor_option: None,
            }, ConversationEntry {
                entry_identifier: String::from("synthetic:flow-1:2"),
                sequence: 2,
                source_ordinal: 2,
                timestamp_nanos: 1_700_000_000_000_000_001,
                entry_text: String::from("A final machine response."),
                source_kind: SourceKind::FinalResponse,
                provenance: Provenance::Machine,
                attributed_actor_option: Some(String::from("Fixture Flow")),
            }],
        }),
        Response::PsycheSubmitted(SubmissionReceipt {
            request_identifier: String::from("request-send-1"),
            submission_disposition: SubmissionDisposition::Accepted,
            submission_reason_option: None,
            relay_identifier_option: Some(String::from("relay-1")),
            event_identifier_option: Some(String::from("event-1")),
            timestamp_nanos: 1_700_000_000_000_000_003,
            receipt_grade: ReceiptGrade::IngressAccepted,
        }),
        Response::OperationUnavailable(OperationFailure {
            request_identifier: String::from("request-conversation-2"),
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
        subscription_token: String::from("token-1"),
        projected_interface_state: ProjectedInterfaceState {
            revision_counter: 1,
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
    use protos::{Protosizable, ReaderBudget, Textualizable};
    use signal_mentci::{Query, Response};

    const CANONICAL: &str = include_str!("../examples/canonical.datom");

    fn budget() -> Budget {
        Budget {
            remaining: 1 << 20,
            reader: ReaderBudget { remaining: 1 << 20 },
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
            out.push_str(&query.datomize(vec![]).protosize().textualize());
            out.push('\n');
        }
        for response in canonical_responses() {
            out.push_str(&response.datomize(vec![]).protosize().textualize());
            out.push('\n');
        }
        std::fs::write(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/canonical.datom"),
            out,
        )
        .expect("write canonical");
    }

    /// The canonical file is the codec's own text for the canonical values.
    #[test]
    fn the_canonical_file_is_what_the_codec_writes() {
        let mut written: Vec<String> = Vec::new();
        for query in canonical_queries() {
            written.push(query.datomize(vec![]).protosize().textualize());
        }
        for response in canonical_responses() {
            written.push(response.datomize(vec![]).protosize().textualize());
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
                    assert_eq!(query.datomize(vec![]).protosize().textualize(), line)
                }
                (None, Some(response)) => {
                    assert_eq!(response.datomize(vec![]).protosize().textualize(), line)
                }
                (Some(_), Some(_)) => panic!("ambiguous canonical line: {line}"),
                (None, None) => panic!("unreadable canonical line: {line}"),
            }
        }
    }
}
