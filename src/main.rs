use std::collections::HashSet;

use rand::prelude::IteratorRandom;
use rand::thread_rng;
use rs_graph::{Builder, VecGraph};
use rs_graph::traits::{FiniteGraph, Indexable};
use rs_graph::vecgraph::VecGraphBuilder;
use clap::{Parser, ValueEnum};

use crate::Coloring::{Candidate, Permanent};

type Color = usize;

#[derive(Copy, Clone, Debug)]
enum Coloring {
    Permanent(Color),
    Candidate(Color),
}

impl Coloring {
    fn color(&self) -> &Color {
        match self {
            Permanent(v) => { v }
            Candidate(v) => { v }
        }
    }
}

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    coloring: Coloring,
    inbox: Vec<(usize, Coloring)>,
}

fn N(id: usize) -> Node {
    Node {
        id,
        coloring: Candidate(id),
        inbox: Vec::new(),
    }
}

/// creates a complete graph with `num_nodes` vertices
/// the graph has max degree `num_nodes`
/// returns the graph, a vector of nodes and delta (max degree)
fn complete_graph(num_nodes: usize) -> (VecGraph, Vec<Node>, usize) {
    let mut nodes = Vec::with_capacity(num_nodes);
    let mut g = VecGraphBuilder::new();
    let g_nodes = g.add_nodes(num_nodes);

    for n1 in &g_nodes {
        for n2 in &g_nodes {
            if n1 != n2 {
                g.add_edge(*n1, *n2);
            }
        }
        nodes.push(N(n1.index()));
    }

    let delta = num_nodes - 1;
    (g.into_graph(), nodes, delta)
}

/// creates a graph that is a chain of vertices with `num_nodes` vertices
/// the degree has max degree 2
/// returns the graph, a vector of nodes and delta (max degree)
fn chain(num_nodes: usize) -> (VecGraph, Vec<Node>, usize) {
    let mut nodes = Vec::with_capacity(num_nodes);
    let mut g = VecGraphBuilder::new();

    let g_nodes = g.add_nodes(num_nodes);

    for n in &g_nodes {
        nodes.push(N(n.index()));
    }

    for i in 0..g_nodes.len() - 1 {
        g.add_edge(g_nodes[i], g_nodes[i + 1]);
        g.add_edge(g_nodes[i + 1], g_nodes[i]);
    }

    (g.into_graph(), nodes, 2)
}

fn distributed_randomized_coloring_algorithm(graph: VecGraph, nodes: &mut Vec<Node>, delta: usize, verbose: bool) {
    // we have delta + 1 available color
    let list_of_colors: HashSet<Color> = (0..=delta).collect();
    assert_eq!(list_of_colors.len(), delta + 1);

    if verbose {
        println!("Starting algorithm");
    }
    let mut round = 1;
    let mut rng = thread_rng();

    // in the first round no node has a permanent color, so everybody chooses a random color
    for node in nodes.iter_mut() {
        let random_color = list_of_colors.iter().choose(&mut rng).unwrap();
        node.coloring = Candidate(*random_color);
        if verbose {
            println!("node {:3} chose color {:?}", node.id, node.coloring);
        }
    }

    loop {
        if verbose {
            println!("\nStarting round {round}");
        }

        // exchange color with all neighbors
        for e in graph.edges() {
            let (u, v) = graph.enodes(e);
            let c = nodes[u.index()].coloring;
            nodes[v.index()].inbox.push((u.index(), c));

            if verbose {
                println!("node {:3}: sending to node {:3}: ({}, {:?})", u.index(), v.index(), u.index(), c);
            }
        }

        let has_candidate_color = |n: &&mut Node| match n.coloring {
            Candidate(_) => true,
            Permanent(_) => false
        };

        // for all non permanent nodes compute available set of colors and permanently color if possible
        // if not do next iteration and choose new random color
        for node in nodes.iter_mut().filter(has_candidate_color) {
            if verbose {
                println!("node {:3} is none permanent", node.id);
            }
            let mut available_colors = list_of_colors.clone();
            let mut candidate_colors = list_of_colors.clone();

            for (_, coloring) in &node.inbox {
                if let Permanent(v) = coloring {
                    available_colors.remove(v);
                }
                candidate_colors.remove(coloring.color());
            }

            // reset inbox
            node.inbox.clear();

            // check if node can go permanent
            if candidate_colors.contains(node.coloring.color()) {
                if verbose {
                    println!("node {:3}: my color {:?} is used by nobody lets go permanent", node.id, node.coloring);
                }
                node.coloring = Permanent(*node.coloring.color());
                break;
            }

            let random_color = available_colors.iter().choose(&mut rng).unwrap();
            node.coloring = Candidate(*random_color);

            if verbose {
                println!("node {:3} cannot be fixed chose new color {:?}", node.id, node.coloring);
            }
        }

        // check if the graph has a valid coloring
        if nodes.iter_mut().filter(has_candidate_color).next().is_none() {
            if verbose {
                println!("no candidate colors left, coloring should be fixed");
                println!("Finished after {round} rounds\n");
            }
            break;
        }

        // print new coloring
        for node in nodes.iter_mut() {
            if verbose {
                println!("node {:3} has color {:?}", node.id, node.coloring);
            }
        }

        round += 1;
    }
}


