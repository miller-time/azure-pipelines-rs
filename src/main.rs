use std::{collections::HashSet, env, error::Error, fs, process};

use azure_pipelines_rs::core::v1::{depends::DependsOn, pipeline::Pipeline};

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

    validate_stage_depends(&pipeline)?;
    validate_job_depends(&pipeline)?;

    println!("pipeline valid");

    Ok(())
}

fn validate_stage_depends(pipeline: &Pipeline) -> Result<(), String> {
    let mut stage_names = HashSet::new();
    for stage in &pipeline.extends.parameters.stages {
        if let Some(depends_on) = &stage.depends_on {
            match depends_on {
                DependsOn::Single(other) => {
                    if !stage_names.contains(other) {
                        return Err(format!(
                            "stage {:?} depends on non-existent stage {other}",
                            stage.name
                        ));
                    }
                }
                DependsOn::Multi(others) => {
                    for other in others {
                        if !stage_names.contains(other) {
                            return Err(format!(
                                "stage {:?} depends on non-existent stage {other}",
                                stage.name
                            ));
                        }
                    }
                }
            }
        }
        if let Some(name) = &stage.name {
            stage_names.insert(name.clone());
        }
    }

    Ok(())
}

fn validate_job_depends(pipeline: &Pipeline) -> Result<(), String> {
    for stage in &pipeline.extends.parameters.stages {
        let mut job_names = HashSet::new();
        for job in &stage.jobs {
            if let Some(depends_on) = &job.depends_on {
                match depends_on {
                    DependsOn::Single(other) => {
                        if !job_names.contains(other) {
                            return Err(format!(
                                "job {:?} depends on non-existent job {other}",
                                job.name
                            ));
                        }
                    }
                    DependsOn::Multi(others) => {
                        for other in others {
                            if !job_names.contains(other) {
                                return Err(format!(
                                    "job {:?} depends on non-existent job {other}",
                                    job.name
                                ));
                            }
                        }
                    }
                }
            }
            if let Some(name) = &job.name {
                job_names.insert(name.clone());
            } else if let Some(name) = job.parameters.get("jobNameOverride") {
                if let Some(name) = name.as_str() {
                    job_names.insert(name.to_string());
                }
            }
        }
    }

    Ok(())
}
