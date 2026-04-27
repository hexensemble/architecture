use architecture::engine::logging::*;
use architecture::net::config::*;
use architecture::net::server_sim::*;
use architecture::net::stepper::*;
use architecture::net::transport::*;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging("dedicated_server.log".to_string())?;

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:27960".to_string());

    // Create server

    let (mut server, mut server_transport, server_addr) = create_server(addr.clone())?;

    // Create server shutdown handler

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        log::info!("[Dedicated Server] Shutting down server");
        running_clone.store(false, Ordering::SeqCst);
    })?;

    // Start server

    log::info!("[Dedicated Server] Starting server");

    let mut sim = ServerSim::default();
    sim.reset();

    log::info!("[Dedicated Server] Listening on: {}", server_addr);

    let fixed_dt = sim.fixed_dt();
    let mut server_stepper = ServerStepper::new(fixed_dt, MAX_STEPS_PER_FRAME);

    // Update server

    while running.load(Ordering::SeqCst) {
        server_stepper.wait_and_run(|| {
            update_server(
                "Dedicated Server".to_string(),
                &mut server,
                &mut server_transport,
                &mut sim,
                fixed_dt,
            );
        });
    }

    // Shutdown server

    log::info!("[Dedicated Server] Disconnecting clients");
    server.disconnect_all();
    // Flush packets
    server_transport.send_packets(&mut server);
    log::info!("[Dedicated Server] Shutdown complete");

    Ok(())
}
