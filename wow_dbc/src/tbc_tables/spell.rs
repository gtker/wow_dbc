use crate::{
    DbcTable, ExtendedLocalizedString, Indexable,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::tbc_tables::faction::FactionKey;
use crate::tbc_tables::spell_cast_times::SpellCastTimesKey;
use crate::tbc_tables::spell_category::SpellCategoryKey;
use crate::tbc_tables::spell_dispel_type::SpellDispelTypeKey;
use crate::tbc_tables::spell_duration::SpellDurationKey;
use crate::tbc_tables::spell_focus_object::SpellFocusObjectKey;
use crate::tbc_tables::spell_icon::SpellIconKey;
use crate::tbc_tables::spell_mechanic::SpellMechanicKey;
use crate::tys::WritableString;
use crate::util::StringCache;
use std::io::Write;
pub use wow_world_base::tbc::AuraMod;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Spell {
    pub rows: Vec<SpellRow>,
}

impl DbcTable for Spell {
    type Row = SpellRow;

    const FILENAME: &'static str = "Spell.dbc";
    const FIELD_COUNT: usize = 216;
    const ROW_SIZE: usize = 864;

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

        if header.record_size != Self::ROW_SIZE as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: Self::ROW_SIZE as u32,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != Self::FIELD_COUNT as u32 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: Self::FIELD_COUNT as u32,
                    actual: header.field_count,
                },
            ));
        }

        let mut r = vec![0_u8; (header.record_count * header.record_size) as usize];
        b.read_exact(&mut r)?;
        let mut string_block = vec![0_u8; header.string_block_size as usize];
        b.read_exact(&mut string_block)?;

        let mut rows = Vec::with_capacity(header.record_count as usize);

        for mut chunk in r.chunks(header.record_size as usize) {
            let chunk = &mut chunk;

            // id: primary_key (Spell) int32
            let id = SpellKey::new(crate::util::read_i32_le(chunk)?);

            // category: foreign_key (SpellCategory) int32
            let category = SpellCategoryKey::new(crate::util::read_i32_le(chunk)?.into());

            // cast_u_i: int32
            let cast_u_i = crate::util::read_i32_le(chunk)?;

            // dispel_type: foreign_key (SpellDispelType) int32
            let dispel_type = SpellDispelTypeKey::new(crate::util::read_i32_le(chunk)?.into());

            // mechanic: foreign_key (SpellMechanic) int32
            let mechanic = SpellMechanicKey::new(crate::util::read_i32_le(chunk)?.into());

            // attributes: int32
            let attributes = crate::util::read_i32_le(chunk)?;

            // attributes_ex: int32
            let attributes_ex = crate::util::read_i32_le(chunk)?;

            // attributes_ex_b: int32
            let attributes_ex_b = crate::util::read_i32_le(chunk)?;

            // attributes_ex_c: int32
            let attributes_ex_c = crate::util::read_i32_le(chunk)?;

            // attributes_ex_d: int32
            let attributes_ex_d = crate::util::read_i32_le(chunk)?;

            // attributes_ex_e: int32
            let attributes_ex_e = crate::util::read_i32_le(chunk)?;

            // attributes_ex_f: int32
            let attributes_ex_f = crate::util::read_i32_le(chunk)?;

            // shapeshift_mask: int32
            let shapeshift_mask = crate::util::read_i32_le(chunk)?;

            // shapeshift_exclude: int32
            let shapeshift_exclude = crate::util::read_i32_le(chunk)?;

            // targets: int32
            let targets = crate::util::read_i32_le(chunk)?;

            // target_creature_type: int32
            let target_creature_type = crate::util::read_i32_le(chunk)?;

            // requires_spell_focus: foreign_key (SpellFocusObject) int32
            let requires_spell_focus = SpellFocusObjectKey::new(crate::util::read_i32_le(chunk)?.into());

            // facing_caster_flags: int32
            let facing_caster_flags = crate::util::read_i32_le(chunk)?;

            // caster_aura_state: int32
            let caster_aura_state = crate::util::read_i32_le(chunk)?;

            // target_aura_state: int32
            let target_aura_state = crate::util::read_i32_le(chunk)?;

            // exclude_caster_aura_state: int32
            let exclude_caster_aura_state = crate::util::read_i32_le(chunk)?;

            // exclude_target_aura_state: int32
            let exclude_target_aura_state = crate::util::read_i32_le(chunk)?;

            // casting_time_index: foreign_key (SpellCastTimes) int32
            let casting_time_index = SpellCastTimesKey::new(crate::util::read_i32_le(chunk)?.into());

            // recovery_time: int32
            let recovery_time = crate::util::read_i32_le(chunk)?;

            // category_recovery_time: int32
            let category_recovery_time = crate::util::read_i32_le(chunk)?;

            // interrupt_flags: int32
            let interrupt_flags = crate::util::read_i32_le(chunk)?;

            // aura_interrupt_flags: int32
            let aura_interrupt_flags = crate::util::read_i32_le(chunk)?;

            // channel_interrupt_flags: int32
            let channel_interrupt_flags = crate::util::read_i32_le(chunk)?;

            // proc_type_mask: int32
            let proc_type_mask = crate::util::read_i32_le(chunk)?;

            // proc_chance: int32
            let proc_chance = crate::util::read_i32_le(chunk)?;

            // proc_charges: int32
            let proc_charges = crate::util::read_i32_le(chunk)?;

            // max_level: int32
            let max_level = crate::util::read_i32_le(chunk)?;

            // base_level: int32
            let base_level = crate::util::read_i32_le(chunk)?;

            // spell_level: int32
            let spell_level = crate::util::read_i32_le(chunk)?;

            // duration_index: foreign_key (SpellDuration) int32
            let duration_index = SpellDurationKey::new(crate::util::read_i32_le(chunk)?.into());

            // power_type: int32
            let power_type = crate::util::read_i32_le(chunk)?;

            // mana_cost: int32
            let mana_cost = crate::util::read_i32_le(chunk)?;

            // mana_cost_per_level: int32
            let mana_cost_per_level = crate::util::read_i32_le(chunk)?;

            // mana_per_second: int32
            let mana_per_second = crate::util::read_i32_le(chunk)?;

            // mana_per_second_per_level: int32
            let mana_per_second_per_level = crate::util::read_i32_le(chunk)?;

            // range_index: int32
            let range_index = crate::util::read_i32_le(chunk)?;

            // speed: float
            let speed = crate::util::read_f32_le(chunk)?;

            // modal_next_spell: int32
            let modal_next_spell = crate::util::read_i32_le(chunk)?;

            // cumulative_aura: int32
            let cumulative_aura = crate::util::read_i32_le(chunk)?;

            // totem: int32[2]
            let totem = crate::util::read_array_i32::<2>(chunk)?;

            // reagent: int32[8]
            let reagent = crate::util::read_array_i32::<8>(chunk)?;

            // reagent_count: int32[8]
            let reagent_count = crate::util::read_array_i32::<8>(chunk)?;

            // equipped_item_class: int32
            let equipped_item_class = crate::util::read_i32_le(chunk)?;

            // equipped_item_subclass: int32
            let equipped_item_subclass = crate::util::read_i32_le(chunk)?;

            // equipped_item_inv_types: int32
            let equipped_item_inv_types = crate::util::read_i32_le(chunk)?;

            // effect: int32[3]
            let effect = crate::util::read_array_i32::<3>(chunk)?;

            // effect_die_sides: int32[3]
            let effect_die_sides = crate::util::read_array_i32::<3>(chunk)?;

            // effect_base_dice: int32[3]
            let effect_base_dice = crate::util::read_array_i32::<3>(chunk)?;

            // effect_dice_per_level: int32[3]
            let effect_dice_per_level = crate::util::read_array_i32::<3>(chunk)?;

            // effect_real_points_per_level: float[3]
            let effect_real_points_per_level = crate::util::read_array_f32::<3>(chunk)?;

            // effect_base_points: int32[3]
            let effect_base_points = crate::util::read_array_i32::<3>(chunk)?;

            // effect_mechanic: int32[3]
            let effect_mechanic = crate::util::read_array_i32::<3>(chunk)?;

            // implicit_target_a: int32[3]
            let implicit_target_a = crate::util::read_array_i32::<3>(chunk)?;

            // implicit_target_b: int32[3]
            let implicit_target_b = crate::util::read_array_i32::<3>(chunk)?;

            // effect_radius_index: int32[3]
            let effect_radius_index = crate::util::read_array_i32::<3>(chunk)?;

            // effect_aura: AuraMod[3]
            let effect_aura = {
                let mut arr = [AuraMod::default(); 3];
                for i in arr.iter_mut() {
                    *i = crate::util::read_i32_le(chunk)?.try_into()?;
                }

                arr
            };

            // effect_aura_period: int32[3]
            let effect_aura_period = crate::util::read_array_i32::<3>(chunk)?;

            // effect_amplitude: float[3]
            let effect_amplitude = crate::util::read_array_f32::<3>(chunk)?;

            // effect_chain_targets: int32[3]
            let effect_chain_targets = crate::util::read_array_i32::<3>(chunk)?;

            // effect_item_type: int32[3]
            let effect_item_type = crate::util::read_array_i32::<3>(chunk)?;

            // effect_misc_value: int32[3]
            let effect_misc_value = crate::util::read_array_i32::<3>(chunk)?;

            // effect_misc_value_b: int32[3]
            let effect_misc_value_b = crate::util::read_array_i32::<3>(chunk)?;

            // effect_trigger_spell: int32[3]
            let effect_trigger_spell = crate::util::read_array_i32::<3>(chunk)?;

            // effect_points_per_combo: float[3]
            let effect_points_per_combo = crate::util::read_array_f32::<3>(chunk)?;

            // spell_visual_id: int32[2]
            let spell_visual_id = crate::util::read_array_i32::<2>(chunk)?;

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // active_icon_id: foreign_key (SpellIcon) int32
            let active_icon_id = SpellIconKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_priority: int32
            let spell_priority = crate::util::read_i32_le(chunk)?;

            // name_lang: string_ref_loc (Extended)
            let name_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // name_subtext_lang: string_ref_loc (Extended)
            let name_subtext_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // description_lang: string_ref_loc (Extended)
            let description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // aura_description_lang: string_ref_loc (Extended)
            let aura_description_lang = crate::util::read_extended_localized_string(chunk, &string_block)?;

            // mana_cost_pct: int32
            let mana_cost_pct = crate::util::read_i32_le(chunk)?;

            // start_recovery_category: int32
            let start_recovery_category = crate::util::read_i32_le(chunk)?;

            // start_recovery_time: int32
            let start_recovery_time = crate::util::read_i32_le(chunk)?;

            // max_target_level: int32
            let max_target_level = crate::util::read_i32_le(chunk)?;

            // spell_class_set: int32
            let spell_class_set = crate::util::read_i32_le(chunk)?;

            // spell_class_mask: int32[2]
            let spell_class_mask = crate::util::read_array_i32::<2>(chunk)?;

            // max_targets: int32
            let max_targets = crate::util::read_i32_le(chunk)?;

            // defense_type: int32
            let defense_type = crate::util::read_i32_le(chunk)?;

            // prevention_type: int32
            let prevention_type = crate::util::read_i32_le(chunk)?;

            // stance_bar_order: int32
            let stance_bar_order = crate::util::read_i32_le(chunk)?;

            // effect_chain_amplitude: float[3]
            let effect_chain_amplitude = crate::util::read_array_f32::<3>(chunk)?;

            // min_faction_id: foreign_key (Faction) int32
            let min_faction_id = FactionKey::new(crate::util::read_i32_le(chunk)?.into());

            // min_reputation: int32
            let min_reputation = crate::util::read_i32_le(chunk)?;

            // required_aura_vision: int32
            let required_aura_vision = crate::util::read_i32_le(chunk)?;

            // required_totem_category_id: int32[2]
            let required_totem_category_id = crate::util::read_array_i32::<2>(chunk)?;

            // required_areas_id: foreign_key (AreaGroup) int32
            let required_areas_id = crate::util::read_i32_le(chunk)?;

            // school_mask: int32
            let school_mask = crate::util::read_i32_le(chunk)?;


            rows.push(SpellRow {
                id,
                category,
                cast_u_i,
                dispel_type,
                mechanic,
                attributes,
                attributes_ex,
                attributes_ex_b,
                attributes_ex_c,
                attributes_ex_d,
                attributes_ex_e,
                attributes_ex_f,
                shapeshift_mask,
                shapeshift_exclude,
                targets,
                target_creature_type,
                requires_spell_focus,
                facing_caster_flags,
                caster_aura_state,
                target_aura_state,
                exclude_caster_aura_state,
                exclude_target_aura_state,
                casting_time_index,
                recovery_time,
                category_recovery_time,
                interrupt_flags,
                aura_interrupt_flags,
                channel_interrupt_flags,
                proc_type_mask,
                proc_chance,
                proc_charges,
                max_level,
                base_level,
                spell_level,
                duration_index,
                power_type,
                mana_cost,
                mana_cost_per_level,
                mana_per_second,
                mana_per_second_per_level,
                range_index,
                speed,
                modal_next_spell,
                cumulative_aura,
                totem,
                reagent,
                reagent_count,
                equipped_item_class,
                equipped_item_subclass,
                equipped_item_inv_types,
                effect,
                effect_die_sides,
                effect_base_dice,
                effect_dice_per_level,
                effect_real_points_per_level,
                effect_base_points,
                effect_mechanic,
                implicit_target_a,
                implicit_target_b,
                effect_radius_index,
                effect_aura,
                effect_aura_period,
                effect_amplitude,
                effect_chain_targets,
                effect_item_type,
                effect_misc_value,
                effect_misc_value_b,
                effect_trigger_spell,
                effect_points_per_combo,
                spell_visual_id,
                spell_icon_id,
                active_icon_id,
                spell_priority,
                name_lang,
                name_subtext_lang,
                description_lang,
                aura_description_lang,
                mana_cost_pct,
                start_recovery_category,
                start_recovery_time,
                max_target_level,
                spell_class_set,
                spell_class_mask,
                max_targets,
                defense_type,
                prevention_type,
                stance_bar_order,
                effect_chain_amplitude,
                min_faction_id,
                min_reputation,
                required_aura_vision,
                required_totem_category_id,
                required_areas_id,
                school_mask,
            });
        }

        Ok(Spell { rows, })
    }

    fn write(&self, w: &mut impl Write) -> Result<(), std::io::Error> {
        let mut b = Vec::with_capacity(self.rows.len() * Self::ROW_SIZE);

        let mut string_cache = StringCache::new();

        for row in &self.rows {
            // id: primary_key (Spell) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // category: foreign_key (SpellCategory) int32
            b.write_all(&(row.category.id as i32).to_le_bytes())?;

            // cast_u_i: int32
            b.write_all(&row.cast_u_i.to_le_bytes())?;

            // dispel_type: foreign_key (SpellDispelType) int32
            b.write_all(&(row.dispel_type.id as i32).to_le_bytes())?;

            // mechanic: foreign_key (SpellMechanic) int32
            b.write_all(&(row.mechanic.id as i32).to_le_bytes())?;

            // attributes: int32
            b.write_all(&row.attributes.to_le_bytes())?;

            // attributes_ex: int32
            b.write_all(&row.attributes_ex.to_le_bytes())?;

            // attributes_ex_b: int32
            b.write_all(&row.attributes_ex_b.to_le_bytes())?;

            // attributes_ex_c: int32
            b.write_all(&row.attributes_ex_c.to_le_bytes())?;

            // attributes_ex_d: int32
            b.write_all(&row.attributes_ex_d.to_le_bytes())?;

            // attributes_ex_e: int32
            b.write_all(&row.attributes_ex_e.to_le_bytes())?;

            // attributes_ex_f: int32
            b.write_all(&row.attributes_ex_f.to_le_bytes())?;

            // shapeshift_mask: int32
            b.write_all(&row.shapeshift_mask.to_le_bytes())?;

            // shapeshift_exclude: int32
            b.write_all(&row.shapeshift_exclude.to_le_bytes())?;

            // targets: int32
            b.write_all(&row.targets.to_le_bytes())?;

            // target_creature_type: int32
            b.write_all(&row.target_creature_type.to_le_bytes())?;

            // requires_spell_focus: foreign_key (SpellFocusObject) int32
            b.write_all(&(row.requires_spell_focus.id as i32).to_le_bytes())?;

            // facing_caster_flags: int32
            b.write_all(&row.facing_caster_flags.to_le_bytes())?;

            // caster_aura_state: int32
            b.write_all(&row.caster_aura_state.to_le_bytes())?;

            // target_aura_state: int32
            b.write_all(&row.target_aura_state.to_le_bytes())?;

            // exclude_caster_aura_state: int32
            b.write_all(&row.exclude_caster_aura_state.to_le_bytes())?;

            // exclude_target_aura_state: int32
            b.write_all(&row.exclude_target_aura_state.to_le_bytes())?;

            // casting_time_index: foreign_key (SpellCastTimes) int32
            b.write_all(&(row.casting_time_index.id as i32).to_le_bytes())?;

            // recovery_time: int32
            b.write_all(&row.recovery_time.to_le_bytes())?;

            // category_recovery_time: int32
            b.write_all(&row.category_recovery_time.to_le_bytes())?;

            // interrupt_flags: int32
            b.write_all(&row.interrupt_flags.to_le_bytes())?;

            // aura_interrupt_flags: int32
            b.write_all(&row.aura_interrupt_flags.to_le_bytes())?;

            // channel_interrupt_flags: int32
            b.write_all(&row.channel_interrupt_flags.to_le_bytes())?;

            // proc_type_mask: int32
            b.write_all(&row.proc_type_mask.to_le_bytes())?;

            // proc_chance: int32
            b.write_all(&row.proc_chance.to_le_bytes())?;

            // proc_charges: int32
            b.write_all(&row.proc_charges.to_le_bytes())?;

            // max_level: int32
            b.write_all(&row.max_level.to_le_bytes())?;

            // base_level: int32
            b.write_all(&row.base_level.to_le_bytes())?;

            // spell_level: int32
            b.write_all(&row.spell_level.to_le_bytes())?;

            // duration_index: foreign_key (SpellDuration) int32
            b.write_all(&(row.duration_index.id as i32).to_le_bytes())?;

            // power_type: int32
            b.write_all(&row.power_type.to_le_bytes())?;

            // mana_cost: int32
            b.write_all(&row.mana_cost.to_le_bytes())?;

            // mana_cost_per_level: int32
            b.write_all(&row.mana_cost_per_level.to_le_bytes())?;

            // mana_per_second: int32
            b.write_all(&row.mana_per_second.to_le_bytes())?;

            // mana_per_second_per_level: int32
            b.write_all(&row.mana_per_second_per_level.to_le_bytes())?;

            // range_index: int32
            b.write_all(&row.range_index.to_le_bytes())?;

            // speed: float
            b.write_all(&row.speed.to_le_bytes())?;

            // modal_next_spell: int32
            b.write_all(&row.modal_next_spell.to_le_bytes())?;

            // cumulative_aura: int32
            b.write_all(&row.cumulative_aura.to_le_bytes())?;

            // totem: int32[2]
            for i in row.totem {
                b.write_all(&i.to_le_bytes())?;
            }


            // reagent: int32[8]
            for i in row.reagent {
                b.write_all(&i.to_le_bytes())?;
            }


            // reagent_count: int32[8]
            for i in row.reagent_count {
                b.write_all(&i.to_le_bytes())?;
            }


            // equipped_item_class: int32
            b.write_all(&row.equipped_item_class.to_le_bytes())?;

            // equipped_item_subclass: int32
            b.write_all(&row.equipped_item_subclass.to_le_bytes())?;

            // equipped_item_inv_types: int32
            b.write_all(&row.equipped_item_inv_types.to_le_bytes())?;

            // effect: int32[3]
            for i in row.effect {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_die_sides: int32[3]
            for i in row.effect_die_sides {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_base_dice: int32[3]
            for i in row.effect_base_dice {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_dice_per_level: int32[3]
            for i in row.effect_dice_per_level {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_real_points_per_level: float[3]
            for i in row.effect_real_points_per_level {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_base_points: int32[3]
            for i in row.effect_base_points {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_mechanic: int32[3]
            for i in row.effect_mechanic {
                b.write_all(&i.to_le_bytes())?;
            }


            // implicit_target_a: int32[3]
            for i in row.implicit_target_a {
                b.write_all(&i.to_le_bytes())?;
            }


            // implicit_target_b: int32[3]
            for i in row.implicit_target_b {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_radius_index: int32[3]
            for i in row.effect_radius_index {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_aura: AuraMod[3]
            for i in row.effect_aura {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // effect_aura_period: int32[3]
            for i in row.effect_aura_period {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_amplitude: float[3]
            for i in row.effect_amplitude {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_chain_targets: int32[3]
            for i in row.effect_chain_targets {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_item_type: int32[3]
            for i in row.effect_item_type {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_misc_value: int32[3]
            for i in row.effect_misc_value {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_misc_value_b: int32[3]
            for i in row.effect_misc_value_b {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_trigger_spell: int32[3]
            for i in row.effect_trigger_spell {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_points_per_combo: float[3]
            for i in row.effect_points_per_combo {
                b.write_all(&i.to_le_bytes())?;
            }


            // spell_visual_id: int32[2]
            for i in row.spell_visual_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // spell_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.spell_icon_id.id as i32).to_le_bytes())?;

            // active_icon_id: foreign_key (SpellIcon) int32
            b.write_all(&(row.active_icon_id.id as i32).to_le_bytes())?;

            // spell_priority: int32
            b.write_all(&row.spell_priority.to_le_bytes())?;

            // name_lang: string_ref_loc (Extended)
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_cache))?;

            // name_subtext_lang: string_ref_loc (Extended)
            b.write_all(&row.name_subtext_lang.string_indices_as_array(&mut string_cache))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_cache))?;

            // aura_description_lang: string_ref_loc (Extended)
            b.write_all(&row.aura_description_lang.string_indices_as_array(&mut string_cache))?;

            // mana_cost_pct: int32
            b.write_all(&row.mana_cost_pct.to_le_bytes())?;

            // start_recovery_category: int32
            b.write_all(&row.start_recovery_category.to_le_bytes())?;

            // start_recovery_time: int32
            b.write_all(&row.start_recovery_time.to_le_bytes())?;

            // max_target_level: int32
            b.write_all(&row.max_target_level.to_le_bytes())?;

            // spell_class_set: int32
            b.write_all(&row.spell_class_set.to_le_bytes())?;

            // spell_class_mask: int32[2]
            for i in row.spell_class_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // max_targets: int32
            b.write_all(&row.max_targets.to_le_bytes())?;

            // defense_type: int32
            b.write_all(&row.defense_type.to_le_bytes())?;

            // prevention_type: int32
            b.write_all(&row.prevention_type.to_le_bytes())?;

            // stance_bar_order: int32
            b.write_all(&row.stance_bar_order.to_le_bytes())?;

            // effect_chain_amplitude: float[3]
            for i in row.effect_chain_amplitude {
                b.write_all(&i.to_le_bytes())?;
            }


            // min_faction_id: foreign_key (Faction) int32
            b.write_all(&(row.min_faction_id.id as i32).to_le_bytes())?;

            // min_reputation: int32
            b.write_all(&row.min_reputation.to_le_bytes())?;

            // required_aura_vision: int32
            b.write_all(&row.required_aura_vision.to_le_bytes())?;

            // required_totem_category_id: int32[2]
            for i in row.required_totem_category_id {
                b.write_all(&i.to_le_bytes())?;
            }


            // required_areas_id: foreign_key (AreaGroup) int32
            b.write_all(&row.required_areas_id.to_le_bytes())?;

            // school_mask: int32
            b.write_all(&row.school_mask.to_le_bytes())?;

        }

        assert_eq!(b.len(), self.rows.len() * Self::ROW_SIZE);
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: Self::FIELD_COUNT as u32,
            record_size: Self::ROW_SIZE as u32,
            string_block_size: string_cache.size(),
        };

        w.write_all(&header.write_header())?;
        w.write_all(&b)?;
        w.write_all(string_cache.buffer())?;
        Ok(())
    }

}

impl Indexable for Spell {
    type PrimaryKey = SpellKey;
    fn get(&self, key: impl TryInto<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl TryInto<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.try_into().ok()?;
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellKey {
    pub id: i32
}

impl SpellKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
    }

}

impl From<u8> for SpellKey {
    fn from(v: u8) -> Self {
        Self::new(v.into())
    }
}

impl From<u16> for SpellKey {
    fn from(v: u16) -> Self {
        Self::new(v.into())
    }
}

impl From<i8> for SpellKey {
    fn from(v: i8) -> Self {
        Self::new(v.into())
    }
}

impl From<i16> for SpellKey {
    fn from(v: i16) -> Self {
        Self::new(v.into())
    }
}

impl From<i32> for SpellKey {
    fn from(v: i32) -> Self {
        Self::new(v)
    }
}

impl TryFrom<u32> for SpellKey {
    type Error = u32;
    fn try_from(v: u32) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<usize> for SpellKey {
    type Error = usize;
    fn try_from(v: usize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<u64> for SpellKey {
    type Error = u64;
    fn try_from(v: u64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<i64> for SpellKey {
    type Error = i64;
    fn try_from(v: i64) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

impl TryFrom<isize> for SpellKey {
    type Error = isize;
    fn try_from(v: isize) -> Result<Self, Self::Error> {
        Ok(TryInto::<i32>::try_into(v).ok().ok_or(v)?.into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpellRow {
    pub id: SpellKey,
    pub category: SpellCategoryKey,
    pub cast_u_i: i32,
    pub dispel_type: SpellDispelTypeKey,
    pub mechanic: SpellMechanicKey,
    pub attributes: i32,
    pub attributes_ex: i32,
    pub attributes_ex_b: i32,
    pub attributes_ex_c: i32,
    pub attributes_ex_d: i32,
    pub attributes_ex_e: i32,
    pub attributes_ex_f: i32,
    pub shapeshift_mask: i32,
    pub shapeshift_exclude: i32,
    pub targets: i32,
    pub target_creature_type: i32,
    pub requires_spell_focus: SpellFocusObjectKey,
    pub facing_caster_flags: i32,
    pub caster_aura_state: i32,
    pub target_aura_state: i32,
    pub exclude_caster_aura_state: i32,
    pub exclude_target_aura_state: i32,
    pub casting_time_index: SpellCastTimesKey,
    pub recovery_time: i32,
    pub category_recovery_time: i32,
    pub interrupt_flags: i32,
    pub aura_interrupt_flags: i32,
    pub channel_interrupt_flags: i32,
    pub proc_type_mask: i32,
    pub proc_chance: i32,
    pub proc_charges: i32,
    pub max_level: i32,
    pub base_level: i32,
    pub spell_level: i32,
    pub duration_index: SpellDurationKey,
    pub power_type: i32,
    pub mana_cost: i32,
    pub mana_cost_per_level: i32,
    pub mana_per_second: i32,
    pub mana_per_second_per_level: i32,
    pub range_index: i32,
    pub speed: f32,
    pub modal_next_spell: i32,
    pub cumulative_aura: i32,
    pub totem: [i32; 2],
    pub reagent: [i32; 8],
    pub reagent_count: [i32; 8],
    pub equipped_item_class: i32,
    pub equipped_item_subclass: i32,
    pub equipped_item_inv_types: i32,
    pub effect: [i32; 3],
    pub effect_die_sides: [i32; 3],
    pub effect_base_dice: [i32; 3],
    pub effect_dice_per_level: [i32; 3],
    pub effect_real_points_per_level: [f32; 3],
    pub effect_base_points: [i32; 3],
    pub effect_mechanic: [i32; 3],
    pub implicit_target_a: [i32; 3],
    pub implicit_target_b: [i32; 3],
    pub effect_radius_index: [i32; 3],
    pub effect_aura: [AuraMod; 3],
    pub effect_aura_period: [i32; 3],
    pub effect_amplitude: [f32; 3],
    pub effect_chain_targets: [i32; 3],
    pub effect_item_type: [i32; 3],
    pub effect_misc_value: [i32; 3],
    pub effect_misc_value_b: [i32; 3],
    pub effect_trigger_spell: [i32; 3],
    pub effect_points_per_combo: [f32; 3],
    pub spell_visual_id: [i32; 2],
    pub spell_icon_id: SpellIconKey,
    pub active_icon_id: SpellIconKey,
    pub spell_priority: i32,
    pub name_lang: ExtendedLocalizedString,
    pub name_subtext_lang: ExtendedLocalizedString,
    pub description_lang: ExtendedLocalizedString,
    pub aura_description_lang: ExtendedLocalizedString,
    pub mana_cost_pct: i32,
    pub start_recovery_category: i32,
    pub start_recovery_time: i32,
    pub max_target_level: i32,
    pub spell_class_set: i32,
    pub spell_class_mask: [i32; 2],
    pub max_targets: i32,
    pub defense_type: i32,
    pub prevention_type: i32,
    pub stance_bar_order: i32,
    pub effect_chain_amplitude: [f32; 3],
    pub min_faction_id: FactionKey,
    pub min_reputation: i32,
    pub required_aura_vision: i32,
    pub required_totem_category_id: [i32; 2],
    pub required_areas_id: i32,
    pub school_mask: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    #[ignore = "requires DBC files"]
    fn spell() {
        let mut file = File::open("../tbc-dbc/Spell.dbc").expect("Failed to open DBC file");
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).expect("Failed to read DBC file");
        let actual = Spell::read(&mut contents.as_slice()).unwrap();
        let mut v = Vec::with_capacity(contents.len());
        actual.write(&mut v).unwrap();
        let new = Spell::read(&mut v.as_slice()).unwrap();
        assert_eq!(actual, new);
    }
}
