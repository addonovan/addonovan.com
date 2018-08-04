
pub trait Decorator<I, O>
{
    fn apply(&self, input: I) -> O;
}
