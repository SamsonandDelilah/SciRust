// SciRust/mathematics/geometry/src/shape_properties.rs
/// 1. Protected central shape definitions for the entire workspace.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum Shape {
    /// Kugel (Sphere)
    Sphere { radius: f64 },
    /// Quader
    Cuboid { width: f64, height: f64, depth: f64 },
    /// Kreiszylinder (Solid Cylinder)
    Cylinder { radius: f64, height: f64 },
    /// Hohlkugel
    HollowSphere { inner_radius: f64, outer_radius: f64 },
    /// Halbkugel
    Hemisphere { radius: f64 },
    /// Kreiskegel (Circular Cone)
    Cone { radius: f64, height: f64 },
    /// Kreiskegelstumpf (Frustum of a Cone)
    ConeFrustum { bottom_radius: f64, top_radius: f64, height: f64 },
    /// Kreistorus (Circular Torus)
    Torus { major_radius: f64, minor_radius: f64 },
    /// Hohlzylinder (Hollow Cylinder / Pipe)
    HollowCylinder { inner_radius: f64, outer_radius: f64, height: f64 },
    /// Rechteckpyramide (Rectangular Pyramid)
    RectangularPyramid { width: f64, depth: f64, height: f64 },
    /// Ellipsoid
    Ellipsoid { semi_axis_a: f64, semi_axis_b: f64, semi_axis_c: f64 },
    /// Dünner Stab (Thin Rod)
    ThinRod { area: f64, length: f64 },
    /// Solid of revolution defined by height and a radius function r(z).
    SolidOfRevolution { 
        height: f64,
        /// Radius profile function: takes z (0.0 to height) and returns radius r(z)
        profile: fn(f64) -> f64,
    }, 
}

/// 2. Geometric properties trait implementation.
pub trait GeometricProperties {
    fn surface_area(&self) -> f64;
    fn volume(&self) -> f64;
    fn center_of_mass(&self) -> [f64; 3];
}

// mathematics/geometry/src/shape_properties.rs

/// 1. Complete implementation of surface area for all shape variants.
impl GeometricProperties for Shape {
    fn surface_area(&self) -> f64 {
        match self {
            Shape::Sphere { radius } => 4.0 * std::f64::consts::PI * radius.powi(2),
            Shape::Cuboid { width, height, depth } => 2.0 * (width * height + height * depth + width * depth),
            Shape::Cylinder { radius, height } => {
                2.0 * std::f64::consts::PI * radius * height + 2.0 * std::f64::consts::PI * radius.powi(2)
            }
            Shape::HollowSphere { inner_radius, outer_radius } => {
                4.0 * std::f64::consts::PI * (outer_radius.powi(2) + inner_radius.powi(2))
            }
            Shape::Hemisphere { radius } => {
                // Curved surface (2 * pi * r^2) + circular base (pi * r^2)
                3.0 * std::f64::consts::PI * radius.powi(2)
            }
            Shape::Cone { radius, height } => {
                let s = (radius.powi(2) + height.powi(2)).sqrt(); // slant height
                std::f64::consts::PI * radius * (radius + s)
            }
            Shape::ConeFrustum { bottom_radius, top_radius, height } => {
                let s = ((bottom_radius - top_radius).powi(2) + height.powi(2)).sqrt(); // slant height
                std::f64::consts::PI * (bottom_radius + top_radius) * s 
                    + std::f64::consts::PI * bottom_radius.powi(2) 
                    + std::f64::consts::PI * top_radius.powi(2)
            }
            Shape::Torus { major_radius, minor_radius } => {
                4.0 * std::f64::consts::PI.powi(2) * major_radius * minor_radius
            }
            Shape::HollowCylinder { inner_radius, outer_radius, height } => {
                // Outer lateral + Inner lateral + 2 * (Outer base ring - Inner base ring)
                2.0 * std::f64::consts::PI * outer_radius * height 
                    + 2.0 * std::f64::consts::PI * inner_radius * height 
                    + 2.0 * std::f64::consts::PI * (outer_radius.powi(2) - inner_radius.powi(2))
            }
            Shape::RectangularPyramid { width, depth, height } => {
                let slant_a = (width / 2.0).hypot(*height);
                let slant_b = (depth / 2.0).hypot(*height);
                width * depth + width * slant_b + depth * slant_a
            }
            Shape::Ellipsoid { semi_axis_a, semi_axis_b, semi_axis_c } => {
                // Knud Thomsen's approximation formula for ellipsoid surface area (p ≈ 1.6075)
                let p = 1.6075;
                let term = (semi_axis_a.powf(p) * semi_axis_b.powf(p) 
                    + semi_axis_a.powf(p) * semi_axis_c.powf(p) 
                    + semi_axis_b.powf(p) * semi_axis_c.powf(p)) / 3.0;
                4.0 * std::f64::consts::PI * term.powf(1.0 / p)
            }
            Shape::ThinRod { area, length } => {
                // A thin mathematical rod has zero cross-sectional thickness contribution to surface area
                area * 1.0
            }
            Shape::SolidOfRevolution { height, profile } => {
                // Numerical approximation of surface area of revolution using trapezoidal rule: 2 * pi * integral(r(z) * sqrt(1 + (r'(z))^2) dz)
                let steps = 1000;
                let dz = height / steps as f64;
                let mut area = 0.0;
                
                let r_prime = |z: f64| -> f64 {
                    let h_step = 1e-5;
                    (profile(z + h_step) - profile(z - h_step)) / (2.0 * h_step)
                };

                for i in 0..steps {
                    let z0 = i as f64 * dz;
                    let z1 = (i + 1) as f64 * dz;
                    
                    let r0 = profile(z0);
                    let r1 = profile(z1);
                    
                    let ds0 = (1.0 + r_prime(z0).powi(2)).sqrt();
                    let ds1 = (1.0 + r_prime(z1).powi(2)).sqrt();
                    
                    area += 0.5 * (r0 * ds0 + r1 * ds1) * dz;
                }
                
                2.0 * std::f64::consts::PI * area
            }
        }
    }


