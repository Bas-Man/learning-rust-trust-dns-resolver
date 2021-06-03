#[cfg(test)]

mod a_capture {

    use crate::spf::helpers;
    use crate::spf::kinds;
    use crate::spf::mechanism::Mechanism;
    use regex::Regex;

    #[test]
    fn test_match_on_a_only() {
        let string = "a";
        let pattern = Regex::new(r"^(?P<qualifier>[+?~-])?a(?P<mechanism>[:/]{0,1}.+)?").unwrap();
        let option_test: Option<Mechanism<String>>;

        option_test = helpers::capture_matches(pattern, &string, kinds::MechanismKind::A);

        let test = option_test.unwrap();
        assert_eq!(test.is_pass(), true);
        assert_eq!(test.raw(), "a");
        assert_eq!(test.string(), "a");
    }
    #[test]
    fn test_match_on_a_colon() {
        let string = "-a:example.com";
        let pattern = Regex::new(r"^(?P<qualifier>[+?~-])?a(?P<mechanism>[:/]{0,1}.+)?").unwrap();
        let option_test: Option<Mechanism<String>>;

        option_test = helpers::capture_matches(pattern, &string, kinds::MechanismKind::A);

        let test = option_test.unwrap();
        assert_eq!(test.is_fail(), true);
        assert_eq!(test.raw(), ":example.com");
        assert_eq!(test.string(), "-a:example.com");
    }
    #[test]
    fn test_match_on_a_slash() {
        let string = "~a/24";
        let pattern = Regex::new(r"^(?P<qualifier>[+?~-])?a(?P<mechanism>[:/]{0,1}.+)?").unwrap();
        let option_test: Option<Mechanism<String>>;

        option_test = helpers::capture_matches(pattern, &string, kinds::MechanismKind::A);

        let test = option_test.unwrap();
        assert_eq!(test.is_softfail(), true);
        assert_eq!(test.raw(), "/24");
        assert_eq!(test.string(), "~a/24");
    }
    #[test]
    fn test_match_on_a_colon_slash() {
        let string = "+a:example.com/24";
        let pattern = Regex::new(r"^(?P<qualifier>[+?~-])?a(?P<mechanism>[:/]{0,1}.+)?").unwrap();
        let option_test: Option<Mechanism<String>>;

        option_test = helpers::capture_matches(pattern, &string, kinds::MechanismKind::A);

        let test = option_test.unwrap();
        assert_eq!(test.is_pass(), true);
        assert_eq!(test.raw(), ":example.com/24");
        assert_eq!(test.string(), "a:example.com/24");
        //assert!(test.kind.is_a());
    }
}
