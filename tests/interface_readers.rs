use signal_mentci::{
    AnswerText, ApprovalQuestion, ApprovalSource, ContextBody, ContextLabel, CriomeAccess,
    ExplanationText, InterfaceProjection, InterfaceState, PaneContent, PendingQuestionsView,
    ProjectedInterfaceState, PromptText, QuestionContext, QuestionIdentifier, QuestionProposal,
    RevisionCounter, StatusText,
};

fn question_proposal() -> QuestionProposal {
    QuestionProposal::new(
        ApprovalSource::AgentQuestion,
        PromptText::new("approve"),
        Some(AnswerText::new("yes")),
        ExplanationText::new("review"),
        vec![QuestionContext {
            context_label: ContextLabel::new("record"),
            context_body: ContextBody::new("content"),
        }],
    )
}

fn question() -> ApprovalQuestion {
    ApprovalQuestion {
        question_identifier: QuestionIdentifier::new("question-1"),
        question_proposal: question_proposal(),
    }
}

#[test]
fn readers_expose_only_the_projection_content() {
    let state = InterfaceState::new(
        RevisionCounter::new(7),
        StatusText::new("waiting"),
        None,
        vec![PaneContent {
            pane_label: signal_mentci::PaneLabel::new("approval"),
            context_body: ContextBody::new("question-context"),
        }],
        vec![question()],
        CriomeAccess::ReadWrite,
    );
    assert_eq!(state.panes()[0].pane_label.as_str(), "approval");
    assert_eq!(state.pending_questions().len(), 1);
    assert!(state.notification().is_none());

    let pending = ProjectedInterfaceState {
        revision_counter: RevisionCounter::new(8),
        interface_projection: InterfaceProjection::PendingQuestionsProjection(
            PendingQuestionsView::from_questions(vec![question()]),
        ),
    };
    assert_eq!(pending.pending_questions(), &[question()]);
    assert_eq!(pending.criome_access(), None);

    let full = ProjectedInterfaceState {
        revision_counter: RevisionCounter::new(9),
        interface_projection: InterfaceProjection::FullProjection(state),
    };
    assert_eq!(full.pending_questions(), &[question()]);
    assert_eq!(full.criome_access(), Some(CriomeAccess::ReadWrite));

    let proposal = question_proposal();
    assert_eq!(proposal.suggested_answer().unwrap().as_str(), "yes");
    assert_eq!(proposal.context()[0].context_label.as_str(), "record");
    assert_eq!(proposal.context()[0].context_body.as_str(), "content");
}
