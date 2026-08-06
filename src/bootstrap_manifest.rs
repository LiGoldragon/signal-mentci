//! Producer-owned authority state for the ordinary Mentci Interface.
//!
//! Every identity and canonical-order value is an allocated opaque seat.
//! None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}
pub const AUTHORITY_IDENTITY: [u8; 32] = [
    202, 145, 227, 203, 21, 175, 232, 86, 123, 227, 237, 109, 205, 164, 2, 109, 108, 118, 99, 39,
    63, 225, 1, 120, 163, 191, 254, 147, 76, 135, 75, 103,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 138;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 8027;
pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 54168, 0x52acc5134ff10d8b);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 49253, 0x78ba0c5548ebafb5);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 46425, 0xc04419d90415a9ef);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 19102, 0x0105eb5abbc99ab9);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 63302, 0xf6fbe4d7d31e4493);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 42317, 0x87bfc95f00bb2dfd);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 54368, 0x9062dede5a7e4177);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 61526, 0x8f1e29af3c5a6d81);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 30644, 0xa3e43eeb3b17449b);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 55230, 0x36e6b472f3d99d45);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 52950, 0x22d06751a39d31ff);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 33601, 0xac91304f96074149);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 39822, 0xb5502c64b93a2da3);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 22308, 0x6864c1bd8c921d8d);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 14544, 0x206dde8b42729b87);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 6727, 0x6070bac6d98d3611);
pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    50929, 14409, 21886, 34099, 45847, 8610, 33, 24557, 52721, 23786,
];
pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "MentciRequest", 52705, 0x08635d0cdc91d11d),
    DeclarationSeat::new(None, "MentciReply", 26023, 0x7398a373bbf55997),
    DeclarationSeat::new(None, "QuestionIdentifier", 45533, 0x6d4a356106ed02a1),
    DeclarationSeat::new(None, "ProposalIdentifier", 5907, 0x682a225816279ebb),
    DeclarationSeat::new(None, "SubscriptionToken", 27380, 0x5398b94cda834465),
    DeclarationSeat::new(None, "SubscriberName", 18993, 0xd5335730b4d6ee1f),
    DeclarationSeat::new(None, "PromptText", 11611, 0xbf7f26367d951a69),
    DeclarationSeat::new(None, "AnswerText", 36308, 0x5583525dbdef6bc3),
    DeclarationSeat::new(None, "ExplanationText", 48498, 0x9c2e404d246348ad),
    DeclarationSeat::new(None, "ContextLabel", 18074, 0xf3ff021700d77ba7),
    DeclarationSeat::new(None, "ContextBody", 36819, 0xf826181aa0b2d331),
    DeclarationSeat::new(None, "UpdateIdentifier", 25888, 0x6a3d109fc993c1cb),
    DeclarationSeat::new(None, "ComponentLabel", 49835, 0xa5ca6279e190fdf5),
    DeclarationSeat::new(None, "PaneLabel", 6009, 0xda34aa749e2b222f),
    DeclarationSeat::new(None, "StatusText", 1100, 0x71f3f010e7574cf9),
    DeclarationSeat::new(None, "NotificationText", 43484, 0xc7fbf86d0a4ac0d3),
    DeclarationSeat::new(None, "TimestampNanos", 47875, 0xc34b114cefef843d),
    DeclarationSeat::new(None, "RevisionCounter", 2717, 0x71af8825dd2a01b7),
    DeclarationSeat::new(None, "ProposalDigest", 550, 0x4553e143df57a7c1),
    DeclarationSeat::new(None, "ApprovalSource", 42333, 0x0516ac867dae88db),
    DeclarationSeat::new(None, "QuestionContext", 20593, 0xdeda5d062865fb85),
    DeclarationSeat::new(None, "QuestionProposal", 35204, 0xf878db1638d03a3f),
    DeclarationSeat::new(None, "ApprovalQuestion", 56209, 0x2f742a18b9cd0389),
    DeclarationSeat::new(None, "ApprovalDecision", 4079, 0xad6c1f5cae3d39e3),
    DeclarationSeat::new(None, "CriomeAccess", 3329, 0x3471163eb35f83cd),
    DeclarationSeat::new(None, "ApprovalVerdict", 29669, 0x567e3df2bf3debc7),
    DeclarationSeat::new(None, "AnswerProposal", 39956, 0xd02ac8aee7948051),
    DeclarationSeat::new(None, "InterceptPolicyObservation", 4943, 0x816b0377a3d8f3eb),
    DeclarationSeat::new(None, "InterfaceUpdate", 8530, 0x199231896d4b3d15),
    DeclarationSeat::new(None, "InterfaceMutation", 18983, 0x696f5a900a37364f),
    DeclarationSeat::new(None, "PaneContent", 14088, 0xf595e41dc5cf3e19),
    DeclarationSeat::new(None, "InterfaceState", 7472, 0xe2501dab6447d6f3),
    DeclarationSeat::new(None, "InterfaceInterest", 31513, 0x10655b925b1c475d),
    DeclarationSeat::new(None, "InterfaceStateObservation", 10216, 0xf40e8fc5c7a439d7),
    DeclarationSeat::new(None, "ProjectedInterfaceState", 1562, 0xcec1dc025a625ce1),
    DeclarationSeat::new(
        None,
        "InterfaceObservationOpened",
        33210,
        0xaab600fb03b402fb,
    ),
    DeclarationSeat::new(None, "NotificationSlice", 11165, 0x4c68246f2ec9c2a5),
    DeclarationSeat::new(None, "InterfaceProjection", 36471, 0x79354ac1d211165f),
    DeclarationSeat::new(None, "PendingQuestionsView", 38860, 0xec7eec2f2076fca9),
    DeclarationSeat::new(None, "QuestionPresented", 60934, 0xd8a4924c452b9803),
    DeclarationSeat::new(None, "UpdateAccepted", 7737, 0xcc846912dbceceed),
    DeclarationSeat::new(None, "VerdictAccepted", 65370, 0x73a19014d92debe7),
    DeclarationSeat::new(None, "AnswerProposalAdmitted", 52724, 0x2ccb168fe4fa3d71),
    DeclarationSeat::new(
        None,
        "InterfaceObservationRetracted",
        43684,
        0x772a904dcb20b60b,
    ),
    DeclarationSeat::new(None, "RejectionReason", 7901, 0x39692a163baa8c35),
    DeclarationSeat::new(None, "Rejection", 26313, 0x99af72fa9a4eda6f),
    DeclarationSeat::new(Some(52705), "PresentQuestion", 19515, 0x5ecb9193b31d3f39),
    DeclarationSeat::new(Some(52705), "PushUpdate", 44565, 0x2c184231d7e97d13),
    DeclarationSeat::new(
        Some(52705),
        "ObserveInterfaceState",
        12429,
        0x3299cf47c2601a7d,
    ),
    DeclarationSeat::new(Some(52705), "AnswerQuestion", 22065, 0xd6e0039fa8ec01f7),
    DeclarationSeat::new(
        Some(52705),
        "ProposeEditedAnswer",
        13984,
        0xcd644daed0d52201,
    ),
    DeclarationSeat::new(
        Some(52705),
        "CreateInterceptPolicy",
        32593,
        0x9fb95e1f9e400d1b,
    ),
    DeclarationSeat::new(
        Some(52705),
        "ReplaceInterceptPolicy",
        13281,
        0x72f7efb442f699c5,
    ),
    DeclarationSeat::new(
        Some(52705),
        "CancelInterceptPolicy",
        3762,
        0xc8188f66c721827f,
    ),
    DeclarationSeat::new(
        Some(52705),
        "ListInterceptPolicies",
        16371,
        0x404d40c6cb5b05c9,
    ),
    DeclarationSeat::new(
        Some(52705),
        "FetchParkedRequests",
        59459,
        0x53962e7d21c28623,
    ),
    DeclarationSeat::new(
        Some(52705),
        "AnswerParkedRequest",
        56707,
        0xfbe6161ac3f92a0d,
    ),
    DeclarationSeat::new(
        Some(52705),
        "RetractInterfaceObservation",
        23457,
        0x2691e86fce2f7c07,
    ),
    DeclarationSeat::new(Some(26023), "QuestionAccepted", 2138, 0x9f750a9a93ac0a91),
    DeclarationSeat::new(Some(26023), "UpdateApplied", 30792, 0x4799fb73a773082b),
    DeclarationSeat::new(
        Some(26023),
        "InterfaceObservationStarted",
        64692,
        0x00f7a1fd63f6eb55,
    ),
    DeclarationSeat::new(Some(26023), "VerdictRecorded", 15623, 0x8858cbe226fa0e8f),
    DeclarationSeat::new(
        Some(26023),
        "AnswerProposalAccepted",
        52176,
        0x9c103178ab095059,
    ),
    DeclarationSeat::new(
        Some(26023),
        "InterceptPolicyCreated",
        54963,
        0xede7c450b637b333,
    ),
    DeclarationSeat::new(
        Some(26023),
        "InterceptPolicyReplaced",
        4551,
        0xd503d5c14e02fd9d,
    ),
    DeclarationSeat::new(
        Some(26023),
        "InterceptPolicyCancelled",
        25119,
        0xc3ff68bcd2895a17,
    ),
    DeclarationSeat::new(
        Some(26023),
        "InterceptPoliciesListed",
        24736,
        0xe484aad55f77f721,
    ),
    DeclarationSeat::new(
        Some(26023),
        "ParkedRequestsFetched",
        58700,
        0xc2af62a5a75aa73b,
    ),
    DeclarationSeat::new(
        Some(26023),
        "ParkedRequestAnswered",
        13476,
        0x53115a8fbe3480e5,
    ),
    DeclarationSeat::new(
        Some(26023),
        "InterfaceObservationClosed",
        29660,
        0xfbcd7b2e02897e9f,
    ),
    DeclarationSeat::new(Some(26023), "RequestRejected", 34286, 0xf4339f9318411ee9),
    DeclarationSeat::new(Some(42333), "CriomeEscalation", 45772, 0x1a254747c70a0443),
    DeclarationSeat::new(Some(42333), "CriomeInterception", 19270, 0xe3bd02ea1626952d),
    DeclarationSeat::new(Some(42333), "AgentQuestion", 12130, 0x37916ab441ca9c27),
    DeclarationSeat::new(Some(42333), "LocalSystemPrompt", 33327, 0xfad8600eb271e7b1),
    DeclarationSeat::new(
        Some(4079),
        "ApproveSuggestedAnswer",
        15715,
        0xc91f1f9304d7ea4b,
    ),
    DeclarationSeat::new(Some(4079), "Reject", 36925, 0x136b5d2301785a75),
    DeclarationSeat::new(Some(4079), "Defer", 20616, 0xfb8a5b8b2cc0d2af),
    DeclarationSeat::new(Some(3329), "ReadOnly", 4524, 0x4ecc4841ed5b7179),
    DeclarationSeat::new(Some(3329), "ReadWrite", 1330, 0x40b0d791343a7953),
    DeclarationSeat::new(Some(18983), "SetStatus", 26926, 0xfad1b72eaa4cf0bd),
    DeclarationSeat::new(Some(18983), "PostNotification", 19548, 0x2f322e25ba044237),
    DeclarationSeat::new(Some(18983), "SetPaneContent", 42243, 0x0d1ce88de712dc41),
    DeclarationSeat::new(Some(18983), "ClearPane", 28369, 0x98809be7dd0bd15b),
    DeclarationSeat::new(
        Some(18983),
        "PresentApprovalQuestion",
        36174,
        0x5d618d7bfdcb7805,
    ),
    DeclarationSeat::new(
        Some(18983),
        "WithdrawApprovalQuestion",
        56223,
        0x848ff9b812d10abf,
    ),
    DeclarationSeat::new(Some(31513), "FullInterfaceState", 61350, 0xde4523ffa8f14809),
    DeclarationSeat::new(Some(31513), "StatusOnly", 62922, 0xca8eb4ae9c0a1263),
    DeclarationSeat::new(Some(31513), "Notifications", 31722, 0x4631fec9009f104d),
    DeclarationSeat::new(Some(31513), "PendingQuestions", 21828, 0x0ce8a912fb874c47),
    DeclarationSeat::new(Some(11165), "Empty", 50778, 0x3fe41920c413d4d1),
    DeclarationSeat::new(Some(11165), "Present", 12060, 0x4c28509013575c6b),
    DeclarationSeat::new(Some(36471), "FullProjection", 17122, 0x1996c7243376d995),
    DeclarationSeat::new(Some(36471), "StatusProjection", 55541, 0x12f50352cc2b26cf),
    DeclarationSeat::new(
        Some(36471),
        "NotificationProjection",
        19897,
        0x6d7e96a5fddba299,
    ),
    DeclarationSeat::new(
        Some(36471),
        "PendingQuestionsProjection",
        39865,
        0xa7139dd76af9cf73,
    ),
    DeclarationSeat::new(Some(7901), "UnknownQuestion", 38186, 0x283c748d0785f3dd),
    DeclarationSeat::new(
        Some(7901),
        "QuestionAlreadyAnswered",
        18831,
        0xce339423f8e4ba57,
    ),
    DeclarationSeat::new(Some(7901), "MalformedRequest", 30896, 0x53d2918e0c6dd161),
    DeclarationSeat::new(Some(7901), "UnknownSubscriber", 19209, 0xba40dd4c615b8b7b),
    DeclarationSeat::new(Some(7901), "UnsupportedMutation", 12616, 0x100effe563037f25),
    DeclarationSeat::new(
        Some(7901),
        "UnauthorizedProjection",
        6049,
        0xa392999f2a8026df,
    ),
];
