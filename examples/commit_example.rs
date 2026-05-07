use chrono::DateTime;
use semver_common::Commit;

fn main() {
    let c = String::from(
        "490049bf36b19b30d23b4be5a4u94f71b5c6475c
    Author: Some Author <myemail@email.com>
    Date:   Tue Apr 14 17:35:15 2026 -0400
    
        feat: added feature to get commit list
    ",
    );
    let commit = Commit::new_from_commit(c).expect("Commit could not be instantiated during test.");
    assert_eq!(commit.id(), "490049bf36b19b30d23b4be5a4u94f71b5c6475c");
    assert_eq!(commit.author(), "Some Author <myemail@email.com>");
    assert_eq!(
        commit.timestamp(),
        &DateTime::parse_from_str("Tue Apr 14 17:35:15 2026 -0400", "%a %b %d %H:%M:%S %Y %z")
            .unwrap()
    );
    assert_eq!(commit.message(), "feat: added feature to get commit list");
}
