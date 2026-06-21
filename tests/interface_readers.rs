use signal_mentci::{
    AnswerText, ApprovalQuestion, ApprovalSource, ContextBody, ContextLabel, ExplanationText,
    InterfaceProjection, InterfaceState, NotificationText, PaneContent, PaneLabel,
    PendingQuestionsView, ProjectedInterfaceState, PromptText, QuestionContext, QuestionIdentifier,
    QuestionProposal, RevisionCounter, StatusText,
};

fn question_proposal() -> QuestionProposal {
    QuestionProposal::new(
        ApprovalSource::CriomeEscalation,
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

#[test]
fn interface_state_readers_expose_wrapped_fields() {
    let question = approval_question();
    let state = InterfaceState::new(
        RevisionCounter::new(7),
        StatusText::new("waiting"),
        Some(NotificationText::new("new-question")),
        vec![PaneContent {
            pane: PaneLabel::new("approval"),
            body: ContextBody::new("question-context"),
        }],
        vec![question.clone()],
    );

    assert_eq!(
        state.notification().expect("notification").as_str(),
        "new-question"
    );
    assert_eq!(state.panes().len(), 1);
    assert_eq!(state.panes()[0].pane.as_str(), "approval");
    assert_eq!(state.pending_questions(), std::slice::from_ref(&question));
}

#[test]
fn pending_question_projection_reader_matches_full_projection_reader() {
    let question = approval_question();
    let pending_projection = ProjectedInterfaceState {
        revision: RevisionCounter::new(8),
        projection: InterfaceProjection::PendingQuestionsProjection(
            PendingQuestionsView::from_questions(vec![question.clone()]),
        ),
    };
    let full_projection = ProjectedInterfaceState {
        revision: RevisionCounter::new(9),
        projection: InterfaceProjection::FullProjection(InterfaceState::new(
            RevisionCounter::new(9),
            StatusText::new("waiting"),
            None,
            vec![],
            vec![question.clone()],
        )),
    };

    assert_eq!(
        pending_projection.pending_questions(),
        std::slice::from_ref(&question)
    );
    assert_eq!(
        full_projection.pending_questions(),
        std::slice::from_ref(&question)
    );
}

#[test]
fn non_question_projections_have_empty_question_readers() {
    let status_projection = ProjectedInterfaceState {
        revision: RevisionCounter::new(10),
        projection: InterfaceProjection::StatusProjection(StatusText::new("waiting")),
    };
    let notification_projection = ProjectedInterfaceState {
        revision: RevisionCounter::new(11),
        projection: InterfaceProjection::NotificationProjection(Some(NotificationText::new(
            "new-question",
        ))),
    };

    assert!(status_projection.pending_questions().is_empty());
    assert!(notification_projection.pending_questions().is_empty());
}

#[test]
fn question_proposal_readers_expose_optional_answer_and_context() {
    let proposal = question_proposal();

    assert_eq!(
        proposal
            .suggested_answer()
            .expect("suggested answer")
            .as_str(),
        "approve"
    );
    assert_eq!(proposal.context().len(), 1);
    assert_eq!(proposal.context()[0].label.as_str(), "record");
    assert_eq!(
        proposal.context()[0].body.as_str(),
        "content-addressed-preimage"
    );
}
