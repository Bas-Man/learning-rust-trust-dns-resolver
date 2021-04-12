use trust_dns_resolver::config::*;
use trust_dns_resolver::error::ResolveResult;
use trust_dns_resolver::lookup::MxLookup;
use trust_dns_resolver::Resolver;

/// Contains host priority and exchange host name
#[derive(Clone)]
pub struct MxHost {
    priority: u8,
    exchange: String,
}

impl MxHost {
    pub fn new(priority: u8, exchange: String) -> Self {
        Self { priority, exchange }
    }
    pub fn priority(&self) -> u8 {
        self.priority
    }
    pub fn priority_as_ref(&self) -> &u8 {
        &self.priority
    }
    pub fn exchange(&self) -> String {
        self.exchange.clone()
    }
    /// Returns a reference to the exchange string stored in MxHost
    pub fn exchange_as_ref(&self) -> &str {
        self.exchange.as_ref()
    }
}
#[cfg(test)]
mod test {

    use super::MxHost;
    use std::ptr;

    #[test]
    fn mx_host() {
        let mx_host = MxHost::new(5, String::from("alt1.aspmx.l.google.com."));
        let ex_ref = mx_host.exchange_as_ref();
        let pr_ref = mx_host.priority_as_ref();
        // Test Priority
        assert_eq!(mx_host.priority, 5);
        assert_eq!(mx_host.priority(), 5);
        assert_eq!(mx_host.priority_as_ref(), pr_ref);
        // Test Exchange
        assert_eq!(mx_host.exchange, "alt1.aspmx.l.google.com.");
        assert_eq!(mx_host.exchange(), "alt1.aspmx.l.google.com.");
        assert!(ptr::eq(mx_host.exchange_as_ref(), ex_ref));
    }
}
/// Contains the destination domain and a list of its MX Hosts
#[derive(Clone)]
pub struct DomainMxServers {
    domain: String,
    mx_hosts: Option<Vec<MxHost>>,
}

impl DomainMxServers {
    pub fn new(domain: String, mx_hosts: Option<Vec<MxHost>>) -> Self {
        Self { domain, mx_hosts }
    }
    pub fn new_none(domain: String) -> Self {
        DomainMxServers::new(domain, None)
    }
    pub fn domain(&self) -> String {
        self.domain.clone()
    }
    pub fn domain_as_ref(&self) -> &str {
        self.domain.as_ref()
    }
    pub fn mx_hosts_as_ref(&self) -> &Option<Vec<MxHost>> {
        &self.mx_hosts
    }
    pub fn mx_hosts(&self) -> Option<Vec<MxHost>> {
        self.mx_hosts.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::DomainMxServers;
    use super::MxHost;
    use std::ptr;

    #[test]
    fn example_com_has_no_mx() {
        let domain = DomainMxServers::new(String::from("example.com"), None);
        let domain_ref = domain.domain_as_ref();
        let mx_ref = domain.mx_hosts_as_ref();
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.domain(), "example.com");
        assert!(ptr::eq(domain.domain.as_ref(), domain_ref));
        assert!(ptr::eq(domain.mx_hosts_as_ref(), mx_ref));
    }
    #[test]
    fn test_new_none() {
        let domain = DomainMxServers::new_none(String::from("example.com"));
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.mx_hosts.is_none(), true);
    }
    #[test]
    fn example_com_has_mx() {
        let mut mx_hosts = Vec::new();
        mx_hosts.push(MxHost::new(5, String::from("alt1.aspmx.l.google.com.")));
        mx_hosts.push(MxHost::new(10, String::from("alt2.aspmx.l.google.com")));
        let domain = DomainMxServers::new(String::from("example.com"), Some(mx_hosts));
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.mx_hosts.is_some(), true);
        let mx2 = domain.mx_hosts();
        assert!(!ptr::eq(&mx2, domain.mx_hosts_as_ref()));
        assert!(!domain.mx_hosts.unwrap().is_empty());
    }
}

pub fn display_mx(mx_response: &ResolveResult<MxLookup>) {
    match mx_response {
        Err(_) => println!("No Records"),
        Ok(mx_response) => {
            let records = mx_response.iter();
            for record in records {
                println!("{} {}", record.preference(), record.exchange());
                let resolver =
                    Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();
                let lookup_response = resolver.lookup_ip(record.exchange().to_string().as_str());
                match lookup_response {
                    Err(_) => println!("This exchange host has not address."),
                    Ok(lookup_response) => {
                        let ip_addrs = lookup_response.iter();
                        for ip_addr in ip_addrs {
                            if ip_addr.is_ipv4() {
                                println!("   ip4: {}", ip_addr)
                            } else {
                                println!("   ip6: {}", ip_addr)
                            }
                        }
                    }
                }
            }
        }
    }
}
