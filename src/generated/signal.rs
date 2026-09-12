#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ApprovalSource {
    CriomeEscalation(signal_criome::AuthorizationRequestSlot),
    AgentQuestion,
    CriomeInterception(signal_criome::ParkedRequestIdentifier),
    LocalSystemPrompt,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InterfaceInterest {
    PendingQuestions,
    Notifications,
    StatusOnly,
    FullInterfaceState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterfaceUpdate {
    pub update_identifier: UpdateIdentifier,
    pub interface_mutation: InterfaceMutation,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AnswerProposalAdmitted {
    pub proposal_identifier: ProposalIdentifier,
    pub question_identifier: QuestionIdentifier,
    pub proposal_digest: ProposalDigest,
    pub revision_counter: RevisionCounter,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ApprovalQuestion {
    pub question_identifier: QuestionIdentifier,
    pub question_proposal: QuestionProposal,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum CriomeAccess {
    ReadWrite,
    ReadOnly,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RejectionReason {
    UnsupportedMutation,
    UnknownQuestion,
    MalformedRequest,
    UnauthorizedProjection,
    UnknownSubscriber,
    QuestionAlreadyAnswered,
}
#[rustfmt::skip]
pub type ProposalDigest = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum NotificationSlice {
    Empty,
    Present(NotificationText),
}
#[rustfmt::skip]
pub type SubscriptionToken = String;
#[rustfmt::skip]
pub type AnswerText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ApprovalVerdict {
    pub question_identifier: QuestionIdentifier,
    pub approval_decision: ApprovalDecision,
    pub subscriber_name: SubscriberName,
}
#[rustfmt::skip]
pub type ProposalIdentifier = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InterfaceMutation {
    SetPaneContent(PaneContent),
    PostNotification(NotificationText),
    PresentApprovalQuestion(QuestionProposal),
    WithdrawApprovalQuestion(QuestionIdentifier),
    ClearPane(PaneLabel),
    SetStatus(StatusText),
}
#[rustfmt::skip]
pub type UpdateIdentifier = String;
#[rustfmt::skip]
pub type QuestionIdentifier = String;
#[rustfmt::skip]
pub type RevisionCounter = i64;
#[rustfmt::skip]
pub type StatusText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct VerdictAccepted {
    pub question_identifier: QuestionIdentifier,
    pub approval_decision: ApprovalDecision,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterfaceObservationRetracted {
    pub subscription_token: SubscriptionToken,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum InterfaceProjection {
    StatusProjection(StatusText),
    FullProjection(InterfaceState),
    NotificationProjection(NotificationSlice),
    PendingQuestionsProjection(PendingQuestionsView),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterceptPolicyObservation {}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct Rejection {
    pub rejection_reason: RejectionReason,
}
#[rustfmt::skip]
pub type ExplanationText = String;
#[rustfmt::skip]
pub type ComponentLabel = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterfaceObservationOpened {
    pub subscription_token: SubscriptionToken,
    pub projected_interface_state: ProjectedInterfaceState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ApprovalDecision {
    Reject,
    ApproveSuggestedAnswer,
    Defer,
}
#[rustfmt::skip]
pub type PromptText = String;
#[rustfmt::skip]
pub type TimestampNanos = i64;
#[rustfmt::skip]
pub type NotificationText = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct UpdateAccepted {
    pub update_identifier: UpdateIdentifier,
    pub revision_counter: RevisionCounter,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ProjectedInterfaceState {
    pub revision_counter: RevisionCounter,
    pub interface_projection: InterfaceProjection,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct AnswerProposal {
    pub question_identifier: QuestionIdentifier,
    pub answer_text: AnswerText,
    pub subscriber_name: SubscriberName,
}
#[rustfmt::skip]
pub type SubscriberName = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuestionPresented {
    pub question_identifier: QuestionIdentifier,
    pub revision_counter: RevisionCounter,
    pub timestamp_nanos: TimestampNanos,
}
#[rustfmt::skip]
pub type PaneLabel = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuestionContext {
    pub context_label: ContextLabel,
    pub context_body: ContextBody,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterfaceState {
    pub revision_counter: RevisionCounter,
    pub status_text: StatusText,
    pub notification_text: NotificationText,
    pub pane_content: PaneContent,
    pub approval_question: ApprovalQuestion,
    pub criome_access: CriomeAccess,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PendingQuestionsView {
    pub approval_question: ApprovalQuestion,
}
#[rustfmt::skip]
pub type ContextLabel = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct InterfaceStateObservation {
    pub subscriber_name: SubscriberName,
    pub interface_interest: InterfaceInterest,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct PaneContent {
    pub pane_label: PaneLabel,
    pub context_body: ContextBody,
}
#[rustfmt::skip]
pub type ContextBody = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct QuestionProposal {
    pub approval_source: ApprovalSource,
    pub prompt_text: PromptText,
    pub answer_text: AnswerText,
    pub explanation_text: ExplanationText,
    pub question_context: QuestionContext,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    RetractInterfaceObservation(SubscriptionToken),
    PushUpdate(InterfaceUpdate),
    ObserveInterfaceState(InterfaceStateObservation),
    ListInterceptPolicies(InterceptPolicyObservation),
    FetchParkedRequests(signal_criome::ParkedRequestQuery),
    PresentQuestion(QuestionProposal),
    ReplaceInterceptPolicy(signal_criome::InterceptPolicyProposal),
    CreateInterceptPolicy(signal_criome::InterceptPolicyProposal),
    CancelInterceptPolicy(signal_criome::InterceptPolicyCancellation),
    ProposeEditedAnswer(AnswerProposal),
    AnswerQuestion(ApprovalVerdict),
    AnswerParkedRequest(signal_criome::ParkedRequestAnswer),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    InterfaceObservationStarted(InterfaceObservationOpened),
    UpdateApplied(UpdateAccepted),
    ParkedRequestAnswered(signal_criome::ParkedRequestResolution),
    VerdictRecorded(VerdictAccepted),
    AnswerProposalAccepted(AnswerProposalAdmitted),
    QuestionAccepted(QuestionPresented),
    ParkedRequestsFetched(signal_criome::ParkedRequestSnapshot),
    InterceptPolicyCancelled(signal_criome::InterceptPolicyIdentifier),
    InterceptPolicyReplaced(signal_criome::InterceptPolicy),
    InterceptPoliciesListed(signal_criome::ActiveInterceptPolicies),
    InterceptPolicyCreated(signal_criome::InterceptPolicy),
    RequestRejected(Rejection),
    InterfaceObservationClosed(InterfaceObservationRetracted),
}
