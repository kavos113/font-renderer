use crate::ttf::types::{uint16, uint32, Offset32, Reader, Tag};

pub struct TTFTableRecord {
    tag: Tag,
    checksum: uint32,
    offset: Offset32,
    length: uint32,
}

impl TTFTableRecord {
    fn read_from(reader: &mut Reader) -> Self {
        TTFTableRecord {
            tag: reader.read_tag(),
            checksum: reader.read_uint32(),
            offset: reader.read_offset32(),
            length: reader.read_uint32(),
        }
    }
}

pub struct TTFTableDirectory {
    sfnt_version: uint32,
    num_tables: uint16,
    search_range: uint16,
    entry_selector: uint16,
    range_shift: uint16,
    table_records: Vec<TTFTableRecord>,
}

impl TTFTableDirectory {
    pub fn read_from(reader: &mut Reader) -> Self {
        let sfnt_version = reader.read_uint32();
        let num_tables = reader.read_uint16();
        let search_range = reader.read_uint16();
        let entry_selector = reader.read_uint16();
        let range_shift = reader.read_uint16();

        let font_table_records = (0..num_tables)
            .map(|_| TTFTableRecord::read_from(reader))
            .collect();

        TTFTableDirectory {
            sfnt_version,
            num_tables,
            search_range,
            entry_selector,
            range_shift,
            table_records: font_table_records,
        }
    }

    pub fn get_table_record(&self, tag: &Tag) -> Option<&TTFTableRecord> {
        self.table_records.iter().find(|record| record.tag == *tag)
    }
}