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
    InterfaceProjection, InterfaceState, InterfaceStateObservation, MentciEvent,
    MentciFrame as Frame, MentciFrameBody as FrameBody, MentciReply, MentciRequest,
    NotificationText, PaneContent, PaneLabel, PendingQuestionsView, ProjectedInterfaceState,
    PromptText, ProposalDigest, ProposalIdentifier, QuestionContext, QuestionIdentifier,
    QuestionPresented, QuestionProposal, Rejection, RejectionReason, RevisionCounter, StatusText,
    SubscriberName, SubscriptionToken, TimestampNanos, UpdateAccepted, UpdateIdentifier,
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

fn intercept_policy_identifier() -> InterceptPolicyIdentifier {
    InterceptPolicyIdentifier::new("intercept-policy-1")
}

fn parked_request_identifier() -> ParkedRequestIdentifier {
    ParkedRequestIdentifier::new("parked-request-1")
}

fn spirit_process_key() -> SpiritProcessKey {
    SpiritProcessKey::new("spirit-process-main")
}

fn intercept_target() -> InterceptTargetSelector {
    InterceptTargetSelector::new(spirit_process_key())
}

fn spirit_operation_names() -> SpiritOperationNames {
    SpiritOperationNames::from_names(vec![SpiritOperationName::new("Record")])
}

fn intercept_policy_proposal() -> InterceptPolicyProposal {
    InterceptPolicyProposal {
        mentci_session_slot: mentci_session_slot(),
        intercept_target_selector: intercept_target(),
        spirit_operation_names: spirit_operation_names(),
        policy_duration_nanos: PolicyDurationNanos::new(100),
        expiry_action: ExpiryAction::AutoApprove,
        policy_priority: PolicyPriority::new(10),
        policy_overlap_mode: PolicyOverlapMode::RejectSamePriorityOverlap,
    }
}

fn intercept_policy() -> InterceptPolicy {
    InterceptPolicy {
        intercept_policy_identifier: intercept_policy_identifier(),
        mentci_session_slot: mentci_session_slot(),
        intercept_target_selector: intercept_target(),
        spirit_operation_names: spirit_operation_names(),
        intercept_policy_window: InterceptPolicyWindow {
            starts_at: CriomeTimestampNanos::new(20),
            expires_at: CriomeTimestampNanos::new(120),
        },
        expiry_action: ExpiryAction::AutoApprove,
        policy_priority: PolicyPriority::new(10),
    }
}

fn active_intercept_policies() -> ActiveInterceptPolicies {
    ActiveInterceptPolicies::from_policies(vec![intercept_policy()])
}

fn parked_request_query() -> ParkedRequestQuery {
    ParkedRequestQuery {
        optional_mentci_session_slot: Some(mentci_session_slot()),
        optional_intercept_target_selector: Some(intercept_target()),
    }
}

fn parked_request_answer() -> ParkedRequestAnswer {
    ParkedRequestAnswer {
        parked_request_identifier: parked_request_identifier(),
        parked_request_decision: ParkedRequestDecision::Approve,
    }
}

fn parked_spirit_request() -> ParkedSpiritRequest {
    ParkedSpiritRequest {
        parked_request_identifier: parked_request_identifier(),
        intercept_policy_identifier: intercept_policy_identifier(),
        mentci_session_slot: mentci_session_slot(),
        spirit_authorization_context: SpiritAuthorizationContext {
            spirit_operation_name: SpiritOperationName::new("Record"),
            raw_spirit_operation_payload: RawSpiritOperationPayload::new("(Record (...))"),
            spirit_process_key: spirit_process_key(),
        },
        parked_at: CriomeTimestampNanos::new(25),
        expires_at: CriomeTimestampNanos::new(120),
        expiry_action: ExpiryAction::AutoApprove,
    }
}

fn parked_request_snapshot() -> ParkedRequestSnapshot {
    ParkedRequestSnapshot::from_requests(vec![parked_spirit_request()])
}

fn parked_request_resolution() -> ParkedRequestResolution {
    ParkedRequestResolution {
        parked_request_identifier: parked_request_identifier(),
        intercept_policy_identifier: intercept_policy_identifier(),
        parked_request_outcome: ParkedRequestOutcome::Approved,
        approval_audit_source: ApprovalAuditSource::Manual,
        timestamp_nanos: CriomeTimestampNanos::new(30),
    }
}

fn assert_request_round_trips(request: MentciRequest) {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: request.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode request");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode request");
    match decoded.into_body() {
        FrameBody::Request {
            request: decoded_request,
            ..
        } => assert_eq!(decoded_request.payloads().head(), &request),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn assert_reply_round_trips(reply: MentciReply) {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");
    match decoded.into_body() {
        FrameBody::Reply {
            reply: decoded_reply,
            ..
        } => match decoded_reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => assert_eq!(payload, reply),
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            Reply::Rejected { reason } => panic!("unexpected rejected reply: {reason:?}"),
        },
        other => panic!("expected reply frame, got {other:?}"),
    }
}

fn assert_nota_round_trips<Value>(value: &Value)
where
    Value: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let text = value.to_nota();
    let recovered = NotaSource::new(&text).parse::<Value>().expect("decode");
    assert_eq!(&recovered, value);
}

