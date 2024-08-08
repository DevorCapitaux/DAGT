use core::panic;
use reader::Reader;
use std::{collections::HashMap, fs::File};

pub mod reader;

#[derive(Clone)]
pub struct Font {
    glyphs: HashMap<u32, GlyphData>,
    missing_glyph: GlyphData,
    units_per_em: u32,
}

impl Font {
    pub fn get_glyph(&self, c: char) -> &GlyphData {
        self.glyphs
            .get_key_value(&(c as u32))
            .map_or(&self.missing_glyph, |p| &p.1)
    }

    pub fn units_per_em(&self) -> u32 {
        self.units_per_em
    }

    pub fn load(font_path: &str) -> Font {
        let mut file = File::open(font_path).unwrap();
        let mut reader = Reader::new(&mut file);

        let locations = Self::read_table_locations(&mut reader);
        let head_table = *locations.get("head").unwrap();
        let maxp_table = *locations.get("maxp").unwrap();
        let loca_table = *locations.get("loca").unwrap();
        let cmap_table = *locations.get("cmap").unwrap();
        let glyf_table = *locations.get("glyf").unwrap();
        let hhea_table = *locations.get("hhea").unwrap();
        let hmtx_table = *locations.get("hmtx").unwrap();

        reader.goto(head_table as usize);
        reader.skip(18);
        let units_per_em = reader.read_u16();
        reader.skip(30);
        let loc_lookup_bytes_num = if reader.read_i16() == 0 { 2 } else { 4 };

        reader.goto(maxp_table as usize);
        reader.skip(4);
        let glyphs_num = reader.read_u16() as i32;
        let glyph_locations = Self::get_glyph_locations(
            &mut reader,
            glyphs_num,
            loc_lookup_bytes_num,
            loca_table,
            glyf_table,
        );

        let maps = Self::get_maps(&mut reader, cmap_table);
        let mut glyphs = Self::read_glyphs(&mut reader, &glyph_locations, &maps);

        let mut layout_data = vec![(0, 0); glyphs_num as usize];

        reader.goto(hhea_table as usize);
        reader.skip(8);
        let _line_gap = reader.read_i16();
        let _advance_width_max = reader.read_i16();
        reader.skip(22);
        let advance_metrics_num = reader.read_i16();

        reader.goto(hmtx_table as usize);
        let mut last_advance_width = 0;
        for _ in 0..advance_metrics_num {
            let advance_width = reader.read_u16();
            let left_side_bearing = reader.read_i16();
            last_advance_width = advance_width;
            layout_data.push((advance_width, left_side_bearing));
        }

        let rem_num = glyphs_num - advance_metrics_num as i32;

        for i in 0..rem_num {
            let left_side_bearing = reader.read_i16();
            let glyph_index = advance_metrics_num as i32 + i;
            layout_data[glyph_index as usize] = (last_advance_width, left_side_bearing);
        }

        for c in glyphs.iter_mut() {
            c.advance_width = layout_data[c.index as usize].0 as i32;
            c.left_side_bearing = layout_data[c.index as usize].1 as i32;
        }

        let mut missing_glyph = None;

        let glyphs = glyphs
            .iter()
            .map(|glyph| {
                (glyph.unicode, {
                    if glyph.index == 0 {
                        missing_glyph = Some(glyph.clone());
                    }
                    glyph.clone()
                })
            })
            .collect();

        Font {
            glyphs,
            missing_glyph: missing_glyph.unwrap(),
            units_per_em: units_per_em as u32,
        }
    }

    fn read_table_locations(reader: &mut Reader) -> HashMap<String, u32> {
        let mut locations = HashMap::new();

        reader.skip(4);
        let table_num = reader.read_u16();

        reader.skip(6);
        for _ in 0..table_num {
            let tag = reader.read_string(4);
            let _checksum = reader.read_u32();
            let offset = reader.read_u32();
            let _length = reader.read_u32();

            locations.insert(tag, offset);
        }

        locations
    }

    fn get_glyph_locations(
        reader: &mut Reader,
        glyphs_num: i32,
        loc_lookup_bytes_num: i32,
        loca_table: u32,
        glyf_table: u32,
    ) -> Vec<u32> {
        let mut glyph_locs = vec![0; glyphs_num as usize];

        for i in 0..glyphs_num {
            reader.goto(loca_table as usize + (i * loc_lookup_bytes_num) as usize);
            let data_offset = if loc_lookup_bytes_num == 2 {
                reader.read_u16() as u32 * 2
            } else {
                reader.read_u32()
            };
            glyph_locs[i as usize] = glyf_table + data_offset;
        }

        glyph_locs
    }

