use architecture::core::protocol::message::{ClientMessage, ServerMessage};
use architecture::core::transport::endpoint::ClientEndpoint;
use architecture::core::transport::loopback::loopback;
use architecture::server::server::Server;

#[test]
fn server_responds_to_connect() {
    let (mut client_endpoint, server_endpoint) = loopback();
    let mut server = Server::new(server_endpoint);

    client_endpoint.send(ClientMessage::Connect).unwrap();
    server.tick();

    assert!(matches!(
        client_endpoint.recv(),
        Some(ServerMessage::Connected { client_id: 1 })
    ));
    assert!(server.is_client_connected());
}

#[test]
fn server_sends_snapshots_when_connected() {
    let (mut client_endpoint, server_endpoint) = loopback();
    let mut server = Server::new(server_endpoint);

    client_endpoint.send(ClientMessage::Connect).unwrap();
    server.tick();

    let _ = client_endpoint.recv(); // Connected

    match client_endpoint.recv() {
        Some(ServerMessage::Snapshot(snapshot)) => assert_eq!(snapshot.tick, 1),
        _ => panic!("Expected snapshot!"),
    };

    server.tick();

    match client_endpoint.recv() {
        Some(ServerMessage::Snapshot(snapshot)) => assert_eq!(snapshot.tick, 2),
        _ => panic!("Expected snapshot!"),
    };
}

#[test]
fn server_stops_snapshots_on_disconnect() {
    let (mut client_endpoint, server_endpoint) = loopback();
    let mut server = Server::new(server_endpoint);

    client_endpoint.send(ClientMessage::Connect).unwrap();
    server.tick();

    let _ = client_endpoint.recv(); // Connected
    let _ = client_endpoint.recv(); // Snapshot

    client_endpoint.send(ClientMessage::Disconnect).unwrap();
    server.tick();

    assert!(matches!(
        client_endpoint.recv(),
        Some(ServerMessage::Disconnected)
    ));
    assert!(client_endpoint.recv().is_none());
    assert!(!server.is_client_connected());
}

#[test]
fn server_ignores_ticks_without_client() {
    let (mut client_endpoint, server_endpoint) = loopback();
    let mut server = Server::new(server_endpoint);

    server.tick();
    server.tick();
    server.tick();

    assert!(client_endpoint.recv().is_none());
    assert_eq!(server.current_tick(), 0);
}
