# Transport
A collection of routing algorithms implemented for a class on algorithms in public transportation at TCS JU. 

## How to use
### Prerequisites
- modern rust installation
- python with osmnx and networkx liraries installed (only for generating graphs via the python script)
- 
### Running
After generating the graph source file (see sourcing sections) and accordingly adjusting the source paths in `main.rs` to direct to those files, 
executing `cargo run` should output the distances between arbitrarily chosen vertices calculated with all implemented algorithms as well as how many vertices were visited during calculation.
Afterwards a simulation gui should open where you can graphically choose which vertices to use as source and target and which algo to run on them.

## Algorithms
- `dijkstra`
- `bidirectional_dijkstra`
- `astar` - the A^* algorithm with few heuristics too choose from 
- `bidirectional_astar`

## Graph sourcing
I have a graph struct, which stores all the vertices and their info.

### Sourcing
- Graphs can be sourced as:
```rs
let graph = Graph::from_files("graphs/krakow_snap.txt", "graphs/krakow_coords.txt");
```
- this function source files should be in the following formats:
  * the first should list all the edges in the graph in snap format:
    ```
    source sink length
    ```
  * the second should list all the node coordinates:
    ```
    node_number x y
    ```

Those files can be generated usign the python script `graph_building/osm.py`.

## Visualisation tool
To help me debug (and generate endorphins) I implemented a visualisation tool: `visualize_algorithm(graph_arc: Arc<Graph>)`. 
> [!note]
> The visulisation tool is entirely vibecoded, use at your own risk
