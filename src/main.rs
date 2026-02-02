mod brightness;
mod clock;
mod fans;
mod sysfs;
use std::time::{
    Duration,
    Instant,
};

// Bar is our text-based status bar.
// It heavily relies on String as means of abstraction
struct Bar<'a> {
    // title of the whole bar
    title: &'a str,
    // vector of (key, value)
    values: Vec<(&'a str, String)>,
}

impl<'a> std::fmt::Display for Bar<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)?;
        for (key, value) in &self.values {
            write!(f, " | {}: {}", key, value)?;
        }
        Ok(())
    }
}

struct Poller<'a> {
    key: &'a str,
    next_fire: Instant,
    interval: Duration,
    action: fn() -> String,
}

// poll is a event-loop polling mechanism for our status bar.
// Currently, dwmbar-rs only supports simple pollers.
fn poll(title: &str, mut pollers: Vec<Poller>) {
    let mut next = Instant::now(); // Next (i.e. earliest) call to poller
    let mut bar = Bar {
        title,
        values: pollers.iter().map(|t| (t.key, initializing())).collect(),
    };

    let mut lasts: Vec<String> =
        pollers.iter().map(|_| String::default()).collect();

    loop {
        let now = Instant::now();
        let mut updated = false;
        for (i, poller) in pollers.iter_mut().enumerate() {
            // Skip undued pollers
            if now < poller.next_fire {
                continue;
            }
            // New next_fire for poller
            poller.next_fire = now + poller.interval;
            if poller.next_fire < next {
                next = poller.next_fire;
            }
            // Only apply updates if field value changed from last
            let result = (poller.action)();
            if lasts[i] == result {
                continue;
            }

            updated = true;
            lasts[i] = result.clone();
            bar.values[i].1 = result;
        }
        if updated {
            println!("{}", bar);
        }

        // Sleep until then (no busy-waiting)
        std::thread::sleep(next - now);
    }
}

fn main() {
    let now = Instant::now();
    poll(
        "dwmbar-rs",
        vec![
            Poller {
                key: "fans",
                next_fire: now,
                interval: Duration::from_millis(500),
                action: fans::get,
            },
            Poller {
                key: "brightness",
                next_fire: now,
                interval: Duration::from_millis(500),
                action: brightness::get,
            },
            Poller {
                key: "clock",
                next_fire: now,
                interval: Duration::from_millis(500),
                action: || clock::get(clock::CLOCK_DEFAULT.to_owned())(),
            },
        ],
    );
}

fn initializing() -> String {
    String::from("initializing...")
}
