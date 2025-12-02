# Example - Entrypoint Template

* `template.rs` - implementation of the `Parameterized` trait for a pipeline
  [extends][extends-docs] template
* `main.rs` - validation of a pipeline that uses this entrypoint template
* `azure-pipelines.yml` - an example pipeline yaml file

## Run It

```sh
$ cargo run --example entrypoint -- examples/entrypoint/azure-pipelines.yml

writing ast to ast.txt
writing parsed pipeline to parsed.yaml
pipeline valid
```

The file **ast.txt** is just a representation of the internal structures. The
file **parsed.yaml** is just a de-serialization and re-serialization of the
input file.

[extends-docs]: https://docs.rs/azure-pipelines-rs/latest/azure_pipelines_rs/core/v1/extends/index.html
