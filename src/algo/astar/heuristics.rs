pub fn earth_dist(
    coords: (f64, f64),
    target_coords: (f64, f64),
    _source_coords: (f64, f64),
) -> f64 {
    let r = 6371009.0; // earth's radius

    let coords = (coords.0.to_radians(), coords.1.to_radians());
    let target_coords = (target_coords.0.to_radians(), target_coords.1.to_radians());

    let delta_lat = coords.0 - target_coords.0;
    let mid_lat = (coords.0 + target_coords.0) / 2.0;
    let delta_long = coords.1 - target_coords.1;

    let x = delta_lat;
    let y = mid_lat.cos() * delta_long;
    let tunnel_dist = (x.powi(2) + y.powi(2)).sqrt();
    2.0 * r * (tunnel_dist / 2.0).asin()
}

pub fn rev<F>(dist_fn: F) -> impl Fn((f64, f64), (f64, f64), (f64, f64)) -> f64
where
    F: Fn((f64, f64), (f64, f64), (f64, f64)) -> f64 + Send + Sync + 'static,
{
    move |coords, target_coords, source_coords| -dist_fn(coords, target_coords, source_coords)
}

pub fn middle_dist<F>(
    dist_fn: F,
) -> (
    impl Fn((f64, f64), (f64, f64), (f64, f64)) -> f64,
    impl Fn((f64, f64), (f64, f64), (f64, f64)) -> f64,
)
where
    F: Fn((f64, f64), (f64, f64), (f64, f64)) -> f64 + Send + Sync + Copy + 'static,
{
    let forward = move |coords, target_coords, source_coords| {
        (dist_fn(coords, target_coords, source_coords)
            - dist_fn(source_coords, target_coords, source_coords))
            / 2.0
    };

    let backward = move |coords, target_coords, source_coords| {
        (dist_fn(target_coords, target_coords, source_coords)
            - dist_fn(coords, target_coords, source_coords))
            / 2.0
    };

    (forward, backward)
}
