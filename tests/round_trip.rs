use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, RequestPayload, SessionEpoch,
    SubReply,
};
use signal_mentci::{
    AnswerProposal, AnswerProposalAdmitted, AnswerText, ApprovalDecision, ApprovalQuestion,
    ApprovalSource, ApprovalVerdict, AuthorizationRequestSlot, ContextBody, ContextLabel,
    CriomeAccess, ExplanationText, InterfaceInterest, InterfaceMutation,
    InterfaceObservationOpened, InterfaceObservationRetracted, InterfaceProjection, InterfaceState,
    InterfaceStateObservation, MentciEvent, MentciFrame as Frame, MentciFrameBody as FrameBody,
    MentciReply, MentciRequest, NotificationText, PaneContent, PaneLabel, PendingQuestionsView,
    ProjectedInterfaceState, PromptText, ProposalDigest, ProposalIdentifier, QuestionContext,
    QuestionIdentifier, QuestionPresented, QuestionProposal, Rejection, RejectionReason,
    RevisionCounter, StatusText, SubscriberName, SubscriptionToken, TimestampNanos, UpdateAccepted,
    UpdateIdentifier,
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
            label: ContextLabel::new("record"),
            body: ContextBody::new("content-addressed-preimage"),
        }],
    )
}

fn approval_question() -> ApprovalQuestion {
    ApprovalQuestion {
        identifier: QuestionIdentifier::new("question-1"),
        proposal: question_proposal(),
    }
}

fn projected_state() -> ProjectedInterfaceState {
    ProjectedInterfaceState {
        revision: RevisionCounter::new(2),
        projection: InterfaceProjection::PendingQuestionsProjection(
            PendingQuestionsView::from_questions(vec![approval_question()]),
        ),
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
            identifier: UpdateIdentifier::new("update-1"),
            mutation: InterfaceMutation::SetStatus(StatusText::new("waiting")),
        }),
        MentciRequest::ObserveInterfaceState(InterfaceStateObservation {
            subscriber: SubscriberName::new("status-bar"),
            interest: InterfaceInterest::StatusOnly,
        }),
        MentciRequest::AnswerQuestion(ApprovalVerdict {
            question: QuestionIdentifier::new("question-1"),
            decision: ApprovalDecision::ApproveSuggestedAnswer,
            answered_by: SubscriberName::new("psyche"),
        }),
        MentciRequest::ProposeEditedAnswer(AnswerProposal {
            question: QuestionIdentifier::new("question-1"),
            body: AnswerText::new("replacement-nota-object"),
            authored_by: SubscriberName::new("psyche"),
        }),
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
            question: QuestionIdentifier::new("question-1"),
            revision: RevisionCounter::new(1),
            accepted_at: TimestampNanos::new(10),
        }),
        MentciReply::UpdateAccepted(UpdateAccepted {
            identifier: UpdateIdentifier::new("update-1"),
            revision: RevisionCounter::new(2),
        }),
        MentciReply::InterfaceObservationOpened(InterfaceObservationOpened {
            token: SubscriptionToken::new("subscription-1"),
            state: projected_state(),
        }),
        MentciReply::VerdictAccepted(signal_mentci::VerdictAccepted {
            question: QuestionIdentifier::new("question-1"),
            decision: ApprovalDecision::Reject,
            accepted_at: TimestampNanos::new(11),
        }),
        MentciReply::AnswerProposalAdmitted(AnswerProposalAdmitted {
            proposal: ProposalIdentifier::new("proposal-1"),
            question: QuestionIdentifier::new("question-1"),
            digest: ProposalDigest::new("proposal-digest-1"),
            revision: RevisionCounter::new(3),
        }),
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
        revision: RevisionCounter::new(4),
        projection: InterfaceProjection::StatusProjection(StatusText::new("waiting")),
    };
    assert_nota_round_trips(&status_projection);

    let full_projection = ProjectedInterfaceState {
        revision: RevisionCounter::new(5),
        projection: InterfaceProjection::FullProjection(InterfaceState::new(
            RevisionCounter::new(5),
            StatusText::new("waiting"),
            Some(NotificationText::new("new-question")),
            vec![PaneContent {
                pane: PaneLabel::new("approval"),
                body: ContextBody::new("question-context"),
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
            .source
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
            .source
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
    assert!(agent.source.criome_slot().is_none());
}
