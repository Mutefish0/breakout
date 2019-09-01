extern crate image;
use std::path::Path;
use std::os::raw::c_void;
use std::fs::File;
use std::io::{ Seek, SeekFrom, Read, BufReader };

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
      let fin = File::open(path).unwrap_or_else(|e| panic!("Failed to open file: {}", e));
      let mut fin = BufReader::new(fin);

      // 取前12个字节，根据文件签名获取文件格式
      // reference: https://en.wikipedia.org/wiki/List_of_file_signatures
      let mut prefix_bytes: [u8; 12] = [0; 12];
      fin.read_exact(&mut prefix_bytes).unwrap();
      // 重置指针
      fin.seek(SeekFrom::Start(0)).unwrap();
      let image_format = image::guess_format(&prefix_bytes).expect(&format!("Failed to guess_format: {}", src)[..]);

      let is_rgba = match image_format {
        image::ImageFormat::PNG => true,
        image::ImageFormat::JPEG => false,
        _ => false
      };

      let dyn_img = image::load(fin, image_format).expect(&format!("Failed to load texture: {}", src)[..]);

      let format = if is_rgba { gl::RGBA } else { gl::RGB };

      let (width, height, data) = if is_rgba {
        let img = dyn_img.to_rgba();
        (img.width(), img.height(), img.to_vec())
      } else { 
        let img = dyn_img.to_rgb();
        (img.width(), img.height(), img.to_vec())
      };
      gl::TexImage2D(gl::TEXTURE_2D,
                      0,
                      format as i32,
                      width as i32,
                      height as i32,
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