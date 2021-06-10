//! Defines the supported SPF Mechanisms  

/// This enum defines the possible mechanisms.
#[derive(Debug, Clone, PartialEq)]
pub enum MechanismKind {
    /// Represents a Mechanism of type redirect=  
    /// If this is present, no other mechanism should be present.  
    Redirect,
    /// Represents a Mechanism of type a
    /// # Possible Values:  
    /// ```bash
    /// a   
    /// a/24  
    /// a:example.com/24 (/prefix is optional)
    /// ```
    A,
    /// Represents a Mechanism of type mx
    /// Possible values follow the same loyout as for [`A`](MechanismKind::A)
    MX,
    /// Represents a Mechanism of type include:
    /// **Note**: There should only be a maximum of 10 Includes.
    Include,
    /// Represents a Mechanism of type ip4:  
    /// # Example Values:  
    /// ```text
    /// ip4:192.168.11.0/24  
    /// ip4:10.10.1.1
    /// ```
    IpV4,
    /// Represents a Mechanism of type ip6:
    IpV6,
    /// Represents a Mechanism of type ptr: Note: This is rarely use.
    Ptr,
    /// Represents a Mechanism of type exists:
    Exists,
    /// Represents a Mechanism of type all
    All,
}

impl MechanismKind {
    /// Returns `true` if the mechanism is [`Redirect`](MechanismKind::Redirect).
    pub fn is_redirect(&self) -> bool {
        matches!(self, Self::Redirect)
    }
    /// Returns `true` if the mechanism is [`A`](MechanismKind::A).
    pub fn is_a(&self) -> bool {
        matches!(self, Self::A)
    }
    /// Returns `true` if the mechanism is [`MX`](MechanismKind::MX).
    pub fn is_mx(&self) -> bool {
        matches!(self, Self::MX)
    }
    /// Returns `true` if the mechanism is [`Include`](MechanismKind::Include).
    pub fn is_include(&self) -> bool {
        matches!(self, Self::Include)
    }
    /// Returns `true` if the mechanism is [`IpV4`](MechanismKind::IpV4).
    pub fn is_ip_v4(&self) -> bool {
        matches!(self, Self::IpV4)
    }
    /// Returns `true` if the mechanism is [`IpV6`](MechanismKind::IpV6).
    pub fn is_ip_v6(&self) -> bool {
        matches!(self, Self::IpV6)
    }
    /// Returns `true` if the mechanism is [`Ptr`](MechanismKind::Ptr).
    pub fn is_ptr(&self) -> bool {
        matches!(self, Self::Ptr)
    }
    /// Returns `true` if the mechanism is [`Exists`](MechanismKind::Exists).
    pub fn is_exists(&self) -> bool {
        matches!(self, Self::Exists)
    }
    /// Returns `true` if the mechanism is [`All`](MechanismKind::All).
    pub fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }
}

impl Default for MechanismKind {
    fn default() -> Self {
        Self::A
    }
}
