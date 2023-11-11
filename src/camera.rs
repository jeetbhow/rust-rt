use crate::geometry::{Hit, Point3, Ray, Sphere, Vector3};
use image::{Rgb, RgbImage};

pub struct Viewport {
    width: f64,
    aspect_ratio: f64,
}

impl Viewport {
    pub fn new(width: f64, aspect_ratio: f64) -> Viewport {
        Viewport {
            width,
            aspect_ratio,
        }
    }

    fn height(&self) -> f64 {
        self.width / self.aspect_ratio
    }
}

pub struct Camera {
    center: Point3,
    img_width: f64,
    aspect_ratio: f64,
    viewport: Viewport,
    focal_length: f64,
}

impl Camera {
    pub fn new(
        center: Point3,
        viewport: Viewport,
        focal_length: f64,
        img_width: f64,
        aspect_ratio: f64,
    ) -> Camera {
        Camera {
            center,
            viewport,
            img_width,
            aspect_ratio,
            focal_length,
        }
    }

    fn img_height(&self) -> f64 {
        self.img_width / self.aspect_ratio
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        self.viewport.aspect_ratio = aspect_ratio;
    }

    pub fn set_img_width(&mut self, img_width: f64) {
        self.img_width = img_width;
    }

    pub fn render(&self, objects: Vec<impl Hit>) -> RgbImage {
        let img_height = self.img_height();
        let mut img = RgbImage::new(self.img_width as u32, img_height as u32);

        let dx = self.viewport.width / self.img_width;
        let dy = self.viewport.height() / img_height;
        let upper_left_px = Point3(
            -self.viewport.width / 2.0 + dx / 2.0,
            self.viewport.height() / 2.0 + dy / 2.0,
            -self.focal_length,
        );

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let target_px = upper_left_px + Vector3(dx * x as f64, -dy * y as f64, 0.0);
            let ray = Ray::new(self.center, target_px - self.center);
            for object in &objects {
                if let Some(t) = object.hit(&ray) {
                    let normal = object.normal(ray.at(t.0));
                    let intensity = Vector3::dot(normal, -ray.direction());
                    let r = (intensity * 255.0) as u8;
                    *pixel = Rgb([r, 0, 0])
                }
            }
        }

        img
    }
}
