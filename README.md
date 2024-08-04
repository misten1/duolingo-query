# duolingo-query
_____________
#### > A rust library to easily grab duolingo users' profile information through the unofficial duolingo api
_____________

### Examples

````rust
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
println!("Has Duolingo Plus: {}", duo.get_has_plus());
println!("{}", "-".repeat(10));
````

### Warning
> #### This library is sensitive to api changes and as such might break if duolingo updates their api