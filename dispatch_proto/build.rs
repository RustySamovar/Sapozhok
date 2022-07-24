use std::io::Result;

fn main() -> Result<()> {
    let proto_dir = "protobuf";

    let protos = vec![
        // Dispatch protocol
        "QueryRegionListHttpRsp",
        "QueryCurrRegionHttpRsp",
        "RegionSimpleInfo",
        "RegionInfo",
    ];

    let protos: Vec<String> = protos.iter().map(|&x| format!("{}/{}.proto", proto_dir, x)).collect();

    let mut config = prost_build::Config::new();

    config.type_attribute(".", "#[derive(serde::Deserialize)]");

    let ret = config.compile_protos(&protos, &[format!("{}/", proto_dir)]);

    match ret {
        Ok(_) => return Ok(()),
        Err(e) => panic!("{}", e),
    }
}
