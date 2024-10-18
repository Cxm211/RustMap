use std::f64::consts::PI;

const R: f64 = 6371.0;
const TO_RAD: f64 = PI / 180.0;

fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let mut ph1 = ph1;
    let (th1, th2) = (th1 * TO_RAD, th2 * TO_RAD);
    ph1 = (ph1 - ph2) * TO_RAD;

    let dz = th1.sin() - th2.sin();
    let dx = ph1.cos() * th1.cos() - th2.cos();
    let dy = ph1.sin() * th1.cos();
    
    (0.5 * (dx * dx + dy * dy + dz * dz).sqrt()).asin() * 2.0 * R
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    /* Americans don't know kilometers */
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}
