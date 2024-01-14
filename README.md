# color-reduction

This is my implementation for a distributed randomized coloring algorithm
for the assignment 3 bonus exercise from Design and Analysis of Algorithms, WS23.

## How to build

This project was created with [Rust](https://www.rust-lang.org/) version 1.75.0.

1. Install Rust using [rustup](https://www.rust-lang.org/tools/install) (recommended)
2. Clone this repository `git clone https://github.com/thomaslienbacher/color-reduction.git`
3. Use `cargo build --release` in the repository folder, this will build and place the binary
   into `<repo>/target/release`
4. Alternatively use `cargo run --release` to build and run the project

Use `color-reduction --help` to get information on usage.

## Running

Running the program with no args will run the test case.
This will create a complete graph
with 200 nodes and then verify that each node has separate color
using assertions.

```shell
color-reduction 
```

Alternatively one can specify which graph should be generated
using the `-m` option, the values are
`complete-graph`, `chain` or `hydrocarbon`.
A chain graph is simply a graph where each node is
connected to next similar to a linked list (max degree is 2).
A hydrocarbon graph is similar to how hydrocarbon molecules
are made up, it's basically a chain of carbon atoms
with hydrogen attached to them.
The number of nodes in the graph can also be specified.
To get additional information about the algorithm execution
use the verbose flag `-v`.

## Examples

### Run on a complete graph with 100 nodes

```shell
color-reduction -m complete-graph -n 500
```

This takes around 100ms on my system.

#### Run on a chain graph with 3000 nodes

```shell
color-reduction -m chain -n 3000
```

This takes around 10ms on my system.

#### Run on a hydrocarbon graph with 4000 nodes

```shell
color-reduction -m chain -n 4000
```

This takes around 10ms on my system.

## Visualizing graph

Using the `-d` option one can specify a file
that will be used to generate a graphviz dot file.
This file can then be viewed using `xdot`
or converted to a pdf file using
`dot -Tpdf -o graph.pdf graph.dot`.
The graph nodes are colored using
a randomly generated palette of colors.

## Previous version

It also included my implementation for assignment 2
exercise B in previous commits.