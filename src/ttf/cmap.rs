use crate::ttf::types::{int16, uint16, uint24, uint32, uint8, Offset32, Reader};

struct CmapHeader {
    version: uint16,
    num_tables: uint16,
    encoding_records: Vec<CmapEncodingRecord>,
}

struct CmapEncodingRecord {
    platform_id: PlatformId,
    encoding_id: uint16,
    offset: uint16,
}

enum PlatformId {
    Unicode = 0,
    Macintosh = 1,
    Iso = 2,
    Windows = 3,
    Custom = 4,
}

impl TryFrom<uint16> for PlatformId {
    type Error = ();

    fn try_from(value: uint16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PlatformId::Unicode),
            1 => Ok(PlatformId::Macintosh),
            2 => Ok(PlatformId::Iso),
            3 => Ok(PlatformId::Windows),
            4 => Ok(PlatformId::Custom),
            _ => Err(()),
        }
    }
}

enum UnicodeEncodingId {
    Unicode1_0 = 0,
    Unicode1_1 = 1,
    Iso10646 = 2,
    Unicode2_0BMP = 3,
    Unicode2_0Full = 4,
    UnicodeVariationSequences = 5,
    UnicodeFull = 6,
}

enum IsoEncodingId {
    Ascii = 0,
    Iso10646 = 1,
    Iso8859_1 = 2,
}

enum WindowsEncodingId {
    Symbol = 0,
    UnicodeBMP = 1,
    ShiftJIS = 2,
    Big5 = 3,
    Wansung = 4,
    Johab = 5,
    UnicodeFull = 10,
}

impl CmapHeader {
    pub fn read_from(reader: &mut Reader) -> Self {
        let version = reader.read_uint16();
        let num_tables = reader.read_uint16();

        let encoding_records = (0..num_tables)
            .map(|_| CmapEncodingRecord {
                platform_id: PlatformId::try_from(reader.read_uint16()).unwrap_or(PlatformId::Custom),
                encoding_id: reader.read_uint16(),
                offset: reader.read_uint16(),
            })
            .collect();

        CmapHeader {
            version,
            num_tables,
            encoding_records,
        }
    }
}

struct SubtableFormat0 {
    format: uint16,
    length: uint16,
    language: uint16,
    glyph_id_array: [uint8; 256],
}

impl SubtableFormat0 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint16();
        let language = reader.read_uint16();

        let mut glyph_id_array = [0; 256];
        for i in 0..256 {
            glyph_id_array[i] = reader.read_uint8();
        }

        SubtableFormat0 {
            format,
            length,
            language,
            glyph_id_array,
        }
    }
}

struct SubtableFormat2 {
    format: uint16,
    length: uint16,
    language: uint16,
    sub_header_keys: [uint16; 256],
    sub_headers: Vec<SubHeader>,
    glyph_id_array: Vec<uint16>,
}

struct SubHeader {
    first_code: uint16,
    entry_count: uint16,
    id_delta: int16,
    id_range_offset: uint16,
}

impl SubtableFormat2 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint16();
        let language = reader.read_uint16();

        let mut sub_header_keys = [0; 256];
        for i in 0..256 {
            sub_header_keys[i] = reader.read_uint16();
        }

        // TODO read sub_headers and glyph_id_array

        SubtableFormat2 {
            format,
            length,
            language,
            sub_header_keys,
            sub_headers: vec![],
            glyph_id_array: vec![],
        }
    }
}

struct SubtableFormat4 {
    format: uint16,
    length: uint16,
    seg_count_x2: uint16,
    search_range: uint16,
    entry_selector: uint16,
    range_shift: uint16,
    end_code: Vec<uint16>,
    reserved_pad: uint16,
    start_code: Vec<uint16>,
    id_delta: Vec<int16>,
    id_range_offset: Vec<uint16>,
    glyph_id_array: Vec<uint16>,
}

impl SubtableFormat4 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint16();
        let seg_count_x2 = reader.read_uint16();
        let search_range = reader.read_uint16();
        let entry_selector = reader.read_uint16();
        let range_shift = reader.read_uint16();

        let seg_count = seg_count_x2 / 2;

        let end_code = (0..seg_count).map(|_| reader.read_uint16()).collect();
        let reserved_pad = reader.read_uint16();
        let start_code = (0..seg_count).map(|_| reader.read_uint16()).collect();
        let id_delta = (0..seg_count).map(|_| reader.read_int16()).collect();
        let id_range_offset = (0..seg_count).map(|_| reader.read_uint16()).collect();

        let glyph_id_array_length = (length as usize - 14 - (seg_count as usize * 8)) / 2;

        SubtableFormat4 {
            format,
            length,
            seg_count_x2,
            search_range,
            entry_selector,
            range_shift,
            end_code,
            reserved_pad,
            start_code,
            id_delta,
            id_range_offset,
            glyph_id_array: (0..glyph_id_array_length).map(|_| reader.read_uint16()).collect(),
        }
    }
}

struct SubtableFormat6 {
    format: uint16,
    length: uint16,
    language: uint16,
    first_code: uint16,
    entry_count: uint16,
    glyph_id_array: Vec<uint16>,
}

impl SubtableFormat6 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint16();
        let language = reader.read_uint16();
        let first_code = reader.read_uint16();
        let entry_count = reader.read_uint16();

        let glyph_id_array = (0..entry_count).map(|_| reader.read_uint16()).collect();

        SubtableFormat6 {
            format,
            length,
            language,
            first_code,
            entry_count,
            glyph_id_array,
        }
    }
}

