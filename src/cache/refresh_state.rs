use std::io;

/// A multi-state which is used to signal what the effect of
/// [CacheEntry.refresh] was.
#[derive(Debug)]
pub enum RefreshState {
    /// The origin of this entry couldn't be refreshed
    #[allow(unused)]
    NotApplicable,

    /// We didn't need to refresh our cached value (but the origin supported it)
    Unnecessary,

    /// We successfully refreshed our cached value
    Success,

    /// We failed to refresh our cached value
    Failure(io::Error),
}

impl PartialEq for RefreshState {
    fn eq<'a>(&self, other: &'a RefreshState) -> bool {
        use self::RefreshState::*;

        match (self, other) {
            (NotApplicable, NotApplicable) => true,
            (Unnecessary, Unnecessary) => true,
            (Success, Success) => true,
            (Failure(_), Failure(_)) => true,
            _ => false,
        }
    }

    fn ne<'a>(&self, other: &'a RefreshState) -> bool {
        !self.eq(other)
    }
}
