use crate::renderer::pbm::{Image, ImageWithSpace};
use crate::renderer::render::render_glyph;
use crate::ttf::cmap::{CmapHeader, CmapSubtable, PlatformId};
use crate::ttf::glyph::Glyph;
use crate::ttf::table::{HeadTable, HheaTable, HmtxTable, MaxpTable, MaxpTable1_0};
use crate::ttf::table_directory::TTFTableDirectory;
use crate::ttf::types::{Reader, Tag, uint32};

pub struct Font<'a> {
    reader: Reader<'a>,
    directory: TTFTableDirectory,
    head: HeadTable,
    maxp: MaxpTable1_0,
    cmap: CmapSubtable,
    loca: Vec<u32>,
    glyf: Vec<Glyph>,
    hhea: HheaTable,
    hmtx: HmtxTable,
}

impl Font<'_> {
    pub fn from_data(data: &'_ [u8]) -> Font<'_> {
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

        let maxp_1 = match maxp {
            MaxpTable::Version0_5(_) => {
                panic!("TrueType fonts with 'maxp' version 0.5 are not supported")
            }
            MaxpTable::Version1_0(table) => table,
        };

        let cmap_record = table_directory
            .get_table_record(&Tag::from_str("cmap"))
            .expect("Failed to find 'cmap' table");

        r.seek(cmap_record.offset as usize);

        let header = CmapHeader::read_from(&mut r);

        // example: platform_id = 3 (Windows), encoding_id = 1 (Unicode BMP)
        let subtable = header
            .encoding_records
            .iter()
            .find(|record| record.platform_id == PlatformId::Windows && record.encoding_id == 1);

        header
            .encoding_records
            .iter()
            .for_each(|record| println!("Found 'cmap' encoding record: {:?}", record));

        let cmap_subtable = if let Some(record) = subtable {
            r.seek(cmap_record.offset as usize + record.offset as usize);
            CmapSubtable::read_from(&mut r)
        } else {
            panic!("Failed to find a suitable 'cmap' subtable (platform_id=3, encoding_id=1)");
        };

        let hhea_record = table_directory
            .get_table_record(&Tag::from_str("hhea"))
            .expect("Failed to find 'hhea' table");

        r.seek(hhea_record.offset as usize);
        let hhea = HheaTable::read_from(&mut r);

        let hmtx_record = table_directory
            .get_table_record(&Tag::from_str("hmtx"))
            .expect("Failed to find 'hmtx' table");

        r.seek(hmtx_record.offset as usize);
        let hmtx = HmtxTable::read_from(&mut r, maxp_1.num_glyphs, hhea.number_of_h_metrics);

        Font {
            reader: r,
            directory: table_directory,
            head,
            maxp: maxp_1,
            cmap: cmap_subtable,
            loca: vec![],
            glyf: vec![],
            hhea,
            hmtx,
        }
    }

    fn read_loca(&mut self) {
        let loca_record = self
            .directory
            .get_table_record(&Tag::from_str("loca"))
            .expect("Failed to find 'loca' table");

        self.reader.seek(loca_record.offset as usize);

        let num_glyphs = self.maxp.num_glyphs as usize;
        self.loca = if self.head.index_to_loc_format == 0 {
            (0..=num_glyphs)
                .map(|_| self.reader.read_uint16() as u32 * 2)
                .collect()
        } else {
            (0..=num_glyphs)
                .map(|_| self.reader.read_uint32())
                .collect()
        };
    }

    pub fn read_glyf(&mut self) {
        if self.loca.is_empty() {
            self.read_loca();
        }

        let glyf_record = self
            .directory
            .get_table_record(&Tag::from_str("glyf"))
            .expect("Failed to find 'glyf' table");

        self.reader.seek(glyf_record.offset as usize);

        for offset in &self.loca {
            self.reader
                .seek(glyf_record.offset as usize + *offset as usize);
            let glyph = Glyph::read_from(&mut self.reader);
            self.glyf.push(glyph);
        }
    }

    pub fn render_glyph(&self, code: uint32) {
        let index = self
            .cmap
            .get_glyph_id(code)
            .expect("Failed to find glyph ID for the given code point");

        let glyph = &self.glyf[index as usize];

        let img = render_glyph(glyph);
        img.write_to_ppm("glyph.ppm").expect("Failed to write image");
    }

    pub fn render_text(&self, text: &str) {
        let glyphs_info: Vec<ImageWithSpace> = text
            .chars()
            .map(|c| {
                let code = c as u32;

                let index = self
                    .cmap
                    .get_glyph_id(code)
                    .expect("Failed to find glyph ID for the given code point");
                let glyph = &self.glyf[index as usize];

                let img = render_glyph(glyph);

                let advance_width = self.hmtx.get_advance_width(index as usize) as u32;
                let space = advance_width.saturating_sub(img.width);

                ImageWithSpace {
                    image: img,
                    space,
                }
            })
            .collect();

        let final_image = Image::concat_all(&glyphs_info);
        final_image.write_to_ppm("text.ppm").expect("Failed to write image");
    }
}
