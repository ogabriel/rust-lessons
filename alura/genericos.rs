pub fn main() {}

enum Option<T> {
    Some(T),
    None,
}

enum Result<S, E> {
    Ok(S),
    Err(E),
}
