use crate::header::{HEADER_SIZE, DbcHeader};
use crate::header;
use crate::DbcTable;
use std::io::Write;
use crate::Indexable;
use crate::{ConstLocalizedString, LocalizedString};
use crate::vanilla_tables::chr_classes::*;
use crate::vanilla_tables::creature_type::*;
use crate::vanilla_tables::item_class::*;
use crate::vanilla_tables::resistances::*;
use crate::vanilla_tables::spell_cast_times::*;
use crate::vanilla_tables::spell_category::*;
use crate::vanilla_tables::spell_dispel_type::*;
use crate::vanilla_tables::spell_duration::*;
use crate::vanilla_tables::spell_focus_object::*;
use crate::vanilla_tables::spell_icon::*;
use crate::vanilla_tables::spell_mechanic::*;
use crate::vanilla_tables::spell_range::*;
use crate::vanilla_tables::spell_shapeshift_form::*;

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

        if header.record_size != 692 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::RecordSize {
                    expected: 692,
                    actual: header.record_size,
                },
            ));
        }

        if header.field_count != 173 {
            return Err(crate::DbcError::InvalidHeader(
                crate::InvalidHeaderError::FieldCount {
                    expected: 173,
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

            // id: primary_key (Spell) uint32
            let id = SpellKey::new(crate::util::read_u32_le(chunk)?);

            // school: foreign_key (Resistances) uint32
            let school = ResistancesKey::new(crate::util::read_u32_le(chunk)?.into());

            // category: foreign_key (SpellCategory) uint32
            let category = SpellCategoryKey::new(crate::util::read_u32_le(chunk)?.into());

            // cast_ui: int32
            let cast_ui = crate::util::read_i32_le(chunk)?;

            // dispel_type: foreign_key (SpellDispelType) uint32
            let dispel_type = SpellDispelTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // mechanic: foreign_key (SpellMechanic) uint32
            let mechanic = SpellMechanicKey::new(crate::util::read_u32_le(chunk)?.into());

            // attributes: Attributes
            let attributes = Attributes::new(crate::util::read_u32_le(chunk)?);

            // attributes_ex1: AttributesEx1
            let attributes_ex1 = AttributesEx1::new(crate::util::read_u32_le(chunk)?);

            // attributes_ex2: AttributesEx2
            let attributes_ex2 = AttributesEx2::new(crate::util::read_u32_le(chunk)?);

            // attributes_ex3: AttributesEx3
            let attributes_ex3 = AttributesEx3::new(crate::util::read_u32_le(chunk)?);

            // attributes_ex4: AttributesEx4
            let attributes_ex4 = AttributesEx4::new(crate::util::read_u32_le(chunk)?);

            // shapeshift_mask: foreign_key (SpellShapeshiftForm) uint32
            let shapeshift_mask = SpellShapeshiftFormKey::new(crate::util::read_u32_le(chunk)?.into());

            // shapeshift_exclude: foreign_key (SpellShapeshiftForm) uint32
            let shapeshift_exclude = SpellShapeshiftFormKey::new(crate::util::read_u32_le(chunk)?.into());

            // targets: int32
            let targets = crate::util::read_i32_le(chunk)?;

            // target_creature_type: foreign_key (CreatureType) uint32
            let target_creature_type = CreatureTypeKey::new(crate::util::read_u32_le(chunk)?.into());

            // requires_spell_focus: foreign_key (SpellFocusObject) uint32
            let requires_spell_focus = SpellFocusObjectKey::new(crate::util::read_u32_le(chunk)?.into());

            // caster_aura_state: int32
            let caster_aura_state = crate::util::read_i32_le(chunk)?;

            // target_aura_state: int32
            let target_aura_state = crate::util::read_i32_le(chunk)?;

            // casting_time_index: foreign_key (SpellCastTimes) uint32
            let casting_time_index = SpellCastTimesKey::new(crate::util::read_u32_le(chunk)?.into());

            // recovery_time: int32
            let recovery_time = crate::util::read_i32_le(chunk)?;

            // category_recovery_time: int32
            let category_recovery_time = crate::util::read_i32_le(chunk)?;

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

            // duration: foreign_key (SpellDuration) uint32
            let duration = SpellDurationKey::new(crate::util::read_u32_le(chunk)?.into());

            // power_type: int32
            let power_type = crate::util::read_i32_le(chunk)?;

            // mana_cost: int32
            let mana_cost = crate::util::read_i32_le(chunk)?;

            // mana_cost_per_level: int32
            let mana_cost_per_level = crate::util::read_i32_le(chunk)?;

            // mana_cost_per_second: int32
            let mana_cost_per_second = crate::util::read_i32_le(chunk)?;

            // mana_cost_per_second_per_level: int32
            let mana_cost_per_second_per_level = crate::util::read_i32_le(chunk)?;

            // range: foreign_key (SpellRange) uint32
            let range = SpellRangeKey::new(crate::util::read_u32_le(chunk)?.into());

            // speed: float
            let speed = crate::util::read_f32_le(chunk)?;

            // modal_next_spell: foreign_key (Spell) uint32
            let modal_next_spell = SpellKey::new(crate::util::read_u32_le(chunk)?.into());

            // stack_amount: int32
            let stack_amount = crate::util::read_i32_le(chunk)?;

            // totem: int32[2]
            let totem = crate::util::read_array_i32::<2>(chunk)?;

            // reagent: int32[8]
            let reagent = crate::util::read_array_i32::<8>(chunk)?;

            // reagent_count: int32[8]
            let reagent_count = crate::util::read_array_i32::<8>(chunk)?;

            // equipped_item_class: foreign_key (ItemClass) uint32
            let equipped_item_class = ItemClassKey::new(crate::util::read_u32_le(chunk)?.into());

            // equipped_item_subclass: uint32
            let equipped_item_subclass = crate::util::read_u32_le(chunk)?;

            // equipped_item_inventory_type: int32
            let equipped_item_inventory_type = crate::util::read_i32_le(chunk)?;

            // effect: int32[3]
            let effect = crate::util::read_array_i32::<3>(chunk)?;

            // effect_die_sides: int32[3]
            let effect_die_sides = crate::util::read_array_i32::<3>(chunk)?;

            // effect_base_dice: int32[3]
            let effect_base_dice = crate::util::read_array_i32::<3>(chunk)?;

            // effect_dice_per_level: float[3]
            let effect_dice_per_level = crate::util::read_array_f32::<3>(chunk)?;

            // effect_real_points_per_level: float[3]
            let effect_real_points_per_level = crate::util::read_array_f32::<3>(chunk)?;

            // effect_base_points: int32[3]
            let effect_base_points = crate::util::read_array_i32::<3>(chunk)?;

            // effect_mechanic: uint32[3]
            let effect_mechanic = crate::util::read_array_u32::<3>(chunk)?;

            // implicit_target_a: int32[3]
            let implicit_target_a = crate::util::read_array_i32::<3>(chunk)?;

            // implicit_target_b: int32[3]
            let implicit_target_b = crate::util::read_array_i32::<3>(chunk)?;

            // effect_radius: uint32[3]
            let effect_radius = crate::util::read_array_u32::<3>(chunk)?;

            // effect_aura: EffectAura[3]
            let effect_aura = {
                let mut arr = [EffectAura::default(); 3];
                for i in arr.iter_mut() {
                    *i = EffectAura::try_from(crate::util::read_i32_le(chunk)?)?;
                }

                arr
            };

            // effect_amplitude: float[3]
            let effect_amplitude = crate::util::read_array_f32::<3>(chunk)?;

            // effect_multiple_values: float[3]
            let effect_multiple_values = crate::util::read_array_f32::<3>(chunk)?;

            // effect_chain_target: int32[3]
            let effect_chain_target = crate::util::read_array_i32::<3>(chunk)?;

            // effect_item_type: int32[3]
            let effect_item_type = crate::util::read_array_i32::<3>(chunk)?;

            // effect_misc_value: uint32[3]
            let effect_misc_value = crate::util::read_array_u32::<3>(chunk)?;

            // effect_trigger_spell: uint32[3]
            let effect_trigger_spell = crate::util::read_array_u32::<3>(chunk)?;

            // effect_points_per_combo: float[3]
            let effect_points_per_combo = crate::util::read_array_f32::<3>(chunk)?;

            // spell_visual: int32[2]
            let spell_visual = crate::util::read_array_i32::<2>(chunk)?;

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(crate::util::read_u32_le(chunk)?.into());

            // active_icon: int32
            let active_icon = crate::util::read_i32_le(chunk)?;

            // spell_priority: int32
            let spell_priority = crate::util::read_i32_le(chunk)?;

            // unknown_flag: int32
            let unknown_flag = crate::util::read_i32_le(chunk)?;

            // name: string_ref_loc
            let name = crate::util::read_localized_string(chunk, &string_block)?;

            // name_subtext: string_ref_loc
            let name_subtext = crate::util::read_localized_string(chunk, &string_block)?;

            // description: string_ref_loc
            let description = crate::util::read_localized_string(chunk, &string_block)?;

            // aura_description: string_ref_loc
            let aura_description = crate::util::read_localized_string(chunk, &string_block)?;

            // mana_cost_percent: int32
            let mana_cost_percent = crate::util::read_i32_le(chunk)?;

            // start_recovery_category: int32
            let start_recovery_category = crate::util::read_i32_le(chunk)?;

            // start_recovery_time: int32
            let start_recovery_time = crate::util::read_i32_le(chunk)?;

            // max_target_level: int32
            let max_target_level = crate::util::read_i32_le(chunk)?;

            // spell_class_set: foreign_key (ChrClasses) uint32
            let spell_class_set = ChrClassesKey::new(crate::util::read_u32_le(chunk)?.into());

            // spell_class_mask: int32[2]
            let spell_class_mask = crate::util::read_array_i32::<2>(chunk)?;

            // max_targets: int32
            let max_targets = crate::util::read_i32_le(chunk)?;

            // defence_type: int32
            let defence_type = crate::util::read_i32_le(chunk)?;

            // prevention_type: int32
            let prevention_type = crate::util::read_i32_le(chunk)?;

            // stance_bar_order: int32
            let stance_bar_order = crate::util::read_i32_le(chunk)?;

            // damage_multiplier: float[3]
            let damage_multiplier = crate::util::read_array_f32::<3>(chunk)?;

            // min_faction: int32
            let min_faction = crate::util::read_i32_le(chunk)?;

            // min_reputation: int32
            let min_reputation = crate::util::read_i32_le(chunk)?;

            // required_aura_vision: int32
            let required_aura_vision = crate::util::read_i32_le(chunk)?;


            rows.push(SpellRow {
                id,
                school,
                category,
                cast_ui,
                dispel_type,
                mechanic,
                attributes,
                attributes_ex1,
                attributes_ex2,
                attributes_ex3,
                attributes_ex4,
                shapeshift_mask,
                shapeshift_exclude,
                targets,
                target_creature_type,
                requires_spell_focus,
                caster_aura_state,
                target_aura_state,
                casting_time_index,
                recovery_time,
                category_recovery_time,
                aura_interrupt_flags,
                channel_interrupt_flags,
                proc_type_mask,
                proc_chance,
                proc_charges,
                max_level,
                base_level,
                spell_level,
                duration,
                power_type,
                mana_cost,
                mana_cost_per_level,
                mana_cost_per_second,
                mana_cost_per_second_per_level,
                range,
                speed,
                modal_next_spell,
                stack_amount,
                totem,
                reagent,
                reagent_count,
                equipped_item_class,
                equipped_item_subclass,
                equipped_item_inventory_type,
                effect,
                effect_die_sides,
                effect_base_dice,
                effect_dice_per_level,
                effect_real_points_per_level,
                effect_base_points,
                effect_mechanic,
                implicit_target_a,
                implicit_target_b,
                effect_radius,
                effect_aura,
                effect_amplitude,
                effect_multiple_values,
                effect_chain_target,
                effect_item_type,
                effect_misc_value,
                effect_trigger_spell,
                effect_points_per_combo,
                spell_visual,
                spell_icon,
                active_icon,
                spell_priority,
                unknown_flag,
                name,
                name_subtext,
                description,
                aura_description,
                mana_cost_percent,
                start_recovery_category,
                start_recovery_time,
                max_target_level,
                spell_class_set,
                spell_class_mask,
                max_targets,
                defence_type,
                prevention_type,
                stance_bar_order,
                damage_multiplier,
                min_faction,
                min_reputation,
                required_aura_vision,
            });
        }

        Ok(Spell { rows, })
    }

    fn write(&self, b: &mut impl Write) -> Result<(), std::io::Error> {
        let header = DbcHeader {
            record_count: self.rows.len() as u32,
            field_count: 173,
            record_size: 692,
            string_block_size: self.string_block_size(),
        };

        b.write_all(&header.write_header())?;

        let mut string_index = 1;
        for row in &self.rows {
            // id: primary_key (Spell) uint32
            b.write_all(&row.id.id.to_le_bytes())?;

            // school: foreign_key (Resistances) uint32
            b.write_all(&(row.school.id as u32).to_le_bytes())?;

            // category: foreign_key (SpellCategory) uint32
            b.write_all(&(row.category.id as u32).to_le_bytes())?;

            // cast_ui: int32
            b.write_all(&row.cast_ui.to_le_bytes())?;

            // dispel_type: foreign_key (SpellDispelType) uint32
            b.write_all(&(row.dispel_type.id as u32).to_le_bytes())?;

            // mechanic: foreign_key (SpellMechanic) uint32
            b.write_all(&(row.mechanic.id as u32).to_le_bytes())?;

            // attributes: Attributes
            b.write_all(&(row.attributes.as_int() as u32).to_le_bytes())?;

            // attributes_ex1: AttributesEx1
            b.write_all(&(row.attributes_ex1.as_int() as u32).to_le_bytes())?;

            // attributes_ex2: AttributesEx2
            b.write_all(&(row.attributes_ex2.as_int() as u32).to_le_bytes())?;

            // attributes_ex3: AttributesEx3
            b.write_all(&(row.attributes_ex3.as_int() as u32).to_le_bytes())?;

            // attributes_ex4: AttributesEx4
            b.write_all(&(row.attributes_ex4.as_int() as u32).to_le_bytes())?;

            // shapeshift_mask: foreign_key (SpellShapeshiftForm) uint32
            b.write_all(&(row.shapeshift_mask.id as u32).to_le_bytes())?;

            // shapeshift_exclude: foreign_key (SpellShapeshiftForm) uint32
            b.write_all(&(row.shapeshift_exclude.id as u32).to_le_bytes())?;

            // targets: int32
            b.write_all(&row.targets.to_le_bytes())?;

            // target_creature_type: foreign_key (CreatureType) uint32
            b.write_all(&(row.target_creature_type.id as u32).to_le_bytes())?;

            // requires_spell_focus: foreign_key (SpellFocusObject) uint32
            b.write_all(&(row.requires_spell_focus.id as u32).to_le_bytes())?;

            // caster_aura_state: int32
            b.write_all(&row.caster_aura_state.to_le_bytes())?;

            // target_aura_state: int32
            b.write_all(&row.target_aura_state.to_le_bytes())?;

            // casting_time_index: foreign_key (SpellCastTimes) uint32
            b.write_all(&(row.casting_time_index.id as u32).to_le_bytes())?;

            // recovery_time: int32
            b.write_all(&row.recovery_time.to_le_bytes())?;

            // category_recovery_time: int32
            b.write_all(&row.category_recovery_time.to_le_bytes())?;

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

            // duration: foreign_key (SpellDuration) uint32
            b.write_all(&(row.duration.id as u32).to_le_bytes())?;

            // power_type: int32
            b.write_all(&row.power_type.to_le_bytes())?;

            // mana_cost: int32
            b.write_all(&row.mana_cost.to_le_bytes())?;

            // mana_cost_per_level: int32
            b.write_all(&row.mana_cost_per_level.to_le_bytes())?;

            // mana_cost_per_second: int32
            b.write_all(&row.mana_cost_per_second.to_le_bytes())?;

            // mana_cost_per_second_per_level: int32
            b.write_all(&row.mana_cost_per_second_per_level.to_le_bytes())?;

            // range: foreign_key (SpellRange) uint32
            b.write_all(&(row.range.id as u32).to_le_bytes())?;

            // speed: float
            b.write_all(&row.speed.to_le_bytes())?;

            // modal_next_spell: foreign_key (Spell) uint32
            b.write_all(&(row.modal_next_spell.id as u32).to_le_bytes())?;

            // stack_amount: int32
            b.write_all(&row.stack_amount.to_le_bytes())?;

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


            // equipped_item_class: foreign_key (ItemClass) uint32
            b.write_all(&(row.equipped_item_class.id as u32).to_le_bytes())?;

            // equipped_item_subclass: uint32
            b.write_all(&row.equipped_item_subclass.to_le_bytes())?;

            // equipped_item_inventory_type: int32
            b.write_all(&row.equipped_item_inventory_type.to_le_bytes())?;

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


            // effect_dice_per_level: float[3]
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


            // effect_mechanic: uint32[3]
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


            // effect_radius: uint32[3]
            for i in row.effect_radius {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_aura: EffectAura[3]
            for i in row.effect_aura {
                b.write_all(&(i.as_int() as i32).to_le_bytes())?;
            }


            // effect_amplitude: float[3]
            for i in row.effect_amplitude {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_multiple_values: float[3]
            for i in row.effect_multiple_values {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_chain_target: int32[3]
            for i in row.effect_chain_target {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_item_type: int32[3]
            for i in row.effect_item_type {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_misc_value: uint32[3]
            for i in row.effect_misc_value {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_trigger_spell: uint32[3]
            for i in row.effect_trigger_spell {
                b.write_all(&i.to_le_bytes())?;
            }


            // effect_points_per_combo: float[3]
            for i in row.effect_points_per_combo {
                b.write_all(&i.to_le_bytes())?;
            }


            // spell_visual: int32[2]
            for i in row.spell_visual {
                b.write_all(&i.to_le_bytes())?;
            }


            // spell_icon: foreign_key (SpellIcon) uint32
            b.write_all(&(row.spell_icon.id as u32).to_le_bytes())?;

            // active_icon: int32
            b.write_all(&row.active_icon.to_le_bytes())?;

            // spell_priority: int32
            b.write_all(&row.spell_priority.to_le_bytes())?;

            // unknown_flag: int32
            b.write_all(&row.unknown_flag.to_le_bytes())?;

            // name: string_ref_loc
            b.write_all(&row.name.string_indices_as_array(&mut string_index))?;

            // name_subtext: string_ref_loc
            b.write_all(&row.name_subtext.string_indices_as_array(&mut string_index))?;

            // description: string_ref_loc
            b.write_all(&row.description.string_indices_as_array(&mut string_index))?;

            // aura_description: string_ref_loc
            b.write_all(&row.aura_description.string_indices_as_array(&mut string_index))?;

            // mana_cost_percent: int32
            b.write_all(&row.mana_cost_percent.to_le_bytes())?;

            // start_recovery_category: int32
            b.write_all(&row.start_recovery_category.to_le_bytes())?;

            // start_recovery_time: int32
            b.write_all(&row.start_recovery_time.to_le_bytes())?;

            // max_target_level: int32
            b.write_all(&row.max_target_level.to_le_bytes())?;

            // spell_class_set: foreign_key (ChrClasses) uint32
            b.write_all(&(row.spell_class_set.id as u32).to_le_bytes())?;

            // spell_class_mask: int32[2]
            for i in row.spell_class_mask {
                b.write_all(&i.to_le_bytes())?;
            }


            // max_targets: int32
            b.write_all(&row.max_targets.to_le_bytes())?;

            // defence_type: int32
            b.write_all(&row.defence_type.to_le_bytes())?;

            // prevention_type: int32
            b.write_all(&row.prevention_type.to_le_bytes())?;

            // stance_bar_order: int32
            b.write_all(&row.stance_bar_order.to_le_bytes())?;

            // damage_multiplier: float[3]
            for i in row.damage_multiplier {
                b.write_all(&i.to_le_bytes())?;
            }


            // min_faction: int32
            b.write_all(&row.min_faction.to_le_bytes())?;

            // min_reputation: int32
            b.write_all(&row.min_reputation.to_le_bytes())?;

            // required_aura_vision: int32
            b.write_all(&row.required_aura_vision.to_le_bytes())?;

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
            row.name.string_block_as_array(b)?;
            row.name_subtext.string_block_as_array(b)?;
            row.description.string_block_as_array(b)?;
            row.aura_description.string_block_as_array(b)?;
        }

        Ok(())
    }

    fn string_block_size(&self) -> u32 {
        let mut sum = 1;
        for row in &self.rows {
            sum += row.name.string_block_size();
            sum += row.name_subtext.string_block_size();
            sum += row.description.string_block_size();
            sum += row.aura_description.string_block_size();
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
        if header.record_size != 692 {
            panic!("invalid record size, expected 692")
        }

        if header.field_count != 173 {
            panic!("invalid field count, expected 173")
        }

        let string_block = HEADER_SIZE + (header.record_count * header.record_size) as usize;
        let string_block = crate::util::subslice(b, string_block..b.len());
        let mut b_offset = HEADER_SIZE;
        let mut rows = [
            ConstSpellRow {
                id: SpellKey::new(0),
                school: ResistancesKey::new(0),
                category: SpellCategoryKey::new(0),
                cast_ui: 0,
                dispel_type: SpellDispelTypeKey::new(0),
                mechanic: SpellMechanicKey::new(0),
                attributes: Attributes::new(0),
                attributes_ex1: AttributesEx1::new(0),
                attributes_ex2: AttributesEx2::new(0),
                attributes_ex3: AttributesEx3::new(0),
                attributes_ex4: AttributesEx4::new(0),
                shapeshift_mask: SpellShapeshiftFormKey::new(0),
                shapeshift_exclude: SpellShapeshiftFormKey::new(0),
                targets: 0,
                target_creature_type: CreatureTypeKey::new(0),
                requires_spell_focus: SpellFocusObjectKey::new(0),
                caster_aura_state: 0,
                target_aura_state: 0,
                casting_time_index: SpellCastTimesKey::new(0),
                recovery_time: 0,
                category_recovery_time: 0,
                aura_interrupt_flags: 0,
                channel_interrupt_flags: 0,
                proc_type_mask: 0,
                proc_chance: 0,
                proc_charges: 0,
                max_level: 0,
                base_level: 0,
                spell_level: 0,
                duration: SpellDurationKey::new(0),
                power_type: 0,
                mana_cost: 0,
                mana_cost_per_level: 0,
                mana_cost_per_second: 0,
                mana_cost_per_second_per_level: 0,
                range: SpellRangeKey::new(0),
                speed: 0.0,
                modal_next_spell: SpellKey::new(0),
                stack_amount: 0,
                totem: [0; 2],
                reagent: [0; 8],
                reagent_count: [0; 8],
                equipped_item_class: ItemClassKey::new(0),
                equipped_item_subclass: 0,
                equipped_item_inventory_type: 0,
                effect: [0; 3],
                effect_die_sides: [0; 3],
                effect_base_dice: [0; 3],
                effect_dice_per_level: [0.0; 3],
                effect_real_points_per_level: [0.0; 3],
                effect_base_points: [0; 3],
                effect_mechanic: [0; 3],
                implicit_target_a: [0; 3],
                implicit_target_b: [0; 3],
                effect_radius: [0; 3],
                effect_aura: [EffectAura::None; 3],
                effect_amplitude: [0.0; 3],
                effect_multiple_values: [0.0; 3],
                effect_chain_target: [0; 3],
                effect_item_type: [0; 3],
                effect_misc_value: [0; 3],
                effect_trigger_spell: [0; 3],
                effect_points_per_combo: [0.0; 3],
                spell_visual: [0; 2],
                spell_icon: SpellIconKey::new(0),
                active_icon: 0,
                spell_priority: 0,
                unknown_flag: 0,
                name: crate::ConstLocalizedString::empty(),
                name_subtext: crate::ConstLocalizedString::empty(),
                description: crate::ConstLocalizedString::empty(),
                aura_description: crate::ConstLocalizedString::empty(),
                mana_cost_percent: 0,
                start_recovery_category: 0,
                start_recovery_time: 0,
                max_target_level: 0,
                spell_class_set: ChrClassesKey::new(0),
                spell_class_mask: [0; 2],
                max_targets: 0,
                defence_type: 0,
                prevention_type: 0,
                stance_bar_order: 0,
                damage_multiplier: [0.0; 3],
                min_faction: 0,
                min_reputation: 0,
                required_aura_vision: 0,
            }
        ; S];

        let mut i = 0;
        while i < S {
            // id: primary_key (Spell) uint32
            let id = SpellKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // school: foreign_key (Resistances) uint32
            let school = ResistancesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // category: foreign_key (SpellCategory) uint32
            let category = SpellCategoryKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // cast_ui: int32
            let cast_ui = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // dispel_type: foreign_key (SpellDispelType) uint32
            let dispel_type = SpellDispelTypeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // mechanic: foreign_key (SpellMechanic) uint32
            let mechanic = SpellMechanicKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attributes: Attributes
            let attributes = Attributes::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attributes_ex1: AttributesEx1
            let attributes_ex1 = AttributesEx1::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attributes_ex2: AttributesEx2
            let attributes_ex2 = AttributesEx2::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attributes_ex3: AttributesEx3
            let attributes_ex3 = AttributesEx3::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // attributes_ex4: AttributesEx4
            let attributes_ex4 = AttributesEx4::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // shapeshift_mask: foreign_key (SpellShapeshiftForm) uint32
            let shapeshift_mask = SpellShapeshiftFormKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // shapeshift_exclude: foreign_key (SpellShapeshiftForm) uint32
            let shapeshift_exclude = SpellShapeshiftFormKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // targets: int32
            let targets = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_creature_type: foreign_key (CreatureType) uint32
            let target_creature_type = CreatureTypeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // requires_spell_focus: foreign_key (SpellFocusObject) uint32
            let requires_spell_focus = SpellFocusObjectKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // caster_aura_state: int32
            let caster_aura_state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // target_aura_state: int32
            let target_aura_state = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // casting_time_index: foreign_key (SpellCastTimes) uint32
            let casting_time_index = SpellCastTimesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // recovery_time: int32
            let recovery_time = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // category_recovery_time: int32
            let category_recovery_time = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // duration: foreign_key (SpellDuration) uint32
            let duration = SpellDurationKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
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

            // mana_cost_per_second: int32
            let mana_cost_per_second = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // mana_cost_per_second_per_level: int32
            let mana_cost_per_second_per_level = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // range: foreign_key (SpellRange) uint32
            let range = SpellRangeKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // speed: float
            let speed = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // modal_next_spell: foreign_key (Spell) uint32
            let modal_next_spell = SpellKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // stack_amount: int32
            let stack_amount = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // equipped_item_class: foreign_key (ItemClass) uint32
            let equipped_item_class = ItemClassKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // equipped_item_subclass: uint32
            let equipped_item_subclass = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // equipped_item_inventory_type: int32
            let equipped_item_inventory_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // effect_base_dice: int32[3]
            let effect_base_dice = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_dice_per_level: float[3]
            let effect_dice_per_level = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // effect_mechanic: uint32[3]
            let effect_mechanic = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // effect_radius: uint32[3]
            let effect_radius = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // effect_multiple_values: float[3]
            let effect_multiple_values = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_chain_target: int32[3]
            let effect_chain_target = {
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

            // effect_misc_value: uint32[3]
            let effect_misc_value = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // effect_trigger_spell: uint32[3]
            let effect_trigger_spell = {
                let mut a = [0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // spell_visual: int32[2]
            let spell_visual = {
                let mut a = [0; 2];
                let mut i = 0;
                while i < a.len() {
                    a[i] = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // spell_icon: foreign_key (SpellIcon) uint32
            let spell_icon = SpellIconKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // active_icon: int32
            let active_icon = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // spell_priority: int32
            let spell_priority = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // unknown_flag: int32
            let unknown_flag = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // name: string_ref_loc
            let name = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // name_subtext: string_ref_loc
            let name_subtext = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // description: string_ref_loc
            let description = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // aura_description: string_ref_loc
            let aura_description = ConstLocalizedString::new(
                crate::util::get_string_from_block(b_offset, b, string_block),
                crate::util::get_string_from_block(b_offset + 4, b, string_block),
                crate::util::get_string_from_block(b_offset + 8, b, string_block),
                crate::util::get_string_from_block(b_offset + 12, b, string_block),
                crate::util::get_string_from_block(b_offset + 16, b, string_block),
                crate::util::get_string_from_block(b_offset + 20, b, string_block),
                crate::util::get_string_from_block(b_offset + 24, b, string_block),
                crate::util::get_string_from_block(b_offset + 28, b, string_block),
                u32::from_le_bytes([b[b_offset + 32], b[b_offset + 33], b[b_offset + 34], b[b_offset + 35]]),
            );
            b_offset += 36;

            // mana_cost_percent: int32
            let mana_cost_percent = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
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

            // spell_class_set: foreign_key (ChrClasses) uint32
            let spell_class_set = ChrClassesKey::new(u32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]));
            b_offset += 4;

            // spell_class_mask: int32[2]
            let spell_class_mask = {
                let mut a = [0; 2];
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

            // defence_type: int32
            let defence_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // prevention_type: int32
            let prevention_type = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // stance_bar_order: int32
            let stance_bar_order = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // damage_multiplier: float[3]
            let damage_multiplier = {
                let mut a = [0.0; 3];
                let mut i = 0;
                while i < a.len() {
                    a[i] = crate::util::ct_u32_to_f32([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
                    b_offset += 4;
                    i += 1;
                }

                a
            };

            // min_faction: int32
            let min_faction = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // min_reputation: int32
            let min_reputation = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            // required_aura_vision: int32
            let required_aura_vision = i32::from_le_bytes([b[b_offset + 0], b[b_offset + 1], b[b_offset + 2], b[b_offset + 3]]);
            b_offset += 4;

            rows[i] = ConstSpellRow {
                id,
                school,
                category,
                cast_ui,
                dispel_type,
                mechanic,
                attributes,
                attributes_ex1,
                attributes_ex2,
                attributes_ex3,
                attributes_ex4,
                shapeshift_mask,
                shapeshift_exclude,
                targets,
                target_creature_type,
                requires_spell_focus,
                caster_aura_state,
                target_aura_state,
                casting_time_index,
                recovery_time,
                category_recovery_time,
                aura_interrupt_flags,
                channel_interrupt_flags,
                proc_type_mask,
                proc_chance,
                proc_charges,
                max_level,
                base_level,
                spell_level,
                duration,
                power_type,
                mana_cost,
                mana_cost_per_level,
                mana_cost_per_second,
                mana_cost_per_second_per_level,
                range,
                speed,
                modal_next_spell,
                stack_amount,
                totem,
                reagent,
                reagent_count,
                equipped_item_class,
                equipped_item_subclass,
                equipped_item_inventory_type,
                effect,
                effect_die_sides,
                effect_base_dice,
                effect_dice_per_level,
                effect_real_points_per_level,
                effect_base_points,
                effect_mechanic,
                implicit_target_a,
                implicit_target_b,
                effect_radius,
                effect_aura,
                effect_amplitude,
                effect_multiple_values,
                effect_chain_target,
                effect_item_type,
                effect_misc_value,
                effect_trigger_spell,
                effect_points_per_combo,
                spell_visual,
                spell_icon,
                active_icon,
                spell_priority,
                unknown_flag,
                name,
                name_subtext,
                description,
                aura_description,
                mana_cost_percent,
                start_recovery_category,
                start_recovery_time,
                max_target_level,
                spell_class_set,
                spell_class_mask,
                max_targets,
                defence_type,
                prevention_type,
                stance_bar_order,
                damage_multiplier,
                min_faction,
                min_reputation,
                required_aura_vision,
            };
            i += 1;
        }

        Self { rows }
    }

    pub fn to_owned(&self) -> Spell {
        Spell {
            rows: self.rows.iter().map(|s| SpellRow {
                id: s.id,
                school: s.school,
                category: s.category,
                cast_ui: s.cast_ui,
                dispel_type: s.dispel_type,
                mechanic: s.mechanic,
                attributes: s.attributes,
                attributes_ex1: s.attributes_ex1,
                attributes_ex2: s.attributes_ex2,
                attributes_ex3: s.attributes_ex3,
                attributes_ex4: s.attributes_ex4,
                shapeshift_mask: s.shapeshift_mask,
                shapeshift_exclude: s.shapeshift_exclude,
                targets: s.targets,
                target_creature_type: s.target_creature_type,
                requires_spell_focus: s.requires_spell_focus,
                caster_aura_state: s.caster_aura_state,
                target_aura_state: s.target_aura_state,
                casting_time_index: s.casting_time_index,
                recovery_time: s.recovery_time,
                category_recovery_time: s.category_recovery_time,
                aura_interrupt_flags: s.aura_interrupt_flags,
                channel_interrupt_flags: s.channel_interrupt_flags,
                proc_type_mask: s.proc_type_mask,
                proc_chance: s.proc_chance,
                proc_charges: s.proc_charges,
                max_level: s.max_level,
                base_level: s.base_level,
                spell_level: s.spell_level,
                duration: s.duration,
                power_type: s.power_type,
                mana_cost: s.mana_cost,
                mana_cost_per_level: s.mana_cost_per_level,
                mana_cost_per_second: s.mana_cost_per_second,
                mana_cost_per_second_per_level: s.mana_cost_per_second_per_level,
                range: s.range,
                speed: s.speed,
                modal_next_spell: s.modal_next_spell,
                stack_amount: s.stack_amount,
                totem: s.totem,
                reagent: s.reagent,
                reagent_count: s.reagent_count,
                equipped_item_class: s.equipped_item_class,
                equipped_item_subclass: s.equipped_item_subclass,
                equipped_item_inventory_type: s.equipped_item_inventory_type,
                effect: s.effect,
                effect_die_sides: s.effect_die_sides,
                effect_base_dice: s.effect_base_dice,
                effect_dice_per_level: s.effect_dice_per_level,
                effect_real_points_per_level: s.effect_real_points_per_level,
                effect_base_points: s.effect_base_points,
                effect_mechanic: s.effect_mechanic,
                implicit_target_a: s.implicit_target_a,
                implicit_target_b: s.implicit_target_b,
                effect_radius: s.effect_radius,
                effect_aura: s.effect_aura,
                effect_amplitude: s.effect_amplitude,
                effect_multiple_values: s.effect_multiple_values,
                effect_chain_target: s.effect_chain_target,
                effect_item_type: s.effect_item_type,
                effect_misc_value: s.effect_misc_value,
                effect_trigger_spell: s.effect_trigger_spell,
                effect_points_per_combo: s.effect_points_per_combo,
                spell_visual: s.spell_visual,
                spell_icon: s.spell_icon,
                active_icon: s.active_icon,
                spell_priority: s.spell_priority,
                unknown_flag: s.unknown_flag,
                name: s.name.to_string(),
                name_subtext: s.name_subtext.to_string(),
                description: s.description.to_string(),
                aura_description: s.aura_description.to_string(),
                mana_cost_percent: s.mana_cost_percent,
                start_recovery_category: s.start_recovery_category,
                start_recovery_time: s.start_recovery_time,
                max_target_level: s.max_target_level,
                spell_class_set: s.spell_class_set,
                spell_class_mask: s.spell_class_mask,
                max_targets: s.max_targets,
                defence_type: s.defence_type,
                prevention_type: s.prevention_type,
                stance_bar_order: s.stance_bar_order,
                damage_multiplier: s.damage_multiplier,
                min_faction: s.min_faction,
                min_reputation: s.min_reputation,
                required_aura_vision: s.required_aura_vision,
            }).collect(),
        }
    }
    // TODO: Indexable?
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
pub struct SpellKey {
    pub id: u32
}

impl SpellKey {
    pub const fn new(id: u32) -> Self {
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

impl From<u32> for SpellKey {
    fn from(v: u32) -> Self {
        Self::new(v)
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
    ModInvisibilityDetection,
    ObsModHealth,
    ObsModMana,
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
    ModParrySkill,
    ModParryPercent,
    ModDodgeSkill,
    ModDodgePercent,
    ModBlockSkill,
    ModBlockPercent,
    ModCritPercent,
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
    PeriodicManaFunnel,
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
    ModResistChance,
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
    SharePetTracking,
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
    Charisma,
    Persuaded,
    MechanicImmunityMask,
    RetainComboPoints,
    ResistPushback,
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
    PowerBurnMana,
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
    AllowChampionSpells,
    ModSpellDamageOfStatPercent,
    ModSpellHealingOfStatPercent,
    SpiritOfRedemption,
    AoeCharm,
    ModDebuffResistance,
    ModAttackerSpellCritChance,
    ModFlatSpellDamageVersus,
    ModFlatSpellCritDamageVersus,
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
            19 => Self::ModInvisibilityDetection,
            20 => Self::ObsModHealth,
            21 => Self::ObsModMana,
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
            46 => Self::ModParrySkill,
            47 => Self::ModParryPercent,
            48 => Self::ModDodgeSkill,
            49 => Self::ModDodgePercent,
            50 => Self::ModBlockSkill,
            51 => Self::ModBlockPercent,
            52 => Self::ModCritPercent,
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
            63 => Self::PeriodicManaFunnel,
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
            90 => Self::ModResistChance,
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
            119 => Self::SharePetTracking,
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
            145 => Self::Charisma,
            146 => Self::Persuaded,
            147 => Self::MechanicImmunityMask,
            148 => Self::RetainComboPoints,
            149 => Self::ResistPushback,
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
            162 => Self::PowerBurnMana,
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
            173 => Self::AllowChampionSpells,
            174 => Self::ModSpellDamageOfStatPercent,
            175 => Self::ModSpellHealingOfStatPercent,
            176 => Self::SpiritOfRedemption,
            177 => Self::AoeCharm,
            178 => Self::ModDebuffResistance,
            179 => Self::ModAttackerSpellCritChance,
            180 => Self::ModFlatSpellDamageVersus,
            181 => Self::ModFlatSpellCritDamageVersus,
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
            Self::ModInvisibilityDetection => 19,
            Self::ObsModHealth => 20,
            Self::ObsModMana => 21,
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
            Self::ModParrySkill => 46,
            Self::ModParryPercent => 47,
            Self::ModDodgeSkill => 48,
            Self::ModDodgePercent => 49,
            Self::ModBlockSkill => 50,
            Self::ModBlockPercent => 51,
            Self::ModCritPercent => 52,
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
            Self::PeriodicManaFunnel => 63,
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
            Self::ModResistChance => 90,
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
            Self::SharePetTracking => 119,
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
            Self::Charisma => 145,
            Self::Persuaded => 146,
            Self::MechanicImmunityMask => 147,
            Self::RetainComboPoints => 148,
            Self::ResistPushback => 149,
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
            Self::PowerBurnMana => 162,
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
            Self::AllowChampionSpells => 173,
            Self::ModSpellDamageOfStatPercent => 174,
            Self::ModSpellHealingOfStatPercent => 175,
            Self::SpiritOfRedemption => 176,
            Self::AoeCharm => 177,
            Self::ModDebuffResistance => 178,
            Self::ModAttackerSpellCritChance => 179,
            Self::ModFlatSpellDamageVersus => 180,
            Self::ModFlatSpellCritDamageVersus => 181,
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
        }

    }

}

impl Default for EffectAura {
    fn default() -> Self {
        Self::None
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct Attributes {
    value: u32,
}

impl Attributes {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> u32 {
        self.value
    }

    pub const fn none(&self) -> bool {
        self.value == 0
    }

    pub const fn proc_failure_burns_charge(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn uses_ranged_slot(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn on_next_swing_no_damage(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn need_exotic_ammo(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn is_ability(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn is_tradeskill(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn passive(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn do_not_display(&self) -> bool {
        (self.value & 128) != 0
    }

    pub const fn do_not_log(&self) -> bool {
        (self.value & 256) != 0
    }

    pub const fn held_item_only(&self) -> bool {
        (self.value & 512) != 0
    }

    pub const fn on_next_swing(&self) -> bool {
        (self.value & 1024) != 0
    }

    pub const fn wearer_casts_proc_trigger(&self) -> bool {
        (self.value & 2048) != 0
    }

    pub const fn daytime_only(&self) -> bool {
        (self.value & 4096) != 0
    }

    pub const fn night_only(&self) -> bool {
        (self.value & 8192) != 0
    }

    pub const fn only_indoors(&self) -> bool {
        (self.value & 16384) != 0
    }

    pub const fn only_outdoors(&self) -> bool {
        (self.value & 32768) != 0
    }

    pub const fn not_shapeshift(&self) -> bool {
        (self.value & 65536) != 0
    }

    pub const fn only_stealthed(&self) -> bool {
        (self.value & 131072) != 0
    }

    pub const fn do_not_sheath(&self) -> bool {
        (self.value & 262144) != 0
    }

    pub const fn scales_with_creature_level(&self) -> bool {
        (self.value & 524288) != 0
    }

    pub const fn cancels_auto_attack_combat(&self) -> bool {
        (self.value & 1048576) != 0
    }

    pub const fn no_active_defense(&self) -> bool {
        (self.value & 2097152) != 0
    }

    pub const fn track_target_in_cast_player_only(&self) -> bool {
        (self.value & 4194304) != 0
    }

    pub const fn allow_cast_while_dead(&self) -> bool {
        (self.value & 8388608) != 0
    }

    pub const fn allow_while_mounted(&self) -> bool {
        (self.value & 16777216) != 0
    }

    pub const fn cooldown_on_event(&self) -> bool {
        (self.value & 33554432) != 0
    }

    pub const fn aura_is_debuff(&self) -> bool {
        (self.value & 67108864) != 0
    }

    pub const fn allow_while_sitting(&self) -> bool {
        (self.value & 134217728) != 0
    }

    pub const fn not_in_combat_only_peaceful(&self) -> bool {
        (self.value & 268435456) != 0
    }

    pub const fn no_immunities(&self) -> bool {
        (self.value & 536870912) != 0
    }

    pub const fn heartbeat_resist(&self) -> bool {
        (self.value & 1073741824) != 0
    }

    pub const fn no_aura_cancel(&self) -> bool {
        (self.value & 2147483648) != 0
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AttributesEx1 {
    value: u32,
}

impl AttributesEx1 {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> u32 {
        self.value
    }

    pub const fn none(&self) -> bool {
        self.value == 0
    }

    pub const fn dismiss_pet_first(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn use_all_mana(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn is_channeled(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn no_redirection(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn no_skill_increase(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn allow_while_stealthed(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn is_self_channeled(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn no_reflection(&self) -> bool {
        (self.value & 128) != 0
    }

    pub const fn only_peaceful_targets(&self) -> bool {
        (self.value & 256) != 0
    }

    pub const fn initiates_combat_enables_auto_attack(&self) -> bool {
        (self.value & 512) != 0
    }

    pub const fn no_threat(&self) -> bool {
        (self.value & 1024) != 0
    }

    pub const fn aura_unique(&self) -> bool {
        (self.value & 2048) != 0
    }

    pub const fn failure_breaks_stealth(&self) -> bool {
        (self.value & 4096) != 0
    }

    pub const fn toggle_farsight(&self) -> bool {
        (self.value & 8192) != 0
    }

    pub const fn track_target_in_channel(&self) -> bool {
        (self.value & 16384) != 0
    }

    pub const fn immunity_purges_effect(&self) -> bool {
        (self.value & 32768) != 0
    }

    pub const fn immunity_to_hostile_and_friendly_effects(&self) -> bool {
        (self.value & 65536) != 0
    }

    pub const fn no_autocast_ai(&self) -> bool {
        (self.value & 131072) != 0
    }

    pub const fn prevents_anim(&self) -> bool {
        (self.value & 262144) != 0
    }

    pub const fn exclude_caster(&self) -> bool {
        (self.value & 524288) != 0
    }

    pub const fn finishing_move_damage(&self) -> bool {
        (self.value & 1048576) != 0
    }

    pub const fn threat_only_on_miss(&self) -> bool {
        (self.value & 2097152) != 0
    }

    pub const fn finishing_move_duration(&self) -> bool {
        (self.value & 4194304) != 0
    }

    pub const fn unk23(&self) -> bool {
        (self.value & 8388608) != 0
    }

    pub const fn special_skillup(&self) -> bool {
        (self.value & 16777216) != 0
    }

    pub const fn aura_stays_after_combat(&self) -> bool {
        (self.value & 33554432) != 0
    }

    pub const fn require_all_targets(&self) -> bool {
        (self.value & 67108864) != 0
    }

    pub const fn discount_power_on_miss(&self) -> bool {
        (self.value & 134217728) != 0
    }

    pub const fn no_aura_icon(&self) -> bool {
        (self.value & 268435456) != 0
    }

    pub const fn name_in_channel_bar(&self) -> bool {
        (self.value & 536870912) != 0
    }

    pub const fn combo_on_block(&self) -> bool {
        (self.value & 1073741824) != 0
    }

    pub const fn cast_when_learned(&self) -> bool {
        (self.value & 2147483648) != 0
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AttributesEx2 {
    value: u32,
}

impl AttributesEx2 {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> u32 {
        self.value
    }

    pub const fn none(&self) -> bool {
        self.value == 0
    }

    pub const fn allow_dead_target(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn no_shapeshift_ui(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn ignore_line_of_sight(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn allow_low_level_buff(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn use_shapeshift_bar(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn auto_repeat(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn cannot_cast_on_tapped(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn do_not_report_spell_failure(&self) -> bool {
        (self.value & 128) != 0
    }

    pub const fn include_in_advanced_combat_log(&self) -> bool {
        (self.value & 256) != 0
    }

    pub const fn always_cast_as_unit(&self) -> bool {
        (self.value & 512) != 0
    }

    pub const fn special_taming_flag(&self) -> bool {
        (self.value & 1024) != 0
    }

    pub const fn no_target_per_second_costs(&self) -> bool {
        (self.value & 2048) != 0
    }

    pub const fn chain_from_caster(&self) -> bool {
        (self.value & 4096) != 0
    }

    pub const fn enchant_own_item_only(&self) -> bool {
        (self.value & 8192) != 0
    }

    pub const fn allow_while_invisible(&self) -> bool {
        (self.value & 16384) != 0
    }

    pub const fn unk15(&self) -> bool {
        (self.value & 32768) != 0
    }

    pub const fn no_active_pets(&self) -> bool {
        (self.value & 65536) != 0
    }

    pub const fn do_not_reset_combat_timers(&self) -> bool {
        (self.value & 131072) != 0
    }

    pub const fn req_dead_pet(&self) -> bool {
        (self.value & 262144) != 0
    }

    pub const fn allow_while_not_shapeshifted(&self) -> bool {
        (self.value & 524288) != 0
    }

    pub const fn initiate_combat_post_cast(&self) -> bool {
        (self.value & 1048576) != 0
    }

    pub const fn fail_on_all_targets_immune(&self) -> bool {
        (self.value & 2097152) != 0
    }

    pub const fn no_initial_threat(&self) -> bool {
        (self.value & 4194304) != 0
    }

    pub const fn proc_cooldown_on_failure(&self) -> bool {
        (self.value & 8388608) != 0
    }

    pub const fn item_cast_with_owner_skill(&self) -> bool {
        (self.value & 16777216) != 0
    }

    pub const fn dont_block_mana_regen(&self) -> bool {
        (self.value & 33554432) != 0
    }

    pub const fn no_school_immunities(&self) -> bool {
        (self.value & 67108864) != 0
    }

    pub const fn ignore_weaponskill(&self) -> bool {
        (self.value & 134217728) != 0
    }

    pub const fn not_an_action(&self) -> bool {
        (self.value & 268435456) != 0
    }

    pub const fn cant_crit(&self) -> bool {
        (self.value & 536870912) != 0
    }

    pub const fn active_threat(&self) -> bool {
        (self.value & 1073741824) != 0
    }

    pub const fn retain_item_cast(&self) -> bool {
        (self.value & 2147483648) != 0
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AttributesEx3 {
    value: u32,
}

impl AttributesEx3 {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> u32 {
        self.value
    }

    pub const fn none(&self) -> bool {
        self.value == 0
    }

    pub const fn pvp_enabling(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn no_proc_equip_requirement(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn no_casting_bar_text(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn completely_blocked(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn no_res_timer(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn no_durability_loss(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn no_avoidance(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn dot_stacking_rule(&self) -> bool {
        (self.value & 128) != 0
    }

    pub const fn only_on_player(&self) -> bool {
        (self.value & 256) != 0
    }

    pub const fn not_a_proc(&self) -> bool {
        (self.value & 512) != 0
    }

    pub const fn requires_main_hand_weapon(&self) -> bool {
        (self.value & 1024) != 0
    }

    pub const fn only_battlegrounds(&self) -> bool {
        (self.value & 2048) != 0
    }

    pub const fn only_on_ghosts(&self) -> bool {
        (self.value & 4096) != 0
    }

    pub const fn hide_channel_bar(&self) -> bool {
        (self.value & 8192) != 0
    }

    pub const fn hide_in_raid_filter(&self) -> bool {
        (self.value & 16384) != 0
    }

    pub const fn normal_ranged_attack(&self) -> bool {
        (self.value & 32768) != 0
    }

    pub const fn suppress_caster_procs(&self) -> bool {
        (self.value & 65536) != 0
    }

    pub const fn suppress_target_procs(&self) -> bool {
        (self.value & 131072) != 0
    }

    pub const fn always_hit(&self) -> bool {
        (self.value & 262144) != 0
    }

    pub const fn instant_target_procs(&self) -> bool {
        (self.value & 524288) != 0
    }

    pub const fn allow_aura_while_dead(&self) -> bool {
        (self.value & 1048576) != 0
    }

    pub const fn only_proc_outdoors(&self) -> bool {
        (self.value & 2097152) != 0
    }

    pub const fn casting_cancels_autorepeat(&self) -> bool {
        (self.value & 4194304) != 0
    }

    pub const fn no_damage_history(&self) -> bool {
        (self.value & 8388608) != 0
    }

    pub const fn requires_offhand_weapon(&self) -> bool {
        (self.value & 16777216) != 0
    }

    pub const fn treat_as_periodic(&self) -> bool {
        (self.value & 33554432) != 0
    }

    pub const fn can_proc_from_procs(&self) -> bool {
        (self.value & 67108864) != 0
    }

    pub const fn only_proc_on_caster(&self) -> bool {
        (self.value & 134217728) != 0
    }

    pub const fn ignore_caster_and_target_restrictions(&self) -> bool {
        (self.value & 268435456) != 0
    }

    pub const fn ignore_caster_modifiers(&self) -> bool {
        (self.value & 536870912) != 0
    }

    pub const fn do_not_display_range(&self) -> bool {
        (self.value & 1073741824) != 0
    }

    pub const fn not_on_aoe_immune(&self) -> bool {
        (self.value & 2147483648) != 0
    }

}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
pub struct AttributesEx4 {
    value: u32,
}

impl AttributesEx4 {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }

    pub const fn as_int(&self) -> u32 {
        self.value
    }

    pub const fn none(&self) -> bool {
        self.value == 0
    }

    pub const fn no_cast_log(&self) -> bool {
        (self.value & 1) != 0
    }

    pub const fn class_trigger_only_on_target(&self) -> bool {
        (self.value & 2) != 0
    }

    pub const fn aura_expires_offline(&self) -> bool {
        (self.value & 4) != 0
    }

    pub const fn no_helpful_threat(&self) -> bool {
        (self.value & 8) != 0
    }

    pub const fn no_harmful_threat(&self) -> bool {
        (self.value & 16) != 0
    }

    pub const fn allow_client_targeting(&self) -> bool {
        (self.value & 32) != 0
    }

    pub const fn cannot_be_stolen(&self) -> bool {
        (self.value & 64) != 0
    }

    pub const fn allow_cast_while_casting(&self) -> bool {
        (self.value & 128) != 0
    }

    pub const fn ignore_damage_taken_modifiers(&self) -> bool {
        (self.value & 256) != 0
    }

    pub const fn combat_feedback_when_usable(&self) -> bool {
        (self.value & 512) != 0
    }

    pub const fn weapon_speed_cost_scaling(&self) -> bool {
        (self.value & 1024) != 0
    }

    pub const fn no_partial_immunity(&self) -> bool {
        (self.value & 2048) != 0
    }

}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SpellRow {
    pub id: SpellKey,
    pub school: ResistancesKey,
    pub category: SpellCategoryKey,
    pub cast_ui: i32,
    pub dispel_type: SpellDispelTypeKey,
    pub mechanic: SpellMechanicKey,
    pub attributes: Attributes,
    pub attributes_ex1: AttributesEx1,
    pub attributes_ex2: AttributesEx2,
    pub attributes_ex3: AttributesEx3,
    pub attributes_ex4: AttributesEx4,
    pub shapeshift_mask: SpellShapeshiftFormKey,
    pub shapeshift_exclude: SpellShapeshiftFormKey,
    pub targets: i32,
    pub target_creature_type: CreatureTypeKey,
    pub requires_spell_focus: SpellFocusObjectKey,
    pub caster_aura_state: i32,
    pub target_aura_state: i32,
    pub casting_time_index: SpellCastTimesKey,
    pub recovery_time: i32,
    pub category_recovery_time: i32,
    pub aura_interrupt_flags: i32,
    pub channel_interrupt_flags: i32,
    pub proc_type_mask: i32,
    pub proc_chance: i32,
    pub proc_charges: i32,
    pub max_level: i32,
    pub base_level: i32,
    pub spell_level: i32,
    pub duration: SpellDurationKey,
    pub power_type: i32,
    pub mana_cost: i32,
    pub mana_cost_per_level: i32,
    pub mana_cost_per_second: i32,
    pub mana_cost_per_second_per_level: i32,
    pub range: SpellRangeKey,
    pub speed: f32,
    pub modal_next_spell: SpellKey,
    pub stack_amount: i32,
    pub totem: [i32; 2],
    pub reagent: [i32; 8],
    pub reagent_count: [i32; 8],
    pub equipped_item_class: ItemClassKey,
    pub equipped_item_subclass: u32,
    pub equipped_item_inventory_type: i32,
    pub effect: [i32; 3],
    pub effect_die_sides: [i32; 3],
    pub effect_base_dice: [i32; 3],
    pub effect_dice_per_level: [f32; 3],
    pub effect_real_points_per_level: [f32; 3],
    pub effect_base_points: [i32; 3],
    pub effect_mechanic: [u32; 3],
    pub implicit_target_a: [i32; 3],
    pub implicit_target_b: [i32; 3],
    pub effect_radius: [u32; 3],
    pub effect_aura: [EffectAura; 3],
    pub effect_amplitude: [f32; 3],
    pub effect_multiple_values: [f32; 3],
    pub effect_chain_target: [i32; 3],
    pub effect_item_type: [i32; 3],
    pub effect_misc_value: [u32; 3],
    pub effect_trigger_spell: [u32; 3],
    pub effect_points_per_combo: [f32; 3],
    pub spell_visual: [i32; 2],
    pub spell_icon: SpellIconKey,
    pub active_icon: i32,
    pub spell_priority: i32,
    pub unknown_flag: i32,
    pub name: LocalizedString,
    pub name_subtext: LocalizedString,
    pub description: LocalizedString,
    pub aura_description: LocalizedString,
    pub mana_cost_percent: i32,
    pub start_recovery_category: i32,
    pub start_recovery_time: i32,
    pub max_target_level: i32,
    pub spell_class_set: ChrClassesKey,
    pub spell_class_mask: [i32; 2],
    pub max_targets: i32,
    pub defence_type: i32,
    pub prevention_type: i32,
    pub stance_bar_order: i32,
    pub damage_multiplier: [f32; 3],
    pub min_faction: i32,
    pub min_reputation: i32,
    pub required_aura_vision: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ConstSpellRow {
    pub id: SpellKey,
    pub school: ResistancesKey,
    pub category: SpellCategoryKey,
    pub cast_ui: i32,
    pub dispel_type: SpellDispelTypeKey,
    pub mechanic: SpellMechanicKey,
    pub attributes: Attributes,
    pub attributes_ex1: AttributesEx1,
    pub attributes_ex2: AttributesEx2,
    pub attributes_ex3: AttributesEx3,
    pub attributes_ex4: AttributesEx4,
    pub shapeshift_mask: SpellShapeshiftFormKey,
    pub shapeshift_exclude: SpellShapeshiftFormKey,
    pub targets: i32,
    pub target_creature_type: CreatureTypeKey,
    pub requires_spell_focus: SpellFocusObjectKey,
    pub caster_aura_state: i32,
    pub target_aura_state: i32,
    pub casting_time_index: SpellCastTimesKey,
    pub recovery_time: i32,
    pub category_recovery_time: i32,
    pub aura_interrupt_flags: i32,
    pub channel_interrupt_flags: i32,
    pub proc_type_mask: i32,
    pub proc_chance: i32,
    pub proc_charges: i32,
    pub max_level: i32,
    pub base_level: i32,
    pub spell_level: i32,
    pub duration: SpellDurationKey,
    pub power_type: i32,
    pub mana_cost: i32,
    pub mana_cost_per_level: i32,
    pub mana_cost_per_second: i32,
    pub mana_cost_per_second_per_level: i32,
    pub range: SpellRangeKey,
    pub speed: f32,
    pub modal_next_spell: SpellKey,
    pub stack_amount: i32,
    pub totem: [i32; 2],
    pub reagent: [i32; 8],
    pub reagent_count: [i32; 8],
    pub equipped_item_class: ItemClassKey,
    pub equipped_item_subclass: u32,
    pub equipped_item_inventory_type: i32,
    pub effect: [i32; 3],
    pub effect_die_sides: [i32; 3],
    pub effect_base_dice: [i32; 3],
    pub effect_dice_per_level: [f32; 3],
    pub effect_real_points_per_level: [f32; 3],
    pub effect_base_points: [i32; 3],
    pub effect_mechanic: [u32; 3],
    pub implicit_target_a: [i32; 3],
    pub implicit_target_b: [i32; 3],
    pub effect_radius: [u32; 3],
    pub effect_aura: [EffectAura; 3],
    pub effect_amplitude: [f32; 3],
    pub effect_multiple_values: [f32; 3],
    pub effect_chain_target: [i32; 3],
    pub effect_item_type: [i32; 3],
    pub effect_misc_value: [u32; 3],
    pub effect_trigger_spell: [u32; 3],
    pub effect_points_per_combo: [f32; 3],
    pub spell_visual: [i32; 2],
    pub spell_icon: SpellIconKey,
    pub active_icon: i32,
    pub spell_priority: i32,
    pub unknown_flag: i32,
    pub name: ConstLocalizedString,
    pub name_subtext: ConstLocalizedString,
    pub description: ConstLocalizedString,
    pub aura_description: ConstLocalizedString,
    pub mana_cost_percent: i32,
    pub start_recovery_category: i32,
    pub start_recovery_time: i32,
    pub max_target_level: i32,
    pub spell_class_set: ChrClassesKey,
    pub spell_class_mask: [i32; 2],
    pub max_targets: i32,
    pub defence_type: i32,
    pub prevention_type: i32,
    pub stance_bar_order: i32,
    pub damage_multiplier: [f32; 3],
    pub min_faction: i32,
    pub min_reputation: i32,
    pub required_aura_vision: i32,
}

