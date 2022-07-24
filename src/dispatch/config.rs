use std::collections::HashMap;

use mhycrypt::{Ec2bKeyPair, RsaKeyInfo};

use ini::Ini;

pub struct RegionConfig {
    pub name: String,
    pub title: String,
    pub r_type: String,
    pub gateserver_ip: String,
    pub gateserver_port: u32,
    pub secret_key: Ec2bKeyPair,
}

pub struct DispatchConfig {
    pub http_port: u32,
    pub https_port: u32,
    pub ssl_cert: String,
    pub ssl_key: String,
    pub enable_login: bool,
    pub client_secret_key: Ec2bKeyPair,
    pub rsa_keys: HashMap<u8, RsaKeyInfo>,
    pub regions: HashMap<String, RegionConfig>,
}

impl DispatchConfig {
    pub fn load(filename: &str) -> DispatchConfig {
        let conf = Ini::load_from_file(filename).unwrap();

        let dispatch_config = conf.section(Some("DispatchConfig")).unwrap();

        let http_port = dispatch_config.get("HttpPort").unwrap().parse().unwrap();
        let https_port = dispatch_config.get("HttpsPort").unwrap().parse().unwrap();

        let enable_login = dispatch_config.get("EnableLogin").unwrap();
        let key_directory = dispatch_config.get("KeyDirectory").unwrap();

        let client_secret_key = dispatch_config.get("ClientSecretKey").unwrap();
        let client_secret_key = mhycrypt::load_ec2b_keys(&client_secret_key, &key_directory);

        let rsa_key_config = dispatch_config.get("RsaKeyConfig").unwrap();
        let rsa_key_config = mhycrypt::load_rsa_keys(&rsa_key_config, &key_directory);

        let ssl_cert = dispatch_config.get("SslCert").unwrap();
        let ssl_cert = format!("./{}/{}", key_directory, ssl_cert);

        let ssl_key = dispatch_config.get("SslKey").unwrap();
        let ssl_key = format!("./{}/{}", key_directory, ssl_key);

        let regions = dispatch_config.get("Regions").unwrap().split(",");

        let regions = regions.map(|region_name| {
            let region_config = conf.section(Some(region_name)).unwrap();

            let secret_key = region_config.get("SecretKey").unwrap();
            let secret_key = mhycrypt::load_ec2b_keys(&secret_key, &key_directory);

            RegionConfig {
                name: region_config.get("Name").unwrap().to_string(),
                title: region_config.get("Title").unwrap().to_string(),
                r_type: region_config.get("Type").unwrap().to_string(),
                gateserver_ip: region_config.get("GateserverIp").unwrap().to_string(),
                gateserver_port: region_config.get("GateserverPort").unwrap().parse().unwrap(),
                secret_key: secret_key,
            }
        })
            .map(|r| (r.name.clone(), r))
            .collect();

        DispatchConfig {
            http_port: http_port,
            https_port: https_port,
            ssl_cert: ssl_cert,
            ssl_key: ssl_key,
            enable_login: enable_login.parse().unwrap(),
            client_secret_key: client_secret_key,
            rsa_keys: rsa_key_config,
            regions: regions,
        }
    }
}