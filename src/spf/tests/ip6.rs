#[cfg(test)]

mod parse {

    use crate::spf::Spf;

    #[test]
    fn test_ip6_valid() {
        let input = "v=spf1 ip6:2001:4860:4000::/36 ~all";

        let spf: Spf = input.parse().unwrap();
        assert!(spf.ip6().is_some());
        assert_eq!(spf.ip6().unwrap()[0].qualifier().is_pass(), true);
        assert_eq!(spf.ip6().unwrap()[0].raw(), "2001:4860:4000::/36");
        assert_eq!(spf.ip6().unwrap()[0].to_string(), "ip6:2001:4860:4000::/36");
    }
}
