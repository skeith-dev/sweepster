use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    pub action: String,
    pub target: String,
    
    #[arg(short, long)]
    pub option: Option<String>,
    #[arg(short, long)]
    pub criteria: Option<String>,
    #[arg(short = 'n', long, value_delimiter = ' ')]
    pub file_names: Option<Vec<String>>,
    #[arg(short = 'e', long, value_delimiter = ' ')]
    pub file_extensions: Option<Vec<String>>,
    #[arg(short = 'd', long)]
    pub cutoff_date: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub include_extension: bool,
    #[arg(short, long, default_value_t = false)]
    pub print: bool,
    #[arg(short = 'v', long)]
    pub csv_path: Option<String>,
    #[arg(short, long)]
    pub storage_path: Option<String>,

}
