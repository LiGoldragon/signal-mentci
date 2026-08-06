// Handwritten operational behavior for the authority-verified ordinary Mentci Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, Dotos roles, and the allocated Signal frame boundary.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize, rancor::Source as _};
use signal_standard::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 { return Err(WireShapeError); }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name { fn to_wire(&self) -> WireValue { self.payload().to_wire() } fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) } }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")] impl dotos::DotosEncode for $name { fn to_dotos(&self) -> String { dotos::DotosEncode::to_dotos(self.payload()) } }
        #[cfg(feature = "dotos-text")] impl dotos::DotosDecode for $name { fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> { <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new) } }
    };
}
#[allow(unused_macros)]
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name { pub fn new(payload: $inner) -> Self { Self(payload) } pub fn payload(&self) -> &$inner { &self.0 } pub fn into_payload(self) -> $inner { self.0 } }
        impl WireShape for $name { fn to_wire(&self) -> WireValue { self.0.to_wire() } fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) } }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")] impl dotos::DotosEncode for $name { fn to_dotos(&self) -> String { dotos::DotosEncode::to_dotos(&self.0) } }
        #[cfg(feature = "dotos-text")] impl dotos::DotosDecode for $name { fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> { <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self) } }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { let WireValue::Product(fields) = value else { return Err(WireShapeError) }; let mut fields = fields.into_iter(); let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* }; if fields.next().is_some() { return Err(WireShapeError); } Ok(result) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")] impl dotos::DotosEncode for $name { fn to_dotos(&self) -> String { dotos::Delimiter::Parenthesis.wrap([$(dotos::DotosEncode::to_dotos(&self.$field)),*]) } }
        #[cfg(feature = "dotos-text")] impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> { let body = dotos::DotosBody::from_delimited(block, dotos::Delimiter::Parenthesis, stringify!($name))?; let expected = 0usize $(+ { let _ = stringify!($field); 1usize })*; #[allow(unused_mut, unused_variables)] let mut fields = body.expect_fields(stringify!($name), expected)?.iter(); Ok(Self { $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(fields.next().expect("field count checked"))?),* }) }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident { unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? } unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? } }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { match self { $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)* $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)* } }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) }; match ordinal { $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)* $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)* _ => Err(WireShapeError), } }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")] impl dotos::DotosEncode for $name { fn to_dotos(&self) -> String { match self { $(Self::$unit => $unit_visible.to_owned(),)* $(Self::$unary(payload) => format!("{}.{}", $unary_visible, dotos::DotosEncode::to_dotos(payload)),)* } } }
        #[cfg(feature = "dotos-text")] impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> { if let Some(variant) = block.demote_to_string() { return match variant { $($unit_visible => Ok(Self::$unit),)* _ => Err(dotos::DotosDecodeError::UnknownVariant { enum_name: stringify!($name), variant: variant.to_owned() }), }; } let (head, payload) = block.as_application().ok_or(dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) })?; let _ = payload; let variant = head.demote_to_string().ok_or(dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) })?; match variant { $($unary_visible => Ok(Self::$unary(<$payload as dotos::DotosDecode>::from_dotos_block(payload)?)),)* _ => Err(dotos::DotosDecodeError::UnknownVariant { enum_name: stringify!($name), variant: variant.to_owned() }), } }
        }
    };
}
wire_enum!(z2VbRz { unit {  } unary { 0 => z2VSji(z2VTuM) : "RetractInterfaceObservation", 1 => z2VZ1e(z2VNJM) : "PushUpdate", 2 => z2VPTa(z2VNoR) : "ObserveInterfaceState", 3 => z2VQdY(z2VMEW) : "ListInterceptPolicies", 4 => z2VdSS(signal_criome::schema::lib::z2VNo7) : "FetchParkedRequests", 5 => z2VRZk(z2VWEF) : "PresentQuestion", 6 => z2VPiG(signal_criome::schema::lib::z2VUjo) : "ReplaceInterceptPolicy", 7 => z2VVTE(signal_criome::schema::lib::z2VUjo) : "CreateInterceptPolicy", 8 => z2VLt9(signal_criome::schema::lib::z2VXWs) : "CancelInterceptPolicy", 9 => z2VPvP(z2VXeB) : "ProposeEditedAnswer", 10 => z2VSKi(z2VUap) : "AnswerQuestion", 11 => z2Vccz(signal_criome::schema::lib::z2VSGX) : "AnswerParkedRequest" } });
wire_enum!(z2VTVx { unit {  } unary { 0 => z2Vezf(z2VVds) : "InterfaceObservationStarted", 1 => z2VUvB(z2VN4g) : "UpdateApplied", 2 => z2VPmd(signal_criome::schema::lib::z2VW3f) : "ParkedRequestAnswered", 3 => z2VQQe(z2VfCM) : "VerdictRecorded", 4 => z2VbGs(z2VbSK) : "AnswerProposalAccepted", 5 => z2VLQ9(z2Vdss) : "QuestionAccepted", 6 => z2VdDM(signal_criome::schema::lib::z2VRAT) : "ParkedRequestsFetched", 7 => z2VTEN(signal_criome::schema::lib::z2VShF) : "InterceptPolicyCancelled", 8 => z2VM7k(signal_criome::schema::lib::z2VfB8) : "InterceptPolicyReplaced", 9 => z2VT7m(signal_criome::schema::lib::z2Vb3U) : "InterceptPoliciesListed", 10 => z2Vc6v(signal_criome::schema::lib::z2VfB8) : "InterceptPolicyCreated", 11 => z2VVxR(z2VTax) : "RequestRejected", 12 => z2VUaf(z2VYkT) : "InterfaceObservationClosed" } });
wire_external_newtype!(z2VZJL, std::string::String);
wire_external_newtype!(z2VMX8, std::string::String);
wire_external_newtype!(z2VTuM, std::string::String);
wire_external_newtype!(z2VRQk, std::string::String);
wire_external_newtype!(z2VPDU, std::string::String);
wire_external_newtype!(z2VWZH, std::string::String);
wire_external_newtype!(z2VaBT, std::string::String);
wire_external_newtype!(z2VR8u, std::string::String);
wire_external_newtype!(z2VWi6, std::string::String);
wire_external_newtype!(z2VTTd, std::string::String);
wire_external_newtype!(z2VaaW, std::string::String);
wire_external_newtype!(z2VMYt, std::string::String);
wire_external_newtype!(z2VL6F, std::string::String);
wire_external_newtype!(z2VYh1, std::string::String);
wire_external_newtype!(z2VZzi, u64);
wire_external_newtype!(z2VLa8, u64);
wire_external_newtype!(z2VKvm, std::string::String);
wire_enum!(z2VYMA { unit { 1 => z2VPNR : "AgentQuestion", 3 => z2VVft : "LocalSystemPrompt" } unary { 0 => z2VZNT(signal_criome::schema::lib::z2VUph) : "CriomeEscalation", 2 => z2VRVX(signal_criome::schema::lib::z2VfEW) : "CriomeInterception" } });
wire_struct!(z2VRtL { field_0: z2VR8u, field_1: z2VWi6 });
wire_struct!(z2VWEF { field_0: z2VYMA, field_1: z2VPDU, field_2: z2VWZH, field_3: z2VaBT, field_4: z2VRtL });
wire_struct!(z2VcUQ { field_0: z2VZJL, field_1: z2VWEF });
wire_enum!(z2VLyc { unit { 0 => z2VWjv : "Reject", 1 => z2VQSE : "ApproveSuggestedAnswer", 2 => z2VRtj : "Defer" } unary {  } });
wire_enum!(z2VLkg { unit { 0 => z2VLAD : "ReadWrite", 1 => z2VM7H : "ReadOnly" } unary {  } });
wire_struct!(z2VUap { field_0: z2VZJL, field_1: z2VLyc, field_2: z2VRQk });
wire_struct!(z2VXeB { field_0: z2VZJL, field_1: z2VWZH, field_2: z2VRQk });
wire_struct!(z2VMEW {  });
wire_struct!(z2VNJM { field_0: z2VTTd, field_1: z2VRQa });
wire_enum!(z2VRQa { unit {  } unary { 0 => z2VYKc(z2VPxB ) : "SetPaneContent", 1 => z2VRaK(z2VYh1 ) : "PostNotification", 2 => z2VWWy(z2VWEF ) : "PresentApprovalQuestion", 3 => z2VcUe(z2VZJL ) : "WithdrawApprovalQuestion", 4 => z2VUCQ(z2VMYt ) : "ClearPane", 5 => z2VTmX(z2VL6F ) : "SetStatus" } });
wire_struct!(z2VPxB { field_0: z2VMYt, field_1: z2VWi6 });
wire_struct!(z2VMz7 { field_0: z2VLa8, field_1: z2VL6F, field_2: z2VYh1, field_3: z2VPxB, field_4: z2VcUQ, field_5: z2VLkg });
wire_enum!(z2VV8c { unit { 0 => z2VSFd : "PendingQuestions", 1 => z2VVCD : "Notifications", 2 => z2VeU9 : "StatusOnly", 3 => z2Ve13 : "FullInterfaceState" } unary {  } });
wire_struct!(z2VNoR { field_0: z2VRQk, field_1: z2VV8c });
wire_struct!(z2VLED { field_0: z2VLa8, field_1: z2VWc6 });
wire_struct!(z2VVds { field_0: z2VTuM, field_1: z2VLED });
wire_enum!(z2VP5n { unit { 0 => z2Varm : "Empty" } unary { 1 => z2VPMD(z2VYh1 ) : "Present" } });
wire_enum!(z2VWc6 { unit {  } unary { 0 => z2VcGt(z2VL6F ) : "StatusProjection", 1 => z2VQrV(z2VMz7 ) : "FullProjection", 2 => z2VRgL(z2VP5n ) : "NotificationProjection", 3 => z2VXcc(z2VXKH ) : "PendingQuestionsProjection" } });
wire_struct!(z2VXKH { field_0: z2VcUQ });
wire_struct!(z2Vdss { field_0: z2VZJL, field_1: z2VLa8, field_2: z2VZzi });
wire_struct!(z2VN4g { field_0: z2VTTd, field_1: z2VLa8 });
wire_struct!(z2VfCM { field_0: z2VZJL, field_1: z2VLyc, field_2: z2VZzi });
wire_struct!(z2VbSK { field_0: z2VMX8, field_1: z2VZJL, field_2: z2VKvm, field_3: z2VLa8 });
wire_struct!(z2VYkT { field_0: z2VTuM });
wire_enum!(z2VN7W { unit { 0 => z2VPWo : "UnsupportedMutation", 1 => z2VX7f : "UnknownQuestion", 2 => z2VUwy : "MalformedRequest", 3 => z2VMZa : "UnauthorizedProjection", 4 => z2VRUU : "UnknownSubscriber", 5 => z2VRMx : "QuestionAlreadyAnswered" } unary {  } });
wire_struct!(z2VTax { field_0: z2VN7W });

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root { type Archived = <WireValue as Archive>::Archived; type Resolver = <WireValue as Archive>::Resolver; fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) { self.to_wire().resolve(resolver, out); } }
        impl<Serializer> RkyvSerialize<Serializer> for $root where Serializer: rkyv::rancor::Fallible + ?Sized, WireValue: RkyvSerialize<Serializer> { fn serialize(&self, serializer: &mut Serializer) -> Result<Self::Resolver, Serializer::Error> { self.to_wire().serialize(serializer) } }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer> for signal_standard::schema::lib::ArchivedWireValue where Deserializer: rkyv::rancor::Fallible + ?Sized, Deserializer::Error: rkyv::rancor::Source, signal_standard::schema::lib::ArchivedWireValue: RkyvDeserialize<WireValue, Deserializer> { fn deserialize(&self, deserializer: &mut Deserializer) -> Result<$root, Deserializer::Error> { let wire = <signal_standard::schema::lib::ArchivedWireValue as RkyvDeserialize<WireValue, Deserializer>>::deserialize(self, deserializer)?; <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new) } }
    };
}
archive_root!(z2VbRz);
archive_root!(z2VTVx);
archive_root!(z2VZJL);
archive_root!(z2VMX8);
archive_root!(z2VTuM);
archive_root!(z2VRQk);
archive_root!(z2VPDU);
archive_root!(z2VWZH);
archive_root!(z2VaBT);
archive_root!(z2VR8u);
archive_root!(z2VWi6);
archive_root!(z2VTTd);
archive_root!(z2VaaW);
archive_root!(z2VMYt);
archive_root!(z2VL6F);
archive_root!(z2VYh1);
archive_root!(z2VZzi);
archive_root!(z2VLa8);
archive_root!(z2VKvm);
archive_root!(z2VYMA);
archive_root!(z2VRtL);
archive_root!(z2VWEF);
archive_root!(z2VcUQ);
archive_root!(z2VLyc);
archive_root!(z2VLkg);
archive_root!(z2VUap);
archive_root!(z2VXeB);
archive_root!(z2VMEW);
archive_root!(z2VNJM);
archive_root!(z2VRQa);
archive_root!(z2VPxB);
archive_root!(z2VMz7);
archive_root!(z2VV8c);
archive_root!(z2VNoR);
archive_root!(z2VLED);
archive_root!(z2VVds);
archive_root!(z2VP5n);
archive_root!(z2VWc6);
archive_root!(z2VXKH);
archive_root!(z2Vdss);
archive_root!(z2VN4g);
archive_root!(z2VfCM);
archive_root!(z2VbSK);
archive_root!(z2VYkT);
archive_root!(z2VN7W);
archive_root!(z2VTax);

