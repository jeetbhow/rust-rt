use crate::geometry::{Point3, Ray, Sphere, Vector3};
use image::{Rgb, RgbImage};

pub struct Viewport {
    width: f64,
    height: f64,
}

impl Viewport {
    pub fn new(width: f64, height: f64) -> Viewport {
        Viewport { width, height }
    }
}

pub struct Camera {
    location: Point3,
    viewport: Viewport,
    focal_length: f64,
}

impl Camera {
    pub fn new(location: Point3, viewport: Viewport, focal_length: f64) -> Camera {
        Camera {
            location,
            viewport,
            focal_length,
        }
    }

    pub fn render(&self, img_width: f64, img_height: f64) -> RgbImage {
        let mut img = RgbImage::new(img_width as u32, img_height as u32);

        let sphere = Sphere::new(Point3(0.0, 0.0, -2.0), 0.5);

        let dx = self.viewport.width / img_width;
        let dy = self.viewport.height / img_height;
        let upper_left_px = Point3(
            -self.viewport.width / 2.0 + dx / 2.0,
            self.viewport.height / 2.0 + dy / 2.0,
            -self.focal_length,
        );

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let target_px = upper_left_px + Vector3(dx * x as f64, -dy * y as f64, 0.0);
            let ray = Ray::new(self.location, target_px - self.location);
            match sphere.hit(&ray) {
                Some(t) => {
                    let normal = sphere.normal(ray.at(t.0));
                    let intensity = Vector3::dot(normal, -ray.direction());
                    *pixel = Rgb([(intensity * 255.0) as u8, 0, 0])
                }
                None => *pixel = Rgb([0, 0, 0]),
            }
        }

        img
    }
}
