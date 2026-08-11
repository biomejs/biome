use biome_analyze::RuleDomain;
use std::str::FromStr;

#[test]
fn astro_domain_metadata() {
    let domain = RuleDomain::from_str("astro").unwrap();

    assert_eq!(domain, RuleDomain::Astro);
    assert_eq!(domain.as_str(), "astro");
    assert_eq!(domain.manifest_dependencies(), &[&("astro", ">=1.0.0")]);
    assert!(domain.globals().is_empty());
}
