mod basic;
mod hittable;
mod material;
mod optimization;
mod texture;
mod utility;
use crate::{
    basic::{camera::Camera, ray::Ray, vec::Vec3},
    hittable::{sphere, HittableList},
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
};

use image::{ImageBuffer, RgbImage};
use texture::{
    checker::Checker,
    perlin::{NoiseTexture, Perlin},
    solid_color::SolidColor,
};

use std::{fs::File, process::exit, rc::Rc};

use console::style;
use indicatif::{ProgressBar, ProgressStyle};

fn two_spheres() -> HittableList {
    let mut world: HittableList = Default::default();
    let checker = Rc::new(Checker {
        odd: Rc::new(SolidColor::new(0.2, 0.3, 0.1)),
        even: Rc::new(SolidColor::new(0.9, 0.9, 0.9)),
    });
    let pertext = Rc::new(NoiseTexture {
        noise: Perlin::new(),
        scale: 4.,
    });
    let _mat1 = Rc::new(Lambertian { albedo: checker });
    let mat2 = Rc::new(Lambertian { albedo: pertext });

    world.add(sphere::Sphere {
        center: Vec3::new(0., -1000., 0.),
        r: 1000.,
        mat_ptr: mat2.clone(),
    });
    world.add(sphere::Sphere {
        center: Vec3::new(0., 2., 0.),
        r: 2.,
        mat_ptr: mat2,
    });

    world
}

pub fn random_scene() -> HittableList {
    let mut world: HittableList = Default::default();

    let checker = Rc::new(Checker {
        odd: Rc::new(SolidColor::new(0.2, 0.3, 0.1)),
        even: Rc::new(SolidColor::new(0.9, 0.9, 0.9)),
    }); //棋盘状的纹理

    let ground_material = Rc::new(Lambertian { albedo: checker });
    world.add(sphere::Sphere {
        center: Vec3::new(0., -1000., 0.),
        r: 1000.,
        mat_ptr: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utility::random_double(0., 1.);
            let center = Vec3::new(
                a as f64 + 0.9 * utility::random_double(0., 1.),
                0.2,
                b as f64 + 0.9 * utility::random_double(0., 1.),
            );
            if (center - Vec3::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    //diffuse
                    let sphere_material = Rc::new(Lambertian {
                        albedo: Rc::new(SolidColor {
                            color_value: Vec3::elemul(Vec3::random(0., 1.), Vec3::random(0., 1.)),
                        }),
                    });
                    world.add(sphere::MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0., utility::random_double(0., 0.5), 0.),
                        time0: 0.,
                        time1: 1.,
                        r: 0.2,
                        mat_ptr: sphere_material,
                    });
                } else if choose_mat < 0.95 {
                    //metal
                    let sphere_material = Rc::new(Metal {
                        albedo: Vec3::random(0.5, 1.),
                        fuzz: utility::random_double(0., 0.5),
                    });
                    world.add(sphere::Sphere {
                        center,
                        r: 0.2,
                        mat_ptr: sphere_material,
                    });
                } else {
                    //glass
                    let sphere_material = Rc::new(Dielectric { ref_idx: 1.5 });
                    world.add(sphere::Sphere {
                        center,
                        r: 0.2,
                        mat_ptr: sphere_material,
                    });
                }
            }
        }
    }
    let sphere_material = Rc::new(Dielectric { ref_idx: 1.5 });
    world.add(sphere::Sphere {
        center: Vec3::new(0., 1., 0.),
        r: 1.,
        mat_ptr: sphere_material,
    });

    let sphere_material = Rc::new(Lambertian {
        albedo: Rc::new(SolidColor {
            color_value: Vec3::new(0.4, 0.2, 0.1),
        }),
    });
    world.add(sphere::Sphere {
        center: Vec3::new(-4., 1., 0.),
        r: 1.,
        mat_ptr: sphere_material,
    });

    let sphere_material = Rc::new(Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.,
    });
    world.add(sphere::Sphere {
        center: Vec3::new(4., 1., 0.),
        r: 1.,
        mat_ptr: sphere_material,
    });

    world
}

fn main() {
    print!("{}[2J", 27 as char); // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Set cursor position as 1,1
                                                    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = (width as f64 / aspect_ratio) as u32;
    let quality = 60; // From 0 to 100
    let path = "output/output.jpg";

    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let focus_dist = 10.0; //(lookfrom - lookat).length();
    let aperture = 0.1;
    let cam: Camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        aspect_ratio,
        aperture,
        focus_dist,
        0.,
        1.,
    );
    let samples_per_pixel = 100;
    let max_depth = 50;

    let world: HittableList = two_spheres();

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