    /// 1. Complete implementation of volume for all shape variants.
    fn volume(&self) -> f64 {
        match self {
            Shape::Sphere { radius } => (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3),
            Shape::Cuboid { width, height, depth } => width * height * depth,
            Shape::Cylinder { radius, height } => std::f64::consts::PI * radius.powi(2) * height,
            Shape::HollowSphere { inner_radius, outer_radius } => {
                (4.0 / 3.0) * std::f64::consts::PI * (outer_radius.powi(3) - inner_radius.powi(3))
            }
            Shape::Hemisphere { radius } => {
                (2.0 / 3.0) * std::f64::consts::PI * radius.powi(3)
            }
            Shape::Cone { radius, height } => {
                (1.0 / 3.0) * std::f64::consts::PI * radius.powi(2) * height
            }
            Shape::ConeFrustum { bottom_radius, top_radius, height } => {
                (1.0 / 3.0) * std::f64::consts::PI * height 
                    * (bottom_radius.powi(2) + bottom_radius * top_radius + top_radius.powi(2))
            }
            Shape::Torus { major_radius, minor_radius } => {
                2.0 * std::f64::consts::PI.powi(2) * major_radius * minor_radius.powi(2)
            }
            Shape::HollowCylinder { inner_radius, outer_radius, height } => {
                std::f64::consts::PI * (outer_radius.powi(2) - inner_radius.powi(2)) * height
            }
            Shape::RectangularPyramid { width, depth, height } => {
                (1.0 / 3.0) * width * depth * height
            }
            Shape::Ellipsoid { semi_axis_a, semi_axis_b, semi_axis_c } => {
                (4.0 / 3.0) * std::f64::consts::PI * semi_axis_a * semi_axis_b * semi_axis_c
            }
            Shape::ThinRod { area, length } => {
                // A thin rod has zero cross-sectional volume in ideal geometry modeling
                area * length
            }
            Shape::SolidOfRevolution { height, profile } => {
                // Numerical integration of volume using the trapezoidal rule: pi * integral(r(z)^2 dz)
                let steps = 1000;
                let dz = height / steps as f64;
                let mut sum = 0.5 * (profile(0.0).powi(2) + profile(*height).powi(2));

                for i in 1..steps {
                    let z = i as f64 * dz;
                    sum += profile(z).powi(2);
                }

                std::f64::consts::PI * sum * dz
            }
        }
    }


    /// 1. Complete implementation of center of mass for all shape variants.
    fn center_of_mass(&self) -> [f64; 3] {
        match self {
            Shape::Sphere { .. }
            | Shape::HollowSphere { .. }
            | Shape::Torus { .. }
            | Shape::Ellipsoid { .. } => [0.0, 0.0, 0.0],

            Shape::Cuboid { width, height, depth } => {
                [width / 2.0, height / 2.0, depth / 2.0]
            }
            Shape::Cylinder { height, .. }
            | Shape::HollowCylinder { height, .. } => {
                [0.0, 0.0, height / 2.0]
            }
            Shape::Hemisphere { radius } => {
                // Center of mass along the axis of symmetry for a solid hemisphere
                [0.0, 0.0, (3.0 * radius) / 8.0]
            }
            Shape::Cone { height, .. } => {
                [0.0, 0.0, height / 4.0]
            }
            Shape::ConeFrustum { bottom_radius, top_radius, height } => {
                let r1 = bottom_radius;
                let r2 = top_radius;
                let z_cm = (height / 4.0) * (r1.powi(2) + 2.0 * r1 * r2 + 3.0 * r2.powi(2)) 
                    / (r1.powi(2) + r1 * r2 + r2.powi(2));
                [0.0, 0.0, z_cm]
            }
            Shape::RectangularPyramid { height, .. } => {
                [0.0, 0.0, height / 4.0]
            }
            Shape::ThinRod { area, length } => {
                [0.0, 0.0, length / 2.0]
            }
            Shape::SolidOfRevolution { height, profile } => {
                // Numerical integration for center of mass along the z-axis: z_cm = (1 / V) * pi * integral(z * r(z)^2 dz)
                let steps = 1000;
                let dz = height / steps as f64;
                let mut weighted_sum = 0.5 * (0.0 * profile(0.0).powi(2) + height * profile(*height).powi(2));

                for i in 1..steps {
                    let z = i as f64 * dz;
                    weighted_sum += z * profile(z).powi(2);
                }

                let volume = self.volume();
                let z_cm = if volume.abs() > 1e-14 {
                    std::f64::consts::PI * weighted_sum * dz / volume
                } else {
                    height / 2.0
                };

                [0.0, 0.0, z_cm]
            }
        }
    }
}