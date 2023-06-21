use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstExtendedLocalizedString, ExtendedLocalizedString};
use crate::wrath_tables::area_group::*;
use crate::wrath_tables::faction::*;
use crate::wrath_tables::power_display::*;
use crate::wrath_tables::spell_cast_times::*;
use crate::wrath_tables::spell_category::*;
use crate::wrath_tables::spell_description_variables::*;
use crate::wrath_tables::spell_dispel_type::*;
use crate::wrath_tables::spell_duration::*;
use crate::wrath_tables::spell_focus_object::*;
use crate::wrath_tables::spell_icon::*;
use crate::wrath_tables::spell_mechanic::*;
use crate::wrath_tables::spell_missile::*;
use crate::wrath_tables::spell_rune_cost::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Spell {
    pub rows: Vec<SpellRow>,
}

impl DbcTable for Spell {
    type Row = SpellRow;

    fn filename() -> &'static str { "Spell.dbc" }

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = header::parse_header(&header)?;

        if header.record_size != 936 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 936,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 234 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 234,
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

            // attributes_ex_g: int32
            let attributes_ex_g = crate::util::read_i32_le(chunk)?;

            // shapeshift_mask: int32[2]
            let shapeshift_mask = crate::util::read_array_i32::<2>(chunk)?;

            // shapeshift_exclude: int32[2]
            let shapeshift_exclude = crate::util::read_array_i32::<2>(chunk)?;

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

            // caster_aura_spell: int32
            let caster_aura_spell = crate::util::read_i32_le(chunk)?;

            // target_aura_spell: int32
            let target_aura_spell = crate::util::read_i32_le(chunk)?;

            // exclude_caster_aura_spell: int32
            let exclude_caster_aura_spell = crate::util::read_i32_le(chunk)?;

            // exclude_target_aura_spell: int32
            let exclude_target_aura_spell = crate::util::read_i32_le(chunk)?;

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

