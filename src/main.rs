use raytracer::camera::{Camera, Viewport};
use raytracer::geometry::{Point3, Sphere};

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: f64 = 400.0;
    const IMAGE_HEIGHT: f64 = IMAGE_WIDTH / ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    let camera = Camera::new(
        Point3(0.0, 0.0, 0.0),
        Viewport::new(2.0, 2.0 / ASPECT_RATIO),
        FOCAL_LENGTH,
    );

    let img = camera.render(IMAGE_WIDTH, IMAGE_HEIGHT);
    img.save("./images/test.png").unwrap();
}
