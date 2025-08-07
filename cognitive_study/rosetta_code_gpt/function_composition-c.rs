type DoubleToDouble = Box<dyn Fn(f64) -> f64>;

struct Compose {
    f: DoubleToDouble,
    g: DoubleToDouble,
}

impl Compose {
    fn call(&self, x: f64) -> f64 {
        (self.f)((self.g)(x))
    }
}

fn compose(f: DoubleToDouble, g: DoubleToDouble) -> DoubleToDouble {
    let composed = Compose { f, g };
    Box::new(move |x| composed.call(x))
}

fn sin_call(x: f64) -> f64 {
    x.sin()
}

fn asin_call(x: f64) -> f64 {
    x.asin()
}

fn main() {
    let my_sin: DoubleToDouble = Box::new(|x| sin_call(x));
    let my_asin: DoubleToDouble = Box::new(|x| asin_call(x));

    let sin_asin = compose(my_sin, my_asin);

    println!("{:?}", sin_asin(0.5)); // prints "0.5"
}
