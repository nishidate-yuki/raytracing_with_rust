use std::io::prelude::*;
use std::io::Result;
use std::fs::File;

struct Vec3 {
    x: f64,
    y: f64,
    z: f64
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }
}

struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {center: Vec3::new(), radius: 1.0}
    }
}

struct Ray {
    origin: Vec3,
    direction: Vec3
}

fn add(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3{
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z
    }
}

fn sub(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3{
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z
    }
}

fn mul(vec: &Vec3, val: &f64) -> Vec3 {
    Vec3{
        x: vec.x * val,
        y: vec.y * val,
        z: vec.z * val
    }
}

fn normalize(vec: &Vec3) -> Vec3 {
    let len = (vec.x * vec.x + vec.y * vec.y + vec.z * vec.z).sqrt();
    Vec3{
        x: vec.x / len,
        y: vec.y / len,
        z: vec.z / len
    }
}

fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

fn gen_ray(w: &i32, h: &i32, size: &i32) -> Ray {
    let pixel_pos = Vec3 { 
        x: *w as f64 / *size as f64 * 2.0 - 1.0,
        y: -(*h as f64 / *size as f64 * 2.0 - 1.0),
        z: -5.0,
    };
    Ray {
        origin: Vec3 {x: 0.0, y: 0.0, z: -10.0},
        direction: normalize(&sub(&pixel_pos, &Vec3 {x: 0.0, y: 0.0, z: -10.0}))
    }
}

fn intersect(ray: &Ray, sphere: &Sphere) -> (bool, Vec3, Vec3) {
    let a = dot(&ray.direction, &ray.direction);
    let b = dot(&sub(&ray.origin, &sphere.center), &ray.direction);
    let c = dot(&sub(&ray.origin, &sphere.center), &sub(&ray.origin, &sphere.center))
                - sphere.radius * sphere.radius;
    let d = b*b - a*c;

    if d < 0.0 {
        return (false, Vec3::new(), Vec3::new());
    }

    let dist = -b - d.sqrt();
    let pos = add(&ray.origin, &mul(&ray.direction, &dist));
    let normal = normalize(&sub(&pos, &sphere.center));
    (true, pos, normal)
}

fn shade(hit_pos: &Vec3, normal: &Vec3, light_pos: &Vec3) -> (i32, i32, i32) {
    let light_dir = normalize(&sub(&light_pos, &hit_pos));
    let shading = dot(&normal, &light_dir).max(0.0);
    let color = (shading * 255.0).round() as i32;
    (color, color, color)
}

fn main() -> Result<()> {
    let size = 512;
    let mut img = File::create("img.ppm")?;
    write!(img, "P3\n{} {}\n255\n", size, size)?;
    
    let sphere = Sphere::new();
    let light_pos = Vec3{x: 5.0, y: 5.0, z: -5.0};
    
    for h in 0..size {
        for w in 0..size {
            let mut color = (h/2, w/2, 128);
            let ray = gen_ray(&w, &h, &size);
            let (is_hit, hit_pos, normal) = intersect(&ray, &sphere);
            if is_hit {
                color = shade(&hit_pos, &normal, &light_pos);
            }
            write!(img, "{} {} {}\n", color.0, color.1, color.2)?;
        }
    }

    Ok(())
}
