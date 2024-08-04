//! # duolingo-query
//!
//! Grab user information on duolingo easily

pub mod prelude;
pub use crate::prelude::duolingo::*;

#[cfg(test)]
mod duolingo_tests {
    use crate::Duolingo;

    #[tokio::test]
    async fn api_test() -> anyhow::Result<()> {
        // Query information about a duolingo user
        let duo = Duolingo::query("christi3").await?;

        // Print information about users
        println!("{}", "-".repeat(10));
        println!("Streak: {}", duo.get_streak());
        println!("Uid: {}", duo.get_id());

        // Dates are unix timestamps
        println!("Creation date: {}", duo.get_creation_date());

        // Print information about the user's streaks
        duo.get_courses().iter().for_each(|course| {
            println!("{}", "-".repeat(10));
            println!("Course: {}", course.get_title());
            println!("Course XP: {}", course.get_xp());
            println!("Course Lang: {}", course.get_learning_language());
        });

        println!("{}", "-".repeat(10));
        println!("Total XP: {}", duo.get_total_xp());
        println!("Username: {}", duo.get_username());
        println!("Learning language: {}", duo.get_learning_language());
        println!("Has Duolingo Plus: {}", duo.has_plus());
        println!("{}", "-".repeat(10));

        Ok(())
    }
}
