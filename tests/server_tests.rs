use architecture::net::client::Client;
use architecture::net::protocol::message::{ClientMessage, ServerMessage};
use architecture::net::server::Server;
use architecture::net::transport::endpoint::ClientEndpoint;
use architecture::net::transport::loopback::loopback;

#[test]
fn server_responds_to_connect() {
    let (client_endpoint, server_endpoint) = loopback();
    let mut client = Client::new(client_endpoint);
    let mut server = Server::new(server_endpoint);

    client.mut_endpoint().send(ClientMessage::Connect).unwrap();
    server.tick();

    assert!(matches!(
        client.mut_endpoint().recv(),
        Some(ServerMessage::Connected { client_id: 1 })
    ));
    assert!(server.client_connected());
}

#[test]
fn server_sends_snapshots_when_connected() {
    let (client_endpoint, server_endpoint) = loopback();
    let mut client = Client::new(client_endpoint);
    let mut server = Server::new(server_endpoint);

    client.mut_endpoint().send(ClientMessage::Connect).unwrap();
    server.tick();

    let _ = client.mut_endpoint().recv(); // Connected

    match client.mut_endpoint().recv() {
        Some(ServerMessage::Snapshot(snapshot)) => assert_eq!(snapshot.snapshot_tick(), 1),
        _ => panic!("Expected snapshot!"),
    };

    server.tick();

    match client.mut_endpoint().recv() {
        Some(ServerMessage::Snapshot(snapshot)) => assert_eq!(snapshot.snapshot_tick(), 2),
        _ => panic!("Expected snapshot!"),
    };
}

#[test]
fn server_stops_snapshots_on_disconnect() {
    let (client_endpoint, server_endpoint) = loopback();
    let mut client = Client::new(client_endpoint);
    let mut server = Server::new(server_endpoint);

    client.mut_endpoint().send(ClientMessage::Connect).unwrap();
    server.tick();

    let _ = client.mut_endpoint().recv(); // Connected
    let _ = client.mut_endpoint().recv(); // Snapshot

    client
        .mut_endpoint()
        .send(ClientMessage::Disconnect)
        .unwrap();
    server.tick();

    assert!(matches!(
        client.mut_endpoint().recv(),
        Some(ServerMessage::Disconnected)
    ));
    assert!(client.mut_endpoint().recv().is_none());
    assert!(!server.client_connected());
}

#[test]
fn server_ignores_ticks_without_client() {
    let (client_endpoint, server_endpoint) = loopback();
    let mut client = Client::new(client_endpoint);
    let mut server = Server::new(server_endpoint);

    server.tick();
    server.tick();
    server.tick();

    assert!(client.mut_endpoint().recv().is_none());
    assert_eq!(server.current_tick(), 0);
}
