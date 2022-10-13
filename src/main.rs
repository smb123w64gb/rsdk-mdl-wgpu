use clap::Parser;
use std::path::PathBuf;
mod lib;
mod mdl;
mod wgpu_render;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    mdl: PathBuf,
}

fn main() {
    //let args = Args::parse();

    //println!("{}", &args.mdl.display());
    //let mdl = mdl::MDLFile::open(args.mdl).unwrap();
    pollster::block_on(lib::run());
}
