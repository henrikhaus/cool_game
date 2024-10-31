use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    pub position: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub sensitivity: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub fn new(position: Point3<f32>, target: Point3<f32>, aspect_ratio: f32) -> Self {
        let front = (target - position).normalize();
        let right = front.cross(&Vector3::y_axis()).normalize();

        Self {
            position,
            front,
            up: Vector3::y_axis().into_inner(),
            right,
            yaw: -90.0,          // Initial yaw angle facing forward
            pitch: 0.0,           // Initial pitch angle (no tilt)
            speed: 0.05,
            sensitivity: 0.1,     // Adjust sensitivity to control rotation speed
            fov: 45.0,
            aspect_ratio,
            near: 0.1,
            far: 100.0,
        }
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at_rh(&self.position, &(&self.position + self.front), &self.up)
    }

    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Matrix4::new_perspective(self.aspect_ratio, self.fov.to_radians(), self.near, self.far)
    }

    pub fn move_forward(&mut self) {
        self.position += self.front * self.speed;
    }

    pub fn move_backward(&mut self) {
        self.position -= self.front * self.speed;
    }

    pub fn strafe_left(&mut self) {
        self.position -= self.right * self.speed;
    }

    pub fn strafe_right(&mut self) {
        self.position += self.right * self.speed;
    }

    pub fn rotate(&mut self, xoffset: f32, yoffset: f32) {
        self.yaw += xoffset * self.sensitivity;
        self.pitch += yoffset * self.sensitivity;

        // Constrain the pitch angle to avoid flipping the view
        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        // Update the front vector based on the updated yaw and pitch
        let front_x = self.yaw.to_radians().cos() * self.pitch.to_radians().cos();
        let front_y = self.pitch.to_radians().sin();
        let front_z = self.yaw.to_radians().sin() * self.pitch.to_radians().cos();

        self.front = Vector3::new(front_x, front_y, front_z).normalize();
        self.right = self.front.cross(&Vector3::y_axis()).normalize();
    }
}
