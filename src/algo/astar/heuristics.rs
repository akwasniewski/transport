pub fn earth_dist(coords1: (f64, f64), coords2: (f64, f64)) -> f64 {
    let r = 6371009.0; // earth's radius

    let coords1 = (coords1.0.to_radians(), coords1.1.to_radians());
    let coords2 = (coords2.0.to_radians(), coords2.1.to_radians());

    let delta_lat = coords1.0 - coords2.0;
    let mid_lat = (coords1.0 + coords2.0) / 2.0;
    let delta_long = coords1.1 - coords2.1;

    let x = delta_lat;
    let y = mid_lat.cos() * delta_long;
    let tunnel_dist = (x.powi(2) + y.powi(2)).sqrt();
    2.0 * r * (tunnel_dist / 2.0).asin()
}
