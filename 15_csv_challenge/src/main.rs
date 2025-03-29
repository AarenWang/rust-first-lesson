use structopt::StructOpt;
mod opt;
use self::opt::Opt;
fn main() {
    let opt = Opt::from_args();
    println!("{:? }", opt);
}
