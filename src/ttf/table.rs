use crate::ttf::types::{Fixed, LONGDATETIME, Reader, Tag, Version16Dot16, int16, uint16, uint32};

pub struct HeadTable {
    pub major_version: uint16,
    pub minor_version: uint16,
    pub font_revision: Fixed,
    pub check_sum_adjustment: uint32,
    pub magic_number: uint32,
    pub flags: uint16,
    pub units_per_em: uint16,
    pub created: LONGDATETIME,
    pub modified: LONGDATETIME,
    pub x_min: int16,
    pub y_min: int16,
    pub x_max: int16,
    pub y_max: int16,
    pub mac_style: uint16,
    pub lowest_rec_ppem: uint16,
    pub font_direction_hint: int16,
    pub index_to_loc_format: int16,
    pub glyph_data_format: int16,
}

impl HeadTable {
    pub const TAG: &str = "head";

    pub fn read_from(reader: &mut Reader) -> Self {
        HeadTable {
            major_version: reader.read_uint16(),
            minor_version: reader.read_uint16(),
            font_revision: reader.read_fixed(),
            check_sum_adjustment: reader.read_uint32(),
            magic_number: reader.read_uint32(),
            flags: reader.read_uint16(),
            units_per_em: reader.read_uint16(),
            created: reader.read_longdatetime(),
            modified: reader.read_longdatetime(),
            x_min: reader.read_int16(),
            y_min: reader.read_int16(),
            x_max: reader.read_int16(),
            y_max: reader.read_int16(),
            mac_style: reader.read_uint16(),
            lowest_rec_ppem: reader.read_uint16(),
            font_direction_hint: reader.read_int16(),
            index_to_loc_format: reader.read_int16(),
            glyph_data_format: reader.read_int16(),
        }
    }
}

pub struct MaxpTable0_5 {
    pub version: Version16Dot16,
    pub num_glyphs: uint16,
}

pub struct MaxpTable1_0 {
    pub version: Version16Dot16,
    pub num_glyphs: uint16,
    pub max_points: uint16,
    pub max_contours: uint16,
    pub max_composite_points: uint16,
    pub max_composite_contours: uint16,
    pub max_zones: uint16,
    pub max_twilight_points: uint16,
    pub max_storage: uint16,
    pub max_function_defs: uint16,
    pub max_instruction_defs: uint16,
    pub max_stack_elements: uint16,
    pub max_size_of_instructions: uint16,
    pub max_component_elements: uint16,
    pub max_component_depth: uint16,
}

pub enum MaxpTable {
    Version0_5(MaxpTable0_5),
    Version1_0(MaxpTable1_0),
}

impl MaxpTable {
    pub const TAG: &str = "maxp";

    pub fn read_from(reader: &mut Reader) -> Self {
        let version = reader.read_version16dot16();
        let num_glyphs = reader.read_uint16();

        if version == Version16Dot16::from_major_minor(0, 5) {
            MaxpTable::Version0_5(MaxpTable0_5 {
                version,
                num_glyphs,
            })
        } else if version == Version16Dot16::from_major_minor(1, 0) {
            MaxpTable::Version1_0(MaxpTable1_0 {
                version,
                num_glyphs,
                max_points: reader.read_uint16(),
                max_contours: reader.read_uint16(),
                max_composite_points: reader.read_uint16(),
                max_composite_contours: reader.read_uint16(),
                max_zones: reader.read_uint16(),
                max_twilight_points: reader.read_uint16(),
                max_storage: reader.read_uint16(),
                max_function_defs: reader.read_uint16(),
                max_instruction_defs: reader.read_uint16(),
                max_stack_elements: reader.read_uint16(),
                max_size_of_instructions: reader.read_uint16(),
                max_component_elements: reader.read_uint16(),
                max_component_depth: reader.read_uint16(),
            })
        } else {
            panic!("Unsupported 'maxp' table version: {}", version.to_string());
        }
    }
}
