use crate::hit::HittableList;
use crate::material;
use crate::objects::sphere::Sphere;
use crate::vec::Color;
use crate::vec::Point3;
use std::rc::Rc;

/// Create a simple, efficient scene with three spheres.
#[allow(dead_code)]
pub fn scene() -> HittableList<f64> {
    let mut world = HittableList::new(Vec::new());

    // Materials

    let default_material = Rc::new(material::Lambertian {
        albedo: Color {
            x: 122.0 / 255.0,
            y: 175.0 / 255.0,
            z: 238.0 / 255.0,
        },
    });

    let grey_material = Rc::new(material::Lambertian {
        albedo: Color {
            x: 255.0 / 255.0,
            y: 255.0 / 255.0,
            z: 255.0 / 255.0,
        },
    });

    let ground_material = Rc::new(material::Lambertian {
        albedo: Color {
            x: 28.0 / 255.0,
            y: 28.0 / 255.0,
            z: 28.0 / 255.0,
        },
    });

    let metal_material = Rc::new(material::Metal {
        albedo: Color {
            x: 64.0 / 255.0,
            y: 64.0 / 255.0,
            z: 64.0 / 255.0,
        },
        fuzz: 0.1,
    });

    let mirror_material = Rc::new(material::Metal {
        albedo: Color {
            x: 224.0 / 255.0,
            y: 232.0 / 255.0,
            z: 245.0 / 255.0,
        },
        fuzz: 0.0,
    });

    let metal_red_material = Rc::new(material::Metal {
        albedo: Color {
            x: 208.0 / 255.0,
            y: 66.0 / 255.0,
            z: 70.0 / 255.0,
        },
        fuzz: 0.3,
    });

    // Mirror sphere
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 1.10,
            y: 0.6,
            z: -4.0,
        },
        radius: 1.0,
        material: mirror_material.clone(),
    }));

    // Blue sphere
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -1.3,
            y: 0.60,
            z: -2.9,
        },
        radius: 1.0,
        material: default_material.clone(),
    }));

    // Glass sphere
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.01,
            y: 0.83,
            z: -0.1,
        },
        radius: 0.22,
        material: Rc::new(material::Dielectric {
            ir: 1.5,
            albedo: Color {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }),
    }));

    // "World" sphere
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: -1000.45,
            z: -1.2,
        },
        radius: 1000.0,
        material: ground_material.clone(),
    }));

    world
}
