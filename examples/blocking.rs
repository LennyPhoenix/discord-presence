use discord_presence::{Client, Event};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let mut drpc = Client::new(1003450375732482138);

    let drpc_thread = drpc.start();

    drpc.block_until_event(Event::Ready)?;

    assert!(Client::is_ready());

    // Set the activity
    drpc.set_activity(|act| act.state("rusting frfr"))?;

    ctrlc::set_handler(move || {
        println!("Exiting...");
        drpc.clear_activity().unwrap();
        std::process::exit(0);
    })?;

    if let Some(err) = drpc_thread.join().err() {
        dbg!(err);
    }

    Ok(())
}
