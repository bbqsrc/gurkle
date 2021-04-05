use anyhow::*;
use graphql_client_codegen::{generate_module_token_stream, GraphQLClientCodegenOptions};
use std::{fs::File, path::Path};
use std::io::Write as _;
use std::path::PathBuf;
use syn::Token;

pub(crate) struct CliCodegenParams {
    pub query_paths: Vec<PathBuf>,
    pub schema_path: PathBuf,
    pub selected_operation: Option<String>,
    pub variables_derives: Option<String>,
    pub response_derives: Option<String>,
    pub deprecation_strategy: Option<String>,
    pub no_formatting: bool,
    pub module_visibility: Option<String>,
    pub output_directory: Option<PathBuf>,
    pub custom_scalars_module: Option<String>,
}

pub(crate) fn generate_code(params: CliCodegenParams) -> Result<()> {
    let CliCodegenParams {
        variables_derives,
        response_derives,
        deprecation_strategy,
        no_formatting,
        output_directory,
        module_visibility: _module_visibility,
        query_paths,
        schema_path,
        selected_operation,
        custom_scalars_module,
    } = params;

    let deprecation_strategy = deprecation_strategy.as_ref().and_then(|s| s.parse().ok());

    let mut options = GraphQLClientCodegenOptions::new();

    options.set_module_visibility(
        syn::VisPublic {
            pub_token: <Token![pub]>::default(),
        }
        .into(),
    );

    if let Some(selected_operation) = selected_operation {
        options.set_operation_name(selected_operation);
    }

    if let Some(variables_derives) = variables_derives {
        options.set_variables_derives(variables_derives);
    }

    if let Some(response_derives) = response_derives {
        options.set_response_derives(response_derives);
    }

    if let Some(deprecation_strategy) = deprecation_strategy {
        options.set_deprecation_strategy(deprecation_strategy);
    }

    if let Some(custom_scalars_module) = custom_scalars_module {
        let custom_scalars_module = syn::parse_str(&custom_scalars_module).with_context(|| {
            format!(
                "Invalid custom scalars module path: {}",
                custom_scalars_module
            )
        })?;

        options.set_custom_scalars_module(custom_scalars_module);
    }

    let gen = generate_module_token_stream(query_paths.clone(), &schema_path, options).unwrap();

    let generated_code = gen.to_string();

    let dest_file_path: PathBuf = output_directory
        .map(|output_dir| output_dir.join("mod.rs"))
        .unwrap_or_else(move || "mod.rs".into());

    let mut file = File::create(&dest_file_path)?;
    write!(file, "{}", generated_code)?;

    if !no_formatting {
        format(&dest_file_path)?;
    };

    Ok(())
}

fn format(path: &Path) -> Result<(), std::io::Error> {
    std::process::Command::new("rustfmt")
        .arg(path)
        .output()
        .map(|_| ())
}
