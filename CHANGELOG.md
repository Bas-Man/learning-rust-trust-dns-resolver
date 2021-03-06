0.2.3 2021-12-09
================

- Add `#[cfg(feature = warn-dns)]` to code documentation to remove warnings and errors when feature is not enabled.

0.2.2 2021-12-09
================

- Add Feature (warn-dns)  
  With this feature enabled any malformed DNS records will cause `has_warnings()` to be set to `true`. Their values can then be accessed using `warnings()`. Even though they are invalid. No error will be generated.
  
0.2.1 2021-11-03
================

- Documentation Update


0.2.0 2021-11-02 
================

Breaking Changes.
================================================================

- as_spf() removed. Replaced with a *Display* trait. Use `to_string()`
- .parse() removed. Replaced with *FromStr* trait. Use `parse::<Spf>()`

Changes
=======

- Bump Version to 0.2.0
- Add this CHANGELOG.md file.
- Complete re-orginisation of the crate's module layout.
  - Note that spf::{qualifier::Qualifier, kinds::MechanismKind, mechanism::Mechanism} no longer exist. These can now be accessed more simply as: 
    -  mechanism::{Qualifer, Kind, Mechanism} 
- Correct license reference to correctly show as MIT license
- Introduced ability to build SPF records programmatically. 
  - See examples/build-new-spf.rs
- Implemented **Display** trait for structs *Spf* and *Mechanism*
  - This also depreciates the previously implemented *as_spf()* and *string()* methods.
- implemented **FromStr** trait for Struct *Spf*
- Implemented Errors for `Mechanism`
- Improved regular expressions to handle more strings when parsing.
