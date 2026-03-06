pub type uint8 = u8;
pub type int8 = i8;
pub type uint16 = u16;
pub type int16 = i16;
pub type uint24 = [u8; 3];
pub type uint32 = u32;
pub type int32 = i32;
pub type Fixed = i32;
pub type FWORD = i16;
pub type UFWORD = u16;
pub type Offset8 = u8;
pub type Offset16 = u16;
pub type Offset24 = [u8; 3];
pub type Offset32 = u32;

pub struct F2DOT14(i16);
pub struct LONGDATETIME(i64);
#[derive(Eq, PartialEq)]
pub struct Tag(u32);
#[derive(Eq, PartialEq)]
pub struct Version16Dot16(u32);

impl F2DOT14 {
    pub fn to_f32(&self) -> f32 {
        self.0 as f32 / 16384.0
    }
}

pub fn f32_to_f2dot14(value: f32) -> F2DOT14 {
    let scaled = (value * 16384.0).round() as i16;
    F2DOT14(scaled)
}

impl Tag {
    pub fn from_str(s: &str) -> Self {
        let bytes = s.as_bytes();
        let mut value = 0u32;
        for &b in bytes.iter().take(4) {
            value = (value << 8) | b as u32;
        }
        Tag(value)
    }

    pub fn from_chars(c1: char, c2: char, c3: char, c4: char) -> Self {
        let value = ((c1 as u32) << 24) | ((c2 as u32) << 16) | ((c3 as u32) << 8) | (c4 as u32);
        Tag(value)
    }

    pub fn to_string(&self) -> String {
        let bytes = [
            ((self.0 >> 24) & 0xFF) as u8,
            ((self.0 >> 16) & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            (self.0 & 0xFF) as u8,
        ];
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

impl Version16Dot16 {
    pub fn from_major_minor(major: u16, minor: u16) -> Self {
        Version16Dot16(((major as u32) << 16) | (minor as u32))
    }

    pub fn to_string(&self) -> String {
        let major = self.0 >> 16;
        let minor = self.0 & 0xFFFF;
        format!("{}.{}", major, minor)
    }
}

impl LONGDATETIME {
    pub fn to_datetime(&self) -> std::time::SystemTime {
        let seconds_since_1904 = self.0;
        let unix_epoch = std::time::UNIX_EPOCH;
        let mac_epoch = unix_epoch + std::time::Duration::from_secs(2082844800);
        mac_epoch + std::time::Duration::from_secs(seconds_since_1904 as u64)
    }

    pub fn to_string(&self) -> String {
        let datetime = self.to_datetime();
        let datetime: chrono::DateTime<chrono::Utc> = datetime.into();
        datetime.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

pub struct Reader<'a> {
    full_data: &'a [u8],
    current: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader {
            full_data: data,
            current: data,
        }
    }

    pub fn seek(&mut self, offset: usize) {
        if offset < self.full_data.len() {
            self.current = &self.full_data[offset..];
        }
    }

    pub fn tell(&self) -> usize {
        self.full_data.len() - self.current.len()
    }

    pub fn read_uint8(&mut self) -> uint8 {
        let (buf, rest) = self.current.split_at(1);
        self.current = rest;

        buf[0]
    }

    pub fn read_int8(&mut self) -> int8 {
        let (buf, rest) = self.current.split_at(1);
        self.current = rest;

        buf[0] as i8
    }

    pub fn read_uint16(&mut self) -> uint16 {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;

        u16::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_int16(&mut self) -> int16 {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;

        i16::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_uint24(&mut self) -> uint24 {
        let (buf, rest) = self.current.split_at(3);
        self.current = rest;

        [buf[0], buf[1], buf[2]]
    }

    pub fn read_uint32(&mut self) -> uint32 {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;

        u32::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_int32(&mut self) -> int32 {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;

        i32::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_fixed(&mut self) -> Fixed {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;

        i32::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_fword(&mut self) -> FWORD {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;

        i16::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_ufword(&mut self) -> UFWORD {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;

        u16::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_f2dot14(&mut self) -> F2DOT14 {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;

        F2DOT14(i16::from_be_bytes(buf.try_into().unwrap()))
    }

    pub fn read_longdatetime(&mut self) -> LONGDATETIME {
        let (buf, rest) = self.current.split_at(8);
        self.current = rest;

        LONGDATETIME(i64::from_be_bytes(buf.try_into().unwrap()))
    }

    pub fn read_tag(&mut self) -> Tag {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;

        Tag(u32::from_be_bytes(buf.try_into().unwrap()))
    }

    pub fn read_offset8(&mut self) -> Offset8 {
        let (buf, rest) = self.current.split_at(1);
        self.current = rest;

        buf[0]
    }

    pub fn read_offset16(&mut self) -> Offset16 {
        let (buf, rest) = self.current.split_at(2);
        self.current = rest;

        u16::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_offset24(&mut self) -> Offset24 {
        let (buf, rest) = self.current.split_at(3);
        self.current = rest;

        [buf[0], buf[1], buf[2]]
    }

    pub fn read_offset32(&mut self) -> Offset32 {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;

        u32::from_be_bytes(buf.try_into().unwrap())
    }

    pub fn read_version16dot16(&mut self) -> Version16Dot16 {
        let (buf, rest) = self.current.split_at(4);
        self.current = rest;

        Version16Dot16(u32::from_be_bytes(buf.try_into().unwrap()))
    }

    pub fn read(&mut self, size: usize) -> &'a [u8] {
        let (buf, rest) = self.current.split_at(size);
        self.current = rest;

        buf
    }
}
