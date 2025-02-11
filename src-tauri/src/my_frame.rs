use std::mem::size_of;

// my frame format:
//
//    0                   1
//    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  X high bits  |  X low bits   |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  Y high bits  |  Y low bits   |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//   |  Z high bits  |  Z low bits   |
//   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

pub struct MyFrame {
    x_h: u8,
    x_l: u8,
    y_h: u8,
    y_l: u8,
    z_h: u8,
    z_l: u8,
}

impl MyFrame {
    pub fn new() -> Self {
        Self {
            x_h: 0, x_l: 0,
            y_h: 0, y_l: 0,
            z_h: 0, z_l: 0
        }
    }

    pub fn from_fixed(buf: &[u8; size_of::<Self>()]) -> Self {
        Self {
            x_h: buf[0], x_l: buf[1],
            y_h: buf[2], y_l: buf[3],
            z_h: buf[4], z_l: buf[5]
        }
    }

    pub fn from_vec(buf: &Vec<u8>) -> Result<Self, ()> {
        if buf.len() < size_of::<Self>() {
            Err(())
        } else {
            Ok(Self {
                x_h: buf[0], x_l: buf[1],
                y_h: buf[2], y_l: buf[3],
                z_h: buf[4], z_l: buf[5]
            })
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![
            self.x_h, self.x_l,
            self.y_h, self.y_l,
            self.z_h, self.z_l,
        ]
    }

    pub fn to_string(&self) -> String {
        format!("x: {}, y: {}, z: {}", self.get_x(), self.get_y(), self.get_z())
    }

    pub fn get_x(&self) -> i16 {
        (((self.x_h as u16) << 8) | (self.x_l as u16)) as i16
    }

    pub fn get_y(&self) -> i16 {
        (((self.y_h as u16) << 8) | (self.y_l as u16)) as i16
    }

    pub fn get_z(&self) -> i16 {
        (((self.z_h as u16) << 8) | (self.z_l as u16)) as i16
    }

    pub fn as_bytes(&self) -> [u8; size_of::<Self>()] {
        return [
            self.x_h, self.x_l,
            self.y_h, self.y_l,
            self.z_h, self.z_l,
        ];
    }
}
