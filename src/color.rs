use std::fmt;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Color {
            r: r.clamp(0, 255) as u8,
            g: g.clamp(0, 255) as u8,
            b: b.clamp(0, 255) as u8,
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;
        Color::new(r.into(), g.into(), b.into())
    }

    pub fn from_hex_str(hex_str: &str) -> Result<Color, String> {
        let hex_str = hex_str.trim_start_matches('#');

        if hex_str.len() != 6 {
            return Err("Hex string must be exactly 6 characters long".to_string());
        }

        let hex = u32::from_str_radix(hex_str, 16)
            .map_err(|_| "Failed to parse hex string".to_string())?;

        Ok(Color::from_hex(hex))
    }

    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        let r = self.r.saturating_add(other.r);
        let g = self.g.saturating_add(other.g);
        let b = self.b.saturating_add(other.b);
        Color::new(r.into(), g.into(), b.into())
    }
}

impl std::ops::Mul<f32> for Color {
    type Output = Color;

    fn mul(self, factor: f32) -> Color {
        let r = (self.r as f32 * factor).clamp(0.0, 255.0) as u8;
        let g = (self.g as f32 * factor).clamp(0.0, 255.0) as u8;
        let b = (self.b as f32 * factor).clamp(0.0, 255.0) as u8;
        Color::new(r.into(), g.into(), b.into())
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        let r = (self.r as i16 - other.r as i16).clamp(0, 255) as u8;
        let g = (self.g as i16 - other.g as i16).clamp(0, 255) as u8;
        let b = (self.b as i16 - other.b as i16).clamp(0, 255) as u8;
        Color::new(r.into(), g.into(), b.into())
    }
}
