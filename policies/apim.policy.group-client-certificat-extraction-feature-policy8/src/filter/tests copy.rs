#[cfg(test)]
mod tests {
    use crate::utils::{check_not_empty, hash_dns_value};

    #[test]
    fn test_tc01_valid_cn_only() {
        let input = "cn=client.bpi.co.id";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc02_valid_cn_domain_style() {
        let input = "cn=apix.sandbox-111094.com";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc03_full_dn_structure() {
        let input = "cn=wrcwebdlf01.axa-id.intraxa,o=axa services indonesia,l=indonesia,st=indonesia,c=id";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc04_wildcard_cn() {
        let input = "cn=*.axa.co.id,o=pt axa services indonesia,l=jakarta selatan,st=daerah khusus ibukota jakarta,c=id";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc05_minimal_dn_with_wildcard() {
        let input = "cn=*.sandbox-111094.com";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc06_email_and_dn() {
        let input = "1.2.840.113549.1.9.1=#1624...,cn=*.nprod.sg.corp.intraxa";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc07_case_insensitive_dn() {
        let input = "CN=preprodesg.axa.co.id, O=PT Asuransi AXA Indonesia, L=JAKARTA, C=ID";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc08_long_cn_nested_ou() {
        let input = "cn=surrender-api.axa-li-jp-preprod-lpl-int,ou=api,ou=services,ou=axa,ou=jp";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc09_variation_in_cn() {
        let input = "cn=psdept-preprod.surrender-api,ou=api,ou=services,ou=axa,ou=jp";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc10_oid_and_email_hex() {
        let input = "1.2.840.113549.1.9.1=#1618...,cn=api-np.int.krungthai-axa.co.th";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc11_short_cn_nested_ou() {
        let input = "cn=eip_stage,ou=infra,ou=platform";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc12_cn_with_ph_domain() {
        let input = "cn=sfdc-prod.axa.com.ph";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc13_very_long_cn() {
        let input = "cn=claimsdatareg-preprod.claims-payment-input-hats-common-util-api";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc14_external_partner_hsbc() {
        let input = "cn=giil-filenet-hk,ou=hsbc";
        assert!(check_not_empty(input).is_ok());
        let hash = hash_dns_value(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_tc15_empty_input_should_fail() {
        let input = "";
        assert!(check_not_empty(input).is_err());
    }

    #[test]
    fn test_tc16_whitespace_only_should_fail() {
        let input = "   ";
        assert!(check_not_empty(input).is_err());
    }
}
