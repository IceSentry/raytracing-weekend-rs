pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    match value {
        value if value < min => min,
        value if value > max => max,
        _ => value,
    }
}
