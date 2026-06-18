//! Schema-derived Signal contract for Mentci's programmable UI surface.
//!
//! The Mentci daemon owns canonical UI state. This crate owns the typed wire
//! vocabulary used by thin clients and agentic flows to present questions,
//! observe projected state, submit closed verdicts, and propose edited answers
//! as new typed proposal objects.

#[rustfmt::skip]
#[allow(clippy::large_enum_variant, dead_code, private_interfaces)]
pub mod schema;

pub use schema::lib::*;

pub type MentciRequest = Input;
pub type MentciReply = Output;
pub type MentciFrame = signal_frame::StreamingFrame<Input, Output, MentciEvent>;
pub type MentciFrameBody = signal_frame::StreamingFrameBody<Input, Output, MentciEvent>;
pub type MentciReplyEnvelope = ReplyEnvelope;
pub type MentciRequestBuilder = RequestBuilder;
pub type MentciOperationKind = InputRoute;

impl Input {
    pub fn operation_kind(&self) -> InputRoute {
        self.route()
    }
}

macro_rules! string_accessor {
    ($($type:ident),* $(,)?) => {
        $(
            impl $type {
                pub fn as_str(&self) -> &str {
                    self.payload().as_str()
                }
            }
        )*
    };
}

string_accessor!(
    QuestionIdentifier,
    ProposalIdentifier,
    SubscriptionToken,
    SubscriberName,
    PromptText,
    AnswerText,
    ExplanationText,
    ContextLabel,
    ContextBody,
    UpdateIdentifier,
    ComponentLabel,
    PaneLabel,
    StatusText,
    NotificationText,
    ProposalDigest,
    SocketPath,
);

impl RevisionCounter {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl TimestampNanos {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl QuestionProposal {
    pub fn new(
        source: ApprovalSource,
        prompt: PromptText,
        suggested_answer: Option<AnswerText>,
        explanation: ExplanationText,
        context: Vec<QuestionContext>,
    ) -> Self {
        Self {
            source,
            prompt,
            suggested_answer: SuggestedAnswer::new(suggested_answer),
            explanation,
            context: Context::new(context),
        }
    }
}

impl InterfaceState {
    pub fn new(
        revision: RevisionCounter,
        status: StatusText,
        notification: Option<NotificationText>,
        panes: Vec<PaneContent>,
        pending_questions: Vec<ApprovalQuestion>,
    ) -> Self {
        Self {
            revision,
            status,
            notification: Notification::new(notification),
            panes: Panes::new(panes),
            pending_questions: PendingQuestions::new(pending_questions),
        }
    }
}

impl PendingQuestionsView {
    pub fn from_questions(questions: Vec<ApprovalQuestion>) -> Self {
        Self::new(VisibleQuestions::new(questions))
    }
}
