use crate::hit::HittableList;
use crate::material;
use crate::objects::sphere::Sphere;
use crate::random::{random_float, random_float_in_range};
use crate::vec::Color;
use crate::vec::Point3;
use std::rc::Rc;

#[allow(dead_code)]
pub fn scene() -> HittableList<f64> {
    let mut world = HittableList::new(Vec::new());

    // Ground

    let ground_material = Rc::new(material::Lambertian {
        albedo: Color {
            x: 80.0 / 255.0,
            y: 144.0 / 255.0,
            z: 22.0 / 255.0,
        },
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.0,
            z: 0.0,
        },
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    // Sphere 1

    let material1 = Rc::new(material::Dielectric {
        albedo: Color {
            x: 242.0 / 255.0,
            y: 111.0 / 255.0,
            z: 112.0 / 255.0,
        },
        ir: 1.5,
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material1.clone(),
    }));

    // Sphere 2

    let material2 = Rc::new(material::Lambertian {
        albedo: Color {
            x: 111.0 / 255.0,
            y: 165.0 / 255.0,
            z: 242.0 / 255.0,
        },
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: -4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material2.clone(),
    }));

    // Sphere 3

    let material3 = Rc::new(material::Metal {
        albedo: Color {
            x: 0.7,
            y: 0.6,
            z: 0.5,
        },
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 4.0,
            y: 1.0,
            z: 0.0,
        },
        radius: 1.0,
        material: material3.clone(),
    }));

    let boundary = Point3 {
        x: 4.0,
        y: 0.2,
        z: 0.0,
    };

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random_float();

            let center = Point3 {
                x: (a as f64) + 0.9 * random_float::<f64>(),
                y: 0.2,
                z: (b as f64) + 0.9 * random_float::<f64>(),
            };

            if (center - boundary).length() > 0.9 {
                if choose_mat < 0.66 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Rc::new(material::Lambertian { albedo });

                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material.clone(),
                    }));
                } else if choose_mat < 0.85 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_float_in_range(0.0, 0.5);
                    let sphere_material = Rc::new(material::Metal { albedo, fuzz });

                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material.clone(),
                    }));
                } else {
                    let albedo = Color::random_range(0.8, 1.0);
                    let sphere_material = Rc::new(material::Dielectric { albedo, ir: 1.5 });

                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: sphere_material.clone(),
                    }));
                }
            }
        }
    }

    world
}