pub enum ContractMarker {}
impl signal_frame::WireContract for ContractMarker { const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(match signal_frame::ContractId::try_new(11) { Ok(value) => value, Err(_) => panic!("contract ID is allocated") }, match signal_frame::WireRevision::try_new(2) { Ok(value) => value, Err(_) => panic!("wire revision is allocated") }); }

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError { #[error("failed to encode bound signal frame")] FrameEncode, #[error("failed to decode bound signal frame")] ArchiveDecode, #[error("unexpected signal frame body")] UnexpectedFrameBody, #[error("expected one request operation, found {found}")] OperationCount { found: usize } }

#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)] pub enum InputRoute { RetractInterfaceObservation, PushUpdate, ObserveInterfaceState, ListInterceptPolicies, FetchParkedRequests, PresentQuestion, ReplaceInterceptPolicy, CreateInterceptPolicy, CancelInterceptPolicy, ProposeEditedAnswer, AnswerQuestion, AnswerParkedRequest }
#[derive(Clone, Copy, Debug, PartialEq, Eq)] #[repr(u8)] pub enum OutputRoute { InterfaceObservationStarted, UpdateApplied, ParkedRequestAnswered, VerdictRecorded, AnswerProposalAccepted, QuestionAccepted, ParkedRequestsFetched, InterceptPolicyCancelled, InterceptPolicyReplaced, InterceptPoliciesListed, InterceptPolicyCreated, RequestRejected, InterfaceObservationClosed }

impl z2VbRz {
    pub fn route(&self) -> InputRoute { match self { Self::z2VSji(_) => InputRoute::RetractInterfaceObservation, Self::z2VZ1e(_) => InputRoute::PushUpdate, Self::z2VPTa(_) => InputRoute::ObserveInterfaceState, Self::z2VQdY(_) => InputRoute::ListInterceptPolicies, Self::z2VdSS(_) => InputRoute::FetchParkedRequests, Self::z2VRZk(_) => InputRoute::PresentQuestion, Self::z2VPiG(_) => InputRoute::ReplaceInterceptPolicy, Self::z2VVTE(_) => InputRoute::CreateInterceptPolicy, Self::z2VLt9(_) => InputRoute::CancelInterceptPolicy, Self::z2VPvP(_) => InputRoute::ProposeEditedAnswer, Self::z2VSKi(_) => InputRoute::AnswerQuestion, Self::z2Vccz(_) => InputRoute::AnswerParkedRequest } }
    pub fn wire_route(&self) -> signal_frame::WireRoute { signal_frame::WireRoute::new(signal_frame::RootCode::new(0), signal_frame::VariantCode::new(self.route() as u8)) }
    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame { let route = self.wire_route(); Frame::new(route, FrameBody::Request { exchange, request: signal_frame::Request::from_payload(self) }) }
    pub fn encode_request_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> { self.into_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode) }
}
impl z2VTVx {
    pub fn route(&self) -> OutputRoute { match self { Self::z2Vezf(_) => OutputRoute::InterfaceObservationStarted, Self::z2VUvB(_) => OutputRoute::UpdateApplied, Self::z2VPmd(_) => OutputRoute::ParkedRequestAnswered, Self::z2VQQe(_) => OutputRoute::VerdictRecorded, Self::z2VbGs(_) => OutputRoute::AnswerProposalAccepted, Self::z2VLQ9(_) => OutputRoute::QuestionAccepted, Self::z2VdDM(_) => OutputRoute::ParkedRequestsFetched, Self::z2VTEN(_) => OutputRoute::InterceptPolicyCancelled, Self::z2VM7k(_) => OutputRoute::InterceptPolicyReplaced, Self::z2VT7m(_) => OutputRoute::InterceptPoliciesListed, Self::z2Vc6v(_) => OutputRoute::InterceptPolicyCreated, Self::z2VVxR(_) => OutputRoute::RequestRejected, Self::z2VUaf(_) => OutputRoute::InterfaceObservationClosed } }
    pub fn wire_route(&self) -> signal_frame::WireRoute { signal_frame::WireRoute::new(signal_frame::RootCode::new(1), signal_frame::VariantCode::new(self.route() as u8)) }
    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame { let route = self.wire_route(); let reply = signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self))); Frame::new(route, FrameBody::Reply { exchange, reply }) }
    pub fn encode_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> { self.into_reply_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode) }
}
impl signal_frame::RequestPayload for z2VbRz {}
impl signal_frame::SignalOperationHeads for z2VbRz { const HEADS: &'static [&'static str] = &["RetractInterfaceObservation", "PushUpdate", "ObserveInterfaceState", "ListInterceptPolicies", "FetchParkedRequests", "PresentQuestion", "ReplaceInterceptPolicy", "CreateInterceptPolicy", "CancelInterceptPolicy", "ProposeEditedAnswer", "AnswerQuestion", "AnswerParkedRequest"]; }
impl signal_frame::LogVariant for z2VbRz { fn log_variant(&self) -> u64 { let route = self.wire_route(); u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8) } }

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2VbRz, z2VTVx>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2VbRz, z2VTVx>;
pub type Request = signal_frame::Request<z2VbRz>;
pub type ReplyEnvelope = signal_frame::Reply<z2VTVx>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2VbRz>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> { Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode) }
    pub fn decode_single_request(bytes: &[u8]) -> Result<(signal_frame::ExchangeIdentifier, z2VbRz), SignalFrameError> { match Self::decode_frame(bytes)?.into_body() { FrameBody::Request { exchange, request } => { let found = request.payloads().len(); if found != 1 { return Err(SignalFrameError::OperationCount { found }); } Ok((exchange, request.payloads.into_head())) }, _ => Err(SignalFrameError::UnexpectedFrameBody) } }
}

