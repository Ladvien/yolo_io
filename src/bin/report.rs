use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use yolo_io::{YoloDataQualityReport, YoloProject, YoloProjectConfig};

#[derive(Parser, Debug)]
#[command(author, version, about = "Generate a data quality report")]
pub struct Cli {
    /// Path to the YAML configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// Output file path
    #[arg(short, long, default_value = "report.json")]
    output: PathBuf,

    /// Output format of the report
    #[arg(short, long, value_enum, default_value_t = Format::Json)]
    format: Format,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum Format {
    Json,
    Yaml,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let config = YoloProjectConfig::new(&cli.config)?;
    let project = YoloProject::new(&config)?;

    if cli.format == Format::Yaml {
        if let Some(report_yaml) = YoloDataQualityReport::generate_yaml(project) {
            std::fs::write(cli.output, report_yaml)?;
        }
    } else if let Some(report_json) = YoloDataQualityReport::generate(project) {
        std::fs::write(cli.output, report_json)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_required_args() {
        let args = ["test", "--config", "path/to/config.yaml"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse args");
        assert_eq!(cli.config, PathBuf::from("path/to/config.yaml"));
        assert_eq!(cli.output, PathBuf::from("report.json"));
        assert_eq!(cli.format, Format::Json);
    }

    #[test]
    fn parses_custom_output_and_format() {
        let args = [
            "test", "--config", "c.yaml", "--output", "out.yml", "--format", "yaml",
        ];
        let cli = Cli::try_parse_from(args).expect("Failed to parse args");
        assert_eq!(cli.output, PathBuf::from("out.yml"));
        assert_eq!(cli.format, Format::Yaml);
    }
}
