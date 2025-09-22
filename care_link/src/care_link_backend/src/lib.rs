#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
