use clap::Parser;
use mappings::Mappings;
// use dexlib::{multidex::StreamExt, MultiDexReader};
use std::path::PathBuf;

mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file to match with
    #[arg(short)]
    a: PathBuf,
    // /// Input file to match against
    #[arg(short)]
    b: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mappings = Mappings::from_files(vec![args.a, args.b])?;
    println!("{:#?}", mappings.classes());
    Ok(())
}

/*

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("reading input a");
    let a = MultiDexReader::from_file(args.a).expect("error reading input a");
    // println!("reading input b");
    // let b = MultiDexReader::from_file(args.b).expect("error reading input b");
    println!("done reading");

    let classes_a = a.classes().collect::<Vec<_>>().await;
    let methods_a: usize = classes_a.iter().map(|c| c.methods().count()).sum();
    let fields_a: usize = classes_a.iter().map(|c| c.fields().count()).sum();
    println!(
        "a has {} classes, {} methods, {} fields",
        classes_a.len(),
        methods_a,
        fields_a
    );
}

*/
