use crate::hit::HittableList;
use crate::material;
use crate::objects::sphere::Sphere;
use crate::vec::Color;
use crate::vec::Point3;
use std::sync::Arc;

/// Create a scene with five large spheres in the background and five glassy colored spheres in the
/// foreground.
#[allow(dead_code)]
pub fn scene() -> Arc<HittableList<f64>> {
    let mut world = HittableList::new(Vec::new());

    // Materials

    let default_material = Arc::new(material::Lambertian {
        albedo: Color {
            x: 122.0 / 255.0,
            y: 175.0 / 255.0,
            z: 238.0 / 255.0,
        },
    });

    let grey_material = Arc::new(material::Lambertian {
        albedo: Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
    });

    let ground_material = Arc::new(material::Lambertian {
        albedo: Color {
            x: 72.0 / 255.0,
            y: 72.0 / 255.0,
            z: 72.0 / 255.0,
        },
    });

    let metal_material = Arc::new(material::Metal {
        albedo: Color {
            x: 64.0 / 255.0,
            y: 64.0 / 255.0,
            z: 64.0 / 255.0,
        },
        fuzz: 0.1,
    });

    let mirror_material = Arc::new(material::Metal {
        albedo: Color {
            x: 253.0 / 255.0,
            y: 253.0 / 255.0,
            z: 1.0,
        },
        fuzz: 0.0,
    });

    let metal_red_material = Arc::new(material::Metal {
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
        material: grey_material,
    }));

    // Sphere 2
    world.add(Box::new(Sphere {
        center: Point3 {
            x: -1.84,
            y: 0.45,
            z: -4.028 - 0.5,
        },
        radius: 0.9,
        material: metal_material,
    }));

    // Sphere 3 (center)
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.45,
            z: -4.3 - 0.5,
        },
        radius: 0.9,
        material: default_material,
    }));

    // Sphere 4
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 1.84,
            y: 0.45,
            z: -4.028 - 0.5,
        },
        radius: 0.9,
        material: mirror_material,
    }));

    // Sphere 5
    world.add(Box::new(Sphere {
        center: Point3 {
            x: 3.363,
            y: 0.45,
            z: -2.705 - 0.5,
        },
        radius: 0.9,
        material: metal_red_material,
    }));

    world.add(Box::new(Sphere {
        center: Point3 {
            x: 0.0,
            y: 0.45,
            z: -1.0,
        },
        radius: 0.9,
        material: Arc::new(material::Dielectric {
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
        material: ground_material,
    }));

    Arc::new(world)
}
