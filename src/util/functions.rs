use std::time::Instant;

pub fn template<S>(item: S) -> String
    where S: AsRef<str> {
    use constants::TEMPLATE_DIR;
    format!("{}/{}", TEMPLATE_DIR, item.as_ref())
}

pub fn style<S>(item: S) -> String
    where S: AsRef<str> {
    use constants::STYLE_DIR;
    format!("{}/{}", STYLE_DIR, item.as_ref())
}

pub fn current_year() -> i32 {
    use chrono::Local;
    use chrono::Datelike;
    Local::now().date().year()
}

pub fn elapsed(start: Instant) -> u64 {
    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs();
    let nanos = elapsed.subsec_nanos() as u64;

    (seconds * 1000000000) + nanos
}
