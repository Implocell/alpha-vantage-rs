use alpha_vantage::fundemental_data::company_information::get_external_data;
use std::env;

#[test]
fn it_gets_external_company_information_data() {
    env::set_var("API_KEY", "demo");

    let res = get_external_data("IBM");

    match res {
        Ok(v) => assert_eq!(v.cik, 51143),
        Err(e) => panic!("{}", e.to_string()),
    }
}
