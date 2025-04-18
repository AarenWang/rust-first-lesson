use structopt_derive::*;
#[derive(StructOpt, Debug)]
#[structopt(name = "csv_challenge", about = " Useage ")]
pub struct Opt {
    #[structopt(help = "Input file")]
    pub input: String,
    #[structopt(help = "Column Name")]
    pub column_name: String,
    #[structopt(help = "Replacement Column Name")]
    pub replacement: String,
    #[structopt(help = "Output file,stdout if not presend")]
    pub output: Option<String>,
}
