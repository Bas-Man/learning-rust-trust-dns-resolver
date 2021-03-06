use ipnetwork::IpNetworkError;

/// A list of expected possible errors for SPF records.
#[derive(Debug, PartialEq)]
pub enum SpfError {
    /// Source is invalid, SPF struct was not created using `from_str()`
    InvalidSource,
    /// Source string length exceeds 255 Characters
    SourceLengthExceeded,
    /// Exceeds RFC lookup limit.
    LookupLimitExceeded,
    /// Source Spf String has not been parsed.
    HasNotBeenParsed,
    /// Only one white space is permitted between mechanisms.
    WhiteSpaceSyntaxError,
    /// Invalid SPF
    InvalidSPF,
    /// Redirect with `All` Mechanism
    RedirectWithAllMechanism,
    /// Network Address is not valid
    InvalidIPAddr(IpNetworkError),
}
impl std::fmt::Display for SpfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpfError::InvalidSource => write!(f, "Source string not valid."),
            SpfError::SourceLengthExceeded => write!(f, "Spf record exceeds 255 characters."),
            SpfError::LookupLimitExceeded => write!(f, "Too many DNS lookups."),
            SpfError::HasNotBeenParsed => write!(f, "Source string has not been parsed."),
            SpfError::WhiteSpaceSyntaxError => {
                write!(
                    f,
                    "Spf contains two or more consecutive whitespace characters."
                )
            }
            SpfError::InvalidSPF => write!(f, "Spf record is invalid."),
            SpfError::RedirectWithAllMechanism => {
                write!(f, "Redirect with unexpected 'All' Mechanism")
            }
            SpfError::InvalidIPAddr(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl From<IpNetworkError> for SpfError {
    fn from(err: IpNetworkError) -> Self {
        SpfError::InvalidIPAddr(err)
    }
}

impl std::error::Error for SpfError {}

impl SpfError {
    /// Returns `true` if the SpfError is any of those listed [`SpfError`](SpfError).
    pub fn is_spf_error(&self) -> bool {
        matches!(self, Self::InvalidSource)
            || matches!(self, Self::SourceLengthExceeded)
            || matches!(self, Self::LookupLimitExceeded)
            || matches!(self, Self::HasNotBeenParsed)
            || matches!(self, Self::InvalidSPF)
            || matches!(self, Self::RedirectWithAllMechanism)
            || matches!(self, Self::InvalidIPAddr(_))
    }
    /// Returns `true` if the SpfError indicates and Invalid Source error.
    pub fn is_invalid_source(&self) -> bool {
        matches!(self, Self::InvalidSource)
    }
    /// Returns `true` if the SpfError indictes source length exceeds 255 characters.
    pub fn is_source_length_exceeded(&self) -> bool {
        matches!(self, Self::SourceLengthExceeded)
    }
    /// Returns `true` if the SpfError indicates SPF contains more than 10 DNS lookups.
    pub fn is_lookup_limit_exceeded(&self) -> bool {
        matches!(self, Self::LookupLimitExceeded)
    }
    /// Returns `true` if the SpfError indicates source of Spf has not been parsed.
    pub fn is_has_not_been_parsed(&self) -> bool {
        matches!(self, Self::HasNotBeenParsed)
    }
    /// Returns `true` if the SpfError indicates this is an invalid Spf Record.
    pub fn is_invalid_spf(&self) -> bool {
        matches!(self, Self::InvalidSPF)
    }
    /// Returns `true` if the SpfError indicates the presents of `All` Mechanism
    pub fn is_redirect_with_all_mechanism(&self) -> bool {
        matches!(self, Self::RedirectWithAllMechanism)
    }
    /// Returns `true` if the SpfError indicates an Invalid IP Address
    pub fn is_invalid_ip_addr(&self) -> bool {
        matches!(self, Self::InvalidIPAddr(_))
    }
}

#[test]
fn is_any_spf_error() {
    let err = SpfError::InvalidSource;
    assert_eq!(err.is_spf_error(), true);
}
#[test]
fn is_invalid_source() {
    let err = SpfError::InvalidSource;
    assert_eq!(err.is_invalid_source(), true);
}
#[test]
fn is_source_length_exceeded() {
    let err = SpfError::SourceLengthExceeded;
    assert_eq!(err.is_source_length_exceeded(), true);
}
#[test]
fn is_lookup_limit_exceeded() {
    let err = SpfError::LookupLimitExceeded;
    assert_eq!(err.is_lookup_limit_exceeded(), true)
}
#[test]
fn is_has_not_been_parsed() {
    let err = SpfError::HasNotBeenParsed;
    assert_eq!(err.is_has_not_been_parsed(), true)
}
#[test]
fn is_invalid_spf() {
    let err = SpfError::InvalidSPF;
    assert_eq!(err.is_invalid_spf(), true)
}
#[test]
fn is_redirect_with_all_mechanism() {
    let err = SpfError::RedirectWithAllMechanism;
    assert_eq!(err.is_redirect_with_all_mechanism(), true)
}
#[test]
fn is_invalid_ip_addr() {
    let bad_ip = "203.32.160.0/33"
        .parse::<ipnetwork::IpNetwork>()
        .unwrap_err();
    let err = SpfError::InvalidIPAddr(bad_ip);
    assert_eq!(err.is_invalid_ip_addr(), true)
}
