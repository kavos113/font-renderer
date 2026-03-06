use crate::ttf::table::{HeadTable, MaxpTable};

pub struct Font {
    head: HeadTable,
    maxp: MaxpTable,
}