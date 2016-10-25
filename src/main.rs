// Calculate the haversine distance between two points

struct Haversine {
    lat1: f64,
    lat2: f64,
    long1: f64,
    long2: f64,
}

impl Haversine {
    fn distance(&self) -> f64 {
        // a = sin²(Δφ/2) + cos φ1 ⋅ cos φ2 ⋅ sin²(Δλ/2)
        // c = 2 ⋅ atan2( √a, √(1−a) )
        // distance = r ⋅ c
        // where φ is latitude, λ is longitude, r is earth’s radius (mean radius = 6,371km);
        let r = 6371.00;
        let lat1_rad = self.lat1.to_radians();
        let lat2_rad = self.lat2.to_radians();
        let long1_rad = self.long1.to_radians();
        let long2_rad = self.long2.to_radians();

        let a = ( (lat2_rad - lat1_rad) / 2.00).sin().powf(2.00) + lat1_rad.cos() * lat2_rad.cos() * ( (long2_rad - long1_rad) / 2.00).sin().powf(2.00);
        let c = 2.00 * ((a).sqrt().atan2((1.00 - a).sqrt()));
        r * c
    }
}

fn main() {
    let a = Haversine { lat1:38.898556, lat2:38.897147, long1:-77.037852, long2:-77.043934};
    println!("{}", a.distance());
}
