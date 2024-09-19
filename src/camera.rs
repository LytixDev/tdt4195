use glm::{vec3, Vec3};
use std::f32::consts::PI;

pub struct Camera {
    position: Vec3,
    speed: f32,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            position: vec3(0.0, 0.0, 0.0),
            speed: 1.5,
            yaw: -PI / 2.0,
            pitch: 0.0,
        }
    }

    pub fn to_projection(&self, perspective: glm::Mat4) -> glm::Mat4 {
        let forward: glm::Vec3= glm::vec3(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        );
        let right: glm::Vec3 = glm::normalize(&glm::cross(&glm::vec3(0.0, 1.0, 0.0), &forward));
        let up: glm::Vec3 = glm::normalize(&glm::cross(&forward, &right));

        let look_at: glm::Mat4 = glm::look_at(
            &self.position, 
            &(self.position + forward), 
            &up
        );

        let translate: glm::Mat4 = glm::translation(&glm::vec3(0.0, 0.0, -2.0));
        let projection: glm::Mat4 = look_at * perspective * translate;
        return projection;
    }

    pub fn move_forward(&mut self, delta: f32) {
        let forward: glm::Vec3 = glm::vec3(
            self.yaw.cos(),
            0.0,
            self.yaw.sin(),
        );

        self.position += forward * self.speed * delta;
    }

    pub fn move_up(&mut self, delta: f32) {
        self.position += glm::vec3(0.0, 1.0, 0.0) * self.speed * delta;
    }

    pub fn move_side(&mut self, delta: f32) {
        let side: glm::Vec3 = glm::vec3(
            self.yaw.sin(),
            0.0,
            self.yaw.cos(),
        );
        
        self.position += side * self.speed * delta;
    }

    pub fn rotate_right(&mut self, delta: f32) {
        self.yaw += delta;
    }

    pub fn rotate_up(&mut self, delta: f32) {
        self.pitch += delta;
    }
}