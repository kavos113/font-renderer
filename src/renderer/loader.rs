use std::fs;
use crate::ttf::table::{HeadTable, MaxpTable};
use crate::ttf::table_directory::TTFTableDirectory;
use crate::ttf::types::{Reader, Tag};

pub struct Font {
    head: HeadTable,
    maxp: MaxpTable,
}

impl Font {
    pub fn from_file(path: &str) -> Self {
        let buffer = fs::read(path).expect("Failed to read font file");
        let mut r = Reader(&buffer);

        let table_directory = TTFTableDirectory::read_from(&mut r);

        let head_record = table_directory
            .get_table_record(&Tag::from_str(HeadTable::TAG))
            .expect("Failed to find 'head' table");

        let maxp_record = table_directory
            .get_table_record(&Tag::from_str(MaxpTable::TAG))
            .expect("Failed to find 'maxp' table");
    }
}