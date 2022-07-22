use anyhow::Result;
use ethers_core::{
    types::{Address, Bytes},
    utils::get_create2_address,
};
use ethers_solc::Project;
use owo_colors::OwoColorize;
use rand::RngCore;
use structopt::StructOpt;

use std::{path::PathBuf, str::FromStr};

fn parse_hex(src: &str) -> Result<Bytes, hex::FromHexError> {
    hex::decode(src).map(|b| b.into())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "vanity",
    about = "CREATE2 based salt finder for required vanity contract address"
)]
struct Opt {
    #[structopt(short, long)]
    prefix: bool,

    #[structopt(short = "c", long = "contract", parse(from_os_str))]
    contract: PathBuf,

    #[structopt(short = "d", long = "deployer", parse(try_from_str = Address::from_str))]
    deployer: Address,

    #[structopt(short = "m", long = "matches", parse(try_from_str = parse_hex))]
    matches: Bytes,
}

fn pretty_print<I: IntoIterator<Item = String>>(tags: I, values: I) {
    for (tag, value) in tags.into_iter().zip(values.into_iter()) {
        println!("{}: {}", tag.bold(), value.green());
    }
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let contract_name = opt
        .contract
        .file_stem()
        .ok_or_else(|| anyhow::anyhow!("could not parse contract name"))?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("could not convert OsStr to str"))?
        .to_string();

    let project = Project::builder().build().unwrap();
    let output = project.compile_file(opt.contract)?.output();
    let contract = output
        .contracts
        .find_first(&contract_name)
        .ok_or_else(|| anyhow::anyhow!("contract: {:?} not found", contract_name))?;
    let code = contract
        .bytecode()
        .ok_or_else(|| anyhow::anyhow!("contract: {:?} bytecode not found", contract_name))?;

    let mut rng = rand::thread_rng();
    let mut salt = [0u8; 32];
    loop {
        rng.fill_bytes(&mut salt);
        let vanity_addr = get_create2_address(opt.deployer, salt, code.clone()).to_fixed_bytes();
        if opt.prefix && vanity_addr.starts_with(&opt.matches) {
            pretty_print(
                ["Salt".to_string(), "Vanity Address".to_string()],
                [hex::encode(&salt), hex::encode(&vanity_addr)],
            );
            break;
        }
        if !opt.prefix && vanity_addr.ends_with(&opt.matches) {
            pretty_print(
                ["Salt".to_string(), "Vanity Address".to_string()],
                [hex::encode(&salt), hex::encode(&vanity_addr)],
            );
            break;
        }
    }

    Ok(())
}
