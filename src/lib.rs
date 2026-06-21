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
        criome_access: CriomeAccess,
    ) -> Self {
        Self {
            revision,
            status,
            notification: Notification::new(notification),
            panes: Panes::new(panes),
            pending_questions: PendingQuestions::new(pending_questions),
            criome_access,
        }
    }

    /// The current notification, if one is posted. Reader for the
    /// `pub(crate)`-wrapped field so consumers (mentci-lib's shared
    /// observability model) can project state they did not build.
    pub fn notification(&self) -> Option<&NotificationText> {
        self.notification.payload().as_ref()
    }

    /// The open panes.
    pub fn panes(&self) -> &[PaneContent] {
        self.panes.payload().as_slice()
    }

    /// The pending approval questions in this canonical state.
    pub fn pending_questions(&self) -> &[ApprovalQuestion] {
        self.pending_questions.payload().as_slice()
    }

    /// The daemon's criome access level, mirrored into canonical state so a
    /// client renders read-only or read-write controls.
    pub fn criome_access(&self) -> CriomeAccess {
        self.criome_access
    }
}

impl PendingQuestionsView {
    pub fn from_questions(questions: Vec<ApprovalQuestion>) -> Self {
        Self::new(VisibleQuestions::new(questions))
    }

    /// The questions a `PendingQuestions`-interest subscriber is allowed to
    /// see. Reader for the `pub(crate)`-wrapped inner.
    pub fn questions(&self) -> &[ApprovalQuestion] {
        self.payload().payload().as_slice()
    }
}

impl ProjectedInterfaceState {
    /// Extract the pending approval questions visible in this projection,
    /// whatever the interest. A `FullProjection` reads the canonical state's
    /// queue; a `PendingQuestionsProjection` reads its visible slice; the
    /// status / notification projections carry no questions. This is the one
    /// reader the shared observability model needs to drive the approval
    /// cursor regardless of which interest opened the stream.
    pub fn pending_questions(&self) -> &[ApprovalQuestion] {
        match &self.projection {
            InterfaceProjection::FullProjection(state) => state.pending_questions(),
            InterfaceProjection::PendingQuestionsProjection(view) => view.questions(),
            InterfaceProjection::StatusProjection(_)
            | InterfaceProjection::NotificationProjection(_) => &[],
        }
    }

    /// The mirrored criome access level visible in this projection. Only a
    /// `FullProjection` carries the canonical `InterfaceState` and thus the
    /// mode; the status / notification / pending-questions slices do not, so a
    /// client on a narrow interest learns no mode (`None`) and defaults to
    /// observation-only.
    pub fn criome_access(&self) -> Option<CriomeAccess> {
        match &self.projection {
            InterfaceProjection::FullProjection(state) => Some(state.criome_access()),
            InterfaceProjection::StatusProjection(_)
            | InterfaceProjection::NotificationProjection(_)
            | InterfaceProjection::PendingQuestionsProjection(_) => None,
        }
    }
}

impl QuestionProposal {
    /// The suggested answer, if the asking agent supplied one. Reader for the
    /// `pub(crate)`-wrapped field.
    pub fn suggested_answer(&self) -> Option<&AnswerText> {
        self.suggested_answer.payload().as_ref()
    }

    /// The context entries attached to this question.
    pub fn context(&self) -> &[QuestionContext] {
        self.context.payload().as_slice()
    }
}

impl ApprovalSource {
    /// The parked criome authorization slot, when this question originated from
    /// a criome `EscalateToPsyche` escalation. A closed verdict on such a
    /// question keys straight back to criome by this slot (the shared
    /// `signal-criome` identity, no stringly round-trip); other sources have
    /// none. The shared client model reads this to decide whether an answered
    /// question routes to criome or to the mentci socket.
    pub fn criome_slot(&self) -> Option<&AuthorizationRequestSlot> {
        match self {
            ApprovalSource::CriomeEscalation(slot) => Some(slot),
            ApprovalSource::AgentQuestion | ApprovalSource::LocalSystemPrompt => None,
        }
    }
}
