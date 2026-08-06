use dotos::{DotosEncode, DotosSource};
use signal_mentci::*;

fn exchange() -> signal_frame::ExchangeIdentifier {
    signal_frame::ExchangeIdentifier::new(
        signal_frame::SessionEpoch::new(1),
        signal_frame::ExchangeLane::Connector,
        signal_frame::LaneSequence::first(),
    )
}

#[test]
fn authority_projected_request_round_trips_through_dotos_and_the_bound_frame() {
    let request = z2VbRz::z2VSji(z2VTuM::new("fixture".to_owned()));
    let text = request.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VbRz>()
            .expect("request Dotos decodes"),
        request
    );
    let encoded = request
        .clone()
        .encode_request_frame(exchange())
        .expect("request frame encodes");
    let (decoded_exchange, decoded) =
        ContractMarker::decode_single_request(&encoded).expect("request frame decodes");
    assert_eq!(decoded_exchange, exchange());
    assert_eq!(decoded, request);
}

#[test]
fn authority_projected_reply_round_trips_through_dotos_and_archive_storage() {
    let reply = z2VTVx::z2Vezf(z2VVds {
        field_0: z2VTuM::new("fixture".to_owned()),
        field_1: z2VLED {
            field_0: z2VLa8::new(1),
            field_1: z2VWc6::z2VcGt(z2VL6F::new("fixture".to_owned())),
        },
    });
    let text = reply.to_dotos();
    assert_eq!(
        DotosSource::new(&text)
            .parse::<z2VTVx>()
            .expect("reply Dotos decodes"),
        reply
    );
    let archive = rkyv::to_bytes::<rkyv::rancor::Error>(&reply).expect("reply archives");
    let recovered =
        rkyv::from_bytes::<z2VTVx, rkyv::rancor::Error>(&archive).expect("reply recovers");
    assert_eq!(recovered, reply);
}
