use ethers::{contract::Abigen, solc::Solc};

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();
    args.next().unwrap(); // skip program name

    let path_prefix = String::from("/home/by/bots/bots/");
    let contract = args.next().unwrap();
    println!("{}", contract);
    let json = path_prefix.clone() + "abi-json/" + &contract + ".json";
    let bindings_path = path_prefix + "src/bindings/" + &contract + ".rs";

    let contract_name = args.next().unwrap_or(contract.to_owned());
    let contract: String = args.next().unwrap_or(json.to_owned());

    println!("Generating bindings for {}\n", contract);

    // compile it
    let abi = if contract.ends_with(".sol") {
        let contracts = Solc::default().compile_source(&contract)?;
        let abi = contracts.get(&contract, &contract_name).unwrap().abi.unwrap();
        serde_json::to_string(abi).unwrap()
    } else {
        contract
    };

    let bindings = Abigen::new(&contract_name, abi)?.generate()?;

    // print to stdout if no output arg is given
    bindings.write_to_file(&bindings_path)?;
    // if let Some(output_path) = args.next() {
    //     bindings.write_to_file(&output_path)?;
    // } else {
    //     bindings.write(std::io::stdout())?;
    // }

    Ok(())
}
