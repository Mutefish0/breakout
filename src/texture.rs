extern crate image;
use image::GenericImage;
use std::path::Path;
use std::os::raw::c_void;
use std::ffi::OsStr;

pub struct Texture {
  pub id: u32
}

impl Texture {
  pub fn new(src: &str) -> Texture {
    let mut id = 0;
    unsafe {
      gl::GenTextures(1, &mut id);
      gl::BindTexture(gl::TEXTURE_2D, id); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
      // set the texture wrapping parameters
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
      // set texture filtering parameters
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
      gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
      // load image, create texture and generate mipmaps
      let path = Path::new(src);
      let format = if path.extension() == Some(OsStr::new("png")) { gl::RGBA } else { gl::RGB };
      let img = image::open(path).expect("Failed to load texture");
      let data = img.raw_pixels();

      println!("width: {}, height: {}", img.width(), img.height());
      let offset = 4 * (512 * 20 + 20);
      println!("pix1: {}, {}, {}, {}", data[offset + 0], data[offset + 1], data[offset + 2], data[offset + 3]);

      gl::TexImage2D(gl::TEXTURE_2D,
                      0,
                      format as i32,
                      img.width() as i32,
                      img.height() as i32,
                      0,
                      format,
                      gl::UNSIGNED_BYTE,
                      &data[0] as *const u8 as *const c_void);
      gl::GenerateMipmap(gl::TEXTURE_2D);
      gl::BindTexture(gl::TEXTURE_2D, 0);
    }
    Texture { id }
  }

  pub fn bind(&self) {
    unsafe {
      gl::ActiveTexture(gl::TEXTURE0);
      gl::BindTexture(gl::TEXTURE_2D, self.id);
    }
  }

}