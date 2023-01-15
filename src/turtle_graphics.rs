use std::collections::VecDeque;

use macaw::{Mat4, Quat, Vec3};

/// A turtle that can be moved arround..
/// A turtle is an entity moving relative to it self.
/// Google 'turtle graphics' for more information.
#[derive(Clone, Copy)]
pub struct Turtle {
    /// The turtle's rotation/orientation.
    rotation: Mat4,
    scale: Mat4,
    origin: Vec3,
}

impl Turtle {
    pub fn new() -> Self {
        Self {
            rotation: Mat4::IDENTITY,
            scale: Mat4::IDENTITY,
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    /// Moves the turtle forward for its relative orientation.
    pub fn forward(&mut self, len: f32) {
        self.origin += self.transform(Vec3::new(0.0, len, 0.0));
    }

    /// Sets the origin of the turtle.
    pub fn set_origin(&mut self, postion: Vec3) {
        self.origin = postion;
    }

    pub fn scale(&mut self, scale: f32) {
        self.scale *= Mat4::from_scale(Vec3::new(scale, scale, scale));
    }

    /// Rotates the trurtle arround the z axis.
    pub fn rotate_z(&mut self, rotation_angle: f32) {
        self.rotation *= Mat4::from_rotation_z(rotation_angle);
    }

    /// Rotates the trurtle arround the x axis.
    pub fn rotate_x(&mut self, rotation_angle: f32) {
        self.rotation *= Mat4::from_rotation_x(rotation_angle);
    }

    /// Rotates the trurtle arround the y axis.
    pub fn rotate_y(&mut self, rotation_angle: f32) {
        self.rotation *= Mat4::from_rotation_y(rotation_angle);
    }

    /// Returns the origin position of the turret.
    pub fn origin(&mut self) -> Vec3 {
        self.origin
    }

    /// Transform the given position by applying the rotation and scale.
    fn transform(&self, position: Vec3) -> Vec3 {
        (self.scale * self.rotation).transform_point3(position)
    }

    pub fn rotation(&self) -> Quat {
        Quat::from_mat4(&self.rotation)
    }
}

/// The trutle transform stack stores turtle transforms for a given L-system.
/// Many L-systems use a transform stack to reset to a stored transform at the end of a recursion path.
pub struct TurtleTranformStack {
    transforms: VecDeque<Turtle>,
}

impl TurtleTranformStack {
    pub fn new() -> Self {
        Self {
            transforms: VecDeque::new(),
        }
    }
    pub fn push(&mut self, tranform: Turtle) {
        self.transforms.push_back(tranform);
    }

    pub fn pop(&mut self) -> Turtle {
        self.transforms.pop_back().unwrap()
    }
}
