use crate::ttf::glyph::Glyph;
use crate::ttf::table::{HeadTable, MaxpTable};
use crate::ttf::table_directory::TTFTableDirectory;
use crate::ttf::types::{Reader, Tag};

pub struct Font<'a> {
    data: &'a [u8],
    reader: Reader<'a>,
    head: HeadTable,
    maxp: MaxpTable,
    loca: Vec<u32>,
    glyf: Vec<Glyph>,
}

impl Font<'_> {
    pub fn from_file(data: &'_ [u8]) -> Font<'_> {
        let mut r = Reader::new(data);

        let table_directory = TTFTableDirectory::read_from(&mut r);

        let head_record = table_directory
            .get_table_record(&Tag::from_str(HeadTable::TAG))
            .expect("Failed to find 'head' table");

        let maxp_record = table_directory
            .get_table_record(&Tag::from_str(MaxpTable::TAG))
            .expect("Failed to find 'maxp' table");

        r.seek(head_record.offset as usize);
        let head = HeadTable::read_from(&mut r);

        r.seek(maxp_record.offset as usize);
        let maxp = MaxpTable::read_from(&mut r);

        Font {
            data,
            reader: r,
            head,
            maxp,
            loca: vec![],
            glyf: vec![],
        }
    }
}
