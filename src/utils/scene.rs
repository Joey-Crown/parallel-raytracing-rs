use rand::{Rng, thread_rng};
use crate::utils::camera::Camera;
use crate::utils::geometry::{HittableList, Sphere};
use crate::utils::material::Material;
use crate::utils::vector::Vec3;

pub fn random_scene(aspect_ratio: f32, fov: f32) -> (HittableList, Camera) {
    let cam_origin = Vec3::new(13.0, 2.0, 3.0);
    let cam_direction = Vec3::new(0.0, 0.0, 0.0);
    let cam_up = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let mut rng = thread_rng();
    let mut world = HittableList::new(vec![]);

    let material_ground = Material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) };
    let ground_sphere = Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, material_ground));

    world.objects.push(ground_sphere);

    for i in -11..11 {
        for j in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(i as f32 + 0.9 * rng.gen::<f32>(), 0.2, j as f32 + 0.9 * rng.gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material: Material;

                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random(0.0..1.0) * Vec3::random(0.0..1.0);
                    material = Material::Lambertian { albedo };
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    material = Material::Metal { albedo, fuzz };
                } else {
                    // Glass
                    material = Material::Dielctric { ir: 1.5 };
                }

                let sphere = Box::new(Sphere::new(center, 0.2, material));
                world.objects.push(sphere);
            }
        }
    }

    let material1 = Material::Dielctric { ir: 1.5 };
    let material2 = Material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1) };
    let material3 = Material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0 };

    let sphere1 = Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));
    let sphere2 = Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));
    let sphere3 = Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    world.objects.push(sphere1);
    world.objects.push(sphere2);
    world.objects.push(sphere3);

    let camera = Camera::new(
        cam_origin,
        cam_direction,
        cam_up,
        fov,
        aspect_ratio,
        aperture,
        distance_to_focus
    );

    (world, camera)
}

pub fn default_scene(aspect: f32, fov: f32) -> (HittableList, Camera) {
    let cam_origin = Vec3::new(3.0, 3.0, 2.0);
    let cam_direction = Vec3::new(0.0, 0.0, -1.0);
    let cam_up = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = (cam_origin - cam_direction).length();
    let aperture = 2.0;

    let material_ground = Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) };
    let material_center = Material::Lambertian { albedo: Vec3::new(0.7, 0.3, 0.3) };
    let material_left = Material::Dielctric { ir: 1.5 };
    let material_left_inner = Material::Dielctric { ir: 1.5};
    let material_right = Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 1.0
    };

    let world = HittableList::new(vec![

        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left_inner)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right))

    ]);

    let camera = Camera::new(
        cam_origin,
        cam_direction,
        cam_up,
        fov,
        aspect,
        aperture,
        distance_to_focus
    );

    (world, camera)
}