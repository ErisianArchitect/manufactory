#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct LockoutKey;

/// Used to restrict user access.
/// ```
/// // this function can only be called by Manufactory.
/// fn use_lockout(lock: Lockout, data: String) {
///     // ...
/// }
/// // This type can only be created by Manufactory.
/// pub struct Locked(Lockout, String);
/// ```
pub struct Lockout(LockoutKey);

#[inline]
#[must_use]
pub(crate) const fn lock() -> Lockout {
    Lockout(LockoutKey)
}