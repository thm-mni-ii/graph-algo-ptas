#[cfg(feature = "cli")]
use std::path::PathBuf;

#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use graph_algo_ptas::generation::planar::generate;
#[cfg(feature = "cli")]
use petgraph::dot::{Config, Dot};

#[cfg(feature = "cli")]
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Generate Random graph with n Nodes
    #[clap(short, long, value_parser, value_name = "n")]
    generate: Option<usize>,

    /// File in dot format to read input graph
    #[clap(short, long, value_parser, value_name = "FILE")]
    input: Option<PathBuf>,
}

#[cfg(feature = "cli")]
fn main() {
    let cli = Cli::parse();

    if cli.input.is_some() {
        println!("input solution");
        // TODO
        return;
    }

    if cli.generate.is_some() {
        println!("random solution");
        let n = cli.generate.unwrap();

        if n < 4 {
            println!("generation reqires n >= 4");
            return;
        }

        let graph = generate(n).to_pet_graph();
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
        // TODO

        return;
    }

    println!("Invalid usage, see --help for help")
}
