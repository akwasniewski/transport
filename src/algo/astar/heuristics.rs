use crate::graph::Graph;

pub fn earth_dist(graph: &Graph, cur: u32, _from: u32, to: u32) -> f32 {
    let r = 6371009.0; // earth's radius

    let cur_coords = graph[cur].coords;
    let cur_coords = (cur_coords.0.to_radians(), cur_coords.1.to_radians());
    let target_coords = graph[to].coords;
    let target_coords = (target_coords.0.to_radians(), target_coords.1.to_radians());

    let delta_lat = cur_coords.0 - target_coords.0;
    let mid_lat = (cur_coords.0 + target_coords.0) / 2.0;
    let delta_long = cur_coords.1 - target_coords.1;

    let x = delta_lat;
    let y = mid_lat.cos() * delta_long;
    let tunnel_dist = (x.powi(2) + y.powi(2)).sqrt();
    2.0 * r * (tunnel_dist / 2.0).asin()
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