    fn get_maps(reader: &mut Reader, cmap_table: u32) -> Vec<GlyphMap> {
        let mut maps = Vec::new();

        reader.goto(cmap_table as usize);

        let _version = reader.read_u16(); // 0
        let subtable_num = reader.read_u16() as u32; // 5

        let mut subtable_offset = 0;
        let mut unicode_version: i32 = -1;

        for _ in 0..subtable_num {
            let platform_id = reader.read_u16();
            let platform_specific_id = reader.read_u16();
            let offset = reader.read_u32();

            // pid = 0; psid = 3; offset = 44
            // pid = 0; psid = 4; offset = 8112
            // pid = 1; psid = 0; offset = 10852
            // pid = 3; psid = 1; offset = 44
            // pid = 3; psid = 10; offset = 8112

            // Unicode encoding
            if platform_id == 0 && platform_specific_id as i32 > unicode_version {
                match platform_specific_id {
                    0 | 1 | 3 | 4 => {
                        subtable_offset = offset;
                        unicode_version = platform_specific_id as i32;
                    }
                    _ => (),
                }
            }
            // Microsoft encoding
            else if platform_id == 3 && unicode_version == -1 {
                match platform_specific_id {
                    1 | 10 => subtable_offset = offset,
                    _ => (),
                }
            }
        }

        if subtable_offset == 0 {
            panic!("Font doesn't contain supported map type");
        }

        reader.goto((cmap_table + subtable_offset) as usize);
        let format = reader.read_u16();

        if format != 4 && format != 12 {
            panic!("Font cmpa format not supported");
        }

        if format == 4 {
            let _length = reader.read_u16();
            let _language = reader.read_u16();
            let seg_count_x2 = reader.read_u16();
            let seg_count = seg_count_x2 / 2;
            // skip searchRange, entrySelector, rangeShift
            reader.skip(6);
            let mut end_code = Vec::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                end_code.push(reader.read_u16());
            }
            // skip reservedPad
            reader.skip(2);
            let mut start_code = Vec::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                start_code.push(reader.read_u16());
            }
            let mut id_delta = Vec::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                id_delta.push(reader.read_u16());
            }
            let mut id_range_offset = Vec::with_capacity(seg_count as usize);
            for _ in 0..seg_count {
                id_range_offset.push(reader.read_u16());
            }

            for i in 0..start_code.len() {
                let _start_code = start_code[i];
                let _end_code = end_code[i];
                let _id_range_offset = id_range_offset[i];

                todo!();
            }
        } else if format == 12 {
            reader.skip(10);
            let n_groups = reader.read_u32();

            for _ in 0..n_groups {
                let start_char_code = reader.read_u32();
                let end_char_code = reader.read_u32();
                let start_glyph_code = reader.read_u32();

                let n_chars = 1 + end_char_code - start_char_code;
                for offset in 0..n_chars {
                    let unicode = start_char_code + offset;
                    let index = start_glyph_code + offset;

                    maps.push(GlyphMap { index, unicode })
                }
            }
        }

        maps.push(GlyphMap {
            index: 0,
            unicode: 65535,
        });

        maps
    }

    fn read_glyphs(
        reader: &mut Reader,
        glyph_locations: &[u32],
        maps: &[GlyphMap],
    ) -> Vec<GlyphData> {
        let mut glyphs = Vec::with_capacity(maps.len());

        for i in 0..maps.len() {
            let map = maps[i];

            let mut glyph_data = Self::read_glyph(reader, glyph_locations, map.index);
            glyph_data.unicode = map.unicode;
            glyphs.push(glyph_data);
        }

        glyphs
    }

    fn read_glyph(reader: &mut Reader, glyph_locations: &[u32], glyph_index: u32) -> GlyphData {
        let glyph_location = glyph_locations[glyph_index as usize];

        reader.goto(glyph_location as usize);
        let contour_count = reader.read_i16() as i32;

        if contour_count >= 0 {
            Self::read_simple_glyph(reader, glyph_locations, glyph_index)
        } else {
            Self::read_simple_glyph(reader, glyph_locations, 0)
            // Self::read_compound_glyph(reader, glyph_locations, glyph_index)
        }
    }

    fn read_simple_glyph(
        reader: &mut Reader,
        glyph_locations: &[u32],
        glyph_index: u32,
    ) -> GlyphData {
        reader.goto(glyph_locations[glyph_index as usize] as usize);

        let mut glyph_data: GlyphData = Default::default();
        glyph_data.index = glyph_index;

        let contour_count = reader.read_i16() as i32;
        if contour_count < 0 {
            panic!("Expected simple glyph, but found compound glyph instead")
        }

        glyph_data.min_x = reader.read_i16() as i32;
        glyph_data.min_y = reader.read_i16() as i32;
        glyph_data.max_x = reader.read_i16() as i32;
        glyph_data.max_y = reader.read_i16() as i32;
        glyph_data.width = glyph_data.max_x - glyph_data.min_x;
        glyph_data.height = glyph_data.max_y - glyph_data.min_y;

        let mut points_num = 0;
        let mut contour_end_indices: Vec<i32> = Vec::with_capacity(contour_count as usize);
        for _ in 0..contour_count {
            let contour_end_index = reader.read_u16() as i32;
            points_num = std::cmp::max(points_num, contour_end_index + 1);
            contour_end_indices.push(contour_end_index);
        }

        let instructions_len = reader.read_i16() as i32;
        reader.skip(instructions_len as usize);

        let mut flags = Vec::with_capacity(points_num as usize);

        let mut i = 0usize;
        while i < points_num as usize {
            let flag = Flag::new(reader.read_u8());
            flags.push(flag);

            if flag.bit_is_set(FlagBit::Repeat) {
                let repeat_count = reader.read_u8();

                for _ in 0..repeat_count {
                    i += 1;
                    flags.push(flag);
                }
            }
            i += 1;
        }

        let mut x = Vec::with_capacity(points_num as usize);
        let mut y = Vec::with_capacity(points_num as usize);
        let mut on_curve = Vec::with_capacity(points_num as usize);

        let mut coord_val = 0;

        for i in 0..points_num {
            let flag = flags[i as usize];

            if flag.bit_is_set(FlagBit::XShort) {
                let offset = reader.read_u8();
                if flag.bit_is_set(FlagBit::XInstruction) {
                    coord_val += offset as i32;
                } else {
                    coord_val -= offset as i32;
                }
            } else if !flag.bit_is_set(FlagBit::XInstruction) {
                coord_val += reader.read_i16() as i32;
            }

            x.push(coord_val);
        }

        let mut coord_val = 0;

        for i in 0..points_num {
            let flag = flags[i as usize];

            if flag.bit_is_set(FlagBit::YShort) {
                let offset = reader.read_u8();
                if flag.bit_is_set(FlagBit::YInstruction) {
                    coord_val += offset as i32;
                } else {
                    coord_val -= offset as i32;
                }
            } else if !flag.bit_is_set(FlagBit::YInstruction) {
                coord_val += reader.read_i16() as i32;
            }

            y.push(coord_val);
            on_curve.push(flag.bit_is_set(FlagBit::OnCurve));
        }

        let points = x
            .iter()
            .zip(y.iter())
            .zip(on_curve.iter())
            .map(|((x, y), on_curve)| Point {
                x: *x,
                y: *y,
                on_curve: *on_curve,
            })
            .collect();

        glyph_data.points = points;
        glyph_data.contour_indices = contour_end_indices;

        glyph_data
    }

    fn _read_compound_glyph(
        _reader: &mut Reader,
        _glyph_locations: &[u32],
        _glyph_index: u32,
    ) -> GlyphData {
        todo!();
    }
}

#[derive(Clone, Copy)]
struct GlyphMap {
    index: u32,
    unicode: u32,
}

#[derive(Default, Debug, Clone)]
pub struct GlyphData {
    pub unicode: u32,
    pub index: u32,
    pub points: Vec<Point>,
    pub contour_indices: Vec<i32>,
    pub advance_width: i32,
    pub left_side_bearing: i32,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub on_curve: bool,
}

#[repr(i32)]
enum FlagBit {
    OnCurve = 0,
    XShort = 1,
    YShort = 2,
    Repeat = 3,
    XInstruction = 4,
    YInstruction = 5,
}

#[derive(Clone, Copy)]
struct Flag {
    flag: u8,
}

impl Flag {
    fn new(flag: u8) -> Flag {
        Flag { flag }
    }

    fn bit_is_set(&self, bit: FlagBit) -> bool {
        ((self.flag >> bit as i32) & 1) == 1
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn load() {
        let _font = Font::load("/usr/share/fonts/TTF/JetBrainsMonoNerdFontMono-Bold.ttf");
    }
}
