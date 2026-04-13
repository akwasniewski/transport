use hashbrown::{HashMap, HashSet};
use osmpbf::{ElementReader, Element};
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

// ── Public config ─────────────────────────────────────────────────────────────

pub struct ParseConfig {
    /// Drop nodes outside the largest weakly-connected component (mirrors osmnx).
    pub largest_component_only: bool,
}

impl Default for ParseConfig {
    fn default() -> Self {
        Self { largest_component_only: true }
    }
}

pub struct ParseResult {
    pub node_count: usize,
    pub edge_count: usize,
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Parse an OSM PBF file and write:
/// - `{output_dir}/{name}_snap.txt`   — `u v length_m` per line
/// - `{output_dir}/{name}_coords.txt` — `node lat lon` per line
pub fn parse_osm(
    input: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
    name: &str,
    config: ParseConfig,
) -> Result<ParseResult, Box<dyn std::error::Error>> {
    let input = input.as_ref();
    let output_dir = output_dir.as_ref();

    fs::create_dir_all(output_dir)?;

    eprint!("[1/5] scanning ways ...        ");
    let (needed_nodes, ways) = collect_way_nodes(input);
    eprintln!("{} ways, {} nodes", ways.len(), needed_nodes.len());

    eprint!("[2/5] loading coordinates ...  ");
    let coords = collect_coords(input, &needed_nodes);
    eprintln!("{} loaded", coords.len());

    eprint!("[3/5] building edges ...       ");
    let raw_edges = build_edges(&ways, &coords);
    drop(ways);
    let mut live_nodes: HashSet<i64> = HashSet::new();
    for &(u, v, _) in &raw_edges { live_nodes.insert(u); live_nodes.insert(v); }
    eprintln!("{} edges, {} nodes", raw_edges.len(), live_nodes.len());

    let keep_nodes: HashSet<i64> = if config.largest_component_only {
        eprint!("[4/5] largest component ...    ");
        let node_vec: Vec<i64> = live_nodes.iter().copied().collect();
        let wcc = largest_wcc(&node_vec, &raw_edges);
        eprintln!("{} nodes kept ({} removed)", wcc.len(), live_nodes.len() - wcc.len());
        wcc
    } else {
        eprintln!("[4/5] skipped (largest_component_only=false)");
        live_nodes
    };

    let filtered_edges: Vec<RawEdge> = raw_edges
        .into_par_iter()
        .filter(|(u, v, _)| keep_nodes.contains(u) && keep_nodes.contains(v))
        .collect();

    let mut sorted_nodes: Vec<i64> = keep_nodes.iter().copied().collect();
    sorted_nodes.sort_unstable();
    let label: HashMap<i64, usize> = sorted_nodes
        .iter()
        .enumerate()
        .map(|(i, &id)| (id, i))
        .collect();

    let mut best: HashMap<(usize, usize), f64> = HashMap::new();
    for (u_osm, v_osm, dist) in &filtered_edges {
        if let (Some(&u), Some(&v)) = (label.get(u_osm), label.get(v_osm)) {
            let e = best.entry((u, v)).or_insert(f64::INFINITY);
            if dist < e { *e = *dist; }
        }
    }

    eprint!("[5/5] writing files ...        ");
    let snap_path = output_dir.join(format!("{name}_snap.txt"));
    let mut w = BufWriter::new(File::create(&snap_path)?);
    for ((u, v), dist) in &best {
        writeln!(w, "{u} {v} {dist}")?;
    }

    let coords_path = output_dir.join(format!("{name}_coords.txt"));
    let mut w = BufWriter::new(File::create(&coords_path)?);
    for (new_id, &osm_id) in sorted_nodes.iter().enumerate() {
        if let Some(&(lat, lon)) = coords.get(&osm_id) {
            writeln!(w, "{new_id} {lat} {lon}")?;
        }
    }
    eprintln!("{} nodes, {} edges", sorted_nodes.len(), best.len());

    Ok(ParseResult {
        node_count: sorted_nodes.len(),
        edge_count: best.len(),
    })
}

// ── Internals ─────────────────────────────────────────────────────────────────

type RawEdge = (i64, i64, f64);

fn is_driveable(highway: &str) -> bool {
    matches!(
        highway,
        "motorway" | "trunk" | "primary" | "secondary" | "tertiary"
        | "unclassified" | "residential"
        | "motorway_link" | "trunk_link" | "primary_link"
        | "secondary_link" | "tertiary_link"
        | "living_street" | "road" | "busway"
    )
}

#[inline]
fn haversine_m(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6_371_000.0;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * R * a.sqrt().asin()
}

fn collect_way_nodes(path: &Path) -> (HashSet<i64>, Vec<(Vec<i64>, bool)>) {
    let reader = ElementReader::from_path(path).expect("cannot open PBF file");

    let (nodes, ways) = reader
        .par_map_reduce(
            |element| {
                let mut local_nodes: HashSet<i64> = HashSet::new();
                let mut local_ways: Vec<(Vec<i64>, bool)> = Vec::new();

                if let Element::Way(way) = element {
                    let mut highway_val: Option<&str> = None;
                    let mut oneway = false;
                    let mut junction_roundabout = false;
                    let mut access_blocked = false;

                    for (k, v) in way.tags() {
                        match k {
                            "highway" => highway_val = Some(v),
                            "oneway" => oneway = matches!(v, "yes" | "true" | "1"),
                            "junction" => junction_roundabout = matches!(v, "roundabout" | "circular"),
                            "access" => access_blocked = matches!(v, "no" | "private"),
                            _ => {}
                        }
                    }

                    if highway_val.map_or(false, is_driveable) && !access_blocked {
                        let node_ids: Vec<i64> = way.refs().collect();
                        if node_ids.len() >= 2 {
                            for &n in &node_ids { local_nodes.insert(n); }
                            local_ways.push((node_ids, oneway || junction_roundabout));
                        }
                    }
                }
                (local_nodes, local_ways)
            },
            || (HashSet::new(), Vec::new()),
            |(mut an, mut aw), (bn, bw)| { an.extend(bn); aw.extend(bw); (an, aw) },
        )
        .unwrap_or_default();

    (nodes, ways)
}

fn collect_coords(path: &Path, needed: &HashSet<i64>) -> HashMap<i64, (f64, f64)> {
    let reader = ElementReader::from_path(path).expect("cannot open PBF file");

    reader
        .par_map_reduce(
            |element| {
                let mut local: HashMap<i64, (f64, f64)> = HashMap::new();
                match element {
                    Element::Node(n) if needed.contains(&n.id()) => {
                        local.insert(n.id(), (n.lat(), n.lon()));
                    }
                    Element::DenseNode(n) if needed.contains(&n.id()) => {
                        local.insert(n.id(), (n.lat(), n.lon()));
                    }
                    _ => {}
                }
                local
            },
            HashMap::new,
            |mut a, b| { a.extend(b); a },
        )
        .unwrap_or_default()
}

fn build_edges(ways: &[(Vec<i64>, bool)], coords: &HashMap<i64, (f64, f64)>) -> Vec<RawEdge> {
    ways.par_iter()
        .flat_map(|(nodes, is_oneway)| {
            let mut edges = Vec::new();
            for w in nodes.windows(2) {
                let (u, v) = (w[0], w[1]);
                if let (Some(&(lat1, lon1)), Some(&(lat2, lon2))) =
                    (coords.get(&u), coords.get(&v))
                {
                    let dist = haversine_m(lat1, lon1, lat2, lon2);
                    edges.push((u, v, dist));
                    if !is_oneway { edges.push((v, u, dist)); }
                }
            }
            edges
        })
        .collect()
}

fn largest_wcc(node_ids: &[i64], edges: &[RawEdge]) -> HashSet<i64> {
    let idx: HashMap<i64, usize> =
        node_ids.iter().enumerate().map(|(i, &n)| (n, i)).collect();
    let n = node_ids.len();
    let mut parent: Vec<usize> = (0..n).collect();
    let mut rank: Vec<u8> = vec![0; n];

    fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
        while parent[x] != x { parent[x] = parent[parent[x]]; x = parent[x]; }
        x
    }
    fn union(parent: &mut Vec<usize>, rank: &mut Vec<u8>, a: usize, b: usize) {
        let (ra, rb) = (find(parent, a), find(parent, b));
        if ra == rb { return; }
        match rank[ra].cmp(&rank[rb]) {
            std::cmp::Ordering::Less    => parent[ra] = rb,
            std::cmp::Ordering::Greater => parent[rb] = ra,
            std::cmp::Ordering::Equal   => { parent[rb] = ra; rank[ra] += 1; }
        }
    }

    for &(u, v, _) in edges {
        if let (Some(&ui), Some(&vi)) = (idx.get(&u), idx.get(&v)) {
            union(&mut parent, &mut rank, ui, vi);
        }
    }

    let mut comp_size: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        *comp_size.entry(find(&mut parent, i)).or_insert(0) += 1;
    }
    let best_root = *comp_size.iter().max_by_key(|(_, sz)| *sz).unwrap().0;

    node_ids.iter().enumerate()
        .filter(|&(i, _)| find(&mut parent, i) == best_root)
        .map(|(_, &id)| id)
        .collect()
}
