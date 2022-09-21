use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;

#[derive(Debug, Clone, PartialEq)]
pub struct DestructibleModelData {
    pub rows: Vec<DestructibleModelDataRow>,
}

impl DbcTable for DestructibleModelData {
    type Row = DestructibleModelDataRow;

    fn filename() -> &'static str { "DestructibleModelData.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 76 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 76,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 19 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 19,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (DestructibleModelData) int32
            let id = DestructibleModelDataKey::new(crate::util::read_i32_le(chunk)?);

            // state0_impact_effect_doodad_set: int32
            let state0_impact_effect_doodad_set = crate::util::read_i32_le(chunk)?;

            // state0_ambient_doodad_set: int32
            let state0_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // state1_w_m_o: int32
            let state1_w_m_o = crate::util::read_i32_le(chunk)?;

            // state1_destruction_doodad_set: int32
            let state1_destruction_doodad_set = crate::util::read_i32_le(chunk)?;

            // state1_impact_effect_doodad_set: int32
            let state1_impact_effect_doodad_set = crate::util::read_i32_le(chunk)?;

            // state1_ambient_doodad_set: int32
            let state1_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // state2_w_m_o: int32
            let state2_w_m_o = crate::util::read_i32_le(chunk)?;

            // state2_destruction_doodad_set: int32
            let state2_destruction_doodad_set = crate::util::read_i32_le(chunk)?;

            // state2_impact_effect_doodad_set: int32
            let state2_impact_effect_doodad_set = crate::util::read_i32_le(chunk)?;

            // state2_ambient_doodad_set: int32
            let state2_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // state3_w_m_o: int32
            let state3_w_m_o = crate::util::read_i32_le(chunk)?;

            // state3_init_doodad_set: int32
            let state3_init_doodad_set = crate::util::read_i32_le(chunk)?;

            // state3_ambient_doodad_set: int32
            let state3_ambient_doodad_set = crate::util::read_i32_le(chunk)?;

            // eject_direction: int32
            let eject_direction = crate::util::read_i32_le(chunk)?;

            // repair_ground_fx: int32
            let repair_ground_fx = crate::util::read_i32_le(chunk)?;

            // do_not_highlight: int32
            let do_not_highlight = crate::util::read_i32_le(chunk)?;

            // heal_effect: int32
            let heal_effect = crate::util::read_i32_le(chunk)?;

            // heal_effect_speed: int32
            let heal_effect_speed = crate::util::read_i32_le(chunk)?;


            rows.push(DestructibleModelDataRow {
                id,
                state0_impact_effect_doodad_set,
                state0_ambient_doodad_set,
                state1_w_m_o,
                state1_destruction_doodad_set,
                state1_impact_effect_doodad_set,
                state1_ambient_doodad_set,
                state2_w_m_o,
                state2_destruction_doodad_set,
                state2_impact_effect_doodad_set,
                state2_ambient_doodad_set,
                state3_w_m_o,
                state3_init_doodad_set,
                state3_ambient_doodad_set,
                eject_direction,
                repair_ground_fx,
                do_not_highlight,
                heal_effect,
                heal_effect_speed,
            });
        }

        Ok(DestructibleModelData { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 19,
            record_size: 76,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (DestructibleModelData) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // state0_impact_effect_doodad_set: int32
            b.write_all(&row.state0_impact_effect_doodad_set.to_le_bytes())?;

            // state0_ambient_doodad_set: int32
            b.write_all(&row.state0_ambient_doodad_set.to_le_bytes())?;

            // state1_w_m_o: int32
            b.write_all(&row.state1_w_m_o.to_le_bytes())?;

            // state1_destruction_doodad_set: int32
            b.write_all(&row.state1_destruction_doodad_set.to_le_bytes())?;

            // state1_impact_effect_doodad_set: int32
            b.write_all(&row.state1_impact_effect_doodad_set.to_le_bytes())?;

            // state1_ambient_doodad_set: int32
            b.write_all(&row.state1_ambient_doodad_set.to_le_bytes())?;

            // state2_w_m_o: int32
            b.write_all(&row.state2_w_m_o.to_le_bytes())?;

            // state2_destruction_doodad_set: int32
            b.write_all(&row.state2_destruction_doodad_set.to_le_bytes())?;

            // state2_impact_effect_doodad_set: int32
            b.write_all(&row.state2_impact_effect_doodad_set.to_le_bytes())?;

            // state2_ambient_doodad_set: int32
            b.write_all(&row.state2_ambient_doodad_set.to_le_bytes())?;

            // state3_w_m_o: int32
            b.write_all(&row.state3_w_m_o.to_le_bytes())?;

            // state3_init_doodad_set: int32
            b.write_all(&row.state3_init_doodad_set.to_le_bytes())?;

            // state3_ambient_doodad_set: int32
            b.write_all(&row.state3_ambient_doodad_set.to_le_bytes())?;

            // eject_direction: int32
            b.write_all(&row.eject_direction.to_le_bytes())?;

            // repair_ground_fx: int32
            b.write_all(&row.repair_ground_fx.to_le_bytes())?;

            // do_not_highlight: int32
            b.write_all(&row.do_not_highlight.to_le_bytes())?;

            // heal_effect: int32
            b.write_all(&row.heal_effect.to_le_bytes())?;

            // heal_effect_speed: int32
            b.write_all(&row.heal_effect_speed.to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for DestructibleModelData {
    type PrimaryKey = DestructibleModelDataKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct DestructibleModelDataKey {
    pub id: i32
}

impl DestructibleModelDataKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct DestructibleModelDataRow {
    pub id: DestructibleModelDataKey,
    pub state0_impact_effect_doodad_set: i32,
    pub state0_ambient_doodad_set: i32,
    pub state1_w_m_o: i32,
    pub state1_destruction_doodad_set: i32,
    pub state1_impact_effect_doodad_set: i32,
    pub state1_ambient_doodad_set: i32,
    pub state2_w_m_o: i32,
    pub state2_destruction_doodad_set: i32,
    pub state2_impact_effect_doodad_set: i32,
    pub state2_ambient_doodad_set: i32,
    pub state3_w_m_o: i32,
    pub state3_init_doodad_set: i32,
    pub state3_ambient_doodad_set: i32,
    pub eject_direction: i32,
    pub repair_ground_fx: i32,
    pub do_not_highlight: i32,
    pub heal_effect: i32,
    pub heal_effect_speed: i32,
}

