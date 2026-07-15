use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_criome::{
    ActiveInterceptPolicies, ApprovalAuditSource, ExpiryAction, InterceptPolicy,
    InterceptPolicyCancellation, InterceptPolicyIdentifier, InterceptPolicyProposal,
    InterceptPolicyWindow, InterceptTargetSelector, MentciSessionSlot, ParkedRequestAnswer,
    ParkedRequestDecision, ParkedRequestIdentifier, ParkedRequestOutcome, ParkedRequestQuery,
    ParkedRequestResolution, ParkedRequestSnapshot, ParkedSpiritRequest, PolicyDurationNanos,
    PolicyOverlapMode, PolicyPriority, RawSpiritOperationPayload, SpiritAuthorizationContext,
    SpiritOperationName, SpiritOperationNames, SpiritProcessKey,
    TimestampNanos as CriomeTimestampNanos,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};
use signal_mentci::{
    AnswerProposal, AnswerProposalAdmitted, AnswerText, ApprovalDecision, ApprovalQuestion,
    ApprovalSource, ApprovalVerdict, AuthorizationRequestSlot, ContextBody, ContextLabel,
    CriomeAccess, ExplanationText, InterceptPolicyObservation, InterfaceInterest,
    InterfaceMutation, InterfaceObservationOpened, InterfaceObservationRetracted,
    InterfaceProjection, InterfaceState, InterfaceStateObservation, MentciFrame as Frame,
    MentciFrameBody as FrameBody, MentciReply, MentciRequest, NotificationText, PaneContent,
    PaneLabel, PendingQuestionsView, ProjectedInterfaceState, PromptText, ProposalDigest,
    ProposalIdentifier, QuestionContext, QuestionIdentifier, QuestionPresented, QuestionProposal,
    Rejection, RejectionReason, RevisionCounter, StatusText, SubscriberName, SubscriptionToken,
    TimestampNanos, UpdateAccepted, UpdateIdentifier, VerdictAccepted,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn question_proposal() -> QuestionProposal {
    QuestionProposal::new(
        ApprovalSource::CriomeEscalation(AuthorizationRequestSlot::new("slot-1")),
        PromptText::new("approve-spirit-record"),
        Some(AnswerText::new("approve")),
        ExplanationText::new("agent-proposed-answer"),
        vec![QuestionContext {
            context_label: ContextLabel::new("record"),
            context_body: ContextBody::new("content-addressed-preimage"),
        }],
    )
}

fn approval_question() -> ApprovalQuestion {
    ApprovalQuestion {
        question_identifier: QuestionIdentifier::new("question-1"),
        question_proposal: question_proposal(),
    }
}

fn projected_state() -> ProjectedInterfaceState {
    ProjectedInterfaceState {
        revision_counter: RevisionCounter::new(2),
        interface_projection: InterfaceProjection::PendingQuestionsProjection(
            PendingQuestionsView::from_questions(vec![approval_question()]),
        ),
    }
}

fn mentci_session_slot() -> MentciSessionSlot {
    MentciSessionSlot::new("mentci-session-1")
}

fn policy_identifier() -> InterceptPolicyIdentifier {
    InterceptPolicyIdentifier::new("intercept-policy-1")
}

fn parked_request_identifier() -> ParkedRequestIdentifier {
    ParkedRequestIdentifier::new("parked-request-1")
}

fn process_key() -> SpiritProcessKey {
    SpiritProcessKey::new("spirit-process-main")
}

fn intercept_target() -> InterceptTargetSelector {
    InterceptTargetSelector::new(process_key())
}

fn operation_names() -> SpiritOperationNames {
    SpiritOperationNames::from_names(vec![SpiritOperationName::new("Record")])
}

fn policy_proposal() -> InterceptPolicyProposal {
    InterceptPolicyProposal {
        mentci_session_slot: mentci_session_slot(),
        intercept_target_selector: intercept_target(),
        spirit_operation_names: operation_names(),
        policy_duration_nanos: PolicyDurationNanos::new(100),
        expiry_action: ExpiryAction::AutoApprove,
        policy_priority: PolicyPriority::new(10),
        policy_overlap_mode: PolicyOverlapMode::RejectSamePriorityOverlap,
    }
}

fn policy() -> InterceptPolicy {
    InterceptPolicy {
        intercept_policy_identifier: policy_identifier(),
        mentci_session_slot: mentci_session_slot(),
        intercept_target_selector: intercept_target(),
        spirit_operation_names: operation_names(),
        intercept_policy_window: InterceptPolicyWindow {
            starts_at: CriomeTimestampNanos::new(20),
            expires_at: CriomeTimestampNanos::new(120),
        },
        expiry_action: ExpiryAction::AutoApprove,
        policy_priority: PolicyPriority::new(10),
    }
}

fn parked_request_snapshot() -> ParkedRequestSnapshot {
    ParkedRequestSnapshot::from_requests(vec![ParkedSpiritRequest {
        parked_request_identifier: parked_request_identifier(),
        intercept_policy_identifier: policy_identifier(),
        mentci_session_slot: mentci_session_slot(),
        spirit_authorization_context: SpiritAuthorizationContext {
            spirit_operation_name: SpiritOperationName::new("Record"),
            raw_spirit_operation_payload: RawSpiritOperationPayload::new("(Record (...))"),
            spirit_process_key: process_key(),
        },
        parked_at: CriomeTimestampNanos::new(25),
        expires_at: CriomeTimestampNanos::new(120),
        expiry_action: ExpiryAction::AutoApprove,
    }])
}

fn resolution() -> ParkedRequestResolution {
    ParkedRequestResolution {
        parked_request_identifier: parked_request_identifier(),
        intercept_policy_identifier: policy_identifier(),
        parked_request_outcome: ParkedRequestOutcome::Approved,
        approval_audit_source: ApprovalAuditSource::Manual,
        timestamp_nanos: CriomeTimestampNanos::new(30),
    }
}

fn assert_nota_round_trip<Value>(value: &Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(&recovered, value);
}

fn assert_request_frame(request: MentciRequest) {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let decoded = Frame::decode_length_prefixed(&frame.encode_length_prefixed().expect("encode"))
        .expect("decode");
    match decoded.into_body() {
        FrameBody::Request {
            request: decoded, ..
        } => assert_eq!(decoded.payloads().head(), &request),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn assert_reply_frame(reply: MentciReply) {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let decoded = Frame::decode_length_prefixed(&frame.encode_length_prefixed().expect("encode"))
        .expect("decode");
    match decoded.into_body() {
        FrameBody::Reply {
            reply: Reply::Accepted { per_operation, .. },
            ..
        } => {
            assert_eq!(per_operation.into_head(), SubReply::Ok(reply));
        }
        other => panic!("expected accepted reply frame, got {other:?}"),
    }
}

#[test]
fn every_operation_round_trips_through_nota_and_exchange_frame() {
    let requests = [
        MentciRequest::PresentQuestion(question_proposal()),
        MentciRequest::PushUpdate(signal_mentci::InterfaceUpdate {
            update_identifier: UpdateIdentifier::new("update-1"),
            interface_mutation: InterfaceMutation::SetStatus(StatusText::new("waiting")),
        }),
        MentciRequest::ObserveInterfaceState(InterfaceStateObservation {
            subscriber_name: SubscriberName::new("status-bar"),
            interface_interest: InterfaceInterest::StatusOnly,
        }),
        MentciRequest::AnswerQuestion(ApprovalVerdict {
            question_identifier: QuestionIdentifier::new("question-1"),
            approval_decision: ApprovalDecision::ApproveSuggestedAnswer,
            subscriber_name: SubscriberName::new("psyche"),
        }),
        MentciRequest::ProposeEditedAnswer(AnswerProposal {
            question_identifier: QuestionIdentifier::new("question-1"),
            answer_text: AnswerText::new("replacement"),
            subscriber_name: SubscriberName::new("psyche"),
        }),
        MentciRequest::CreateInterceptPolicy(policy_proposal()),
        MentciRequest::ReplaceInterceptPolicy(policy_proposal()),
        MentciRequest::CancelInterceptPolicy(InterceptPolicyCancellation::new(policy_identifier())),
        MentciRequest::ListInterceptPolicies(InterceptPolicyObservation::new()),
        MentciRequest::FetchParkedRequests(ParkedRequestQuery {
            optional_mentci_session_slot: Some(mentci_session_slot()),
            optional_intercept_target_selector: Some(intercept_target()),
        }),
        MentciRequest::AnswerParkedRequest(ParkedRequestAnswer {
            parked_request_identifier: parked_request_identifier(),
            parked_request_decision: ParkedRequestDecision::Approve,
        }),
        MentciRequest::RetractInterfaceObservation(SubscriptionToken::new("subscription-1")),
    ];
    for request in requests {
        assert_request_frame(request.clone());
        assert_nota_round_trip(&request);
    }
}

#[test]
fn every_reply_round_trips_through_nota_and_exchange_frame() {
    let replies = [
        MentciReply::QuestionAccepted(QuestionPresented {
            question_identifier: QuestionIdentifier::new("question-1"),
            revision_counter: RevisionCounter::new(1),
            timestamp_nanos: TimestampNanos::new(10),
        }),
        MentciReply::UpdateApplied(UpdateAccepted {
            update_identifier: UpdateIdentifier::new("update-1"),
            revision_counter: RevisionCounter::new(2),
        }),
        MentciReply::InterfaceObservationStarted(InterfaceObservationOpened {
            subscription_token: SubscriptionToken::new("subscription-1"),
            projected_interface_state: projected_state(),
        }),
        MentciReply::VerdictRecorded(VerdictAccepted {
            question_identifier: QuestionIdentifier::new("question-1"),
            approval_decision: ApprovalDecision::Reject,
            timestamp_nanos: TimestampNanos::new(11),
        }),
        MentciReply::AnswerProposalAccepted(AnswerProposalAdmitted {
            proposal_identifier: ProposalIdentifier::new("proposal-1"),
            question_identifier: QuestionIdentifier::new("question-1"),
            proposal_digest: ProposalDigest::new("proposal-digest-1"),
            revision_counter: RevisionCounter::new(3),
        }),
        MentciReply::InterceptPolicyCreated(policy()),
        MentciReply::InterceptPolicyReplaced(policy()),
        MentciReply::InterceptPolicyCancelled(policy_identifier()),
        MentciReply::InterceptPoliciesListed(ActiveInterceptPolicies::from_policies(
            vec![policy()],
        )),
        MentciReply::ParkedRequestsFetched(parked_request_snapshot()),
        MentciReply::ParkedRequestAnswered(resolution()),
        MentciReply::InterfaceObservationClosed(InterfaceObservationRetracted::new(
            SubscriptionToken::new("subscription-1"),
        )),
        MentciReply::RequestRejected(Rejection::new(RejectionReason::UnknownQuestion)),
    ];
    for reply in replies {
        assert_reply_frame(reply.clone());
        assert_nota_round_trip(&reply);
    }
}

#[test]
fn cross_contract_criome_slot_survives_nota() {
    let proposal = question_proposal();
    assert_eq!(
        proposal
            .approval_source
            .criome_slot()
            .map(AuthorizationRequestSlot::as_str),
        Some("slot-1"),
    );
    let recovered: QuestionProposal = NotaSource::new(&proposal.to_nota())
        .parse()
        .expect("decode");
    assert_eq!(
        recovered
            .approval_source
            .criome_slot()
            .map(AuthorizationRequestSlot::as_str),
        Some("slot-1"),
    );
}

#[test]
fn full_projection_preserves_criome_access() {
    let state = ProjectedInterfaceState {
        revision_counter: RevisionCounter::new(5),
        interface_projection: InterfaceProjection::FullProjection(InterfaceState::new(
            RevisionCounter::new(5),
            StatusText::new("waiting"),
            Some(NotificationText::new("new-question")),
            vec![PaneContent {
                pane_label: PaneLabel::new("approval"),
                context_body: ContextBody::new("question-context"),
            }],
            vec![approval_question()],
            CriomeAccess::ReadWrite,
        )),
    };
    assert_nota_round_trip(&state);
    assert_eq!(state.criome_access(), Some(CriomeAccess::ReadWrite));
}
