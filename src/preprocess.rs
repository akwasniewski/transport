use std::time::Instant;

use crate::{graph::Graph, utility};

pub fn preprocess_landmarks(graph_prefix: &str){
    let start = Instant::now();
    println!("Starting landmark preprocess....");

    let snap_path = format!("graphs/{}_snap.txt", graph_prefix);
    let coords_path = format!("graphs/{}_coords.txt", graph_prefix);
    
    let mut graph = Graph::from_files(&snap_path, &coords_path);


    
    let farthest_landmarks_path = format!("graphs/{}_landmarks_farthest.bin", graph_prefix);
    let random_landmarks_path = format!("graphs/{}_landmarks_random.bin", graph_prefix);

    graph.get_farthest_landmarks(64);
    graph.save_landmarks(&farthest_landmarks_path);

    graph.get_random_landmarks(64);
    graph.save_landmarks(&random_landmarks_path);
    println!("Landmark preprocess took {:?}\n", start.elapsed())
}

pub fn preprocess_flags(graph_prefix: &str){
    let start = Instant::now();
    println!("Starting flag preprocess....");

    let snap_path = format!("graphs/{}_snap.txt", graph_prefix);
    let coords_path = format!("graphs/{}_coords.txt", graph_prefix);
    let flags_path = format!("graphs/{}_flags.bin", graph_prefix);


    let mut graph = Graph::from_files(&snap_path, &coords_path);
    graph.divide_into_regions_dijkstra(128);
    graph.preprocess_region_edges(128, utility::EdgeDir::Forward);
    graph.preprocess_region_edges(128, utility::EdgeDir::Reverse);

    let _ = graph.save_edge_region_cache(flags_path);

    println!("Flag preprocess took {:?}\n", start.elapsed())
}

pub fn preprocess_contraction(graph_prefix: &str){
    let start = Instant::now();
    println!("Starting contraction preprocess....");

    let snap_path = format!("graphs/{}_snap.txt", graph_prefix);
    let coords_path = format!("graphs/{}_coords.txt", graph_prefix);
    let contraction_path = format!("graphs/{}_contraction.bin", graph_prefix);


    let mut graph = Graph::from_files(&snap_path, &coords_path);
    let added_edges = graph.contract();
    graph.save_contraction(&contraction_path).expect("Failed to save contraction paths");

    println!("Contraction added, {added_edges} edges, took {:?} \n", start.elapsed());
}
