use crate::{
    camera::{Camera, CameraConfig, CameraConfigBuilder},
    hittable::{
        box_rect::BoxRect,
        bvh_node::BvhNode,
        flip_normals::FlipNormals,
        hittable_list::HittableList,
        moving_sphere::MovingSphere,
        rect::{Rect, StaticAxis},
        sphere::Sphere,
        Hittables,
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
        "cornell_box" => cornell_box_scene(),
        _ => cornell_box_scene(),
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
        odd: Box::new(ConstantTexture::new(0.2, 0.3, 0.1)),
        even: Box::new(ConstantTexture::new(0.9, 0.9, 0.9)),
    })
}

pub fn random_scene(rng: &mut impl Rng) -> Scene {
    let mut world = vec![
        Hittables::from(Sphere {
            center: Vec3::new(0., -1000., 0.),
            radius: 1000.,
            mat: Lambertian::new(default_checker()),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(0., 1., 0.),
            radius: 1.,
            mat: MaterialType::from(Dielectric { ref_idx: 1.5 }),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(-4., 1., 0.),
            radius: 1.,
            mat: Lambertian::new(ConstantTexture::new(0.4, 0.2, 0.1)),
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
                    x if (0.0..0.8).contains(&x) => Lambertian::new(ConstantTexture::new(
                        random_double(rng) * random_double(rng),
                        random_double(rng) * random_double(rng),
                        random_double(rng) * random_double(rng),
                    )),
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
                        center1: center + Vec3::new(0.0, 0.5 * random_double(rng), 0.0),
                        time: 0.0..1.0,
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
        hittables: BvhNode::new(world, 0.0, 1.0, 0),
    }
}

pub fn two_spheres() -> Scene {
    let hittables = HittableList::new(vec![
        Hittables::from(Sphere {
            center: Vec3::new(0.0, 10.0, 0.0),
            radius: 10.0,
            mat: Lambertian::new(default_checker()),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(0.0, -10.0, 0.0),
            radius: 10.0,
            mat: Lambertian::new(default_checker()),
        }),
    ]);

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

    let hittables = HittableList::new(vec![
        Hittables::from(Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            mat: Lambertian::new(noise_texture.clone()),
        }),
        Hittables::from(Sphere {
            center: Vec3::new(0.0, 2.0, 0.0),
            radius: 2.,
            mat: Lambertian::new(noise_texture),
        }),
    ]);

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
        center: Vec3::new(0.0, 0.0, 0.0),
        radius: 2.0,
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
        emit: ConstantTexture::new(4.0, 4.0, 4.0),
    });

    let hittables = HittableList::new(vec![
        Hittables::from(Sphere {
            center: Vec3::newi(0, -1000, 0),
            radius: 1000.,
            mat: Lambertian::new(noise_texture.clone()),
        }),
        Hittables::from(Sphere {
            center: Vec3::newi(0, 2, 0),
            radius: 2.,
            mat: Lambertian::new(noise_texture),
        }),
        Hittables::from(Sphere {
            center: Vec3::newi(0, 7, 0),
            radius: 2.,
            mat: light_mat.clone(),
        }),
        Rect::new(3.0..5.0, 1.0..3.0, -2.0, StaticAxis::Z, light_mat),
    ]);

    let mut config = default_config();
    config.lookfrom = Vec3::newi(16, 3, 2);
    config.lookat = Vec3::newi(0, 1, 0);
    config.vfov = 40.0;

    Scene {
        camera: Camera::new(config),
        hittables,
    }
}

pub fn cornell_box() -> Vec<Hittables> {
    let red = Lambertian::new(ConstantTexture::new(0.65, 0.05, 0.05));
    let green = Lambertian::new(ConstantTexture::new(0.12, 0.45, 0.15));
    let white = Lambertian::new(ConstantTexture::new(0.73, 0.73, 0.73));

    let light = DiffuseLight::new(ConstantTexture::new(15.0, 15.0, 15.0));

    let background_box = HittableList::new(vec![
        Rect::new(213.0..343.0, 227.0..332.0, 554.0, StaticAxis::Y, light),
        Rect::new(0.0..555.0, 0.0..555.0, 0.0, StaticAxis::Y, white.clone()), //floor
        FlipNormals::new(Rect::new(
            0.0..555.0,
            0.0..555.0,
            555.0,
            StaticAxis::Y,
            white.clone(),
        )), //ceiling
        FlipNormals::new(Rect::new(
            0.0..555.0,
            0.0..555.0,
            555.0,
            StaticAxis::Z,
            white.clone(),
        )), // rear wall
        Rect::new(0.0..555.0, 0.0..555.0, 0.0, StaticAxis::X, red),
        FlipNormals::new(Rect::new(
            0.0..555.0,
            0.0..555.0,
            555.0,
            StaticAxis::X,
            green,
        )),
    ]);

    vec![
        background_box,
        BoxRect::new(
            Vec3::newi(130, 0, 65),
            Vec3::newi(295, 165, 230),
            white.clone(),
        ),
        BoxRect::new(
            Vec3::newi(265, 0, 295),
            Vec3::newi(430, 330, 460),
            white.clone(),
        ),
    ]
}

pub fn cornell_box_scene() -> Scene {
    let mut cam_config = default_config();
    cam_config.lookfrom = Vec3::newi(278, 278, -800);
    cam_config.lookat = Vec3::newi(278, 278, 0);
    cam_config.focus_dist = 10.;
    cam_config.vfov = 40.;

    Scene {
        camera: Camera::new(cam_config),
        hittables: BvhNode::new(cornell_box(), 0.0, 1.0, 0),
    }
}