/// this is the test case, it generates a complete graph with 200 vertices
/// in such a case each color may only be used once
/// we check this by checking the length of the deduplicated vector containing
/// all colors has the same length as the vector containing all the nodes
fn test_case(verbose: bool) {
    let (graph, mut nodes, delta) = complete_graph(200);
    distributed_randomized_coloring_algorithm(graph, &mut nodes, delta, verbose);

    println!("\n\nAlgorithm finished:");
    for node in nodes.iter_mut() {
        println!("node {:3} has permanent color {:3}", node.id, node.coloring.color());
    }

    // in a complete graph, each color must only be used once
    nodes.sort_by(|a, b| a.coloring.color().cmp(&b.coloring.color()));
    println!("\nSorting by color:");
    for node in nodes.iter_mut() {
        println!("node {:3} has permanent color {:3}", node.id, node.coloring.color());
    }

    // the length must be the same after the deduplication
    let all_nodes_len = nodes.len();
    nodes.dedup_by_key(|n| *n.coloring.color());
    assert_eq!(nodes.len(), all_nodes_len);
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Run mode
    #[arg(value_enum, default_value_t = RunMode::Testcase)]
    mode: RunMode,

    /// Number of nodes to be used, has no effect for testcase run mode
    #[arg(default_value_t = 1, value_parser = clap::value_parser ! (u64).range(1..))]
    num: u64,

    /// Print additional information while running the algorithm
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum RunMode {
    Testcase,
    CompleteGraph,
    Chain,
}

fn main() {
    let cli = Cli::parse();
    let num_nodes = cli.num as usize;
    println!("Running in {:?} mode with {num_nodes} vertices", cli.mode);

    match cli.mode {
        RunMode::Testcase => {
            test_case(cli.verbose);
        }
        RunMode::CompleteGraph => {
            let (graph, mut nodes, delta) = complete_graph(num_nodes);
            distributed_randomized_coloring_algorithm(graph, &mut nodes, delta, cli.verbose);

            for node in nodes.iter_mut() {
                println!("node {:3} has permanent color {:3}", node.id, node.coloring.color());
            }
        }
        RunMode::Chain => {
            let (graph, mut nodes, delta) = chain(num_nodes);
            distributed_randomized_coloring_algorithm(graph, &mut nodes, delta, cli.verbose);

            for node in nodes.iter_mut() {
                println!("node {:3} has permanent color {:3}", node.id, node.coloring.color());
            }
        }
    }
}
