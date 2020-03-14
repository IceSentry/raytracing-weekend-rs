use crate::{
    camera::Camera,
    hittable::{
        bvh_node::BvhNode, hittable_list::HittableList, moving_sphere::MovingSphere,
        sphere::Sphere, Hittables,
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, MaterialType},
    random::random_double,
    texture::{
        checker_texture::CheckerTexture, constant_texture::ConstantTexture,
        noise_texture::NoiseTexture, perlin::Perlin, TextureType,
    },
    vec3::Vec3,
    HEIGHT, WIDTH,
};
use rand::Rng;

pub struct Scene {
    pub camera: Camera,
    pub hittables: Hittables,
}

fn default_camera() -> Camera {
    let lookfrom = Vec3::new(13., 2., 3.);
    let lookat = Vec3::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let vfov = 20.;
    let ratio = WIDTH as f32 / HEIGHT as f32;
    let dist_to_focus = 10.;
    let aperture = 0.0;
    Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ratio,
        aperture,
        dist_to_focus,
        0.,
        1.,
    )
}

fn default_checker() -> TextureType {
    TextureType::from(CheckerTexture {
        odd: Box::new(TextureType::from(ConstantTexture {
            color: Vec3::new(0.2, 0.3, 0.1),
        })),
        even: Box::new(TextureType::from(ConstantTexture {
            color: Vec3::new(0.9, 0.9, 0.9),
        })),
    })
}

pub fn random_scene(rng: &mut impl Rng) -> Scene {
    let mut world = vec![
        Hittables::from(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            mat: MaterialType::from(Lambertian {
                albedo: default_checker(),
            }),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(0., 1., 0.),
            radius: 1.,
            mat: MaterialType::from(Dielectric { ref_idx: 1.5 }),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(-4., 1., 0.),
            radius: 1.,
            mat: MaterialType::from(Lambertian {
                albedo: TextureType::from(ConstantTexture {
                    color: Vec3::new(0.4, 0.2, 0.1),
                }),
            }),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(4., 1., 0.),
            radius: 1.,
            mat: MaterialType::from(Metal {
                albedo: Vec3::new(0.7, 0.6, 0.5),
                fuzz: 0.,
            }),
        }),
    ];

    (-11..11).for_each(|a| {
        (-11..11).for_each(|b| {
            let center = Vec3::new(
                a as f32 + 0.9 * random_double(rng),
                0.2,
                b as f32 + 0.9 * random_double(rng),
            );

            if (center - Vec3::new(4., 0.2, 0.)).norm() > 0.9 {
                let material = match random_double(rng) {
                    x if (0.0..0.8).contains(&x) => MaterialType::from(Lambertian {
                        albedo: TextureType::from(ConstantTexture {
                            color: Vec3::new(
                                random_double(rng) * random_double(rng),
                                random_double(rng) * random_double(rng),
                                random_double(rng) * random_double(rng),
                            ),
                        }),
                    }),
                    x if (0.8..0.95).contains(&x) => MaterialType::from(Metal {
                        albedo: Vec3::new(
                            0.5 * (1. + random_double(rng)),
                            0.5 * (1. + random_double(rng)),
                            0.5 * (1. + random_double(rng)),
                        ),
                        fuzz: 0.5 * random_double(rng),
                    }),
                    _ => MaterialType::from(Dielectric { ref_idx: 1.5 }),
                };

                let radius = 0.2;
                world.push(match material {
                    MaterialType::Lambertian(..) => Hittables::from(MovingSphere {
                        center0: center,
                        center1: center + Vec3::new(0., 0.5 * random_double(rng), 0.),
                        time0: 0.,
                        time1: 1.0,
                        radius,
                        material,
                    }),
                    _ => Hittables::from(Sphere {
                        center,
                        radius,
                        mat: material,
                    }),
                });
            }
        });
    });

    Scene {
        camera: default_camera(),
        hittables: Hittables::BvhNode(BvhNode::new(world, 0., 1., 0)),
    }
}

pub fn two_spheres() -> Scene {
    let hittables = Hittables::from(HittableList {
        list: vec![
            Hittables::from(Sphere {
                center: Vec3::new(0., 10., 0.),
                radius: 10.,
                mat: MaterialType::from(Lambertian {
                    albedo: default_checker(),
                }),
            }),
            Hittables::from(Sphere {
                center: Vec3::new(0., -10., 0.),
                radius: 10.,
                mat: MaterialType::from(Lambertian {
                    albedo: default_checker(),
                }),
            }),
        ],
    });

    Scene {
        camera: default_camera(),
        hittables,
    }
}

pub fn two_perlin_spheres() -> Scene {
    let noise_texture = TextureType::from(NoiseTexture {
        perlin: Perlin,
        scale: 7.,
    });

    let hittables = Hittables::from(HittableList {
        list: vec![
            Hittables::from(Sphere {
                center: Vec3::new(0., -1000., 0.),
                radius: 1000.,
                mat: MaterialType::from(Lambertian {
                    albedo: noise_texture.clone(),
                }),
            }),
            Hittables::from(Sphere {
                center: Vec3::new(0., 2., 0.),
                radius: 2.,
                mat: MaterialType::from(Lambertian {
                    albedo: noise_texture,
                }),
            }),
        ],
    });

    Scene {
        camera: default_camera(),
        hittables,
    }
}