#[test]
fn request_variants_round_trip() {
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
            answer_text: AnswerText::new("replacement-nota-object"),
            subscriber_name: SubscriberName::new("psyche"),
        }),
        MentciRequest::CreateInterceptPolicy(intercept_policy_proposal()),
        MentciRequest::ReplaceInterceptPolicy(intercept_policy_proposal()),
        MentciRequest::CancelInterceptPolicy(InterceptPolicyCancellation::new(
            intercept_policy_identifier(),
        )),
        MentciRequest::ListInterceptPolicies(InterceptPolicyObservation::new()),
        MentciRequest::FetchParkedRequests(parked_request_query()),
        MentciRequest::AnswerParkedRequest(parked_request_answer()),
        MentciRequest::RetractInterfaceObservation(SubscriptionToken::new("subscription-1")),
    ];
    for request in requests {
        assert_request_round_trips(request.clone());
        assert_nota_round_trips(&request);
    }
}

#[test]
fn reply_variants_round_trip() {
    let replies = [
        MentciReply::QuestionPresented(QuestionPresented {
            question_identifier: QuestionIdentifier::new("question-1"),
            revision_counter: RevisionCounter::new(1),
            timestamp_nanos: TimestampNanos::new(10),
        }),
        MentciReply::UpdateAccepted(UpdateAccepted {
            update_identifier: UpdateIdentifier::new("update-1"),
            revision_counter: RevisionCounter::new(2),
        }),
        MentciReply::InterfaceObservationOpened(InterfaceObservationOpened {
            subscription_token: SubscriptionToken::new("subscription-1"),
            projected_interface_state: projected_state(),
        }),
        MentciReply::VerdictAccepted(signal_mentci::VerdictAccepted {
            question_identifier: QuestionIdentifier::new("question-1"),
            approval_decision: ApprovalDecision::Reject,
            timestamp_nanos: TimestampNanos::new(11),
        }),
        MentciReply::AnswerProposalAdmitted(AnswerProposalAdmitted {
            proposal_identifier: ProposalIdentifier::new("proposal-1"),
            question_identifier: QuestionIdentifier::new("question-1"),
            proposal_digest: ProposalDigest::new("proposal-digest-1"),
            revision_counter: RevisionCounter::new(3),
        }),
        MentciReply::InterceptPolicyCreated(intercept_policy()),
        MentciReply::InterceptPolicyReplaced(intercept_policy()),
        MentciReply::InterceptPolicyCancelled(intercept_policy_identifier()),
        MentciReply::InterceptPoliciesListed(active_intercept_policies()),
        MentciReply::ParkedRequestsFetched(parked_request_snapshot()),
        MentciReply::ParkedRequestAnswered(parked_request_resolution()),
        MentciReply::InterfaceObservationRetracted(InterfaceObservationRetracted::new(
            SubscriptionToken::new("subscription-1"),
        )),
        MentciReply::Rejection(Rejection::new(RejectionReason::UnknownQuestion)),
    ];
    for reply in replies {
        assert_reply_round_trips(reply.clone());
        assert_nota_round_trips(&reply);
    }
}

#[test]
fn event_round_trips() {
    let event = MentciEvent::InterfaceStateChanged(projected_state());
    assert_nota_round_trips(&event);
}

#[test]
fn closed_verdict_has_no_authored_answer_variant() {
    for decision in [
        ApprovalDecision::ApproveSuggestedAnswer,
        ApprovalDecision::Reject,
        ApprovalDecision::Defer,
    ] {
        assert_nota_round_trips(&decision);
    }
}

#[test]
fn projected_state_can_hide_full_question_context() {
    let status_projection = ProjectedInterfaceState {
        revision_counter: RevisionCounter::new(4),
        interface_projection: InterfaceProjection::StatusProjection(StatusText::new("waiting")),
    };
    assert_nota_round_trips(&status_projection);

    let full_projection = ProjectedInterfaceState {
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
    assert_nota_round_trips(&full_projection);
}

#[test]
fn criome_escalation_source_carries_the_slot() {
    // The seam: a criome-sourced question keeps its parked slot, typed, so a
    // client answering it routes the verdict back to criome by that slot.
    let proposal = question_proposal();
    assert_eq!(
        proposal
            .approval_source
            .criome_slot()
            .map(AuthorizationRequestSlot::as_str),
        Some("slot-1"),
    );

    // The slot survives the NOTA round trip.
    let recovered: QuestionProposal = NotaSource::new(&proposal.to_nota())
        .parse()
        .expect("decode proposal");
    assert_eq!(
        recovered
            .approval_source
            .criome_slot()
            .map(AuthorizationRequestSlot::as_str),
        Some("slot-1"),
    );

    // Non-criome sources carry no slot.
    let agent = QuestionProposal::new(
        ApprovalSource::AgentQuestion,
        PromptText::new("ask"),
        None,
        ExplanationText::new("local"),
        vec![],
    );
    assert!(agent.approval_source.criome_slot().is_none());
}

#[test]
fn criome_interception_source_carries_the_parked_request_identifier() {
    let proposal = QuestionProposal::new(
        ApprovalSource::CriomeInterception(parked_request_identifier()),
        PromptText::new("approve-intercepted-spirit-operation"),
        Some(AnswerText::new("approve")),
        ExplanationText::new("raw-payload-visible"),
        vec![QuestionContext {
            context_label: ContextLabel::new("raw-spirit-operation"),
            context_body: ContextBody::new("(Record (...))"),
        }],
    );

    assert_eq!(
        proposal
            .approval_source
            .parked_request()
            .map(ParkedRequestIdentifier::as_str),
        Some("parked-request-1"),
    );
    assert!(proposal.approval_source.criome_slot().is_none());
    assert_nota_round_trips(&proposal);
}
