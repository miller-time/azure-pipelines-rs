use std::{env, error::Error, fs, process};

use azure_pipelines_rs::core::v1::pipeline::Pipeline;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: azure-pipelines-rs PIPELINE_FILE");
        process::exit(1);
    }

    let pipeline_path = &args[1];
    let contents = fs::read_to_string(pipeline_path)?;

    let pipeline: Pipeline = serde_yaml::from_str(&contents)?;

    println!("writing ast to ast.txt");
    fs::write("ast.txt", format!("{:#?}", pipeline))?;

    println!("writing parsed pipeline to parsed.yaml");
    let parsed = serde_yaml::to_string(&pipeline)?;
    fs::write("parsed.yaml", parsed)?;

    println!("pipeline valid");

    Ok(())
}
