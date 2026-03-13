fn main() {
    let Ok((nodes, edges)) = osm4routing::read("map.osm.pbf") else {panic!("dfd")};
    println!("{:?}", edges[0]);
}
