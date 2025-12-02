use std::collections::HashSet;

use crate::core::v1::{depends::DependsOn, job::Job, stage::Stage};

pub fn validate_dependencies(stages: &[Stage]) -> Result<(), String> {
    validate_stage_depends(stages)?;
    validate_job_depends(stages)?;
    Ok(())
}

fn validate_stage_depends(stages: &[Stage]) -> Result<(), String> {
    let mut stage_names = HashSet::new();
    for stage in stages {
        if let Stage::Stage(stage) = stage {
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
    }

    Ok(())
}

fn validate_job_depends(stages: &[Stage]) -> Result<(), String> {
    for stage in stages {
        if let Stage::Stage(stage) = stage {
            let mut job_names = HashSet::new();
            for job in &stage.jobs {
                match job {
                    Job::Job(job) => {
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
                        }
                    }
                    Job::Template(job) => {
                        if let Some(name) = job.parameters.get("jobNameOverride") {
                            if let Some(name) = name.as_str() {
                                job_names.insert(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
