use crate::{
    camera::{Camera, CameraConfig, CameraConfigBuilder},
    hittable::{
        bvh_node::BvhNode, hittable_list::HittableList, moving_sphere::MovingSphere,
        sphere::Sphere, xy_rect::XYRect, Hittables,
    },
    material::{Dielectric, DiffuseLight, Lambertian, MaterialType, Metal},
    random::random_double,
    texture::{
        checker_texture::CheckerTexture, constant_texture::ConstantTexture,
        image_texture::ImageTexture, noise_texture::NoiseTexture, perlin::Perlin, TextureType,
    },
    vec3::Vec3,
};
use rand::Rng;

pub struct Scene {
    pub camera: Camera,
    pub hittables: Hittables,
}

pub fn get_scene_from_name(name: &str, rng: &mut impl Rng) -> Scene {
    let scene = match name {
        "two_spheres" => two_spheres(),
        "two_perlin_spheres" => two_perlin_spheres(),
        "random" => random_scene(rng),
        "earth" => earth(),
        "simple_light" => simple_light(),
        _ => random_scene(rng),
    };

    println!("{} scene generated", name);

    scene
}

fn default_config() -> CameraConfig {
    CameraConfigBuilder::default()
        .lookfrom(Vec3::new(13., 2., 3.))
        .lookat(Vec3::new(0., 0., 0.))
        .vfov(20.)
        .focus_dist(10.)
        .build()
        .unwrap()
}

fn default_camera() -> Camera {
    Camera::new(default_config())
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

pub fn earth() -> Scene {
    // To test this, use -n 1 -d 1
    let image = image::open("assets/textures/earthmap.jpg")
        .expect("earthmap.jpg not found")
        .to_rgb();
    let (nx, ny) = image.dimensions();

    let earth = Hittables::from(Sphere {
        center: Vec3::new(0., 0., 0.),
        radius: 2.,
        mat: MaterialType::from(Lambertian {
            albedo: TextureType::from(ImageTexture {
                data: image.into_raw(),
                nx,
                ny,
            }),
        }),
    });

    Scene {
        camera: default_camera(),
        hittables: earth,
    }
}

pub fn simple_light() -> Scene {
    let noise_texture = TextureType::from(NoiseTexture {
        perlin: Perlin,
        scale: 7.,
    });

    let light_mat = MaterialType::from(DiffuseLight {
        emit: TextureType::from(ConstantTexture {
            color: Vec3::newi(4, 4, 4),
        }),
    });

    let hittables = Hittables::from(HittableList {
        list: vec![
            Hittables::from(Sphere {
                center: Vec3::newi(0, -1000, 0),
                radius: 1000.,
                mat: MaterialType::from(Lambertian {
                    albedo: noise_texture.clone(),
                }),
            }),
            Hittables::from(Sphere {
                center: Vec3::newi(0, 2, 0),
                radius: 2.,
                mat: MaterialType::from(Lambertian {
                    albedo: noise_texture,
                }),
            }),
            Hittables::from(Sphere {
                center: Vec3::newi(0, 7, 0),
                radius: 2.,
                mat: light_mat.clone(),
            }),
            Hittables::from(XYRect::new(3., 5., 1., 3., -2., light_mat)),
        ],
    });

    let mut config = default_config();
    config.lookfrom = Vec3::newi(16, 3, 2);
    config.lookat = Vec3::new(0., 1., 0.);
    config.vfov = 40.;

    Scene {
        camera: Camera::new(config),
        hittables,
    }
}
