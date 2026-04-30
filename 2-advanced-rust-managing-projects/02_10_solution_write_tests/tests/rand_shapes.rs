use shapes::*;
use rand::prelude::*;

#[test]
fn it_rand_rectangle() {
    let mut rng = thread_rng();
    let w = rng.gen_range(0.0..10.0); // width
    let h = rng.gen_range(0.0..10.0); // height

    let rect = Rectangle::new(w, h);

    let area = rect.get_feature(Feature::Area);
    assert_eq!(area, w * h);

    let perimeter = rect.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, 2.0 * w + 2.0 * h);
}

#[test]
fn it_rand_circle() {
    let mut rng = thread_rng();
    let r = rng.gen_range(0.0..10.0); // radius

    let circ = Circle::new(r);

    let area = circ.get_feature(Feature::Area);
    assert_eq!(area, std::f64::consts::PI * r.powi(2));

    let perimeter = circ.get_feature(Feature::Perimeter);
    assert_eq!(perimeter, 2.0 * std::f64::consts::PI * r);
}