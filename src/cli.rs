use clap::Parser;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {

    pub action: String,
    pub target: Option<String>,
    
    #[arg(short, long)]
    pub option: Option<String>,
    #[arg(short, long)]
    pub criteria: Option<String>,
    #[arg(short = 'n', long)]
    pub file_names: Option<Vec<String>>,
    #[arg(short = 'e', long)]
    pub file_extensions: Option<Vec<String>>,
    #[arg(short = 'd', long)]
    pub cutoff_date: Option<String>,

    #[arg(short, long)]
    pub include_extension: Option<bool>,
    #[arg(short, long)]
    pub print: Option<bool>,
    #[arg(short = 'v', long)]
    pub csv_path: Option<String>,
    #[arg(short, long)]
    pub storage_path: Option<String>,

}
