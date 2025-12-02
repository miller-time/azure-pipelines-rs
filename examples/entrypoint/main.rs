use std::{error::Error, fs};

use clap::Parser;

use azure_pipelines_rs::{
    core::v1::pipeline::Pipeline, templates::parameterized::Parameterized,
    validator::dependencies::validate_dependencies,
};

mod template;

use template::ExampleEntrypoint;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.pipeline_file)?;

    let pipeline: Pipeline = serde_yaml::from_str(&contents)?;

    println!("writing ast to ast.txt");
    fs::write("ast.txt", format!("{:#?}", &pipeline))?;

    println!("writing parsed pipeline to parsed.yaml");
    let parsed = serde_yaml::to_string(&pipeline)?;
    fs::write("parsed.yaml", parsed)?;

    let parameters = ExampleEntrypoint::get_parameters(&pipeline.extends.parameters)?;
    validate_dependencies(&parameters.stages)?;

    println!("pipeline valid");

    Ok(())
}

#[derive(Parser)]
#[command(version, about = "Azure Pipeline Validator")]
struct Args {
    #[arg(value_name = "PIPELINE_FILE", help = "Path to your pipeline yaml file")]
    pipeline_file: String,
}
