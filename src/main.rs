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

    let _pipeline: Pipeline = serde_yaml::from_str(&contents)?;

    println!("pipeline valid");

    Ok(())
}
