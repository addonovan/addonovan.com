use std::time::Instant;

pub fn template<S>(item: S) -> String
    where S: AsRef<str> {
    use constants::TEMPLATE_DIR;
    format!("{}/{}", TEMPLATE_DIR, item.as_ref())
}

pub fn style<S>(item: S) -> String
    where S: AsRef<str> {
    use constants::STYLE_LINK;
    format!("{}/{}", STYLE_LINK, item.as_ref())
}

pub fn current_date() -> (i32, u32, u32) {
    use chrono::Local;
    use chrono::Datelike;
    let date = Local::now().date();
    (date.year(), date.month(), date.day())
}

pub fn elapsed(start: Instant) -> u64 {
    let elapsed = start.elapsed();
    let seconds = elapsed.as_secs();
    let nanos = elapsed.subsec_nanos() as u64;

    (seconds * 1000000000) + nanos
}
