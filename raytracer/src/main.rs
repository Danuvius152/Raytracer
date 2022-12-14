mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec;
use crate::{
    camera::Camera,
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    vec::Vec3,
};

use image::{ImageBuffer, RgbImage};

use std::{fs::File, process::exit, rc::Rc};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
                                                    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;
    let quality = 60; // From 0 to 100
    let path = "output/output.jpg";

    let lookfrom = Vec3::new(3., 3., 2.);
    let lookat = Vec3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let focus_dist = (lookfrom - lookat).length();
    let aperture = 2.0;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        focus_dist,
    );
    let samples_per_pixel = 100;
    let max_depth = 50;

    //let r = (PI / 4.).cos();
    let mut world: HittableList = Default::default();
    let material_ground = Rc::new(Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    let material_left = Rc::new(Dielectric { ref_idx: 1.5 });
    // let material_left = Rc::new(Metal {
    //     albedo: Vec3::new(0.8, 0.8, 0.8),
    //     fuzz: 0.3,
    // });
    let material_right = Rc::new(Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(0., 0., -1.),
        r: 0.5,
        mat_ptr: material_center,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(0., -100.5, -1.),
        r: 100.,
        mat_ptr: material_ground,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(1., 0., -1.),
        r: 0.5,
        mat_ptr: material_right,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(-1., 0., -1.),
        r: 0.5,
        mat_ptr: material_left.clone(),
    });
    world.add(sphere::Sphere {
        center: Vec3::new(-1., 0., -1.),
        r: -0.45,
        mat_ptr: material_left,
    });

    println!(
        "Image size: {}\nJPEG quality: {}",
        style(width.to_string() + &"x".to_string() + &height.to_string()).yellow(),
        style(quality.to_string()).yellow(),
    );

    // Create image data
    let mut img: RgbImage = ImageBuffer::new(width, height);
    // Progress bar UI powered by library `indicatif`
    // Get environment variable CI, which is true for GitHub Action
    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    // Generate image
    for y in (0..height).rev() {
        for x in 0..width {
            let mut color = Vec3::new(0., 0., 0.);
            let mut i = 0;
            while i < samples_per_pixel {
                let u = (x as f64 + utility::random_double(0., 1.)) / width as f64;
                let v = (y as f64 + utility::random_double(0., 1.)) / height as f64;
                let r = Camera::get_ray(cam, u, v);
                color += Ray::ray_color(r, &world, max_depth);
                i += 1;
            }
            let pixel_color = utility::get_pixel_color(color, samples_per_pixel);
            let pixel = img.get_pixel_mut(x, height - y - 1);
            *pixel = image::Rgb(pixel_color);
            progress.inc(1);
        }
    }
    progress.finish();

    // Output image to file
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
