use crate::{
    DbcTable, Indexable, LocalizedString,
};
use crate::header::{
    DbcHeader, HEADER_SIZE, parse_header,
};
use crate::vanilla_tables::chr_classes::ChrClassesKey;
use crate::vanilla_tables::creature_type::CreatureTypeKey;
use crate::vanilla_tables::item_class::ItemClassKey;
use crate::vanilla_tables::resistances::ResistancesKey;
use crate::vanilla_tables::spell_cast_times::SpellCastTimesKey;
use crate::vanilla_tables::spell_category::SpellCategoryKey;
use crate::vanilla_tables::spell_dispel_type::SpellDispelTypeKey;
use crate::vanilla_tables::spell_duration::SpellDurationKey;
use crate::vanilla_tables::spell_focus_object::SpellFocusObjectKey;
use crate::vanilla_tables::spell_icon::SpellIconKey;
use crate::vanilla_tables::spell_mechanic::SpellMechanicKey;
use crate::vanilla_tables::spell_range::SpellRangeKey;
use crate::vanilla_tables::spell_shapeshift_form::SpellShapeshiftFormKey;
use std::io::Write;
use wow_world_base::vanilla::{
    Attributes, AttributesEx1, AttributesEx2, AttributesEx3, AttributesEx4, AuraMod,
};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Spell {
    pub rows: Vec<SpellRow>,
}

impl DbcTable for Spell {
    type Row = SpellRow;

    const FILENAME: &'static str = "Spell.dbc";

    fn rows(&self) -> &[Self::Row] { &self.rows }
    fn rows_mut(&mut self) -> &mut [Self::Row] { &mut self.rows }

    fn read(b: &mut impl std::io::Read) -> Result<Self, crate::DbcError> {
        let mut header = [0_u8; HEADER_SIZE];
        b.read_exact(&mut header)?;
        let header = parse_header(&header)?;

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
            let attributes = Attributes::new(crate::util::read_u32_le(chunk)? as _);

            // attributes_ex1: AttributesEx1
            let attributes_ex1 = AttributesEx1::new(crate::util::read_u32_le(chunk)? as _);

            // attributes_ex2: AttributesEx2
            let attributes_ex2 = AttributesEx2::new(crate::util::read_u32_le(chunk)? as _);

            // attributes_ex3: AttributesEx3
            let attributes_ex3 = AttributesEx3::new(crate::util::read_u32_le(chunk)? as _);

            // attributes_ex4: AttributesEx4
            let attributes_ex4 = AttributesEx4::new(crate::util::read_u32_le(chunk)? as _);

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

            // effect_aura: AuraMod[3]
            let effect_aura = {
                let mut arr = [AuraMod::default(); 3];
                for i in arr.iter_mut() {
                    *i = crate::util::read_i32_le(chunk)?.try_into()?;
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


            // effect_aura: AuraMod[3]
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
    pub effect_aura: [AuraMod; 3],
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

