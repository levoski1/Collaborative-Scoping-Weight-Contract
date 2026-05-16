use wave_scoping::errors::Error;
use wave_scoping::wave_scoping::WaveScoping;

const VOTING_PERIOD: u64 = 1000;
const EMERGENCY_TIMELOCK: u64 = 5;
const MAX_WEIGHT: u64 = 100;
const POINTS_DIVISOR: u64 = 10;
const DECAY_RATE: u64 = 100;
const SLASH_MAX_BPS: u64 = 2000;
const SLASH_BURN_RATE: u64 = 5000;

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: wave-scoping <command> [args...]");
        eprintln!();
        eprintln!("Commands:");
        eprintln!("  create-wave <name> <duration_blocks>");
        eprintln!("  register <url>");
        eprintln!("  vote <caller> <url> <weight>");
        eprintln!("  fast-track <caller> <url> <reason>");
        eprintln!("  start-work <caller> <url>");
        eprintln!("  finalize <caller> <wave_id>");
        eprintln!("  adjust-points <caller> <url> <new_points> <reason>");
        eprintln!("  get-issue <url>");
        eprintln!("  advance-blocks <n>");
        eprintln!("  demo");
        return Ok(());
    }

    let mut hub = WaveScoping::new(
        "owner",
        VOTING_PERIOD,
        EMERGENCY_TIMELOCK,
        MAX_WEIGHT,
        POINTS_DIVISOR,
        DECAY_RATE,
        SLASH_MAX_BPS,
        SLASH_BURN_RATE,
    )?;

    match args[1].as_str() {
        "create-wave" if args.len() >= 4 => {
            let name = &args[2];
            let duration: u64 = args[3].parse().map_err(|_| Error::InvalidWeight)?;
            let id = hub.create_wave("owner", name, duration)?;
            println!("Created wave {}: {}", id, name);
        }
        "register" if args.len() >= 3 => {
            hub.register_issue("owner", &args[2])?;
            println!("Registered issue: {}", args[2]);
        }
        "vote" if args.len() >= 5 => {
            hub.vote_on_issue(&args[2], &args[3], args[4].parse().map_err(|_| Error::InvalidWeight)?)?;
            println!("Vote cast by {} on {}", args[2], args[3]);
        }
        "fast-track" if args.len() >= 5 => {
            hub.fast_track_issue(&args[2], &args[3], &args[4])?;
            println!("Fast-tracked issue: {}", args[3]);
        }
        "start-work" if args.len() >= 4 => {
            hub.start_work(&args[2], &args[3])?;
            println!("Work started by {} on {}", args[2], args[3]);
        }
        "finalize" if args.len() >= 4 => {
            let wave_id: u64 = args[3].parse().map_err(|_| Error::InvalidWeight)?;
            hub.finalize_wave(&args[2], wave_id)?;
            println!("Finalized wave {}", wave_id);
        }
        "adjust-points" if args.len() >= 6 => {
            let new_points: u64 = args[4].parse().map_err(|_| Error::InvalidWeight)?;
            hub.adjust_points(&args[2], &args[3], new_points, &args[5])?;
            println!("Points adjusted for {}: {}", args[3], new_points);
        }
        "get-issue" if args.len() >= 3 => {
            let (weight, points, emergency, contributor) = hub.get_issue(&args[2])?;
            println!(
                "Issue {}: weight={}, points={}, emergency={}, contributor={:?}",
                args[2], weight, points, emergency, contributor
            );
        }
        "advance-blocks" if args.len() >= 3 => {
            let n: u64 = args[2].parse().map_err(|_| Error::InvalidWeight)?;
            hub.advance_blocks(n);
            println!("Advanced {} blocks to {}", n, hub.block_number);
        }
        "demo" => run_demo(&mut hub)?,
        _ => {
            eprintln!("Unknown command or incorrect arguments");
        }
    }

    Ok(())
}

fn run_demo(hub: &mut WaveScoping) -> Result<(), Error> {
    println!("=== WaveScoping Demo ===");
    println!();

    let alice = "alice";
    let bob = "bob";
    let charlie = "charlie";

    let wave_id = hub.create_wave("owner", "Sprint 2026.1", VOTING_PERIOD)?;
    println!("1. Created wave: id={}, name=Sprint 2026.1", wave_id);

    hub.register_issue("owner", "https://github.com/owner/repo/issues/1")?;
    hub.register_issue("owner", "https://github.com/owner/repo/issues/2")?;
    println!("2. Registered issues 1 and 2");

    hub.vote_on_issue(alice, "https://github.com/owner/repo/issues/1", 30)?;
    hub.vote_on_issue(bob, "https://github.com/owner/repo/issues/1", 20)?;
    hub.vote_on_issue(charlie, "https://github.com/owner/repo/issues/2", 50)?;
    println!("3. Votes cast: Alice(30), Bob(20) on issue 1; Charlie(50) on issue 2");

    let (w1, p1, _, _) = hub.get_issue("https://github.com/owner/repo/issues/1")?;
    let (w2, p2, _, _) = hub.get_issue("https://github.com/owner/repo/issues/2")?;
    println!("4. Issue 1: weight={}, points={}", w1, p1);
    println!("   Issue 2: weight={}, points={}", w2, p2);

    hub.start_work(alice, "https://github.com/owner/repo/issues/1")?;
    println!("5. Alice started work on issue 1");

    hub.finalize_wave("owner", wave_id)?;
    println!("6. Finalized wave {}", wave_id);

    println!();
    println!("=== Reputation Balances ===");
    println!("Alice:   {}", hub.reputation_manager.balance_of(alice));
    println!("Bob:     {}", hub.reputation_manager.balance_of(bob));
    println!("Charlie: {}", hub.reputation_manager.balance_of(charlie));

    println!();
    println!("=== Events Log ===");
    let mut all_events: Vec<String> = Vec::new();
    for event in &hub.events {
        all_events.push(format!("{:?}", event));
    }
    for event in &hub.reputation_manager.events {
        all_events.push(format!("{:?}", event));
    }
    for event in &hub.emergency_scoping.events {
        all_events.push(format!("{:?}", event));
    }
    for event in &hub.slashing_manager.events {
        all_events.push(format!("{:?}", event));
    }
    for ev in all_events {
        println!("  - {}", ev);
    }

    Ok(())
}
