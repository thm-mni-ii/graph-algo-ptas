# PTAS für schwere Probleme auf planaren Graphen

<div align="center">

[![CI/CD](https://github.com/thm-mni-ii/graph-algo-ptas/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/thm-mni-ii/graph-algo-ptas/actions/workflows/ci-cd.yml) 
[![Coverage Status](https://coveralls.io/repos/github/thm-mni-ii/graph-algo-ptas/badge.svg?branch=main)](https://coveralls.io/github/thm-mni-ii/graph-algo-ptas?branch=main)
[![Docs](https://img.shields.io/github/workflow/status/thm-mni-ii/graph-algo-ptas/Pages/main?label=Docs&logo=Github)](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/)
[![Benchmark](https://img.shields.io/github/workflow/status/thm-mni-ii/graph-algo-ptas/Pages/main?label=Benchmark&logo=Github)](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/report/)

</div>

Ein PTAS (engl.: *Polynomial Time Approximation Scheme*) ist ein Approximationsalgorithmus für ein (meist schweres) Problem, der einen Wert $ε > 0$ als Parameter hat und eine Lösung ausgibt, die bei Minimierungsproblemen höchstens $(1 + ε)$-Mal und bei Maximierungsproblemen mindestens $(1 - ε)$-Mal so groß wie die Optimallösung ist. Wichtig ist, dass die Laufzeit des Algorithmus polynomiell in $n$ sein muss.

## Bakers Ansatz

Eine Technik zum Entwurf von PTAS für verschiedene schwere Probleme auf planaren Graphen wurde 1994 von Baker entwickelt [^1]. Im Wesentlichen werden folgende Schritte durchgeführt:

1. Aufteilen des Eingabegraphen in k-außenplanare Ringe (wobei $k=1/ε$)
2. Berechnung der optimalen Lösung auf jedem Ring mit Hilfe von Baumzerlegungen und dynamischer Programmierung
3. Kombinieren der optimalen Teillösungen zu approximativer Gesamtlösung

## Projektbeschreibung

Im Rahmen dieses Projekts wurde Bakers Ansatz implementiert. [Hier](docs/data_structure.md) wird auf die verwendeten Datenstrukturen und Algorithmen zum Umgang mit planaren Graphen und planaren Einbettungen eingegangen. [Hier](docs/algorithm) werden die die Algorithmen für die einzelnen Schritte des PTAS beschrieben.

[^1]: BAKER, BRENDA S. 1994. Appoximation Algorithms for NP-Complete Problems on Planar Graphs, J. *ACM 41, January 1994, pp 153-180*

## Das CLI-Tool

### Erstellen

Um das CLI Tool zu bauen, kann folgender Befehl verwendet werden:

`cargo build --release --features="cli"`

### Verwendung

Das CLI Tool kann hiernach folgendermaßen verwendet werden:

- `./target/release/graph-algo-ptas-cli <options>` 
- *oder* `cargo run --release --features="cli" -- <options>` (Hierbei wird `cargo build` nicht benötigt.)
  
Dabei können folgende Optionen angegeben werden:

```sh
USAGE:
    graph-algo-ptas-cli [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -g, --generate <n>    Generate Random graph with n vertices
    -h, --help            Print help information
    -i, --input <FILE>    File in dot format to read input graph from
    -V, --version         Print version information

SUBCOMMANDS:
    embed              Generates an embeding for the graph
    help               Print this message or the help of the given subcommand(s)
    independent-set    Calculates Maximal Independent Set (Default)
    print              Prints the generated/input graph
    vertex-cover       Calculates Minimal Vertex Cover
```

> :warning: Hierbei ist zu beachten, dass die Option `-g <n>` oder `-i <FILE>` immer angegeben werden muss.

## Die Library

Zur Verwendung dieser `Crate` muss einfach nur [`graph-algo-ptas`](https://crates.io/crates/graph-algo-ptas) zur `cargo.toml` hinzugefügt werden. Eine Dokumentation aller zur Verfügung stehenden Funktionen befindet sich [hier](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/).