mod camera;
mod hittable;
mod ray;
mod sphere;
mod utility;
mod vec;
use crate::{camera::Camera, hittable::HittableList, ray::Ray, vec::Vec3};

use std::{fs::File, process::exit};

use image::{ImageBuffer, RgbImage};

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

    let cam: Camera = Camera::new();
    let samples_per_pixel = 100;

    let mut world: HittableList = Default::default();
    world.add(sphere::Sphere {
        center: Vec3::new(0., 0., -1.),
        r: 0.5,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(0., -100.5, -1.),
        r: 100.,
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
                color += Ray::ray_color(r, &world);
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
