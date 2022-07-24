use std::path::Path;
use std::io::Read;
use std::fs;
use std::collections::HashMap;

use ini::Ini;
use openssl::pkey::{Private};
use openssl::rsa::{Rsa};
use serde::{Deserializer, Deserialize};
use serde::de::Error;

// Keys stuff
#[derive(Deserialize,Debug)]
pub struct RsaKeyInfo {
    pub key_id: u8,
    #[serde(deserialize_with = "deserialize_priv_key")]
    pub encrypt_key: Rsa<Private>,
    #[serde(deserialize_with = "deserialize_priv_key")]
    pub signing_key: Rsa<Private>,
}

/*
fn deserialize_pub_key<'de, D>(deserializer: D) -> Result<Rsa<Public>, D::Error>
    where
        D: Deserializer<'de>,
{
    let public_key_pem: String = Deserialize::deserialize(deserializer)?;

    Rsa::public_key_from_pem(public_key_pem.as_bytes()).map_err(D::Error::custom)
}*/

fn deserialize_priv_key<'de, D>(deserializer: D) -> Result<Rsa<Private>, D::Error>
    where
        D: Deserializer<'de>,
{
    let private_key_pem: String = Deserialize::deserialize(deserializer)?;

    Rsa::private_key_from_pem(private_key_pem.as_bytes()).map_err(D::Error::custom)
}

pub struct Ec2bKeyPair {
    pub ec2b: Vec<u8>,
    pub xorpad: Vec<u8>,
}

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
        let client_secret_key = Self::load_keys(&client_secret_key, &key_directory);

        let rsa_key_config = dispatch_config.get("RsaKeyConfig").unwrap();
        let rsa_key_config = Self::load_rsa_keys(&rsa_key_config, &key_directory);

        let ssl_cert = dispatch_config.get("SslCert").unwrap();
        let ssl_cert = format!("./{}/{}", key_directory, ssl_cert);

        let ssl_key = dispatch_config.get("SslKey").unwrap();
        let ssl_key = format!("./{}/{}", key_directory, ssl_key);

        let regions = dispatch_config.get("Regions").unwrap().split(",");

        let regions = regions.map(|region_name| {
            let region_config = conf.section(Some(region_name)).unwrap();

            let secret_key = region_config.get("SecretKey").unwrap();
            let secret_key = Self::load_keys(&secret_key, &key_directory);

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

    fn load_keys(name: &str, key_directory: &str) -> Ec2bKeyPair {
        // Key
        let filename = format!("./{}/{}.key", key_directory, name);
        let mut f = fs::File::open(&filename).expect(&format!("File '{}' not found", filename));
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut key = vec![0; metadata.len() as usize];
        f.read(&mut key).expect("buffer overflow");
        // Ec2b
        let filename = format!("./{}/{}.ec2b", key_directory, name);
        let mut f = fs::File::open(&filename).expect(&format!("File '{}' not found", filename));
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut ec2b = vec![0; metadata.len() as usize];
        f.read(&mut ec2b).expect("buffer overflow");

        Ec2bKeyPair {
            ec2b: ec2b,
            xorpad: key,
        }
    }

    fn load_rsa_keys(name: &str, key_directory: &str) -> HashMap<u8, RsaKeyInfo> {
        // Key depo
        let path = format!("./{}/{}.json", key_directory, name);
        let json_file_path = Path::new(&path);
        let json_file_str = fs::read_to_string(json_file_path).unwrap_or_else(|_| panic!("File {} not found", path));
        let data: Vec<RsaKeyInfo> = serde_json::from_str(&json_file_str).expect(&format!("Error while reading json {}", name));

        data.into_iter().map(|ki| (ki.key_id, ki)).collect()
    }
}