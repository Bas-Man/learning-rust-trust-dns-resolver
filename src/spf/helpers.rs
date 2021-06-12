use crate::spf::kinds;
use crate::spf::mechanism::Mechanism;
use crate::spf::qualifier::Qualifier;
use lazy_static::lazy_static;
use regex::Regex;

pub(crate) const MECHANISM_A_PATTERN: &str =
    r"^(?P<qualifier>[+?~-])?a[:]{0,1}(?P<mechanism>[/]{0,1}.+)?";
pub(crate) const MECHANISM_MX_PATTERN: &str =
    r"^(?P<qualifier>[+?~-])?mx[:]{0,1}(?P<mechanism>[/]{0,1}.+)?";
pub(crate) const MECHANISM_PTR_PATTERN: &str =
    r"^(?P<qualifier>[+?~-])?ptr[:]{0,1}(?P<mechanism>.+)?";

pub(crate) fn capture_matches(
    string: &str,
    kind: kinds::MechanismKind,
) -> Option<Mechanism<String>> {
    lazy_static! {
        static ref A_RE: Regex = Regex::new(MECHANISM_A_PATTERN).unwrap();
        static ref MX_RE: Regex = Regex::new(MECHANISM_MX_PATTERN).unwrap();
        static ref PTR_RE: Regex = Regex::new(MECHANISM_PTR_PATTERN).unwrap();
    }
    let caps = match kind {
        kinds::MechanismKind::A => A_RE.captures(string),
        kinds::MechanismKind::MX => MX_RE.captures(string),
        kinds::MechanismKind::Ptr => PTR_RE.captures(string),
        _ => unreachable!(),
    };
    let qualifier_char: char;
    let mut qualifier_result: Qualifier = Qualifier::Pass;
    let mechanism_string: String;
    let mechanism;
    match caps {
        None => return None,
        Some(caps) => {
            // There was a match
            if caps.name("qualifier").is_some() {
                qualifier_char = caps
                    .name("qualifier")
                    .unwrap()
                    .as_str()
                    .chars()
                    .nth(0)
                    .unwrap();
                qualifier_result = char_to_qualifier(qualifier_char);
            };
            if caps.name("mechanism").is_some() {
                mechanism_string = caps.name("mechanism").unwrap().as_str().to_string();
                mechanism = Mechanism::new(kind, qualifier_result, (*mechanism_string).to_string());
            } else {
                //mechanism_string = match kind {
                //    kinds::MechanismKind::A => "a".to_string(),
                //    kinds::MechanismKind::MX => "mx".to_string(),
                //    kinds::MechanismKind::Ptr => "ptr".to_string(),
                //    _ => unreachable!(),
                //};
                // These values will be blank.
                mechanism_string = String::new();
                mechanism = Mechanism::new(kind, qualifier_result, mechanism_string);
            }

            Some(mechanism)
        }
    }
}

pub(crate) fn char_to_qualifier(c: char) -> Qualifier {
    match c {
        '+' => return Qualifier::Pass,
        '-' => return Qualifier::Fail,
        '~' => return Qualifier::SoftFail,
        '?' => return Qualifier::Neutral,
        _ => return Qualifier::Pass, // This should probably be Neutral
    }
}
