use raytracer::camera::{Camera, Viewport};
use raytracer::geometry::{Point3, Sphere};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 1280.0;
    const VIEWPORT_WIDTH: f64 = 2.0;
    const FOCAL_LENGTH: f64 = 1.0;
    const SAMPLES: u8 = 100;

    let cam = Camera::new(
        Point3(0.0, 0.0, 0.0),
        Viewport::new(VIEWPORT_WIDTH, ASPECT_RATIO),
        FOCAL_LENGTH,
        IMAGE_WIDTH,
        ASPECT_RATIO,
        SAMPLES,
    );

    let right_sphere = Sphere::new(Point3(0.0, 0.0, -3.0), 0.5);
    let left_sphere = Sphere::new(Point3(-0.5, 0.0, -2.0), 0.5);
    let objects = vec![left_sphere, right_sphere];

    let img = cam.render(objects);
    img.save("./images/test.png").unwrap();
}