struct SubtableFormat8 {
    format: uint16,
    length: uint32,
    language: uint32,
    is32: Vec<uint32>,
    num_groups: uint32,
    groups: Vec<SequentialMapGroup>,
}

struct SequentialMapGroup {
    start_char_code: uint32,
    end_char_code: uint32,
    start_glyph_id: uint32,
}

impl SubtableFormat8 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint32();
        let language = reader.read_uint32();

        let num_groups = reader.read_uint32();
        let groups = (0..num_groups)
            .map(|_| SequentialMapGroup {
                start_char_code: reader.read_uint32(),
                end_char_code: reader.read_uint32(),
                start_glyph_id: reader.read_uint32(),
            })
            .collect();

        SubtableFormat8 {
            format,
            length,
            language,
            is32: vec![],
            num_groups,
            groups,
        }
    }
}

struct SubtableFormat10 {
    format: uint16,
    length: uint32,
    language: uint32,
    start_char_code: uint32,
    num_char_codes: uint32,
    glyph_id_array: Vec<uint16>,
}

impl SubtableFormat10 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint32();
        let language = reader.read_uint32();
        let start_char_code = reader.read_uint32();
        let num_char_codes = reader.read_uint32();

        let glyph_id_array = (0..num_char_codes).map(|_| reader.read_uint16()).collect();

        SubtableFormat10 {
            format,
            length,
            language,
            start_char_code,
            num_char_codes,
            glyph_id_array,
        }
    }
}

struct SubtableFormat12 {
    format: uint16,
    length: uint32,
    language: uint32,
    num_groups: uint32,
    groups: Vec<SequentialMapGroup>,
}

impl SubtableFormat12 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint32();
        let language = reader.read_uint32();

        let num_groups = reader.read_uint32();
        let groups = (0..num_groups)
            .map(|_| SequentialMapGroup {
                start_char_code: reader.read_uint32(),
                end_char_code: reader.read_uint32(),
                start_glyph_id: reader.read_uint32(),
            })
            .collect();

        SubtableFormat12 {
            format,
            length,
            language,
            num_groups,
            groups,
        }
    }
}

struct SubtableFormat13 {
    format: uint16,
    length: uint32,
    language: uint32,
    num_groups: uint32,
    groups: Vec<ConstantMapGroup>,
}

struct ConstantMapGroup {
    start_char_code: uint32,
    end_char_code: uint32,
    glyph_id: uint32,
}

impl SubtableFormat13 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint32();
        let language = reader.read_uint32();

        let num_groups = reader.read_uint32();
        let groups = (0..num_groups)
            .map(|_| ConstantMapGroup {
                start_char_code: reader.read_uint32(),
                end_char_code: reader.read_uint32(),
                glyph_id: reader.read_uint32(),
            })
            .collect();

        SubtableFormat13 {
            format,
            length,
            language,
            num_groups,
            groups,
        }
    }
}

struct SubtableFormat14 {
    format: uint16,
    length: uint32,
    number_of_var_selector_records: uint32,
    var_selector_records: Vec<VarSelectorRecord>,
}

impl SubtableFormat14 {
    fn read_from(reader: &mut Reader, format: uint16) -> Self {
        let length = reader.read_uint32();
        let number_of_var_selector_records = reader.read_uint32();

        let var_selector_records = (0..number_of_var_selector_records)
            .map(|_| VarSelectorRecord {
                var_selector: reader.read_uint24(),
                default_uvs_offset: reader.read_offset32(),
                non_default_uvs_offset: reader.read_offset32(),
            })
            .collect();

        SubtableFormat14 {
            format,
            length,
            number_of_var_selector_records,
            var_selector_records,
        }
    }
}

struct VarSelectorRecord {
    var_selector: uint24,
    default_uvs_offset: Offset32,
    non_default_uvs_offset: Offset32,
}

pub enum CmapSubtable {
    Format0(SubtableFormat0),
    Format2(SubtableFormat2),
    Format4(SubtableFormat4),
    Format6(SubtableFormat6),
    Format8(SubtableFormat8),
    Format10(SubtableFormat10),
    Format12(SubtableFormat12),
    Format13(SubtableFormat13),
    Format14(SubtableFormat14),
}

impl CmapSubtable {
    pub fn read_from(reader: &mut Reader) -> Self {
        let format = reader.read_uint16();

        match format {
            0 => CmapSubtable::Format0(SubtableFormat0::read_from(reader, format)),
            2 => CmapSubtable::Format2(SubtableFormat2::read_from(reader, format)),
            4 => CmapSubtable::Format4(SubtableFormat4::read_from(reader, format)),
            6 => CmapSubtable::Format6(SubtableFormat6::read_from(reader, format)),
            8 => CmapSubtable::Format8(SubtableFormat8::read_from(reader, format)),
            10 => CmapSubtable::Format10(SubtableFormat10::read_from(reader, format)),
            12 => CmapSubtable::Format12(SubtableFormat12::read_from(reader, format)),
            13 => CmapSubtable::Format13(SubtableFormat13::read_from(reader, format)),
            14 => CmapSubtable::Format14(SubtableFormat14::read_from(reader, format)),
            _ => panic!("Unsupported cmap subtable format: {}", format),
        }
    }
}