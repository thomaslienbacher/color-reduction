use std::collections::{HashMap, HashSet};
use rs_graph::{Builder, IndexGraph, VecGraph};
use rs_graph::traits::{FiniteGraph, Indexable};
use rs_graph::vecgraph::VecGraphBuilder;

#[derive(Clone, Debug)]
struct Node {
    id: usize,
    color: usize,
    inbox: Vec<(usize, usize)>,
}

fn N(id: usize) -> Node {
    Node {
        id,
        color: id,
        inbox: Vec::new(),
    }
}

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

/// returns max color used
fn apply_halving(graph: &VecGraph, nodes: &mut Vec<Node>, delta: usize) -> usize {
    println!("Start algorithm: ");
    let mut r = 1;

    // first round calculate own color / 2 and send to all others
    for node in nodes.iter_mut() {
        node.color = node.color.div_ceil(2);
        //println!("node {} has new color {}", node.id, node.color);
    }

    for e in graph.edges() {
        let (u, v) = graph.enodes(e);
        let c = nodes[u.index()].color;
        nodes[v.index()].inbox.push((u.index(), c));
        //println!("node {}: sending to node {}: ({}, {})", u.index(), v.index(), u.index(), c);
    }
    r += 1;

    // loop until all discrepancies are fixed
    for _ in 0..delta {
        println!("\nstarting round {r}");

        // process messages and calculate new color if needed
        for node in nodes.iter_mut() {
            //println!("node {}: doing processing", node.id);
            let mut neighbor_color_set = HashSet::new();
            for (_, c) in &node.inbox {
                neighbor_color_set.insert(c);
            }

            // check is my color in the neighbor set
            if !neighbor_color_set.contains(&node.color) {
                //println!("node {}: i have a good color :)", node.id);
                node.inbox.clear();
                continue;
            } else {
                println!("node {:3}: somebody is also using my color", node.id);
            }

            // get max id of problematic color
            let mut max_id = 0;
            let mut all_colors = node.inbox.clone();
            all_colors.push((node.id, node.color));
            for (id, c) in all_colors {
                if c == node.color {
                    max_id = max_id.max(id);
                }
            }
            println!("max id node with color {} is node {}", node.color, max_id);

            // check if i am someone that needs to change the color
            if max_id == node.id {
                //println!("node {}: i am the problematic node :(", node.id);

                // find smallest number that is not in neighborcolor set
                let mut not_used = 0;
                while neighbor_color_set.contains(&not_used) {
                    not_used += 1;
                }

                node.color = not_used;
                println!("node {:3}: setting my new color to {:3}", node.id, node.color);
            } else {
                //println!("node {}: i am NOT the problematic node", node.id);
            }

            node.inbox.clear();
        }

        // send colors to all neighbors
        for e in graph.edges() {
            let (u, v) = graph.enodes(e);
            let c = nodes[u.index()].color;
            nodes[v.index()].inbox.push((u.index(), c));
            //println!("node {}: sending to node {}: ({}, {})", u.index(), v.index(), u.index(), c);
        }

        println!("\nround colors:");
        for node in nodes.iter_mut() {
            println!("node {:3} has color {:3}", node.id, node.color);
        }

        r += 1;
    }

    //println!("\nfinal colors:");
    for node in nodes.iter_mut() {
        //println!("node {:3} has color {:3}", node.id, node.color);
    }

    println!("\nfinal colors sorted:");
    //nodes.sort_by(|n, b| n.color.cmp(&b.color));
    for node in nodes.iter_mut() {
        println!("node {:3} has color {:3}", node.id, node.color);
    }

    nodes.iter().max_by(|a, b| a.color.cmp(&b.color)).unwrap().color
}

fn main() {
    let (graph, mut nodes, delta) = chain(4);

    /*println!("delta = {delta}");
    println!("Graph edges: ");
    for e in graph.edges() {
        let (u, v) = graph.enodes(e);
        println!("({}, {})", u.index(), v.index());
    }*/

    let mut halvings = 0;
    loop {
        let new_max = apply_halving(&graph, &mut nodes, delta);
        halvings += 1;
        println!("new coloring : = {new_max} at {halvings}");
        if new_max <= delta + 1 {
            break;
        }
        break;
    }

    //println!("other: {}", delta * delta + log_star(1000 * 29770000))
}

fn log_star(mut n: usize) -> usize {
    let mut i = 0;
    while n > 2 {
        n = n.ilog2() as usize;
        i += 1;
    }
    i
}
