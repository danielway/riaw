use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(self, samples_per_pixel: u32) {
        let mut r = self.x();
        let mut g = self.y();
        let mut b = self.z();

        let scale = 1.0 / samples_per_pixel as f64;
        r *= scale;
        g *= scale;
        b *= scale;

        r = linear_to_gamma_2(r);
        g = linear_to_gamma_2(g);
        b = linear_to_gamma_2(b);

        let intensity = Interval::new(0.000, 0.999);
        println!(
            "{} {} {}",
            (256.0 * intensity.clamp(r)) as u32,
            (256.0 * intensity.clamp(g)) as u32,
            (256.0 * intensity.clamp(b)) as u32
        );
    }
}

fn linear_to_gamma_2(x: f64) -> f64 {
    x.sqrt()
}
