use crate::shader::Shader;
use crate::texture::Texture;
extern crate gl;
use self::gl::types::*;
use cgmath::{Matrix4, vec3, Vector2, Vector3, Deg, ortho};
use std::os::raw::c_void;
use std::ptr;
use std::mem;
use std::ffi::CStr;

pub struct Sprite {
  shader: Shader,
  vao: GLuint,
  window_width: u32,
  window_height: u32,
}

impl Sprite {
  pub fn new(window_width: u32, window_height: u32) -> Sprite {
    let shader = Shader::new("src/sprite.vs", "src/sprite.fs");
    
    let vertices: [f32; 24] = [
      // 位置     // 纹理
      0.0, 1.0, 0.0, 1.0,
      1.0, 0.0, 1.0, 0.0,
      0.0, 0.0, 0.0, 0.0,

      0.0, 1.0, 0.0, 1.0,
      1.0, 1.0, 1.0, 1.0,
      1.0, 0.0, 1.0, 0.0
    ];

    let (mut vbo, mut vao) = (0, 0);
    
    unsafe {
      gl::GenVertexArrays(1, &mut vao);
      gl::GenBuffers(1, &mut vbo);

      gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW
      );
      let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
      gl::BindVertexArray(vao);
      gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, ptr::null());
      gl::EnableVertexAttribArray(0);
      gl::BindBuffer(gl::ARRAY_BUFFER, 0);  
      gl::BindVertexArray(0);
    }

    Sprite {
      window_width,
      window_height,
      shader,
      vao
    }
  }
  pub fn draw(&self, texture: &Texture, position: Vector2<f32>, size: Vector2<f32>, rotate: f32, color: Vector3<f32>) {
    unsafe {
      self.shader.useProgram();
      // 缩放
      let mut model = Matrix4::from_nonuniform_scale(size.x, size.y, 1.0);
      // 将中心移动到原点
      model = Matrix4::from_translation(vec3(-0.5 * size.x, -0.5 * size.y, 1.0)) * model;
      // 绕原点旋转
      model = Matrix4::from_angle_z(Deg(rotate)) * model;
      // 移动
      model = Matrix4::from_translation(vec3(position.x + 0.5 * size.x, position.y + 0.5 * size.y, 0.0)) * model;

      let projection = ortho(0.0, self.window_width as f32, self.window_height as f32, 0.0, -1.0, 1.0);

      self.shader.setMat4(c_str!("model"), &model);
      self.shader.setMat4(c_str!("projection"), &projection);
      self.shader.setVector3(c_str!("spriteColor"), &color);
      texture.bind();
      gl::BindVertexArray(self.vao);
      gl::DrawArrays(gl::TRIANGLES, 0, 6);
      gl::BindVertexArray(0);
    }
  }
}