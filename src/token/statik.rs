use super::Lifetime;

/// A static, non-expiring token.
#[derive(Debug)]
pub struct Static;

impl Lifetime for Static {
    fn expired(&self) -> bool { false }
}
