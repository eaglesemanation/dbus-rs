use dbus::ffidisp::Connection;
use dbus::message::MatchRule;

// This programs implements the equivalent of running the "dbus-monitor" tool
fn main() {
    // First open up a connection to the session bus.
    let conn = Connection::new_session().expect("D-Bus connection failed");

    // Second create a rule to match messages we want to receive; in this example we add no
    // further requirements, so all messages will match
    let mut rule = MatchRule::new();
    rule.eavesdrop = true; // this lets us eavesdrop on *all* session messages, not just ours

    // Start matching
    conn.add_match(&rule.match_str()).expect("add_match failed");

    // Loop and print out all messages received as they come.
    // Some can be quite large, e.g. if they contain embedded images..
    loop {
        if let Some(msg) = conn.incoming(5000).next() {
            println!("{:?}", msg);
        }
    }
}
