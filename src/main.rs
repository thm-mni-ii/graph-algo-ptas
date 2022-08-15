#[cfg(feature = "cli")]
use std::path::PathBuf;

#[cfg(feature = "cli")]
use clap::Parser;
#[cfg(feature = "cli")]
use clap::Subcommand;
#[cfg(feature = "cli")]
use graph_algo_ptas::data_structure::dot_reader::read_graph;
#[cfg(feature = "cli")]
use graph_algo_ptas::data_structure::graph_dcel::GraphDCEL;
#[cfg(feature = "cli")]
use graph_algo_ptas::embeding::{index::Embeding, maximal_planar::index::MaximalPlanar};
#[cfg(feature = "cli")]
use graph_algo_ptas::generation::planar::generate;
#[cfg(feature = "cli")]
use petgraph::dot::{Config, Dot};
#[cfg(feature = "cli")]
use petgraph::stable_graph::DefaultIx;
#[cfg(feature = "cli")]
use petgraph::stable_graph::StableGraph;
#[cfg(feature = "cli")]
use petgraph::Undirected;

#[cfg(feature = "cli")]
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Generate Random graph with n vertices
    #[clap(short, long, value_parser, value_name = "n")]
    generate: Option<usize>,

    /// File in dot format to read input graph
    #[clap(short, long, value_parser, value_name = "FILE")]
    input: Option<PathBuf>,

    #[clap(subcommand)]
    mode: Option<Commands>,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Prints the generated/input graph
    Print {},
    /// Generates an embeding for the graph
    Embed {},
}

#[cfg(feature = "cli")]
fn main() {
    use std::fs;

    let cli = Cli::parse();
    let mut generated = false;
    let mut input_graph: Option<_> = None;

    if let Some(file) = cli.input {
        println!("[ptas] read input graph");
        let file_input = fs::read_to_string(file);
        match file_input {
            Ok(graph_text) => input_graph = read_graph(graph_text),
            Err(_) => eprintln!("[ptas] Invalid input file"),
        }
    }

    if cli.generate.is_some() {
        println!("[ptas] generate random graph");
        let n = cli.generate.unwrap();

        if n < 4 {
            eprintln!("generation reqires n >= 4");
            return;
        }

        input_graph = Some(generate(n, None).to_pet_graph());
        generated = true;
    }

    if input_graph.is_none() {
        return eprintln!("Invalid usage, see --help for help");
    }

    match cli.mode {
        Some(Commands::Print {}) => {
            print_graph(&input_graph.unwrap());
        }
        Some(Commands::Embed {}) => embed_graph(&input_graph.unwrap(), generated),
        None => embed_graph(&input_graph.unwrap(), generated),
    }
}

#[cfg(feature = "cli")]
fn print_graph(graph: &StableGraph<u32, (), Undirected, DefaultIx>) {
    println!("[ptas] graph in dot format:");
    println!(
        "{:?}",
        Dot::with_config(graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
    );
}

#[cfg(feature = "cli")]
fn embed_graph(graph: &StableGraph<u32, (), Undirected, DefaultIx>, generated: bool) {
    let dcel = MaximalPlanar::embed(graph.clone());

    if generated {
        print_graph(graph);
    }

    println!("[ptas] embedet input graph:");
    dcel.get_vertexes().for_each(|v| println!("{:?}", v));
    println!();
    dcel.get_darts().for_each(|d| println!("{:?}", d));
}
