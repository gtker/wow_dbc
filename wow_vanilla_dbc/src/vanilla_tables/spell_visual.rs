use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::vanilla_tables::sound_entries::*;
use crate::vanilla_tables::spell_visual_kit::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellVisual {
    pub rows: Vec<SpellVisualRow>,
}

impl DbcTable for SpellVisual {
    type Row = SpellVisualRow;

    fn filename() -> &'static str { "SpellVisual.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 64 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 64,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 16 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 16,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (SpellVisual) uint32
            let id = SpellVisualKey::new(crate::util::read_u32_le(chunk)?);

            // precast_kit: foreign_key (SpellVisualKit) uint32
            let precast_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // cast_kit: foreign_key (SpellVisualKit) uint32
            let cast_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // impact_kit: foreign_key (SpellVisualKit) uint32
            let impact_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // state_kit: foreign_key (SpellVisualKit) uint32
            let state_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // state_done_kit: foreign_key (SpellVisualKit) uint32
            let state_done_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // channel_kit: foreign_key (SpellVisualKit) uint32
            let channel_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // has_missile: uint32
            let has_missile = crate::util::read_u32_le(chunk)?;

            // missile_model: int32
            let missile_model = crate::util::read_i32_le(chunk)?;

            // missile_path_type: int32
            let missile_path_type = crate::util::read_i32_le(chunk)?;

            // missile_destination_attachment: int32
            let missile_destination_attachment = crate::util::read_i32_le(chunk)?;

            // missile_sound: foreign_key (SoundEntries) uint32
            let missile_sound = SoundEntriesKey::new(crate::util::read_u32_le(chunk)?.into());

            // anim_event_sound: foreign_key (SpellVisualKit) uint32
            let anim_event_sound = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // flags: int32
            let flags = crate::util::read_i32_le(chunk)?;

            // caster_impact_kit: foreign_key (SpellVisualKit) uint32
            let caster_impact_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());

            // target_impact_kit: foreign_key (SpellVisualKit) uint32
            let target_impact_kit = SpellVisualKitKey::new(crate::util::read_u32_le(chunk)?.into());


            rows.push(SpellVisualRow {
                id,
                precast_kit,
                cast_kit,
                impact_kit,
                state_kit,
                state_done_kit,
                channel_kit,
                has_missile,
                missile_model,
                missile_path_type,
                missile_destination_attachment,
                missile_sound,
                anim_event_sound,
                flags,
                caster_impact_kit,
                target_impact_kit,
            });
        }

        Ok(SpellVisual { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 16,
            record_size: 64,
            string_block_size: 1,
        };

        b.write_all(&header.write_header())?;

        for row in &self.rows {
            // id: primary_key (SpellVisual) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // precast_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.precast_kit.id as u32).to_le_bytes())?;

            // cast_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.cast_kit.id as u32).to_le_bytes())?;

            // impact_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.impact_kit.id as u32).to_le_bytes())?;

            // state_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.state_kit.id as u32).to_le_bytes())?;

            // state_done_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.state_done_kit.id as u32).to_le_bytes())?;

            // channel_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.channel_kit.id as u32).to_le_bytes())?;

            // has_missile: uint32
            b.write_all(&row.has_missile.to_le_bytes())?;

            // missile_model: int32
            b.write_all(&row.missile_model.to_le_bytes())?;

            // missile_path_type: int32
            b.write_all(&row.missile_path_type.to_le_bytes())?;

            // missile_destination_attachment: int32
            b.write_all(&row.missile_destination_attachment.to_le_bytes())?;

            // missile_sound: foreign_key (SoundEntries) uint32
            b.write_all(&(row.missile_sound.id as u32).to_le_bytes())?;

            // anim_event_sound: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.anim_event_sound.id as u32).to_le_bytes())?;

            // flags: int32
            b.write_all(&row.flags.to_le_bytes())?;

            // caster_impact_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.caster_impact_kit.id as u32).to_le_bytes())?;

            // target_impact_kit: foreign_key (SpellVisualKit) uint32
            b.write_all(&(row.target_impact_kit.id as u32).to_le_bytes())?;

        }

        b.write_all(&[0_u8])?;

        Ok(())
    }

}

impl Indexable for SpellVisual {
    type PrimaryKey = SpellVisualKey;
    fn get(&self, key: &Self::PrimaryKey) -> Option<&Self::Row> {
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: &Self::PrimaryKey) -> Option<&mut Self::Row> {
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct SpellVisualKey {
    pub id: u32
}

impl SpellVisualKey {
    pub const fn new(id: u32) -> Self {
        Self { id }
    }

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpellVisualRow {
    pub id: SpellVisualKey,
    pub precast_kit: SpellVisualKitKey,
    pub cast_kit: SpellVisualKitKey,
    pub impact_kit: SpellVisualKitKey,
    pub state_kit: SpellVisualKitKey,
    pub state_done_kit: SpellVisualKitKey,
    pub channel_kit: SpellVisualKitKey,
    pub has_missile: u32,
    pub missile_model: i32,
    pub missile_path_type: i32,
    pub missile_destination_attachment: i32,
    pub missile_sound: SoundEntriesKey,
    pub anim_event_sound: SpellVisualKitKey,
    pub flags: i32,
    pub caster_impact_kit: SpellVisualKitKey,
    pub target_impact_kit: SpellVisualKitKey,
}

