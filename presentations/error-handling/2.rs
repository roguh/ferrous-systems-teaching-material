fn this_can_fail(succeeds: bool) -> Result<String, String> {
    if succeeds {
        Ok(String::from("Success"))
    } else {
        Err(String::from("Error"))
    }
}

fn main() {
    this_can_fail(true);
}