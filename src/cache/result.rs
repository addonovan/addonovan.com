
pub enum CacheResult<T, E> {
    Err(E),
    New(T),
    Cached(T),
}

impl<T, E> CacheResult<T, E> {

    pub fn into_result(self) -> Result<T, E> {
        match self {
            CacheResult::New(item)
            | CacheResult::Cached(item) => Ok(item),
            CacheResult::Err(err) => Err(err),
        }
    }

}
