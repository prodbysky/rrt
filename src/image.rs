use std::io::Write;

use crate::vector3::Vector3;

type Pixel = Vector3;

impl From<u32> for Pixel {
    fn from(value: u32) -> Self {
        Self {
            x: ((value >> 24) & 0xff) as f64 / 255.0,
            y: ((value >> 16) & 0xff) as f64 / 255.0,
            z: ((value >> 8) & 0xff) as f64 / 255.0,
        }
    }
}

pub struct Image {
    data: Vec<Pixel>,
    pub w: u16,
    pub h: u16,
}

impl Image {
    pub fn new(w: u16, h: u16) -> Self {
        Self {
            data: vec![Pixel::from(0x00000000); (w * h) as usize],
            w,
            h,
        }
    }

    pub fn write_ppm(&self, f: &mut impl Write) -> std::io::Result<()> {
        writeln!(f, "P3")?;
        writeln!(f, "{} {}", self.w, self.h)?;
        writeln!(f, "255")?;

        for Pixel { x, y, z } in &self.data {
            writeln!(
                f,
                "{} {} {}",
                (x * 255.0) as u8,
                (y * 255.0) as u8,
                (z * 255.0) as u8
            )?;
        }
        Ok(())
    }
}