            // effect_aura: EffectAura[3]
            let effect_aura = {
                let mut arr = [EffectAura::default(); 3];
                for i in arr.iter_mut() {
                    *i = EffectAura::try_from(crate::util::read_i32_le(chunk)?)?;
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

            // effect_spell_class_mask_a: int32[3]
            let effect_spell_class_mask_a = crate::util::read_array_i32::<3>(chunk)?;

            // effect_spell_class_mask_b: int32[3]
            let effect_spell_class_mask_b = crate::util::read_array_i32::<3>(chunk)?;

            // effect_spell_class_mask_c: int32[3]
            let effect_spell_class_mask_c = crate::util::read_array_i32::<3>(chunk)?;

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

            // spell_class_mask: int32[3]
            let spell_class_mask = crate::util::read_array_i32::<3>(chunk)?;

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
            let required_areas_id = AreaGroupKey::new(crate::util::read_i32_le(chunk)?.into());

            // school_mask: int32
            let school_mask = crate::util::read_i32_le(chunk)?;

            // rune_cost_id: foreign_key (SpellRuneCost) int32
            let rune_cost_id = SpellRuneCostKey::new(crate::util::read_i32_le(chunk)?.into());

            // spell_missile_id: foreign_key (SpellMissile) int32
            let spell_missile_id = SpellMissileKey::new(crate::util::read_i32_le(chunk)?.into());

            // power_display_id: foreign_key (PowerDisplay) int32
            let power_display_id = PowerDisplayKey::new(crate::util::read_i32_le(chunk)?.into());

            // effect_bonus_coefficient: float[3]
            let effect_bonus_coefficient = crate::util::read_array_f32::<3>(chunk)?;

            // description_variables_id: foreign_key (SpellDescriptionVariables) int32
            let description_variables_id = SpellDescriptionVariablesKey::new(crate::util::read_i32_le(chunk)?.into());

            // difficulty: int32
            let difficulty = crate::util::read_i32_le(chunk)?;


            rows.push(SpellRow {
                id,
                category,
                dispel_type,
                mechanic,
                attributes,
                attributes_ex,
                attributes_ex_b,
                attributes_ex_c,
                attributes_ex_d,
                attributes_ex_e,
                attributes_ex_f,
                attributes_ex_g,
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
                caster_aura_spell,
                target_aura_spell,
                exclude_caster_aura_spell,
                exclude_target_aura_spell,
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
                effect_spell_class_mask_a,
                effect_spell_class_mask_b,
                effect_spell_class_mask_c,
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
                rune_cost_id,
                spell_missile_id,
                power_display_id,
                effect_bonus_coefficient,
                description_variables_id,
                difficulty,
            });
        }

        Ok(Spell { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 234,
            record_size: 936,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Spell) int32
            b.write_all(&row.id.id.to_le_bytes())?;

            // category: foreign_key (SpellCategory) int32
            b.write_all(&(row.category.id as i32).to_le_bytes())?;

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

            // attributes_ex_g: int32
            b.write_all(&row.attributes_ex_g.to_le_bytes())?;

            // shapeshift_mask: int32[2]
            for i in row.shapeshift_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // shapeshift_exclude: int32[2]
            for i in row.shapeshift_exclude {
                b.write_all(&i.to_le_bytes())?;
            }


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

            // caster_aura_spell: int32
            b.write_all(&row.caster_aura_spell.to_le_bytes())?;

            // target_aura_spell: int32
            b.write_all(&row.target_aura_spell.to_le_bytes())?;

            // exclude_caster_aura_spell: int32
            b.write_all(&row.exclude_caster_aura_spell.to_le_bytes())?;

            // exclude_target_aura_spell: int32
            b.write_all(&row.exclude_target_aura_spell.to_le_bytes())?;

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


            // effect_aura: EffectAura[3]
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


            // effect_spell_class_mask_a: int32[3]
            for i in row.effect_spell_class_mask_a {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_spell_class_mask_b: int32[3]
            for i in row.effect_spell_class_mask_b {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_spell_class_mask_c: int32[3]
            for i in row.effect_spell_class_mask_c {
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
            b.write_all(&row.name_lang.string_indices_as_array(&mut string_index))?;

            // name_subtext_lang: string_ref_loc (Extended)
            b.write_all(&row.name_subtext_lang.string_indices_as_array(&mut string_index))?;

            // description_lang: string_ref_loc (Extended)
            b.write_all(&row.description_lang.string_indices_as_array(&mut string_index))?;

            // aura_description_lang: string_ref_loc (Extended)
            b.write_all(&row.aura_description_lang.string_indices_as_array(&mut string_index))?;

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

            // spell_class_mask: int32[3]
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
            b.write_all(&(row.required_areas_id.id as i32).to_le_bytes())?;

            // school_mask: int32
            b.write_all(&row.school_mask.to_le_bytes())?;

            // rune_cost_id: foreign_key (SpellRuneCost) int32
            b.write_all(&(row.rune_cost_id.id as i32).to_le_bytes())?;

            // spell_missile_id: foreign_key (SpellMissile) int32
            b.write_all(&(row.spell_missile_id.id as i32).to_le_bytes())?;

            // power_display_id: foreign_key (PowerDisplay) int32
            b.write_all(&(row.power_display_id.id as i32).to_le_bytes())?;

            // effect_bonus_coefficient: float[3]
            for i in row.effect_bonus_coefficient {
                b.write_all(&i.to_le_bytes())?;
            }


            // description_variables_id: foreign_key (SpellDescriptionVariables) int32
            b.write_all(&(row.description_variables_id.id as i32).to_le_bytes())?;

            // difficulty: int32
            b.write_all(&row.difficulty.to_le_bytes())?;

        }

        self.write_string_block(b)?;

        Ok(())
    }

}

impl Indexable for Spell {
    type PrimaryKey = SpellKey;
    fn get(&self, key: impl Into<Self::PrimaryKey>) -> Option<&Self::Row> {
        let key = key.into();
        self.rows.iter().find(|a| a.id.id == key.id)
    }

    fn get_mut(&mut self, key: impl Into<Self::PrimaryKey>) -> Option<&mut Self::Row> {
        let key = key.into();
        self.rows.iter_mut().find(|a| a.id.id == key.id)
    }

}

impl Spell {
    fn write_string_block(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        b.write_all(&[0])?;

        for row in &self.rows {
            row.name_lang.string_block_as_array(b)?;
            row.name_subtext_lang.string_block_as_array(b)?;
            row.description_lang.string_block_as_array(b)?;
            row.aura_description_lang.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name_lang.string_block_size();
            sum += row.name_subtext_lang.string_block_size();
            sum += row.description_lang.string_block_size();
            sum += row.aura_description_lang.string_block_size();
        }

        sum as u32
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstSpell<const S: usize> {
    pub rows: [ConstSpellRow; S],
}

impl<const S: usize> ConstSpell<S> {
    pub const fn const_read(b: &'static [u8], header: &DbcHeader) -> Self {
        if header.record_size != 936 {
            panic!("invalid record size, expected 936")
        }

        if header.field_count != 234 {
            panic!("invalid field count, expected 234")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstSpellRow {
                id: SpellKey::new(0),
                category: SpellCategoryKey::new(0),
                dispel_type: SpellDispelTypeKey::new(0),
                mechanic: SpellMechanicKey::new(0),
                attributes: 0,
                attributes_ex: 0,
                attributes_ex_b: 0,
                attributes_ex_c: 0,
                attributes_ex_d: 0,
                attributes_ex_e: 0,
                attributes_ex_f: 0,
                attributes_ex_g: 0,
                shapeshift_mask: [0; 2],
                shapeshift_exclude: [0; 2],
                targets: 0,
                target_creature_type: 0,
                requires_spell_focus: SpellFocusObjectKey::new(0),
                facing_caster_flags: 0,
                caster_aura_state: 0,
                target_aura_state: 0,
                exclude_caster_aura_state: 0,
                exclude_target_aura_state: 0,
                caster_aura_spell: 0,
                target_aura_spell: 0,
                exclude_caster_aura_spell: 0,
                exclude_target_aura_spell: 0,
                casting_time_index: SpellCastTimesKey::new(0),
                recovery_time: 0,
                category_recovery_time: 0,
                interrupt_flags: 0,
                aura_interrupt_flags: 0,
                channel_interrupt_flags: 0,
                proc_type_mask: 0,
                proc_chance: 0,
                proc_charges: 0,
                max_level: 0,
                base_level: 0,
                spell_level: 0,
                duration_index: SpellDurationKey::new(0),
                power_type: 0,
                mana_cost: 0,
                mana_cost_per_level: 0,
                mana_per_second: 0,
                mana_per_second_per_level: 0,
                range_index: 0,
                speed: 0.0,
                modal_next_spell: 0,
                cumulative_aura: 0,
                totem: [0; 2],
                reagent: [0; 8],
                reagent_count: [0; 8],
                equipped_item_class: 0,
                equipped_item_subclass: 0,
                equipped_item_inv_types: 0,
                effect: [0; 3],
                effect_die_sides: [0; 3],
                effect_real_points_per_level: [0.0; 3],
                effect_base_points: [0; 3],
                effect_mechanic: [0; 3],
                implicit_target_a: [0; 3],
                implicit_target_b: [0; 3],
                effect_radius_index: [0; 3],
                effect_aura: [EffectAura::None; 3],
                effect_aura_period: [0; 3],
                effect_amplitude: [0.0; 3],
                effect_chain_targets: [0; 3],
                effect_item_type: [0; 3],
                effect_misc_value: [0; 3],
                effect_misc_value_b: [0; 3],
                effect_trigger_spell: [0; 3],
                effect_points_per_combo: [0.0; 3],
                effect_spell_class_mask_a: [0; 3],
                effect_spell_class_mask_b: [0; 3],
                effect_spell_class_mask_c: [0; 3],
                spell_visual_id: [0; 2],
                spell_icon_id: SpellIconKey::new(0),
                active_icon_id: SpellIconKey::new(0),
                spell_priority: 0,
                name_lang: crate::ConstExtendedLocalizedString::empty(),
                name_subtext_lang: crate::ConstExtendedLocalizedString::empty(),
                description_lang: crate::ConstExtendedLocalizedString::empty(),
                aura_description_lang: crate::ConstExtendedLocalizedString::empty(),
                mana_cost_pct: 0,
                start_recovery_category: 0,
                start_recovery_time: 0,
                max_target_level: 0,
                spell_class_set: 0,
                spell_class_mask: [0; 3],
                max_targets: 0,
                defense_type: 0,
                prevention_type: 0,
                stance_bar_order: 0,
                effect_chain_amplitude: [0.0; 3],
                min_faction_id: FactionKey::new(0),
                min_reputation: 0,
                required_aura_vision: 0,
                required_totem_category_id: [0; 2],
                required_areas_id: AreaGroupKey::new(0),
                school_mask: 0,
                rune_cost_id: SpellRuneCostKey::new(0),
                spell_missile_id: SpellMissileKey::new(0),
                power_display_id: PowerDisplayKey::new(0),
                effect_bonus_coefficient: [0.0; 3],
                description_variables_id: SpellDescriptionVariablesKey::new(0),
                difficulty: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Spell) int32
            let id = SpellKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // category: foreign_key (SpellCategory) int32
            let category = SpellCategoryKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // dispel_type: foreign_key (SpellDispelType) int32
            let dispel_type = SpellDispelTypeKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // mechanic: foreign_key (SpellMechanic) int32
            let mechanic = SpellMechanicKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attributes: int32
            let attributes = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex: int32
            let attributes_ex = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex_b: int32
            let attributes_ex_b = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex_c: int32
            let attributes_ex_c = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex_d: int32
            let attributes_ex_d = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex_e: int32
            let attributes_ex_e = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex_f: int32
            let attributes_ex_f = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // attributes_ex_g: int32
            let attributes_ex_g = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // shapeshift_mask: int32[2]
            let shapeshift_mask = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // shapeshift_exclude: int32[2]
            let shapeshift_exclude = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // targets: int32
            let targets = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_creature_type: int32
            let target_creature_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // requires_spell_focus: foreign_key (SpellFocusObject) int32
            let requires_spell_focus = SpellFocusObjectKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // facing_caster_flags: int32
            let facing_caster_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // caster_aura_state: int32
            let caster_aura_state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_aura_state: int32
            let target_aura_state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // exclude_caster_aura_state: int32
            let exclude_caster_aura_state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // exclude_target_aura_state: int32
            let exclude_target_aura_state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // caster_aura_spell: int32
            let caster_aura_spell = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_aura_spell: int32
            let target_aura_spell = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // exclude_caster_aura_spell: int32
            let exclude_caster_aura_spell = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // exclude_target_aura_spell: int32
            let exclude_target_aura_spell = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // casting_time_index: foreign_key (SpellCastTimes) int32
            let casting_time_index = SpellCastTimesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // recovery_time: int32
            let recovery_time = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // category_recovery_time: int32
            let category_recovery_time = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // interrupt_flags: int32
            let interrupt_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // aura_interrupt_flags: int32
            let aura_interrupt_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // channel_interrupt_flags: int32
            let channel_interrupt_flags = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // proc_type_mask: int32
            let proc_type_mask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // proc_chance: int32
            let proc_chance = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // proc_charges: int32
            let proc_charges = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_level: int32
            let max_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // base_level: int32
            let base_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // spell_level: int32
            let spell_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // duration_index: foreign_key (SpellDuration) int32
            let duration_index = SpellDurationKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // power_type: int32
            let power_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mana_cost: int32
            let mana_cost = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mana_cost_per_level: int32
            let mana_cost_per_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mana_per_second: int32
            let mana_per_second = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mana_per_second_per_level: int32
            let mana_per_second_per_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // range_index: int32
            let range_index = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // speed: float
            let speed = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // modal_next_spell: int32
            let modal_next_spell = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // cumulative_aura: int32
            let cumulative_aura = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // totem: int32[2]
            let totem = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // reagent: int32[8]
            let reagent = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // reagent_count: int32[8]
            let reagent_count = {
                let mut a = [0; 8];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // equipped_item_class: int32
            let equipped_item_class = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // equipped_item_subclass: int32
            let equipped_item_subclass = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // equipped_item_inv_types: int32
            let equipped_item_inv_types = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // effect: int32[3]
            let effect = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_die_sides: int32[3]
            let effect_die_sides = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_real_points_per_level: float[3]
            let effect_real_points_per_level = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_base_points: int32[3]
            let effect_base_points = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_mechanic: int32[3]
            let effect_mechanic = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // implicit_target_a: int32[3]
            let implicit_target_a = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // implicit_target_b: int32[3]
            let implicit_target_b = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_radius_index: int32[3]
            let effect_radius_index = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_aura: EffectAura[3]
            let effect_aura = {
                let mut a = [EffectAura::None; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = match EffectAura::from_value(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]])) {
                        Some(e) => e,
                        None => panic!(),
                    };
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_aura_period: int32[3]
            let effect_aura_period = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_amplitude: float[3]
            let effect_amplitude = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_chain_targets: int32[3]
            let effect_chain_targets = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_item_type: int32[3]
            let effect_item_type = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_misc_value: int32[3]
            let effect_misc_value = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_misc_value_b: int32[3]
            let effect_misc_value_b = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_trigger_spell: int32[3]
            let effect_trigger_spell = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_points_per_combo: float[3]
            let effect_points_per_combo = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_spell_class_mask_a: int32[3]
            let effect_spell_class_mask_a = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_spell_class_mask_b: int32[3]
            let effect_spell_class_mask_b = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_spell_class_mask_c: int32[3]
            let effect_spell_class_mask_c = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // spell_visual_id: int32[2]
            let spell_visual_id = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // spell_icon_id: foreign_key (SpellIcon) int32
            let spell_icon_id = SpellIconKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // active_icon_id: foreign_key (SpellIcon) int32
            let active_icon_id = SpellIconKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // spell_priority: int32
            let spell_priority = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // name_lang: string_ref_loc (Extended)
            let name_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // name_subtext_lang: string_ref_loc (Extended)
            let name_subtext_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // description_lang: string_ref_loc (Extended)
            let description_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // aura_description_lang: string_ref_loc (Extended)
            let aura_description_lang = ConstExtendedLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                crate::util::get_string_from_block(b_offset + 32, b, string_block),
                crate::util::get_string_from_block(b_offset + 36, b, string_block),
                crate::util::get_string_from_block(b_offset + 40, b, string_block),
                crate::util::get_string_from_block(b_offset + 44, b, string_block),
                crate::util::get_string_from_block(b_offset + 48, b, string_block),
                crate::util::get_string_from_block(b_offset + 52, b, string_block),
                crate::util::get_string_from_block(b_offset + 56, b, string_block),
                crate::util::get_string_from_block(b_offset + 60, b, string_block),
                u32::from_le_bytes([b[b_offset + 64], b[b_offset + 65], b[b_offset + 66], b[b_offset + 67]]),
            );
            b_offset += 68;

            // mana_cost_pct: int32
            let mana_cost_pct = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // start_recovery_category: int32
            let start_recovery_category = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // start_recovery_time: int32
            let start_recovery_time = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // max_target_level: int32
            let max_target_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // spell_class_set: int32
            let spell_class_set = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // spell_class_mask: int32[3]
            let spell_class_mask = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // max_targets: int32
            let max_targets = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // defense_type: int32
            let defense_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // prevention_type: int32
            let prevention_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // stance_bar_order: int32
            let stance_bar_order = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // effect_chain_amplitude: float[3]
            let effect_chain_amplitude = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // min_faction_id: foreign_key (Faction) int32
            let min_faction_id = FactionKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // min_reputation: int32
            let min_reputation = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // required_aura_vision: int32
            let required_aura_vision = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // required_totem_category_id: int32[2]
            let required_totem_category_id = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // required_areas_id: foreign_key (AreaGroup) int32
            let required_areas_id = AreaGroupKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // school_mask: int32
            let school_mask = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // rune_cost_id: foreign_key (SpellRuneCost) int32
            let rune_cost_id = SpellRuneCostKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // spell_missile_id: foreign_key (SpellMissile) int32
            let spell_missile_id = SpellMissileKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // power_display_id: foreign_key (PowerDisplay) int32
            let power_display_id = PowerDisplayKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // effect_bonus_coefficient: float[3]
            let effect_bonus_coefficient = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // description_variables_id: foreign_key (SpellDescriptionVariables) int32
            let description_variables_id = SpellDescriptionVariablesKey::new(i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // difficulty: int32
            let difficulty = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstSpellRow {
                id,
                category,
                dispel_type,
                mechanic,
                attributes,
                attributes_ex,
                attributes_ex_b,
                attributes_ex_c,
                attributes_ex_d,
                attributes_ex_e,
                attributes_ex_f,
                attributes_ex_g,
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
                caster_aura_spell,
                target_aura_spell,
                exclude_caster_aura_spell,
                exclude_target_aura_spell,
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
                effect_spell_class_mask_a,
                effect_spell_class_mask_b,
                effect_spell_class_mask_c,
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
                rune_cost_id,
                spell_missile_id,
                power_display_id,
                effect_bonus_coefficient,
                description_variables_id,
                difficulty,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Spell {
        Spell {
            rows: self.rows.iter().map(|s| SpellRow {
                id: s.id,
                category: s.category,
                dispel_type: s.dispel_type,
                mechanic: s.mechanic,
                attributes: s.attributes,
                attributes_ex: s.attributes_ex,
                attributes_ex_b: s.attributes_ex_b,
                attributes_ex_c: s.attributes_ex_c,
                attributes_ex_d: s.attributes_ex_d,
                attributes_ex_e: s.attributes_ex_e,
                attributes_ex_f: s.attributes_ex_f,
                attributes_ex_g: s.attributes_ex_g,
                shapeshift_mask: s.shapeshift_mask,
                shapeshift_exclude: s.shapeshift_exclude,
                targets: s.targets,
                target_creature_type: s.target_creature_type,
                requires_spell_focus: s.requires_spell_focus,
                facing_caster_flags: s.facing_caster_flags,
                caster_aura_state: s.caster_aura_state,
                target_aura_state: s.target_aura_state,
                exclude_caster_aura_state: s.exclude_caster_aura_state,
                exclude_target_aura_state: s.exclude_target_aura_state,
                caster_aura_spell: s.caster_aura_spell,
                target_aura_spell: s.target_aura_spell,
                exclude_caster_aura_spell: s.exclude_caster_aura_spell,
                exclude_target_aura_spell: s.exclude_target_aura_spell,
                casting_time_index: s.casting_time_index,
                recovery_time: s.recovery_time,
                category_recovery_time: s.category_recovery_time,
                interrupt_flags: s.interrupt_flags,
                aura_interrupt_flags: s.aura_interrupt_flags,
                channel_interrupt_flags: s.channel_interrupt_flags,
                proc_type_mask: s.proc_type_mask,
                proc_chance: s.proc_chance,
                proc_charges: s.proc_charges,
                max_level: s.max_level,
                base_level: s.base_level,
                spell_level: s.spell_level,
                duration_index: s.duration_index,
                power_type: s.power_type,
                mana_cost: s.mana_cost,
                mana_cost_per_level: s.mana_cost_per_level,
                mana_per_second: s.mana_per_second,
                mana_per_second_per_level: s.mana_per_second_per_level,
                range_index: s.range_index,
                speed: s.speed,
                modal_next_spell: s.modal_next_spell,
                cumulative_aura: s.cumulative_aura,
                totem: s.totem,
                reagent: s.reagent,
                reagent_count: s.reagent_count,
                equipped_item_class: s.equipped_item_class,
                equipped_item_subclass: s.equipped_item_subclass,
                equipped_item_inv_types: s.equipped_item_inv_types,
                effect: s.effect,
                effect_die_sides: s.effect_die_sides,
                effect_real_points_per_level: s.effect_real_points_per_level,
                effect_base_points: s.effect_base_points,
                effect_mechanic: s.effect_mechanic,
                implicit_target_a: s.implicit_target_a,
                implicit_target_b: s.implicit_target_b,
                effect_radius_index: s.effect_radius_index,
                effect_aura: s.effect_aura,
                effect_aura_period: s.effect_aura_period,
                effect_amplitude: s.effect_amplitude,
                effect_chain_targets: s.effect_chain_targets,
                effect_item_type: s.effect_item_type,
                effect_misc_value: s.effect_misc_value,
                effect_misc_value_b: s.effect_misc_value_b,
                effect_trigger_spell: s.effect_trigger_spell,
                effect_points_per_combo: s.effect_points_per_combo,
                effect_spell_class_mask_a: s.effect_spell_class_mask_a,
                effect_spell_class_mask_b: s.effect_spell_class_mask_b,
                effect_spell_class_mask_c: s.effect_spell_class_mask_c,
                spell_visual_id: s.spell_visual_id,
                spell_icon_id: s.spell_icon_id,
                active_icon_id: s.active_icon_id,
                spell_priority: s.spell_priority,
                name_lang: s.name_lang.to_string(),
                name_subtext_lang: s.name_subtext_lang.to_string(),
                description_lang: s.description_lang.to_string(),
                aura_description_lang: s.aura_description_lang.to_string(),
                mana_cost_pct: s.mana_cost_pct,
                start_recovery_category: s.start_recovery_category,
                start_recovery_time: s.start_recovery_time,
                max_target_level: s.max_target_level,
                spell_class_set: s.spell_class_set,
                spell_class_mask: s.spell_class_mask,
                max_targets: s.max_targets,
                defense_type: s.defense_type,
                prevention_type: s.prevention_type,
                stance_bar_order: s.stance_bar_order,
                effect_chain_amplitude: s.effect_chain_amplitude,
                min_faction_id: s.min_faction_id,
                min_reputation: s.min_reputation,
                required_aura_vision: s.required_aura_vision,
                required_totem_category_id: s.required_totem_category_id,
                required_areas_id: s.required_areas_id,
                school_mask: s.school_mask,
                rune_cost_id: s.rune_cost_id,
                spell_missile_id: s.spell_missile_id,
                power_display_id: s.power_display_id,
                effect_bonus_coefficient: s.effect_bonus_coefficient,
                description_variables_id: s.description_variables_id,
                difficulty: s.difficulty,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellKey {
    pub id: i32
}

impl SpellKey {
    pub const fn new(id: i32) -> Self {
        Self { id }
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

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum EffectAura {
    None,
    BindSight,
    ModPossess,
    PeriodicDamage,
    Dummy,
    ModConfuse,
    ModCharm,
    ModFear,
    PeriodicHeal,
    ModAttackspeed,
    ModThreat,
    ModTaunt,
    ModStun,
    ModDamageDone,
    ModDamageTaken,
    DamageShield,
    ModStealth,
    ModStealthDetect,
    ModInvisibility,
    ModInvisibilityDetect,
    ObsModHealth,
    ObsModPower,
    ModResistance,
    PeriodicTriggerSpell,
    PeriodicEnergize,
    ModPacify,
    ModRoot,
    ModSilence,
    ReflectSpells,
    ModStat,
    ModSkill,
    ModIncreaseSpeed,
    ModIncreaseMountedSpeed,
    ModDecreaseSpeed,
    ModIncreaseHealth,
    ModIncreaseEnergy,
    ModShapeshift,
    EffectImmunity,
    StateImmunity,
    SchoolImmunity,
    DamageImmunity,
    DispelImmunity,
    ProcTriggerSpell,
    ProcTriggerDamage,
    TrackCreatures,
    TrackResources,
    Unknown46,
    ModParryPercent,
    PeriodicTriggerSpellFromClient,
    ModDodgePercent,
    ModCriticalHealingAmount,
    ModBlockPercent,
    ModWeaponCritPercent,
    PeriodicLeech,
    ModHitChance,
    ModSpellHitChance,
    Transform,
    ModSpellCritChance,
    ModIncreaseSwimSpeed,
    ModDamageDoneCreature,
    ModPacifySilence,
    ModScale,
    PeriodicHealthFunnel,
    Unknown63,
    PeriodicManaLeech,
    ModCastingSpeedNotStack,
    FeignDeath,
    ModDisarm,
    ModStalked,
    SchoolAbsorb,
    ExtraAttacks,
    ModSpellCritChanceSchool,
    ModPowerCostSchoolPct,
    ModPowerCostSchool,
    ReflectSpellsSchool,
    ModLanguage,
    FarSight,
    MechanicImmunity,
    Mounted,
    ModDamagePercentDone,
    ModPercentStat,
    SplitDamagePct,
    WaterBreathing,
    ModBaseResistance,
    ModRegen,
    ModPowerRegen,
    ChannelDeathItem,
    ModDamagePercentTaken,
    ModHealthRegenPercent,
    PeriodicDamagePercent,
    Unknown90,
    ModDetectRange,
    PreventsFleeing,
    ModUnattackable,
    InterruptRegen,
    Ghost,
    SpellMagnet,
    ManaShield,
    ModSkillTalent,
    ModAttackPower,
    AurasVisible,
    ModResistancePct,
    ModMeleeAttackPowerVersus,
    ModTotalThreat,
    WaterWalk,
    FeatherFall,
    Hover,
    AddFlatModifier,
    AddPctModifier,
    AddTargetTrigger,
    ModPowerRegenPercent,
    AddCasterHitTrigger,
    OverrideClassScripts,
    ModRangedDamageTaken,
    ModRangedDamageTakenPct,
    ModHealing,
    ModRegenDuringCombat,
    ModMechanicResistance,
    ModHealingPct,
    Unknown119,
    Untrackable,
    Empathy,
    ModOffhandDamagePct,
    ModTargetResistance,
    ModRangedAttackPower,
    ModMeleeDamageTaken,
    ModMeleeDamageTakenPct,
    RangedAttackPowerAttackerBonus,
    ModPossessPet,
    ModSpeedAlways,
    ModMountedSpeedAlways,
    ModRangedAttackPowerVersus,
    ModIncreaseEnergyPercent,
    ModIncreaseHealthPercent,
    ModManaRegenInterrupt,
    ModHealingDone,
    ModHealingDonePercent,
    ModTotalStatPercentage,
    ModMeleeHaste,
    ForceReaction,
    ModRangedHaste,
    ModRangedAmmoHaste,
    ModBaseResistancePct,
    ModResistanceExclusive,
    SafeFall,
    ModPetTalentPoints,
    AllowTamePetType,
    MechanicImmunityMask,
    RetainComboPoints,
    ReducePushback,
    ModShieldBlockvaluePct,
    TrackStealthed,
    ModDetectedRange,
    SplitDamageFlat,
    ModStealthLevel,
    ModWaterBreathing,
    ModReputationGain,
    PetDamageMulti,
    ModShieldBlockvalue,
    NoPvpCredit,
    ModAoeAvoidance,
    ModHealthRegenInCombat,
    PowerBurn,
    ModCritDamageBonus,
    Unknown164,
    MeleeAttackPowerAttackerBonus,
    ModAttackPowerPct,
    ModRangedAttackPowerPct,
    ModDamageDoneVersus,
    ModCritPercentVersus,
    DetectAmore,
    ModSpeedNotStack,
    ModMountedSpeedNotStack,
    Unknown173,
    ModSpellDamageOfStatPercent,
    ModSpellHealingOfStatPercent,
    SpiritOfRedemption,
    AoeCharm,
    ModDebuffResistance,
    ModAttackerSpellCritChance,
    ModFlatSpellDamageVersus,
    Unknown181,
    ModResistanceOfStatPercent,
    ModCriticalThreat,
    ModAttackerMeleeHitChance,
    ModAttackerRangedHitChance,
    ModAttackerSpellHitChance,
    ModAttackerMeleeCritChance,
    ModAttackerRangedCritChance,
    ModRating,
    ModFactionReputationGain,
    UseNormalMovementSpeed,
    ModMeleeRangedHaste,
    MeleeSlow,
    ModTargetAbsorbSchool,
    ModTargetAbilityAbsorbSchool,
    ModCooldown,
    ModAttackerSpellAndWeaponCritChance,
    Unknown198,
    ModIncreasesSpellPctToHit,
    ModXpPct,
    Fly,
    IgnoreCombatResult,
    ModAttackerMeleeCritDamage,
    ModAttackerRangedCritDamage,
    ModSchoolCritDmgTaken,
    ModIncreaseVehicleFlightSpeed,
    ModIncreaseMountedFlightSpeed,
    ModIncreaseFlightSpeed,
    ModMountedFlightSpeedAlways,
    ModVehicleSpeedAlways,
    ModFlightSpeedNotStack,
    ModRangedAttackPowerOfStatPercent,
    ModRageFromDamageDealt,
    Unknown214,
    ArenaPreparation,
    HasteSpells,
    ModMeleeHaste2,
    HasteRanged,
    ModManaRegenFromStat,
    ModRatingFromStat,
    ModDetaunt,
    Unknown222,
    RaidProcFromCharge,
    Unknown224,
    RaidProcFromChargeWithValue,
    PeriodicDummy,
    PeriodicTriggerSpellWithValue,
    DetectStealth,
    ModAoeDamageAvoidance,
    Unknown230,
    ProcTriggerSpellWithValue,
    MechanicDurationMod,
    ChangeModelForAllHumanoids,
    MechanicDurationModNotStack,
    ModDispelResist,
    ControlVehicle,
    ModSpellDamageOfAttackPower,
    ModSpellHealingOfAttackPower,
    ModScale2,
    ModExpertise,
    ForceMoveForward,
    ModSpellDamageFromHealing,
    ModFaction,
    ComprehendLanguage,
    ModAuraDurationByDispel,
    ModAuraDurationByDispelNotStack,
    CloneCaster,
    ModCombatResultChance,
    ConvertRune,
    ModIncreaseHealth2,
    ModEnemyDodge,
    ModSpeedSlowAll,
    ModBlockCritChance,
    ModDisarmOffhand,
    ModMechanicDamageTakenPercent,
    NoReagentUse,
    ModTargetResistBySpellClass,
    Unknown258,
    ModHotPct,
    ScreenEffect,
    Phase,
    AbilityIgnoreAurastate,
    AllowOnlyAbility,
    Unknown264,
    Unknown265,
    Unknown266,
    ModImmuneAuraApplySchool,
    ModAttackPowerOfStatPercent,
    ModIgnoreTargetResist,
    ModAbilityIgnoreTargetResist,
    ModDamageFromCaster,
    IgnoreMeleeReset,
    XRay,
    AbilityConsumeNoAmmo,
    ModIgnoreShapeshift,
    ModDamageDoneForMechanic,
    ModMaxAffectedTargets,
    ModDisarmRanged,
    InitializeImages,
    ModArmorPenetrationPct,
    ModHonorGainPct,
    ModBaseHealthPct,
    ModHealingReceived,
    Linked,
    ModAttackPowerOfArmor,
    AbilityPeriodicCrit,
    DeflectSpells,
    IgnoreHitDirection,
    PreventDurabilityLoss,
    ModCritPct,
    ModXpQuestPct,
    OpenStable,
    OverrideSpells,
    PreventRegeneratePower,
    Unknown295,
    SetVehicleId,
    BlockSpellFamily,
    Strangulate,
    Unknown299,
    ShareDamagePct,
    SchoolHealAbsorb,
    Unknown302,
    ModDamageDoneVersusAurastate,
    ModFakeInebriate,
    ModMinimumSpeed,
    Unknown306,
    HealAbsorbTest,
    ModCritChanceForCaster,
    Unknown309,
    ModCreatureAoeDamageAvoidance,
    Unknown311,
    Unknown312,
    Unknown313,
    PreventResurrection,
    UnderwaterWalking,
    PeriodicHaste,
}

impl EffectAura {
    const fn from_value(value: i32) -> Option<Self> {
        Some(match value {
            0 => Self::None,
            1 => Self::BindSight,
            2 => Self::ModPossess,
            3 => Self::PeriodicDamage,
            4 => Self::Dummy,
            5 => Self::ModConfuse,
            6 => Self::ModCharm,
            7 => Self::ModFear,
            8 => Self::PeriodicHeal,
            9 => Self::ModAttackspeed,
            10 => Self::ModThreat,
            11 => Self::ModTaunt,
            12 => Self::ModStun,
            13 => Self::ModDamageDone,
            14 => Self::ModDamageTaken,
            15 => Self::DamageShield,
            16 => Self::ModStealth,
            17 => Self::ModStealthDetect,
            18 => Self::ModInvisibility,
            19 => Self::ModInvisibilityDetect,
            20 => Self::ObsModHealth,
            21 => Self::ObsModPower,
            22 => Self::ModResistance,
            23 => Self::PeriodicTriggerSpell,
            24 => Self::PeriodicEnergize,
            25 => Self::ModPacify,
            26 => Self::ModRoot,
            27 => Self::ModSilence,
            28 => Self::ReflectSpells,
            29 => Self::ModStat,
            30 => Self::ModSkill,
            31 => Self::ModIncreaseSpeed,
            32 => Self::ModIncreaseMountedSpeed,
            33 => Self::ModDecreaseSpeed,
            34 => Self::ModIncreaseHealth,
            35 => Self::ModIncreaseEnergy,
            36 => Self::ModShapeshift,
            37 => Self::EffectImmunity,
            38 => Self::StateImmunity,
            39 => Self::SchoolImmunity,
            40 => Self::DamageImmunity,
            41 => Self::DispelImmunity,
            42 => Self::ProcTriggerSpell,
            43 => Self::ProcTriggerDamage,
            44 => Self::TrackCreatures,
            45 => Self::TrackResources,
            46 => Self::Unknown46,
            47 => Self::ModParryPercent,
            48 => Self::PeriodicTriggerSpellFromClient,
            49 => Self::ModDodgePercent,
            50 => Self::ModCriticalHealingAmount,
            51 => Self::ModBlockPercent,
            52 => Self::ModWeaponCritPercent,
            53 => Self::PeriodicLeech,
            54 => Self::ModHitChance,
            55 => Self::ModSpellHitChance,
            56 => Self::Transform,
            57 => Self::ModSpellCritChance,
            58 => Self::ModIncreaseSwimSpeed,
            59 => Self::ModDamageDoneCreature,
            60 => Self::ModPacifySilence,
            61 => Self::ModScale,
            62 => Self::PeriodicHealthFunnel,
            63 => Self::Unknown63,
            64 => Self::PeriodicManaLeech,
            65 => Self::ModCastingSpeedNotStack,
            66 => Self::FeignDeath,
            67 => Self::ModDisarm,
            68 => Self::ModStalked,
            69 => Self::SchoolAbsorb,
            70 => Self::ExtraAttacks,
            71 => Self::ModSpellCritChanceSchool,
            72 => Self::ModPowerCostSchoolPct,
            73 => Self::ModPowerCostSchool,
            74 => Self::ReflectSpellsSchool,
            75 => Self::ModLanguage,
            76 => Self::FarSight,
            77 => Self::MechanicImmunity,
            78 => Self::Mounted,
            79 => Self::ModDamagePercentDone,
            80 => Self::ModPercentStat,
            81 => Self::SplitDamagePct,
            82 => Self::WaterBreathing,
            83 => Self::ModBaseResistance,
            84 => Self::ModRegen,
            85 => Self::ModPowerRegen,
            86 => Self::ChannelDeathItem,
            87 => Self::ModDamagePercentTaken,
            88 => Self::ModHealthRegenPercent,
            89 => Self::PeriodicDamagePercent,
            90 => Self::Unknown90,
            91 => Self::ModDetectRange,
            92 => Self::PreventsFleeing,
            93 => Self::ModUnattackable,
            94 => Self::InterruptRegen,
            95 => Self::Ghost,
            96 => Self::SpellMagnet,
            97 => Self::ManaShield,
            98 => Self::ModSkillTalent,
            99 => Self::ModAttackPower,
            100 => Self::AurasVisible,
            101 => Self::ModResistancePct,
            102 => Self::ModMeleeAttackPowerVersus,
            103 => Self::ModTotalThreat,
            104 => Self::WaterWalk,
            105 => Self::FeatherFall,
            106 => Self::Hover,
            107 => Self::AddFlatModifier,
            108 => Self::AddPctModifier,
            109 => Self::AddTargetTrigger,
            110 => Self::ModPowerRegenPercent,
            111 => Self::AddCasterHitTrigger,
            112 => Self::OverrideClassScripts,
            113 => Self::ModRangedDamageTaken,
            114 => Self::ModRangedDamageTakenPct,
            115 => Self::ModHealing,
            116 => Self::ModRegenDuringCombat,
            117 => Self::ModMechanicResistance,
            118 => Self::ModHealingPct,
            119 => Self::Unknown119,
            120 => Self::Untrackable,
            121 => Self::Empathy,
            122 => Self::ModOffhandDamagePct,
            123 => Self::ModTargetResistance,
            124 => Self::ModRangedAttackPower,
            125 => Self::ModMeleeDamageTaken,
            126 => Self::ModMeleeDamageTakenPct,
            127 => Self::RangedAttackPowerAttackerBonus,
            128 => Self::ModPossessPet,
            129 => Self::ModSpeedAlways,
            130 => Self::ModMountedSpeedAlways,
            131 => Self::ModRangedAttackPowerVersus,
            132 => Self::ModIncreaseEnergyPercent,
            133 => Self::ModIncreaseHealthPercent,
            134 => Self::ModManaRegenInterrupt,
            135 => Self::ModHealingDone,
            136 => Self::ModHealingDonePercent,
            137 => Self::ModTotalStatPercentage,
            138 => Self::ModMeleeHaste,
            139 => Self::ForceReaction,
            140 => Self::ModRangedHaste,
            141 => Self::ModRangedAmmoHaste,
            142 => Self::ModBaseResistancePct,
            143 => Self::ModResistanceExclusive,
            144 => Self::SafeFall,
            145 => Self::ModPetTalentPoints,
            146 => Self::AllowTamePetType,
            147 => Self::MechanicImmunityMask,
            148 => Self::RetainComboPoints,
            149 => Self::ReducePushback,
            150 => Self::ModShieldBlockvaluePct,
            151 => Self::TrackStealthed,
            152 => Self::ModDetectedRange,
            153 => Self::SplitDamageFlat,
            154 => Self::ModStealthLevel,
            155 => Self::ModWaterBreathing,
            156 => Self::ModReputationGain,
            157 => Self::PetDamageMulti,
            158 => Self::ModShieldBlockvalue,
            159 => Self::NoPvpCredit,
            160 => Self::ModAoeAvoidance,
            161 => Self::ModHealthRegenInCombat,
            162 => Self::PowerBurn,
            163 => Self::ModCritDamageBonus,
            164 => Self::Unknown164,
            165 => Self::MeleeAttackPowerAttackerBonus,
            166 => Self::ModAttackPowerPct,
            167 => Self::ModRangedAttackPowerPct,
            168 => Self::ModDamageDoneVersus,
            169 => Self::ModCritPercentVersus,
            170 => Self::DetectAmore,
            171 => Self::ModSpeedNotStack,
            172 => Self::ModMountedSpeedNotStack,
            173 => Self::Unknown173,
            174 => Self::ModSpellDamageOfStatPercent,
            175 => Self::ModSpellHealingOfStatPercent,
            176 => Self::SpiritOfRedemption,
            177 => Self::AoeCharm,
            178 => Self::ModDebuffResistance,
            179 => Self::ModAttackerSpellCritChance,
            180 => Self::ModFlatSpellDamageVersus,
            181 => Self::Unknown181,
            182 => Self::ModResistanceOfStatPercent,
            183 => Self::ModCriticalThreat,
            184 => Self::ModAttackerMeleeHitChance,
            185 => Self::ModAttackerRangedHitChance,
            186 => Self::ModAttackerSpellHitChance,
            187 => Self::ModAttackerMeleeCritChance,
            188 => Self::ModAttackerRangedCritChance,
            189 => Self::ModRating,
            190 => Self::ModFactionReputationGain,
            191 => Self::UseNormalMovementSpeed,
            192 => Self::ModMeleeRangedHaste,
            193 => Self::MeleeSlow,
            194 => Self::ModTargetAbsorbSchool,
            195 => Self::ModTargetAbilityAbsorbSchool,
            196 => Self::ModCooldown,
            197 => Self::ModAttackerSpellAndWeaponCritChance,
            198 => Self::Unknown198,
            199 => Self::ModIncreasesSpellPctToHit,
            200 => Self::ModXpPct,
            201 => Self::Fly,
            202 => Self::IgnoreCombatResult,
            203 => Self::ModAttackerMeleeCritDamage,
            204 => Self::ModAttackerRangedCritDamage,
            205 => Self::ModSchoolCritDmgTaken,
            206 => Self::ModIncreaseVehicleFlightSpeed,
            207 => Self::ModIncreaseMountedFlightSpeed,
            208 => Self::ModIncreaseFlightSpeed,
            209 => Self::ModMountedFlightSpeedAlways,
            210 => Self::ModVehicleSpeedAlways,
            211 => Self::ModFlightSpeedNotStack,
            212 => Self::ModRangedAttackPowerOfStatPercent,
            213 => Self::ModRageFromDamageDealt,
            214 => Self::Unknown214,
            215 => Self::ArenaPreparation,
            216 => Self::HasteSpells,
            217 => Self::ModMeleeHaste2,
            218 => Self::HasteRanged,
            219 => Self::ModManaRegenFromStat,
            220 => Self::ModRatingFromStat,
            221 => Self::ModDetaunt,
            222 => Self::Unknown222,
            223 => Self::RaidProcFromCharge,
            224 => Self::Unknown224,
            225 => Self::RaidProcFromChargeWithValue,
            226 => Self::PeriodicDummy,
            227 => Self::PeriodicTriggerSpellWithValue,
            228 => Self::DetectStealth,
            229 => Self::ModAoeDamageAvoidance,
            230 => Self::Unknown230,
            231 => Self::ProcTriggerSpellWithValue,
            232 => Self::MechanicDurationMod,
            233 => Self::ChangeModelForAllHumanoids,
            234 => Self::MechanicDurationModNotStack,
            235 => Self::ModDispelResist,
            236 => Self::ControlVehicle,
            237 => Self::ModSpellDamageOfAttackPower,
            238 => Self::ModSpellHealingOfAttackPower,
            239 => Self::ModScale2,
            240 => Self::ModExpertise,
            241 => Self::ForceMoveForward,
            242 => Self::ModSpellDamageFromHealing,
            243 => Self::ModFaction,
            244 => Self::ComprehendLanguage,
            245 => Self::ModAuraDurationByDispel,
            246 => Self::ModAuraDurationByDispelNotStack,
            247 => Self::CloneCaster,
            248 => Self::ModCombatResultChance,
            249 => Self::ConvertRune,
            250 => Self::ModIncreaseHealth2,
            251 => Self::ModEnemyDodge,
            252 => Self::ModSpeedSlowAll,
            253 => Self::ModBlockCritChance,
            254 => Self::ModDisarmOffhand,
            255 => Self::ModMechanicDamageTakenPercent,
            256 => Self::NoReagentUse,
            257 => Self::ModTargetResistBySpellClass,
            258 => Self::Unknown258,
            259 => Self::ModHotPct,
            260 => Self::ScreenEffect,
            261 => Self::Phase,
            262 => Self::AbilityIgnoreAurastate,
            263 => Self::AllowOnlyAbility,
            264 => Self::Unknown264,
            265 => Self::Unknown265,
            266 => Self::Unknown266,
            267 => Self::ModImmuneAuraApplySchool,
            268 => Self::ModAttackPowerOfStatPercent,
            269 => Self::ModIgnoreTargetResist,
            270 => Self::ModAbilityIgnoreTargetResist,
            271 => Self::ModDamageFromCaster,
            272 => Self::IgnoreMeleeReset,
            273 => Self::XRay,
            274 => Self::AbilityConsumeNoAmmo,
            275 => Self::ModIgnoreShapeshift,
            276 => Self::ModDamageDoneForMechanic,
            277 => Self::ModMaxAffectedTargets,
            278 => Self::ModDisarmRanged,
            279 => Self::InitializeImages,
            280 => Self::ModArmorPenetrationPct,
            281 => Self::ModHonorGainPct,
            282 => Self::ModBaseHealthPct,
            283 => Self::ModHealingReceived,
            284 => Self::Linked,
            285 => Self::ModAttackPowerOfArmor,
            286 => Self::AbilityPeriodicCrit,
            287 => Self::DeflectSpells,
            288 => Self::IgnoreHitDirection,
            289 => Self::PreventDurabilityLoss,
            290 => Self::ModCritPct,
            291 => Self::ModXpQuestPct,
            292 => Self::OpenStable,
            293 => Self::OverrideSpells,
            294 => Self::PreventRegeneratePower,
            295 => Self::Unknown295,
            296 => Self::SetVehicleId,
            297 => Self::BlockSpellFamily,
            298 => Self::Strangulate,
            299 => Self::Unknown299,
            300 => Self::ShareDamagePct,
            301 => Self::SchoolHealAbsorb,
            302 => Self::Unknown302,
            303 => Self::ModDamageDoneVersusAurastate,
            304 => Self::ModFakeInebriate,
            305 => Self::ModMinimumSpeed,
            306 => Self::Unknown306,
            307 => Self::HealAbsorbTest,
            308 => Self::ModCritChanceForCaster,
            309 => Self::Unknown309,
            310 => Self::ModCreatureAoeDamageAvoidance,
            311 => Self::Unknown311,
            312 => Self::Unknown312,
            313 => Self::Unknown313,
            314 => Self::PreventResurrection,
            315 => Self::UnderwaterWalking,
            316 => Self::PeriodicHaste,
            _ => return None,
        })
    }
}

impl TryFrom<i32> for EffectAura {
    type Error = crate::InvalidEnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::from_value(value).ok_or(crate::InvalidEnumError::new("EffectAura", value as i64))
    }

}

impl EffectAura {
    pub const fn as_int(&self) -> i32 {
        match self {
            Self::None => 0,
            Self::BindSight => 1,
            Self::ModPossess => 2,
            Self::PeriodicDamage => 3,
            Self::Dummy => 4,
            Self::ModConfuse => 5,
            Self::ModCharm => 6,
            Self::ModFear => 7,
            Self::PeriodicHeal => 8,
            Self::ModAttackspeed => 9,
            Self::ModThreat => 10,
            Self::ModTaunt => 11,
            Self::ModStun => 12,
            Self::ModDamageDone => 13,
            Self::ModDamageTaken => 14,
            Self::DamageShield => 15,
            Self::ModStealth => 16,
            Self::ModStealthDetect => 17,
            Self::ModInvisibility => 18,
            Self::ModInvisibilityDetect => 19,
            Self::ObsModHealth => 20,
            Self::ObsModPower => 21,
            Self::ModResistance => 22,
            Self::PeriodicTriggerSpell => 23,
            Self::PeriodicEnergize => 24,
            Self::ModPacify => 25,
            Self::ModRoot => 26,
            Self::ModSilence => 27,
            Self::ReflectSpells => 28,
            Self::ModStat => 29,
            Self::ModSkill => 30,
            Self::ModIncreaseSpeed => 31,
            Self::ModIncreaseMountedSpeed => 32,
            Self::ModDecreaseSpeed => 33,
            Self::ModIncreaseHealth => 34,
            Self::ModIncreaseEnergy => 35,
            Self::ModShapeshift => 36,
            Self::EffectImmunity => 37,
            Self::StateImmunity => 38,
            Self::SchoolImmunity => 39,
            Self::DamageImmunity => 40,
            Self::DispelImmunity => 41,
            Self::ProcTriggerSpell => 42,
            Self::ProcTriggerDamage => 43,
            Self::TrackCreatures => 44,
            Self::TrackResources => 45,
            Self::Unknown46 => 46,
            Self::ModParryPercent => 47,
            Self::PeriodicTriggerSpellFromClient => 48,
            Self::ModDodgePercent => 49,
            Self::ModCriticalHealingAmount => 50,
            Self::ModBlockPercent => 51,
            Self::ModWeaponCritPercent => 52,
            Self::PeriodicLeech => 53,
            Self::ModHitChance => 54,
            Self::ModSpellHitChance => 55,
            Self::Transform => 56,
            Self::ModSpellCritChance => 57,
            Self::ModIncreaseSwimSpeed => 58,
            Self::ModDamageDoneCreature => 59,
            Self::ModPacifySilence => 60,
            Self::ModScale => 61,
            Self::PeriodicHealthFunnel => 62,
            Self::Unknown63 => 63,
            Self::PeriodicManaLeech => 64,
            Self::ModCastingSpeedNotStack => 65,
            Self::FeignDeath => 66,
            Self::ModDisarm => 67,
            Self::ModStalked => 68,
            Self::SchoolAbsorb => 69,
            Self::ExtraAttacks => 70,
            Self::ModSpellCritChanceSchool => 71,
            Self::ModPowerCostSchoolPct => 72,
            Self::ModPowerCostSchool => 73,
            Self::ReflectSpellsSchool => 74,
            Self::ModLanguage => 75,
            Self::FarSight => 76,
            Self::MechanicImmunity => 77,
            Self::Mounted => 78,
            Self::ModDamagePercentDone => 79,
            Self::ModPercentStat => 80,
            Self::SplitDamagePct => 81,
            Self::WaterBreathing => 82,
            Self::ModBaseResistance => 83,
            Self::ModRegen => 84,
            Self::ModPowerRegen => 85,
            Self::ChannelDeathItem => 86,
            Self::ModDamagePercentTaken => 87,
            Self::ModHealthRegenPercent => 88,
            Self::PeriodicDamagePercent => 89,
            Self::Unknown90 => 90,
            Self::ModDetectRange => 91,
            Self::PreventsFleeing => 92,
            Self::ModUnattackable => 93,
            Self::InterruptRegen => 94,
            Self::Ghost => 95,
            Self::SpellMagnet => 96,
            Self::ManaShield => 97,
            Self::ModSkillTalent => 98,
            Self::ModAttackPower => 99,
            Self::AurasVisible => 100,
            Self::ModResistancePct => 101,
            Self::ModMeleeAttackPowerVersus => 102,
            Self::ModTotalThreat => 103,
            Self::WaterWalk => 104,
            Self::FeatherFall => 105,
            Self::Hover => 106,
            Self::AddFlatModifier => 107,
            Self::AddPctModifier => 108,
            Self::AddTargetTrigger => 109,
            Self::ModPowerRegenPercent => 110,
            Self::AddCasterHitTrigger => 111,
            Self::OverrideClassScripts => 112,
            Self::ModRangedDamageTaken => 113,
            Self::ModRangedDamageTakenPct => 114,
            Self::ModHealing => 115,
            Self::ModRegenDuringCombat => 116,
            Self::ModMechanicResistance => 117,
            Self::ModHealingPct => 118,
            Self::Unknown119 => 119,
            Self::Untrackable => 120,
            Self::Empathy => 121,
            Self::ModOffhandDamagePct => 122,
            Self::ModTargetResistance => 123,
            Self::ModRangedAttackPower => 124,
            Self::ModMeleeDamageTaken => 125,
            Self::ModMeleeDamageTakenPct => 126,
            Self::RangedAttackPowerAttackerBonus => 127,
            Self::ModPossessPet => 128,
            Self::ModSpeedAlways => 129,
            Self::ModMountedSpeedAlways => 130,
            Self::ModRangedAttackPowerVersus => 131,
            Self::ModIncreaseEnergyPercent => 132,
            Self::ModIncreaseHealthPercent => 133,
            Self::ModManaRegenInterrupt => 134,
            Self::ModHealingDone => 135,
            Self::ModHealingDonePercent => 136,
            Self::ModTotalStatPercentage => 137,
            Self::ModMeleeHaste => 138,
            Self::ForceReaction => 139,
            Self::ModRangedHaste => 140,
            Self::ModRangedAmmoHaste => 141,
            Self::ModBaseResistancePct => 142,
            Self::ModResistanceExclusive => 143,
            Self::SafeFall => 144,
            Self::ModPetTalentPoints => 145,
            Self::AllowTamePetType => 146,
            Self::MechanicImmunityMask => 147,
            Self::RetainComboPoints => 148,
            Self::ReducePushback => 149,
            Self::ModShieldBlockvaluePct => 150,
            Self::TrackStealthed => 151,
            Self::ModDetectedRange => 152,
            Self::SplitDamageFlat => 153,
            Self::ModStealthLevel => 154,
            Self::ModWaterBreathing => 155,
            Self::ModReputationGain => 156,
            Self::PetDamageMulti => 157,
            Self::ModShieldBlockvalue => 158,
            Self::NoPvpCredit => 159,
            Self::ModAoeAvoidance => 160,
            Self::ModHealthRegenInCombat => 161,
            Self::PowerBurn => 162,
            Self::ModCritDamageBonus => 163,
            Self::Unknown164 => 164,
            Self::MeleeAttackPowerAttackerBonus => 165,
            Self::ModAttackPowerPct => 166,
            Self::ModRangedAttackPowerPct => 167,
            Self::ModDamageDoneVersus => 168,
            Self::ModCritPercentVersus => 169,
            Self::DetectAmore => 170,
            Self::ModSpeedNotStack => 171,
            Self::ModMountedSpeedNotStack => 172,
            Self::Unknown173 => 173,
            Self::ModSpellDamageOfStatPercent => 174,
            Self::ModSpellHealingOfStatPercent => 175,
            Self::SpiritOfRedemption => 176,
            Self::AoeCharm => 177,
            Self::ModDebuffResistance => 178,
            Self::ModAttackerSpellCritChance => 179,
            Self::ModFlatSpellDamageVersus => 180,
            Self::Unknown181 => 181,
            Self::ModResistanceOfStatPercent => 182,
            Self::ModCriticalThreat => 183,
            Self::ModAttackerMeleeHitChance => 184,
            Self::ModAttackerRangedHitChance => 185,
            Self::ModAttackerSpellHitChance => 186,
            Self::ModAttackerMeleeCritChance => 187,
            Self::ModAttackerRangedCritChance => 188,
            Self::ModRating => 189,
            Self::ModFactionReputationGain => 190,
            Self::UseNormalMovementSpeed => 191,
            Self::ModMeleeRangedHaste => 192,
            Self::MeleeSlow => 193,
            Self::ModTargetAbsorbSchool => 194,
            Self::ModTargetAbilityAbsorbSchool => 195,
            Self::ModCooldown => 196,
            Self::ModAttackerSpellAndWeaponCritChance => 197,
            Self::Unknown198 => 198,
            Self::ModIncreasesSpellPctToHit => 199,
            Self::ModXpPct => 200,
            Self::Fly => 201,
            Self::IgnoreCombatResult => 202,
            Self::ModAttackerMeleeCritDamage => 203,
            Self::ModAttackerRangedCritDamage => 204,
            Self::ModSchoolCritDmgTaken => 205,
            Self::ModIncreaseVehicleFlightSpeed => 206,
            Self::ModIncreaseMountedFlightSpeed => 207,
            Self::ModIncreaseFlightSpeed => 208,
            Self::ModMountedFlightSpeedAlways => 209,
            Self::ModVehicleSpeedAlways => 210,
            Self::ModFlightSpeedNotStack => 211,
            Self::ModRangedAttackPowerOfStatPercent => 212,
            Self::ModRageFromDamageDealt => 213,
            Self::Unknown214 => 214,
            Self::ArenaPreparation => 215,
            Self::HasteSpells => 216,
            Self::ModMeleeHaste2 => 217,
            Self::HasteRanged => 218,
            Self::ModManaRegenFromStat => 219,
            Self::ModRatingFromStat => 220,
            Self::ModDetaunt => 221,
            Self::Unknown222 => 222,
            Self::RaidProcFromCharge => 223,
            Self::Unknown224 => 224,
            Self::RaidProcFromChargeWithValue => 225,
            Self::PeriodicDummy => 226,
            Self::PeriodicTriggerSpellWithValue => 227,
            Self::DetectStealth => 228,
            Self::ModAoeDamageAvoidance => 229,
            Self::Unknown230 => 230,
            Self::ProcTriggerSpellWithValue => 231,
            Self::MechanicDurationMod => 232,
            Self::ChangeModelForAllHumanoids => 233,
            Self::MechanicDurationModNotStack => 234,
            Self::ModDispelResist => 235,
            Self::ControlVehicle => 236,
            Self::ModSpellDamageOfAttackPower => 237,
            Self::ModSpellHealingOfAttackPower => 238,
            Self::ModScale2 => 239,
            Self::ModExpertise => 240,
            Self::ForceMoveForward => 241,
            Self::ModSpellDamageFromHealing => 242,
            Self::ModFaction => 243,
            Self::ComprehendLanguage => 244,
            Self::ModAuraDurationByDispel => 245,
            Self::ModAuraDurationByDispelNotStack => 246,
            Self::CloneCaster => 247,
            Self::ModCombatResultChance => 248,
            Self::ConvertRune => 249,
            Self::ModIncreaseHealth2 => 250,
            Self::ModEnemyDodge => 251,
            Self::ModSpeedSlowAll => 252,
            Self::ModBlockCritChance => 253,
            Self::ModDisarmOffhand => 254,
            Self::ModMechanicDamageTakenPercent => 255,
            Self::NoReagentUse => 256,
            Self::ModTargetResistBySpellClass => 257,
            Self::Unknown258 => 258,
            Self::ModHotPct => 259,
            Self::ScreenEffect => 260,
            Self::Phase => 261,
            Self::AbilityIgnoreAurastate => 262,
            Self::AllowOnlyAbility => 263,
            Self::Unknown264 => 264,
            Self::Unknown265 => 265,
            Self::Unknown266 => 266,
            Self::ModImmuneAuraApplySchool => 267,
            Self::ModAttackPowerOfStatPercent => 268,
            Self::ModIgnoreTargetResist => 269,
            Self::ModAbilityIgnoreTargetResist => 270,
            Self::ModDamageFromCaster => 271,
            Self::IgnoreMeleeReset => 272,
            Self::XRay => 273,
            Self::AbilityConsumeNoAmmo => 274,
            Self::ModIgnoreShapeshift => 275,
            Self::ModDamageDoneForMechanic => 276,
            Self::ModMaxAffectedTargets => 277,
            Self::ModDisarmRanged => 278,
            Self::InitializeImages => 279,
            Self::ModArmorPenetrationPct => 280,
            Self::ModHonorGainPct => 281,
            Self::ModBaseHealthPct => 282,
            Self::ModHealingReceived => 283,
            Self::Linked => 284,
            Self::ModAttackPowerOfArmor => 285,
            Self::AbilityPeriodicCrit => 286,
            Self::DeflectSpells => 287,
            Self::IgnoreHitDirection => 288,
            Self::PreventDurabilityLoss => 289,
            Self::ModCritPct => 290,
            Self::ModXpQuestPct => 291,
            Self::OpenStable => 292,
            Self::OverrideSpells => 293,
            Self::PreventRegeneratePower => 294,
            Self::Unknown295 => 295,
            Self::SetVehicleId => 296,
            Self::BlockSpellFamily => 297,
            Self::Strangulate => 298,
            Self::Unknown299 => 299,
            Self::ShareDamagePct => 300,
            Self::SchoolHealAbsorb => 301,
            Self::Unknown302 => 302,
            Self::ModDamageDoneVersusAurastate => 303,
            Self::ModFakeInebriate => 304,
            Self::ModMinimumSpeed => 305,
            Self::Unknown306 => 306,
            Self::HealAbsorbTest => 307,
            Self::ModCritChanceForCaster => 308,
            Self::Unknown309 => 309,
            Self::ModCreatureAoeDamageAvoidance => 310,
            Self::Unknown311 => 311,
            Self::Unknown312 => 312,
            Self::Unknown313 => 313,
            Self::PreventResurrection => 314,
            Self::UnderwaterWalking => 315,
            Self::PeriodicHaste => 316,
        }

    }

}

impl Default for EffectAura {
    fn default() -> Self {
        Self::None
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellRow {
    pub id: SpellKey,
    pub category: SpellCategoryKey,
    pub dispel_type: SpellDispelTypeKey,
    pub mechanic: SpellMechanicKey,
    pub attributes: i32,
    pub attributes_ex: i32,
    pub attributes_ex_b: i32,
    pub attributes_ex_c: i32,
    pub attributes_ex_d: i32,
    pub attributes_ex_e: i32,
    pub attributes_ex_f: i32,
    pub attributes_ex_g: i32,
    pub shapeshift_mask: [i32; 2],
    pub shapeshift_exclude: [i32; 2],
    pub targets: i32,
    pub target_creature_type: i32,
    pub requires_spell_focus: SpellFocusObjectKey,
    pub facing_caster_flags: i32,
    pub caster_aura_state: i32,
    pub target_aura_state: i32,
    pub exclude_caster_aura_state: i32,
    pub exclude_target_aura_state: i32,
    pub caster_aura_spell: i32,
    pub target_aura_spell: i32,
    pub exclude_caster_aura_spell: i32,
    pub exclude_target_aura_spell: i32,
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
    pub effect_real_points_per_level: [f32; 3],
    pub effect_base_points: [i32; 3],
    pub effect_mechanic: [i32; 3],
    pub implicit_target_a: [i32; 3],
    pub implicit_target_b: [i32; 3],
    pub effect_radius_index: [i32; 3],
    pub effect_aura: [EffectAura; 3],
    pub effect_aura_period: [i32; 3],
    pub effect_amplitude: [f32; 3],
    pub effect_chain_targets: [i32; 3],
    pub effect_item_type: [i32; 3],
    pub effect_misc_value: [i32; 3],
    pub effect_misc_value_b: [i32; 3],
    pub effect_trigger_spell: [i32; 3],
    pub effect_points_per_combo: [f32; 3],
    pub effect_spell_class_mask_a: [i32; 3],
    pub effect_spell_class_mask_b: [i32; 3],
    pub effect_spell_class_mask_c: [i32; 3],
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
    pub spell_class_mask: [i32; 3],
    pub max_targets: i32,
    pub defense_type: i32,
    pub prevention_type: i32,
    pub stance_bar_order: i32,
    pub effect_chain_amplitude: [f32; 3],
    pub min_faction_id: FactionKey,
    pub min_reputation: i32,
    pub required_aura_vision: i32,
    pub required_totem_category_id: [i32; 2],
    pub required_areas_id: AreaGroupKey,
    pub school_mask: i32,
    pub rune_cost_id: SpellRuneCostKey,
    pub spell_missile_id: SpellMissileKey,
    pub power_display_id: PowerDisplayKey,
    pub effect_bonus_coefficient: [f32; 3],
    pub description_variables_id: SpellDescriptionVariablesKey,
    pub difficulty: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSpellRow {
    pub id: SpellKey,
    pub category: SpellCategoryKey,
    pub dispel_type: SpellDispelTypeKey,
    pub mechanic: SpellMechanicKey,
    pub attributes: i32,
    pub attributes_ex: i32,
    pub attributes_ex_b: i32,
    pub attributes_ex_c: i32,
    pub attributes_ex_d: i32,
    pub attributes_ex_e: i32,
    pub attributes_ex_f: i32,
    pub attributes_ex_g: i32,
    pub shapeshift_mask: [i32; 2],
    pub shapeshift_exclude: [i32; 2],
    pub targets: i32,
    pub target_creature_type: i32,
    pub requires_spell_focus: SpellFocusObjectKey,
    pub facing_caster_flags: i32,
    pub caster_aura_state: i32,
    pub target_aura_state: i32,
    pub exclude_caster_aura_state: i32,
    pub exclude_target_aura_state: i32,
    pub caster_aura_spell: i32,
    pub target_aura_spell: i32,
    pub exclude_caster_aura_spell: i32,
    pub exclude_target_aura_spell: i32,
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
    pub effect_real_points_per_level: [f32; 3],
    pub effect_base_points: [i32; 3],
    pub effect_mechanic: [i32; 3],
    pub implicit_target_a: [i32; 3],
    pub implicit_target_b: [i32; 3],
    pub effect_radius_index: [i32; 3],
    pub effect_aura: [EffectAura; 3],
    pub effect_aura_period: [i32; 3],
    pub effect_amplitude: [f32; 3],
    pub effect_chain_targets: [i32; 3],
    pub effect_item_type: [i32; 3],
    pub effect_misc_value: [i32; 3],
    pub effect_misc_value_b: [i32; 3],
    pub effect_trigger_spell: [i32; 3],
    pub effect_points_per_combo: [f32; 3],
    pub effect_spell_class_mask_a: [i32; 3],
    pub effect_spell_class_mask_b: [i32; 3],
    pub effect_spell_class_mask_c: [i32; 3],
    pub spell_visual_id: [i32; 2],
    pub spell_icon_id: SpellIconKey,
    pub active_icon_id: SpellIconKey,
    pub spell_priority: i32,
    pub name_lang: ConstExtendedLocalizedString,
    pub name_subtext_lang: ConstExtendedLocalizedString,
    pub description_lang: ConstExtendedLocalizedString,
    pub aura_description_lang: ConstExtendedLocalizedString,
    pub mana_cost_pct: i32,
    pub start_recovery_category: i32,
    pub start_recovery_time: i32,
    pub max_target_level: i32,
    pub spell_class_set: i32,
    pub spell_class_mask: [i32; 3],
    pub max_targets: i32,
    pub defense_type: i32,
    pub prevention_type: i32,
    pub stance_bar_order: i32,
    pub effect_chain_amplitude: [f32; 3],
    pub min_faction_id: FactionKey,
    pub min_reputation: i32,
    pub required_aura_vision: i32,
    pub required_totem_category_id: [i32; 2],
    pub required_areas_id: AreaGroupKey,
    pub school_mask: i32,
    pub rune_cost_id: SpellRuneCostKey,
    pub spell_missile_id: SpellMissileKey,
    pub power_display_id: PowerDisplayKey,
    pub effect_bonus_coefficient: [f32; 3],
    pub description_variables_id: SpellDescriptionVariablesKey,
    pub difficulty: i32,
}

