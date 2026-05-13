use crate::{algo::{alt, alt_arc_flags, astar, astar_arc_flags, astar_bidirectional, dijkstra, dijkstra_arc_flags, dijkstra_bidirectional}, graph::Graph, utility};

pub fn preprocess_dijkstra(graph_prefix: &str){

    println!("Starting Preprocess....");
    let snap_path = format!("graphs/{}_snap.txt", graph_prefix);
    let coords_path = format!("graphs/{}_coords.txt", graph_prefix);
    let flags_path = format!("graphs/{}_flags.bin", graph_prefix);
    
    let mut graph = Graph::from_files(&snap_path, &coords_path);
    graph.divide_into_regions_dijkstra(64);
    graph.preprocess_region_edges(64, utility::EdgeDir::Forward);
    graph.preprocess_region_edges(64, utility::EdgeDir::Reverse);

    let _ = graph.save_edge_region_cache(flags_path);

    
    let farthest_landmarks_path = format!("graphs/{}_landmarks_farthest.bin", graph_prefix);
    let random_landmarks_path = format!("graphs/{}_landmarks_random.bin", graph_prefix);

    graph.get_farthest_landmarks(64);
    graph.save_landmarks(&farthest_landmarks_path);

    graph.get_random_landmarks(64);
    graph.save_landmarks(&random_landmarks_path);
    println!("\n")
}
pub fn benchmark(
    graph_prefix: &str,
    from: u32,
    to: u32){
        let snap_path = format!("graphs/{}_snap.txt", graph_prefix);
        let coords_path = format!("graphs/{}_coords.txt", graph_prefix);
        let flags_path = format!("graphs/{}_flags.bin", graph_prefix);
        let farthest_landmarks_path = format!("graphs/{}_landmarks_farthest.bin", graph_prefix);
        let random_landmarks_path = format!("graphs/{}_landmarks_random.bin", graph_prefix);
        let mut graph = Graph::from_files(&snap_path, &coords_path);

        let _ = graph.load_edge_region_cache(flags_path);
        println!("Starting Benchmark....");
        println!("\n===== Dijkstra =====");
        println!("Dijkstra: {}", dijkstra(&graph, from, to));
        println!("Dijkstra arc flags: {}", dijkstra_arc_flags(&graph, from, to));
        println!("Dijkstra bidirectional: {}", dijkstra_bidirectional(&graph, from, to));

        println!("\n===== A* =====");
        println!("Astar: {}", astar(&graph, from, to));
        println!("Astar arc flags: {}", astar_arc_flags(&graph, from, to));

        println!("Astar bidirectional: {}", astar_bidirectional(&graph, from, to));
        graph.load_landmarks(&farthest_landmarks_path);
        println!("\n===== Alt =====");
        println!("farthest: {}", alt(&graph, from, to));
        println!("farthest arc flags: {}", alt_arc_flags(&graph, from, to));
        graph.load_landmarks(&random_landmarks_path);
        println!("random: {}", alt(&graph, from, to));
        println!("random arc flags: {}", alt_arc_flags(&graph, from, to));
}
