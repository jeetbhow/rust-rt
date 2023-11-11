use crate::geometry::{Hit, Point3, Ray, Vector3};
use crate::utility::Pixel;
use image::{Rgb, RgbImage};
use rand::{distributions::Uniform, rngs::SmallRng};
use rand::{Rng, SeedableRng};

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
    samples: u8,
    dx: f64,
    dy: f64,
    upper_left_px: Point3,
}

impl Camera {
    pub fn new(
        center: Point3,
        viewport: Viewport,
        focal_length: f64,
        img_width: f64,
        aspect_ratio: f64,
        samples: u8,
    ) -> Camera {
        let dx = viewport.width / img_width;
        let dy = viewport.height() / (img_width / aspect_ratio);

        let upper_left_px = Point3(
            -viewport.width / 2.0 + dx / 2.0,
            viewport.height() / 2.0 + dy / 2.0,
            -focal_length,
        );

        Camera {
            center,
            viewport,
            img_width,
            aspect_ratio,
            samples,
            dx,
            dy,
            upper_left_px,
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

        let mut rng = SmallRng::from_entropy();
        let random_offsets_x: Vec<f64> =
            std::iter::repeat_with(|| rng.sample(Uniform::new(-self.dx, self.dx)))
                .take(self.samples as usize)
                .collect();

        let random_offsets_y: Vec<f64> =
            std::iter::repeat_with(|| rng.sample(Uniform::new(-self.dy, self.dy)))
                .take(self.samples as usize)
                .collect();

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let mut average = Pixel(0.0, 0.0, 0.0);

            let viewport_x = self.dx * x as f64;
            let viewport_y = -self.dy * y as f64;

            for i in 0..self.samples {
                let rndx = random_offsets_x[i as usize];
                let rndy = random_offsets_y[i as usize];

                let target_px =
                    self.upper_left_px + Vector3((viewport_x) + rndx, (viewport_y) + rndy, 0.0);

                let ray = Ray::new(self.center, target_px - self.center);
                let color = Camera::cast_ray(&ray, &objects);

                if let Some(c) = color {
                    average += c;
                } else {
                    break;
                }
            }

            average /= self.samples;

            *pixel = Rgb([
                (average.0 * 255.0) as u8,
                (average.1 * 255.0) as u8,
                (average.2 * 255.0) as u8,
            ]);
        }

        img
    }

    fn cast_ray(ray: &Ray, objects: &Vec<impl Hit>) -> Option<Pixel> {
        for object in objects {
            if let Some(t) = object.hit(&ray) {
                let normal = object.normal(ray.at(t.0));
                let intensity = Vector3::dot(normal, -ray.direction());
                return Some(Pixel(intensity as f32, 0.0, 0.0));
            }
        }
        None
    }
}
