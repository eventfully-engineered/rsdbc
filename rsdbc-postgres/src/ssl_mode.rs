
#[derive(Debug, Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum SslMode {
    /// Do not use TLS.
    Disable,
    Allow,
    /// Attempt to connect with TLS but allow sessions without.
    Prefer,
    /// Require the use of TLS.
    Require,
    VerifyCA,
    VerifyFull
}
