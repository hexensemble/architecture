use architecture::net::protocol::message::{ClientMessage, ServerMessage};
use architecture::net::protocol::snapshot::ServerWorldSnapshot;
use architecture::net::transport::endpoint::{ClientEndpoint, ServerEndpoint};
use architecture::net::transport::loopback::loopback;

#[test]
fn client_to_server() {
    let (mut client_endpoint, mut server_endpoint) = loopback();

    client_endpoint.send(ClientMessage::Connect).unwrap();

    let msg = server_endpoint.recv();
    assert!(matches!(msg, Some(ClientMessage::Connect)));
    assert!(server_endpoint.recv().is_none());
}

#[test]
fn server_to_client() {
    let (mut client_endpoint, mut server_endpoint) = loopback();

    server_endpoint
        .send(ServerMessage::Connected { client_id: 666 })
        .unwrap();

    let msg = client_endpoint.recv();
    assert!(matches!(
        msg,
        Some(ServerMessage::Connected { client_id: 666 })
    ));
    assert!(client_endpoint.recv().is_none());
}

#[test]
fn round_trip() {
    let (mut client_endpoint, mut server_endpoint) = loopback();

    client_endpoint.send(ClientMessage::Connect).unwrap();
    assert!(matches!(
        server_endpoint.recv(),
        Some(ClientMessage::Connect)
    ));

    server_endpoint
        .send(ServerMessage::Connected { client_id: 666 })
        .unwrap();
    assert!(matches!(
        client_endpoint.recv(),
        Some(ServerMessage::Connected { client_id: 666 })
    ));
}

#[test]
fn multiple_messages_preserve_order() {
    let (mut client_endpoint, mut server_endpoint) = loopback();

    client_endpoint.send(ClientMessage::Connect).unwrap();
    client_endpoint.send(ClientMessage::Disconnect).unwrap();

    assert!(matches!(
        server_endpoint.recv(),
        Some(ClientMessage::Connect)
    ));
    assert!(matches!(
        server_endpoint.recv(),
        Some(ClientMessage::Disconnect)
    ));
    assert!(server_endpoint.recv().is_none());
}

#[test]
fn snapshot_round_trip() {
    let (mut client_endpoint, mut server_endpoint) = loopback();

    let snapshot = ServerWorldSnapshot::default();

    server_endpoint
        .send(ServerMessage::Snapshot(snapshot))
        .unwrap();

    match client_endpoint.recv() {
        Some(ServerMessage::Snapshot(snap)) => assert_eq!(snap.snapshot_tick(), 666),
        _ => panic!("Expected snapshot!"),
    }
}
