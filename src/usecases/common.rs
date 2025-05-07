#[derive(Debug)]
pub struct FatalError(pub anyhow::Error);

impl<E> From<E> for FatalError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
