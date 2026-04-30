pub struct Rectangle {
    width: f64,
    height: f64
}

pub struct Circle {
    radius: f64
}

pub enum Feature {
    Area,
    Perimeter
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle {width, height}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        self.width * self.height
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle {radius}
    }

    pub fn get_feature(&self, feature: Feature) -> f64 {
        match feature {
            Feature::Area => self.calc_area(),
            Feature::Perimeter => self.calc_perimeter()
        }
    }

    fn calc_area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }

    fn calc_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_rectangle() {
        let w = 1.2; // width
        let h = 3.4; // height
        let rect = Rectangle::new(w, h);

        // test public functions
        let area = rect.get_feature(Feature::Area);
        assert_eq!(area, w * h);

        let perimeter = rect.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 2.0 * w + 2.0 * h);

        // test private functions
        let area = rect.calc_area();
        assert_eq!(area, w * h);

        let perimeter = rect.calc_perimeter();
        assert_eq!(perimeter, 2.0 * w + 2.0 * h);
    }

    #[test]
    fn ut_circle() {
        let r = 5.6; // radius
        let circ = Circle::new(r);

        // test public functions
        let area = circ.get_feature(Feature::Area);
        assert_eq!(area, std::f64::consts::PI * r.powi(2));

        let perimeter = circ.get_feature(Feature::Perimeter);
        assert_eq!(perimeter, 2.0 * std::f64::consts::PI * r);

        // test private functions
        let area = circ.calc_area();
        assert_eq!(area, std::f64::consts::PI * r.powi(2));

        let perimeter = circ.calc_perimeter();
        assert_eq!(perimeter, 2.0 * std::f64::consts::PI * r);
    }
}