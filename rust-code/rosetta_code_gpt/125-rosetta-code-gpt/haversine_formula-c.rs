const R: f64 = 6371.0;
const TO_RAD: f64 = std::f64::consts::PI / 180.0;

fn dist(th1: f64, ph1: f64, th2: f64, ph2: f64) -> f64 {
    let mut dx: f64;
    let mut dy: f64;
    let mut dz: f64;
    let mut ph1 = ph1;
    ph1 -= ph2;
    let (ph1, th1, th2) = (ph1 * TO_RAD, th1 * TO_RAD, th2 * TO_RAD);

    dz = th1.sin() - th2.sin();
    dx = (ph1.cos() * th1.cos()) - th2.cos();
    dy = ph1.sin() * th1.cos();
    (dz * dz + dx * dx + dy * dy).sqrt() / 2.0_f64).asin() * 2.0 * R
}

fn main() {
    let d = dist(36.12, -86.67, 33.94, -118.4);
    println!("dist: {:.1} km ({:.1} mi.)", d, d / 1.609344);
}