use decon_spf::mechanism::{Kind, Mechanism, ParsedMechanism, Qualifier};
use decon_spf::Spf;

fn main() {
    let mut spf1 = Spf::new();
    spf1.set_v1();
    let ip_m_1 = ParsedMechanism::new("+ip4:203.32.160.0/24").unwrap();
    let ip_m_2 = ParsedMechanism::new("+ip4:203.32.166.0/24").unwrap();
    let mx = ParsedMechanism::new("mx").unwrap();

    spf1.append_ip_mechanism(ip_m_1.network());
    spf1.append_ip_mechanism(ip_m_2.network());
    spf1.append_mechanism(mx.txt());

    spf1.append_mechanism("a:test.com".parse().unwrap());

    println!("New spf 1: >{}<", spf1);
    assert_eq!(
        spf1.to_string(),
        "v=spf1 a:test.com mx ip4:203.32.160.0/24 ip4:203.32.166.0/24"
    );

    let mut spf2 = Spf::new();
    spf2.set_v1();
    let ip = "203.32.166.0/24".parse().unwrap();
    spf2.append_ip_mechanism(Mechanism::new_ip(Qualifier::Pass, ip));

    println!("\nNew spf 2: >{}<", spf2);
    println!("Add mx to spf2");
    spf2.append_mechanism(Mechanism::new_mx_without_mechanism(Qualifier::Pass));
    println!("Altered spf 2: >{}<", spf2);
    println!("Clear mx from spf2");
    spf2.clear_mechanism(Kind::MX);
    println!("Altered spf 2: >{}<", spf2);

    let mut spf3 = Spf::new();
    spf3.set_v2_pra();
    spf3.append_mechanism(Mechanism::new_a_without_mechanism(Qualifier::Pass));
    spf3.append_mechanism(Mechanism::new_all(Qualifier::Neutral));

    println!("\nNew spf 3: >{}<", spf3);
    println!("Change spf3 all to Fail");
    spf3.append_mechanism(Mechanism::new_all(Qualifier::Fail));
    println!("Altered spf 3: >{}<", spf3);
}
