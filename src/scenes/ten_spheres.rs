use crate::hit::HittableList;
use crate::material;
use crate::objects::sphere::Sphere;
use crate::random_float;
use crate::Color;
use crate::Point3;
use std::rc::Rc;

/// Create a scene with five large spheres in the background and five glassy colored spheres in the
/// foreground.
#[allow(dead_code)]
pub fn ten_spheres() -> HittableList<f64> {
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
            x: 72.0 / 255.0,
            y: 72.0 / 255.0,
            z: 72.0 / 255.0,
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
            x: 253.0 / 255.0,
            y: 253.0 / 255.0,
            z: 255.0 / 255.0,
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

    // Sphere 1
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -3.363,
            y: 0.45,
            z: -2.705 - 0.5,
        },
        radius: 0.9,
        material: grey_material.clone(),
    }));

    // Sphere 2
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -1.84,
            y: 0.45,
            z: -4.028 - 0.5,
        },
        radius: 0.9,
        material: metal_material.clone(),
    }));

    // Sphere 3 (center)
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.45,
            z: -4.3 - 0.5,
        },
        radius: 0.9,
        material: default_material.clone(),
    }));

    // Sphere 4
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 1.84,
            y: 0.45,
            z: -4.028 - 0.5,
        },
        radius: 0.9,
        material: mirror_material.clone(),
    }));

    // Sphere 5
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 3.363,
            y: 0.45,
            z: -2.705 - 0.5,
        },
        radius: 0.9,
        material: metal_red_material.clone(),
    }));

    for s in 0..5 {
        // Glass sphere
        let s = s as f64;
        world.add(Box::new(Sphere {
            center: Point3 {
                x: -0.7 + s * 0.35,
                y: -0.28,
                z: 1.0,
            },
            radius: 0.15,
            material: Rc::new(material::Dielectric {
                ir: 1.5,
                albedo: Color {
                    x: random_float(),
                    y: random_float(),
                    z: random_float(),
                },
            }),
        }));
    }

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
