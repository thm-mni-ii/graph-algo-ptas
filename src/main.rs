#[cfg(feature = "cli")]
mod cli {
    use clap::Parser;
    use clap::Subcommand;
    use graph_algo_ptas::algorithm::dynamic_programming::solve::DpProblem;
    use graph_algo_ptas::algorithm::ptas::ptas;
    use graph_algo_ptas::data_structure::dot_reader::read_graph;
    use graph_algo_ptas::data_structure::graph_dcel::GraphDCEL;
    use graph_algo_ptas::embedding::{index::Embedding, maximal_planar::index::MaximalPlanar};
    use graph_algo_ptas::generation::planar::generate;
    use petgraph::dot::{Config, Dot};
    use petgraph::stable_graph::DefaultIx;
    use petgraph::stable_graph::StableGraph;
    use petgraph::Undirected;
    use std::path::PathBuf;

    #[derive(Parser)]
    #[clap(author, version, about, long_about = None)]
    struct Cli {
        /// Generate Random graph with n vertices
        #[clap(short, long, value_parser, value_name = "n")]
        generate: Option<usize>,

        /// File in dot format to read input graph from
        #[clap(short, long, value_parser, value_name = "FILE")]
        input: Option<PathBuf>,

        #[clap(subcommand)]
        mode: Option<Commands>,
    }

    #[derive(Subcommand)]
    enum Commands {
        /// Prints the generated/input graph
        Print {},
        /// Generates an embedding for the graph
        Embed {},
        /// Calculates Minimal Vertex Cover
        VertexCover {},
        /// Calculates Maximal Independent Set (Default)
        IndependentSet {},
    }

    pub fn main() {
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
            Some(Commands::VertexCover {}) => run_ptas(&input_graph.unwrap(), generated, true),
            Some(Commands::IndependentSet {}) | None => {
                run_ptas(&input_graph.unwrap(), generated, false)
            }
        }
    }

    fn print_graph(graph: &StableGraph<(), (), Undirected, DefaultIx>) {
        println!("[ptas] graph in dot format:");
        println!(
            "{:?}",
            Dot::with_config(graph, &[Config::EdgeNoLabel, Config::NodeIndexLabel])
        );
    }

    fn embed_graph(graph: &StableGraph<(), (), Undirected, DefaultIx>, generated: bool) {
        let dcel = MaximalPlanar::embed(graph.clone());

        if generated {
            print_graph(graph);
        }

        println!("[ptas] embedded input graph:");
        dcel.get_vertexes().for_each(|v| println!("{:?}", v));
        println!();
        dcel.get_darts().for_each(|d| println!("{:?}", d));
    }

    fn run_ptas(
        graph: &StableGraph<(), (), Undirected, DefaultIx>,
        generated: bool,
        min_vertex_cover: bool,
    ) {
        let (prob, out_text) = if min_vertex_cover {
            (DpProblem::min_vertex_cover(), "Minimum Vertex Cover")
        } else {
            (DpProblem::max_independent_set(), "Maximum Independent Set")
        };
        let sol = ptas(graph, &prob, 0.5);

        if generated {
            print_graph(graph);
        }

        println!("[ptas] {}:", out_text);
        println!("{:?}", sol);
    }
}

#[cfg(feature = "cli")]
fn main() {
    cli::main()
}
