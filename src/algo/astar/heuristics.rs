use crate::graph::Graph;

pub fn earth_dist(graph: &Graph, cur: u32, _from: u32, to: u32) -> f32 {
    let r = 6_371_000.0_f32;

    let (lat1, lon1) = graph[cur].coords;
    let (lat2, lon2) = graph[to].coords;

    let lat1 = lat1.to_radians();
    let lon1 = lon1.to_radians();
    let lat2 = lat2.to_radians();
    let lon2 = lon2.to_radians();

    let dlat = lat2 - lat1;
    let dlon = lon2 - lon1;

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().asin();

    r * c
}

pub fn rev<F>(potential: F) -> impl Fn(&Graph, u32, u32, u32) -> f32
where
    F: Fn(&Graph, u32, u32, u32) -> f32 + Send + Sync + 'static,
{
    move |graph, cur, from, to| -potential(graph, cur, from, to)
}

pub fn middle_dist<F>(
    dist_fn: F,
) -> (
    Box<dyn for<'a> Fn(&'a Graph, u32, u32, u32) -> f32 + Send + Sync>,
    Box<dyn for<'a> Fn(&'a Graph, u32, u32, u32) -> f32 + Send + Sync>,
)
where
    F: for<'a> Fn(&'a Graph, u32, u32, u32) -> f32 + Send + Sync + Copy + 'static,
{
    let forward = move |graph: &Graph, cur: u32, from: u32, to: u32| {
        (dist_fn(graph, cur, from, to) - dist_fn(graph, from, from, to)) / 2.0
    };
    let backward = move |graph: &Graph, cur: u32, from: u32, to: u32| {
        (dist_fn(graph, to, from, to) - dist_fn(graph, cur, from, to)) / 2.0
    };
    (Box::new(forward), Box::new(backward))
}

pub fn dijkstra_potential(_graph: &Graph, _cur: u32, _from: u32, _to: u32) -> f32{
    0.0
}
