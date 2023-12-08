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

fn main() {
    const NUM_NODES: usize = 300;
    let mut nodes = Vec::with_capacity(NUM_NODES);
    let mut g = VecGraphBuilder::new();

    let g_nodes = g.add_nodes(NUM_NODES);

    for n1 in &g_nodes {
        for n2 in &g_nodes {
            if n1 != n2 {
                g.add_edge(*n1, *n2);
            }
        }
        nodes.push(N(n1.index()));
    }

    let graph: VecGraph<usize> = g.into_graph();

    println!("Start algorithm: ");
    let mut r = 1;

    // first round calculate own color / 2 and send to all others
    for node in &mut nodes {
        node.color = node.color.div_ceil(23);
        println!("node {} has new color {}", node.id, node.color);
    }

    for e in graph.edges() {
        let (u, v) = graph.enodes(e);
        let c = nodes[u.index()].color;
        nodes[v.index()].inbox.push((u.index(), c));
        //println!("node {}: sending to node {}: ({}, {})", u.index(), v.index(), u.index(), c);
    }
    r += 1;

    // loop until all discrepancies are fixed
    for _ in 0..NUM_NODES {
        println!("\nstarting round {r}");

        // process messages and calculate new color if needed
        for node in &mut nodes {
            println!("node {}: doing processing", node.id);
            let mut neighbor_color_set = HashSet::new();
            for (_, c) in &node.inbox {
                neighbor_color_set.insert(c);
            }

            // check is my color in the neighbor set
            if !neighbor_color_set.contains(&node.color) {
                println!("node {}: i have a good color :)", node.id);
                node.inbox.clear();
                continue;
            } else {
                println!("node {}: somebody is also using my color", node.id);
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
                println!("node {}: i am the problematic node :(", node.id);

                // find smallest number that is not in neighborcolor set
                let mut not_used = 0;
                while neighbor_color_set.contains(&not_used) {
                    not_used += 1;
                }

                node.color = not_used;
                println!("node {}: setting my new color", node.color);
            } else {
                println!("node {}: i am NOT the problematic node", node.id);
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

        r += 1;
    }

    println!("\nfinal colors:");
    nodes.sort_by(|n, b| n.color.cmp(&b.color));
    for node in &nodes {
        println!("node {:3} has color {:3}", node.id, node.color);
    }
}