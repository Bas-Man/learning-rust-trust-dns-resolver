use trust_dns_resolver::config::*;
use trust_dns_resolver::error::ResolveResult;
use trust_dns_resolver::lookup::MxLookup;
use trust_dns_resolver::Resolver;

/// Contains host preference and exchange host name
#[derive(Clone)]
pub struct MxHost {
    preference: u16,
    exchange: String,
}

impl MxHost {
    pub fn new(preference: u16, exchange: String) -> Self {
        Self {
            preference,
            exchange,
        }
    }
    pub fn preference(&self) -> u16 {
        self.preference
    }
    pub fn exchange(&self) -> &String {
        &self.exchange
    }
}
#[cfg(test)]
mod test {

    use super::MxHost;

    #[test]
    fn mx_host() {
        let mx_host = MxHost::new(5, String::from("alt1.aspmx.l.google.com."));
        // Test preference
        assert_eq!(mx_host.preference, 5);
        assert_eq!(mx_host.preference(), 5);
        // Test Exchange
        assert_eq!(mx_host.exchange, "alt1.aspmx.l.google.com.");
        assert_eq!(mx_host.exchange(), "alt1.aspmx.l.google.com.");
    }
}
/// Contains the destination domain and a list of its MX Hosts
#[derive(Clone)]
pub struct DomainMxServers {
    domain: String,
    mx_hosts: Option<Vec<MxHost>>,
}

impl DomainMxServers {
    pub fn new(domain: &String) -> Self {
        Self {
            domain: domain.clone(),
            mx_hosts: None,
        }
    }
    pub fn new_with_mx(domain: &String, mx_hosts: Option<Vec<MxHost>>) -> Self {
        Self {
            domain: domain.clone(),
            mx_hosts,
        }
    }
    pub fn domain(&self) -> &String {
        &self.domain
    }
    pub fn mx_hosts(&self) -> &Option<Vec<MxHost>> {
        &self.mx_hosts
    }
    pub fn parse(&mut self, mx_lookup: &ResolveResult<MxLookup>) {
        match mx_lookup {
            Err(_) => {}
            Ok(mx_lookup) => {
                let mut mx_records: Vec<MxHost> = Vec::new();
                let records = mx_lookup.iter();
                for mx in records {
                    mx_records.push(MxHost::new(mx.preference(), mx.exchange().to_string()));
                }
                self.mx_hosts = Some(mx_records);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::DomainMxServers;
    use super::MxHost;

    #[test]
    fn example_com_has_no_mx() {
        let domain = DomainMxServers::new(&String::from("example.com"));
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.domain(), "example.com");
    }
    #[test]
    fn example_com_has_mx() {
        let mut mx_hosts = Vec::new();
        mx_hosts.push(MxHost::new(5, String::from("alt1.aspmx.l.google.com.")));
        mx_hosts.push(MxHost::new(10, String::from("alt2.aspmx.l.google.com")));
        let domain = DomainMxServers::new_with_mx(&String::from("example.com"), Some(mx_hosts));
        assert_eq!(domain.domain, "example.com");
        assert_eq!(domain.mx_hosts.is_some(), true);
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
