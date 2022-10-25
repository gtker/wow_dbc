use crate::{Options, SqliteError};
use rusqlite::{Connection, params};
use wow_dbc::DbcTable;
use wow_dbc::wrath_tables::*;

pub(crate) fn write_to_sqlite(file_name: &str, file_contents: &mut &[u8], options: &Options) -> Result<(), SqliteError> {
    let mut conn = Connection::open(&options.output_path)?;
    match file_name {
        "Cfg_Categories.dbc" => {
            let data = cfg_categories::Cfg_Categories::read(file_contents)?;
            let (table, insert) = Cfg_Categories();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.locale_mask,
                row.create_charset_mask,
                row.flags,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SoundEmitters.dbc" => {
            let data = sound_emitters::SoundEmitters::read(file_contents)?;
            let (table, insert) = SoundEmitters();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.position[0],
                row.position[1],
                row.position[2],
                row.direction[0],
                row.direction[1],
                row.direction[2],
                row.sound_entry_advanced_id.id,
                row.map_id.id,
                &row.name,
                ])?;
            }
            tx.commit()?;
        }
        "WorldMapTransforms.dbc" => {
            let data = world_map_transforms::WorldMapTransforms::read(file_contents)?;
            let (table, insert) = WorldMapTransforms();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.region_min[0],
                row.region_min[1],
                row.region_max[0],
                row.region_max[1],
                row.new_map_id.id,
                row.region_offset[0],
                row.region_offset[1],
                row.new_dungeon_map_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "LoadingScreens.dbc" => {
            let data = loading_screens::LoadingScreens::read(file_contents)?;
            let (table, insert) = LoadingScreens();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.file_name,
                row.has_wide_screen,
                ])?;
            }
            tx.commit()?;
        }
        "PetPersonality.dbc" => {
            let data = pet_personality::PetPersonality::read(file_contents)?;
            let (table, insert) = PetPersonality();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.happiness_threshold[0],
                row.happiness_threshold[1],
                row.happiness_threshold[2],
                row.happiness_damage[0],
                row.happiness_damage[1],
                row.happiness_damage[2],
                ])?;
            }
            tx.commit()?;
        }
        "MailTemplate.dbc" => {
            let data = mail_template::MailTemplate::read(file_contents)?;
            let (table, insert) = MailTemplate();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.subject_lang.en_gb,
                &row.subject_lang.ko_kr,
                &row.subject_lang.fr_fr,
                &row.subject_lang.de_de,
                &row.subject_lang.en_cn,
                &row.subject_lang.en_tw,
                &row.subject_lang.es_es,
                &row.subject_lang.es_mx,
                &row.subject_lang.ru_ru,
                &row.subject_lang.ja_jp,
                &row.subject_lang.pt_pt,
                &row.subject_lang.it_it,
                &row.subject_lang.unknown_12,
                &row.subject_lang.unknown_13,
                &row.subject_lang.unknown_14,
                &row.subject_lang.unknown_15,
                &row.subject_lang.flags,
                &row.body_lang.en_gb,
                &row.body_lang.ko_kr,
                &row.body_lang.fr_fr,
                &row.body_lang.de_de,
                &row.body_lang.en_cn,
                &row.body_lang.en_tw,
                &row.body_lang.es_es,
                &row.body_lang.es_mx,
                &row.body_lang.ru_ru,
                &row.body_lang.ja_jp,
                &row.body_lang.pt_pt,
                &row.body_lang.it_it,
                &row.body_lang.unknown_12,
                &row.body_lang.unknown_13,
                &row.body_lang.unknown_14,
                &row.body_lang.unknown_15,
                &row.body_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "WorldSafeLocs.dbc" => {
            let data = world_safe_locs::WorldSafeLocs::read(file_contents)?;
            let (table, insert) = WorldSafeLocs();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.continent.id,
                row.loc[0],
                row.loc[1],
                row.loc[2],
                &row.area_name_lang.en_gb,
                &row.area_name_lang.ko_kr,
                &row.area_name_lang.fr_fr,
                &row.area_name_lang.de_de,
                &row.area_name_lang.en_cn,
                &row.area_name_lang.en_tw,
                &row.area_name_lang.es_es,
                &row.area_name_lang.es_mx,
                &row.area_name_lang.ru_ru,
                &row.area_name_lang.ja_jp,
                &row.area_name_lang.pt_pt,
                &row.area_name_lang.it_it,
                &row.area_name_lang.unknown_12,
                &row.area_name_lang.unknown_13,
                &row.area_name_lang.unknown_14,
                &row.area_name_lang.unknown_15,
                &row.area_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "FactionGroup.dbc" => {
            let data = faction_group::FactionGroup::read(file_contents)?;
            let (table, insert) = FactionGroup();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.mask_id,
                &row.internal_name,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "PvpDifficulty.dbc" => {
            let data = pvp_difficulty::PvpDifficulty::read(file_contents)?;
            let (table, insert) = PvpDifficulty();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.range_index,
                row.min_level,
                row.max_level,
                row.difficulty,
                ])?;
            }
            tx.commit()?;
        }
        "BannedAddOns.dbc" => {
            let data = banned_add_ons::BannedAddOns::read(file_contents)?;
            let (table, insert) = BannedAddOns();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.name_m_d5[0],
                row.name_m_d5[1],
                row.name_m_d5[2],
                row.name_m_d5[3],
                row.version_m_d5[0],
                row.version_m_d5[1],
                row.version_m_d5[2],
                row.version_m_d5[3],
                row.last_modified,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "EmotesText.dbc" => {
            let data = emotes_text::EmotesText::read(file_contents)?;
            let (table, insert) = EmotesText();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.emote_id.id,
                row.emote_text[0],
                row.emote_text[1],
                row.emote_text[2],
                row.emote_text[3],
                row.emote_text[4],
                row.emote_text[5],
                row.emote_text[6],
                row.emote_text[7],
                row.emote_text[8],
                row.emote_text[9],
                row.emote_text[10],
                row.emote_text[11],
                row.emote_text[12],
                row.emote_text[13],
                row.emote_text[14],
                row.emote_text[15],
                ])?;
            }
            tx.commit()?;
        }
        "AttackAnimKits.dbc" => {
            let data = attack_anim_kits::AttackAnimKits::read(file_contents)?;
            let (table, insert) = AttackAnimKits();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_subclass_id,
                row.anim_type_id,
                row.anim_frequency,
                row.which_hand,
                ])?;
            }
            tx.commit()?;
        }
        "LightIntBand.dbc" => {
            let data = light_int_band::LightIntBand::read(file_contents)?;
            let (table, insert) = LightIntBand();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.num,
                row.time[0],
                row.time[1],
                row.time[2],
                row.time[3],
                row.time[4],
                row.time[5],
                row.time[6],
                row.time[7],
                row.time[8],
                row.time[9],
                row.time[10],
                row.time[11],
                row.time[12],
                row.time[13],
                row.time[14],
                row.time[15],
                row.data[0],
                row.data[1],
                row.data[2],
                row.data[3],
                row.data[4],
                row.data[5],
                row.data[6],
                row.data[7],
                row.data[8],
                row.data[9],
                row.data[10],
                row.data[11],
                row.data[12],
                row.data[13],
                row.data[14],
                row.data[15],
                ])?;
            }
            tx.commit()?;
        }
        "CreatureSpellData.dbc" => {
            let data = creature_spell_data::CreatureSpellData::read(file_contents)?;
            let (table, insert) = CreatureSpellData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.spells[0],
                row.spells[1],
                row.spells[2],
                row.spells[3],
                row.availability[0],
                row.availability[1],
                row.availability[2],
                row.availability[3],
                ])?;
            }
            tx.commit()?;
        }
        "CreatureModelData.dbc" => {
            let data = creature_model_data::CreatureModelData::read(file_contents)?;
            let (table, insert) = CreatureModelData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                &row.model_name,
                row.size_class,
                row.model_scale,
                row.blood_id.id,
                row.footprint_texture_id.id,
                row.footprint_texture_length,
                row.footprint_texture_width,
                row.footprint_particle_scale,
                row.foley_material_id.id,
                row.footstep_shake_size,
                row.death_thud_shake_size,
                row.sound_id.id,
                row.collision_width,
                row.collision_height,
                row.mount_height,
                row.geo_box_min_x,
                row.geo_box_min_y,
                row.geo_box_min_z,
                row.geo_box_max_x,
                row.geo_box_max_y,
                row.geo_box_max_z,
                row.world_effect_scale,
                row.attached_effect_scale,
                row.missile_collision_radius,
                row.missile_collision_push,
                row.missile_collision_raise,
                ])?;
            }
            tx.commit()?;
        }
        "SkillCostsData.dbc" => {
            let data = skill_costs_data::SkillCostsData::read(file_contents)?;
            let (table, insert) = SkillCostsData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.skill_costs_id,
                row.cost[0],
                row.cost[1],
                row.cost[2],
                ])?;
            }
            tx.commit()?;
        }
        "ItemGroupSounds.dbc" => {
            let data = item_group_sounds::ItemGroupSounds::read(file_contents)?;
            let (table, insert) = ItemGroupSounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound[0],
                row.sound[1],
                row.sound[2],
                row.sound[3],
                ])?;
            }
            tx.commit()?;
        }
        "LiquidMaterial.dbc" => {
            let data = liquid_material::LiquidMaterial::read(file_contents)?;
            let (table, insert) = LiquidMaterial();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.l_v_f,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "VehicleUIIndicator.dbc" => {
            let data = vehicle_ui_indicator::VehicleUIIndicator::read(file_contents)?;
            let (table, insert) = VehicleUIIndicator();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.background_texture,
                ])?;
            }
            tx.commit()?;
        }
        "ItemRandomSuffix.dbc" => {
            let data = item_random_suffix::ItemRandomSuffix::read(file_contents)?;
            let (table, insert) = ItemRandomSuffix();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.internal_name,
                row.enchantment[0],
                row.enchantment[1],
                row.enchantment[2],
                row.enchantment[3],
                row.enchantment[4],
                row.allocation_pct[0],
                row.allocation_pct[1],
                row.allocation_pct[2],
                row.allocation_pct[3],
                row.allocation_pct[4],
                ])?;
            }
            tx.commit()?;
        }
        "ZoneMusic.dbc" => {
            let data = zone_music::ZoneMusic::read(file_contents)?;
            let (table, insert) = ZoneMusic();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.set_name,
                row.silence_interval_min[0],
                row.silence_interval_min[1],
                row.silence_interval_max[0],
                row.silence_interval_max[1],
                row.sounds[0],
                row.sounds[1],
                ])?;
            }
            tx.commit()?;
        }
        "CharVariations.dbc" => {
            let data = char_variations::CharVariations::read(file_contents)?;
            let (table, insert) = CharVariations();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.race_id.id,
                row.sex_id,
                row.texture_hold_layer[0],
                row.texture_hold_layer[1],
                row.texture_hold_layer[2],
                row.texture_hold_layer[3],
                ])?;
            }
            tx.commit()?;
        }
        "GlyphProperties.dbc" => {
            let data = glyph_properties::GlyphProperties::read(file_contents)?;
            let (table, insert) = GlyphProperties();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.spell_id.id,
                row.glyph_slot_flags,
                row.spell_icon_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "GMSurveyQuestions.dbc" => {
            let data = gm_survey_questions::GMSurveyQuestions::read(file_contents)?;
            let (table, insert) = GMSurveyQuestions();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.question_lang.en_gb,
                &row.question_lang.ko_kr,
                &row.question_lang.fr_fr,
                &row.question_lang.de_de,
                &row.question_lang.en_cn,
                &row.question_lang.en_tw,
                &row.question_lang.es_es,
                &row.question_lang.es_mx,
                &row.question_lang.ru_ru,
                &row.question_lang.ja_jp,
                &row.question_lang.pt_pt,
                &row.question_lang.it_it,
                &row.question_lang.unknown_12,
                &row.question_lang.unknown_13,
                &row.question_lang.unknown_14,
                &row.question_lang.unknown_15,
                &row.question_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "ItemSubClass.dbc" => {
            let data = item_sub_class::ItemSubClass::read(file_contents)?;
            let (table, insert) = ItemSubClass();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.class_id,
                row.sub_class_id,
                row.prerequisite_proficiency,
                row.postrequisite_proficiency,
                row.flags,
                row.display_flags,
                row.weapon_parry_seq,
                row.weapon_ready_seq,
                row.weapon_attack_seq,
                row.weapon_swing_size,
                &row.display_name_lang.en_gb,
                &row.display_name_lang.ko_kr,
                &row.display_name_lang.fr_fr,
                &row.display_name_lang.de_de,
                &row.display_name_lang.en_cn,
                &row.display_name_lang.en_tw,
                &row.display_name_lang.es_es,
                &row.display_name_lang.es_mx,
                &row.display_name_lang.ru_ru,
                &row.display_name_lang.ja_jp,
                &row.display_name_lang.pt_pt,
                &row.display_name_lang.it_it,
                &row.display_name_lang.unknown_12,
                &row.display_name_lang.unknown_13,
                &row.display_name_lang.unknown_14,
                &row.display_name_lang.unknown_15,
                &row.display_name_lang.flags,
                &row.verbose_name_lang.en_gb,
                &row.verbose_name_lang.ko_kr,
                &row.verbose_name_lang.fr_fr,
                &row.verbose_name_lang.de_de,
                &row.verbose_name_lang.en_cn,
                &row.verbose_name_lang.en_tw,
                &row.verbose_name_lang.es_es,
                &row.verbose_name_lang.es_mx,
                &row.verbose_name_lang.ru_ru,
                &row.verbose_name_lang.ja_jp,
                &row.verbose_name_lang.pt_pt,
                &row.verbose_name_lang.it_it,
                &row.verbose_name_lang.unknown_12,
                &row.verbose_name_lang.unknown_13,
                &row.verbose_name_lang.unknown_14,
                &row.verbose_name_lang.unknown_15,
                &row.verbose_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "FactionTemplate.dbc" => {
            let data = faction_template::FactionTemplate::read(file_contents)?;
            let (table, insert) = FactionTemplate();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.faction.id,
                row.flags,
                row.faction_group.id,
                row.friend_group,
                row.enemy_group,
                row.enemies[0],
                row.enemies[1],
                row.enemies[2],
                row.enemies[3],
                row.friend[0],
                row.friend[1],
                row.friend[2],
                row.friend[3],
                ])?;
            }
            tx.commit()?;
        }
        "SkillLineCategory.dbc" => {
            let data = skill_line_category::SkillLineCategory::read(file_contents)?;
            let (table, insert) = SkillLineCategory();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.sort_index,
                ])?;
            }
            tx.commit()?;
        }
        "CharSections.dbc" => {
            let data = char_sections::CharSections::read(file_contents)?;
            let (table, insert) = CharSections();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race_id.id,
                row.sex_id,
                row.base_section,
                row.texture_name[0],
                row.texture_name[1],
                row.texture_name[2],
                row.flags,
                row.variation_index,
                row.color_index,
                ])?;
            }
            tx.commit()?;
        }
        "ItemVisuals.dbc" => {
            let data = item_visuals::ItemVisuals::read(file_contents)?;
            let (table, insert) = ItemVisuals();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.slot[0],
                row.slot[1],
                row.slot[2],
                row.slot[3],
                row.slot[4],
                ])?;
            }
            tx.commit()?;
        }
        "TalentTab.dbc" => {
            let data = talent_tab::TalentTab::read(file_contents)?;
            let (table, insert) = TalentTab();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.spell_icon_id.id,
                row.race_mask,
                row.class_mask,
                row.category_enum_id,
                row.order_index,
                &row.background_file,
                ])?;
            }
            tx.commit()?;
        }
        "Movie.dbc" => {
            let data = movie::Movie::read(file_contents)?;
            let (table, insert) = Movie();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.filename,
                row.volume,
                ])?;
            }
            tx.commit()?;
        }
        "GMSurveyAnswers.dbc" => {
            let data = gm_survey_answers::GMSurveyAnswers::read(file_contents)?;
            let (table, insert) = GMSurveyAnswers();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sort_index,
                row.g_m_survey_question_id.id,
                &row.answer_lang.en_gb,
                &row.answer_lang.ko_kr,
                &row.answer_lang.fr_fr,
                &row.answer_lang.de_de,
                &row.answer_lang.en_cn,
                &row.answer_lang.en_tw,
                &row.answer_lang.es_es,
                &row.answer_lang.es_mx,
                &row.answer_lang.ru_ru,
                &row.answer_lang.ja_jp,
                &row.answer_lang.pt_pt,
                &row.answer_lang.it_it,
                &row.answer_lang.unknown_12,
                &row.answer_lang.unknown_13,
                &row.answer_lang.unknown_14,
                &row.answer_lang.unknown_15,
                &row.answer_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "MovieFileData.dbc" => {
            let data = movie_file_data::MovieFileData::read(file_contents)?;
            let (table, insert) = MovieFileData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.file_data_id.id,
                row.resolution,
                ])?;
            }
            tx.commit()?;
        }
        "SpellItemEnchantmentCondition.dbc" => {
            let data = spell_item_enchantment_condition::SpellItemEnchantmentCondition::read(file_contents)?;
            let (table, insert) = SpellItemEnchantmentCondition();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.lt_operand_type[0],
                row.lt_operand_type[1],
                row.lt_operand_type[2],
                row.lt_operand_type[3],
                row.lt_operand_type[4],
                row.lt_operand[0],
                row.lt_operand[1],
                row.lt_operand[2],
                row.lt_operand[3],
                row.lt_operand[4],
                row.operator[0],
                row.operator[1],
                row.operator[2],
                row.operator[3],
                row.operator[4],
                row.rt_operand_type[0],
                row.rt_operand_type[1],
                row.rt_operand_type[2],
                row.rt_operand_type[3],
                row.rt_operand_type[4],
                row.rt_operand[0],
                row.rt_operand[1],
                row.rt_operand[2],
                row.rt_operand[3],
                row.rt_operand[4],
                row.logic[0],
                row.logic[1],
                row.logic[2],
                row.logic[3],
                row.logic[4],
                ])?;
            }
            tx.commit()?;
        }
        "AreaTable.dbc" => {
            let data = area_table::AreaTable::read(file_contents)?;
            let (table, insert) = AreaTable();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.continent_id.id,
                row.parent_area_id.id,
                row.area_bit,
                row.flags,
                row.sound_provider_pref.id,
                row.sound_provider_pref_underwater.id,
                row.ambience_id.id,
                row.zone_music.id,
                row.intro_sound.id,
                row.exploration_level,
                &row.area_name_lang.en_gb,
                &row.area_name_lang.ko_kr,
                &row.area_name_lang.fr_fr,
                &row.area_name_lang.de_de,
                &row.area_name_lang.en_cn,
                &row.area_name_lang.en_tw,
                &row.area_name_lang.es_es,
                &row.area_name_lang.es_mx,
                &row.area_name_lang.ru_ru,
                &row.area_name_lang.ja_jp,
                &row.area_name_lang.pt_pt,
                &row.area_name_lang.it_it,
                &row.area_name_lang.unknown_12,
                &row.area_name_lang.unknown_13,
                &row.area_name_lang.unknown_14,
                &row.area_name_lang.unknown_15,
                &row.area_name_lang.flags,
                row.faction_group_mask,
                row.liquid_type_id[0],
                row.liquid_type_id[1],
                row.liquid_type_id[2],
                row.liquid_type_id[3],
                row.min_elevation,
                row.ambient_multiplier,
                row.light_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "TaxiNodes.dbc" => {
            let data = taxi_nodes::TaxiNodes::read(file_contents)?;
            let (table, insert) = TaxiNodes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.continent_id.id,
                row.pos[0],
                row.pos[1],
                row.pos[2],
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.mount_creature_id[0],
                row.mount_creature_id[1],
                ])?;
            }
            tx.commit()?;
        }
        "TransportPhysics.dbc" => {
            let data = transport_physics::TransportPhysics::read(file_contents)?;
            let (table, insert) = TransportPhysics();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.wave_amp,
                row.wave_time_scale,
                row.roll_amp,
                row.roll_time_scale,
                row.pitch_amp,
                row.pitch_time_scale,
                row.max_bank,
                row.max_bank_turn_speed,
                row.speed_damp_thresh,
                row.speed_damp,
                ])?;
            }
            tx.commit()?;
        }
        "ZoneIntroMusicTable.dbc" => {
            let data = zone_intro_music_table::ZoneIntroMusicTable::read(file_contents)?;
            let (table, insert) = ZoneIntroMusicTable();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.sound_id.id,
                row.priority,
                row.min_delay_minutes,
                ])?;
            }
            tx.commit()?;
        }
        "gtChanceToSpellCritBase.dbc" => {
            let data = gt_chance_to_spell_crit_base::gtChanceToSpellCritBase::read(file_contents)?;
            let (table, insert) = gtChanceToSpellCritBase();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "ItemVisualEffects.dbc" => {
            let data = item_visual_effects::ItemVisualEffects::read(file_contents)?;
            let (table, insert) = ItemVisualEffects();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model,
                ])?;
            }
            tx.commit()?;
        }
        "SpellShapeshiftForm.dbc" => {
            let data = spell_shapeshift_form::SpellShapeshiftForm::read(file_contents)?;
            let (table, insert) = SpellShapeshiftForm();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.bonus_action_bar,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.flags,
                row.creature_type.id,
                row.attack_icon_id.id,
                row.combat_round_time,
                row.creature_display_id[0],
                row.creature_display_id[1],
                row.creature_display_id[2],
                row.creature_display_id[3],
                row.preset_spell_id[0],
                row.preset_spell_id[1],
                row.preset_spell_id[2],
                row.preset_spell_id[3],
                row.preset_spell_id[4],
                row.preset_spell_id[5],
                row.preset_spell_id[6],
                row.preset_spell_id[7],
                ])?;
            }
            tx.commit()?;
        }
        "SpellEffectCameraShakes.dbc" => {
            let data = spell_effect_camera_shakes::SpellEffectCameraShakes::read(file_contents)?;
            let (table, insert) = SpellEffectCameraShakes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.camera_shake[0],
                row.camera_shake[1],
                row.camera_shake[2],
                ])?;
            }
            tx.commit()?;
        }
        "FootstepTerrainLookup.dbc" => {
            let data = footstep_terrain_lookup::FootstepTerrainLookup::read(file_contents)?;
            let (table, insert) = FootstepTerrainLookup();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.creature_footstep_id,
                row.terrain_sound_id,
                row.sound_id.id,
                row.sound_id_splash.id,
                ])?;
            }
            tx.commit()?;
        }
        "SpellVisual.dbc" => {
            let data = spell_visual::SpellVisual::read(file_contents)?;
            let (table, insert) = SpellVisual();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.precast_kit,
                row.cast_kit,
                row.impact_kit,
                row.state_kit,
                row.state_done_kit,
                row.channel_kit,
                row.has_missile,
                row.missile_model,
                row.missile_path_type,
                row.missile_destination_attachment,
                row.missile_sound,
                row.anim_event_sound_id.id,
                row.flags,
                row.caster_impact_kit,
                row.target_impact_kit,
                row.missile_attachment,
                row.missile_follow_ground_height,
                row.missile_follow_ground_drop_speed,
                row.missile_follow_ground_approach,
                row.missile_follow_ground_flags,
                row.missile_motion,
                row.missile_targeting_kit.id,
                row.instant_area_kit,
                row.impact_area_kit,
                row.persistent_area_kit,
                row.missile_cast_offset[0],
                row.missile_cast_offset[1],
                row.missile_cast_offset[2],
                row.missile_impact_offset[0],
                row.missile_impact_offset[1],
                row.missile_impact_offset[2],
                ])?;
            }
            tx.commit()?;
        }
        "VehicleSeat.dbc" => {
            let data = vehicle_seat::VehicleSeat::read(file_contents)?;
            let (table, insert) = VehicleSeat();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.field_3_3_5_12213_001,
                row.attachment_id,
                row.attachment_offset[0],
                row.attachment_offset[1],
                row.attachment_offset[2],
                row.field_3_3_5_12213_004,
                row.enter_speed,
                row.enter_gravity,
                row.enter_min_duration,
                row.enter_max_duration,
                row.enter_min_arc_height,
                row.enter_max_arc_height,
                row.enter_anim_start,
                row.enter_anim_loop,
                row.ride_anim_start,
                row.ride_anim_loop,
                row.ride_upper_anim_start,
                row.ride_upper_anim_loop,
                row.field_3_3_5_12213_017,
                row.exit_speed,
                row.exit_gravity,
                row.exit_min_duration,
                row.exit_max_duration,
                row.exit_min_arc_height,
                row.exit_max_arc_height,
                row.exit_anim_start,
                row.exit_anim_loop,
                row.exit_anim_end,
                row.field_3_3_5_12213_027,
                row.passenger_pitch,
                row.field_3_3_5_12213_029,
                row.passenger_attachment_id,
                row.vehicle_enter_anim,
                row.vehicle_exit_anim,
                row.vehicle_ride_anim_loop,
                row.field_3_3_5_12213_034,
                row.vehicle_exit_anim_bone,
                row.vehicle_enter_anim_bone,
                row.field_3_3_5_12213_037,
                row.field_3_3_5_12213_038,
                row.vehicle_ability_display,
                row.enter_u_i_sound_id.id,
                row.field_3_3_5_12213_041,
                row.ui_skin,
                row.field_3_3_5_12213_043,
                row.field_3_3_5_12213_044,
                row.field_3_3_5_12213_045,
                row.field_3_3_5_12213_046,
                row.field_3_3_5_12213_047,
                row.field_3_3_5_12213_048,
                row.field_3_3_5_12213_049,
                row.field_3_3_5_12213_050,
                row.field_3_3_5_12213_051,
                row.field_3_3_5_12213_052,
                row.field_3_3_5_12213_053,
                row.field_3_3_5_12213_054,
                row.field_3_3_5_12213_055,
                ])?;
            }
            tx.commit()?;
        }
        "gtChanceToMeleeCritBase.dbc" => {
            let data = gt_chance_to_melee_crit_base::gtChanceToMeleeCritBase::read(file_contents)?;
            let (table, insert) = gtChanceToMeleeCritBase();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "Cfg_Configs.dbc" => {
            let data = cfg_configs::Cfg_Configs::read(file_contents)?;
            let (table, insert) = Cfg_Configs();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.realm_type,
                row.player_killing_allowed,
                row.roleplaying,
                ])?;
            }
            tx.commit()?;
        }
        "ServerMessages.dbc" => {
            let data = server_messages::ServerMessages::read(file_contents)?;
            let (table, insert) = ServerMessages();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text_lang.en_gb,
                &row.text_lang.ko_kr,
                &row.text_lang.fr_fr,
                &row.text_lang.de_de,
                &row.text_lang.en_cn,
                &row.text_lang.en_tw,
                &row.text_lang.es_es,
                &row.text_lang.es_mx,
                &row.text_lang.ru_ru,
                &row.text_lang.ja_jp,
                &row.text_lang.pt_pt,
                &row.text_lang.it_it,
                &row.text_lang.unknown_12,
                &row.text_lang.unknown_13,
                &row.text_lang.unknown_14,
                &row.text_lang.unknown_15,
                &row.text_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "Package.dbc" => {
            let data = package::Package::read(file_contents)?;
            let (table, insert) = Package();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.icon,
                row.cost,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SpellRadius.dbc" => {
            let data = spell_radius::SpellRadius::read(file_contents)?;
            let (table, insert) = SpellRadius();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.radius,
                row.radius_per_level,
                row.radius_max,
                ])?;
            }
            tx.commit()?;
        }
        "Achievement.dbc" => {
            let data = achievement::Achievement::read(file_contents)?;
            let (table, insert) = Achievement();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.faction.id,
                row.instance_id.id,
                row.supercedes.id,
                &row.title_lang.en_gb,
                &row.title_lang.ko_kr,
                &row.title_lang.fr_fr,
                &row.title_lang.de_de,
                &row.title_lang.en_cn,
                &row.title_lang.en_tw,
                &row.title_lang.es_es,
                &row.title_lang.es_mx,
                &row.title_lang.ru_ru,
                &row.title_lang.ja_jp,
                &row.title_lang.pt_pt,
                &row.title_lang.it_it,
                &row.title_lang.unknown_12,
                &row.title_lang.unknown_13,
                &row.title_lang.unknown_14,
                &row.title_lang.unknown_15,
                &row.title_lang.flags,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                row.category.id,
                row.points,
                row.ui_order,
                row.flags,
                row.icon_id.id,
                &row.reward_lang.en_gb,
                &row.reward_lang.ko_kr,
                &row.reward_lang.fr_fr,
                &row.reward_lang.de_de,
                &row.reward_lang.en_cn,
                &row.reward_lang.en_tw,
                &row.reward_lang.es_es,
                &row.reward_lang.es_mx,
                &row.reward_lang.ru_ru,
                &row.reward_lang.ja_jp,
                &row.reward_lang.pt_pt,
                &row.reward_lang.it_it,
                &row.reward_lang.unknown_12,
                &row.reward_lang.unknown_13,
                &row.reward_lang.unknown_14,
                &row.reward_lang.unknown_15,
                &row.reward_lang.flags,
                row.minimum_criteria,
                row.shares_criteria.id,
                ])?;
            }
            tx.commit()?;
        }
        "SoundEntries.dbc" => {
            let data = sound_entries::SoundEntries::read(file_contents)?;
            let (table, insert) = SoundEntries();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_type,
                &row.name,
                row.file[0],
                row.file[1],
                row.file[2],
                row.file[3],
                row.file[4],
                row.file[5],
                row.file[6],
                row.file[7],
                row.file[8],
                row.file[9],
                row.freq[0],
                row.freq[1],
                row.freq[2],
                row.freq[3],
                row.freq[4],
                row.freq[5],
                row.freq[6],
                row.freq[7],
                row.freq[8],
                row.freq[9],
                &row.directory_base,
                row.volume_float,
                row.flags,
                row.min_distance,
                row.distance_cutoff,
                row.e_a_x_def,
                row.sound_entries_advanced_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "EnvironmentalDamage.dbc" => {
            let data = environmental_damage::EnvironmentalDamage::read(file_contents)?;
            let (table, insert) = EnvironmentalDamage();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.enum_id,
                row.visualkit_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "DestructibleModelData.dbc" => {
            let data = destructible_model_data::DestructibleModelData::read(file_contents)?;
            let (table, insert) = DestructibleModelData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.state0_impact_effect_doodad_set,
                row.state0_ambient_doodad_set,
                row.state1_w_m_o,
                row.state1_destruction_doodad_set,
                row.state1_impact_effect_doodad_set,
                row.state1_ambient_doodad_set,
                row.state2_w_m_o,
                row.state2_destruction_doodad_set,
                row.state2_impact_effect_doodad_set,
                row.state2_ambient_doodad_set,
                row.state3_w_m_o,
                row.state3_init_doodad_set,
                row.state3_ambient_doodad_set,
                row.eject_direction,
                row.repair_ground_fx,
                row.do_not_highlight,
                row.heal_effect,
                row.heal_effect_speed,
                ])?;
            }
            tx.commit()?;
        }
        "WeaponImpactSounds.dbc" => {
            let data = weapon_impact_sounds::WeaponImpactSounds::read(file_contents)?;
            let (table, insert) = WeaponImpactSounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.weapon_sub_class_id,
                row.parry_sound_type,
                row.impact_sound_id[0],
                row.impact_sound_id[1],
                row.impact_sound_id[2],
                row.impact_sound_id[3],
                row.impact_sound_id[4],
                row.impact_sound_id[5],
                row.impact_sound_id[6],
                row.impact_sound_id[7],
                row.impact_sound_id[8],
                row.impact_sound_id[9],
                row.crit_impact_sound_id[0],
                row.crit_impact_sound_id[1],
                row.crit_impact_sound_id[2],
                row.crit_impact_sound_id[3],
                row.crit_impact_sound_id[4],
                row.crit_impact_sound_id[5],
                row.crit_impact_sound_id[6],
                row.crit_impact_sound_id[7],
                row.crit_impact_sound_id[8],
                row.crit_impact_sound_id[9],
                ])?;
            }
            tx.commit()?;
        }
        "TaxiPath.dbc" => {
            let data = taxi_path::TaxiPath::read(file_contents)?;
            let (table, insert) = TaxiPath();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.from_taxi_node.id,
                row.to_taxi_node.id,
                row.cost,
                ])?;
            }
            tx.commit()?;
        }
        "NPCSounds.dbc" => {
            let data = npc_sounds::NPCSounds::read(file_contents)?;
            let (table, insert) = NPCSounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_id[0],
                row.sound_id[1],
                row.sound_id[2],
                row.sound_id[3],
                ])?;
            }
            tx.commit()?;
        }
        "VehicleUIIndSeat.dbc" => {
            let data = vehicle_ui_ind_seat::VehicleUIIndSeat::read(file_contents)?;
            let (table, insert) = VehicleUIIndSeat();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.vehicle_u_i_indicator_id.id,
                row.virtual_seat_index,
                row.x_pos,
                row.y_pos,
                ])?;
            }
            tx.commit()?;
        }
        "NameGen.dbc" => {
            let data = name_gen::NameGen::read(file_contents)?;
            let (table, insert) = NameGen();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.race_id.id,
                row.sex,
                ])?;
            }
            tx.commit()?;
        }
        "LockType.dbc" => {
            let data = lock_type::LockType::read(file_contents)?;
            let (table, insert) = LockType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.resource_name_lang.en_gb,
                &row.resource_name_lang.ko_kr,
                &row.resource_name_lang.fr_fr,
                &row.resource_name_lang.de_de,
                &row.resource_name_lang.en_cn,
                &row.resource_name_lang.en_tw,
                &row.resource_name_lang.es_es,
                &row.resource_name_lang.es_mx,
                &row.resource_name_lang.ru_ru,
                &row.resource_name_lang.ja_jp,
                &row.resource_name_lang.pt_pt,
                &row.resource_name_lang.it_it,
                &row.resource_name_lang.unknown_12,
                &row.resource_name_lang.unknown_13,
                &row.resource_name_lang.unknown_14,
                &row.resource_name_lang.unknown_15,
                &row.resource_name_lang.flags,
                &row.verb_lang.en_gb,
                &row.verb_lang.ko_kr,
                &row.verb_lang.fr_fr,
                &row.verb_lang.de_de,
                &row.verb_lang.en_cn,
                &row.verb_lang.en_tw,
                &row.verb_lang.es_es,
                &row.verb_lang.es_mx,
                &row.verb_lang.ru_ru,
                &row.verb_lang.ja_jp,
                &row.verb_lang.pt_pt,
                &row.verb_lang.it_it,
                &row.verb_lang.unknown_12,
                &row.verb_lang.unknown_13,
                &row.verb_lang.unknown_14,
                &row.verb_lang.unknown_15,
                &row.verb_lang.flags,
                &row.cursor_name,
                ])?;
            }
            tx.commit()?;
        }
        "PowerDisplay.dbc" => {
            let data = power_display::PowerDisplay::read(file_contents)?;
            let (table, insert) = PowerDisplay();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.actual_type,
                &row.global_string_base_tag,
                row.red,
                row.green,
                row.blue,
                ])?;
            }
            tx.commit()?;
        }
        "HolidayNames.dbc" => {
            let data = holiday_names::HolidayNames::read(file_contents)?;
            let (table, insert) = HolidayNames();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "CharacterFacialHairStyles.dbc" => {
            let data = character_facial_hair_styles::CharacterFacialHairStyles::read(file_contents)?;
            let (table, insert) = CharacterFacialHairStyles();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.race_id.id,
                row.sex_id,
                row.variation_id,
                row.geoset[0],
                row.geoset[1],
                row.geoset[2],
                row.geoset[3],
                row.geoset[4],
                ])?;
            }
            tx.commit()?;
        }
        "CinematicSequences.dbc" => {
            let data = cinematic_sequences::CinematicSequences::read(file_contents)?;
            let (table, insert) = CinematicSequences();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_id.id,
                row.camera[0],
                row.camera[1],
                row.camera[2],
                row.camera[3],
                row.camera[4],
                row.camera[5],
                row.camera[6],
                row.camera[7],
                ])?;
            }
            tx.commit()?;
        }
        "WowError_Strings.dbc" => {
            let data = wow_error_strings::WowError_Strings::read(file_contents)?;
            let (table, insert) = WowError_Strings();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SpellDescriptionVariables.dbc" => {
            let data = spell_description_variables::SpellDescriptionVariables::read(file_contents)?;
            let (table, insert) = SpellDescriptionVariables();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.variables,
                ])?;
            }
            tx.commit()?;
        }
        "MovieVariation.dbc" => {
            let data = movie_variation::MovieVariation::read(file_contents)?;
            let (table, insert) = MovieVariation();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.movie_id.id,
                row.file_data_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "ObjectEffectPackage.dbc" => {
            let data = object_effect_package::ObjectEffectPackage::read(file_contents)?;
            let (table, insert) = ObjectEffectPackage();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
            tx.commit()?;
        }
        "SpellItemEnchantment.dbc" => {
            let data = spell_item_enchantment::SpellItemEnchantment::read(file_contents)?;
            let (table, insert) = SpellItemEnchantment();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.charges,
                row.effect[0],
                row.effect[1],
                row.effect[2],
                row.effect_points_min[0],
                row.effect_points_min[1],
                row.effect_points_min[2],
                row.effect_points_max[0],
                row.effect_points_max[1],
                row.effect_points_max[2],
                row.effect_arg[0],
                row.effect_arg[1],
                row.effect_arg[2],
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.item_visual.id,
                row.flags,
                row.src_item_id,
                row.condition_id.id,
                row.required_skill_id.id,
                row.required_skill_rank,
                row.min_level,
                ])?;
            }
            tx.commit()?;
        }
        "ItemExtendedCost.dbc" => {
            let data = item_extended_cost::ItemExtendedCost::read(file_contents)?;
            let (table, insert) = ItemExtendedCost();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.honor_points,
                row.arena_points,
                row.arena_bracket,
                row.item_id[0],
                row.item_id[1],
                row.item_id[2],
                row.item_id[3],
                row.item_id[4],
                row.item_count[0],
                row.item_count[1],
                row.item_count[2],
                row.item_count[3],
                row.item_count[4],
                row.required_arena_rating,
                row.item_purchase_group.id,
                ])?;
            }
            tx.commit()?;
        }
        "ItemClass.dbc" => {
            let data = item_class::ItemClass::read(file_contents)?;
            let (table, insert) = ItemClass();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.class_id,
                row.subclass_map_id,
                row.flags,
                &row.class_name_lang.en_gb,
                &row.class_name_lang.ko_kr,
                &row.class_name_lang.fr_fr,
                &row.class_name_lang.de_de,
                &row.class_name_lang.en_cn,
                &row.class_name_lang.en_tw,
                &row.class_name_lang.es_es,
                &row.class_name_lang.es_mx,
                &row.class_name_lang.ru_ru,
                &row.class_name_lang.ja_jp,
                &row.class_name_lang.pt_pt,
                &row.class_name_lang.it_it,
                &row.class_name_lang.unknown_12,
                &row.class_name_lang.unknown_13,
                &row.class_name_lang.unknown_14,
                &row.class_name_lang.unknown_15,
                &row.class_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "ItemCondExtCosts.dbc" => {
            let data = item_cond_ext_costs::ItemCondExtCosts::read(file_contents)?;
            let (table, insert) = ItemCondExtCosts();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cond_extended_cost,
                row.item_extended_cost_entry.id,
                row.arena_season,
                ])?;
            }
            tx.commit()?;
        }
        "ItemLimitCategory.dbc" => {
            let data = item_limit_category::ItemLimitCategory::read(file_contents)?;
            let (table, insert) = ItemLimitCategory();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.quantity,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SpellCategory.dbc" => {
            let data = spell_category::SpellCategory::read(file_contents)?;
            let (table, insert) = SpellCategory();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "gtOCTRegenHP.dbc" => {
            let data = gt_oct_regen_hp::gtOCTRegenHP::read(file_contents)?;
            let (table, insert) = gtOCTRegenHP();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "AreaGroup.dbc" => {
            let data = area_group::AreaGroup::read(file_contents)?;
            let (table, insert) = AreaGroup();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.area_id[0],
                row.area_id[1],
                row.area_id[2],
                row.area_id[3],
                row.area_id[4],
                row.area_id[5],
                row.next_area_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "TeamContributionPoints.dbc" => {
            let data = team_contribution_points::TeamContributionPoints::read(file_contents)?;
            let (table, insert) = TeamContributionPoints();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "WorldMapArea.dbc" => {
            let data = world_map_area::WorldMapArea::read(file_contents)?;
            let (table, insert) = WorldMapArea();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.area_id.id,
                &row.area_name,
                row.loc_left,
                row.loc_right,
                row.loc_top,
                row.loc_bottom,
                row.display_map_id.id,
                row.default_dungeon_floor,
                row.parent_world_map_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "Lock.dbc" => {
            let data = lock::Lock::read(file_contents)?;
            let (table, insert) = Lock();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ty[0],
                row.ty[1],
                row.ty[2],
                row.ty[3],
                row.ty[4],
                row.ty[5],
                row.ty[6],
                row.ty[7],
                row.index[0],
                row.index[1],
                row.index[2],
                row.index[3],
                row.index[4],
                row.index[5],
                row.index[6],
                row.index[7],
                row.skill[0],
                row.skill[1],
                row.skill[2],
                row.skill[3],
                row.skill[4],
                row.skill[5],
                row.skill[6],
                row.skill[7],
                row.action[0],
                row.action[1],
                row.action[2],
                row.action[3],
                row.action[4],
                row.action[5],
                row.action[6],
                row.action[7],
                ])?;
            }
            tx.commit()?;
        }
        "ScalingStatValues.dbc" => {
            let data = scaling_stat_values::ScalingStatValues::read(file_contents)?;
            let (table, insert) = ScalingStatValues();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.charlevel,
                row.shoulder_budget,
                row.trinket_budget,
                row.weapon_budget1_h,
                row.ranged_budget,
                row.cloth_shoulder_armor,
                row.leather_shoulder_armor,
                row.mail_shoulder_armor,
                row.plate_shoulder_armor,
                row.weapon_d_p_s1_h,
                row.weapon_d_p_s2_h,
                row.spellcaster_d_p_s1_h,
                row.spellcaster_d_p_s2_h,
                row.ranged_d_p_s,
                row.wand_d_p_s,
                row.spell_power,
                row.primary_budget,
                row.tertiary_budget,
                row.cloth_cloak_armor,
                row.cloth_chest_armor,
                row.leather_chest_armor,
                row.mail_chest_armor,
                row.plate_chest_armor,
                ])?;
            }
            tx.commit()?;
        }
        "SpellCastTimes.dbc" => {
            let data = spell_cast_times::SpellCastTimes::read(file_contents)?;
            let (table, insert) = SpellCastTimes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.base,
                row.per_level,
                row.minimum,
                ])?;
            }
            tx.commit()?;
        }
        "FootprintTextures.dbc" => {
            let data = footprint_textures::FootprintTextures::read(file_contents)?;
            let (table, insert) = FootprintTextures();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.footstep_filename,
                ])?;
            }
            tx.commit()?;
        }
        "GameObjectArtKit.dbc" => {
            let data = game_object_art_kit::GameObjectArtKit::read(file_contents)?;
            let (table, insert) = GameObjectArtKit();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.texture_variation[0],
                row.texture_variation[1],
                row.texture_variation[2],
                row.attach_model[0],
                row.attach_model[1],
                row.attach_model[2],
                row.attach_model[3],
                ])?;
            }
            tx.commit()?;
        }
        "SpellDuration.dbc" => {
            let data = spell_duration::SpellDuration::read(file_contents)?;
            let (table, insert) = SpellDuration();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.duration,
                row.duration_per_level,
                row.max_duration,
                ])?;
            }
            tx.commit()?;
        }
        "QuestSort.dbc" => {
            let data = quest_sort::QuestSort::read(file_contents)?;
            let (table, insert) = QuestSort();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.sort_name_lang.en_gb,
                &row.sort_name_lang.ko_kr,
                &row.sort_name_lang.fr_fr,
                &row.sort_name_lang.de_de,
                &row.sort_name_lang.en_cn,
                &row.sort_name_lang.en_tw,
                &row.sort_name_lang.es_es,
                &row.sort_name_lang.es_mx,
                &row.sort_name_lang.ru_ru,
                &row.sort_name_lang.ja_jp,
                &row.sort_name_lang.pt_pt,
                &row.sort_name_lang.it_it,
                &row.sort_name_lang.unknown_12,
                &row.sort_name_lang.unknown_13,
                &row.sort_name_lang.unknown_14,
                &row.sort_name_lang.unknown_15,
                &row.sort_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "UISoundLookups.dbc" => {
            let data = ui_sound_lookups::UISoundLookups::read(file_contents)?;
            let (table, insert) = UISoundLookups();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_id.id,
                &row.sound_name,
                ])?;
            }
            tx.commit()?;
        }
        "GroundEffectTexture.dbc" => {
            let data = ground_effect_texture::GroundEffectTexture::read(file_contents)?;
            let (table, insert) = GroundEffectTexture();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.doodad_id[0],
                row.doodad_id[1],
                row.doodad_id[2],
                row.doodad_id[3],
                row.doodad_weight[0],
                row.doodad_weight[1],
                row.doodad_weight[2],
                row.doodad_weight[3],
                row.density,
                row.sound,
                ])?;
            }
            tx.commit()?;
        }
        "ChatChannels.dbc" => {
            let data = chat_channels::ChatChannels::read(file_contents)?;
            let (table, insert) = ChatChannels();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.faction_group,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.shortcut_lang.en_gb,
                &row.shortcut_lang.ko_kr,
                &row.shortcut_lang.fr_fr,
                &row.shortcut_lang.de_de,
                &row.shortcut_lang.en_cn,
                &row.shortcut_lang.en_tw,
                &row.shortcut_lang.es_es,
                &row.shortcut_lang.es_mx,
                &row.shortcut_lang.ru_ru,
                &row.shortcut_lang.ja_jp,
                &row.shortcut_lang.pt_pt,
                &row.shortcut_lang.it_it,
                &row.shortcut_lang.unknown_12,
                &row.shortcut_lang.unknown_13,
                &row.shortcut_lang.unknown_14,
                &row.shortcut_lang.unknown_15,
                &row.shortcut_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "CurrencyTypes.dbc" => {
            let data = currency_types::CurrencyTypes::read(file_contents)?;
            let (table, insert) = CurrencyTypes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_id.id,
                row.category_id.id,
                row.bit_index,
                ])?;
            }
            tx.commit()?;
        }
        "CurrencyCategory.dbc" => {
            let data = currency_category::CurrencyCategory::read(file_contents)?;
            let (table, insert) = CurrencyCategory();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "EmotesTextData.dbc" => {
            let data = emotes_text_data::EmotesTextData::read(file_contents)?;
            let (table, insert) = EmotesTextData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text_lang.en_gb,
                &row.text_lang.ko_kr,
                &row.text_lang.fr_fr,
                &row.text_lang.de_de,
                &row.text_lang.en_cn,
                &row.text_lang.en_tw,
                &row.text_lang.es_es,
                &row.text_lang.es_mx,
                &row.text_lang.ru_ru,
                &row.text_lang.ja_jp,
                &row.text_lang.pt_pt,
                &row.text_lang.it_it,
                &row.text_lang.unknown_12,
                &row.text_lang.unknown_13,
                &row.text_lang.unknown_14,
                &row.text_lang.unknown_15,
                &row.text_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "EmotesTextSound.dbc" => {
            let data = emotes_text_sound::EmotesTextSound::read(file_contents)?;
            let (table, insert) = EmotesTextSound();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.emotes_text_id.id,
                row.race_id.id,
                row.sex_id,
                row.sound_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "Light.dbc" => {
            let data = light::Light::read(file_contents)?;
            let (table, insert) = Light();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.continent_id.id,
                row.game_coords[0],
                row.game_coords[1],
                row.game_coords[2],
                row.game_falloff_start,
                row.game_falloff_end,
                row.light_params_id[0],
                row.light_params_id[1],
                row.light_params_id[2],
                row.light_params_id[3],
                row.light_params_id[4],
                row.light_params_id[5],
                row.light_params_id[6],
                row.light_params_id[7],
                ])?;
            }
            tx.commit()?;
        }
        "CreatureFamily.dbc" => {
            let data = creature_family::CreatureFamily::read(file_contents)?;
            let (table, insert) = CreatureFamily();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.min_scale,
                row.min_scale_level,
                row.max_scale,
                row.max_scale_level,
                row.skill_line[0],
                row.skill_line[1],
                row.pet_food_mask,
                row.pet_talent_type,
                row.category_enum_id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.icon_file,
                ])?;
            }
            tx.commit()?;
        }
        "Talent.dbc" => {
            let data = talent::Talent::read(file_contents)?;
            let (table, insert) = Talent();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.tab_id,
                row.tier_id,
                row.column_index,
                row.spell_rank[0],
                row.spell_rank[1],
                row.spell_rank[2],
                row.spell_rank[3],
                row.spell_rank[4],
                row.spell_rank[5],
                row.spell_rank[6],
                row.spell_rank[7],
                row.spell_rank[8],
                row.prereq_talent[0],
                row.prereq_talent[1],
                row.prereq_talent[2],
                row.prereq_rank[0],
                row.prereq_rank[1],
                row.prereq_rank[2],
                row.flags,
                row.required_spell_id.id,
                row.category_mask[0],
                row.category_mask[1],
                ])?;
            }
            tx.commit()?;
        }
        "ItemRandomProperties.dbc" => {
            let data = item_random_properties::ItemRandomProperties::read(file_contents)?;
            let (table, insert) = ItemRandomProperties();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.enchantment[0],
                row.enchantment[1],
                row.enchantment[2],
                row.enchantment[3],
                row.enchantment[4],
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "DurabilityCosts.dbc" => {
            let data = durability_costs::DurabilityCosts::read(file_contents)?;
            let (table, insert) = DurabilityCosts();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.weapon_sub_class_cost[0],
                row.weapon_sub_class_cost[1],
                row.weapon_sub_class_cost[2],
                row.weapon_sub_class_cost[3],
                row.weapon_sub_class_cost[4],
                row.weapon_sub_class_cost[5],
                row.weapon_sub_class_cost[6],
                row.weapon_sub_class_cost[7],
                row.weapon_sub_class_cost[8],
                row.weapon_sub_class_cost[9],
                row.weapon_sub_class_cost[10],
                row.weapon_sub_class_cost[11],
                row.weapon_sub_class_cost[12],
                row.weapon_sub_class_cost[13],
                row.weapon_sub_class_cost[14],
                row.weapon_sub_class_cost[15],
                row.weapon_sub_class_cost[16],
                row.weapon_sub_class_cost[17],
                row.weapon_sub_class_cost[18],
                row.weapon_sub_class_cost[19],
                row.weapon_sub_class_cost[20],
                row.armor_sub_class_cost[0],
                row.armor_sub_class_cost[1],
                row.armor_sub_class_cost[2],
                row.armor_sub_class_cost[3],
                row.armor_sub_class_cost[4],
                row.armor_sub_class_cost[5],
                row.armor_sub_class_cost[6],
                row.armor_sub_class_cost[7],
                ])?;
            }
            tx.commit()?;
        }
        "SpellChainEffects.dbc" => {
            let data = spell_chain_effects::SpellChainEffects::read(file_contents)?;
            let (table, insert) = SpellChainEffects();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.avg_seg_len,
                row.width,
                row.noise_scale,
                row.tex_coord_scale,
                row.seg_duration,
                row.seg_delay,
                &row.texture,
                row.flags,
                row.joint_count,
                row.joint_offset_radius,
                row.joints_per_minor_joint,
                row.minor_joints_per_major_joint,
                row.minor_joint_scale,
                row.major_joint_scale,
                row.joint_move_speed,
                row.joint_smoothness,
                row.min_duration_between_joint_jumps,
                row.max_duration_between_joint_jumps,
                row.wave_height,
                row.wave_freq,
                row.wave_speed,
                row.min_wave_angle,
                row.max_wave_angle,
                row.min_wave_spin,
                row.max_wave_spin,
                row.arc_height,
                row.min_arc_angle,
                row.max_arc_angle,
                row.min_arc_spin,
                row.max_arc_spin,
                row.delay_between_effects,
                row.min_flicker_on_duration,
                row.max_flicker_on_duration,
                row.min_flicker_off_duration,
                row.max_flicker_off_duration,
                row.pulse_speed,
                row.pulse_on_length,
                row.pulse_fade_length,
                row.alpha,
                row.red,
                row.green,
                row.blue,
                row.blend_mode,
                &row.combo,
                row.render_layer,
                row.texture_length,
                row.wave_phase,
                ])?;
            }
            tx.commit()?;
        }
        "OverrideSpellData.dbc" => {
            let data = override_spell_data::OverrideSpellData::read(file_contents)?;
            let (table, insert) = OverrideSpellData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.spells[0],
                row.spells[1],
                row.spells[2],
                row.spells[3],
                row.spells[4],
                row.spells[5],
                row.spells[6],
                row.spells[7],
                row.spells[8],
                row.spells[9],
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SpellMissile.dbc" => {
            let data = spell_missile::SpellMissile::read(file_contents)?;
            let (table, insert) = SpellMissile();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.default_pitch_min,
                row.default_pitch_max,
                row.default_speed_min,
                row.default_speed_max,
                row.randomize_facing_min,
                row.randomize_facing_max,
                row.randomize_pitch_min,
                row.randomize_pitch_max,
                row.randomize_speed_min,
                row.randomize_speed_max,
                row.gravity,
                row.max_duration,
                row.collision_radius,
                ])?;
            }
            tx.commit()?;
        }
        "SpellRuneCost.dbc" => {
            let data = spell_rune_cost::SpellRuneCost::read(file_contents)?;
            let (table, insert) = SpellRuneCost();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.blood,
                row.unholy,
                row.frost,
                row.runic_power,
                ])?;
            }
            tx.commit()?;
        }
        "CinematicCamera.dbc" => {
            let data = cinematic_camera::CinematicCamera::read(file_contents)?;
            let (table, insert) = CinematicCamera();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model,
                row.sound_id.id,
                row.origin[0],
                row.origin[1],
                row.origin[2],
                row.origin_facing,
                ])?;
            }
            tx.commit()?;
        }
        "FileData.dbc" => {
            let data = file_data::FileData::read(file_contents)?;
            let (table, insert) = FileData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.filename,
                &row.filepath,
                ])?;
            }
            tx.commit()?;
        }
        "ObjectEffectModifier.dbc" => {
            let data = object_effect_modifier::ObjectEffectModifier::read(file_contents)?;
            let (table, insert) = ObjectEffectModifier();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.input_type,
                row.map_type,
                row.output_type,
                row.param[0],
                row.param[1],
                row.param[2],
                row.param[3],
                ])?;
            }
            tx.commit()?;
        }
        "CreatureSoundData.dbc" => {
            let data = creature_sound_data::CreatureSoundData::read(file_contents)?;
            let (table, insert) = CreatureSoundData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_exertion_id.id,
                row.sound_exertion_critical_id.id,
                row.sound_injury_id.id,
                row.sound_injury_critical_id.id,
                row.sound_injury_crushing_blow_id,
                row.sound_death_id.id,
                row.sound_stun_id.id,
                row.sound_stand_id.id,
                row.sound_footstep_id.id,
                row.sound_aggro_id.id,
                row.sound_wing_flap_id.id,
                row.sound_wing_glide_id.id,
                row.sound_alert_id.id,
                row.sound_fidget[0],
                row.sound_fidget[1],
                row.sound_fidget[2],
                row.sound_fidget[3],
                row.sound_fidget[4],
                row.custom_attack[0],
                row.custom_attack[1],
                row.custom_attack[2],
                row.custom_attack[3],
                row.n_p_c_sound_id,
                row.loop_sound_id.id,
                row.creature_impact_type,
                row.sound_jump_start_id.id,
                row.sound_jump_end_id.id,
                row.sound_pet_attack_id.id,
                row.sound_pet_order_id.id,
                row.sound_pet_dismiss_id.id,
                row.fidget_delay_seconds_min,
                row.fidget_delay_seconds_max,
                row.birth_sound_id.id,
                row.spell_cast_directed_sound_id.id,
                row.submerge_sound_id.id,
                row.submerged_sound_id.id,
                row.creature_sound_data_id_pet.id,
                ])?;
            }
            tx.commit()?;
        }
        "VideoHardware.dbc" => {
            let data = video_hardware::VideoHardware::read(file_contents)?;
            let (table, insert) = VideoHardware();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.vendor_id,
                row.device_id,
                row.farclip_idx,
                row.terrain_l_o_d_dist_idx,
                row.terrain_shadow_l_o_d,
                row.detail_doodad_density_idx,
                row.detail_doodad_alpha,
                row.animating_doodad_idx,
                row.trilinear,
                row.num_lights,
                row.specularity,
                row.water_l_o_d_idx,
                row.particle_density_idx,
                row.unit_draw_dist_idx,
                row.small_cull_dist_idx,
                row.resolution_idx,
                row.base_mip_level,
                &row.ogl_overrides,
                &row.d3d_overrides,
                row.fix_lag,
                row.multisample,
                row.atlasdisable,
                ])?;
            }
            tx.commit()?;
        }
        "PageTextMaterial.dbc" => {
            let data = page_text_material::PageTextMaterial::read(file_contents)?;
            let (table, insert) = PageTextMaterial();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
            tx.commit()?;
        }
        "ParticleColor.dbc" => {
            let data = particle_color::ParticleColor::read(file_contents)?;
            let (table, insert) = ParticleColor();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.start[0],
                row.start[1],
                row.start[2],
                row.m_id[0],
                row.m_id[1],
                row.m_id[2],
                row.end[0],
                row.end[1],
                row.end[2],
                ])?;
            }
            tx.commit()?;
        }
        "GemProperties.dbc" => {
            let data = gem_properties::GemProperties::read(file_contents)?;
            let (table, insert) = GemProperties();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.enchant_id.id,
                row.maxcount_inv,
                row.maxcount_item,
                row.ty,
                ])?;
            }
            tx.commit()?;
        }
        "CameraShakes.dbc" => {
            let data = camera_shakes::CameraShakes::read(file_contents)?;
            let (table, insert) = CameraShakes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.shake_type,
                row.direction,
                row.amplitude,
                row.frequency,
                row.duration,
                row.phase,
                row.coefficient,
                ])?;
            }
            tx.commit()?;
        }
        "GameTables.dbc" => {
            let data = game_tables::GameTables::read(file_contents)?;
            let (table, insert) = GameTables();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                &row.name,
                row.num_rows,
                row.num_columns,
                ])?;
            }
            tx.commit()?;
        }
        "Languages.dbc" => {
            let data = languages::Languages::read(file_contents)?;
            let (table, insert) = Languages();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "Item.dbc" => {
            let data = item::Item::read(file_contents)?;
            let (table, insert) = Item();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.class_id,
                row.subclass_id,
                row.sound_override_subclass_id,
                row.material.id,
                row.display_info_id,
                row.inventory_type,
                row.sheathe_type,
                ])?;
            }
            tx.commit()?;
        }
        "Resistances.dbc" => {
            let data = resistances::Resistances::read(file_contents)?;
            let (table, insert) = Resistances();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.fizzle_sound_id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "AttackAnimTypes.dbc" => {
            let data = attack_anim_types::AttackAnimTypes::read(file_contents)?;
            let (table, insert) = AttackAnimTypes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.anim_id,
                &row.anim_name,
                ])?;
            }
            tx.commit()?;
        }
        "Map.dbc" => {
            let data = map::Map::read(file_contents)?;
            let (table, insert) = Map();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.directory,
                row.instance_type,
                row.flags,
                row.p_v_p,
                &row.map_name_lang.en_gb,
                &row.map_name_lang.ko_kr,
                &row.map_name_lang.fr_fr,
                &row.map_name_lang.de_de,
                &row.map_name_lang.en_cn,
                &row.map_name_lang.en_tw,
                &row.map_name_lang.es_es,
                &row.map_name_lang.es_mx,
                &row.map_name_lang.ru_ru,
                &row.map_name_lang.ja_jp,
                &row.map_name_lang.pt_pt,
                &row.map_name_lang.it_it,
                &row.map_name_lang.unknown_12,
                &row.map_name_lang.unknown_13,
                &row.map_name_lang.unknown_14,
                &row.map_name_lang.unknown_15,
                &row.map_name_lang.flags,
                row.area_table_id.id,
                &row.map_description0_lang.en_gb,
                &row.map_description0_lang.ko_kr,
                &row.map_description0_lang.fr_fr,
                &row.map_description0_lang.de_de,
                &row.map_description0_lang.en_cn,
                &row.map_description0_lang.en_tw,
                &row.map_description0_lang.es_es,
                &row.map_description0_lang.es_mx,
                &row.map_description0_lang.ru_ru,
                &row.map_description0_lang.ja_jp,
                &row.map_description0_lang.pt_pt,
                &row.map_description0_lang.it_it,
                &row.map_description0_lang.unknown_12,
                &row.map_description0_lang.unknown_13,
                &row.map_description0_lang.unknown_14,
                &row.map_description0_lang.unknown_15,
                &row.map_description0_lang.flags,
                &row.map_description1_lang.en_gb,
                &row.map_description1_lang.ko_kr,
                &row.map_description1_lang.fr_fr,
                &row.map_description1_lang.de_de,
                &row.map_description1_lang.en_cn,
                &row.map_description1_lang.en_tw,
                &row.map_description1_lang.es_es,
                &row.map_description1_lang.es_mx,
                &row.map_description1_lang.ru_ru,
                &row.map_description1_lang.ja_jp,
                &row.map_description1_lang.pt_pt,
                &row.map_description1_lang.it_it,
                &row.map_description1_lang.unknown_12,
                &row.map_description1_lang.unknown_13,
                &row.map_description1_lang.unknown_14,
                &row.map_description1_lang.unknown_15,
                &row.map_description1_lang.flags,
                row.loading_screen_id.id,
                row.minimap_icon_scale,
                row.corpse_map_id.id,
                row.corpse[0],
                row.corpse[1],
                row.time_of_day_override,
                row.expansion_id,
                row.raid_offset,
                row.max_players,
                ])?;
            }
            tx.commit()?;
        }
        "TotemCategory.dbc" => {
            let data = totem_category::TotemCategory::read(file_contents)?;
            let (table, insert) = TotemCategory();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.totem_category_type,
                row.totem_category_mask,
                ])?;
            }
            tx.commit()?;
        }
        "NamesReserved.dbc" => {
            let data = names_reserved::NamesReserved::read(file_contents)?;
            let (table, insert) = NamesReserved();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.language,
                ])?;
            }
            tx.commit()?;
        }
        "SpellVisualKitAreaModel.dbc" => {
            let data = spell_visual_kit_area_model::SpellVisualKitAreaModel::read(file_contents)?;
            let (table, insert) = SpellVisualKitAreaModel();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.enum_id,
                ])?;
            }
            tx.commit()?;
        }
        "CreatureDisplayInfo.dbc" => {
            let data = creature_display_info::CreatureDisplayInfo::read(file_contents)?;
            let (table, insert) = CreatureDisplayInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.model_id.id,
                row.sound_id.id,
                row.extended_display_info_id.id,
                row.creature_model_scale,
                row.creature_model_alpha,
                row.texture_variation[0],
                row.texture_variation[1],
                row.texture_variation[2],
                &row.portrait_texture_name,
                row.size_class,
                row.blood_id.id,
                row.n_p_c_sound_id.id,
                row.particle_color_id.id,
                row.creature_geoset_data,
                row.object_effect_package_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "RandPropPoints.dbc" => {
            let data = rand_prop_points::RandPropPoints::read(file_contents)?;
            let (table, insert) = RandPropPoints();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.epic[0],
                row.epic[1],
                row.epic[2],
                row.epic[3],
                row.epic[4],
                row.superior[0],
                row.superior[1],
                row.superior[2],
                row.superior[3],
                row.superior[4],
                row.good[0],
                row.good[1],
                row.good[2],
                row.good[3],
                row.good[4],
                ])?;
            }
            tx.commit()?;
        }
        "ScreenEffect.dbc" => {
            let data = screen_effect::ScreenEffect::read(file_contents)?;
            let (table, insert) = ScreenEffect();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.effect,
                row.param[0],
                row.param[1],
                row.param[2],
                row.param[3],
                row.light_params_id.id,
                row.sound_ambience_id.id,
                row.zone_music_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "LFGDungeonGroup.dbc" => {
            let data = lfg_dungeon_group::LFGDungeonGroup::read(file_contents)?;
            let (table, insert) = LFGDungeonGroup();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.order_index,
                row.parent_group_id,
                row.type_id,
                ])?;
            }
            tx.commit()?;
        }
        "MapDifficulty.dbc" => {
            let data = map_difficulty::MapDifficulty::read(file_contents)?;
            let (table, insert) = MapDifficulty();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.difficulty,
                &row.message_lang.en_gb,
                &row.message_lang.ko_kr,
                &row.message_lang.fr_fr,
                &row.message_lang.de_de,
                &row.message_lang.en_cn,
                &row.message_lang.en_tw,
                &row.message_lang.es_es,
                &row.message_lang.es_mx,
                &row.message_lang.ru_ru,
                &row.message_lang.ja_jp,
                &row.message_lang.pt_pt,
                &row.message_lang.it_it,
                &row.message_lang.unknown_12,
                &row.message_lang.unknown_13,
                &row.message_lang.unknown_14,
                &row.message_lang.unknown_15,
                &row.message_lang.flags,
                row.raid_duration,
                row.max_players,
                &row.difficultystring,
                ])?;
            }
            tx.commit()?;
        }
        "QuestXP.dbc" => {
            let data = quest_xp::QuestXP::read(file_contents)?;
            let (table, insert) = QuestXP();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.difficulty[0],
                row.difficulty[1],
                row.difficulty[2],
                row.difficulty[3],
                row.difficulty[4],
                row.difficulty[5],
                row.difficulty[6],
                row.difficulty[7],
                row.difficulty[8],
                row.difficulty[9],
                ])?;
            }
            tx.commit()?;
        }
        "CharHairTextures.dbc" => {
            let data = char_hair_textures::CharHairTextures::read(file_contents)?;
            let (table, insert) = CharHairTextures();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.field_0_5_3_3368_001_race.id,
                row.field_0_5_3_3368_002_gender,
                row.field_0_5_3_3368_003,
                row.field_0_5_3_3368_004_mayberacemask,
                row.field_0_5_3_3368_005_the_x_in_hair_xy_blp,
                row.field_0_5_3_3368_006,
                row.field_0_5_3_3368_007,
                ])?;
            }
            tx.commit()?;
        }
        "Achievement_Category.dbc" => {
            let data = achievement_category::Achievement_Category::read(file_contents)?;
            let (table, insert) = Achievement_Category();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.parent.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.ui_order,
                ])?;
            }
            tx.commit()?;
        }
        "DurabilityQuality.dbc" => {
            let data = durability_quality::DurabilityQuality::read(file_contents)?;
            let (table, insert) = DurabilityQuality();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "SpellRange.dbc" => {
            let data = spell_range::SpellRange::read(file_contents)?;
            let (table, insert) = SpellRange();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.range_min[0],
                row.range_min[1],
                row.range_max[0],
                row.range_max[1],
                row.flags,
                &row.display_name_lang.en_gb,
                &row.display_name_lang.ko_kr,
                &row.display_name_lang.fr_fr,
                &row.display_name_lang.de_de,
                &row.display_name_lang.en_cn,
                &row.display_name_lang.en_tw,
                &row.display_name_lang.es_es,
                &row.display_name_lang.es_mx,
                &row.display_name_lang.ru_ru,
                &row.display_name_lang.ja_jp,
                &row.display_name_lang.pt_pt,
                &row.display_name_lang.it_it,
                &row.display_name_lang.unknown_12,
                &row.display_name_lang.unknown_13,
                &row.display_name_lang.unknown_14,
                &row.display_name_lang.unknown_15,
                &row.display_name_lang.flags,
                &row.display_name_short_lang.en_gb,
                &row.display_name_short_lang.ko_kr,
                &row.display_name_short_lang.fr_fr,
                &row.display_name_short_lang.de_de,
                &row.display_name_short_lang.en_cn,
                &row.display_name_short_lang.en_tw,
                &row.display_name_short_lang.es_es,
                &row.display_name_short_lang.es_mx,
                &row.display_name_short_lang.ru_ru,
                &row.display_name_short_lang.ja_jp,
                &row.display_name_short_lang.pt_pt,
                &row.display_name_short_lang.it_it,
                &row.display_name_short_lang.unknown_12,
                &row.display_name_short_lang.unknown_13,
                &row.display_name_short_lang.unknown_14,
                &row.display_name_short_lang.unknown_15,
                &row.display_name_short_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "WMOAreaTable.dbc" => {
            let data = wmo_area_table::WMOAreaTable::read(file_contents)?;
            let (table, insert) = WMOAreaTable();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.w_m_o_id,
                row.name_set_id,
                row.w_m_o_group_id,
                row.sound_provider_pref.id,
                row.sound_provider_pref_underwater.id,
                row.ambience_id.id,
                row.zone_music.id,
                row.intro_sound.id,
                row.flags,
                row.area_table_id.id,
                &row.area_name_lang.en_gb,
                &row.area_name_lang.ko_kr,
                &row.area_name_lang.fr_fr,
                &row.area_name_lang.de_de,
                &row.area_name_lang.en_cn,
                &row.area_name_lang.en_tw,
                &row.area_name_lang.es_es,
                &row.area_name_lang.es_mx,
                &row.area_name_lang.ru_ru,
                &row.area_name_lang.ja_jp,
                &row.area_name_lang.pt_pt,
                &row.area_name_lang.it_it,
                &row.area_name_lang.unknown_12,
                &row.area_name_lang.unknown_13,
                &row.area_name_lang.unknown_14,
                &row.area_name_lang.unknown_15,
                &row.area_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SpellDispelType.dbc" => {
            let data = spell_dispel_type::SpellDispelType::read(file_contents)?;
            let (table, insert) = SpellDispelType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.mask,
                row.immunity_possible,
                &row.internal_name,
                ])?;
            }
            tx.commit()?;
        }
        "ItemBagFamily.dbc" => {
            let data = item_bag_family::ItemBagFamily::read(file_contents)?;
            let (table, insert) = ItemBagFamily();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "WorldStateZoneSounds.dbc" => {
            let data = world_state_zone_sounds::WorldStateZoneSounds::read(file_contents)?;
            let (table, insert) = WorldStateZoneSounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.world_state_id,
                row.world_state_value,
                row.area_id.id,
                row.w_m_o_area_id.id,
                row.zone_intro_music_id.id,
                row.zone_music_id.id,
                row.sound_ambience_id.id,
                row.sound_provider_preferences_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "ObjectEffect.dbc" => {
            let data = object_effect::ObjectEffect::read(file_contents)?;
            let (table, insert) = ObjectEffect();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.object_effect_group_id.id,
                row.trigger_type,
                row.event_type,
                row.effect_rec_type,
                row.effect_rec_id,
                row.attachment,
                row.offset[0],
                row.offset[1],
                row.offset[2],
                row.object_effect_modifier_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "AuctionHouse.dbc" => {
            let data = auction_house::AuctionHouse::read(file_contents)?;
            let (table, insert) = AuctionHouse();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.faction_id.id,
                row.deposit_rate,
                row.consignment_rate,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "WorldChunkSounds.dbc" => {
            let data = world_chunk_sounds::WorldChunkSounds::read(file_contents)?;
            let (table, insert) = WorldChunkSounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.chunk_x,
                row.chunk_y,
                row.subchunk_x,
                row.subchunk_y,
                row.zone_intro_music_id,
                row.zone_music_id.id,
                row.sound_ambience_id.id,
                row.sound_provider_preferences_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "gtOCTClassCombatRatingScalar.dbc" => {
            let data = gt_oct_class_combat_rating_scalar::gtOCTClassCombatRatingScalar::read(file_contents)?;
            let (table, insert) = gtOCTClassCombatRatingScalar();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "SpellVisualEffectName.dbc" => {
            let data = spell_visual_effect_name::SpellVisualEffectName::read(file_contents)?;
            let (table, insert) = SpellVisualEffectName();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.file_name,
                row.area_effect_size,
                row.scale,
                row.min_allowed_scale,
                row.max_allowed_scale,
                ])?;
            }
            tx.commit()?;
        }
        "SpellMissileMotion.dbc" => {
            let data = spell_missile_motion::SpellMissileMotion::read(file_contents)?;
            let (table, insert) = SpellMissileMotion();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.script_body,
                row.flags,
                row.missile_count,
                ])?;
            }
            tx.commit()?;
        }
        "Weather.dbc" => {
            let data = weather::Weather::read(file_contents)?;
            let (table, insert) = Weather();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ambience_id.id,
                row.effect_type,
                row.transition_sky_box,
                row.effect_color[0],
                row.effect_color[1],
                row.effect_color[2],
                &row.effect_texture,
                ])?;
            }
            tx.commit()?;
        }
        "GameObjectDisplayInfo.dbc" => {
            let data = game_object_display_info::GameObjectDisplayInfo::read(file_contents)?;
            let (table, insert) = GameObjectDisplayInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model_name,
                row.sound[0],
                row.sound[1],
                row.sound[2],
                row.sound[3],
                row.sound[4],
                row.sound[5],
                row.sound[6],
                row.sound[7],
                row.sound[8],
                row.sound[9],
                row.geo_box_min[0],
                row.geo_box_min[1],
                row.geo_box_min[2],
                row.geo_box_max[0],
                row.geo_box_max[1],
                row.geo_box_max[2],
                row.object_effect_package_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "DungeonEncounter.dbc" => {
            let data = dungeon_encounter::DungeonEncounter::read(file_contents)?;
            let (table, insert) = DungeonEncounter();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.difficulty,
                row.order_index,
                row.bit,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.spell_icon_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "gtOCTRegenMP.dbc" => {
            let data = gt_oct_regen_mp::gtOCTRegenMP::read(file_contents)?;
            let (table, insert) = gtOCTRegenMP();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "CreatureMovementInfo.dbc" => {
            let data = creature_movement_info::CreatureMovementInfo::read(file_contents)?;
            let (table, insert) = CreatureMovementInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.smooth_facing_chase_rate,
                ])?;
            }
            tx.commit()?;
        }
        "CharTitles.dbc" => {
            let data = char_titles::CharTitles::read(file_contents)?;
            let (table, insert) = CharTitles();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.condition_id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.name1_lang.en_gb,
                &row.name1_lang.ko_kr,
                &row.name1_lang.fr_fr,
                &row.name1_lang.de_de,
                &row.name1_lang.en_cn,
                &row.name1_lang.en_tw,
                &row.name1_lang.es_es,
                &row.name1_lang.es_mx,
                &row.name1_lang.ru_ru,
                &row.name1_lang.ja_jp,
                &row.name1_lang.pt_pt,
                &row.name1_lang.it_it,
                &row.name1_lang.unknown_12,
                &row.name1_lang.unknown_13,
                &row.name1_lang.unknown_14,
                &row.name1_lang.unknown_15,
                &row.name1_lang.flags,
                row.mask_id,
                ])?;
            }
            tx.commit()?;
        }
        "ObjectEffectPackageElem.dbc" => {
            let data = object_effect_package_elem::ObjectEffectPackageElem::read(file_contents)?;
            let (table, insert) = ObjectEffectPackageElem();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.object_effect_package_id.id,
                row.object_effect_group_id.id,
                row.state_type,
                ])?;
            }
            tx.commit()?;
        }
        "ChrClasses.dbc" => {
            let data = chr_classes::ChrClasses::read(file_contents)?;
            let (table, insert) = ChrClasses();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.damage_bonus_stat,
                row.display_power,
                &row.pet_name_token,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.name_female_lang.en_gb,
                &row.name_female_lang.ko_kr,
                &row.name_female_lang.fr_fr,
                &row.name_female_lang.de_de,
                &row.name_female_lang.en_cn,
                &row.name_female_lang.en_tw,
                &row.name_female_lang.es_es,
                &row.name_female_lang.es_mx,
                &row.name_female_lang.ru_ru,
                &row.name_female_lang.ja_jp,
                &row.name_female_lang.pt_pt,
                &row.name_female_lang.it_it,
                &row.name_female_lang.unknown_12,
                &row.name_female_lang.unknown_13,
                &row.name_female_lang.unknown_14,
                &row.name_female_lang.unknown_15,
                &row.name_female_lang.flags,
                &row.name_male_lang.en_gb,
                &row.name_male_lang.ko_kr,
                &row.name_male_lang.fr_fr,
                &row.name_male_lang.de_de,
                &row.name_male_lang.en_cn,
                &row.name_male_lang.en_tw,
                &row.name_male_lang.es_es,
                &row.name_male_lang.es_mx,
                &row.name_male_lang.ru_ru,
                &row.name_male_lang.ja_jp,
                &row.name_male_lang.pt_pt,
                &row.name_male_lang.it_it,
                &row.name_male_lang.unknown_12,
                &row.name_male_lang.unknown_13,
                &row.name_male_lang.unknown_14,
                &row.name_male_lang.unknown_15,
                &row.name_male_lang.flags,
                &row.filename,
                row.spell_class_set,
                row.flags,
                row.cinematic_sequence_id.id,
                row.required_expansion,
                ])?;
            }
            tx.commit()?;
        }
        "GMSurveyCurrentSurvey.dbc" => {
            let data = gm_survey_current_survey::GMSurveyCurrentSurvey::read(file_contents)?;
            let (table, insert) = GMSurveyCurrentSurvey();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.gm_survey_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "QuestInfo.dbc" => {
            let data = quest_info::QuestInfo::read(file_contents)?;
            let (table, insert) = QuestInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.info_name_lang.en_gb,
                &row.info_name_lang.ko_kr,
                &row.info_name_lang.fr_fr,
                &row.info_name_lang.de_de,
                &row.info_name_lang.en_cn,
                &row.info_name_lang.en_tw,
                &row.info_name_lang.es_es,
                &row.info_name_lang.es_mx,
                &row.info_name_lang.ru_ru,
                &row.info_name_lang.ja_jp,
                &row.info_name_lang.pt_pt,
                &row.info_name_lang.it_it,
                &row.info_name_lang.unknown_12,
                &row.info_name_lang.unknown_13,
                &row.info_name_lang.unknown_14,
                &row.info_name_lang.unknown_15,
                &row.info_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SkillLine.dbc" => {
            let data = skill_line::SkillLine::read(file_contents)?;
            let (table, insert) = SkillLine();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.category_id.id,
                row.skill_costs_id,
                &row.display_name_lang.en_gb,
                &row.display_name_lang.ko_kr,
                &row.display_name_lang.fr_fr,
                &row.display_name_lang.de_de,
                &row.display_name_lang.en_cn,
                &row.display_name_lang.en_tw,
                &row.display_name_lang.es_es,
                &row.display_name_lang.es_mx,
                &row.display_name_lang.ru_ru,
                &row.display_name_lang.ja_jp,
                &row.display_name_lang.pt_pt,
                &row.display_name_lang.it_it,
                &row.display_name_lang.unknown_12,
                &row.display_name_lang.unknown_13,
                &row.display_name_lang.unknown_14,
                &row.display_name_lang.unknown_15,
                &row.display_name_lang.flags,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                row.spell_icon_id.id,
                &row.alternate_verb_lang.en_gb,
                &row.alternate_verb_lang.ko_kr,
                &row.alternate_verb_lang.fr_fr,
                &row.alternate_verb_lang.de_de,
                &row.alternate_verb_lang.en_cn,
                &row.alternate_verb_lang.en_tw,
                &row.alternate_verb_lang.es_es,
                &row.alternate_verb_lang.es_mx,
                &row.alternate_verb_lang.ru_ru,
                &row.alternate_verb_lang.ja_jp,
                &row.alternate_verb_lang.pt_pt,
                &row.alternate_verb_lang.it_it,
                &row.alternate_verb_lang.unknown_12,
                &row.alternate_verb_lang.unknown_13,
                &row.alternate_verb_lang.unknown_14,
                &row.alternate_verb_lang.unknown_15,
                &row.alternate_verb_lang.flags,
                row.can_link,
                ])?;
            }
            tx.commit()?;
        }
        "GroundEffectDoodad.dbc" => {
            let data = ground_effect_doodad::GroundEffectDoodad::read(file_contents)?;
            let (table, insert) = GroundEffectDoodad();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.doodadpath,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "ItemSet.dbc" => {
            let data = item_set::ItemSet::read(file_contents)?;
            let (table, insert) = ItemSet();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.item_id[0],
                row.item_id[1],
                row.item_id[2],
                row.item_id[3],
                row.item_id[4],
                row.item_id[5],
                row.item_id[6],
                row.item_id[7],
                row.item_id[8],
                row.item_id[9],
                row.item_id[10],
                row.item_id[11],
                row.item_id[12],
                row.item_id[13],
                row.item_id[14],
                row.item_id[15],
                row.item_id[16],
                row.set_spell_id[0],
                row.set_spell_id[1],
                row.set_spell_id[2],
                row.set_spell_id[3],
                row.set_spell_id[4],
                row.set_spell_id[5],
                row.set_spell_id[6],
                row.set_spell_id[7],
                row.set_threshold[0],
                row.set_threshold[1],
                row.set_threshold[2],
                row.set_threshold[3],
                row.set_threshold[4],
                row.set_threshold[5],
                row.set_threshold[6],
                row.set_threshold[7],
                row.required_skill.id,
                row.required_skill_rank,
                ])?;
            }
            tx.commit()?;
        }
        "gtNPCManaCostScaler.dbc" => {
            let data = gt_npc_mana_cost_scaler::gtNPCManaCostScaler::read(file_contents)?;
            let (table, insert) = gtNPCManaCostScaler();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "DanceMoves.dbc" => {
            let data = dance_moves::DanceMoves::read(file_contents)?;
            let (table, insert) = DanceMoves();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ty,
                row.param,
                row.fallback,
                row.racemask,
                &row.internal_name,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.lock_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "SpellMechanic.dbc" => {
            let data = spell_mechanic::SpellMechanic::read(file_contents)?;
            let (table, insert) = SpellMechanic();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.state_name_lang.en_gb,
                &row.state_name_lang.ko_kr,
                &row.state_name_lang.fr_fr,
                &row.state_name_lang.de_de,
                &row.state_name_lang.en_cn,
                &row.state_name_lang.en_tw,
                &row.state_name_lang.es_es,
                &row.state_name_lang.es_mx,
                &row.state_name_lang.ru_ru,
                &row.state_name_lang.ja_jp,
                &row.state_name_lang.pt_pt,
                &row.state_name_lang.it_it,
                &row.state_name_lang.unknown_12,
                &row.state_name_lang.unknown_13,
                &row.state_name_lang.unknown_14,
                &row.state_name_lang.unknown_15,
                &row.state_name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "BattlemasterList.dbc" => {
            let data = battlemaster_list::BattlemasterList::read(file_contents)?;
            let (table, insert) = BattlemasterList();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id[0],
                row.map_id[1],
                row.map_id[2],
                row.map_id[3],
                row.map_id[4],
                row.map_id[5],
                row.map_id[6],
                row.map_id[7],
                row.instance_type,
                row.groups_allowed,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.max_group_size,
                row.holiday_world_state,
                row.min_level,
                row.max_level,
                ])?;
            }
            tx.commit()?;
        }
        "WorldMapOverlay.dbc" => {
            let data = world_map_overlay::WorldMapOverlay::read(file_contents)?;
            let (table, insert) = WorldMapOverlay();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_area_id.id,
                row.area_id[0],
                row.area_id[1],
                row.area_id[2],
                row.area_id[3],
                row.map_point_x,
                row.map_point_y,
                &row.texture_name,
                row.texture_width,
                row.texture_height,
                row.offset_x,
                row.offset_y,
                row.hit_rect_top,
                row.hit_rect_left,
                row.hit_rect_bottom,
                row.hit_rect_right,
                ])?;
            }
            tx.commit()?;
        }
        "GlyphSlot.dbc" => {
            let data = glyph_slot::GlyphSlot::read(file_contents)?;
            let (table, insert) = GlyphSlot();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ty,
                row.tooltip,
                ])?;
            }
            tx.commit()?;
        }
        "GameTips.dbc" => {
            let data = game_tips::GameTips::read(file_contents)?;
            let (table, insert) = GameTips();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text_lang.en_gb,
                &row.text_lang.ko_kr,
                &row.text_lang.fr_fr,
                &row.text_lang.de_de,
                &row.text_lang.en_cn,
                &row.text_lang.en_tw,
                &row.text_lang.es_es,
                &row.text_lang.es_mx,
                &row.text_lang.ru_ru,
                &row.text_lang.ja_jp,
                &row.text_lang.pt_pt,
                &row.text_lang.it_it,
                &row.text_lang.unknown_12,
                &row.text_lang.unknown_13,
                &row.text_lang.unknown_14,
                &row.text_lang.unknown_15,
                &row.text_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "TaxiPathNode.dbc" => {
            let data = taxi_path_node::TaxiPathNode::read(file_contents)?;
            let (table, insert) = TaxiPathNode();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.path_id.id,
                row.node_index,
                row.continent_id.id,
                row.loc[0],
                row.loc[1],
                row.loc[2],
                row.flags,
                row.delay,
                row.arrival_event_id,
                row.departure_event_id,
                ])?;
            }
            tx.commit()?;
        }
        "gtRegenHPPerSpt.dbc" => {
            let data = gt_regen_hp_per_spt::gtRegenHPPerSpt::read(file_contents)?;
            let (table, insert) = gtRegenHPPerSpt();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "ItemPetFood.dbc" => {
            let data = item_pet_food::ItemPetFood::read(file_contents)?;
            let (table, insert) = ItemPetFood();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "ObjectEffectGroup.dbc" => {
            let data = object_effect_group::ObjectEffectGroup::read(file_contents)?;
            let (table, insert) = ObjectEffectGroup();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
            tx.commit()?;
        }
        "TerrainType.dbc" => {
            let data = terrain_type::TerrainType::read(file_contents)?;
            let (table, insert) = TerrainType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.terrain_id,
                &row.terrain_desc,
                row.footstep_spray_run,
                row.footstep_spray_walk,
                row.sound_id,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "GMTicketCategory.dbc" => {
            let data = gm_ticket_category::GMTicketCategory::read(file_contents)?;
            let (table, insert) = GMTicketCategory();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.category_lang.en_gb,
                &row.category_lang.ko_kr,
                &row.category_lang.fr_fr,
                &row.category_lang.de_de,
                &row.category_lang.en_cn,
                &row.category_lang.en_tw,
                &row.category_lang.es_es,
                &row.category_lang.es_mx,
                &row.category_lang.ru_ru,
                &row.category_lang.ja_jp,
                &row.category_lang.pt_pt,
                &row.category_lang.it_it,
                &row.category_lang.unknown_12,
                &row.category_lang.unknown_13,
                &row.category_lang.unknown_14,
                &row.category_lang.unknown_15,
                &row.category_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "QuestFactionReward.dbc" => {
            let data = quest_faction_reward::QuestFactionReward::read(file_contents)?;
            let (table, insert) = QuestFactionReward();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.difficulty[0],
                row.difficulty[1],
                row.difficulty[2],
                row.difficulty[3],
                row.difficulty[4],
                row.difficulty[5],
                row.difficulty[6],
                row.difficulty[7],
                row.difficulty[8],
                row.difficulty[9],
                ])?;
            }
            tx.commit()?;
        }
        "TransportAnimation.dbc" => {
            let data = transport_animation::TransportAnimation::read(file_contents)?;
            let (table, insert) = TransportAnimation();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.transport_id,
                row.time_index,
                row.pos[0],
                row.pos[1],
                row.pos[2],
                row.sequence_id,
                ])?;
            }
            tx.commit()?;
        }
        "DeathThudLookups.dbc" => {
            let data = death_thud_lookups::DeathThudLookups::read(file_contents)?;
            let (table, insert) = DeathThudLookups();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.size_class,
                row.terrain_type_sound_id.id,
                row.sound_entry_id.id,
                row.sound_entry_id_water.id,
                ])?;
            }
            tx.commit()?;
        }
        "StableSlotPrices.dbc" => {
            let data = stable_slot_prices::StableSlotPrices::read(file_contents)?;
            let (table, insert) = StableSlotPrices();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cost,
                ])?;
            }
            tx.commit()?;
        }
        "SummonProperties.dbc" => {
            let data = summon_properties::SummonProperties::read(file_contents)?;
            let (table, insert) = SummonProperties();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.control,
                row.faction.id,
                row.title,
                row.slot,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "CharBaseInfo.dbc" => {
            let data = char_base_info::CharBaseInfo::read(file_contents)?;
            let (table, insert) = CharBaseInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.race_id.id,
                row.class_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "DungeonMap.dbc" => {
            let data = dungeon_map::DungeonMap::read(file_contents)?;
            let (table, insert) = DungeonMap();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.floor_index,
                row.min_x,
                row.max_x,
                row.min_y,
                row.max_y,
                row.parent_world_map_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "DeclinedWordCases.dbc" => {
            let data = declined_word_cases::DeclinedWordCases::read(file_contents)?;
            let (table, insert) = DeclinedWordCases();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.declined_word_id.id,
                row.case_index,
                &row.declined_word,
                ])?;
            }
            tx.commit()?;
        }
        "Achievement_Criteria.dbc" => {
            let data = achievement_criteria::Achievement_Criteria::read(file_contents)?;
            let (table, insert) = Achievement_Criteria();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.achievement_id.id,
                row.ty,
                row.asset_id,
                row.quantity,
                row.start_event,
                row.start_asset,
                row.fail_event,
                row.fail_asset,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                row.flags,
                row.timer_start_event,
                row.timer_asset_id,
                row.timer_time,
                row.ui_order,
                ])?;
            }
            tx.commit()?;
        }
        "HelmetGeosetVisData.dbc" => {
            let data = helmet_geoset_vis_data::HelmetGeosetVisData::read(file_contents)?;
            let (table, insert) = HelmetGeosetVisData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.hide_geoset[0],
                row.hide_geoset[1],
                row.hide_geoset[2],
                row.hide_geoset[3],
                row.hide_geoset[4],
                row.hide_geoset[5],
                row.hide_geoset[6],
                ])?;
            }
            tx.commit()?;
        }
        "SkillLineAbility.dbc" => {
            let data = skill_line_ability::SkillLineAbility::read(file_contents)?;
            let (table, insert) = SkillLineAbility();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.skill_line.id,
                row.spell.id,
                row.race_mask,
                row.class_mask,
                row.exclude_race,
                row.exclude_class,
                row.min_skill_line_rank,
                row.superceded_by_spell.id,
                row.acquire_method,
                row.trivial_skill_line_rank_high,
                row.trivial_skill_line_rank_low,
                row.character_points[0],
                row.character_points[1],
                ])?;
            }
            tx.commit()?;
        }
        "SpellFocusObject.dbc" => {
            let data = spell_focus_object::SpellFocusObject::read(file_contents)?;
            let (table, insert) = SpellFocusObject();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SoundSamplePreferences.dbc" => {
            let data = sound_sample_preferences::SoundSamplePreferences::read(file_contents)?;
            let (table, insert) = SoundSamplePreferences();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.field_0_6_0_3592_001,
                row.field_0_6_0_3592_002,
                row.e_a_x2_sample_room,
                row.field_0_6_0_3592_004,
                row.field_0_6_0_3592_005,
                row.field_0_6_0_3592_006,
                row.field_0_6_0_3592_007,
                row.e_a_x2_sample_occlusion_l_f_ratio,
                row.e_a_x2_sample_occlusion_room_ratio,
                row.field_0_6_0_3592_010,
                row.e_a_x1_effect_level,
                row.field_0_6_0_3592_012,
                row.field_0_6_0_3592_013,
                row.e_a_x3_sample_exclusion,
                row.field_0_6_0_3592_015,
                row.field_0_6_0_3592_016,
                ])?;
            }
            tx.commit()?;
        }
        "HolidayDescriptions.dbc" => {
            let data = holiday_descriptions::HolidayDescriptions::read(file_contents)?;
            let (table, insert) = HolidayDescriptions();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "CharStartOutfit.dbc" => {
            let data = char_start_outfit::CharStartOutfit::read(file_contents)?;
            let (table, insert) = CharStartOutfit();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race_id.id,
                row.class_id.id,
                row.sex_id,
                row.outfit_id,
                row.item_id[0],
                row.item_id[1],
                row.item_id[2],
                row.item_id[3],
                row.item_id[4],
                row.item_id[5],
                row.item_id[6],
                row.item_id[7],
                row.item_id[8],
                row.item_id[9],
                row.item_id[10],
                row.item_id[11],
                row.item_id[12],
                row.item_id[13],
                row.item_id[14],
                row.item_id[15],
                row.item_id[16],
                row.item_id[17],
                row.item_id[18],
                row.item_id[19],
                row.item_id[20],
                row.item_id[21],
                row.item_id[22],
                row.item_id[23],
                row.display_item_id[0],
                row.display_item_id[1],
                row.display_item_id[2],
                row.display_item_id[3],
                row.display_item_id[4],
                row.display_item_id[5],
                row.display_item_id[6],
                row.display_item_id[7],
                row.display_item_id[8],
                row.display_item_id[9],
                row.display_item_id[10],
                row.display_item_id[11],
                row.display_item_id[12],
                row.display_item_id[13],
                row.display_item_id[14],
                row.display_item_id[15],
                row.display_item_id[16],
                row.display_item_id[17],
                row.display_item_id[18],
                row.display_item_id[19],
                row.display_item_id[20],
                row.display_item_id[21],
                row.display_item_id[22],
                row.display_item_id[23],
                row.inventory_type[0],
                row.inventory_type[1],
                row.inventory_type[2],
                row.inventory_type[3],
                row.inventory_type[4],
                row.inventory_type[5],
                row.inventory_type[6],
                row.inventory_type[7],
                row.inventory_type[8],
                row.inventory_type[9],
                row.inventory_type[10],
                row.inventory_type[11],
                row.inventory_type[12],
                row.inventory_type[13],
                row.inventory_type[14],
                row.inventory_type[15],
                row.inventory_type[16],
                row.inventory_type[17],
                row.inventory_type[18],
                row.inventory_type[19],
                row.inventory_type[20],
                row.inventory_type[21],
                row.inventory_type[22],
                row.inventory_type[23],
                ])?;
            }
            tx.commit()?;
        }
        "SpellVisualKit.dbc" => {
            let data = spell_visual_kit::SpellVisualKit::read(file_contents)?;
            let (table, insert) = SpellVisualKit();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.start_anim_id.id,
                row.anim_id.id,
                row.head_effect,
                row.chest_effect,
                row.base_effect,
                row.left_hand_effect,
                row.right_hand_effect,
                row.breath_effect,
                row.left_weapon_effect,
                row.right_weapon_effect,
                row.special_effect[0],
                row.special_effect[1],
                row.special_effect[2],
                row.world_effect,
                row.sound_id.id,
                row.shake_id.id,
                row.char_proc[0],
                row.char_proc[1],
                row.char_proc[2],
                row.char_proc[3],
                row.char_param_zero[0],
                row.char_param_zero[1],
                row.char_param_zero[2],
                row.char_param_zero[3],
                row.char_param_one[0],
                row.char_param_one[1],
                row.char_param_one[2],
                row.char_param_one[3],
                row.char_param_two[0],
                row.char_param_two[1],
                row.char_param_two[2],
                row.char_param_two[3],
                row.char_param_three[0],
                row.char_param_three[1],
                row.char_param_three[2],
                row.char_param_three[3],
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "gtChanceToMeleeCrit.dbc" => {
            let data = gt_chance_to_melee_crit::gtChanceToMeleeCrit::read(file_contents)?;
            let (table, insert) = gtChanceToMeleeCrit();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "GMSurveySurveys.dbc" => {
            let data = gm_survey_surveys::GMSurveySurveys::read(file_contents)?;
            let (table, insert) = GMSurveySurveys();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.q[0],
                row.q[1],
                row.q[2],
                row.q[3],
                row.q[4],
                row.q[5],
                row.q[6],
                row.q[7],
                row.q[8],
                row.q[9],
                ])?;
            }
            tx.commit()?;
        }
        "CharHairGeosets.dbc" => {
            let data = char_hair_geosets::CharHairGeosets::read(file_contents)?;
            let (table, insert) = CharHairGeosets();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race_id.id,
                row.sex_id,
                row.variation_id,
                row.geoset_id,
                row.showscalp,
                ])?;
            }
            tx.commit()?;
        }
        "PetitionType.dbc" => {
            let data = petition_type::PetitionType::read(file_contents)?;
            let (table, insert) = PetitionType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.ty,
                ])?;
            }
            tx.commit()?;
        }
        "UnitBlood.dbc" => {
            let data = unit_blood::UnitBlood::read(file_contents)?;
            let (table, insert) = UnitBlood();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.combat_blood_spurt_front[0],
                row.combat_blood_spurt_front[1],
                row.combat_blood_spurt_back[0],
                row.combat_blood_spurt_back[1],
                row.ground_blood[0],
                row.ground_blood[1],
                row.ground_blood[2],
                row.ground_blood[3],
                row.ground_blood[4],
                ])?;
            }
            tx.commit()?;
        }
        "Vehicle.dbc" => {
            let data = vehicle::Vehicle::read(file_contents)?;
            let (table, insert) = Vehicle();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.turn_speed,
                row.pitch_speed,
                row.pitch_min,
                row.pitch_max,
                row.seat_id[0],
                row.seat_id[1],
                row.seat_id[2],
                row.seat_id[3],
                row.seat_id[4],
                row.seat_id[5],
                row.seat_id[6],
                row.seat_id[7],
                row.mouse_look_offset_pitch,
                row.camera_fade_dist_scalar_min,
                row.camera_fade_dist_scalar_max,
                row.camera_pitch_offset,
                row.facing_limit_right,
                row.facing_limit_left,
                row.mssl_trgt_turn_lingering,
                row.mssl_trgt_pitch_lingering,
                row.mssl_trgt_mouse_lingering,
                row.mssl_trgt_end_opacity,
                row.mssl_trgt_arc_speed,
                row.mssl_trgt_arc_repeat,
                row.mssl_trgt_arc_width,
                row.mssl_trgt_impact_radius[0],
                row.mssl_trgt_impact_radius[1],
                &row.mssl_trgt_arc_texture,
                &row.mssl_trgt_impact_texture,
                row.mssl_trgt_impact_model[0],
                row.mssl_trgt_impact_model[1],
                row.camera_yaw_offset,
                row.ui_locomotion_type,
                row.mssl_trgt_impact_tex_radius,
                row.vehicle_u_i_indicator_id.id,
                row.power_display_id[0],
                row.power_display_id[1],
                row.power_display_id[2],
                ])?;
            }
            tx.commit()?;
        }
        "Startup_Strings.dbc" => {
            let data = startup_strings::Startup_Strings::read(file_contents)?;
            let (table, insert) = Startup_Strings();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.message_lang.en_gb,
                &row.message_lang.ko_kr,
                &row.message_lang.fr_fr,
                &row.message_lang.de_de,
                &row.message_lang.en_cn,
                &row.message_lang.en_tw,
                &row.message_lang.es_es,
                &row.message_lang.es_mx,
                &row.message_lang.ru_ru,
                &row.message_lang.ja_jp,
                &row.message_lang.pt_pt,
                &row.message_lang.it_it,
                &row.message_lang.unknown_12,
                &row.message_lang.unknown_13,
                &row.message_lang.unknown_14,
                &row.message_lang.unknown_15,
                &row.message_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "gtBarberShopCostBase.dbc" => {
            let data = gt_barber_shop_cost_base::gtBarberShopCostBase::read(file_contents)?;
            let (table, insert) = gtBarberShopCostBase();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "Holidays.dbc" => {
            let data = holidays::Holidays::read(file_contents)?;
            let (table, insert) = Holidays();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.duration[0],
                row.duration[1],
                row.duration[2],
                row.duration[3],
                row.duration[4],
                row.duration[5],
                row.duration[6],
                row.duration[7],
                row.duration[8],
                row.duration[9],
                row.date[0],
                row.date[1],
                row.date[2],
                row.date[3],
                row.date[4],
                row.date[5],
                row.date[6],
                row.date[7],
                row.date[8],
                row.date[9],
                row.date[10],
                row.date[11],
                row.date[12],
                row.date[13],
                row.date[14],
                row.date[15],
                row.date[16],
                row.date[17],
                row.date[18],
                row.date[19],
                row.date[20],
                row.date[21],
                row.date[22],
                row.date[23],
                row.date[24],
                row.date[25],
                row.region,
                row.looping,
                row.calendar_flags[0],
                row.calendar_flags[1],
                row.calendar_flags[2],
                row.calendar_flags[3],
                row.calendar_flags[4],
                row.calendar_flags[5],
                row.calendar_flags[6],
                row.calendar_flags[7],
                row.calendar_flags[8],
                row.calendar_flags[9],
                row.holiday_name_id.id,
                row.holiday_description_id.id,
                &row.texture_file_name,
                row.priority,
                row.calendar_filter_type,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "gtRegenMPPerSpt.dbc" => {
            let data = gt_regen_mp_per_spt::gtRegenMPPerSpt::read(file_contents)?;
            let (table, insert) = gtRegenMPPerSpt();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "SoundWaterType.dbc" => {
            let data = sound_water_type::SoundWaterType::read(file_contents)?;
            let (table, insert) = SoundWaterType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_type,
                row.sound_subtype,
                row.sound_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "LanguageWords.dbc" => {
            let data = language_words::LanguageWords::read(file_contents)?;
            let (table, insert) = LanguageWords();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.language_id.id,
                &row.word,
                ])?;
            }
            tx.commit()?;
        }
        "TerrainTypeSounds.dbc" => {
            let data = terrain_type_sounds::TerrainTypeSounds::read(file_contents)?;
            let (table, insert) = TerrainTypeSounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                ])?;
            }
            tx.commit()?;
        }
        "WorldMapContinent.dbc" => {
            let data = world_map_continent::WorldMapContinent::read(file_contents)?;
            let (table, insert) = WorldMapContinent();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.left_boundary,
                row.right_boundary,
                row.top_boundary,
                row.bottom_boundary,
                row.continent_offset[0],
                row.continent_offset[1],
                row.scale,
                row.taxi_min[0],
                row.taxi_min[1],
                row.taxi_max[0],
                row.taxi_max[1],
                row.world_map_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "Spell.dbc" => {
            let data = spell::Spell::read(file_contents)?;
            let (table, insert) = Spell();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.category.id,
                row.dispel_type.id,
                row.mechanic.id,
                row.attributes,
                row.attributes_ex,
                row.attributes_ex_b,
                row.attributes_ex_c,
                row.attributes_ex_d,
                row.attributes_ex_e,
                row.attributes_ex_f,
                row.attributes_ex_g,
                row.shapeshift_mask[0],
                row.shapeshift_mask[1],
                row.shapeshift_exclude[0],
                row.shapeshift_exclude[1],
                row.targets,
                row.target_creature_type,
                row.requires_spell_focus.id,
                row.facing_caster_flags,
                row.caster_aura_state,
                row.target_aura_state,
                row.exclude_caster_aura_state,
                row.exclude_target_aura_state,
                row.caster_aura_spell,
                row.target_aura_spell,
                row.exclude_caster_aura_spell,
                row.exclude_target_aura_spell,
                row.casting_time_index.id,
                row.recovery_time,
                row.category_recovery_time,
                row.interrupt_flags,
                row.aura_interrupt_flags,
                row.channel_interrupt_flags,
                row.proc_type_mask,
                row.proc_chance,
                row.proc_charges,
                row.max_level,
                row.base_level,
                row.spell_level,
                row.duration_index.id,
                row.power_type,
                row.mana_cost,
                row.mana_cost_per_level,
                row.mana_per_second,
                row.mana_per_second_per_level,
                row.range_index,
                row.speed,
                row.modal_next_spell,
                row.cumulative_aura,
                row.totem[0],
                row.totem[1],
                row.reagent[0],
                row.reagent[1],
                row.reagent[2],
                row.reagent[3],
                row.reagent[4],
                row.reagent[5],
                row.reagent[6],
                row.reagent[7],
                row.reagent_count[0],
                row.reagent_count[1],
                row.reagent_count[2],
                row.reagent_count[3],
                row.reagent_count[4],
                row.reagent_count[5],
                row.reagent_count[6],
                row.reagent_count[7],
                row.equipped_item_class,
                row.equipped_item_subclass,
                row.equipped_item_inv_types,
                row.effect[0],
                row.effect[1],
                row.effect[2],
                row.effect_die_sides[0],
                row.effect_die_sides[1],
                row.effect_die_sides[2],
                row.effect_real_points_per_level[0],
                row.effect_real_points_per_level[1],
                row.effect_real_points_per_level[2],
                row.effect_base_points[0],
                row.effect_base_points[1],
                row.effect_base_points[2],
                row.effect_mechanic[0],
                row.effect_mechanic[1],
                row.effect_mechanic[2],
                row.implicit_target_a[0],
                row.implicit_target_a[1],
                row.implicit_target_a[2],
                row.implicit_target_b[0],
                row.implicit_target_b[1],
                row.implicit_target_b[2],
                row.effect_radius_index[0],
                row.effect_radius_index[1],
                row.effect_radius_index[2],
                row.effect_aura[0],
                row.effect_aura[1],
                row.effect_aura[2],
                row.effect_aura_period[0],
                row.effect_aura_period[1],
                row.effect_aura_period[2],
                row.effect_amplitude[0],
                row.effect_amplitude[1],
                row.effect_amplitude[2],
                row.effect_chain_targets[0],
                row.effect_chain_targets[1],
                row.effect_chain_targets[2],
                row.effect_item_type[0],
                row.effect_item_type[1],
                row.effect_item_type[2],
                row.effect_misc_value[0],
                row.effect_misc_value[1],
                row.effect_misc_value[2],
                row.effect_misc_value_b[0],
                row.effect_misc_value_b[1],
                row.effect_misc_value_b[2],
                row.effect_trigger_spell[0],
                row.effect_trigger_spell[1],
                row.effect_trigger_spell[2],
                row.effect_points_per_combo[0],
                row.effect_points_per_combo[1],
                row.effect_points_per_combo[2],
                row.effect_spell_class_mask_a[0],
                row.effect_spell_class_mask_a[1],
                row.effect_spell_class_mask_a[2],
                row.effect_spell_class_mask_b[0],
                row.effect_spell_class_mask_b[1],
                row.effect_spell_class_mask_b[2],
                row.effect_spell_class_mask_c[0],
                row.effect_spell_class_mask_c[1],
                row.effect_spell_class_mask_c[2],
                row.spell_visual_id[0],
                row.spell_visual_id[1],
                row.spell_icon_id.id,
                row.active_icon_id.id,
                row.spell_priority,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.name_subtext_lang.en_gb,
                &row.name_subtext_lang.ko_kr,
                &row.name_subtext_lang.fr_fr,
                &row.name_subtext_lang.de_de,
                &row.name_subtext_lang.en_cn,
                &row.name_subtext_lang.en_tw,
                &row.name_subtext_lang.es_es,
                &row.name_subtext_lang.es_mx,
                &row.name_subtext_lang.ru_ru,
                &row.name_subtext_lang.ja_jp,
                &row.name_subtext_lang.pt_pt,
                &row.name_subtext_lang.it_it,
                &row.name_subtext_lang.unknown_12,
                &row.name_subtext_lang.unknown_13,
                &row.name_subtext_lang.unknown_14,
                &row.name_subtext_lang.unknown_15,
                &row.name_subtext_lang.flags,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                &row.aura_description_lang.en_gb,
                &row.aura_description_lang.ko_kr,
                &row.aura_description_lang.fr_fr,
                &row.aura_description_lang.de_de,
                &row.aura_description_lang.en_cn,
                &row.aura_description_lang.en_tw,
                &row.aura_description_lang.es_es,
                &row.aura_description_lang.es_mx,
                &row.aura_description_lang.ru_ru,
                &row.aura_description_lang.ja_jp,
                &row.aura_description_lang.pt_pt,
                &row.aura_description_lang.it_it,
                &row.aura_description_lang.unknown_12,
                &row.aura_description_lang.unknown_13,
                &row.aura_description_lang.unknown_14,
                &row.aura_description_lang.unknown_15,
                &row.aura_description_lang.flags,
                row.mana_cost_pct,
                row.start_recovery_category,
                row.start_recovery_time,
                row.max_target_level,
                row.spell_class_set,
                row.spell_class_mask[0],
                row.spell_class_mask[1],
                row.spell_class_mask[2],
                row.max_targets,
                row.defense_type,
                row.prevention_type,
                row.stance_bar_order,
                row.effect_chain_amplitude[0],
                row.effect_chain_amplitude[1],
                row.effect_chain_amplitude[2],
                row.min_faction_id.id,
                row.min_reputation,
                row.required_aura_vision,
                row.required_totem_category_id[0],
                row.required_totem_category_id[1],
                row.required_areas_id.id,
                row.school_mask,
                row.rune_cost_id.id,
                row.spell_missile_id.id,
                row.power_display_id.id,
                row.effect_bonus_coefficient[0],
                row.effect_bonus_coefficient[1],
                row.effect_bonus_coefficient[2],
                row.description_variables_id.id,
                row.difficulty,
                ])?;
            }
            tx.commit()?;
        }
        "CreatureDisplayInfoExtra.dbc" => {
            let data = creature_display_info_extra::CreatureDisplayInfoExtra::read(file_contents)?;
            let (table, insert) = CreatureDisplayInfoExtra();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.display_race_id.id,
                row.display_sex_id,
                row.skin_id,
                row.face_id,
                row.hair_style_id,
                row.hair_color_id,
                row.facial_hair_id,
                row.n_p_c_item_display[0],
                row.n_p_c_item_display[1],
                row.n_p_c_item_display[2],
                row.n_p_c_item_display[3],
                row.n_p_c_item_display[4],
                row.n_p_c_item_display[5],
                row.n_p_c_item_display[6],
                row.n_p_c_item_display[7],
                row.n_p_c_item_display[8],
                row.n_p_c_item_display[9],
                row.n_p_c_item_display[10],
                row.flags,
                &row.bake_name,
                ])?;
            }
            tx.commit()?;
        }
        "ItemSubClassMask.dbc" => {
            let data = item_sub_class_mask::ItemSubClassMask::read(file_contents)?;
            let (table, insert) = ItemSubClassMask();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.class_id,
                row.mask,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "gtChanceToSpellCrit.dbc" => {
            let data = gt_chance_to_spell_crit::gtChanceToSpellCrit::read(file_contents)?;
            let (table, insert) = gtChanceToSpellCrit();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "SkillTiers.dbc" => {
            let data = skill_tiers::SkillTiers::read(file_contents)?;
            let (table, insert) = SkillTiers();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cost[0],
                row.cost[1],
                row.cost[2],
                row.cost[3],
                row.cost[4],
                row.cost[5],
                row.cost[6],
                row.cost[7],
                row.cost[8],
                row.cost[9],
                row.cost[10],
                row.cost[11],
                row.cost[12],
                row.cost[13],
                row.cost[14],
                row.cost[15],
                row.value[0],
                row.value[1],
                row.value[2],
                row.value[3],
                row.value[4],
                row.value[5],
                row.value[6],
                row.value[7],
                row.value[8],
                row.value[9],
                row.value[10],
                row.value[11],
                row.value[12],
                row.value[13],
                row.value[14],
                row.value[15],
                ])?;
            }
            tx.commit()?;
        }
        "SpellVisualKitModelAttach.dbc" => {
            let data = spell_visual_kit_model_attach::SpellVisualKitModelAttach::read(file_contents)?;
            let (table, insert) = SpellVisualKitModelAttach();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.parent_spell_visual_kit_id.id,
                row.spell_visual_effect_name_id.id,
                row.attachment_id,
                row.offset[0],
                row.offset[1],
                row.offset[2],
                row.yaw,
                row.pitch,
                row.roll,
                ])?;
            }
            tx.commit()?;
        }
        "BankBagSlotPrices.dbc" => {
            let data = bank_bag_slot_prices::BankBagSlotPrices::read(file_contents)?;
            let (table, insert) = BankBagSlotPrices();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cost,
                ])?;
            }
            tx.commit()?;
        }
        "LoadingScreenTaxiSplines.dbc" => {
            let data = loading_screen_taxi_splines::LoadingScreenTaxiSplines::read(file_contents)?;
            let (table, insert) = LoadingScreenTaxiSplines();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.path_id.id,
                row.locx[0],
                row.locx[1],
                row.locx[2],
                row.locx[3],
                row.locx[4],
                row.locx[5],
                row.locx[6],
                row.locx[7],
                row.locy[0],
                row.locy[1],
                row.locy[2],
                row.locy[3],
                row.locy[4],
                row.locy[5],
                row.locy[6],
                row.locy[7],
                row.leg_index,
                ])?;
            }
            tx.commit()?;
        }
        "AreaPOI.dbc" => {
            let data = area_poi::AreaPOI::read(file_contents)?;
            let (table, insert) = AreaPOI();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.importance,
                row.icon[0],
                row.icon[1],
                row.icon[2],
                row.icon[3],
                row.icon[4],
                row.icon[5],
                row.icon[6],
                row.icon[7],
                row.icon[8],
                row.faction_id.id,
                row.pos[0],
                row.pos[1],
                row.pos[2],
                row.continent_id.id,
                row.flags,
                row.area_id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                row.world_state_id,
                row.world_map_link,
                ])?;
            }
            tx.commit()?;
        }
        "TransportRotation.dbc" => {
            let data = transport_rotation::TransportRotation::read(file_contents)?;
            let (table, insert) = TransportRotation();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.game_objects_id,
                row.time_index,
                row.rot[0],
                row.rot[1],
                row.rot[2],
                row.rot[3],
                ])?;
            }
            tx.commit()?;
        }
        "SoundFilterElem.dbc" => {
            let data = sound_filter_elem::SoundFilterElem::read(file_contents)?;
            let (table, insert) = SoundFilterElem();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_filter_id.id,
                row.order_index,
                row.filter_type,
                row.params[0],
                row.params[1],
                row.params[2],
                row.params[3],
                row.params[4],
                row.params[5],
                row.params[6],
                row.params[7],
                row.params[8],
                ])?;
            }
            tx.commit()?;
        }
        "PaperDollItemFrame.dbc" => {
            let data = paper_doll_item_frame::PaperDollItemFrame::read(file_contents)?;
            let (table, insert) = PaperDollItemFrame();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                &row.item_button_name,
                &row.slot_icon,
                row.slot_number,
                ])?;
            }
            tx.commit()?;
        }
        "ItemDisplayInfo.dbc" => {
            let data = item_display_info::ItemDisplayInfo::read(file_contents)?;
            let (table, insert) = ItemDisplayInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.model_name[0],
                row.model_name[1],
                row.model_texture[0],
                row.model_texture[1],
                row.inventory_icon[0],
                row.inventory_icon[1],
                row.geoset_group[0],
                row.geoset_group[1],
                row.geoset_group[2],
                row.flags,
                row.spell_visual_id.id,
                row.group_sound_index,
                row.helmet_geoset_vis_id[0],
                row.helmet_geoset_vis_id[1],
                row.texture[0],
                row.texture[1],
                row.texture[2],
                row.texture[3],
                row.texture[4],
                row.texture[5],
                row.texture[6],
                row.texture[7],
                row.item_visual,
                row.particle_color_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "SoundEntriesAdvanced.dbc" => {
            let data = sound_entries_advanced::SoundEntriesAdvanced::read(file_contents)?;
            let (table, insert) = SoundEntriesAdvanced();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_entry_id.id,
                row.inner_radius2_d,
                row.time_a,
                row.time_b,
                row.time_c,
                row.time_d,
                row.random_offset_range,
                row.usage,
                row.time_interval_min,
                row.time_interval_max,
                row.volume_slider_category,
                row.duck_to_s_f_x,
                row.duck_to_music,
                row.duck_to_ambience,
                row.inner_radius_of_influence,
                row.outer_radius_of_influence,
                row.time_to_duck,
                row.time_to_unduck,
                row.inside_angle,
                row.outside_angle,
                row.outside_volume,
                row.outer_radius2_d,
                &row.name,
                ])?;
            }
            tx.commit()?;
        }
        "SpellDifficulty.dbc" => {
            let data = spell_difficulty::SpellDifficulty::read(file_contents)?;
            let (table, insert) = SpellDifficulty();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.difficulty_spell_id[0],
                row.difficulty_spell_id[1],
                row.difficulty_spell_id[2],
                row.difficulty_spell_id[3],
                ])?;
            }
            tx.commit()?;
        }
        "LightSkybox.dbc" => {
            let data = light_skybox::LightSkybox::read(file_contents)?;
            let (table, insert) = LightSkybox();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "ChatProfanity.dbc" => {
            let data = chat_profanity::ChatProfanity::read(file_contents)?;
            let (table, insert) = ChatProfanity();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text,
                row.language,
                ])?;
            }
            tx.commit()?;
        }
        "Material.dbc" => {
            let data = material::Material::read(file_contents)?;
            let (table, insert) = Material();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.foley_sound_id,
                row.sheathe_sound_id,
                row.unsheathe_sound_id,
                ])?;
            }
            tx.commit()?;
        }
        "Stationery.dbc" => {
            let data = stationery::Stationery::read(file_contents)?;
            let (table, insert) = Stationery();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_id.id,
                &row.texture,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "LiquidType.dbc" => {
            let data = liquid_type::LiquidType::read(file_contents)?;
            let (table, insert) = LiquidType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.flags,
                row.sound_bank,
                row.sound_id.id,
                row.spell_id.id,
                row.max_darken_depth,
                row.fog_darken_intensity,
                row.amb_darken_intensity,
                row.dir_darken_intensity,
                row.light_id.id,
                row.particle_scale,
                row.particle_movement,
                row.particle_tex_slots,
                row.material_id.id,
                row.texture[0],
                row.texture[1],
                row.texture[2],
                row.texture[3],
                row.texture[4],
                row.texture[5],
                row.color[0],
                row.color[1],
                row.float[0],
                row.float[1],
                row.float[2],
                row.float[3],
                row.float[4],
                row.float[5],
                row.float[6],
                row.float[7],
                row.float[8],
                row.float[9],
                row.float[10],
                row.float[11],
                row.float[12],
                row.float[13],
                row.float[14],
                row.float[15],
                row.float[16],
                row.float[17],
                row.int[0],
                row.int[1],
                row.int[2],
                row.int[3],
                ])?;
            }
            tx.commit()?;
        }
        "SpamMessages.dbc" => {
            let data = spam_messages::SpamMessages::read(file_contents)?;
            let (table, insert) = SpamMessages();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text,
                ])?;
            }
            tx.commit()?;
        }
        "AnimationData.dbc" => {
            let data = animation_data::AnimationData::read(file_contents)?;
            let (table, insert) = AnimationData();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.weaponflags,
                row.bodyflags,
                row.flags,
                row.fallback.id,
                row.behavior_id.id,
                row.behavior_tier,
                ])?;
            }
            tx.commit()?;
        }
        "WorldStateUI.dbc" => {
            let data = world_state_ui::WorldStateUI::read(file_contents)?;
            let (table, insert) = WorldStateUI();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.area_id.id,
                row.phase_shift,
                &row.icon,
                &row.string_lang.en_gb,
                &row.string_lang.ko_kr,
                &row.string_lang.fr_fr,
                &row.string_lang.de_de,
                &row.string_lang.en_cn,
                &row.string_lang.en_tw,
                &row.string_lang.es_es,
                &row.string_lang.es_mx,
                &row.string_lang.ru_ru,
                &row.string_lang.ja_jp,
                &row.string_lang.pt_pt,
                &row.string_lang.it_it,
                &row.string_lang.unknown_12,
                &row.string_lang.unknown_13,
                &row.string_lang.unknown_14,
                &row.string_lang.unknown_15,
                &row.string_lang.flags,
                &row.tooltip_lang.en_gb,
                &row.tooltip_lang.ko_kr,
                &row.tooltip_lang.fr_fr,
                &row.tooltip_lang.de_de,
                &row.tooltip_lang.en_cn,
                &row.tooltip_lang.en_tw,
                &row.tooltip_lang.es_es,
                &row.tooltip_lang.es_mx,
                &row.tooltip_lang.ru_ru,
                &row.tooltip_lang.ja_jp,
                &row.tooltip_lang.pt_pt,
                &row.tooltip_lang.it_it,
                &row.tooltip_lang.unknown_12,
                &row.tooltip_lang.unknown_13,
                &row.tooltip_lang.unknown_14,
                &row.tooltip_lang.unknown_15,
                &row.tooltip_lang.flags,
                row.state_variable,
                row.ty,
                &row.dynamic_icon,
                &row.dynamic_tooltip_lang.en_gb,
                &row.dynamic_tooltip_lang.ko_kr,
                &row.dynamic_tooltip_lang.fr_fr,
                &row.dynamic_tooltip_lang.de_de,
                &row.dynamic_tooltip_lang.en_cn,
                &row.dynamic_tooltip_lang.en_tw,
                &row.dynamic_tooltip_lang.es_es,
                &row.dynamic_tooltip_lang.es_mx,
                &row.dynamic_tooltip_lang.ru_ru,
                &row.dynamic_tooltip_lang.ja_jp,
                &row.dynamic_tooltip_lang.pt_pt,
                &row.dynamic_tooltip_lang.it_it,
                &row.dynamic_tooltip_lang.unknown_12,
                &row.dynamic_tooltip_lang.unknown_13,
                &row.dynamic_tooltip_lang.unknown_14,
                &row.dynamic_tooltip_lang.unknown_15,
                &row.dynamic_tooltip_lang.flags,
                &row.extended_u_i,
                row.extended_u_i_state_variable[0],
                row.extended_u_i_state_variable[1],
                row.extended_u_i_state_variable[2],
                ])?;
            }
            tx.commit()?;
        }
        "ScalingStatDistribution.dbc" => {
            let data = scaling_stat_distribution::ScalingStatDistribution::read(file_contents)?;
            let (table, insert) = ScalingStatDistribution();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.stat_id[0],
                row.stat_id[1],
                row.stat_id[2],
                row.stat_id[3],
                row.stat_id[4],
                row.stat_id[5],
                row.stat_id[6],
                row.stat_id[7],
                row.stat_id[8],
                row.stat_id[9],
                row.bonus[0],
                row.bonus[1],
                row.bonus[2],
                row.bonus[3],
                row.bonus[4],
                row.bonus[5],
                row.bonus[6],
                row.bonus[7],
                row.bonus[8],
                row.bonus[9],
                row.maxlevel,
                ])?;
            }
            tx.commit()?;
        }
        "DeclinedWord.dbc" => {
            let data = declined_word::DeclinedWord::read(file_contents)?;
            let (table, insert) = DeclinedWord();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.word,
                ])?;
            }
            tx.commit()?;
        }
        "SoundProviderPreferences.dbc" => {
            let data = sound_provider_preferences::SoundProviderPreferences::read(file_contents)?;
            let (table, insert) = SoundProviderPreferences();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.description,
                row.flags,
                row.e_a_x_environment_selection,
                row.e_a_x_decay_time,
                row.e_a_x2_environment_size,
                row.e_a_x2_environment_diffusion,
                row.e_a_x2_room,
                row.e_a_x2_room_h_f,
                row.e_a_x2_decay_h_f_ratio,
                row.e_a_x2_reflections,
                row.e_a_x2_reflections_delay,
                row.e_a_x2_reverb,
                row.e_a_x2_reverb_delay,
                row.e_a_x2_room_rolloff,
                row.e_a_x2_air_absorption,
                row.e_a_x3_room_l_f,
                row.e_a_x3_decay_l_f_ratio,
                row.e_a_x3_echo_time,
                row.e_a_x3_echo_depth,
                row.e_a_x3_modulation_time,
                row.e_a_x3_modulation_depth,
                row.e_a_x3_h_f_reference,
                row.e_a_x3_l_f_reference,
                ])?;
            }
            tx.commit()?;
        }
        "ItemPurchaseGroup.dbc" => {
            let data = item_purchase_group::ItemPurchaseGroup::read(file_contents)?;
            let (table, insert) = ItemPurchaseGroup();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_id[0],
                row.item_id[1],
                row.item_id[2],
                row.item_id[3],
                row.item_id[4],
                row.item_id[5],
                row.item_id[6],
                row.item_id[7],
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "ChrRaces.dbc" => {
            let data = chr_races::ChrRaces::read(file_contents)?;
            let (table, insert) = ChrRaces();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.faction_id.id,
                row.exploration_sound_id.id,
                row.male_display_id.id,
                row.female_display_id.id,
                &row.client_prefix,
                row.base_language.id,
                row.creature_type.id,
                row.res_sickness_spell_id.id,
                row.splash_sound_id.id,
                &row.client_file_string,
                row.cinematic_sequence_id.id,
                row.alliance,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.name_female_lang.en_gb,
                &row.name_female_lang.ko_kr,
                &row.name_female_lang.fr_fr,
                &row.name_female_lang.de_de,
                &row.name_female_lang.en_cn,
                &row.name_female_lang.en_tw,
                &row.name_female_lang.es_es,
                &row.name_female_lang.es_mx,
                &row.name_female_lang.ru_ru,
                &row.name_female_lang.ja_jp,
                &row.name_female_lang.pt_pt,
                &row.name_female_lang.it_it,
                &row.name_female_lang.unknown_12,
                &row.name_female_lang.unknown_13,
                &row.name_female_lang.unknown_14,
                &row.name_female_lang.unknown_15,
                &row.name_female_lang.flags,
                &row.name_male_lang.en_gb,
                &row.name_male_lang.ko_kr,
                &row.name_male_lang.fr_fr,
                &row.name_male_lang.de_de,
                &row.name_male_lang.en_cn,
                &row.name_male_lang.en_tw,
                &row.name_male_lang.es_es,
                &row.name_male_lang.es_mx,
                &row.name_male_lang.ru_ru,
                &row.name_male_lang.ja_jp,
                &row.name_male_lang.pt_pt,
                &row.name_male_lang.it_it,
                &row.name_male_lang.unknown_12,
                &row.name_male_lang.unknown_13,
                &row.name_male_lang.unknown_14,
                &row.name_male_lang.unknown_15,
                &row.name_male_lang.flags,
                row.facial_hair_customization[0],
                row.facial_hair_customization[1],
                &row.hair_customization,
                row.required_expansion,
                ])?;
            }
            tx.commit()?;
        }
        "SheatheSoundLookups.dbc" => {
            let data = sheathe_sound_lookups::SheatheSoundLookups::read(file_contents)?;
            let (table, insert) = SheatheSoundLookups();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.class_id,
                row.subclass_id,
                row.material.id,
                row.check_material,
                row.sheathe_sound,
                row.unsheathe_sound,
                ])?;
            }
            tx.commit()?;
        }
        "UnitBloodLevels.dbc" => {
            let data = unit_blood_levels::UnitBloodLevels::read(file_contents)?;
            let (table, insert) = UnitBloodLevels();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.violencelevel[0],
                row.violencelevel[1],
                row.violencelevel[2],
                ])?;
            }
            tx.commit()?;
        }
        "SpellIcon.dbc" => {
            let data = spell_icon::SpellIcon::read(file_contents)?;
            let (table, insert) = SpellIcon();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.texture_filename,
                ])?;
            }
            tx.commit()?;
        }
        "LFGDungeons.dbc" => {
            let data = lfg_dungeons::LFGDungeons::read(file_contents)?;
            let (table, insert) = LFGDungeons();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.min_level,
                row.max_level,
                row.target_level,
                row.target_level_min,
                row.target_level_max,
                row.map_id.id,
                row.difficulty,
                row.flags,
                row.type_id,
                row.faction.id,
                &row.texture_filename,
                row.expansion_level,
                row.order_index,
                row.group_id,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SoundFilter.dbc" => {
            let data = sound_filter::SoundFilter::read(file_contents)?;
            let (table, insert) = SoundFilter();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
            tx.commit()?;
        }
        "StringLookups.dbc" => {
            let data = string_lookups::StringLookups::read(file_contents)?;
            let (table, insert) = StringLookups();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.string,
                ])?;
            }
            tx.commit()?;
        }
        "VocalUISounds.dbc" => {
            let data = vocal_ui_sounds::VocalUISounds::read(file_contents)?;
            let (table, insert) = VocalUISounds();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.vocal_u_i_enum,
                row.race_id.id,
                row.normal_sound_id[0],
                row.normal_sound_id[1],
                row.pissed_sound_id[0],
                row.pissed_sound_id[1],
                ])?;
            }
            tx.commit()?;
        }
        "LFGDungeonExpansion.dbc" => {
            let data = lfg_dungeon_expansion::LFGDungeonExpansion::read(file_contents)?;
            let (table, insert) = LFGDungeonExpansion();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.lfg_id,
                row.expansion_level,
                row.random_id,
                row.hard_level_min,
                row.hard_level_max,
                row.target_level_min,
                row.target_level_max,
                ])?;
            }
            tx.commit()?;
        }
        "SkillRaceClassInfo.dbc" => {
            let data = skill_race_class_info::SkillRaceClassInfo::read(file_contents)?;
            let (table, insert) = SkillRaceClassInfo();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.skill_id.id,
                row.race_mask,
                row.class_mask,
                row.flags,
                row.min_level,
                row.skill_tier_id.id,
                row.skill_cost_index,
                ])?;
            }
            tx.commit()?;
        }
        "AreaTrigger.dbc" => {
            let data = area_trigger::AreaTrigger::read(file_contents)?;
            let (table, insert) = AreaTrigger();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.continent_id.id,
                row.pos[0],
                row.pos[1],
                row.pos[2],
                row.radius,
                row.box_length,
                row.box_width,
                row.box_height,
                row.box_yaw,
                ])?;
            }
            tx.commit()?;
        }
        "LightParams.dbc" => {
            let data = light_params::LightParams::read(file_contents)?;
            let (table, insert) = LightParams();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.highlight_sky,
                row.light_skybox_id.id,
                row.glow,
                row.water_shallow_alpha,
                row.water_deep_alpha,
                row.ocean_shallow_alpha,
                row.ocean_deep_alpha,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "SpellVisualPrecastTransitions.dbc" => {
            let data = spell_visual_precast_transitions::SpellVisualPrecastTransitions::read(file_contents)?;
            let (table, insert) = SpellVisualPrecastTransitions();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.precast_load_anim_name,
                &row.precast_hold_anim_name,
                ])?;
            }
            tx.commit()?;
        }
        "NamesProfanity.dbc" => {
            let data = names_profanity::NamesProfanity::read(file_contents)?;
            let (table, insert) = NamesProfanity();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.language,
                ])?;
            }
            tx.commit()?;
        }
        "Emotes.dbc" => {
            let data = emotes::Emotes::read(file_contents)?;
            let (table, insert) = Emotes();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.emote_slash_command,
                row.anim_id.id,
                row.emote_flags,
                row.emote_spec_proc,
                row.emote_spec_proc_param,
                row.event_sound_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "CreatureType.dbc" => {
            let data = creature_type::CreatureType::read(file_contents)?;
            let (table, insert) = CreatureType();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.flags,
                ])?;
            }
            tx.commit()?;
        }
        "BarberShopStyle.dbc" => {
            let data = barber_shop_style::BarberShopStyle::read(file_contents)?;
            let (table, insert) = BarberShopStyle();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ty,
                &row.display_name_lang.en_gb,
                &row.display_name_lang.ko_kr,
                &row.display_name_lang.fr_fr,
                &row.display_name_lang.de_de,
                &row.display_name_lang.en_cn,
                &row.display_name_lang.en_tw,
                &row.display_name_lang.es_es,
                &row.display_name_lang.es_mx,
                &row.display_name_lang.ru_ru,
                &row.display_name_lang.ja_jp,
                &row.display_name_lang.pt_pt,
                &row.display_name_lang.it_it,
                &row.display_name_lang.unknown_12,
                &row.display_name_lang.unknown_13,
                &row.display_name_lang.unknown_14,
                &row.display_name_lang.unknown_15,
                &row.display_name_lang.flags,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                row.cost_modifier,
                row.race.id,
                row.sex,
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "DungeonMapChunk.dbc" => {
            let data = dungeon_map_chunk::DungeonMapChunk::read(file_contents)?;
            let (table, insert) = DungeonMapChunk();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.w_m_o_group_id.id,
                row.dungeon_map_id.id,
                row.min_z,
                ])?;
            }
            tx.commit()?;
        }
        "gtCombatRatings.dbc" => {
            let data = gt_combat_ratings::gtCombatRatings::read(file_contents)?;
            let (table, insert) = gtCombatRatings();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
            tx.commit()?;
        }
        "Faction.dbc" => {
            let data = faction::Faction::read(file_contents)?;
            let (table, insert) = Faction();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.reputation_index,
                row.reputation_race_mask[0],
                row.reputation_race_mask[1],
                row.reputation_race_mask[2],
                row.reputation_race_mask[3],
                row.reputation_class_mask[0],
                row.reputation_class_mask[1],
                row.reputation_class_mask[2],
                row.reputation_class_mask[3],
                row.reputation_base[0],
                row.reputation_base[1],
                row.reputation_base[2],
                row.reputation_base[3],
                row.reputation_flags[0],
                row.reputation_flags[1],
                row.reputation_flags[2],
                row.reputation_flags[3],
                row.parent_faction_id.id,
                row.parent_faction_mod[0],
                row.parent_faction_mod[1],
                row.parent_faction_cap[0],
                row.parent_faction_cap[1],
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                &row.description_lang.en_gb,
                &row.description_lang.ko_kr,
                &row.description_lang.fr_fr,
                &row.description_lang.de_de,
                &row.description_lang.en_cn,
                &row.description_lang.en_tw,
                &row.description_lang.es_es,
                &row.description_lang.es_mx,
                &row.description_lang.ru_ru,
                &row.description_lang.ja_jp,
                &row.description_lang.pt_pt,
                &row.description_lang.it_it,
                &row.description_lang.unknown_12,
                &row.description_lang.unknown_13,
                &row.description_lang.unknown_14,
                &row.description_lang.unknown_15,
                &row.description_lang.flags,
                ])?;
            }
            tx.commit()?;
        }
        "Exhaustion.dbc" => {
            let data = exhaustion::Exhaustion::read(file_contents)?;
            let (table, insert) = Exhaustion();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.xp,
                row.factor,
                row.outdoor_hours,
                row.inn_hours,
                &row.name_lang.en_gb,
                &row.name_lang.ko_kr,
                &row.name_lang.fr_fr,
                &row.name_lang.de_de,
                &row.name_lang.en_cn,
                &row.name_lang.en_tw,
                &row.name_lang.es_es,
                &row.name_lang.es_mx,
                &row.name_lang.ru_ru,
                &row.name_lang.ja_jp,
                &row.name_lang.pt_pt,
                &row.name_lang.it_it,
                &row.name_lang.unknown_12,
                &row.name_lang.unknown_13,
                &row.name_lang.unknown_14,
                &row.name_lang.unknown_15,
                &row.name_lang.flags,
                row.threshold,
                ])?;
            }
            tx.commit()?;
        }
        "WeaponSwingSounds2.dbc" => {
            let data = weapon_swing_sounds2::WeaponSwingSounds2::read(file_contents)?;
            let (table, insert) = WeaponSwingSounds2();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.swing_type,
                row.crit,
                row.sound_id.id,
                ])?;
            }
            tx.commit()?;
        }
        "SoundAmbience.dbc" => {
            let data = sound_ambience::SoundAmbience::read(file_contents)?;
            let (table, insert) = SoundAmbience();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ambience_id[0],
                row.ambience_id[1],
                ])?;
            }
            tx.commit()?;
        }
        "LightFloatBand.dbc" => {
            let data = light_float_band::LightFloatBand::read(file_contents)?;
            let (table, insert) = LightFloatBand();
            let tx = conn.transaction()?;
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.num,
                row.time[0],
                row.time[1],
                row.time[2],
                row.time[3],
                row.time[4],
                row.time[5],
                row.time[6],
                row.time[7],
                row.time[8],
                row.time[9],
                row.time[10],
                row.time[11],
                row.time[12],
                row.time[13],
                row.time[14],
                row.time[15],
                row.data[0],
                row.data[1],
                row.data[2],
                row.data[3],
                row.data[4],
                row.data[5],
                row.data[6],
                row.data[7],
                row.data[8],
                row.data[9],
                row.data[10],
                row.data[11],
                row.data[12],
                row.data[13],
                row.data[14],
                row.data[15],
                ])?;
            }
            tx.commit()?;
        }
        v => return Err(SqliteError::FilenameNotFound { name: v.to_string() }),
    }
    Ok(())
}
#[allow(non_snake_case)]
pub(crate) fn Cfg_Categories() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Cfg_Categories (
        id INTEGER PRIMARY KEY NOT NULL,
        locale_mask INTEGER  NOT NULL,
        create_charset_mask INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Cfg_Categories (
        id,
        locale_mask,
        create_charset_mask,
        flags,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundEmitters() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundEmitters (
        id INTEGER PRIMARY KEY NOT NULL,
        position_0 REAL NOT NULL,
        position_1 REAL NOT NULL,
        position_2 REAL NOT NULL,
        direction_0 REAL NOT NULL,
        direction_1 REAL NOT NULL,
        direction_2 REAL NOT NULL,
        sound_entry_advanced_id INTEGER  NOT NULL,
        map_id INTEGER  NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SoundEmitters (
        id,
        position_0,
        position_1,
        position_2,
        direction_0,
        direction_1,
        direction_2,
        sound_entry_advanced_id,
        map_id,
        name
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldMapTransforms() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapTransforms (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        region_min_0 REAL NOT NULL,
        region_min_1 REAL NOT NULL,
        region_max_0 REAL NOT NULL,
        region_max_1 REAL NOT NULL,
        new_map_id INTEGER  NOT NULL,
        region_offset_0 REAL NOT NULL,
        region_offset_1 REAL NOT NULL,
        new_dungeon_map_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WorldMapTransforms (
        id,
        map_id,
        region_min_0,
        region_min_1,
        region_max_0,
        region_max_1,
        new_map_id,
        region_offset_0,
        region_offset_1,
        new_dungeon_map_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LoadingScreens() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LoadingScreens (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        file_name TEXT  NOT NULL,
        has_wide_screen INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LoadingScreens (
        id,
        name,
        file_name,
        has_wide_screen
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn PetPersonality() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PetPersonality (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        happiness_threshold_0 INTEGER NOT NULL,
        happiness_threshold_1 INTEGER NOT NULL,
        happiness_threshold_2 INTEGER NOT NULL,
        happiness_damage_0 REAL NOT NULL,
        happiness_damage_1 REAL NOT NULL,
        happiness_damage_2 REAL NOT NULL
    );"
    ,
    "INSERT INTO PetPersonality (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        happiness_threshold_0,
        happiness_threshold_1,
        happiness_threshold_2,
        happiness_damage_0,
        happiness_damage_1,
        happiness_damage_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn MailTemplate() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS MailTemplate (
        id INTEGER PRIMARY KEY NOT NULL,
        subject_lang_en_gb TEXT NOT NULL,
        subject_lang_ko_kr TEXT NOT NULL,
        subject_lang_fr_fr TEXT NOT NULL,
        subject_lang_de_de TEXT NOT NULL,
        subject_lang_en_cn TEXT NOT NULL,
        subject_lang_en_tw TEXT NOT NULL,
        subject_lang_es_es TEXT NOT NULL,
        subject_lang_es_mx TEXT NOT NULL,
        subject_lang_ru_ru TEXT NOT NULL,
        subject_lang_ja_jp TEXT NOT NULL,
        subject_lang_pt_pt TEXT NOT NULL,
        subject_lang_it_it TEXT NOT NULL,
        subject_lang_unknown_12 TEXT NOT NULL,
        subject_lang_unknown_13 TEXT NOT NULL,
        subject_lang_unknown_14 TEXT NOT NULL,
        subject_lang_unknown_15 TEXT NOT NULL,
        subject_lang_flags TEXT NOT NULL,
        body_lang_en_gb TEXT NOT NULL,
        body_lang_ko_kr TEXT NOT NULL,
        body_lang_fr_fr TEXT NOT NULL,
        body_lang_de_de TEXT NOT NULL,
        body_lang_en_cn TEXT NOT NULL,
        body_lang_en_tw TEXT NOT NULL,
        body_lang_es_es TEXT NOT NULL,
        body_lang_es_mx TEXT NOT NULL,
        body_lang_ru_ru TEXT NOT NULL,
        body_lang_ja_jp TEXT NOT NULL,
        body_lang_pt_pt TEXT NOT NULL,
        body_lang_it_it TEXT NOT NULL,
        body_lang_unknown_12 TEXT NOT NULL,
        body_lang_unknown_13 TEXT NOT NULL,
        body_lang_unknown_14 TEXT NOT NULL,
        body_lang_unknown_15 TEXT NOT NULL,
        body_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO MailTemplate (
        id,
        subject_lang_en_gb,
        subject_lang_ko_kr,
        subject_lang_fr_fr,
        subject_lang_de_de,
        subject_lang_en_cn,
        subject_lang_en_tw,
        subject_lang_es_es,
        subject_lang_es_mx,
        subject_lang_ru_ru,
        subject_lang_ja_jp,
        subject_lang_pt_pt,
        subject_lang_it_it,
        subject_lang_unknown_12,
        subject_lang_unknown_13,
        subject_lang_unknown_14,
        subject_lang_unknown_15,
        subject_lang_flags,
        body_lang_en_gb,
        body_lang_ko_kr,
        body_lang_fr_fr,
        body_lang_de_de,
        body_lang_en_cn,
        body_lang_en_tw,
        body_lang_es_es,
        body_lang_es_mx,
        body_lang_ru_ru,
        body_lang_ja_jp,
        body_lang_pt_pt,
        body_lang_it_it,
        body_lang_unknown_12,
        body_lang_unknown_13,
        body_lang_unknown_14,
        body_lang_unknown_15,
        body_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldSafeLocs() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldSafeLocs (
        id INTEGER PRIMARY KEY NOT NULL,
        continent INTEGER  NOT NULL,
        loc_0 REAL NOT NULL,
        loc_1 REAL NOT NULL,
        loc_2 REAL NOT NULL,
        area_name_lang_en_gb TEXT NOT NULL,
        area_name_lang_ko_kr TEXT NOT NULL,
        area_name_lang_fr_fr TEXT NOT NULL,
        area_name_lang_de_de TEXT NOT NULL,
        area_name_lang_en_cn TEXT NOT NULL,
        area_name_lang_en_tw TEXT NOT NULL,
        area_name_lang_es_es TEXT NOT NULL,
        area_name_lang_es_mx TEXT NOT NULL,
        area_name_lang_ru_ru TEXT NOT NULL,
        area_name_lang_ja_jp TEXT NOT NULL,
        area_name_lang_pt_pt TEXT NOT NULL,
        area_name_lang_it_it TEXT NOT NULL,
        area_name_lang_unknown_12 TEXT NOT NULL,
        area_name_lang_unknown_13 TEXT NOT NULL,
        area_name_lang_unknown_14 TEXT NOT NULL,
        area_name_lang_unknown_15 TEXT NOT NULL,
        area_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO WorldSafeLocs (
        id,
        continent,
        loc_0,
        loc_1,
        loc_2,
        area_name_lang_en_gb,
        area_name_lang_ko_kr,
        area_name_lang_fr_fr,
        area_name_lang_de_de,
        area_name_lang_en_cn,
        area_name_lang_en_tw,
        area_name_lang_es_es,
        area_name_lang_es_mx,
        area_name_lang_ru_ru,
        area_name_lang_ja_jp,
        area_name_lang_pt_pt,
        area_name_lang_it_it,
        area_name_lang_unknown_12,
        area_name_lang_unknown_13,
        area_name_lang_unknown_14,
        area_name_lang_unknown_15,
        area_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn FactionGroup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FactionGroup (
        id INTEGER PRIMARY KEY NOT NULL,
        mask_id INTEGER  NOT NULL,
        internal_name TEXT  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO FactionGroup (
        id,
        mask_id,
        internal_name,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn PvpDifficulty() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PvpDifficulty (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        range_index INTEGER  NOT NULL,
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        difficulty INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO PvpDifficulty (
        id,
        map_id,
        range_index,
        min_level,
        max_level,
        difficulty
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn BannedAddOns() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS BannedAddOns (
        id INTEGER PRIMARY KEY NOT NULL,
        name_m_d5_0 INTEGER NOT NULL,
        name_m_d5_1 INTEGER NOT NULL,
        name_m_d5_2 INTEGER NOT NULL,
        name_m_d5_3 INTEGER NOT NULL,
        version_m_d5_0 INTEGER NOT NULL,
        version_m_d5_1 INTEGER NOT NULL,
        version_m_d5_2 INTEGER NOT NULL,
        version_m_d5_3 INTEGER NOT NULL,
        last_modified INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO BannedAddOns (
        id,
        name_m_d5_0,
        name_m_d5_1,
        name_m_d5_2,
        name_m_d5_3,
        version_m_d5_0,
        version_m_d5_1,
        version_m_d5_2,
        version_m_d5_3,
        last_modified,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn EmotesText() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EmotesText (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        emote_id INTEGER  NOT NULL,
        emote_text_0 INTEGER NOT NULL,
        emote_text_1 INTEGER NOT NULL,
        emote_text_2 INTEGER NOT NULL,
        emote_text_3 INTEGER NOT NULL,
        emote_text_4 INTEGER NOT NULL,
        emote_text_5 INTEGER NOT NULL,
        emote_text_6 INTEGER NOT NULL,
        emote_text_7 INTEGER NOT NULL,
        emote_text_8 INTEGER NOT NULL,
        emote_text_9 INTEGER NOT NULL,
        emote_text_10 INTEGER NOT NULL,
        emote_text_11 INTEGER NOT NULL,
        emote_text_12 INTEGER NOT NULL,
        emote_text_13 INTEGER NOT NULL,
        emote_text_14 INTEGER NOT NULL,
        emote_text_15 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO EmotesText (
        id,
        name,
        emote_id,
        emote_text_0,
        emote_text_1,
        emote_text_2,
        emote_text_3,
        emote_text_4,
        emote_text_5,
        emote_text_6,
        emote_text_7,
        emote_text_8,
        emote_text_9,
        emote_text_10,
        emote_text_11,
        emote_text_12,
        emote_text_13,
        emote_text_14,
        emote_text_15
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AttackAnimKits() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AttackAnimKits (
        id INTEGER PRIMARY KEY NOT NULL,
        item_subclass_id INTEGER  NOT NULL,
        anim_type_id INTEGER  NOT NULL,
        anim_frequency INTEGER  NOT NULL,
        which_hand INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AttackAnimKits (
        id,
        item_subclass_id,
        anim_type_id,
        anim_frequency,
        which_hand
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LightIntBand() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LightIntBand (
        id INTEGER PRIMARY KEY NOT NULL,
        num INTEGER  NOT NULL,
        time_0 INTEGER NOT NULL,
        time_1 INTEGER NOT NULL,
        time_2 INTEGER NOT NULL,
        time_3 INTEGER NOT NULL,
        time_4 INTEGER NOT NULL,
        time_5 INTEGER NOT NULL,
        time_6 INTEGER NOT NULL,
        time_7 INTEGER NOT NULL,
        time_8 INTEGER NOT NULL,
        time_9 INTEGER NOT NULL,
        time_10 INTEGER NOT NULL,
        time_11 INTEGER NOT NULL,
        time_12 INTEGER NOT NULL,
        time_13 INTEGER NOT NULL,
        time_14 INTEGER NOT NULL,
        time_15 INTEGER NOT NULL,
        data_0 INTEGER NOT NULL,
        data_1 INTEGER NOT NULL,
        data_2 INTEGER NOT NULL,
        data_3 INTEGER NOT NULL,
        data_4 INTEGER NOT NULL,
        data_5 INTEGER NOT NULL,
        data_6 INTEGER NOT NULL,
        data_7 INTEGER NOT NULL,
        data_8 INTEGER NOT NULL,
        data_9 INTEGER NOT NULL,
        data_10 INTEGER NOT NULL,
        data_11 INTEGER NOT NULL,
        data_12 INTEGER NOT NULL,
        data_13 INTEGER NOT NULL,
        data_14 INTEGER NOT NULL,
        data_15 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO LightIntBand (
        id,
        num,
        time_0,
        time_1,
        time_2,
        time_3,
        time_4,
        time_5,
        time_6,
        time_7,
        time_8,
        time_9,
        time_10,
        time_11,
        time_12,
        time_13,
        time_14,
        time_15,
        data_0,
        data_1,
        data_2,
        data_3,
        data_4,
        data_5,
        data_6,
        data_7,
        data_8,
        data_9,
        data_10,
        data_11,
        data_12,
        data_13,
        data_14,
        data_15
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureSpellData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureSpellData (
        id INTEGER PRIMARY KEY NOT NULL,
        spells_0 INTEGER NOT NULL,
        spells_1 INTEGER NOT NULL,
        spells_2 INTEGER NOT NULL,
        spells_3 INTEGER NOT NULL,
        availability_0 INTEGER NOT NULL,
        availability_1 INTEGER NOT NULL,
        availability_2 INTEGER NOT NULL,
        availability_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CreatureSpellData (
        id,
        spells_0,
        spells_1,
        spells_2,
        spells_3,
        availability_0,
        availability_1,
        availability_2,
        availability_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureModelData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureModelData (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        model_name TEXT  NOT NULL,
        size_class INTEGER  NOT NULL,
        model_scale REAL  NOT NULL,
        blood_id INTEGER  NOT NULL,
        footprint_texture_id INTEGER  NOT NULL,
        footprint_texture_length REAL  NOT NULL,
        footprint_texture_width REAL  NOT NULL,
        footprint_particle_scale REAL  NOT NULL,
        foley_material_id INTEGER  NOT NULL,
        footstep_shake_size INTEGER  NOT NULL,
        death_thud_shake_size INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL,
        collision_width REAL  NOT NULL,
        collision_height REAL  NOT NULL,
        mount_height REAL  NOT NULL,
        geo_box_min_x REAL  NOT NULL,
        geo_box_min_y REAL  NOT NULL,
        geo_box_min_z REAL  NOT NULL,
        geo_box_max_x REAL  NOT NULL,
        geo_box_max_y REAL  NOT NULL,
        geo_box_max_z REAL  NOT NULL,
        world_effect_scale REAL  NOT NULL,
        attached_effect_scale REAL  NOT NULL,
        missile_collision_radius REAL  NOT NULL,
        missile_collision_push REAL  NOT NULL,
        missile_collision_raise REAL  NOT NULL
    );"
    ,
    "INSERT INTO CreatureModelData (
        id,
        flags,
        model_name,
        size_class,
        model_scale,
        blood_id,
        footprint_texture_id,
        footprint_texture_length,
        footprint_texture_width,
        footprint_particle_scale,
        foley_material_id,
        footstep_shake_size,
        death_thud_shake_size,
        sound_id,
        collision_width,
        collision_height,
        mount_height,
        geo_box_min_x,
        geo_box_min_y,
        geo_box_min_z,
        geo_box_max_x,
        geo_box_max_y,
        geo_box_max_z,
        world_effect_scale,
        attached_effect_scale,
        missile_collision_radius,
        missile_collision_push,
        missile_collision_raise
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillCostsData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillCostsData (
        id INTEGER PRIMARY KEY NOT NULL,
        skill_costs_id INTEGER  NOT NULL,
        cost_0 INTEGER NOT NULL,
        cost_1 INTEGER NOT NULL,
        cost_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SkillCostsData (
        id,
        skill_costs_id,
        cost_0,
        cost_1,
        cost_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemGroupSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemGroupSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_0 INTEGER NOT NULL,
        sound_1 INTEGER NOT NULL,
        sound_2 INTEGER NOT NULL,
        sound_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ItemGroupSounds (
        id,
        sound_0,
        sound_1,
        sound_2,
        sound_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LiquidMaterial() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LiquidMaterial (
        id INTEGER PRIMARY KEY NOT NULL,
        l_v_f INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LiquidMaterial (
        id,
        l_v_f,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn VehicleUIIndicator() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS VehicleUIIndicator (
        id INTEGER PRIMARY KEY NOT NULL,
        background_texture TEXT  NOT NULL
    );"
    ,
    "INSERT INTO VehicleUIIndicator (
        id,
        background_texture
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemRandomSuffix() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemRandomSuffix (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        internal_name TEXT  NOT NULL,
        enchantment_0 INTEGER NOT NULL,
        enchantment_1 INTEGER NOT NULL,
        enchantment_2 INTEGER NOT NULL,
        enchantment_3 INTEGER NOT NULL,
        enchantment_4 INTEGER NOT NULL,
        allocation_pct_0 INTEGER NOT NULL,
        allocation_pct_1 INTEGER NOT NULL,
        allocation_pct_2 INTEGER NOT NULL,
        allocation_pct_3 INTEGER NOT NULL,
        allocation_pct_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ItemRandomSuffix (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        internal_name,
        enchantment_0,
        enchantment_1,
        enchantment_2,
        enchantment_3,
        enchantment_4,
        allocation_pct_0,
        allocation_pct_1,
        allocation_pct_2,
        allocation_pct_3,
        allocation_pct_4
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ZoneMusic() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ZoneMusic (
        id INTEGER PRIMARY KEY NOT NULL,
        set_name TEXT  NOT NULL,
        silence_interval_min_0 INTEGER NOT NULL,
        silence_interval_min_1 INTEGER NOT NULL,
        silence_interval_max_0 INTEGER NOT NULL,
        silence_interval_max_1 INTEGER NOT NULL,
        sounds_0 INTEGER NOT NULL,
        sounds_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ZoneMusic (
        id,
        set_name,
        silence_interval_min_0,
        silence_interval_min_1,
        silence_interval_max_0,
        silence_interval_max_1,
        sounds_0,
        sounds_1
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharVariations() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharVariations (
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        texture_hold_layer_0 INTEGER NOT NULL,
        texture_hold_layer_1 INTEGER NOT NULL,
        texture_hold_layer_2 INTEGER NOT NULL,
        texture_hold_layer_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CharVariations (
        race_id,
        sex_id,
        texture_hold_layer_0,
        texture_hold_layer_1,
        texture_hold_layer_2,
        texture_hold_layer_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GlyphProperties() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GlyphProperties (
        id INTEGER PRIMARY KEY NOT NULL,
        spell_id INTEGER  NOT NULL,
        glyph_slot_flags INTEGER  NOT NULL,
        spell_icon_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GlyphProperties (
        id,
        spell_id,
        glyph_slot_flags,
        spell_icon_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GMSurveyQuestions() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveyQuestions (
        id INTEGER PRIMARY KEY NOT NULL,
        question_lang_en_gb TEXT NOT NULL,
        question_lang_ko_kr TEXT NOT NULL,
        question_lang_fr_fr TEXT NOT NULL,
        question_lang_de_de TEXT NOT NULL,
        question_lang_en_cn TEXT NOT NULL,
        question_lang_en_tw TEXT NOT NULL,
        question_lang_es_es TEXT NOT NULL,
        question_lang_es_mx TEXT NOT NULL,
        question_lang_ru_ru TEXT NOT NULL,
        question_lang_ja_jp TEXT NOT NULL,
        question_lang_pt_pt TEXT NOT NULL,
        question_lang_it_it TEXT NOT NULL,
        question_lang_unknown_12 TEXT NOT NULL,
        question_lang_unknown_13 TEXT NOT NULL,
        question_lang_unknown_14 TEXT NOT NULL,
        question_lang_unknown_15 TEXT NOT NULL,
        question_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GMSurveyQuestions (
        id,
        question_lang_en_gb,
        question_lang_ko_kr,
        question_lang_fr_fr,
        question_lang_de_de,
        question_lang_en_cn,
        question_lang_en_tw,
        question_lang_es_es,
        question_lang_es_mx,
        question_lang_ru_ru,
        question_lang_ja_jp,
        question_lang_pt_pt,
        question_lang_it_it,
        question_lang_unknown_12,
        question_lang_unknown_13,
        question_lang_unknown_14,
        question_lang_unknown_15,
        question_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemSubClass() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemSubClass (
        class_id INTEGER  NOT NULL,
        sub_class_id INTEGER  NOT NULL,
        prerequisite_proficiency INTEGER  NOT NULL,
        postrequisite_proficiency INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        display_flags INTEGER  NOT NULL,
        weapon_parry_seq INTEGER  NOT NULL,
        weapon_ready_seq INTEGER  NOT NULL,
        weapon_attack_seq INTEGER  NOT NULL,
        weapon_swing_size INTEGER  NOT NULL,
        display_name_lang_en_gb TEXT NOT NULL,
        display_name_lang_ko_kr TEXT NOT NULL,
        display_name_lang_fr_fr TEXT NOT NULL,
        display_name_lang_de_de TEXT NOT NULL,
        display_name_lang_en_cn TEXT NOT NULL,
        display_name_lang_en_tw TEXT NOT NULL,
        display_name_lang_es_es TEXT NOT NULL,
        display_name_lang_es_mx TEXT NOT NULL,
        display_name_lang_ru_ru TEXT NOT NULL,
        display_name_lang_ja_jp TEXT NOT NULL,
        display_name_lang_pt_pt TEXT NOT NULL,
        display_name_lang_it_it TEXT NOT NULL,
        display_name_lang_unknown_12 TEXT NOT NULL,
        display_name_lang_unknown_13 TEXT NOT NULL,
        display_name_lang_unknown_14 TEXT NOT NULL,
        display_name_lang_unknown_15 TEXT NOT NULL,
        display_name_lang_flags TEXT NOT NULL,
        verbose_name_lang_en_gb TEXT NOT NULL,
        verbose_name_lang_ko_kr TEXT NOT NULL,
        verbose_name_lang_fr_fr TEXT NOT NULL,
        verbose_name_lang_de_de TEXT NOT NULL,
        verbose_name_lang_en_cn TEXT NOT NULL,
        verbose_name_lang_en_tw TEXT NOT NULL,
        verbose_name_lang_es_es TEXT NOT NULL,
        verbose_name_lang_es_mx TEXT NOT NULL,
        verbose_name_lang_ru_ru TEXT NOT NULL,
        verbose_name_lang_ja_jp TEXT NOT NULL,
        verbose_name_lang_pt_pt TEXT NOT NULL,
        verbose_name_lang_it_it TEXT NOT NULL,
        verbose_name_lang_unknown_12 TEXT NOT NULL,
        verbose_name_lang_unknown_13 TEXT NOT NULL,
        verbose_name_lang_unknown_14 TEXT NOT NULL,
        verbose_name_lang_unknown_15 TEXT NOT NULL,
        verbose_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemSubClass (
        class_id,
        sub_class_id,
        prerequisite_proficiency,
        postrequisite_proficiency,
        flags,
        display_flags,
        weapon_parry_seq,
        weapon_ready_seq,
        weapon_attack_seq,
        weapon_swing_size,
        display_name_lang_en_gb,
        display_name_lang_ko_kr,
        display_name_lang_fr_fr,
        display_name_lang_de_de,
        display_name_lang_en_cn,
        display_name_lang_en_tw,
        display_name_lang_es_es,
        display_name_lang_es_mx,
        display_name_lang_ru_ru,
        display_name_lang_ja_jp,
        display_name_lang_pt_pt,
        display_name_lang_it_it,
        display_name_lang_unknown_12,
        display_name_lang_unknown_13,
        display_name_lang_unknown_14,
        display_name_lang_unknown_15,
        display_name_lang_flags,
        verbose_name_lang_en_gb,
        verbose_name_lang_ko_kr,
        verbose_name_lang_fr_fr,
        verbose_name_lang_de_de,
        verbose_name_lang_en_cn,
        verbose_name_lang_en_tw,
        verbose_name_lang_es_es,
        verbose_name_lang_es_mx,
        verbose_name_lang_ru_ru,
        verbose_name_lang_ja_jp,
        verbose_name_lang_pt_pt,
        verbose_name_lang_it_it,
        verbose_name_lang_unknown_12,
        verbose_name_lang_unknown_13,
        verbose_name_lang_unknown_14,
        verbose_name_lang_unknown_15,
        verbose_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn FactionTemplate() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FactionTemplate (
        id INTEGER PRIMARY KEY NOT NULL,
        faction INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        faction_group INTEGER  NOT NULL,
        friend_group INTEGER  NOT NULL,
        enemy_group INTEGER  NOT NULL,
        enemies_0 INTEGER NOT NULL,
        enemies_1 INTEGER NOT NULL,
        enemies_2 INTEGER NOT NULL,
        enemies_3 INTEGER NOT NULL,
        friend_0 INTEGER NOT NULL,
        friend_1 INTEGER NOT NULL,
        friend_2 INTEGER NOT NULL,
        friend_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO FactionTemplate (
        id,
        faction,
        flags,
        faction_group,
        friend_group,
        enemy_group,
        enemies_0,
        enemies_1,
        enemies_2,
        enemies_3,
        friend_0,
        friend_1,
        friend_2,
        friend_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillLineCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillLineCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        sort_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SkillLineCategory (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        sort_index
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharSections() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharSections (
        id INTEGER PRIMARY KEY NOT NULL,
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        base_section INTEGER  NOT NULL,
        texture_name_0 TEXT NOT NULL,
        texture_name_1 TEXT NOT NULL,
        texture_name_2 TEXT NOT NULL,
        flags INTEGER  NOT NULL,
        variation_index INTEGER  NOT NULL,
        color_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharSections (
        id,
        race_id,
        sex_id,
        base_section,
        texture_name_0,
        texture_name_1,
        texture_name_2,
        flags,
        variation_index,
        color_index
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemVisuals() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemVisuals (
        id INTEGER PRIMARY KEY NOT NULL,
        slot_0 INTEGER NOT NULL,
        slot_1 INTEGER NOT NULL,
        slot_2 INTEGER NOT NULL,
        slot_3 INTEGER NOT NULL,
        slot_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ItemVisuals (
        id,
        slot_0,
        slot_1,
        slot_2,
        slot_3,
        slot_4
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TalentTab() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TalentTab (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        spell_icon_id INTEGER  NOT NULL,
        race_mask INTEGER  NOT NULL,
        class_mask INTEGER  NOT NULL,
        category_enum_id INTEGER  NOT NULL,
        order_index INTEGER  NOT NULL,
        background_file TEXT  NOT NULL
    );"
    ,
    "INSERT INTO TalentTab (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        spell_icon_id,
        race_mask,
        class_mask,
        category_enum_id,
        order_index,
        background_file
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Movie() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Movie (
        id INTEGER PRIMARY KEY NOT NULL,
        filename TEXT  NOT NULL,
        volume INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Movie (
        id,
        filename,
        volume
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GMSurveyAnswers() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveyAnswers (
        id INTEGER PRIMARY KEY NOT NULL,
        sort_index INTEGER  NOT NULL,
        g_m_survey_question_id INTEGER  NOT NULL,
        answer_lang_en_gb TEXT NOT NULL,
        answer_lang_ko_kr TEXT NOT NULL,
        answer_lang_fr_fr TEXT NOT NULL,
        answer_lang_de_de TEXT NOT NULL,
        answer_lang_en_cn TEXT NOT NULL,
        answer_lang_en_tw TEXT NOT NULL,
        answer_lang_es_es TEXT NOT NULL,
        answer_lang_es_mx TEXT NOT NULL,
        answer_lang_ru_ru TEXT NOT NULL,
        answer_lang_ja_jp TEXT NOT NULL,
        answer_lang_pt_pt TEXT NOT NULL,
        answer_lang_it_it TEXT NOT NULL,
        answer_lang_unknown_12 TEXT NOT NULL,
        answer_lang_unknown_13 TEXT NOT NULL,
        answer_lang_unknown_14 TEXT NOT NULL,
        answer_lang_unknown_15 TEXT NOT NULL,
        answer_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GMSurveyAnswers (
        id,
        sort_index,
        g_m_survey_question_id,
        answer_lang_en_gb,
        answer_lang_ko_kr,
        answer_lang_fr_fr,
        answer_lang_de_de,
        answer_lang_en_cn,
        answer_lang_en_tw,
        answer_lang_es_es,
        answer_lang_es_mx,
        answer_lang_ru_ru,
        answer_lang_ja_jp,
        answer_lang_pt_pt,
        answer_lang_it_it,
        answer_lang_unknown_12,
        answer_lang_unknown_13,
        answer_lang_unknown_14,
        answer_lang_unknown_15,
        answer_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn MovieFileData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS MovieFileData (
        file_data_id INTEGER  NOT NULL,
        resolution INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO MovieFileData (
        file_data_id,
        resolution
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellItemEnchantmentCondition() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellItemEnchantmentCondition (
        id INTEGER PRIMARY KEY NOT NULL,
        lt_operand_type_0 INTEGER NOT NULL,
        lt_operand_type_1 INTEGER NOT NULL,
        lt_operand_type_2 INTEGER NOT NULL,
        lt_operand_type_3 INTEGER NOT NULL,
        lt_operand_type_4 INTEGER NOT NULL,
        lt_operand_0 INTEGER NOT NULL,
        lt_operand_1 INTEGER NOT NULL,
        lt_operand_2 INTEGER NOT NULL,
        lt_operand_3 INTEGER NOT NULL,
        lt_operand_4 INTEGER NOT NULL,
        operator_0 INTEGER NOT NULL,
        operator_1 INTEGER NOT NULL,
        operator_2 INTEGER NOT NULL,
        operator_3 INTEGER NOT NULL,
        operator_4 INTEGER NOT NULL,
        rt_operand_type_0 INTEGER NOT NULL,
        rt_operand_type_1 INTEGER NOT NULL,
        rt_operand_type_2 INTEGER NOT NULL,
        rt_operand_type_3 INTEGER NOT NULL,
        rt_operand_type_4 INTEGER NOT NULL,
        rt_operand_0 INTEGER NOT NULL,
        rt_operand_1 INTEGER NOT NULL,
        rt_operand_2 INTEGER NOT NULL,
        rt_operand_3 INTEGER NOT NULL,
        rt_operand_4 INTEGER NOT NULL,
        logic_0 INTEGER NOT NULL,
        logic_1 INTEGER NOT NULL,
        logic_2 INTEGER NOT NULL,
        logic_3 INTEGER NOT NULL,
        logic_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SpellItemEnchantmentCondition (
        id,
        lt_operand_type_0,
        lt_operand_type_1,
        lt_operand_type_2,
        lt_operand_type_3,
        lt_operand_type_4,
        lt_operand_0,
        lt_operand_1,
        lt_operand_2,
        lt_operand_3,
        lt_operand_4,
        operator_0,
        operator_1,
        operator_2,
        operator_3,
        operator_4,
        rt_operand_type_0,
        rt_operand_type_1,
        rt_operand_type_2,
        rt_operand_type_3,
        rt_operand_type_4,
        rt_operand_0,
        rt_operand_1,
        rt_operand_2,
        rt_operand_3,
        rt_operand_4,
        logic_0,
        logic_1,
        logic_2,
        logic_3,
        logic_4
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AreaTable() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaTable (
        id INTEGER PRIMARY KEY NOT NULL,
        continent_id INTEGER  NOT NULL,
        parent_area_id INTEGER  NOT NULL,
        area_bit INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        sound_provider_pref INTEGER  NOT NULL,
        sound_provider_pref_underwater INTEGER  NOT NULL,
        ambience_id INTEGER  NOT NULL,
        zone_music INTEGER  NOT NULL,
        intro_sound INTEGER  NOT NULL,
        exploration_level INTEGER  NOT NULL,
        area_name_lang_en_gb TEXT NOT NULL,
        area_name_lang_ko_kr TEXT NOT NULL,
        area_name_lang_fr_fr TEXT NOT NULL,
        area_name_lang_de_de TEXT NOT NULL,
        area_name_lang_en_cn TEXT NOT NULL,
        area_name_lang_en_tw TEXT NOT NULL,
        area_name_lang_es_es TEXT NOT NULL,
        area_name_lang_es_mx TEXT NOT NULL,
        area_name_lang_ru_ru TEXT NOT NULL,
        area_name_lang_ja_jp TEXT NOT NULL,
        area_name_lang_pt_pt TEXT NOT NULL,
        area_name_lang_it_it TEXT NOT NULL,
        area_name_lang_unknown_12 TEXT NOT NULL,
        area_name_lang_unknown_13 TEXT NOT NULL,
        area_name_lang_unknown_14 TEXT NOT NULL,
        area_name_lang_unknown_15 TEXT NOT NULL,
        area_name_lang_flags TEXT NOT NULL,
        faction_group_mask INTEGER  NOT NULL,
        liquid_type_id_0 INTEGER NOT NULL,
        liquid_type_id_1 INTEGER NOT NULL,
        liquid_type_id_2 INTEGER NOT NULL,
        liquid_type_id_3 INTEGER NOT NULL,
        min_elevation REAL  NOT NULL,
        ambient_multiplier REAL  NOT NULL,
        light_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AreaTable (
        id,
        continent_id,
        parent_area_id,
        area_bit,
        flags,
        sound_provider_pref,
        sound_provider_pref_underwater,
        ambience_id,
        zone_music,
        intro_sound,
        exploration_level,
        area_name_lang_en_gb,
        area_name_lang_ko_kr,
        area_name_lang_fr_fr,
        area_name_lang_de_de,
        area_name_lang_en_cn,
        area_name_lang_en_tw,
        area_name_lang_es_es,
        area_name_lang_es_mx,
        area_name_lang_ru_ru,
        area_name_lang_ja_jp,
        area_name_lang_pt_pt,
        area_name_lang_it_it,
        area_name_lang_unknown_12,
        area_name_lang_unknown_13,
        area_name_lang_unknown_14,
        area_name_lang_unknown_15,
        area_name_lang_flags,
        faction_group_mask,
        liquid_type_id_0,
        liquid_type_id_1,
        liquid_type_id_2,
        liquid_type_id_3,
        min_elevation,
        ambient_multiplier,
        light_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TaxiNodes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TaxiNodes (
        id INTEGER PRIMARY KEY NOT NULL,
        continent_id INTEGER  NOT NULL,
        pos_0 REAL NOT NULL,
        pos_1 REAL NOT NULL,
        pos_2 REAL NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        mount_creature_id_0 INTEGER NOT NULL,
        mount_creature_id_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO TaxiNodes (
        id,
        continent_id,
        pos_0,
        pos_1,
        pos_2,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        mount_creature_id_0,
        mount_creature_id_1
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TransportPhysics() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TransportPhysics (
        id INTEGER PRIMARY KEY NOT NULL,
        wave_amp REAL  NOT NULL,
        wave_time_scale REAL  NOT NULL,
        roll_amp REAL  NOT NULL,
        roll_time_scale REAL  NOT NULL,
        pitch_amp REAL  NOT NULL,
        pitch_time_scale REAL  NOT NULL,
        max_bank REAL  NOT NULL,
        max_bank_turn_speed REAL  NOT NULL,
        speed_damp_thresh REAL  NOT NULL,
        speed_damp REAL  NOT NULL
    );"
    ,
    "INSERT INTO TransportPhysics (
        id,
        wave_amp,
        wave_time_scale,
        roll_amp,
        roll_time_scale,
        pitch_amp,
        pitch_time_scale,
        max_bank,
        max_bank_turn_speed,
        speed_damp_thresh,
        speed_damp
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ZoneIntroMusicTable() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ZoneIntroMusicTable (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        sound_id INTEGER  NOT NULL,
        priority INTEGER  NOT NULL,
        min_delay_minutes INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ZoneIntroMusicTable (
        id,
        name,
        sound_id,
        priority,
        min_delay_minutes
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtChanceToSpellCritBase() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtChanceToSpellCritBase (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtChanceToSpellCritBase (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemVisualEffects() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemVisualEffects (
        id INTEGER PRIMARY KEY NOT NULL,
        model TEXT  NOT NULL
    );"
    ,
    "INSERT INTO ItemVisualEffects (
        id,
        model
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellShapeshiftForm() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellShapeshiftForm (
        id INTEGER PRIMARY KEY NOT NULL,
        bonus_action_bar INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        flags INTEGER  NOT NULL,
        creature_type INTEGER  NOT NULL,
        attack_icon_id INTEGER  NOT NULL,
        combat_round_time INTEGER  NOT NULL,
        creature_display_id_0 INTEGER NOT NULL,
        creature_display_id_1 INTEGER NOT NULL,
        creature_display_id_2 INTEGER NOT NULL,
        creature_display_id_3 INTEGER NOT NULL,
        preset_spell_id_0 INTEGER NOT NULL,
        preset_spell_id_1 INTEGER NOT NULL,
        preset_spell_id_2 INTEGER NOT NULL,
        preset_spell_id_3 INTEGER NOT NULL,
        preset_spell_id_4 INTEGER NOT NULL,
        preset_spell_id_5 INTEGER NOT NULL,
        preset_spell_id_6 INTEGER NOT NULL,
        preset_spell_id_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SpellShapeshiftForm (
        id,
        bonus_action_bar,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        flags,
        creature_type,
        attack_icon_id,
        combat_round_time,
        creature_display_id_0,
        creature_display_id_1,
        creature_display_id_2,
        creature_display_id_3,
        preset_spell_id_0,
        preset_spell_id_1,
        preset_spell_id_2,
        preset_spell_id_3,
        preset_spell_id_4,
        preset_spell_id_5,
        preset_spell_id_6,
        preset_spell_id_7
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellEffectCameraShakes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellEffectCameraShakes (
        id INTEGER PRIMARY KEY NOT NULL,
        camera_shake_0 INTEGER NOT NULL,
        camera_shake_1 INTEGER NOT NULL,
        camera_shake_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SpellEffectCameraShakes (
        id,
        camera_shake_0,
        camera_shake_1,
        camera_shake_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn FootstepTerrainLookup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FootstepTerrainLookup (
        id INTEGER PRIMARY KEY NOT NULL,
        creature_footstep_id INTEGER  NOT NULL,
        terrain_sound_id INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL,
        sound_id_splash INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO FootstepTerrainLookup (
        id,
        creature_footstep_id,
        terrain_sound_id,
        sound_id,
        sound_id_splash
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellVisual() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisual (
        id INTEGER PRIMARY KEY NOT NULL,
        precast_kit INTEGER  NOT NULL,
        cast_kit INTEGER  NOT NULL,
        impact_kit INTEGER  NOT NULL,
        state_kit INTEGER  NOT NULL,
        state_done_kit INTEGER  NOT NULL,
        channel_kit INTEGER  NOT NULL,
        has_missile INTEGER  NOT NULL,
        missile_model INTEGER  NOT NULL,
        missile_path_type INTEGER  NOT NULL,
        missile_destination_attachment INTEGER  NOT NULL,
        missile_sound INTEGER  NOT NULL,
        anim_event_sound_id INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        caster_impact_kit INTEGER  NOT NULL,
        target_impact_kit INTEGER  NOT NULL,
        missile_attachment INTEGER  NOT NULL,
        missile_follow_ground_height INTEGER  NOT NULL,
        missile_follow_ground_drop_speed INTEGER  NOT NULL,
        missile_follow_ground_approach INTEGER  NOT NULL,
        missile_follow_ground_flags INTEGER  NOT NULL,
        missile_motion INTEGER  NOT NULL,
        missile_targeting_kit INTEGER  NOT NULL,
        instant_area_kit INTEGER  NOT NULL,
        impact_area_kit INTEGER  NOT NULL,
        persistent_area_kit INTEGER  NOT NULL,
        missile_cast_offset_0 REAL NOT NULL,
        missile_cast_offset_1 REAL NOT NULL,
        missile_cast_offset_2 REAL NOT NULL,
        missile_impact_offset_0 REAL NOT NULL,
        missile_impact_offset_1 REAL NOT NULL,
        missile_impact_offset_2 REAL NOT NULL
    );"
    ,
    "INSERT INTO SpellVisual (
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
        anim_event_sound_id,
        flags,
        caster_impact_kit,
        target_impact_kit,
        missile_attachment,
        missile_follow_ground_height,
        missile_follow_ground_drop_speed,
        missile_follow_ground_approach,
        missile_follow_ground_flags,
        missile_motion,
        missile_targeting_kit,
        instant_area_kit,
        impact_area_kit,
        persistent_area_kit,
        missile_cast_offset_0,
        missile_cast_offset_1,
        missile_cast_offset_2,
        missile_impact_offset_0,
        missile_impact_offset_1,
        missile_impact_offset_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn VehicleSeat() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS VehicleSeat (
        id INTEGER PRIMARY KEY NOT NULL,
        field_3_3_5_12213_001 REAL  NOT NULL,
        attachment_id INTEGER  NOT NULL,
        attachment_offset_0 REAL NOT NULL,
        attachment_offset_1 REAL NOT NULL,
        attachment_offset_2 REAL NOT NULL,
        field_3_3_5_12213_004 REAL  NOT NULL,
        enter_speed REAL  NOT NULL,
        enter_gravity REAL  NOT NULL,
        enter_min_duration REAL  NOT NULL,
        enter_max_duration REAL  NOT NULL,
        enter_min_arc_height REAL  NOT NULL,
        enter_max_arc_height REAL  NOT NULL,
        enter_anim_start INTEGER  NOT NULL,
        enter_anim_loop INTEGER  NOT NULL,
        ride_anim_start INTEGER  NOT NULL,
        ride_anim_loop INTEGER  NOT NULL,
        ride_upper_anim_start INTEGER  NOT NULL,
        ride_upper_anim_loop INTEGER  NOT NULL,
        field_3_3_5_12213_017 REAL  NOT NULL,
        exit_speed REAL  NOT NULL,
        exit_gravity REAL  NOT NULL,
        exit_min_duration REAL  NOT NULL,
        exit_max_duration REAL  NOT NULL,
        exit_min_arc_height REAL  NOT NULL,
        exit_max_arc_height REAL  NOT NULL,
        exit_anim_start INTEGER  NOT NULL,
        exit_anim_loop INTEGER  NOT NULL,
        exit_anim_end INTEGER  NOT NULL,
        field_3_3_5_12213_027 REAL  NOT NULL,
        passenger_pitch REAL  NOT NULL,
        field_3_3_5_12213_029 REAL  NOT NULL,
        passenger_attachment_id INTEGER  NOT NULL,
        vehicle_enter_anim INTEGER  NOT NULL,
        vehicle_exit_anim INTEGER  NOT NULL,
        vehicle_ride_anim_loop INTEGER  NOT NULL,
        field_3_3_5_12213_034 INTEGER  NOT NULL,
        vehicle_exit_anim_bone INTEGER  NOT NULL,
        vehicle_enter_anim_bone INTEGER  NOT NULL,
        field_3_3_5_12213_037 REAL  NOT NULL,
        field_3_3_5_12213_038 REAL  NOT NULL,
        vehicle_ability_display INTEGER  NOT NULL,
        enter_u_i_sound_id INTEGER  NOT NULL,
        field_3_3_5_12213_041 INTEGER  NOT NULL,
        ui_skin INTEGER  NOT NULL,
        field_3_3_5_12213_043 REAL  NOT NULL,
        field_3_3_5_12213_044 REAL  NOT NULL,
        field_3_3_5_12213_045 REAL  NOT NULL,
        field_3_3_5_12213_046 INTEGER  NOT NULL,
        field_3_3_5_12213_047 REAL  NOT NULL,
        field_3_3_5_12213_048 REAL  NOT NULL,
        field_3_3_5_12213_049 REAL  NOT NULL,
        field_3_3_5_12213_050 REAL  NOT NULL,
        field_3_3_5_12213_051 REAL  NOT NULL,
        field_3_3_5_12213_052 REAL  NOT NULL,
        field_3_3_5_12213_053 REAL  NOT NULL,
        field_3_3_5_12213_054 REAL  NOT NULL,
        field_3_3_5_12213_055 REAL  NOT NULL
    );"
    ,
    "INSERT INTO VehicleSeat (
        id,
        field_3_3_5_12213_001,
        attachment_id,
        attachment_offset_0,
        attachment_offset_1,
        attachment_offset_2,
        field_3_3_5_12213_004,
        enter_speed,
        enter_gravity,
        enter_min_duration,
        enter_max_duration,
        enter_min_arc_height,
        enter_max_arc_height,
        enter_anim_start,
        enter_anim_loop,
        ride_anim_start,
        ride_anim_loop,
        ride_upper_anim_start,
        ride_upper_anim_loop,
        field_3_3_5_12213_017,
        exit_speed,
        exit_gravity,
        exit_min_duration,
        exit_max_duration,
        exit_min_arc_height,
        exit_max_arc_height,
        exit_anim_start,
        exit_anim_loop,
        exit_anim_end,
        field_3_3_5_12213_027,
        passenger_pitch,
        field_3_3_5_12213_029,
        passenger_attachment_id,
        vehicle_enter_anim,
        vehicle_exit_anim,
        vehicle_ride_anim_loop,
        field_3_3_5_12213_034,
        vehicle_exit_anim_bone,
        vehicle_enter_anim_bone,
        field_3_3_5_12213_037,
        field_3_3_5_12213_038,
        vehicle_ability_display,
        enter_u_i_sound_id,
        field_3_3_5_12213_041,
        ui_skin,
        field_3_3_5_12213_043,
        field_3_3_5_12213_044,
        field_3_3_5_12213_045,
        field_3_3_5_12213_046,
        field_3_3_5_12213_047,
        field_3_3_5_12213_048,
        field_3_3_5_12213_049,
        field_3_3_5_12213_050,
        field_3_3_5_12213_051,
        field_3_3_5_12213_052,
        field_3_3_5_12213_053,
        field_3_3_5_12213_054,
        field_3_3_5_12213_055
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtChanceToMeleeCritBase() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtChanceToMeleeCritBase (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtChanceToMeleeCritBase (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Cfg_Configs() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Cfg_Configs (
        id INTEGER PRIMARY KEY NOT NULL,
        realm_type INTEGER  NOT NULL,
        player_killing_allowed INTEGER  NOT NULL,
        roleplaying INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Cfg_Configs (
        id,
        realm_type,
        player_killing_allowed,
        roleplaying
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ServerMessages() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ServerMessages (
        id INTEGER PRIMARY KEY NOT NULL,
        text_lang_en_gb TEXT NOT NULL,
        text_lang_ko_kr TEXT NOT NULL,
        text_lang_fr_fr TEXT NOT NULL,
        text_lang_de_de TEXT NOT NULL,
        text_lang_en_cn TEXT NOT NULL,
        text_lang_en_tw TEXT NOT NULL,
        text_lang_es_es TEXT NOT NULL,
        text_lang_es_mx TEXT NOT NULL,
        text_lang_ru_ru TEXT NOT NULL,
        text_lang_ja_jp TEXT NOT NULL,
        text_lang_pt_pt TEXT NOT NULL,
        text_lang_it_it TEXT NOT NULL,
        text_lang_unknown_12 TEXT NOT NULL,
        text_lang_unknown_13 TEXT NOT NULL,
        text_lang_unknown_14 TEXT NOT NULL,
        text_lang_unknown_15 TEXT NOT NULL,
        text_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ServerMessages (
        id,
        text_lang_en_gb,
        text_lang_ko_kr,
        text_lang_fr_fr,
        text_lang_de_de,
        text_lang_en_cn,
        text_lang_en_tw,
        text_lang_es_es,
        text_lang_es_mx,
        text_lang_ru_ru,
        text_lang_ja_jp,
        text_lang_pt_pt,
        text_lang_it_it,
        text_lang_unknown_12,
        text_lang_unknown_13,
        text_lang_unknown_14,
        text_lang_unknown_15,
        text_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Package() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Package (
        id INTEGER PRIMARY KEY NOT NULL,
        icon TEXT  NOT NULL,
        cost INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Package (
        id,
        icon,
        cost,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellRadius() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellRadius (
        id INTEGER PRIMARY KEY NOT NULL,
        radius REAL  NOT NULL,
        radius_per_level REAL  NOT NULL,
        radius_max REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellRadius (
        id,
        radius,
        radius_per_level,
        radius_max
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Achievement() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Achievement (
        id INTEGER PRIMARY KEY NOT NULL,
        faction INTEGER  NOT NULL,
        instance_id INTEGER  NOT NULL,
        supercedes INTEGER  NOT NULL,
        title_lang_en_gb TEXT NOT NULL,
        title_lang_ko_kr TEXT NOT NULL,
        title_lang_fr_fr TEXT NOT NULL,
        title_lang_de_de TEXT NOT NULL,
        title_lang_en_cn TEXT NOT NULL,
        title_lang_en_tw TEXT NOT NULL,
        title_lang_es_es TEXT NOT NULL,
        title_lang_es_mx TEXT NOT NULL,
        title_lang_ru_ru TEXT NOT NULL,
        title_lang_ja_jp TEXT NOT NULL,
        title_lang_pt_pt TEXT NOT NULL,
        title_lang_it_it TEXT NOT NULL,
        title_lang_unknown_12 TEXT NOT NULL,
        title_lang_unknown_13 TEXT NOT NULL,
        title_lang_unknown_14 TEXT NOT NULL,
        title_lang_unknown_15 TEXT NOT NULL,
        title_lang_flags TEXT NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL,
        category INTEGER  NOT NULL,
        points INTEGER  NOT NULL,
        ui_order INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        icon_id INTEGER  NOT NULL,
        reward_lang_en_gb TEXT NOT NULL,
        reward_lang_ko_kr TEXT NOT NULL,
        reward_lang_fr_fr TEXT NOT NULL,
        reward_lang_de_de TEXT NOT NULL,
        reward_lang_en_cn TEXT NOT NULL,
        reward_lang_en_tw TEXT NOT NULL,
        reward_lang_es_es TEXT NOT NULL,
        reward_lang_es_mx TEXT NOT NULL,
        reward_lang_ru_ru TEXT NOT NULL,
        reward_lang_ja_jp TEXT NOT NULL,
        reward_lang_pt_pt TEXT NOT NULL,
        reward_lang_it_it TEXT NOT NULL,
        reward_lang_unknown_12 TEXT NOT NULL,
        reward_lang_unknown_13 TEXT NOT NULL,
        reward_lang_unknown_14 TEXT NOT NULL,
        reward_lang_unknown_15 TEXT NOT NULL,
        reward_lang_flags TEXT NOT NULL,
        minimum_criteria INTEGER  NOT NULL,
        shares_criteria INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Achievement (
        id,
        faction,
        instance_id,
        supercedes,
        title_lang_en_gb,
        title_lang_ko_kr,
        title_lang_fr_fr,
        title_lang_de_de,
        title_lang_en_cn,
        title_lang_en_tw,
        title_lang_es_es,
        title_lang_es_mx,
        title_lang_ru_ru,
        title_lang_ja_jp,
        title_lang_pt_pt,
        title_lang_it_it,
        title_lang_unknown_12,
        title_lang_unknown_13,
        title_lang_unknown_14,
        title_lang_unknown_15,
        title_lang_flags,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags,
        category,
        points,
        ui_order,
        flags,
        icon_id,
        reward_lang_en_gb,
        reward_lang_ko_kr,
        reward_lang_fr_fr,
        reward_lang_de_de,
        reward_lang_en_cn,
        reward_lang_en_tw,
        reward_lang_es_es,
        reward_lang_es_mx,
        reward_lang_ru_ru,
        reward_lang_ja_jp,
        reward_lang_pt_pt,
        reward_lang_it_it,
        reward_lang_unknown_12,
        reward_lang_unknown_13,
        reward_lang_unknown_14,
        reward_lang_unknown_15,
        reward_lang_flags,
        minimum_criteria,
        shares_criteria
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60,
        ?61,
        ?62
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundEntries() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundEntries (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_type INTEGER  NOT NULL,
        name TEXT  NOT NULL,
        file_0 TEXT NOT NULL,
        file_1 TEXT NOT NULL,
        file_2 TEXT NOT NULL,
        file_3 TEXT NOT NULL,
        file_4 TEXT NOT NULL,
        file_5 TEXT NOT NULL,
        file_6 TEXT NOT NULL,
        file_7 TEXT NOT NULL,
        file_8 TEXT NOT NULL,
        file_9 TEXT NOT NULL,
        freq_0 INTEGER NOT NULL,
        freq_1 INTEGER NOT NULL,
        freq_2 INTEGER NOT NULL,
        freq_3 INTEGER NOT NULL,
        freq_4 INTEGER NOT NULL,
        freq_5 INTEGER NOT NULL,
        freq_6 INTEGER NOT NULL,
        freq_7 INTEGER NOT NULL,
        freq_8 INTEGER NOT NULL,
        freq_9 INTEGER NOT NULL,
        directory_base TEXT  NOT NULL,
        volume_float REAL  NOT NULL,
        flags INTEGER  NOT NULL,
        min_distance REAL  NOT NULL,
        distance_cutoff REAL  NOT NULL,
        e_a_x_def INTEGER  NOT NULL,
        sound_entries_advanced_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundEntries (
        id,
        sound_type,
        name,
        file_0,
        file_1,
        file_2,
        file_3,
        file_4,
        file_5,
        file_6,
        file_7,
        file_8,
        file_9,
        freq_0,
        freq_1,
        freq_2,
        freq_3,
        freq_4,
        freq_5,
        freq_6,
        freq_7,
        freq_8,
        freq_9,
        directory_base,
        volume_float,
        flags,
        min_distance,
        distance_cutoff,
        e_a_x_def,
        sound_entries_advanced_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn EnvironmentalDamage() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EnvironmentalDamage (
        id INTEGER PRIMARY KEY NOT NULL,
        enum_id INTEGER  NOT NULL,
        visualkit_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO EnvironmentalDamage (
        id,
        enum_id,
        visualkit_id
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DestructibleModelData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DestructibleModelData (
        id INTEGER PRIMARY KEY NOT NULL,
        state0_impact_effect_doodad_set INTEGER  NOT NULL,
        state0_ambient_doodad_set INTEGER  NOT NULL,
        state1_w_m_o INTEGER  NOT NULL,
        state1_destruction_doodad_set INTEGER  NOT NULL,
        state1_impact_effect_doodad_set INTEGER  NOT NULL,
        state1_ambient_doodad_set INTEGER  NOT NULL,
        state2_w_m_o INTEGER  NOT NULL,
        state2_destruction_doodad_set INTEGER  NOT NULL,
        state2_impact_effect_doodad_set INTEGER  NOT NULL,
        state2_ambient_doodad_set INTEGER  NOT NULL,
        state3_w_m_o INTEGER  NOT NULL,
        state3_init_doodad_set INTEGER  NOT NULL,
        state3_ambient_doodad_set INTEGER  NOT NULL,
        eject_direction INTEGER  NOT NULL,
        repair_ground_fx INTEGER  NOT NULL,
        do_not_highlight INTEGER  NOT NULL,
        heal_effect INTEGER  NOT NULL,
        heal_effect_speed INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO DestructibleModelData (
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
        heal_effect_speed
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WeaponImpactSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WeaponImpactSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        weapon_sub_class_id INTEGER  NOT NULL,
        parry_sound_type INTEGER  NOT NULL,
        impact_sound_id_0 INTEGER NOT NULL,
        impact_sound_id_1 INTEGER NOT NULL,
        impact_sound_id_2 INTEGER NOT NULL,
        impact_sound_id_3 INTEGER NOT NULL,
        impact_sound_id_4 INTEGER NOT NULL,
        impact_sound_id_5 INTEGER NOT NULL,
        impact_sound_id_6 INTEGER NOT NULL,
        impact_sound_id_7 INTEGER NOT NULL,
        impact_sound_id_8 INTEGER NOT NULL,
        impact_sound_id_9 INTEGER NOT NULL,
        crit_impact_sound_id_0 INTEGER NOT NULL,
        crit_impact_sound_id_1 INTEGER NOT NULL,
        crit_impact_sound_id_2 INTEGER NOT NULL,
        crit_impact_sound_id_3 INTEGER NOT NULL,
        crit_impact_sound_id_4 INTEGER NOT NULL,
        crit_impact_sound_id_5 INTEGER NOT NULL,
        crit_impact_sound_id_6 INTEGER NOT NULL,
        crit_impact_sound_id_7 INTEGER NOT NULL,
        crit_impact_sound_id_8 INTEGER NOT NULL,
        crit_impact_sound_id_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO WeaponImpactSounds (
        id,
        weapon_sub_class_id,
        parry_sound_type,
        impact_sound_id_0,
        impact_sound_id_1,
        impact_sound_id_2,
        impact_sound_id_3,
        impact_sound_id_4,
        impact_sound_id_5,
        impact_sound_id_6,
        impact_sound_id_7,
        impact_sound_id_8,
        impact_sound_id_9,
        crit_impact_sound_id_0,
        crit_impact_sound_id_1,
        crit_impact_sound_id_2,
        crit_impact_sound_id_3,
        crit_impact_sound_id_4,
        crit_impact_sound_id_5,
        crit_impact_sound_id_6,
        crit_impact_sound_id_7,
        crit_impact_sound_id_8,
        crit_impact_sound_id_9
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TaxiPath() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TaxiPath (
        id INTEGER PRIMARY KEY NOT NULL,
        from_taxi_node INTEGER  NOT NULL,
        to_taxi_node INTEGER  NOT NULL,
        cost INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TaxiPath (
        id,
        from_taxi_node,
        to_taxi_node,
        cost
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NPCSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NPCSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_id_0 INTEGER NOT NULL,
        sound_id_1 INTEGER NOT NULL,
        sound_id_2 INTEGER NOT NULL,
        sound_id_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO NPCSounds (
        id,
        sound_id_0,
        sound_id_1,
        sound_id_2,
        sound_id_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn VehicleUIIndSeat() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS VehicleUIIndSeat (
        id INTEGER PRIMARY KEY NOT NULL,
        vehicle_u_i_indicator_id INTEGER  NOT NULL,
        virtual_seat_index INTEGER  NOT NULL,
        x_pos REAL  NOT NULL,
        y_pos REAL  NOT NULL
    );"
    ,
    "INSERT INTO VehicleUIIndSeat (
        id,
        vehicle_u_i_indicator_id,
        virtual_seat_index,
        x_pos,
        y_pos
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NameGen() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NameGen (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        race_id INTEGER  NOT NULL,
        sex INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO NameGen (
        id,
        name,
        race_id,
        sex
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LockType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LockType (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        resource_name_lang_en_gb TEXT NOT NULL,
        resource_name_lang_ko_kr TEXT NOT NULL,
        resource_name_lang_fr_fr TEXT NOT NULL,
        resource_name_lang_de_de TEXT NOT NULL,
        resource_name_lang_en_cn TEXT NOT NULL,
        resource_name_lang_en_tw TEXT NOT NULL,
        resource_name_lang_es_es TEXT NOT NULL,
        resource_name_lang_es_mx TEXT NOT NULL,
        resource_name_lang_ru_ru TEXT NOT NULL,
        resource_name_lang_ja_jp TEXT NOT NULL,
        resource_name_lang_pt_pt TEXT NOT NULL,
        resource_name_lang_it_it TEXT NOT NULL,
        resource_name_lang_unknown_12 TEXT NOT NULL,
        resource_name_lang_unknown_13 TEXT NOT NULL,
        resource_name_lang_unknown_14 TEXT NOT NULL,
        resource_name_lang_unknown_15 TEXT NOT NULL,
        resource_name_lang_flags TEXT NOT NULL,
        verb_lang_en_gb TEXT NOT NULL,
        verb_lang_ko_kr TEXT NOT NULL,
        verb_lang_fr_fr TEXT NOT NULL,
        verb_lang_de_de TEXT NOT NULL,
        verb_lang_en_cn TEXT NOT NULL,
        verb_lang_en_tw TEXT NOT NULL,
        verb_lang_es_es TEXT NOT NULL,
        verb_lang_es_mx TEXT NOT NULL,
        verb_lang_ru_ru TEXT NOT NULL,
        verb_lang_ja_jp TEXT NOT NULL,
        verb_lang_pt_pt TEXT NOT NULL,
        verb_lang_it_it TEXT NOT NULL,
        verb_lang_unknown_12 TEXT NOT NULL,
        verb_lang_unknown_13 TEXT NOT NULL,
        verb_lang_unknown_14 TEXT NOT NULL,
        verb_lang_unknown_15 TEXT NOT NULL,
        verb_lang_flags TEXT NOT NULL,
        cursor_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LockType (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        resource_name_lang_en_gb,
        resource_name_lang_ko_kr,
        resource_name_lang_fr_fr,
        resource_name_lang_de_de,
        resource_name_lang_en_cn,
        resource_name_lang_en_tw,
        resource_name_lang_es_es,
        resource_name_lang_es_mx,
        resource_name_lang_ru_ru,
        resource_name_lang_ja_jp,
        resource_name_lang_pt_pt,
        resource_name_lang_it_it,
        resource_name_lang_unknown_12,
        resource_name_lang_unknown_13,
        resource_name_lang_unknown_14,
        resource_name_lang_unknown_15,
        resource_name_lang_flags,
        verb_lang_en_gb,
        verb_lang_ko_kr,
        verb_lang_fr_fr,
        verb_lang_de_de,
        verb_lang_en_cn,
        verb_lang_en_tw,
        verb_lang_es_es,
        verb_lang_es_mx,
        verb_lang_ru_ru,
        verb_lang_ja_jp,
        verb_lang_pt_pt,
        verb_lang_it_it,
        verb_lang_unknown_12,
        verb_lang_unknown_13,
        verb_lang_unknown_14,
        verb_lang_unknown_15,
        verb_lang_flags,
        cursor_name
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn PowerDisplay() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PowerDisplay (
        id INTEGER PRIMARY KEY NOT NULL,
        actual_type INTEGER  NOT NULL,
        global_string_base_tag TEXT  NOT NULL,
        red INTEGER  NOT NULL,
        green INTEGER  NOT NULL,
        blue INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO PowerDisplay (
        id,
        actual_type,
        global_string_base_tag,
        red,
        green,
        blue
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn HolidayNames() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS HolidayNames (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO HolidayNames (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharacterFacialHairStyles() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharacterFacialHairStyles (
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        variation_id INTEGER  NOT NULL,
        geoset_0 INTEGER NOT NULL,
        geoset_1 INTEGER NOT NULL,
        geoset_2 INTEGER NOT NULL,
        geoset_3 INTEGER NOT NULL,
        geoset_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CharacterFacialHairStyles (
        race_id,
        sex_id,
        variation_id,
        geoset_0,
        geoset_1,
        geoset_2,
        geoset_3,
        geoset_4
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CinematicSequences() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CinematicSequences (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_id INTEGER  NOT NULL,
        camera_0 INTEGER NOT NULL,
        camera_1 INTEGER NOT NULL,
        camera_2 INTEGER NOT NULL,
        camera_3 INTEGER NOT NULL,
        camera_4 INTEGER NOT NULL,
        camera_5 INTEGER NOT NULL,
        camera_6 INTEGER NOT NULL,
        camera_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CinematicSequences (
        id,
        sound_id,
        camera_0,
        camera_1,
        camera_2,
        camera_3,
        camera_4,
        camera_5,
        camera_6,
        camera_7
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WowError_Strings() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WowError_Strings (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO WowError_Strings (
        id,
        name,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellDescriptionVariables() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellDescriptionVariables (
        id INTEGER PRIMARY KEY NOT NULL,
        variables TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellDescriptionVariables (
        id,
        variables
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn MovieVariation() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS MovieVariation (
        id INTEGER PRIMARY KEY NOT NULL,
        movie_id INTEGER  NOT NULL,
        file_data_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO MovieVariation (
        id,
        movie_id,
        file_data_id
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ObjectEffectPackage() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ObjectEffectPackage (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO ObjectEffectPackage (
        id,
        name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellItemEnchantment() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellItemEnchantment (
        id INTEGER PRIMARY KEY NOT NULL,
        charges INTEGER  NOT NULL,
        effect_0 INTEGER NOT NULL,
        effect_1 INTEGER NOT NULL,
        effect_2 INTEGER NOT NULL,
        effect_points_min_0 INTEGER NOT NULL,
        effect_points_min_1 INTEGER NOT NULL,
        effect_points_min_2 INTEGER NOT NULL,
        effect_points_max_0 INTEGER NOT NULL,
        effect_points_max_1 INTEGER NOT NULL,
        effect_points_max_2 INTEGER NOT NULL,
        effect_arg_0 INTEGER NOT NULL,
        effect_arg_1 INTEGER NOT NULL,
        effect_arg_2 INTEGER NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        item_visual INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        src_item_id INTEGER  NOT NULL,
        condition_id INTEGER  NOT NULL,
        required_skill_id INTEGER  NOT NULL,
        required_skill_rank INTEGER  NOT NULL,
        min_level INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellItemEnchantment (
        id,
        charges,
        effect_0,
        effect_1,
        effect_2,
        effect_points_min_0,
        effect_points_min_1,
        effect_points_min_2,
        effect_points_max_0,
        effect_points_max_1,
        effect_points_max_2,
        effect_arg_0,
        effect_arg_1,
        effect_arg_2,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        item_visual,
        flags,
        src_item_id,
        condition_id,
        required_skill_id,
        required_skill_rank,
        min_level
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemExtendedCost() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemExtendedCost (
        id INTEGER PRIMARY KEY NOT NULL,
        honor_points INTEGER  NOT NULL,
        arena_points INTEGER  NOT NULL,
        arena_bracket INTEGER  NOT NULL,
        item_id_0 INTEGER NOT NULL,
        item_id_1 INTEGER NOT NULL,
        item_id_2 INTEGER NOT NULL,
        item_id_3 INTEGER NOT NULL,
        item_id_4 INTEGER NOT NULL,
        item_count_0 INTEGER NOT NULL,
        item_count_1 INTEGER NOT NULL,
        item_count_2 INTEGER NOT NULL,
        item_count_3 INTEGER NOT NULL,
        item_count_4 INTEGER NOT NULL,
        required_arena_rating INTEGER  NOT NULL,
        item_purchase_group INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ItemExtendedCost (
        id,
        honor_points,
        arena_points,
        arena_bracket,
        item_id_0,
        item_id_1,
        item_id_2,
        item_id_3,
        item_id_4,
        item_count_0,
        item_count_1,
        item_count_2,
        item_count_3,
        item_count_4,
        required_arena_rating,
        item_purchase_group
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemClass() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemClass (
        class_id INTEGER  NOT NULL,
        subclass_map_id INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        class_name_lang_en_gb TEXT NOT NULL,
        class_name_lang_ko_kr TEXT NOT NULL,
        class_name_lang_fr_fr TEXT NOT NULL,
        class_name_lang_de_de TEXT NOT NULL,
        class_name_lang_en_cn TEXT NOT NULL,
        class_name_lang_en_tw TEXT NOT NULL,
        class_name_lang_es_es TEXT NOT NULL,
        class_name_lang_es_mx TEXT NOT NULL,
        class_name_lang_ru_ru TEXT NOT NULL,
        class_name_lang_ja_jp TEXT NOT NULL,
        class_name_lang_pt_pt TEXT NOT NULL,
        class_name_lang_it_it TEXT NOT NULL,
        class_name_lang_unknown_12 TEXT NOT NULL,
        class_name_lang_unknown_13 TEXT NOT NULL,
        class_name_lang_unknown_14 TEXT NOT NULL,
        class_name_lang_unknown_15 TEXT NOT NULL,
        class_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemClass (
        class_id,
        subclass_map_id,
        flags,
        class_name_lang_en_gb,
        class_name_lang_ko_kr,
        class_name_lang_fr_fr,
        class_name_lang_de_de,
        class_name_lang_en_cn,
        class_name_lang_en_tw,
        class_name_lang_es_es,
        class_name_lang_es_mx,
        class_name_lang_ru_ru,
        class_name_lang_ja_jp,
        class_name_lang_pt_pt,
        class_name_lang_it_it,
        class_name_lang_unknown_12,
        class_name_lang_unknown_13,
        class_name_lang_unknown_14,
        class_name_lang_unknown_15,
        class_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemCondExtCosts() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemCondExtCosts (
        id INTEGER PRIMARY KEY NOT NULL,
        cond_extended_cost INTEGER  NOT NULL,
        item_extended_cost_entry INTEGER  NOT NULL,
        arena_season INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ItemCondExtCosts (
        id,
        cond_extended_cost,
        item_extended_cost_entry,
        arena_season
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemLimitCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemLimitCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        quantity INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ItemLimitCategory (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        quantity,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellCategory (
        id,
        flags
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtOCTRegenHP() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtOCTRegenHP (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtOCTRegenHP (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AreaGroup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaGroup (
        id INTEGER PRIMARY KEY NOT NULL,
        area_id_0 INTEGER NOT NULL,
        area_id_1 INTEGER NOT NULL,
        area_id_2 INTEGER NOT NULL,
        area_id_3 INTEGER NOT NULL,
        area_id_4 INTEGER NOT NULL,
        area_id_5 INTEGER NOT NULL,
        next_area_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AreaGroup (
        id,
        area_id_0,
        area_id_1,
        area_id_2,
        area_id_3,
        area_id_4,
        area_id_5,
        next_area_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TeamContributionPoints() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TeamContributionPoints (
        id INTEGER PRIMARY KEY NOT NULL,
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO TeamContributionPoints (
        id,
        data
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldMapArea() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapArea (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        area_id INTEGER  NOT NULL,
        area_name TEXT  NOT NULL,
        loc_left REAL  NOT NULL,
        loc_right REAL  NOT NULL,
        loc_top REAL  NOT NULL,
        loc_bottom REAL  NOT NULL,
        display_map_id INTEGER  NOT NULL,
        default_dungeon_floor INTEGER  NOT NULL,
        parent_world_map_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WorldMapArea (
        id,
        map_id,
        area_id,
        area_name,
        loc_left,
        loc_right,
        loc_top,
        loc_bottom,
        display_map_id,
        default_dungeon_floor,
        parent_world_map_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Lock() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Lock (
        id INTEGER PRIMARY KEY NOT NULL,
        ty_0 INTEGER NOT NULL,
        ty_1 INTEGER NOT NULL,
        ty_2 INTEGER NOT NULL,
        ty_3 INTEGER NOT NULL,
        ty_4 INTEGER NOT NULL,
        ty_5 INTEGER NOT NULL,
        ty_6 INTEGER NOT NULL,
        ty_7 INTEGER NOT NULL,
        index_0 INTEGER NOT NULL,
        index_1 INTEGER NOT NULL,
        index_2 INTEGER NOT NULL,
        index_3 INTEGER NOT NULL,
        index_4 INTEGER NOT NULL,
        index_5 INTEGER NOT NULL,
        index_6 INTEGER NOT NULL,
        index_7 INTEGER NOT NULL,
        skill_0 INTEGER NOT NULL,
        skill_1 INTEGER NOT NULL,
        skill_2 INTEGER NOT NULL,
        skill_3 INTEGER NOT NULL,
        skill_4 INTEGER NOT NULL,
        skill_5 INTEGER NOT NULL,
        skill_6 INTEGER NOT NULL,
        skill_7 INTEGER NOT NULL,
        action_0 INTEGER NOT NULL,
        action_1 INTEGER NOT NULL,
        action_2 INTEGER NOT NULL,
        action_3 INTEGER NOT NULL,
        action_4 INTEGER NOT NULL,
        action_5 INTEGER NOT NULL,
        action_6 INTEGER NOT NULL,
        action_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO Lock (
        id,
        ty_0,
        ty_1,
        ty_2,
        ty_3,
        ty_4,
        ty_5,
        ty_6,
        ty_7,
        index_0,
        index_1,
        index_2,
        index_3,
        index_4,
        index_5,
        index_6,
        index_7,
        skill_0,
        skill_1,
        skill_2,
        skill_3,
        skill_4,
        skill_5,
        skill_6,
        skill_7,
        action_0,
        action_1,
        action_2,
        action_3,
        action_4,
        action_5,
        action_6,
        action_7
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ScalingStatValues() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ScalingStatValues (
        id INTEGER PRIMARY KEY NOT NULL,
        charlevel INTEGER  NOT NULL,
        shoulder_budget INTEGER  NOT NULL,
        trinket_budget INTEGER  NOT NULL,
        weapon_budget1_h INTEGER  NOT NULL,
        ranged_budget INTEGER  NOT NULL,
        cloth_shoulder_armor INTEGER  NOT NULL,
        leather_shoulder_armor INTEGER  NOT NULL,
        mail_shoulder_armor INTEGER  NOT NULL,
        plate_shoulder_armor INTEGER  NOT NULL,
        weapon_d_p_s1_h INTEGER  NOT NULL,
        weapon_d_p_s2_h INTEGER  NOT NULL,
        spellcaster_d_p_s1_h INTEGER  NOT NULL,
        spellcaster_d_p_s2_h INTEGER  NOT NULL,
        ranged_d_p_s INTEGER  NOT NULL,
        wand_d_p_s INTEGER  NOT NULL,
        spell_power INTEGER  NOT NULL,
        primary_budget INTEGER  NOT NULL,
        tertiary_budget INTEGER  NOT NULL,
        cloth_cloak_armor INTEGER  NOT NULL,
        cloth_chest_armor INTEGER  NOT NULL,
        leather_chest_armor INTEGER  NOT NULL,
        mail_chest_armor INTEGER  NOT NULL,
        plate_chest_armor INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ScalingStatValues (
        id,
        charlevel,
        shoulder_budget,
        trinket_budget,
        weapon_budget1_h,
        ranged_budget,
        cloth_shoulder_armor,
        leather_shoulder_armor,
        mail_shoulder_armor,
        plate_shoulder_armor,
        weapon_d_p_s1_h,
        weapon_d_p_s2_h,
        spellcaster_d_p_s1_h,
        spellcaster_d_p_s2_h,
        ranged_d_p_s,
        wand_d_p_s,
        spell_power,
        primary_budget,
        tertiary_budget,
        cloth_cloak_armor,
        cloth_chest_armor,
        leather_chest_armor,
        mail_chest_armor,
        plate_chest_armor
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellCastTimes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellCastTimes (
        id INTEGER PRIMARY KEY NOT NULL,
        base INTEGER  NOT NULL,
        per_level INTEGER  NOT NULL,
        minimum INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellCastTimes (
        id,
        base,
        per_level,
        minimum
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn FootprintTextures() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FootprintTextures (
        id INTEGER PRIMARY KEY NOT NULL,
        footstep_filename TEXT  NOT NULL
    );"
    ,
    "INSERT INTO FootprintTextures (
        id,
        footstep_filename
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GameObjectArtKit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GameObjectArtKit (
        id INTEGER PRIMARY KEY NOT NULL,
        texture_variation_0 TEXT NOT NULL,
        texture_variation_1 TEXT NOT NULL,
        texture_variation_2 TEXT NOT NULL,
        attach_model_0 TEXT NOT NULL,
        attach_model_1 TEXT NOT NULL,
        attach_model_2 TEXT NOT NULL,
        attach_model_3 TEXT NOT NULL
    );"
    ,
    "INSERT INTO GameObjectArtKit (
        id,
        texture_variation_0,
        texture_variation_1,
        texture_variation_2,
        attach_model_0,
        attach_model_1,
        attach_model_2,
        attach_model_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellDuration() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellDuration (
        id INTEGER PRIMARY KEY NOT NULL,
        duration INTEGER  NOT NULL,
        duration_per_level INTEGER  NOT NULL,
        max_duration INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellDuration (
        id,
        duration,
        duration_per_level,
        max_duration
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn QuestSort() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS QuestSort (
        id INTEGER PRIMARY KEY NOT NULL,
        sort_name_lang_en_gb TEXT NOT NULL,
        sort_name_lang_ko_kr TEXT NOT NULL,
        sort_name_lang_fr_fr TEXT NOT NULL,
        sort_name_lang_de_de TEXT NOT NULL,
        sort_name_lang_en_cn TEXT NOT NULL,
        sort_name_lang_en_tw TEXT NOT NULL,
        sort_name_lang_es_es TEXT NOT NULL,
        sort_name_lang_es_mx TEXT NOT NULL,
        sort_name_lang_ru_ru TEXT NOT NULL,
        sort_name_lang_ja_jp TEXT NOT NULL,
        sort_name_lang_pt_pt TEXT NOT NULL,
        sort_name_lang_it_it TEXT NOT NULL,
        sort_name_lang_unknown_12 TEXT NOT NULL,
        sort_name_lang_unknown_13 TEXT NOT NULL,
        sort_name_lang_unknown_14 TEXT NOT NULL,
        sort_name_lang_unknown_15 TEXT NOT NULL,
        sort_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO QuestSort (
        id,
        sort_name_lang_en_gb,
        sort_name_lang_ko_kr,
        sort_name_lang_fr_fr,
        sort_name_lang_de_de,
        sort_name_lang_en_cn,
        sort_name_lang_en_tw,
        sort_name_lang_es_es,
        sort_name_lang_es_mx,
        sort_name_lang_ru_ru,
        sort_name_lang_ja_jp,
        sort_name_lang_pt_pt,
        sort_name_lang_it_it,
        sort_name_lang_unknown_12,
        sort_name_lang_unknown_13,
        sort_name_lang_unknown_14,
        sort_name_lang_unknown_15,
        sort_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn UISoundLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS UISoundLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_id INTEGER  NOT NULL,
        sound_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO UISoundLookups (
        id,
        sound_id,
        sound_name
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GroundEffectTexture() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GroundEffectTexture (
        id INTEGER PRIMARY KEY NOT NULL,
        doodad_id_0 INTEGER NOT NULL,
        doodad_id_1 INTEGER NOT NULL,
        doodad_id_2 INTEGER NOT NULL,
        doodad_id_3 INTEGER NOT NULL,
        doodad_weight_0 INTEGER NOT NULL,
        doodad_weight_1 INTEGER NOT NULL,
        doodad_weight_2 INTEGER NOT NULL,
        doodad_weight_3 INTEGER NOT NULL,
        density INTEGER  NOT NULL,
        sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GroundEffectTexture (
        id,
        doodad_id_0,
        doodad_id_1,
        doodad_id_2,
        doodad_id_3,
        doodad_weight_0,
        doodad_weight_1,
        doodad_weight_2,
        doodad_weight_3,
        density,
        sound
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ChatChannels() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChatChannels (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        faction_group INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        shortcut_lang_en_gb TEXT NOT NULL,
        shortcut_lang_ko_kr TEXT NOT NULL,
        shortcut_lang_fr_fr TEXT NOT NULL,
        shortcut_lang_de_de TEXT NOT NULL,
        shortcut_lang_en_cn TEXT NOT NULL,
        shortcut_lang_en_tw TEXT NOT NULL,
        shortcut_lang_es_es TEXT NOT NULL,
        shortcut_lang_es_mx TEXT NOT NULL,
        shortcut_lang_ru_ru TEXT NOT NULL,
        shortcut_lang_ja_jp TEXT NOT NULL,
        shortcut_lang_pt_pt TEXT NOT NULL,
        shortcut_lang_it_it TEXT NOT NULL,
        shortcut_lang_unknown_12 TEXT NOT NULL,
        shortcut_lang_unknown_13 TEXT NOT NULL,
        shortcut_lang_unknown_14 TEXT NOT NULL,
        shortcut_lang_unknown_15 TEXT NOT NULL,
        shortcut_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ChatChannels (
        id,
        flags,
        faction_group,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        shortcut_lang_en_gb,
        shortcut_lang_ko_kr,
        shortcut_lang_fr_fr,
        shortcut_lang_de_de,
        shortcut_lang_en_cn,
        shortcut_lang_en_tw,
        shortcut_lang_es_es,
        shortcut_lang_es_mx,
        shortcut_lang_ru_ru,
        shortcut_lang_ja_jp,
        shortcut_lang_pt_pt,
        shortcut_lang_it_it,
        shortcut_lang_unknown_12,
        shortcut_lang_unknown_13,
        shortcut_lang_unknown_14,
        shortcut_lang_unknown_15,
        shortcut_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CurrencyTypes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CurrencyTypes (
        id INTEGER PRIMARY KEY NOT NULL,
        item_id INTEGER  NOT NULL,
        category_id INTEGER  NOT NULL,
        bit_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CurrencyTypes (
        id,
        item_id,
        category_id,
        bit_index
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CurrencyCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CurrencyCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO CurrencyCategory (
        id,
        flags,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn EmotesTextData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EmotesTextData (
        id INTEGER PRIMARY KEY NOT NULL,
        text_lang_en_gb TEXT NOT NULL,
        text_lang_ko_kr TEXT NOT NULL,
        text_lang_fr_fr TEXT NOT NULL,
        text_lang_de_de TEXT NOT NULL,
        text_lang_en_cn TEXT NOT NULL,
        text_lang_en_tw TEXT NOT NULL,
        text_lang_es_es TEXT NOT NULL,
        text_lang_es_mx TEXT NOT NULL,
        text_lang_ru_ru TEXT NOT NULL,
        text_lang_ja_jp TEXT NOT NULL,
        text_lang_pt_pt TEXT NOT NULL,
        text_lang_it_it TEXT NOT NULL,
        text_lang_unknown_12 TEXT NOT NULL,
        text_lang_unknown_13 TEXT NOT NULL,
        text_lang_unknown_14 TEXT NOT NULL,
        text_lang_unknown_15 TEXT NOT NULL,
        text_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO EmotesTextData (
        id,
        text_lang_en_gb,
        text_lang_ko_kr,
        text_lang_fr_fr,
        text_lang_de_de,
        text_lang_en_cn,
        text_lang_en_tw,
        text_lang_es_es,
        text_lang_es_mx,
        text_lang_ru_ru,
        text_lang_ja_jp,
        text_lang_pt_pt,
        text_lang_it_it,
        text_lang_unknown_12,
        text_lang_unknown_13,
        text_lang_unknown_14,
        text_lang_unknown_15,
        text_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn EmotesTextSound() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EmotesTextSound (
        id INTEGER PRIMARY KEY NOT NULL,
        emotes_text_id INTEGER  NOT NULL,
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO EmotesTextSound (
        id,
        emotes_text_id,
        race_id,
        sex_id,
        sound_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Light() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Light (
        id INTEGER PRIMARY KEY NOT NULL,
        continent_id INTEGER  NOT NULL,
        game_coords_0 REAL NOT NULL,
        game_coords_1 REAL NOT NULL,
        game_coords_2 REAL NOT NULL,
        game_falloff_start REAL  NOT NULL,
        game_falloff_end REAL  NOT NULL,
        light_params_id_0 INTEGER NOT NULL,
        light_params_id_1 INTEGER NOT NULL,
        light_params_id_2 INTEGER NOT NULL,
        light_params_id_3 INTEGER NOT NULL,
        light_params_id_4 INTEGER NOT NULL,
        light_params_id_5 INTEGER NOT NULL,
        light_params_id_6 INTEGER NOT NULL,
        light_params_id_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO Light (
        id,
        continent_id,
        game_coords_0,
        game_coords_1,
        game_coords_2,
        game_falloff_start,
        game_falloff_end,
        light_params_id_0,
        light_params_id_1,
        light_params_id_2,
        light_params_id_3,
        light_params_id_4,
        light_params_id_5,
        light_params_id_6,
        light_params_id_7
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureFamily() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureFamily (
        id INTEGER PRIMARY KEY NOT NULL,
        min_scale REAL  NOT NULL,
        min_scale_level INTEGER  NOT NULL,
        max_scale REAL  NOT NULL,
        max_scale_level INTEGER  NOT NULL,
        skill_line_0 INTEGER NOT NULL,
        skill_line_1 INTEGER NOT NULL,
        pet_food_mask INTEGER  NOT NULL,
        pet_talent_type INTEGER  NOT NULL,
        category_enum_id INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        icon_file TEXT  NOT NULL
    );"
    ,
    "INSERT INTO CreatureFamily (
        id,
        min_scale,
        min_scale_level,
        max_scale,
        max_scale_level,
        skill_line_0,
        skill_line_1,
        pet_food_mask,
        pet_talent_type,
        category_enum_id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        icon_file
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Talent() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Talent (
        id INTEGER PRIMARY KEY NOT NULL,
        tab_id INTEGER  NOT NULL,
        tier_id INTEGER  NOT NULL,
        column_index INTEGER  NOT NULL,
        spell_rank_0 INTEGER NOT NULL,
        spell_rank_1 INTEGER NOT NULL,
        spell_rank_2 INTEGER NOT NULL,
        spell_rank_3 INTEGER NOT NULL,
        spell_rank_4 INTEGER NOT NULL,
        spell_rank_5 INTEGER NOT NULL,
        spell_rank_6 INTEGER NOT NULL,
        spell_rank_7 INTEGER NOT NULL,
        spell_rank_8 INTEGER NOT NULL,
        prereq_talent_0 INTEGER NOT NULL,
        prereq_talent_1 INTEGER NOT NULL,
        prereq_talent_2 INTEGER NOT NULL,
        prereq_rank_0 INTEGER NOT NULL,
        prereq_rank_1 INTEGER NOT NULL,
        prereq_rank_2 INTEGER NOT NULL,
        flags INTEGER  NOT NULL,
        required_spell_id INTEGER  NOT NULL,
        category_mask_0 INTEGER NOT NULL,
        category_mask_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO Talent (
        id,
        tab_id,
        tier_id,
        column_index,
        spell_rank_0,
        spell_rank_1,
        spell_rank_2,
        spell_rank_3,
        spell_rank_4,
        spell_rank_5,
        spell_rank_6,
        spell_rank_7,
        spell_rank_8,
        prereq_talent_0,
        prereq_talent_1,
        prereq_talent_2,
        prereq_rank_0,
        prereq_rank_1,
        prereq_rank_2,
        flags,
        required_spell_id,
        category_mask_0,
        category_mask_1
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemRandomProperties() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemRandomProperties (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        enchantment_0 INTEGER NOT NULL,
        enchantment_1 INTEGER NOT NULL,
        enchantment_2 INTEGER NOT NULL,
        enchantment_3 INTEGER NOT NULL,
        enchantment_4 INTEGER NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemRandomProperties (
        id,
        name,
        enchantment_0,
        enchantment_1,
        enchantment_2,
        enchantment_3,
        enchantment_4,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DurabilityCosts() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DurabilityCosts (
        id INTEGER PRIMARY KEY NOT NULL,
        weapon_sub_class_cost_0 INTEGER NOT NULL,
        weapon_sub_class_cost_1 INTEGER NOT NULL,
        weapon_sub_class_cost_2 INTEGER NOT NULL,
        weapon_sub_class_cost_3 INTEGER NOT NULL,
        weapon_sub_class_cost_4 INTEGER NOT NULL,
        weapon_sub_class_cost_5 INTEGER NOT NULL,
        weapon_sub_class_cost_6 INTEGER NOT NULL,
        weapon_sub_class_cost_7 INTEGER NOT NULL,
        weapon_sub_class_cost_8 INTEGER NOT NULL,
        weapon_sub_class_cost_9 INTEGER NOT NULL,
        weapon_sub_class_cost_10 INTEGER NOT NULL,
        weapon_sub_class_cost_11 INTEGER NOT NULL,
        weapon_sub_class_cost_12 INTEGER NOT NULL,
        weapon_sub_class_cost_13 INTEGER NOT NULL,
        weapon_sub_class_cost_14 INTEGER NOT NULL,
        weapon_sub_class_cost_15 INTEGER NOT NULL,
        weapon_sub_class_cost_16 INTEGER NOT NULL,
        weapon_sub_class_cost_17 INTEGER NOT NULL,
        weapon_sub_class_cost_18 INTEGER NOT NULL,
        weapon_sub_class_cost_19 INTEGER NOT NULL,
        weapon_sub_class_cost_20 INTEGER NOT NULL,
        armor_sub_class_cost_0 INTEGER NOT NULL,
        armor_sub_class_cost_1 INTEGER NOT NULL,
        armor_sub_class_cost_2 INTEGER NOT NULL,
        armor_sub_class_cost_3 INTEGER NOT NULL,
        armor_sub_class_cost_4 INTEGER NOT NULL,
        armor_sub_class_cost_5 INTEGER NOT NULL,
        armor_sub_class_cost_6 INTEGER NOT NULL,
        armor_sub_class_cost_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO DurabilityCosts (
        id,
        weapon_sub_class_cost_0,
        weapon_sub_class_cost_1,
        weapon_sub_class_cost_2,
        weapon_sub_class_cost_3,
        weapon_sub_class_cost_4,
        weapon_sub_class_cost_5,
        weapon_sub_class_cost_6,
        weapon_sub_class_cost_7,
        weapon_sub_class_cost_8,
        weapon_sub_class_cost_9,
        weapon_sub_class_cost_10,
        weapon_sub_class_cost_11,
        weapon_sub_class_cost_12,
        weapon_sub_class_cost_13,
        weapon_sub_class_cost_14,
        weapon_sub_class_cost_15,
        weapon_sub_class_cost_16,
        weapon_sub_class_cost_17,
        weapon_sub_class_cost_18,
        weapon_sub_class_cost_19,
        weapon_sub_class_cost_20,
        armor_sub_class_cost_0,
        armor_sub_class_cost_1,
        armor_sub_class_cost_2,
        armor_sub_class_cost_3,
        armor_sub_class_cost_4,
        armor_sub_class_cost_5,
        armor_sub_class_cost_6,
        armor_sub_class_cost_7
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellChainEffects() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellChainEffects (
        id INTEGER PRIMARY KEY NOT NULL,
        avg_seg_len REAL  NOT NULL,
        width REAL  NOT NULL,
        noise_scale REAL  NOT NULL,
        tex_coord_scale REAL  NOT NULL,
        seg_duration INTEGER  NOT NULL,
        seg_delay INTEGER  NOT NULL,
        texture TEXT  NOT NULL,
        flags INTEGER  NOT NULL,
        joint_count INTEGER  NOT NULL,
        joint_offset_radius REAL  NOT NULL,
        joints_per_minor_joint INTEGER  NOT NULL,
        minor_joints_per_major_joint INTEGER  NOT NULL,
        minor_joint_scale REAL  NOT NULL,
        major_joint_scale REAL  NOT NULL,
        joint_move_speed REAL  NOT NULL,
        joint_smoothness REAL  NOT NULL,
        min_duration_between_joint_jumps REAL  NOT NULL,
        max_duration_between_joint_jumps REAL  NOT NULL,
        wave_height REAL  NOT NULL,
        wave_freq REAL  NOT NULL,
        wave_speed REAL  NOT NULL,
        min_wave_angle REAL  NOT NULL,
        max_wave_angle REAL  NOT NULL,
        min_wave_spin REAL  NOT NULL,
        max_wave_spin REAL  NOT NULL,
        arc_height REAL  NOT NULL,
        min_arc_angle REAL  NOT NULL,
        max_arc_angle REAL  NOT NULL,
        min_arc_spin REAL  NOT NULL,
        max_arc_spin REAL  NOT NULL,
        delay_between_effects REAL  NOT NULL,
        min_flicker_on_duration REAL  NOT NULL,
        max_flicker_on_duration REAL  NOT NULL,
        min_flicker_off_duration REAL  NOT NULL,
        max_flicker_off_duration REAL  NOT NULL,
        pulse_speed REAL  NOT NULL,
        pulse_on_length REAL  NOT NULL,
        pulse_fade_length REAL  NOT NULL,
        alpha INTEGER  NOT NULL,
        red INTEGER  NOT NULL,
        green INTEGER  NOT NULL,
        blue INTEGER  NOT NULL,
        blend_mode INTEGER  NOT NULL,
        combo TEXT  NOT NULL,
        render_layer INTEGER  NOT NULL,
        texture_length REAL  NOT NULL,
        wave_phase REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellChainEffects (
        id,
        avg_seg_len,
        width,
        noise_scale,
        tex_coord_scale,
        seg_duration,
        seg_delay,
        texture,
        flags,
        joint_count,
        joint_offset_radius,
        joints_per_minor_joint,
        minor_joints_per_major_joint,
        minor_joint_scale,
        major_joint_scale,
        joint_move_speed,
        joint_smoothness,
        min_duration_between_joint_jumps,
        max_duration_between_joint_jumps,
        wave_height,
        wave_freq,
        wave_speed,
        min_wave_angle,
        max_wave_angle,
        min_wave_spin,
        max_wave_spin,
        arc_height,
        min_arc_angle,
        max_arc_angle,
        min_arc_spin,
        max_arc_spin,
        delay_between_effects,
        min_flicker_on_duration,
        max_flicker_on_duration,
        min_flicker_off_duration,
        max_flicker_off_duration,
        pulse_speed,
        pulse_on_length,
        pulse_fade_length,
        alpha,
        red,
        green,
        blue,
        blend_mode,
        combo,
        render_layer,
        texture_length,
        wave_phase
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn OverrideSpellData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS OverrideSpellData (
        id INTEGER PRIMARY KEY NOT NULL,
        spells_0 INTEGER NOT NULL,
        spells_1 INTEGER NOT NULL,
        spells_2 INTEGER NOT NULL,
        spells_3 INTEGER NOT NULL,
        spells_4 INTEGER NOT NULL,
        spells_5 INTEGER NOT NULL,
        spells_6 INTEGER NOT NULL,
        spells_7 INTEGER NOT NULL,
        spells_8 INTEGER NOT NULL,
        spells_9 INTEGER NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO OverrideSpellData (
        id,
        spells_0,
        spells_1,
        spells_2,
        spells_3,
        spells_4,
        spells_5,
        spells_6,
        spells_7,
        spells_8,
        spells_9,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellMissile() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellMissile (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        default_pitch_min REAL  NOT NULL,
        default_pitch_max REAL  NOT NULL,
        default_speed_min REAL  NOT NULL,
        default_speed_max REAL  NOT NULL,
        randomize_facing_min REAL  NOT NULL,
        randomize_facing_max REAL  NOT NULL,
        randomize_pitch_min REAL  NOT NULL,
        randomize_pitch_max REAL  NOT NULL,
        randomize_speed_min REAL  NOT NULL,
        randomize_speed_max REAL  NOT NULL,
        gravity REAL  NOT NULL,
        max_duration REAL  NOT NULL,
        collision_radius REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellMissile (
        id,
        flags,
        default_pitch_min,
        default_pitch_max,
        default_speed_min,
        default_speed_max,
        randomize_facing_min,
        randomize_facing_max,
        randomize_pitch_min,
        randomize_pitch_max,
        randomize_speed_min,
        randomize_speed_max,
        gravity,
        max_duration,
        collision_radius
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellRuneCost() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellRuneCost (
        id INTEGER PRIMARY KEY NOT NULL,
        blood INTEGER  NOT NULL,
        unholy INTEGER  NOT NULL,
        frost INTEGER  NOT NULL,
        runic_power INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellRuneCost (
        id,
        blood,
        unholy,
        frost,
        runic_power
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CinematicCamera() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CinematicCamera (
        id INTEGER PRIMARY KEY NOT NULL,
        model TEXT  NOT NULL,
        sound_id INTEGER  NOT NULL,
        origin_0 REAL NOT NULL,
        origin_1 REAL NOT NULL,
        origin_2 REAL NOT NULL,
        origin_facing REAL  NOT NULL
    );"
    ,
    "INSERT INTO CinematicCamera (
        id,
        model,
        sound_id,
        origin_0,
        origin_1,
        origin_2,
        origin_facing
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn FileData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FileData (
        id INTEGER PRIMARY KEY NOT NULL,
        filename TEXT  NOT NULL,
        filepath TEXT  NOT NULL
    );"
    ,
    "INSERT INTO FileData (
        id,
        filename,
        filepath
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ObjectEffectModifier() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ObjectEffectModifier (
        id INTEGER PRIMARY KEY NOT NULL,
        input_type INTEGER  NOT NULL,
        map_type INTEGER  NOT NULL,
        output_type INTEGER  NOT NULL,
        param_0 REAL NOT NULL,
        param_1 REAL NOT NULL,
        param_2 REAL NOT NULL,
        param_3 REAL NOT NULL
    );"
    ,
    "INSERT INTO ObjectEffectModifier (
        id,
        input_type,
        map_type,
        output_type,
        param_0,
        param_1,
        param_2,
        param_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureSoundData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureSoundData (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_exertion_id INTEGER  NOT NULL,
        sound_exertion_critical_id INTEGER  NOT NULL,
        sound_injury_id INTEGER  NOT NULL,
        sound_injury_critical_id INTEGER  NOT NULL,
        sound_injury_crushing_blow_id INTEGER  NOT NULL,
        sound_death_id INTEGER  NOT NULL,
        sound_stun_id INTEGER  NOT NULL,
        sound_stand_id INTEGER  NOT NULL,
        sound_footstep_id INTEGER  NOT NULL,
        sound_aggro_id INTEGER  NOT NULL,
        sound_wing_flap_id INTEGER  NOT NULL,
        sound_wing_glide_id INTEGER  NOT NULL,
        sound_alert_id INTEGER  NOT NULL,
        sound_fidget_0 INTEGER NOT NULL,
        sound_fidget_1 INTEGER NOT NULL,
        sound_fidget_2 INTEGER NOT NULL,
        sound_fidget_3 INTEGER NOT NULL,
        sound_fidget_4 INTEGER NOT NULL,
        custom_attack_0 INTEGER NOT NULL,
        custom_attack_1 INTEGER NOT NULL,
        custom_attack_2 INTEGER NOT NULL,
        custom_attack_3 INTEGER NOT NULL,
        n_p_c_sound_id INTEGER  NOT NULL,
        loop_sound_id INTEGER  NOT NULL,
        creature_impact_type INTEGER  NOT NULL,
        sound_jump_start_id INTEGER  NOT NULL,
        sound_jump_end_id INTEGER  NOT NULL,
        sound_pet_attack_id INTEGER  NOT NULL,
        sound_pet_order_id INTEGER  NOT NULL,
        sound_pet_dismiss_id INTEGER  NOT NULL,
        fidget_delay_seconds_min REAL  NOT NULL,
        fidget_delay_seconds_max REAL  NOT NULL,
        birth_sound_id INTEGER  NOT NULL,
        spell_cast_directed_sound_id INTEGER  NOT NULL,
        submerge_sound_id INTEGER  NOT NULL,
        submerged_sound_id INTEGER  NOT NULL,
        creature_sound_data_id_pet INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CreatureSoundData (
        id,
        sound_exertion_id,
        sound_exertion_critical_id,
        sound_injury_id,
        sound_injury_critical_id,
        sound_injury_crushing_blow_id,
        sound_death_id,
        sound_stun_id,
        sound_stand_id,
        sound_footstep_id,
        sound_aggro_id,
        sound_wing_flap_id,
        sound_wing_glide_id,
        sound_alert_id,
        sound_fidget_0,
        sound_fidget_1,
        sound_fidget_2,
        sound_fidget_3,
        sound_fidget_4,
        custom_attack_0,
        custom_attack_1,
        custom_attack_2,
        custom_attack_3,
        n_p_c_sound_id,
        loop_sound_id,
        creature_impact_type,
        sound_jump_start_id,
        sound_jump_end_id,
        sound_pet_attack_id,
        sound_pet_order_id,
        sound_pet_dismiss_id,
        fidget_delay_seconds_min,
        fidget_delay_seconds_max,
        birth_sound_id,
        spell_cast_directed_sound_id,
        submerge_sound_id,
        submerged_sound_id,
        creature_sound_data_id_pet
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn VideoHardware() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS VideoHardware (
        id INTEGER PRIMARY KEY NOT NULL,
        vendor_id INTEGER  NOT NULL,
        device_id INTEGER  NOT NULL,
        farclip_idx INTEGER  NOT NULL,
        terrain_l_o_d_dist_idx INTEGER  NOT NULL,
        terrain_shadow_l_o_d INTEGER  NOT NULL,
        detail_doodad_density_idx INTEGER  NOT NULL,
        detail_doodad_alpha INTEGER  NOT NULL,
        animating_doodad_idx INTEGER  NOT NULL,
        trilinear INTEGER  NOT NULL,
        num_lights INTEGER  NOT NULL,
        specularity INTEGER  NOT NULL,
        water_l_o_d_idx INTEGER  NOT NULL,
        particle_density_idx INTEGER  NOT NULL,
        unit_draw_dist_idx INTEGER  NOT NULL,
        small_cull_dist_idx INTEGER  NOT NULL,
        resolution_idx INTEGER  NOT NULL,
        base_mip_level INTEGER  NOT NULL,
        ogl_overrides TEXT  NOT NULL,
        d3d_overrides TEXT  NOT NULL,
        fix_lag INTEGER  NOT NULL,
        multisample INTEGER  NOT NULL,
        atlasdisable INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO VideoHardware (
        id,
        vendor_id,
        device_id,
        farclip_idx,
        terrain_l_o_d_dist_idx,
        terrain_shadow_l_o_d,
        detail_doodad_density_idx,
        detail_doodad_alpha,
        animating_doodad_idx,
        trilinear,
        num_lights,
        specularity,
        water_l_o_d_idx,
        particle_density_idx,
        unit_draw_dist_idx,
        small_cull_dist_idx,
        resolution_idx,
        base_mip_level,
        ogl_overrides,
        d3d_overrides,
        fix_lag,
        multisample,
        atlasdisable
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn PageTextMaterial() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PageTextMaterial (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO PageTextMaterial (
        id,
        name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ParticleColor() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ParticleColor (
        id INTEGER PRIMARY KEY NOT NULL,
        start_0 INTEGER NOT NULL,
        start_1 INTEGER NOT NULL,
        start_2 INTEGER NOT NULL,
        m_id_0 INTEGER NOT NULL,
        m_id_1 INTEGER NOT NULL,
        m_id_2 INTEGER NOT NULL,
        end_0 INTEGER NOT NULL,
        end_1 INTEGER NOT NULL,
        end_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ParticleColor (
        id,
        start_0,
        start_1,
        start_2,
        m_id_0,
        m_id_1,
        m_id_2,
        end_0,
        end_1,
        end_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GemProperties() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GemProperties (
        id INTEGER PRIMARY KEY NOT NULL,
        enchant_id INTEGER  NOT NULL,
        maxcount_inv INTEGER  NOT NULL,
        maxcount_item INTEGER  NOT NULL,
        ty INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GemProperties (
        id,
        enchant_id,
        maxcount_inv,
        maxcount_item,
        ty
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CameraShakes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CameraShakes (
        id INTEGER PRIMARY KEY NOT NULL,
        shake_type INTEGER  NOT NULL,
        direction INTEGER  NOT NULL,
        amplitude REAL  NOT NULL,
        frequency REAL  NOT NULL,
        duration REAL  NOT NULL,
        phase REAL  NOT NULL,
        coefficient REAL  NOT NULL
    );"
    ,
    "INSERT INTO CameraShakes (
        id,
        shake_type,
        direction,
        amplitude,
        frequency,
        duration,
        phase,
        coefficient
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GameTables() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GameTables (
        name TEXT  NOT NULL,
        num_rows INTEGER  NOT NULL,
        num_columns INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GameTables (
        name,
        num_rows,
        num_columns
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Languages() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Languages (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Languages (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Item() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Item (
        id INTEGER PRIMARY KEY NOT NULL,
        class_id INTEGER  NOT NULL,
        subclass_id INTEGER  NOT NULL,
        sound_override_subclass_id INTEGER  NOT NULL,
        material INTEGER  NOT NULL,
        display_info_id INTEGER  NOT NULL,
        inventory_type INTEGER  NOT NULL,
        sheathe_type INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Item (
        id,
        class_id,
        subclass_id,
        sound_override_subclass_id,
        material,
        display_info_id,
        inventory_type,
        sheathe_type
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Resistances() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Resistances (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        fizzle_sound_id INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Resistances (
        id,
        flags,
        fizzle_sound_id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AttackAnimTypes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AttackAnimTypes (
        anim_id INTEGER  NOT NULL,
        anim_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO AttackAnimTypes (
        anim_id,
        anim_name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Map() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Map (
        id INTEGER PRIMARY KEY NOT NULL,
        directory TEXT  NOT NULL,
        instance_type INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        p_v_p INTEGER  NOT NULL,
        map_name_lang_en_gb TEXT NOT NULL,
        map_name_lang_ko_kr TEXT NOT NULL,
        map_name_lang_fr_fr TEXT NOT NULL,
        map_name_lang_de_de TEXT NOT NULL,
        map_name_lang_en_cn TEXT NOT NULL,
        map_name_lang_en_tw TEXT NOT NULL,
        map_name_lang_es_es TEXT NOT NULL,
        map_name_lang_es_mx TEXT NOT NULL,
        map_name_lang_ru_ru TEXT NOT NULL,
        map_name_lang_ja_jp TEXT NOT NULL,
        map_name_lang_pt_pt TEXT NOT NULL,
        map_name_lang_it_it TEXT NOT NULL,
        map_name_lang_unknown_12 TEXT NOT NULL,
        map_name_lang_unknown_13 TEXT NOT NULL,
        map_name_lang_unknown_14 TEXT NOT NULL,
        map_name_lang_unknown_15 TEXT NOT NULL,
        map_name_lang_flags TEXT NOT NULL,
        area_table_id INTEGER  NOT NULL,
        map_description0_lang_en_gb TEXT NOT NULL,
        map_description0_lang_ko_kr TEXT NOT NULL,
        map_description0_lang_fr_fr TEXT NOT NULL,
        map_description0_lang_de_de TEXT NOT NULL,
        map_description0_lang_en_cn TEXT NOT NULL,
        map_description0_lang_en_tw TEXT NOT NULL,
        map_description0_lang_es_es TEXT NOT NULL,
        map_description0_lang_es_mx TEXT NOT NULL,
        map_description0_lang_ru_ru TEXT NOT NULL,
        map_description0_lang_ja_jp TEXT NOT NULL,
        map_description0_lang_pt_pt TEXT NOT NULL,
        map_description0_lang_it_it TEXT NOT NULL,
        map_description0_lang_unknown_12 TEXT NOT NULL,
        map_description0_lang_unknown_13 TEXT NOT NULL,
        map_description0_lang_unknown_14 TEXT NOT NULL,
        map_description0_lang_unknown_15 TEXT NOT NULL,
        map_description0_lang_flags TEXT NOT NULL,
        map_description1_lang_en_gb TEXT NOT NULL,
        map_description1_lang_ko_kr TEXT NOT NULL,
        map_description1_lang_fr_fr TEXT NOT NULL,
        map_description1_lang_de_de TEXT NOT NULL,
        map_description1_lang_en_cn TEXT NOT NULL,
        map_description1_lang_en_tw TEXT NOT NULL,
        map_description1_lang_es_es TEXT NOT NULL,
        map_description1_lang_es_mx TEXT NOT NULL,
        map_description1_lang_ru_ru TEXT NOT NULL,
        map_description1_lang_ja_jp TEXT NOT NULL,
        map_description1_lang_pt_pt TEXT NOT NULL,
        map_description1_lang_it_it TEXT NOT NULL,
        map_description1_lang_unknown_12 TEXT NOT NULL,
        map_description1_lang_unknown_13 TEXT NOT NULL,
        map_description1_lang_unknown_14 TEXT NOT NULL,
        map_description1_lang_unknown_15 TEXT NOT NULL,
        map_description1_lang_flags TEXT NOT NULL,
        loading_screen_id INTEGER  NOT NULL,
        minimap_icon_scale REAL  NOT NULL,
        corpse_map_id INTEGER  NOT NULL,
        corpse_0 REAL NOT NULL,
        corpse_1 REAL NOT NULL,
        time_of_day_override INTEGER  NOT NULL,
        expansion_id INTEGER  NOT NULL,
        raid_offset INTEGER  NOT NULL,
        max_players INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Map (
        id,
        directory,
        instance_type,
        flags,
        p_v_p,
        map_name_lang_en_gb,
        map_name_lang_ko_kr,
        map_name_lang_fr_fr,
        map_name_lang_de_de,
        map_name_lang_en_cn,
        map_name_lang_en_tw,
        map_name_lang_es_es,
        map_name_lang_es_mx,
        map_name_lang_ru_ru,
        map_name_lang_ja_jp,
        map_name_lang_pt_pt,
        map_name_lang_it_it,
        map_name_lang_unknown_12,
        map_name_lang_unknown_13,
        map_name_lang_unknown_14,
        map_name_lang_unknown_15,
        map_name_lang_flags,
        area_table_id,
        map_description0_lang_en_gb,
        map_description0_lang_ko_kr,
        map_description0_lang_fr_fr,
        map_description0_lang_de_de,
        map_description0_lang_en_cn,
        map_description0_lang_en_tw,
        map_description0_lang_es_es,
        map_description0_lang_es_mx,
        map_description0_lang_ru_ru,
        map_description0_lang_ja_jp,
        map_description0_lang_pt_pt,
        map_description0_lang_it_it,
        map_description0_lang_unknown_12,
        map_description0_lang_unknown_13,
        map_description0_lang_unknown_14,
        map_description0_lang_unknown_15,
        map_description0_lang_flags,
        map_description1_lang_en_gb,
        map_description1_lang_ko_kr,
        map_description1_lang_fr_fr,
        map_description1_lang_de_de,
        map_description1_lang_en_cn,
        map_description1_lang_en_tw,
        map_description1_lang_es_es,
        map_description1_lang_es_mx,
        map_description1_lang_ru_ru,
        map_description1_lang_ja_jp,
        map_description1_lang_pt_pt,
        map_description1_lang_it_it,
        map_description1_lang_unknown_12,
        map_description1_lang_unknown_13,
        map_description1_lang_unknown_14,
        map_description1_lang_unknown_15,
        map_description1_lang_flags,
        loading_screen_id,
        minimap_icon_scale,
        corpse_map_id,
        corpse_0,
        corpse_1,
        time_of_day_override,
        expansion_id,
        raid_offset,
        max_players
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60,
        ?61,
        ?62,
        ?63,
        ?64,
        ?65,
        ?66
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TotemCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TotemCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        totem_category_type INTEGER  NOT NULL,
        totem_category_mask INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TotemCategory (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        totem_category_type,
        totem_category_mask
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NamesReserved() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NamesReserved (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        language INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO NamesReserved (
        id,
        name,
        language
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellVisualKitAreaModel() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualKitAreaModel (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        enum_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualKitAreaModel (
        id,
        name,
        enum_id
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureDisplayInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureDisplayInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        model_id INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL,
        extended_display_info_id INTEGER  NOT NULL,
        creature_model_scale REAL  NOT NULL,
        creature_model_alpha INTEGER  NOT NULL,
        texture_variation_0 TEXT NOT NULL,
        texture_variation_1 TEXT NOT NULL,
        texture_variation_2 TEXT NOT NULL,
        portrait_texture_name TEXT  NOT NULL,
        size_class INTEGER  NOT NULL,
        blood_id INTEGER  NOT NULL,
        n_p_c_sound_id INTEGER  NOT NULL,
        particle_color_id INTEGER  NOT NULL,
        creature_geoset_data INTEGER  NOT NULL,
        object_effect_package_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CreatureDisplayInfo (
        id,
        model_id,
        sound_id,
        extended_display_info_id,
        creature_model_scale,
        creature_model_alpha,
        texture_variation_0,
        texture_variation_1,
        texture_variation_2,
        portrait_texture_name,
        size_class,
        blood_id,
        n_p_c_sound_id,
        particle_color_id,
        creature_geoset_data,
        object_effect_package_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn RandPropPoints() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS RandPropPoints (
        id INTEGER PRIMARY KEY NOT NULL,
        epic_0 INTEGER NOT NULL,
        epic_1 INTEGER NOT NULL,
        epic_2 INTEGER NOT NULL,
        epic_3 INTEGER NOT NULL,
        epic_4 INTEGER NOT NULL,
        superior_0 INTEGER NOT NULL,
        superior_1 INTEGER NOT NULL,
        superior_2 INTEGER NOT NULL,
        superior_3 INTEGER NOT NULL,
        superior_4 INTEGER NOT NULL,
        good_0 INTEGER NOT NULL,
        good_1 INTEGER NOT NULL,
        good_2 INTEGER NOT NULL,
        good_3 INTEGER NOT NULL,
        good_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO RandPropPoints (
        id,
        epic_0,
        epic_1,
        epic_2,
        epic_3,
        epic_4,
        superior_0,
        superior_1,
        superior_2,
        superior_3,
        superior_4,
        good_0,
        good_1,
        good_2,
        good_3,
        good_4
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ScreenEffect() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ScreenEffect (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        effect INTEGER  NOT NULL,
        param_0 INTEGER NOT NULL,
        param_1 INTEGER NOT NULL,
        param_2 INTEGER NOT NULL,
        param_3 INTEGER NOT NULL,
        light_params_id INTEGER  NOT NULL,
        sound_ambience_id INTEGER  NOT NULL,
        zone_music_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ScreenEffect (
        id,
        name,
        effect,
        param_0,
        param_1,
        param_2,
        param_3,
        light_params_id,
        sound_ambience_id,
        zone_music_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LFGDungeonGroup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LFGDungeonGroup (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        order_index INTEGER  NOT NULL,
        parent_group_id INTEGER  NOT NULL,
        type_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LFGDungeonGroup (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        order_index,
        parent_group_id,
        type_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn MapDifficulty() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS MapDifficulty (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        difficulty INTEGER  NOT NULL,
        message_lang_en_gb TEXT NOT NULL,
        message_lang_ko_kr TEXT NOT NULL,
        message_lang_fr_fr TEXT NOT NULL,
        message_lang_de_de TEXT NOT NULL,
        message_lang_en_cn TEXT NOT NULL,
        message_lang_en_tw TEXT NOT NULL,
        message_lang_es_es TEXT NOT NULL,
        message_lang_es_mx TEXT NOT NULL,
        message_lang_ru_ru TEXT NOT NULL,
        message_lang_ja_jp TEXT NOT NULL,
        message_lang_pt_pt TEXT NOT NULL,
        message_lang_it_it TEXT NOT NULL,
        message_lang_unknown_12 TEXT NOT NULL,
        message_lang_unknown_13 TEXT NOT NULL,
        message_lang_unknown_14 TEXT NOT NULL,
        message_lang_unknown_15 TEXT NOT NULL,
        message_lang_flags TEXT NOT NULL,
        raid_duration INTEGER  NOT NULL,
        max_players INTEGER  NOT NULL,
        difficultystring TEXT  NOT NULL
    );"
    ,
    "INSERT INTO MapDifficulty (
        id,
        map_id,
        difficulty,
        message_lang_en_gb,
        message_lang_ko_kr,
        message_lang_fr_fr,
        message_lang_de_de,
        message_lang_en_cn,
        message_lang_en_tw,
        message_lang_es_es,
        message_lang_es_mx,
        message_lang_ru_ru,
        message_lang_ja_jp,
        message_lang_pt_pt,
        message_lang_it_it,
        message_lang_unknown_12,
        message_lang_unknown_13,
        message_lang_unknown_14,
        message_lang_unknown_15,
        message_lang_flags,
        raid_duration,
        max_players,
        difficultystring
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn QuestXP() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS QuestXP (
        id INTEGER PRIMARY KEY NOT NULL,
        difficulty_0 INTEGER NOT NULL,
        difficulty_1 INTEGER NOT NULL,
        difficulty_2 INTEGER NOT NULL,
        difficulty_3 INTEGER NOT NULL,
        difficulty_4 INTEGER NOT NULL,
        difficulty_5 INTEGER NOT NULL,
        difficulty_6 INTEGER NOT NULL,
        difficulty_7 INTEGER NOT NULL,
        difficulty_8 INTEGER NOT NULL,
        difficulty_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO QuestXP (
        id,
        difficulty_0,
        difficulty_1,
        difficulty_2,
        difficulty_3,
        difficulty_4,
        difficulty_5,
        difficulty_6,
        difficulty_7,
        difficulty_8,
        difficulty_9
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharHairTextures() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharHairTextures (
        id INTEGER PRIMARY KEY NOT NULL,
        field_0_5_3_3368_001_race INTEGER  NOT NULL,
        field_0_5_3_3368_002_gender INTEGER  NOT NULL,
        field_0_5_3_3368_003 INTEGER  NOT NULL,
        field_0_5_3_3368_004_mayberacemask INTEGER  NOT NULL,
        field_0_5_3_3368_005_the_x_in_hair_xy_blp INTEGER  NOT NULL,
        field_0_5_3_3368_006 INTEGER  NOT NULL,
        field_0_5_3_3368_007 INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharHairTextures (
        id,
        field_0_5_3_3368_001_race,
        field_0_5_3_3368_002_gender,
        field_0_5_3_3368_003,
        field_0_5_3_3368_004_mayberacemask,
        field_0_5_3_3368_005_the_x_in_hair_xy_blp,
        field_0_5_3_3368_006,
        field_0_5_3_3368_007
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Achievement_Category() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Achievement_Category (
        id INTEGER PRIMARY KEY NOT NULL,
        parent INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        ui_order INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Achievement_Category (
        id,
        parent,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        ui_order
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DurabilityQuality() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DurabilityQuality (
        id INTEGER PRIMARY KEY NOT NULL,
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO DurabilityQuality (
        id,
        data
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellRange() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellRange (
        id INTEGER PRIMARY KEY NOT NULL,
        range_min_0 REAL NOT NULL,
        range_min_1 REAL NOT NULL,
        range_max_0 REAL NOT NULL,
        range_max_1 REAL NOT NULL,
        flags INTEGER  NOT NULL,
        display_name_lang_en_gb TEXT NOT NULL,
        display_name_lang_ko_kr TEXT NOT NULL,
        display_name_lang_fr_fr TEXT NOT NULL,
        display_name_lang_de_de TEXT NOT NULL,
        display_name_lang_en_cn TEXT NOT NULL,
        display_name_lang_en_tw TEXT NOT NULL,
        display_name_lang_es_es TEXT NOT NULL,
        display_name_lang_es_mx TEXT NOT NULL,
        display_name_lang_ru_ru TEXT NOT NULL,
        display_name_lang_ja_jp TEXT NOT NULL,
        display_name_lang_pt_pt TEXT NOT NULL,
        display_name_lang_it_it TEXT NOT NULL,
        display_name_lang_unknown_12 TEXT NOT NULL,
        display_name_lang_unknown_13 TEXT NOT NULL,
        display_name_lang_unknown_14 TEXT NOT NULL,
        display_name_lang_unknown_15 TEXT NOT NULL,
        display_name_lang_flags TEXT NOT NULL,
        display_name_short_lang_en_gb TEXT NOT NULL,
        display_name_short_lang_ko_kr TEXT NOT NULL,
        display_name_short_lang_fr_fr TEXT NOT NULL,
        display_name_short_lang_de_de TEXT NOT NULL,
        display_name_short_lang_en_cn TEXT NOT NULL,
        display_name_short_lang_en_tw TEXT NOT NULL,
        display_name_short_lang_es_es TEXT NOT NULL,
        display_name_short_lang_es_mx TEXT NOT NULL,
        display_name_short_lang_ru_ru TEXT NOT NULL,
        display_name_short_lang_ja_jp TEXT NOT NULL,
        display_name_short_lang_pt_pt TEXT NOT NULL,
        display_name_short_lang_it_it TEXT NOT NULL,
        display_name_short_lang_unknown_12 TEXT NOT NULL,
        display_name_short_lang_unknown_13 TEXT NOT NULL,
        display_name_short_lang_unknown_14 TEXT NOT NULL,
        display_name_short_lang_unknown_15 TEXT NOT NULL,
        display_name_short_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellRange (
        id,
        range_min_0,
        range_min_1,
        range_max_0,
        range_max_1,
        flags,
        display_name_lang_en_gb,
        display_name_lang_ko_kr,
        display_name_lang_fr_fr,
        display_name_lang_de_de,
        display_name_lang_en_cn,
        display_name_lang_en_tw,
        display_name_lang_es_es,
        display_name_lang_es_mx,
        display_name_lang_ru_ru,
        display_name_lang_ja_jp,
        display_name_lang_pt_pt,
        display_name_lang_it_it,
        display_name_lang_unknown_12,
        display_name_lang_unknown_13,
        display_name_lang_unknown_14,
        display_name_lang_unknown_15,
        display_name_lang_flags,
        display_name_short_lang_en_gb,
        display_name_short_lang_ko_kr,
        display_name_short_lang_fr_fr,
        display_name_short_lang_de_de,
        display_name_short_lang_en_cn,
        display_name_short_lang_en_tw,
        display_name_short_lang_es_es,
        display_name_short_lang_es_mx,
        display_name_short_lang_ru_ru,
        display_name_short_lang_ja_jp,
        display_name_short_lang_pt_pt,
        display_name_short_lang_it_it,
        display_name_short_lang_unknown_12,
        display_name_short_lang_unknown_13,
        display_name_short_lang_unknown_14,
        display_name_short_lang_unknown_15,
        display_name_short_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WMOAreaTable() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WMOAreaTable (
        id INTEGER PRIMARY KEY NOT NULL,
        w_m_o_id INTEGER  NOT NULL,
        name_set_id INTEGER  NOT NULL,
        w_m_o_group_id INTEGER  NOT NULL,
        sound_provider_pref INTEGER  NOT NULL,
        sound_provider_pref_underwater INTEGER  NOT NULL,
        ambience_id INTEGER  NOT NULL,
        zone_music INTEGER  NOT NULL,
        intro_sound INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        area_table_id INTEGER  NOT NULL,
        area_name_lang_en_gb TEXT NOT NULL,
        area_name_lang_ko_kr TEXT NOT NULL,
        area_name_lang_fr_fr TEXT NOT NULL,
        area_name_lang_de_de TEXT NOT NULL,
        area_name_lang_en_cn TEXT NOT NULL,
        area_name_lang_en_tw TEXT NOT NULL,
        area_name_lang_es_es TEXT NOT NULL,
        area_name_lang_es_mx TEXT NOT NULL,
        area_name_lang_ru_ru TEXT NOT NULL,
        area_name_lang_ja_jp TEXT NOT NULL,
        area_name_lang_pt_pt TEXT NOT NULL,
        area_name_lang_it_it TEXT NOT NULL,
        area_name_lang_unknown_12 TEXT NOT NULL,
        area_name_lang_unknown_13 TEXT NOT NULL,
        area_name_lang_unknown_14 TEXT NOT NULL,
        area_name_lang_unknown_15 TEXT NOT NULL,
        area_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO WMOAreaTable (
        id,
        w_m_o_id,
        name_set_id,
        w_m_o_group_id,
        sound_provider_pref,
        sound_provider_pref_underwater,
        ambience_id,
        zone_music,
        intro_sound,
        flags,
        area_table_id,
        area_name_lang_en_gb,
        area_name_lang_ko_kr,
        area_name_lang_fr_fr,
        area_name_lang_de_de,
        area_name_lang_en_cn,
        area_name_lang_en_tw,
        area_name_lang_es_es,
        area_name_lang_es_mx,
        area_name_lang_ru_ru,
        area_name_lang_ja_jp,
        area_name_lang_pt_pt,
        area_name_lang_it_it,
        area_name_lang_unknown_12,
        area_name_lang_unknown_13,
        area_name_lang_unknown_14,
        area_name_lang_unknown_15,
        area_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellDispelType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellDispelType (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        mask INTEGER  NOT NULL,
        immunity_possible INTEGER  NOT NULL,
        internal_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellDispelType (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        mask,
        immunity_possible,
        internal_name
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemBagFamily() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemBagFamily (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemBagFamily (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldStateZoneSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldStateZoneSounds (
        world_state_id INTEGER  NOT NULL,
        world_state_value INTEGER  NOT NULL,
        area_id INTEGER  NOT NULL,
        w_m_o_area_id INTEGER  NOT NULL,
        zone_intro_music_id INTEGER  NOT NULL,
        zone_music_id INTEGER  NOT NULL,
        sound_ambience_id INTEGER  NOT NULL,
        sound_provider_preferences_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WorldStateZoneSounds (
        world_state_id,
        world_state_value,
        area_id,
        w_m_o_area_id,
        zone_intro_music_id,
        zone_music_id,
        sound_ambience_id,
        sound_provider_preferences_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ObjectEffect() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ObjectEffect (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        object_effect_group_id INTEGER  NOT NULL,
        trigger_type INTEGER  NOT NULL,
        event_type INTEGER  NOT NULL,
        effect_rec_type INTEGER  NOT NULL,
        effect_rec_id INTEGER  NOT NULL,
        attachment INTEGER  NOT NULL,
        offset_0 REAL NOT NULL,
        offset_1 REAL NOT NULL,
        offset_2 REAL NOT NULL,
        object_effect_modifier_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ObjectEffect (
        id,
        name,
        object_effect_group_id,
        trigger_type,
        event_type,
        effect_rec_type,
        effect_rec_id,
        attachment,
        offset_0,
        offset_1,
        offset_2,
        object_effect_modifier_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AuctionHouse() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AuctionHouse (
        id INTEGER PRIMARY KEY NOT NULL,
        faction_id INTEGER  NOT NULL,
        deposit_rate INTEGER  NOT NULL,
        consignment_rate INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO AuctionHouse (
        id,
        faction_id,
        deposit_rate,
        consignment_rate,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldChunkSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldChunkSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        chunk_x INTEGER  NOT NULL,
        chunk_y INTEGER  NOT NULL,
        subchunk_x INTEGER  NOT NULL,
        subchunk_y INTEGER  NOT NULL,
        zone_intro_music_id INTEGER  NOT NULL,
        zone_music_id INTEGER  NOT NULL,
        sound_ambience_id INTEGER  NOT NULL,
        sound_provider_preferences_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WorldChunkSounds (
        id,
        chunk_x,
        chunk_y,
        subchunk_x,
        subchunk_y,
        zone_intro_music_id,
        zone_music_id,
        sound_ambience_id,
        sound_provider_preferences_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtOCTClassCombatRatingScalar() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtOCTClassCombatRatingScalar (
        id INTEGER PRIMARY KEY NOT NULL,
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtOCTClassCombatRatingScalar (
        id,
        data
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellVisualEffectName() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualEffectName (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        file_name TEXT  NOT NULL,
        area_effect_size REAL  NOT NULL,
        scale REAL  NOT NULL,
        min_allowed_scale REAL  NOT NULL,
        max_allowed_scale REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualEffectName (
        id,
        name,
        file_name,
        area_effect_size,
        scale,
        min_allowed_scale,
        max_allowed_scale
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellMissileMotion() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellMissileMotion (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        script_body TEXT  NOT NULL,
        flags INTEGER  NOT NULL,
        missile_count INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellMissileMotion (
        id,
        name,
        script_body,
        flags,
        missile_count
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Weather() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Weather (
        id INTEGER PRIMARY KEY NOT NULL,
        ambience_id INTEGER  NOT NULL,
        effect_type INTEGER  NOT NULL,
        transition_sky_box REAL  NOT NULL,
        effect_color_0 REAL NOT NULL,
        effect_color_1 REAL NOT NULL,
        effect_color_2 REAL NOT NULL,
        effect_texture TEXT  NOT NULL
    );"
    ,
    "INSERT INTO Weather (
        id,
        ambience_id,
        effect_type,
        transition_sky_box,
        effect_color_0,
        effect_color_1,
        effect_color_2,
        effect_texture
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GameObjectDisplayInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GameObjectDisplayInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        model_name TEXT  NOT NULL,
        sound_0 INTEGER NOT NULL,
        sound_1 INTEGER NOT NULL,
        sound_2 INTEGER NOT NULL,
        sound_3 INTEGER NOT NULL,
        sound_4 INTEGER NOT NULL,
        sound_5 INTEGER NOT NULL,
        sound_6 INTEGER NOT NULL,
        sound_7 INTEGER NOT NULL,
        sound_8 INTEGER NOT NULL,
        sound_9 INTEGER NOT NULL,
        geo_box_min_0 REAL NOT NULL,
        geo_box_min_1 REAL NOT NULL,
        geo_box_min_2 REAL NOT NULL,
        geo_box_max_0 REAL NOT NULL,
        geo_box_max_1 REAL NOT NULL,
        geo_box_max_2 REAL NOT NULL,
        object_effect_package_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GameObjectDisplayInfo (
        id,
        model_name,
        sound_0,
        sound_1,
        sound_2,
        sound_3,
        sound_4,
        sound_5,
        sound_6,
        sound_7,
        sound_8,
        sound_9,
        geo_box_min_0,
        geo_box_min_1,
        geo_box_min_2,
        geo_box_max_0,
        geo_box_max_1,
        geo_box_max_2,
        object_effect_package_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DungeonEncounter() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DungeonEncounter (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        difficulty INTEGER  NOT NULL,
        order_index INTEGER  NOT NULL,
        bit INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        spell_icon_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO DungeonEncounter (
        id,
        map_id,
        difficulty,
        order_index,
        bit,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        spell_icon_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtOCTRegenMP() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtOCTRegenMP (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtOCTRegenMP (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureMovementInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureMovementInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        smooth_facing_chase_rate REAL  NOT NULL
    );"
    ,
    "INSERT INTO CreatureMovementInfo (
        id,
        smooth_facing_chase_rate
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharTitles() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharTitles (
        id INTEGER PRIMARY KEY NOT NULL,
        condition_id INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        name1_lang_en_gb TEXT NOT NULL,
        name1_lang_ko_kr TEXT NOT NULL,
        name1_lang_fr_fr TEXT NOT NULL,
        name1_lang_de_de TEXT NOT NULL,
        name1_lang_en_cn TEXT NOT NULL,
        name1_lang_en_tw TEXT NOT NULL,
        name1_lang_es_es TEXT NOT NULL,
        name1_lang_es_mx TEXT NOT NULL,
        name1_lang_ru_ru TEXT NOT NULL,
        name1_lang_ja_jp TEXT NOT NULL,
        name1_lang_pt_pt TEXT NOT NULL,
        name1_lang_it_it TEXT NOT NULL,
        name1_lang_unknown_12 TEXT NOT NULL,
        name1_lang_unknown_13 TEXT NOT NULL,
        name1_lang_unknown_14 TEXT NOT NULL,
        name1_lang_unknown_15 TEXT NOT NULL,
        name1_lang_flags TEXT NOT NULL,
        mask_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharTitles (
        id,
        condition_id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        name1_lang_en_gb,
        name1_lang_ko_kr,
        name1_lang_fr_fr,
        name1_lang_de_de,
        name1_lang_en_cn,
        name1_lang_en_tw,
        name1_lang_es_es,
        name1_lang_es_mx,
        name1_lang_ru_ru,
        name1_lang_ja_jp,
        name1_lang_pt_pt,
        name1_lang_it_it,
        name1_lang_unknown_12,
        name1_lang_unknown_13,
        name1_lang_unknown_14,
        name1_lang_unknown_15,
        name1_lang_flags,
        mask_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ObjectEffectPackageElem() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ObjectEffectPackageElem (
        id INTEGER PRIMARY KEY NOT NULL,
        object_effect_package_id INTEGER  NOT NULL,
        object_effect_group_id INTEGER  NOT NULL,
        state_type INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ObjectEffectPackageElem (
        id,
        object_effect_package_id,
        object_effect_group_id,
        state_type
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ChrClasses() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChrClasses (
        id INTEGER PRIMARY KEY NOT NULL,
        damage_bonus_stat INTEGER  NOT NULL,
        display_power INTEGER  NOT NULL,
        pet_name_token TEXT  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        name_female_lang_en_gb TEXT NOT NULL,
        name_female_lang_ko_kr TEXT NOT NULL,
        name_female_lang_fr_fr TEXT NOT NULL,
        name_female_lang_de_de TEXT NOT NULL,
        name_female_lang_en_cn TEXT NOT NULL,
        name_female_lang_en_tw TEXT NOT NULL,
        name_female_lang_es_es TEXT NOT NULL,
        name_female_lang_es_mx TEXT NOT NULL,
        name_female_lang_ru_ru TEXT NOT NULL,
        name_female_lang_ja_jp TEXT NOT NULL,
        name_female_lang_pt_pt TEXT NOT NULL,
        name_female_lang_it_it TEXT NOT NULL,
        name_female_lang_unknown_12 TEXT NOT NULL,
        name_female_lang_unknown_13 TEXT NOT NULL,
        name_female_lang_unknown_14 TEXT NOT NULL,
        name_female_lang_unknown_15 TEXT NOT NULL,
        name_female_lang_flags TEXT NOT NULL,
        name_male_lang_en_gb TEXT NOT NULL,
        name_male_lang_ko_kr TEXT NOT NULL,
        name_male_lang_fr_fr TEXT NOT NULL,
        name_male_lang_de_de TEXT NOT NULL,
        name_male_lang_en_cn TEXT NOT NULL,
        name_male_lang_en_tw TEXT NOT NULL,
        name_male_lang_es_es TEXT NOT NULL,
        name_male_lang_es_mx TEXT NOT NULL,
        name_male_lang_ru_ru TEXT NOT NULL,
        name_male_lang_ja_jp TEXT NOT NULL,
        name_male_lang_pt_pt TEXT NOT NULL,
        name_male_lang_it_it TEXT NOT NULL,
        name_male_lang_unknown_12 TEXT NOT NULL,
        name_male_lang_unknown_13 TEXT NOT NULL,
        name_male_lang_unknown_14 TEXT NOT NULL,
        name_male_lang_unknown_15 TEXT NOT NULL,
        name_male_lang_flags TEXT NOT NULL,
        filename TEXT  NOT NULL,
        spell_class_set INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        cinematic_sequence_id INTEGER  NOT NULL,
        required_expansion INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ChrClasses (
        id,
        damage_bonus_stat,
        display_power,
        pet_name_token,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        name_female_lang_en_gb,
        name_female_lang_ko_kr,
        name_female_lang_fr_fr,
        name_female_lang_de_de,
        name_female_lang_en_cn,
        name_female_lang_en_tw,
        name_female_lang_es_es,
        name_female_lang_es_mx,
        name_female_lang_ru_ru,
        name_female_lang_ja_jp,
        name_female_lang_pt_pt,
        name_female_lang_it_it,
        name_female_lang_unknown_12,
        name_female_lang_unknown_13,
        name_female_lang_unknown_14,
        name_female_lang_unknown_15,
        name_female_lang_flags,
        name_male_lang_en_gb,
        name_male_lang_ko_kr,
        name_male_lang_fr_fr,
        name_male_lang_de_de,
        name_male_lang_en_cn,
        name_male_lang_en_tw,
        name_male_lang_es_es,
        name_male_lang_es_mx,
        name_male_lang_ru_ru,
        name_male_lang_ja_jp,
        name_male_lang_pt_pt,
        name_male_lang_it_it,
        name_male_lang_unknown_12,
        name_male_lang_unknown_13,
        name_male_lang_unknown_14,
        name_male_lang_unknown_15,
        name_male_lang_flags,
        filename,
        spell_class_set,
        flags,
        cinematic_sequence_id,
        required_expansion
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GMSurveyCurrentSurvey() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveyCurrentSurvey (
        id INTEGER PRIMARY KEY NOT NULL,
        gm_survey_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GMSurveyCurrentSurvey (
        id,
        gm_survey_id
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn QuestInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS QuestInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        info_name_lang_en_gb TEXT NOT NULL,
        info_name_lang_ko_kr TEXT NOT NULL,
        info_name_lang_fr_fr TEXT NOT NULL,
        info_name_lang_de_de TEXT NOT NULL,
        info_name_lang_en_cn TEXT NOT NULL,
        info_name_lang_en_tw TEXT NOT NULL,
        info_name_lang_es_es TEXT NOT NULL,
        info_name_lang_es_mx TEXT NOT NULL,
        info_name_lang_ru_ru TEXT NOT NULL,
        info_name_lang_ja_jp TEXT NOT NULL,
        info_name_lang_pt_pt TEXT NOT NULL,
        info_name_lang_it_it TEXT NOT NULL,
        info_name_lang_unknown_12 TEXT NOT NULL,
        info_name_lang_unknown_13 TEXT NOT NULL,
        info_name_lang_unknown_14 TEXT NOT NULL,
        info_name_lang_unknown_15 TEXT NOT NULL,
        info_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO QuestInfo (
        id,
        info_name_lang_en_gb,
        info_name_lang_ko_kr,
        info_name_lang_fr_fr,
        info_name_lang_de_de,
        info_name_lang_en_cn,
        info_name_lang_en_tw,
        info_name_lang_es_es,
        info_name_lang_es_mx,
        info_name_lang_ru_ru,
        info_name_lang_ja_jp,
        info_name_lang_pt_pt,
        info_name_lang_it_it,
        info_name_lang_unknown_12,
        info_name_lang_unknown_13,
        info_name_lang_unknown_14,
        info_name_lang_unknown_15,
        info_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillLine() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillLine (
        id INTEGER PRIMARY KEY NOT NULL,
        category_id INTEGER  NOT NULL,
        skill_costs_id INTEGER  NOT NULL,
        display_name_lang_en_gb TEXT NOT NULL,
        display_name_lang_ko_kr TEXT NOT NULL,
        display_name_lang_fr_fr TEXT NOT NULL,
        display_name_lang_de_de TEXT NOT NULL,
        display_name_lang_en_cn TEXT NOT NULL,
        display_name_lang_en_tw TEXT NOT NULL,
        display_name_lang_es_es TEXT NOT NULL,
        display_name_lang_es_mx TEXT NOT NULL,
        display_name_lang_ru_ru TEXT NOT NULL,
        display_name_lang_ja_jp TEXT NOT NULL,
        display_name_lang_pt_pt TEXT NOT NULL,
        display_name_lang_it_it TEXT NOT NULL,
        display_name_lang_unknown_12 TEXT NOT NULL,
        display_name_lang_unknown_13 TEXT NOT NULL,
        display_name_lang_unknown_14 TEXT NOT NULL,
        display_name_lang_unknown_15 TEXT NOT NULL,
        display_name_lang_flags TEXT NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL,
        spell_icon_id INTEGER  NOT NULL,
        alternate_verb_lang_en_gb TEXT NOT NULL,
        alternate_verb_lang_ko_kr TEXT NOT NULL,
        alternate_verb_lang_fr_fr TEXT NOT NULL,
        alternate_verb_lang_de_de TEXT NOT NULL,
        alternate_verb_lang_en_cn TEXT NOT NULL,
        alternate_verb_lang_en_tw TEXT NOT NULL,
        alternate_verb_lang_es_es TEXT NOT NULL,
        alternate_verb_lang_es_mx TEXT NOT NULL,
        alternate_verb_lang_ru_ru TEXT NOT NULL,
        alternate_verb_lang_ja_jp TEXT NOT NULL,
        alternate_verb_lang_pt_pt TEXT NOT NULL,
        alternate_verb_lang_it_it TEXT NOT NULL,
        alternate_verb_lang_unknown_12 TEXT NOT NULL,
        alternate_verb_lang_unknown_13 TEXT NOT NULL,
        alternate_verb_lang_unknown_14 TEXT NOT NULL,
        alternate_verb_lang_unknown_15 TEXT NOT NULL,
        alternate_verb_lang_flags TEXT NOT NULL,
        can_link INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SkillLine (
        id,
        category_id,
        skill_costs_id,
        display_name_lang_en_gb,
        display_name_lang_ko_kr,
        display_name_lang_fr_fr,
        display_name_lang_de_de,
        display_name_lang_en_cn,
        display_name_lang_en_tw,
        display_name_lang_es_es,
        display_name_lang_es_mx,
        display_name_lang_ru_ru,
        display_name_lang_ja_jp,
        display_name_lang_pt_pt,
        display_name_lang_it_it,
        display_name_lang_unknown_12,
        display_name_lang_unknown_13,
        display_name_lang_unknown_14,
        display_name_lang_unknown_15,
        display_name_lang_flags,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags,
        spell_icon_id,
        alternate_verb_lang_en_gb,
        alternate_verb_lang_ko_kr,
        alternate_verb_lang_fr_fr,
        alternate_verb_lang_de_de,
        alternate_verb_lang_en_cn,
        alternate_verb_lang_en_tw,
        alternate_verb_lang_es_es,
        alternate_verb_lang_es_mx,
        alternate_verb_lang_ru_ru,
        alternate_verb_lang_ja_jp,
        alternate_verb_lang_pt_pt,
        alternate_verb_lang_it_it,
        alternate_verb_lang_unknown_12,
        alternate_verb_lang_unknown_13,
        alternate_verb_lang_unknown_14,
        alternate_verb_lang_unknown_15,
        alternate_verb_lang_flags,
        can_link
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GroundEffectDoodad() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GroundEffectDoodad (
        id INTEGER PRIMARY KEY NOT NULL,
        doodadpath TEXT  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GroundEffectDoodad (
        id,
        doodadpath,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemSet() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemSet (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        item_id_0 INTEGER NOT NULL,
        item_id_1 INTEGER NOT NULL,
        item_id_2 INTEGER NOT NULL,
        item_id_3 INTEGER NOT NULL,
        item_id_4 INTEGER NOT NULL,
        item_id_5 INTEGER NOT NULL,
        item_id_6 INTEGER NOT NULL,
        item_id_7 INTEGER NOT NULL,
        item_id_8 INTEGER NOT NULL,
        item_id_9 INTEGER NOT NULL,
        item_id_10 INTEGER NOT NULL,
        item_id_11 INTEGER NOT NULL,
        item_id_12 INTEGER NOT NULL,
        item_id_13 INTEGER NOT NULL,
        item_id_14 INTEGER NOT NULL,
        item_id_15 INTEGER NOT NULL,
        item_id_16 INTEGER NOT NULL,
        set_spell_id_0 INTEGER NOT NULL,
        set_spell_id_1 INTEGER NOT NULL,
        set_spell_id_2 INTEGER NOT NULL,
        set_spell_id_3 INTEGER NOT NULL,
        set_spell_id_4 INTEGER NOT NULL,
        set_spell_id_5 INTEGER NOT NULL,
        set_spell_id_6 INTEGER NOT NULL,
        set_spell_id_7 INTEGER NOT NULL,
        set_threshold_0 INTEGER NOT NULL,
        set_threshold_1 INTEGER NOT NULL,
        set_threshold_2 INTEGER NOT NULL,
        set_threshold_3 INTEGER NOT NULL,
        set_threshold_4 INTEGER NOT NULL,
        set_threshold_5 INTEGER NOT NULL,
        set_threshold_6 INTEGER NOT NULL,
        set_threshold_7 INTEGER NOT NULL,
        required_skill INTEGER  NOT NULL,
        required_skill_rank INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ItemSet (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        item_id_0,
        item_id_1,
        item_id_2,
        item_id_3,
        item_id_4,
        item_id_5,
        item_id_6,
        item_id_7,
        item_id_8,
        item_id_9,
        item_id_10,
        item_id_11,
        item_id_12,
        item_id_13,
        item_id_14,
        item_id_15,
        item_id_16,
        set_spell_id_0,
        set_spell_id_1,
        set_spell_id_2,
        set_spell_id_3,
        set_spell_id_4,
        set_spell_id_5,
        set_spell_id_6,
        set_spell_id_7,
        set_threshold_0,
        set_threshold_1,
        set_threshold_2,
        set_threshold_3,
        set_threshold_4,
        set_threshold_5,
        set_threshold_6,
        set_threshold_7,
        required_skill,
        required_skill_rank
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtNPCManaCostScaler() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtNPCManaCostScaler (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtNPCManaCostScaler (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DanceMoves() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DanceMoves (
        id INTEGER PRIMARY KEY NOT NULL,
        ty INTEGER  NOT NULL,
        param INTEGER  NOT NULL,
        fallback INTEGER  NOT NULL,
        racemask INTEGER  NOT NULL,
        internal_name TEXT  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        lock_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO DanceMoves (
        id,
        ty,
        param,
        fallback,
        racemask,
        internal_name,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        lock_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellMechanic() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellMechanic (
        id INTEGER PRIMARY KEY NOT NULL,
        state_name_lang_en_gb TEXT NOT NULL,
        state_name_lang_ko_kr TEXT NOT NULL,
        state_name_lang_fr_fr TEXT NOT NULL,
        state_name_lang_de_de TEXT NOT NULL,
        state_name_lang_en_cn TEXT NOT NULL,
        state_name_lang_en_tw TEXT NOT NULL,
        state_name_lang_es_es TEXT NOT NULL,
        state_name_lang_es_mx TEXT NOT NULL,
        state_name_lang_ru_ru TEXT NOT NULL,
        state_name_lang_ja_jp TEXT NOT NULL,
        state_name_lang_pt_pt TEXT NOT NULL,
        state_name_lang_it_it TEXT NOT NULL,
        state_name_lang_unknown_12 TEXT NOT NULL,
        state_name_lang_unknown_13 TEXT NOT NULL,
        state_name_lang_unknown_14 TEXT NOT NULL,
        state_name_lang_unknown_15 TEXT NOT NULL,
        state_name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellMechanic (
        id,
        state_name_lang_en_gb,
        state_name_lang_ko_kr,
        state_name_lang_fr_fr,
        state_name_lang_de_de,
        state_name_lang_en_cn,
        state_name_lang_en_tw,
        state_name_lang_es_es,
        state_name_lang_es_mx,
        state_name_lang_ru_ru,
        state_name_lang_ja_jp,
        state_name_lang_pt_pt,
        state_name_lang_it_it,
        state_name_lang_unknown_12,
        state_name_lang_unknown_13,
        state_name_lang_unknown_14,
        state_name_lang_unknown_15,
        state_name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn BattlemasterList() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS BattlemasterList (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id_0 INTEGER NOT NULL,
        map_id_1 INTEGER NOT NULL,
        map_id_2 INTEGER NOT NULL,
        map_id_3 INTEGER NOT NULL,
        map_id_4 INTEGER NOT NULL,
        map_id_5 INTEGER NOT NULL,
        map_id_6 INTEGER NOT NULL,
        map_id_7 INTEGER NOT NULL,
        instance_type INTEGER  NOT NULL,
        groups_allowed INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        max_group_size INTEGER  NOT NULL,
        holiday_world_state INTEGER  NOT NULL,
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO BattlemasterList (
        id,
        map_id_0,
        map_id_1,
        map_id_2,
        map_id_3,
        map_id_4,
        map_id_5,
        map_id_6,
        map_id_7,
        instance_type,
        groups_allowed,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        max_group_size,
        holiday_world_state,
        min_level,
        max_level
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldMapOverlay() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapOverlay (
        id INTEGER PRIMARY KEY NOT NULL,
        map_area_id INTEGER  NOT NULL,
        area_id_0 INTEGER NOT NULL,
        area_id_1 INTEGER NOT NULL,
        area_id_2 INTEGER NOT NULL,
        area_id_3 INTEGER NOT NULL,
        map_point_x INTEGER  NOT NULL,
        map_point_y INTEGER  NOT NULL,
        texture_name TEXT  NOT NULL,
        texture_width INTEGER  NOT NULL,
        texture_height INTEGER  NOT NULL,
        offset_x INTEGER  NOT NULL,
        offset_y INTEGER  NOT NULL,
        hit_rect_top INTEGER  NOT NULL,
        hit_rect_left INTEGER  NOT NULL,
        hit_rect_bottom INTEGER  NOT NULL,
        hit_rect_right INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WorldMapOverlay (
        id,
        map_area_id,
        area_id_0,
        area_id_1,
        area_id_2,
        area_id_3,
        map_point_x,
        map_point_y,
        texture_name,
        texture_width,
        texture_height,
        offset_x,
        offset_y,
        hit_rect_top,
        hit_rect_left,
        hit_rect_bottom,
        hit_rect_right
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GlyphSlot() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GlyphSlot (
        id INTEGER PRIMARY KEY NOT NULL,
        ty INTEGER  NOT NULL,
        tooltip INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GlyphSlot (
        id,
        ty,
        tooltip
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GameTips() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GameTips (
        id INTEGER PRIMARY KEY NOT NULL,
        text_lang_en_gb TEXT NOT NULL,
        text_lang_ko_kr TEXT NOT NULL,
        text_lang_fr_fr TEXT NOT NULL,
        text_lang_de_de TEXT NOT NULL,
        text_lang_en_cn TEXT NOT NULL,
        text_lang_en_tw TEXT NOT NULL,
        text_lang_es_es TEXT NOT NULL,
        text_lang_es_mx TEXT NOT NULL,
        text_lang_ru_ru TEXT NOT NULL,
        text_lang_ja_jp TEXT NOT NULL,
        text_lang_pt_pt TEXT NOT NULL,
        text_lang_it_it TEXT NOT NULL,
        text_lang_unknown_12 TEXT NOT NULL,
        text_lang_unknown_13 TEXT NOT NULL,
        text_lang_unknown_14 TEXT NOT NULL,
        text_lang_unknown_15 TEXT NOT NULL,
        text_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GameTips (
        id,
        text_lang_en_gb,
        text_lang_ko_kr,
        text_lang_fr_fr,
        text_lang_de_de,
        text_lang_en_cn,
        text_lang_en_tw,
        text_lang_es_es,
        text_lang_es_mx,
        text_lang_ru_ru,
        text_lang_ja_jp,
        text_lang_pt_pt,
        text_lang_it_it,
        text_lang_unknown_12,
        text_lang_unknown_13,
        text_lang_unknown_14,
        text_lang_unknown_15,
        text_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TaxiPathNode() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TaxiPathNode (
        id INTEGER PRIMARY KEY NOT NULL,
        path_id INTEGER  NOT NULL,
        node_index INTEGER  NOT NULL,
        continent_id INTEGER  NOT NULL,
        loc_0 REAL NOT NULL,
        loc_1 REAL NOT NULL,
        loc_2 REAL NOT NULL,
        flags INTEGER  NOT NULL,
        delay INTEGER  NOT NULL,
        arrival_event_id INTEGER  NOT NULL,
        departure_event_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TaxiPathNode (
        id,
        path_id,
        node_index,
        continent_id,
        loc_0,
        loc_1,
        loc_2,
        flags,
        delay,
        arrival_event_id,
        departure_event_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtRegenHPPerSpt() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtRegenHPPerSpt (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtRegenHPPerSpt (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemPetFood() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemPetFood (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemPetFood (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ObjectEffectGroup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ObjectEffectGroup (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO ObjectEffectGroup (
        id,
        name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TerrainType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TerrainType (
        terrain_id INTEGER  NOT NULL,
        terrain_desc TEXT  NOT NULL,
        footstep_spray_run INTEGER  NOT NULL,
        footstep_spray_walk INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TerrainType (
        terrain_id,
        terrain_desc,
        footstep_spray_run,
        footstep_spray_walk,
        sound_id,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GMTicketCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMTicketCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        category_lang_en_gb TEXT NOT NULL,
        category_lang_ko_kr TEXT NOT NULL,
        category_lang_fr_fr TEXT NOT NULL,
        category_lang_de_de TEXT NOT NULL,
        category_lang_en_cn TEXT NOT NULL,
        category_lang_en_tw TEXT NOT NULL,
        category_lang_es_es TEXT NOT NULL,
        category_lang_es_mx TEXT NOT NULL,
        category_lang_ru_ru TEXT NOT NULL,
        category_lang_ja_jp TEXT NOT NULL,
        category_lang_pt_pt TEXT NOT NULL,
        category_lang_it_it TEXT NOT NULL,
        category_lang_unknown_12 TEXT NOT NULL,
        category_lang_unknown_13 TEXT NOT NULL,
        category_lang_unknown_14 TEXT NOT NULL,
        category_lang_unknown_15 TEXT NOT NULL,
        category_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GMTicketCategory (
        id,
        category_lang_en_gb,
        category_lang_ko_kr,
        category_lang_fr_fr,
        category_lang_de_de,
        category_lang_en_cn,
        category_lang_en_tw,
        category_lang_es_es,
        category_lang_es_mx,
        category_lang_ru_ru,
        category_lang_ja_jp,
        category_lang_pt_pt,
        category_lang_it_it,
        category_lang_unknown_12,
        category_lang_unknown_13,
        category_lang_unknown_14,
        category_lang_unknown_15,
        category_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn QuestFactionReward() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS QuestFactionReward (
        id INTEGER PRIMARY KEY NOT NULL,
        difficulty_0 INTEGER NOT NULL,
        difficulty_1 INTEGER NOT NULL,
        difficulty_2 INTEGER NOT NULL,
        difficulty_3 INTEGER NOT NULL,
        difficulty_4 INTEGER NOT NULL,
        difficulty_5 INTEGER NOT NULL,
        difficulty_6 INTEGER NOT NULL,
        difficulty_7 INTEGER NOT NULL,
        difficulty_8 INTEGER NOT NULL,
        difficulty_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO QuestFactionReward (
        id,
        difficulty_0,
        difficulty_1,
        difficulty_2,
        difficulty_3,
        difficulty_4,
        difficulty_5,
        difficulty_6,
        difficulty_7,
        difficulty_8,
        difficulty_9
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TransportAnimation() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TransportAnimation (
        id INTEGER PRIMARY KEY NOT NULL,
        transport_id INTEGER  NOT NULL,
        time_index INTEGER  NOT NULL,
        pos_0 REAL NOT NULL,
        pos_1 REAL NOT NULL,
        pos_2 REAL NOT NULL,
        sequence_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TransportAnimation (
        id,
        transport_id,
        time_index,
        pos_0,
        pos_1,
        pos_2,
        sequence_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DeathThudLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DeathThudLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        size_class INTEGER  NOT NULL,
        terrain_type_sound_id INTEGER  NOT NULL,
        sound_entry_id INTEGER  NOT NULL,
        sound_entry_id_water INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO DeathThudLookups (
        id,
        size_class,
        terrain_type_sound_id,
        sound_entry_id,
        sound_entry_id_water
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn StableSlotPrices() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS StableSlotPrices (
        id INTEGER PRIMARY KEY NOT NULL,
        cost INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO StableSlotPrices (
        id,
        cost
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SummonProperties() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SummonProperties (
        id INTEGER PRIMARY KEY NOT NULL,
        control INTEGER  NOT NULL,
        faction INTEGER  NOT NULL,
        title INTEGER  NOT NULL,
        slot INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SummonProperties (
        id,
        control,
        faction,
        title,
        slot,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharBaseInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharBaseInfo (
        race_id INTEGER  NOT NULL,
        class_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharBaseInfo (
        race_id,
        class_id
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DungeonMap() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DungeonMap (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        floor_index INTEGER  NOT NULL,
        min_x REAL  NOT NULL,
        max_x REAL  NOT NULL,
        min_y REAL  NOT NULL,
        max_y REAL  NOT NULL,
        parent_world_map_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO DungeonMap (
        id,
        map_id,
        floor_index,
        min_x,
        max_x,
        min_y,
        max_y,
        parent_world_map_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DeclinedWordCases() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DeclinedWordCases (
        id INTEGER PRIMARY KEY NOT NULL,
        declined_word_id INTEGER  NOT NULL,
        case_index INTEGER  NOT NULL,
        declined_word TEXT  NOT NULL
    );"
    ,
    "INSERT INTO DeclinedWordCases (
        id,
        declined_word_id,
        case_index,
        declined_word
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Achievement_Criteria() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Achievement_Criteria (
        id INTEGER PRIMARY KEY NOT NULL,
        achievement_id INTEGER  NOT NULL,
        ty INTEGER  NOT NULL,
        asset_id INTEGER  NOT NULL,
        quantity INTEGER  NOT NULL,
        start_event INTEGER  NOT NULL,
        start_asset INTEGER  NOT NULL,
        fail_event INTEGER  NOT NULL,
        fail_asset INTEGER  NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL,
        flags INTEGER  NOT NULL,
        timer_start_event INTEGER  NOT NULL,
        timer_asset_id INTEGER  NOT NULL,
        timer_time INTEGER  NOT NULL,
        ui_order INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Achievement_Criteria (
        id,
        achievement_id,
        ty,
        asset_id,
        quantity,
        start_event,
        start_asset,
        fail_event,
        fail_asset,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags,
        flags,
        timer_start_event,
        timer_asset_id,
        timer_time,
        ui_order
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn HelmetGeosetVisData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS HelmetGeosetVisData (
        id INTEGER PRIMARY KEY NOT NULL,
        hide_geoset_0 INTEGER NOT NULL,
        hide_geoset_1 INTEGER NOT NULL,
        hide_geoset_2 INTEGER NOT NULL,
        hide_geoset_3 INTEGER NOT NULL,
        hide_geoset_4 INTEGER NOT NULL,
        hide_geoset_5 INTEGER NOT NULL,
        hide_geoset_6 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO HelmetGeosetVisData (
        id,
        hide_geoset_0,
        hide_geoset_1,
        hide_geoset_2,
        hide_geoset_3,
        hide_geoset_4,
        hide_geoset_5,
        hide_geoset_6
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillLineAbility() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillLineAbility (
        id INTEGER PRIMARY KEY NOT NULL,
        skill_line INTEGER  NOT NULL,
        spell INTEGER  NOT NULL,
        race_mask INTEGER  NOT NULL,
        class_mask INTEGER  NOT NULL,
        exclude_race INTEGER  NOT NULL,
        exclude_class INTEGER  NOT NULL,
        min_skill_line_rank INTEGER  NOT NULL,
        superceded_by_spell INTEGER  NOT NULL,
        acquire_method INTEGER  NOT NULL,
        trivial_skill_line_rank_high INTEGER  NOT NULL,
        trivial_skill_line_rank_low INTEGER  NOT NULL,
        character_points_0 INTEGER NOT NULL,
        character_points_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SkillLineAbility (
        id,
        skill_line,
        spell,
        race_mask,
        class_mask,
        exclude_race,
        exclude_class,
        min_skill_line_rank,
        superceded_by_spell,
        acquire_method,
        trivial_skill_line_rank_high,
        trivial_skill_line_rank_low,
        character_points_0,
        character_points_1
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellFocusObject() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellFocusObject (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellFocusObject (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundSamplePreferences() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundSamplePreferences (
        id INTEGER PRIMARY KEY NOT NULL,
        field_0_6_0_3592_001 INTEGER  NOT NULL,
        field_0_6_0_3592_002 INTEGER  NOT NULL,
        e_a_x2_sample_room INTEGER  NOT NULL,
        field_0_6_0_3592_004 INTEGER  NOT NULL,
        field_0_6_0_3592_005 INTEGER  NOT NULL,
        field_0_6_0_3592_006 REAL  NOT NULL,
        field_0_6_0_3592_007 INTEGER  NOT NULL,
        e_a_x2_sample_occlusion_l_f_ratio REAL  NOT NULL,
        e_a_x2_sample_occlusion_room_ratio REAL  NOT NULL,
        field_0_6_0_3592_010 INTEGER  NOT NULL,
        e_a_x1_effect_level REAL  NOT NULL,
        field_0_6_0_3592_012 INTEGER  NOT NULL,
        field_0_6_0_3592_013 REAL  NOT NULL,
        e_a_x3_sample_exclusion REAL  NOT NULL,
        field_0_6_0_3592_015 REAL  NOT NULL,
        field_0_6_0_3592_016 INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundSamplePreferences (
        id,
        field_0_6_0_3592_001,
        field_0_6_0_3592_002,
        e_a_x2_sample_room,
        field_0_6_0_3592_004,
        field_0_6_0_3592_005,
        field_0_6_0_3592_006,
        field_0_6_0_3592_007,
        e_a_x2_sample_occlusion_l_f_ratio,
        e_a_x2_sample_occlusion_room_ratio,
        field_0_6_0_3592_010,
        e_a_x1_effect_level,
        field_0_6_0_3592_012,
        field_0_6_0_3592_013,
        e_a_x3_sample_exclusion,
        field_0_6_0_3592_015,
        field_0_6_0_3592_016
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn HolidayDescriptions() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS HolidayDescriptions (
        id INTEGER PRIMARY KEY NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO HolidayDescriptions (
        id,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharStartOutfit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharStartOutfit (
        id INTEGER PRIMARY KEY NOT NULL,
        race_id INTEGER  NOT NULL,
        class_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        outfit_id INTEGER  NOT NULL,
        item_id_0 INTEGER NOT NULL,
        item_id_1 INTEGER NOT NULL,
        item_id_2 INTEGER NOT NULL,
        item_id_3 INTEGER NOT NULL,
        item_id_4 INTEGER NOT NULL,
        item_id_5 INTEGER NOT NULL,
        item_id_6 INTEGER NOT NULL,
        item_id_7 INTEGER NOT NULL,
        item_id_8 INTEGER NOT NULL,
        item_id_9 INTEGER NOT NULL,
        item_id_10 INTEGER NOT NULL,
        item_id_11 INTEGER NOT NULL,
        item_id_12 INTEGER NOT NULL,
        item_id_13 INTEGER NOT NULL,
        item_id_14 INTEGER NOT NULL,
        item_id_15 INTEGER NOT NULL,
        item_id_16 INTEGER NOT NULL,
        item_id_17 INTEGER NOT NULL,
        item_id_18 INTEGER NOT NULL,
        item_id_19 INTEGER NOT NULL,
        item_id_20 INTEGER NOT NULL,
        item_id_21 INTEGER NOT NULL,
        item_id_22 INTEGER NOT NULL,
        item_id_23 INTEGER NOT NULL,
        display_item_id_0 INTEGER NOT NULL,
        display_item_id_1 INTEGER NOT NULL,
        display_item_id_2 INTEGER NOT NULL,
        display_item_id_3 INTEGER NOT NULL,
        display_item_id_4 INTEGER NOT NULL,
        display_item_id_5 INTEGER NOT NULL,
        display_item_id_6 INTEGER NOT NULL,
        display_item_id_7 INTEGER NOT NULL,
        display_item_id_8 INTEGER NOT NULL,
        display_item_id_9 INTEGER NOT NULL,
        display_item_id_10 INTEGER NOT NULL,
        display_item_id_11 INTEGER NOT NULL,
        display_item_id_12 INTEGER NOT NULL,
        display_item_id_13 INTEGER NOT NULL,
        display_item_id_14 INTEGER NOT NULL,
        display_item_id_15 INTEGER NOT NULL,
        display_item_id_16 INTEGER NOT NULL,
        display_item_id_17 INTEGER NOT NULL,
        display_item_id_18 INTEGER NOT NULL,
        display_item_id_19 INTEGER NOT NULL,
        display_item_id_20 INTEGER NOT NULL,
        display_item_id_21 INTEGER NOT NULL,
        display_item_id_22 INTEGER NOT NULL,
        display_item_id_23 INTEGER NOT NULL,
        inventory_type_0 INTEGER NOT NULL,
        inventory_type_1 INTEGER NOT NULL,
        inventory_type_2 INTEGER NOT NULL,
        inventory_type_3 INTEGER NOT NULL,
        inventory_type_4 INTEGER NOT NULL,
        inventory_type_5 INTEGER NOT NULL,
        inventory_type_6 INTEGER NOT NULL,
        inventory_type_7 INTEGER NOT NULL,
        inventory_type_8 INTEGER NOT NULL,
        inventory_type_9 INTEGER NOT NULL,
        inventory_type_10 INTEGER NOT NULL,
        inventory_type_11 INTEGER NOT NULL,
        inventory_type_12 INTEGER NOT NULL,
        inventory_type_13 INTEGER NOT NULL,
        inventory_type_14 INTEGER NOT NULL,
        inventory_type_15 INTEGER NOT NULL,
        inventory_type_16 INTEGER NOT NULL,
        inventory_type_17 INTEGER NOT NULL,
        inventory_type_18 INTEGER NOT NULL,
        inventory_type_19 INTEGER NOT NULL,
        inventory_type_20 INTEGER NOT NULL,
        inventory_type_21 INTEGER NOT NULL,
        inventory_type_22 INTEGER NOT NULL,
        inventory_type_23 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CharStartOutfit (
        id,
        race_id,
        class_id,
        sex_id,
        outfit_id,
        item_id_0,
        item_id_1,
        item_id_2,
        item_id_3,
        item_id_4,
        item_id_5,
        item_id_6,
        item_id_7,
        item_id_8,
        item_id_9,
        item_id_10,
        item_id_11,
        item_id_12,
        item_id_13,
        item_id_14,
        item_id_15,
        item_id_16,
        item_id_17,
        item_id_18,
        item_id_19,
        item_id_20,
        item_id_21,
        item_id_22,
        item_id_23,
        display_item_id_0,
        display_item_id_1,
        display_item_id_2,
        display_item_id_3,
        display_item_id_4,
        display_item_id_5,
        display_item_id_6,
        display_item_id_7,
        display_item_id_8,
        display_item_id_9,
        display_item_id_10,
        display_item_id_11,
        display_item_id_12,
        display_item_id_13,
        display_item_id_14,
        display_item_id_15,
        display_item_id_16,
        display_item_id_17,
        display_item_id_18,
        display_item_id_19,
        display_item_id_20,
        display_item_id_21,
        display_item_id_22,
        display_item_id_23,
        inventory_type_0,
        inventory_type_1,
        inventory_type_2,
        inventory_type_3,
        inventory_type_4,
        inventory_type_5,
        inventory_type_6,
        inventory_type_7,
        inventory_type_8,
        inventory_type_9,
        inventory_type_10,
        inventory_type_11,
        inventory_type_12,
        inventory_type_13,
        inventory_type_14,
        inventory_type_15,
        inventory_type_16,
        inventory_type_17,
        inventory_type_18,
        inventory_type_19,
        inventory_type_20,
        inventory_type_21,
        inventory_type_22,
        inventory_type_23
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60,
        ?61,
        ?62,
        ?63,
        ?64,
        ?65,
        ?66,
        ?67,
        ?68,
        ?69,
        ?70,
        ?71,
        ?72,
        ?73,
        ?74,
        ?75,
        ?76,
        ?77
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellVisualKit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualKit (
        id INTEGER PRIMARY KEY NOT NULL,
        start_anim_id INTEGER  NOT NULL,
        anim_id INTEGER  NOT NULL,
        head_effect INTEGER  NOT NULL,
        chest_effect INTEGER  NOT NULL,
        base_effect INTEGER  NOT NULL,
        left_hand_effect INTEGER  NOT NULL,
        right_hand_effect INTEGER  NOT NULL,
        breath_effect INTEGER  NOT NULL,
        left_weapon_effect INTEGER  NOT NULL,
        right_weapon_effect INTEGER  NOT NULL,
        special_effect_0 INTEGER NOT NULL,
        special_effect_1 INTEGER NOT NULL,
        special_effect_2 INTEGER NOT NULL,
        world_effect INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL,
        shake_id INTEGER  NOT NULL,
        char_proc_0 INTEGER NOT NULL,
        char_proc_1 INTEGER NOT NULL,
        char_proc_2 INTEGER NOT NULL,
        char_proc_3 INTEGER NOT NULL,
        char_param_zero_0 REAL NOT NULL,
        char_param_zero_1 REAL NOT NULL,
        char_param_zero_2 REAL NOT NULL,
        char_param_zero_3 REAL NOT NULL,
        char_param_one_0 REAL NOT NULL,
        char_param_one_1 REAL NOT NULL,
        char_param_one_2 REAL NOT NULL,
        char_param_one_3 REAL NOT NULL,
        char_param_two_0 REAL NOT NULL,
        char_param_two_1 REAL NOT NULL,
        char_param_two_2 REAL NOT NULL,
        char_param_two_3 REAL NOT NULL,
        char_param_three_0 REAL NOT NULL,
        char_param_three_1 REAL NOT NULL,
        char_param_three_2 REAL NOT NULL,
        char_param_three_3 REAL NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualKit (
        id,
        start_anim_id,
        anim_id,
        head_effect,
        chest_effect,
        base_effect,
        left_hand_effect,
        right_hand_effect,
        breath_effect,
        left_weapon_effect,
        right_weapon_effect,
        special_effect_0,
        special_effect_1,
        special_effect_2,
        world_effect,
        sound_id,
        shake_id,
        char_proc_0,
        char_proc_1,
        char_proc_2,
        char_proc_3,
        char_param_zero_0,
        char_param_zero_1,
        char_param_zero_2,
        char_param_zero_3,
        char_param_one_0,
        char_param_one_1,
        char_param_one_2,
        char_param_one_3,
        char_param_two_0,
        char_param_two_1,
        char_param_two_2,
        char_param_two_3,
        char_param_three_0,
        char_param_three_1,
        char_param_three_2,
        char_param_three_3,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtChanceToMeleeCrit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtChanceToMeleeCrit (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtChanceToMeleeCrit (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GMSurveySurveys() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveySurveys (
        id INTEGER PRIMARY KEY NOT NULL,
        q_0 INTEGER NOT NULL,
        q_1 INTEGER NOT NULL,
        q_2 INTEGER NOT NULL,
        q_3 INTEGER NOT NULL,
        q_4 INTEGER NOT NULL,
        q_5 INTEGER NOT NULL,
        q_6 INTEGER NOT NULL,
        q_7 INTEGER NOT NULL,
        q_8 INTEGER NOT NULL,
        q_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO GMSurveySurveys (
        id,
        q_0,
        q_1,
        q_2,
        q_3,
        q_4,
        q_5,
        q_6,
        q_7,
        q_8,
        q_9
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharHairGeosets() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharHairGeosets (
        id INTEGER PRIMARY KEY NOT NULL,
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        variation_id INTEGER  NOT NULL,
        geoset_id INTEGER  NOT NULL,
        showscalp INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharHairGeosets (
        id,
        race_id,
        sex_id,
        variation_id,
        geoset_id,
        showscalp
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn PetitionType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PetitionType (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        ty INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO PetitionType (
        id,
        name,
        ty
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn UnitBlood() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS UnitBlood (
        id INTEGER PRIMARY KEY NOT NULL,
        combat_blood_spurt_front_0 INTEGER NOT NULL,
        combat_blood_spurt_front_1 INTEGER NOT NULL,
        combat_blood_spurt_back_0 INTEGER NOT NULL,
        combat_blood_spurt_back_1 INTEGER NOT NULL,
        ground_blood_0 TEXT NOT NULL,
        ground_blood_1 TEXT NOT NULL,
        ground_blood_2 TEXT NOT NULL,
        ground_blood_3 TEXT NOT NULL,
        ground_blood_4 TEXT NOT NULL
    );"
    ,
    "INSERT INTO UnitBlood (
        id,
        combat_blood_spurt_front_0,
        combat_blood_spurt_front_1,
        combat_blood_spurt_back_0,
        combat_blood_spurt_back_1,
        ground_blood_0,
        ground_blood_1,
        ground_blood_2,
        ground_blood_3,
        ground_blood_4
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Vehicle() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Vehicle (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        turn_speed REAL  NOT NULL,
        pitch_speed REAL  NOT NULL,
        pitch_min REAL  NOT NULL,
        pitch_max REAL  NOT NULL,
        seat_id_0 INTEGER NOT NULL,
        seat_id_1 INTEGER NOT NULL,
        seat_id_2 INTEGER NOT NULL,
        seat_id_3 INTEGER NOT NULL,
        seat_id_4 INTEGER NOT NULL,
        seat_id_5 INTEGER NOT NULL,
        seat_id_6 INTEGER NOT NULL,
        seat_id_7 INTEGER NOT NULL,
        mouse_look_offset_pitch REAL  NOT NULL,
        camera_fade_dist_scalar_min REAL  NOT NULL,
        camera_fade_dist_scalar_max REAL  NOT NULL,
        camera_pitch_offset REAL  NOT NULL,
        facing_limit_right REAL  NOT NULL,
        facing_limit_left REAL  NOT NULL,
        mssl_trgt_turn_lingering REAL  NOT NULL,
        mssl_trgt_pitch_lingering REAL  NOT NULL,
        mssl_trgt_mouse_lingering REAL  NOT NULL,
        mssl_trgt_end_opacity REAL  NOT NULL,
        mssl_trgt_arc_speed REAL  NOT NULL,
        mssl_trgt_arc_repeat REAL  NOT NULL,
        mssl_trgt_arc_width REAL  NOT NULL,
        mssl_trgt_impact_radius_0 REAL NOT NULL,
        mssl_trgt_impact_radius_1 REAL NOT NULL,
        mssl_trgt_arc_texture TEXT  NOT NULL,
        mssl_trgt_impact_texture TEXT  NOT NULL,
        mssl_trgt_impact_model_0 TEXT NOT NULL,
        mssl_trgt_impact_model_1 TEXT NOT NULL,
        camera_yaw_offset REAL  NOT NULL,
        ui_locomotion_type INTEGER  NOT NULL,
        mssl_trgt_impact_tex_radius REAL  NOT NULL,
        vehicle_u_i_indicator_id INTEGER  NOT NULL,
        power_display_id_0 INTEGER NOT NULL,
        power_display_id_1 INTEGER NOT NULL,
        power_display_id_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO Vehicle (
        id,
        flags,
        turn_speed,
        pitch_speed,
        pitch_min,
        pitch_max,
        seat_id_0,
        seat_id_1,
        seat_id_2,
        seat_id_3,
        seat_id_4,
        seat_id_5,
        seat_id_6,
        seat_id_7,
        mouse_look_offset_pitch,
        camera_fade_dist_scalar_min,
        camera_fade_dist_scalar_max,
        camera_pitch_offset,
        facing_limit_right,
        facing_limit_left,
        mssl_trgt_turn_lingering,
        mssl_trgt_pitch_lingering,
        mssl_trgt_mouse_lingering,
        mssl_trgt_end_opacity,
        mssl_trgt_arc_speed,
        mssl_trgt_arc_repeat,
        mssl_trgt_arc_width,
        mssl_trgt_impact_radius_0,
        mssl_trgt_impact_radius_1,
        mssl_trgt_arc_texture,
        mssl_trgt_impact_texture,
        mssl_trgt_impact_model_0,
        mssl_trgt_impact_model_1,
        camera_yaw_offset,
        ui_locomotion_type,
        mssl_trgt_impact_tex_radius,
        vehicle_u_i_indicator_id,
        power_display_id_0,
        power_display_id_1,
        power_display_id_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Startup_Strings() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Startup_Strings (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        message_lang_en_gb TEXT NOT NULL,
        message_lang_ko_kr TEXT NOT NULL,
        message_lang_fr_fr TEXT NOT NULL,
        message_lang_de_de TEXT NOT NULL,
        message_lang_en_cn TEXT NOT NULL,
        message_lang_en_tw TEXT NOT NULL,
        message_lang_es_es TEXT NOT NULL,
        message_lang_es_mx TEXT NOT NULL,
        message_lang_ru_ru TEXT NOT NULL,
        message_lang_ja_jp TEXT NOT NULL,
        message_lang_pt_pt TEXT NOT NULL,
        message_lang_it_it TEXT NOT NULL,
        message_lang_unknown_12 TEXT NOT NULL,
        message_lang_unknown_13 TEXT NOT NULL,
        message_lang_unknown_14 TEXT NOT NULL,
        message_lang_unknown_15 TEXT NOT NULL,
        message_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Startup_Strings (
        id,
        name,
        message_lang_en_gb,
        message_lang_ko_kr,
        message_lang_fr_fr,
        message_lang_de_de,
        message_lang_en_cn,
        message_lang_en_tw,
        message_lang_es_es,
        message_lang_es_mx,
        message_lang_ru_ru,
        message_lang_ja_jp,
        message_lang_pt_pt,
        message_lang_it_it,
        message_lang_unknown_12,
        message_lang_unknown_13,
        message_lang_unknown_14,
        message_lang_unknown_15,
        message_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtBarberShopCostBase() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtBarberShopCostBase (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtBarberShopCostBase (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Holidays() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Holidays (
        id INTEGER PRIMARY KEY NOT NULL,
        duration_0 INTEGER NOT NULL,
        duration_1 INTEGER NOT NULL,
        duration_2 INTEGER NOT NULL,
        duration_3 INTEGER NOT NULL,
        duration_4 INTEGER NOT NULL,
        duration_5 INTEGER NOT NULL,
        duration_6 INTEGER NOT NULL,
        duration_7 INTEGER NOT NULL,
        duration_8 INTEGER NOT NULL,
        duration_9 INTEGER NOT NULL,
        date_0 INTEGER NOT NULL,
        date_1 INTEGER NOT NULL,
        date_2 INTEGER NOT NULL,
        date_3 INTEGER NOT NULL,
        date_4 INTEGER NOT NULL,
        date_5 INTEGER NOT NULL,
        date_6 INTEGER NOT NULL,
        date_7 INTEGER NOT NULL,
        date_8 INTEGER NOT NULL,
        date_9 INTEGER NOT NULL,
        date_10 INTEGER NOT NULL,
        date_11 INTEGER NOT NULL,
        date_12 INTEGER NOT NULL,
        date_13 INTEGER NOT NULL,
        date_14 INTEGER NOT NULL,
        date_15 INTEGER NOT NULL,
        date_16 INTEGER NOT NULL,
        date_17 INTEGER NOT NULL,
        date_18 INTEGER NOT NULL,
        date_19 INTEGER NOT NULL,
        date_20 INTEGER NOT NULL,
        date_21 INTEGER NOT NULL,
        date_22 INTEGER NOT NULL,
        date_23 INTEGER NOT NULL,
        date_24 INTEGER NOT NULL,
        date_25 INTEGER NOT NULL,
        region INTEGER  NOT NULL,
        looping INTEGER  NOT NULL,
        calendar_flags_0 INTEGER NOT NULL,
        calendar_flags_1 INTEGER NOT NULL,
        calendar_flags_2 INTEGER NOT NULL,
        calendar_flags_3 INTEGER NOT NULL,
        calendar_flags_4 INTEGER NOT NULL,
        calendar_flags_5 INTEGER NOT NULL,
        calendar_flags_6 INTEGER NOT NULL,
        calendar_flags_7 INTEGER NOT NULL,
        calendar_flags_8 INTEGER NOT NULL,
        calendar_flags_9 INTEGER NOT NULL,
        holiday_name_id INTEGER  NOT NULL,
        holiday_description_id INTEGER  NOT NULL,
        texture_file_name TEXT  NOT NULL,
        priority INTEGER  NOT NULL,
        calendar_filter_type INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Holidays (
        id,
        duration_0,
        duration_1,
        duration_2,
        duration_3,
        duration_4,
        duration_5,
        duration_6,
        duration_7,
        duration_8,
        duration_9,
        date_0,
        date_1,
        date_2,
        date_3,
        date_4,
        date_5,
        date_6,
        date_7,
        date_8,
        date_9,
        date_10,
        date_11,
        date_12,
        date_13,
        date_14,
        date_15,
        date_16,
        date_17,
        date_18,
        date_19,
        date_20,
        date_21,
        date_22,
        date_23,
        date_24,
        date_25,
        region,
        looping,
        calendar_flags_0,
        calendar_flags_1,
        calendar_flags_2,
        calendar_flags_3,
        calendar_flags_4,
        calendar_flags_5,
        calendar_flags_6,
        calendar_flags_7,
        calendar_flags_8,
        calendar_flags_9,
        holiday_name_id,
        holiday_description_id,
        texture_file_name,
        priority,
        calendar_filter_type,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtRegenMPPerSpt() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtRegenMPPerSpt (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtRegenMPPerSpt (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundWaterType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundWaterType (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_type INTEGER  NOT NULL,
        sound_subtype INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundWaterType (
        id,
        sound_type,
        sound_subtype,
        sound_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LanguageWords() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LanguageWords (
        id INTEGER PRIMARY KEY NOT NULL,
        language_id INTEGER  NOT NULL,
        word TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LanguageWords (
        id,
        language_id,
        word
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TerrainTypeSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TerrainTypeSounds (
        id INTEGER PRIMARY KEY NOT NULL
    );"
    ,
    "INSERT INTO TerrainTypeSounds (
        id
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldMapContinent() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapContinent (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        left_boundary INTEGER  NOT NULL,
        right_boundary INTEGER  NOT NULL,
        top_boundary INTEGER  NOT NULL,
        bottom_boundary INTEGER  NOT NULL,
        continent_offset_0 REAL NOT NULL,
        continent_offset_1 REAL NOT NULL,
        scale REAL  NOT NULL,
        taxi_min_0 REAL NOT NULL,
        taxi_min_1 REAL NOT NULL,
        taxi_max_0 REAL NOT NULL,
        taxi_max_1 REAL NOT NULL,
        world_map_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WorldMapContinent (
        id,
        map_id,
        left_boundary,
        right_boundary,
        top_boundary,
        bottom_boundary,
        continent_offset_0,
        continent_offset_1,
        scale,
        taxi_min_0,
        taxi_min_1,
        taxi_max_0,
        taxi_max_1,
        world_map_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Spell() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Spell (
        id INTEGER PRIMARY KEY NOT NULL,
        category INTEGER  NOT NULL,
        dispel_type INTEGER  NOT NULL,
        mechanic INTEGER  NOT NULL,
        attributes INTEGER  NOT NULL,
        attributes_ex INTEGER  NOT NULL,
        attributes_ex_b INTEGER  NOT NULL,
        attributes_ex_c INTEGER  NOT NULL,
        attributes_ex_d INTEGER  NOT NULL,
        attributes_ex_e INTEGER  NOT NULL,
        attributes_ex_f INTEGER  NOT NULL,
        attributes_ex_g INTEGER  NOT NULL,
        shapeshift_mask_0 INTEGER NOT NULL,
        shapeshift_mask_1 INTEGER NOT NULL,
        shapeshift_exclude_0 INTEGER NOT NULL,
        shapeshift_exclude_1 INTEGER NOT NULL,
        targets INTEGER  NOT NULL,
        target_creature_type INTEGER  NOT NULL,
        requires_spell_focus INTEGER  NOT NULL,
        facing_caster_flags INTEGER  NOT NULL,
        caster_aura_state INTEGER  NOT NULL,
        target_aura_state INTEGER  NOT NULL,
        exclude_caster_aura_state INTEGER  NOT NULL,
        exclude_target_aura_state INTEGER  NOT NULL,
        caster_aura_spell INTEGER  NOT NULL,
        target_aura_spell INTEGER  NOT NULL,
        exclude_caster_aura_spell INTEGER  NOT NULL,
        exclude_target_aura_spell INTEGER  NOT NULL,
        casting_time_index INTEGER  NOT NULL,
        recovery_time INTEGER  NOT NULL,
        category_recovery_time INTEGER  NOT NULL,
        interrupt_flags INTEGER  NOT NULL,
        aura_interrupt_flags INTEGER  NOT NULL,
        channel_interrupt_flags INTEGER  NOT NULL,
        proc_type_mask INTEGER  NOT NULL,
        proc_chance INTEGER  NOT NULL,
        proc_charges INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        base_level INTEGER  NOT NULL,
        spell_level INTEGER  NOT NULL,
        duration_index INTEGER  NOT NULL,
        power_type INTEGER  NOT NULL,
        mana_cost INTEGER  NOT NULL,
        mana_cost_per_level INTEGER  NOT NULL,
        mana_per_second INTEGER  NOT NULL,
        mana_per_second_per_level INTEGER  NOT NULL,
        range_index INTEGER  NOT NULL,
        speed REAL  NOT NULL,
        modal_next_spell INTEGER  NOT NULL,
        cumulative_aura INTEGER  NOT NULL,
        totem_0 INTEGER NOT NULL,
        totem_1 INTEGER NOT NULL,
        reagent_0 INTEGER NOT NULL,
        reagent_1 INTEGER NOT NULL,
        reagent_2 INTEGER NOT NULL,
        reagent_3 INTEGER NOT NULL,
        reagent_4 INTEGER NOT NULL,
        reagent_5 INTEGER NOT NULL,
        reagent_6 INTEGER NOT NULL,
        reagent_7 INTEGER NOT NULL,
        reagent_count_0 INTEGER NOT NULL,
        reagent_count_1 INTEGER NOT NULL,
        reagent_count_2 INTEGER NOT NULL,
        reagent_count_3 INTEGER NOT NULL,
        reagent_count_4 INTEGER NOT NULL,
        reagent_count_5 INTEGER NOT NULL,
        reagent_count_6 INTEGER NOT NULL,
        reagent_count_7 INTEGER NOT NULL,
        equipped_item_class INTEGER  NOT NULL,
        equipped_item_subclass INTEGER  NOT NULL,
        equipped_item_inv_types INTEGER  NOT NULL,
        effect_0 INTEGER NOT NULL,
        effect_1 INTEGER NOT NULL,
        effect_2 INTEGER NOT NULL,
        effect_die_sides_0 INTEGER NOT NULL,
        effect_die_sides_1 INTEGER NOT NULL,
        effect_die_sides_2 INTEGER NOT NULL,
        effect_real_points_per_level_0 REAL NOT NULL,
        effect_real_points_per_level_1 REAL NOT NULL,
        effect_real_points_per_level_2 REAL NOT NULL,
        effect_base_points_0 INTEGER NOT NULL,
        effect_base_points_1 INTEGER NOT NULL,
        effect_base_points_2 INTEGER NOT NULL,
        effect_mechanic_0 INTEGER NOT NULL,
        effect_mechanic_1 INTEGER NOT NULL,
        effect_mechanic_2 INTEGER NOT NULL,
        implicit_target_a_0 INTEGER NOT NULL,
        implicit_target_a_1 INTEGER NOT NULL,
        implicit_target_a_2 INTEGER NOT NULL,
        implicit_target_b_0 INTEGER NOT NULL,
        implicit_target_b_1 INTEGER NOT NULL,
        implicit_target_b_2 INTEGER NOT NULL,
        effect_radius_index_0 INTEGER NOT NULL,
        effect_radius_index_1 INTEGER NOT NULL,
        effect_radius_index_2 INTEGER NOT NULL,
        effect_aura_0 INTEGER NOT NULL,
        effect_aura_1 INTEGER NOT NULL,
        effect_aura_2 INTEGER NOT NULL,
        effect_aura_period_0 INTEGER NOT NULL,
        effect_aura_period_1 INTEGER NOT NULL,
        effect_aura_period_2 INTEGER NOT NULL,
        effect_amplitude_0 REAL NOT NULL,
        effect_amplitude_1 REAL NOT NULL,
        effect_amplitude_2 REAL NOT NULL,
        effect_chain_targets_0 INTEGER NOT NULL,
        effect_chain_targets_1 INTEGER NOT NULL,
        effect_chain_targets_2 INTEGER NOT NULL,
        effect_item_type_0 INTEGER NOT NULL,
        effect_item_type_1 INTEGER NOT NULL,
        effect_item_type_2 INTEGER NOT NULL,
        effect_misc_value_0 INTEGER NOT NULL,
        effect_misc_value_1 INTEGER NOT NULL,
        effect_misc_value_2 INTEGER NOT NULL,
        effect_misc_value_b_0 INTEGER NOT NULL,
        effect_misc_value_b_1 INTEGER NOT NULL,
        effect_misc_value_b_2 INTEGER NOT NULL,
        effect_trigger_spell_0 INTEGER NOT NULL,
        effect_trigger_spell_1 INTEGER NOT NULL,
        effect_trigger_spell_2 INTEGER NOT NULL,
        effect_points_per_combo_0 REAL NOT NULL,
        effect_points_per_combo_1 REAL NOT NULL,
        effect_points_per_combo_2 REAL NOT NULL,
        effect_spell_class_mask_a_0 INTEGER NOT NULL,
        effect_spell_class_mask_a_1 INTEGER NOT NULL,
        effect_spell_class_mask_a_2 INTEGER NOT NULL,
        effect_spell_class_mask_b_0 INTEGER NOT NULL,
        effect_spell_class_mask_b_1 INTEGER NOT NULL,
        effect_spell_class_mask_b_2 INTEGER NOT NULL,
        effect_spell_class_mask_c_0 INTEGER NOT NULL,
        effect_spell_class_mask_c_1 INTEGER NOT NULL,
        effect_spell_class_mask_c_2 INTEGER NOT NULL,
        spell_visual_id_0 INTEGER NOT NULL,
        spell_visual_id_1 INTEGER NOT NULL,
        spell_icon_id INTEGER  NOT NULL,
        active_icon_id INTEGER  NOT NULL,
        spell_priority INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        name_subtext_lang_en_gb TEXT NOT NULL,
        name_subtext_lang_ko_kr TEXT NOT NULL,
        name_subtext_lang_fr_fr TEXT NOT NULL,
        name_subtext_lang_de_de TEXT NOT NULL,
        name_subtext_lang_en_cn TEXT NOT NULL,
        name_subtext_lang_en_tw TEXT NOT NULL,
        name_subtext_lang_es_es TEXT NOT NULL,
        name_subtext_lang_es_mx TEXT NOT NULL,
        name_subtext_lang_ru_ru TEXT NOT NULL,
        name_subtext_lang_ja_jp TEXT NOT NULL,
        name_subtext_lang_pt_pt TEXT NOT NULL,
        name_subtext_lang_it_it TEXT NOT NULL,
        name_subtext_lang_unknown_12 TEXT NOT NULL,
        name_subtext_lang_unknown_13 TEXT NOT NULL,
        name_subtext_lang_unknown_14 TEXT NOT NULL,
        name_subtext_lang_unknown_15 TEXT NOT NULL,
        name_subtext_lang_flags TEXT NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL,
        aura_description_lang_en_gb TEXT NOT NULL,
        aura_description_lang_ko_kr TEXT NOT NULL,
        aura_description_lang_fr_fr TEXT NOT NULL,
        aura_description_lang_de_de TEXT NOT NULL,
        aura_description_lang_en_cn TEXT NOT NULL,
        aura_description_lang_en_tw TEXT NOT NULL,
        aura_description_lang_es_es TEXT NOT NULL,
        aura_description_lang_es_mx TEXT NOT NULL,
        aura_description_lang_ru_ru TEXT NOT NULL,
        aura_description_lang_ja_jp TEXT NOT NULL,
        aura_description_lang_pt_pt TEXT NOT NULL,
        aura_description_lang_it_it TEXT NOT NULL,
        aura_description_lang_unknown_12 TEXT NOT NULL,
        aura_description_lang_unknown_13 TEXT NOT NULL,
        aura_description_lang_unknown_14 TEXT NOT NULL,
        aura_description_lang_unknown_15 TEXT NOT NULL,
        aura_description_lang_flags TEXT NOT NULL,
        mana_cost_pct INTEGER  NOT NULL,
        start_recovery_category INTEGER  NOT NULL,
        start_recovery_time INTEGER  NOT NULL,
        max_target_level INTEGER  NOT NULL,
        spell_class_set INTEGER  NOT NULL,
        spell_class_mask_0 INTEGER NOT NULL,
        spell_class_mask_1 INTEGER NOT NULL,
        spell_class_mask_2 INTEGER NOT NULL,
        max_targets INTEGER  NOT NULL,
        defense_type INTEGER  NOT NULL,
        prevention_type INTEGER  NOT NULL,
        stance_bar_order INTEGER  NOT NULL,
        effect_chain_amplitude_0 REAL NOT NULL,
        effect_chain_amplitude_1 REAL NOT NULL,
        effect_chain_amplitude_2 REAL NOT NULL,
        min_faction_id INTEGER  NOT NULL,
        min_reputation INTEGER  NOT NULL,
        required_aura_vision INTEGER  NOT NULL,
        required_totem_category_id_0 INTEGER NOT NULL,
        required_totem_category_id_1 INTEGER NOT NULL,
        required_areas_id INTEGER  NOT NULL,
        school_mask INTEGER  NOT NULL,
        rune_cost_id INTEGER  NOT NULL,
        spell_missile_id INTEGER  NOT NULL,
        power_display_id INTEGER  NOT NULL,
        effect_bonus_coefficient_0 REAL NOT NULL,
        effect_bonus_coefficient_1 REAL NOT NULL,
        effect_bonus_coefficient_2 REAL NOT NULL,
        description_variables_id INTEGER  NOT NULL,
        difficulty INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Spell (
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
        shapeshift_mask_0,
        shapeshift_mask_1,
        shapeshift_exclude_0,
        shapeshift_exclude_1,
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
        totem_0,
        totem_1,
        reagent_0,
        reagent_1,
        reagent_2,
        reagent_3,
        reagent_4,
        reagent_5,
        reagent_6,
        reagent_7,
        reagent_count_0,
        reagent_count_1,
        reagent_count_2,
        reagent_count_3,
        reagent_count_4,
        reagent_count_5,
        reagent_count_6,
        reagent_count_7,
        equipped_item_class,
        equipped_item_subclass,
        equipped_item_inv_types,
        effect_0,
        effect_1,
        effect_2,
        effect_die_sides_0,
        effect_die_sides_1,
        effect_die_sides_2,
        effect_real_points_per_level_0,
        effect_real_points_per_level_1,
        effect_real_points_per_level_2,
        effect_base_points_0,
        effect_base_points_1,
        effect_base_points_2,
        effect_mechanic_0,
        effect_mechanic_1,
        effect_mechanic_2,
        implicit_target_a_0,
        implicit_target_a_1,
        implicit_target_a_2,
        implicit_target_b_0,
        implicit_target_b_1,
        implicit_target_b_2,
        effect_radius_index_0,
        effect_radius_index_1,
        effect_radius_index_2,
        effect_aura_0,
        effect_aura_1,
        effect_aura_2,
        effect_aura_period_0,
        effect_aura_period_1,
        effect_aura_period_2,
        effect_amplitude_0,
        effect_amplitude_1,
        effect_amplitude_2,
        effect_chain_targets_0,
        effect_chain_targets_1,
        effect_chain_targets_2,
        effect_item_type_0,
        effect_item_type_1,
        effect_item_type_2,
        effect_misc_value_0,
        effect_misc_value_1,
        effect_misc_value_2,
        effect_misc_value_b_0,
        effect_misc_value_b_1,
        effect_misc_value_b_2,
        effect_trigger_spell_0,
        effect_trigger_spell_1,
        effect_trigger_spell_2,
        effect_points_per_combo_0,
        effect_points_per_combo_1,
        effect_points_per_combo_2,
        effect_spell_class_mask_a_0,
        effect_spell_class_mask_a_1,
        effect_spell_class_mask_a_2,
        effect_spell_class_mask_b_0,
        effect_spell_class_mask_b_1,
        effect_spell_class_mask_b_2,
        effect_spell_class_mask_c_0,
        effect_spell_class_mask_c_1,
        effect_spell_class_mask_c_2,
        spell_visual_id_0,
        spell_visual_id_1,
        spell_icon_id,
        active_icon_id,
        spell_priority,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        name_subtext_lang_en_gb,
        name_subtext_lang_ko_kr,
        name_subtext_lang_fr_fr,
        name_subtext_lang_de_de,
        name_subtext_lang_en_cn,
        name_subtext_lang_en_tw,
        name_subtext_lang_es_es,
        name_subtext_lang_es_mx,
        name_subtext_lang_ru_ru,
        name_subtext_lang_ja_jp,
        name_subtext_lang_pt_pt,
        name_subtext_lang_it_it,
        name_subtext_lang_unknown_12,
        name_subtext_lang_unknown_13,
        name_subtext_lang_unknown_14,
        name_subtext_lang_unknown_15,
        name_subtext_lang_flags,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags,
        aura_description_lang_en_gb,
        aura_description_lang_ko_kr,
        aura_description_lang_fr_fr,
        aura_description_lang_de_de,
        aura_description_lang_en_cn,
        aura_description_lang_en_tw,
        aura_description_lang_es_es,
        aura_description_lang_es_mx,
        aura_description_lang_ru_ru,
        aura_description_lang_ja_jp,
        aura_description_lang_pt_pt,
        aura_description_lang_it_it,
        aura_description_lang_unknown_12,
        aura_description_lang_unknown_13,
        aura_description_lang_unknown_14,
        aura_description_lang_unknown_15,
        aura_description_lang_flags,
        mana_cost_pct,
        start_recovery_category,
        start_recovery_time,
        max_target_level,
        spell_class_set,
        spell_class_mask_0,
        spell_class_mask_1,
        spell_class_mask_2,
        max_targets,
        defense_type,
        prevention_type,
        stance_bar_order,
        effect_chain_amplitude_0,
        effect_chain_amplitude_1,
        effect_chain_amplitude_2,
        min_faction_id,
        min_reputation,
        required_aura_vision,
        required_totem_category_id_0,
        required_totem_category_id_1,
        required_areas_id,
        school_mask,
        rune_cost_id,
        spell_missile_id,
        power_display_id,
        effect_bonus_coefficient_0,
        effect_bonus_coefficient_1,
        effect_bonus_coefficient_2,
        description_variables_id,
        difficulty
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60,
        ?61,
        ?62,
        ?63,
        ?64,
        ?65,
        ?66,
        ?67,
        ?68,
        ?69,
        ?70,
        ?71,
        ?72,
        ?73,
        ?74,
        ?75,
        ?76,
        ?77,
        ?78,
        ?79,
        ?80,
        ?81,
        ?82,
        ?83,
        ?84,
        ?85,
        ?86,
        ?87,
        ?88,
        ?89,
        ?90,
        ?91,
        ?92,
        ?93,
        ?94,
        ?95,
        ?96,
        ?97,
        ?98,
        ?99,
        ?100,
        ?101,
        ?102,
        ?103,
        ?104,
        ?105,
        ?106,
        ?107,
        ?108,
        ?109,
        ?110,
        ?111,
        ?112,
        ?113,
        ?114,
        ?115,
        ?116,
        ?117,
        ?118,
        ?119,
        ?120,
        ?121,
        ?122,
        ?123,
        ?124,
        ?125,
        ?126,
        ?127,
        ?128,
        ?129,
        ?130,
        ?131,
        ?132,
        ?133,
        ?134,
        ?135,
        ?136,
        ?137,
        ?138,
        ?139,
        ?140,
        ?141,
        ?142,
        ?143,
        ?144,
        ?145,
        ?146,
        ?147,
        ?148,
        ?149,
        ?150,
        ?151,
        ?152,
        ?153,
        ?154,
        ?155,
        ?156,
        ?157,
        ?158,
        ?159,
        ?160,
        ?161,
        ?162,
        ?163,
        ?164,
        ?165,
        ?166,
        ?167,
        ?168,
        ?169,
        ?170,
        ?171,
        ?172,
        ?173,
        ?174,
        ?175,
        ?176,
        ?177,
        ?178,
        ?179,
        ?180,
        ?181,
        ?182,
        ?183,
        ?184,
        ?185,
        ?186,
        ?187,
        ?188,
        ?189,
        ?190,
        ?191,
        ?192,
        ?193,
        ?194,
        ?195,
        ?196,
        ?197,
        ?198,
        ?199,
        ?200,
        ?201,
        ?202,
        ?203,
        ?204,
        ?205,
        ?206,
        ?207,
        ?208,
        ?209,
        ?210,
        ?211,
        ?212,
        ?213,
        ?214,
        ?215,
        ?216,
        ?217,
        ?218,
        ?219,
        ?220,
        ?221,
        ?222,
        ?223,
        ?224,
        ?225,
        ?226,
        ?227,
        ?228,
        ?229,
        ?230,
        ?231,
        ?232,
        ?233,
        ?234
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureDisplayInfoExtra() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureDisplayInfoExtra (
        id INTEGER PRIMARY KEY NOT NULL,
        display_race_id INTEGER  NOT NULL,
        display_sex_id INTEGER  NOT NULL,
        skin_id INTEGER  NOT NULL,
        face_id INTEGER  NOT NULL,
        hair_style_id INTEGER  NOT NULL,
        hair_color_id INTEGER  NOT NULL,
        facial_hair_id INTEGER  NOT NULL,
        n_p_c_item_display_0 INTEGER NOT NULL,
        n_p_c_item_display_1 INTEGER NOT NULL,
        n_p_c_item_display_2 INTEGER NOT NULL,
        n_p_c_item_display_3 INTEGER NOT NULL,
        n_p_c_item_display_4 INTEGER NOT NULL,
        n_p_c_item_display_5 INTEGER NOT NULL,
        n_p_c_item_display_6 INTEGER NOT NULL,
        n_p_c_item_display_7 INTEGER NOT NULL,
        n_p_c_item_display_8 INTEGER NOT NULL,
        n_p_c_item_display_9 INTEGER NOT NULL,
        n_p_c_item_display_10 INTEGER NOT NULL,
        flags INTEGER  NOT NULL,
        bake_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO CreatureDisplayInfoExtra (
        id,
        display_race_id,
        display_sex_id,
        skin_id,
        face_id,
        hair_style_id,
        hair_color_id,
        facial_hair_id,
        n_p_c_item_display_0,
        n_p_c_item_display_1,
        n_p_c_item_display_2,
        n_p_c_item_display_3,
        n_p_c_item_display_4,
        n_p_c_item_display_5,
        n_p_c_item_display_6,
        n_p_c_item_display_7,
        n_p_c_item_display_8,
        n_p_c_item_display_9,
        n_p_c_item_display_10,
        flags,
        bake_name
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemSubClassMask() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemSubClassMask (
        class_id INTEGER  NOT NULL,
        mask INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemSubClassMask (
        class_id,
        mask,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtChanceToSpellCrit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtChanceToSpellCrit (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtChanceToSpellCrit (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillTiers() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillTiers (
        id INTEGER PRIMARY KEY NOT NULL,
        cost_0 INTEGER NOT NULL,
        cost_1 INTEGER NOT NULL,
        cost_2 INTEGER NOT NULL,
        cost_3 INTEGER NOT NULL,
        cost_4 INTEGER NOT NULL,
        cost_5 INTEGER NOT NULL,
        cost_6 INTEGER NOT NULL,
        cost_7 INTEGER NOT NULL,
        cost_8 INTEGER NOT NULL,
        cost_9 INTEGER NOT NULL,
        cost_10 INTEGER NOT NULL,
        cost_11 INTEGER NOT NULL,
        cost_12 INTEGER NOT NULL,
        cost_13 INTEGER NOT NULL,
        cost_14 INTEGER NOT NULL,
        cost_15 INTEGER NOT NULL,
        value_0 INTEGER NOT NULL,
        value_1 INTEGER NOT NULL,
        value_2 INTEGER NOT NULL,
        value_3 INTEGER NOT NULL,
        value_4 INTEGER NOT NULL,
        value_5 INTEGER NOT NULL,
        value_6 INTEGER NOT NULL,
        value_7 INTEGER NOT NULL,
        value_8 INTEGER NOT NULL,
        value_9 INTEGER NOT NULL,
        value_10 INTEGER NOT NULL,
        value_11 INTEGER NOT NULL,
        value_12 INTEGER NOT NULL,
        value_13 INTEGER NOT NULL,
        value_14 INTEGER NOT NULL,
        value_15 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SkillTiers (
        id,
        cost_0,
        cost_1,
        cost_2,
        cost_3,
        cost_4,
        cost_5,
        cost_6,
        cost_7,
        cost_8,
        cost_9,
        cost_10,
        cost_11,
        cost_12,
        cost_13,
        cost_14,
        cost_15,
        value_0,
        value_1,
        value_2,
        value_3,
        value_4,
        value_5,
        value_6,
        value_7,
        value_8,
        value_9,
        value_10,
        value_11,
        value_12,
        value_13,
        value_14,
        value_15
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellVisualKitModelAttach() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualKitModelAttach (
        id INTEGER PRIMARY KEY NOT NULL,
        parent_spell_visual_kit_id INTEGER  NOT NULL,
        spell_visual_effect_name_id INTEGER  NOT NULL,
        attachment_id INTEGER  NOT NULL,
        offset_0 REAL NOT NULL,
        offset_1 REAL NOT NULL,
        offset_2 REAL NOT NULL,
        yaw REAL  NOT NULL,
        pitch REAL  NOT NULL,
        roll REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualKitModelAttach (
        id,
        parent_spell_visual_kit_id,
        spell_visual_effect_name_id,
        attachment_id,
        offset_0,
        offset_1,
        offset_2,
        yaw,
        pitch,
        roll
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn BankBagSlotPrices() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS BankBagSlotPrices (
        id INTEGER PRIMARY KEY NOT NULL,
        cost INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO BankBagSlotPrices (
        id,
        cost
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LoadingScreenTaxiSplines() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LoadingScreenTaxiSplines (
        id INTEGER PRIMARY KEY NOT NULL,
        path_id INTEGER  NOT NULL,
        locx_0 REAL NOT NULL,
        locx_1 REAL NOT NULL,
        locx_2 REAL NOT NULL,
        locx_3 REAL NOT NULL,
        locx_4 REAL NOT NULL,
        locx_5 REAL NOT NULL,
        locx_6 REAL NOT NULL,
        locx_7 REAL NOT NULL,
        locy_0 REAL NOT NULL,
        locy_1 REAL NOT NULL,
        locy_2 REAL NOT NULL,
        locy_3 REAL NOT NULL,
        locy_4 REAL NOT NULL,
        locy_5 REAL NOT NULL,
        locy_6 REAL NOT NULL,
        locy_7 REAL NOT NULL,
        leg_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LoadingScreenTaxiSplines (
        id,
        path_id,
        locx_0,
        locx_1,
        locx_2,
        locx_3,
        locx_4,
        locx_5,
        locx_6,
        locx_7,
        locy_0,
        locy_1,
        locy_2,
        locy_3,
        locy_4,
        locy_5,
        locy_6,
        locy_7,
        leg_index
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AreaPOI() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaPOI (
        id INTEGER PRIMARY KEY NOT NULL,
        importance INTEGER  NOT NULL,
        icon_0 INTEGER NOT NULL,
        icon_1 INTEGER NOT NULL,
        icon_2 INTEGER NOT NULL,
        icon_3 INTEGER NOT NULL,
        icon_4 INTEGER NOT NULL,
        icon_5 INTEGER NOT NULL,
        icon_6 INTEGER NOT NULL,
        icon_7 INTEGER NOT NULL,
        icon_8 INTEGER NOT NULL,
        faction_id INTEGER  NOT NULL,
        pos_0 REAL NOT NULL,
        pos_1 REAL NOT NULL,
        pos_2 REAL NOT NULL,
        continent_id INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        area_id INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL,
        world_state_id INTEGER  NOT NULL,
        world_map_link INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AreaPOI (
        id,
        importance,
        icon_0,
        icon_1,
        icon_2,
        icon_3,
        icon_4,
        icon_5,
        icon_6,
        icon_7,
        icon_8,
        faction_id,
        pos_0,
        pos_1,
        pos_2,
        continent_id,
        flags,
        area_id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags,
        world_state_id,
        world_map_link
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TransportRotation() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TransportRotation (
        id INTEGER PRIMARY KEY NOT NULL,
        game_objects_id INTEGER  NOT NULL,
        time_index INTEGER  NOT NULL,
        rot_0 REAL NOT NULL,
        rot_1 REAL NOT NULL,
        rot_2 REAL NOT NULL,
        rot_3 REAL NOT NULL
    );"
    ,
    "INSERT INTO TransportRotation (
        id,
        game_objects_id,
        time_index,
        rot_0,
        rot_1,
        rot_2,
        rot_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundFilterElem() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundFilterElem (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_filter_id INTEGER  NOT NULL,
        order_index INTEGER  NOT NULL,
        filter_type INTEGER  NOT NULL,
        params_0 REAL NOT NULL,
        params_1 REAL NOT NULL,
        params_2 REAL NOT NULL,
        params_3 REAL NOT NULL,
        params_4 REAL NOT NULL,
        params_5 REAL NOT NULL,
        params_6 REAL NOT NULL,
        params_7 REAL NOT NULL,
        params_8 REAL NOT NULL
    );"
    ,
    "INSERT INTO SoundFilterElem (
        id,
        sound_filter_id,
        order_index,
        filter_type,
        params_0,
        params_1,
        params_2,
        params_3,
        params_4,
        params_5,
        params_6,
        params_7,
        params_8
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn PaperDollItemFrame() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PaperDollItemFrame (
        item_button_name TEXT  NOT NULL,
        slot_icon TEXT  NOT NULL,
        slot_number INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO PaperDollItemFrame (
        item_button_name,
        slot_icon,
        slot_number
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemDisplayInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemDisplayInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        model_name_0 TEXT NOT NULL,
        model_name_1 TEXT NOT NULL,
        model_texture_0 TEXT NOT NULL,
        model_texture_1 TEXT NOT NULL,
        inventory_icon_0 TEXT NOT NULL,
        inventory_icon_1 TEXT NOT NULL,
        geoset_group_0 INTEGER NOT NULL,
        geoset_group_1 INTEGER NOT NULL,
        geoset_group_2 INTEGER NOT NULL,
        flags INTEGER  NOT NULL,
        spell_visual_id INTEGER  NOT NULL,
        group_sound_index INTEGER  NOT NULL,
        helmet_geoset_vis_id_0 INTEGER NOT NULL,
        helmet_geoset_vis_id_1 INTEGER NOT NULL,
        texture_0 TEXT NOT NULL,
        texture_1 TEXT NOT NULL,
        texture_2 TEXT NOT NULL,
        texture_3 TEXT NOT NULL,
        texture_4 TEXT NOT NULL,
        texture_5 TEXT NOT NULL,
        texture_6 TEXT NOT NULL,
        texture_7 TEXT NOT NULL,
        item_visual INTEGER  NOT NULL,
        particle_color_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ItemDisplayInfo (
        id,
        model_name_0,
        model_name_1,
        model_texture_0,
        model_texture_1,
        inventory_icon_0,
        inventory_icon_1,
        geoset_group_0,
        geoset_group_1,
        geoset_group_2,
        flags,
        spell_visual_id,
        group_sound_index,
        helmet_geoset_vis_id_0,
        helmet_geoset_vis_id_1,
        texture_0,
        texture_1,
        texture_2,
        texture_3,
        texture_4,
        texture_5,
        texture_6,
        texture_7,
        item_visual,
        particle_color_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundEntriesAdvanced() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundEntriesAdvanced (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_entry_id INTEGER  NOT NULL,
        inner_radius2_d REAL  NOT NULL,
        time_a INTEGER  NOT NULL,
        time_b INTEGER  NOT NULL,
        time_c INTEGER  NOT NULL,
        time_d INTEGER  NOT NULL,
        random_offset_range INTEGER  NOT NULL,
        usage INTEGER  NOT NULL,
        time_interval_min INTEGER  NOT NULL,
        time_interval_max INTEGER  NOT NULL,
        volume_slider_category INTEGER  NOT NULL,
        duck_to_s_f_x REAL  NOT NULL,
        duck_to_music REAL  NOT NULL,
        duck_to_ambience REAL  NOT NULL,
        inner_radius_of_influence REAL  NOT NULL,
        outer_radius_of_influence REAL  NOT NULL,
        time_to_duck INTEGER  NOT NULL,
        time_to_unduck INTEGER  NOT NULL,
        inside_angle REAL  NOT NULL,
        outside_angle REAL  NOT NULL,
        outside_volume REAL  NOT NULL,
        outer_radius2_d REAL  NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SoundEntriesAdvanced (
        id,
        sound_entry_id,
        inner_radius2_d,
        time_a,
        time_b,
        time_c,
        time_d,
        random_offset_range,
        usage,
        time_interval_min,
        time_interval_max,
        volume_slider_category,
        duck_to_s_f_x,
        duck_to_music,
        duck_to_ambience,
        inner_radius_of_influence,
        outer_radius_of_influence,
        time_to_duck,
        time_to_unduck,
        inside_angle,
        outside_angle,
        outside_volume,
        outer_radius2_d,
        name
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellDifficulty() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellDifficulty (
        id INTEGER PRIMARY KEY NOT NULL,
        difficulty_spell_id_0 INTEGER NOT NULL,
        difficulty_spell_id_1 INTEGER NOT NULL,
        difficulty_spell_id_2 INTEGER NOT NULL,
        difficulty_spell_id_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SpellDifficulty (
        id,
        difficulty_spell_id_0,
        difficulty_spell_id_1,
        difficulty_spell_id_2,
        difficulty_spell_id_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LightSkybox() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LightSkybox (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LightSkybox (
        id,
        name,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ChatProfanity() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChatProfanity (
        id INTEGER PRIMARY KEY NOT NULL,
        text TEXT  NOT NULL,
        language INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ChatProfanity (
        id,
        text,
        language
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Material() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Material (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        foley_sound_id INTEGER  NOT NULL,
        sheathe_sound_id INTEGER  NOT NULL,
        unsheathe_sound_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Material (
        id,
        flags,
        foley_sound_id,
        sheathe_sound_id,
        unsheathe_sound_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Stationery() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Stationery (
        id INTEGER PRIMARY KEY NOT NULL,
        item_id INTEGER  NOT NULL,
        texture TEXT  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Stationery (
        id,
        item_id,
        texture,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LiquidType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LiquidType (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        flags INTEGER  NOT NULL,
        sound_bank INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL,
        spell_id INTEGER  NOT NULL,
        max_darken_depth REAL  NOT NULL,
        fog_darken_intensity REAL  NOT NULL,
        amb_darken_intensity REAL  NOT NULL,
        dir_darken_intensity REAL  NOT NULL,
        light_id INTEGER  NOT NULL,
        particle_scale REAL  NOT NULL,
        particle_movement INTEGER  NOT NULL,
        particle_tex_slots INTEGER  NOT NULL,
        material_id INTEGER  NOT NULL,
        texture_0 TEXT NOT NULL,
        texture_1 TEXT NOT NULL,
        texture_2 TEXT NOT NULL,
        texture_3 TEXT NOT NULL,
        texture_4 TEXT NOT NULL,
        texture_5 TEXT NOT NULL,
        color_0 INTEGER NOT NULL,
        color_1 INTEGER NOT NULL,
        float_0 REAL NOT NULL,
        float_1 REAL NOT NULL,
        float_2 REAL NOT NULL,
        float_3 REAL NOT NULL,
        float_4 REAL NOT NULL,
        float_5 REAL NOT NULL,
        float_6 REAL NOT NULL,
        float_7 REAL NOT NULL,
        float_8 REAL NOT NULL,
        float_9 REAL NOT NULL,
        float_10 REAL NOT NULL,
        float_11 REAL NOT NULL,
        float_12 REAL NOT NULL,
        float_13 REAL NOT NULL,
        float_14 REAL NOT NULL,
        float_15 REAL NOT NULL,
        float_16 REAL NOT NULL,
        float_17 REAL NOT NULL,
        int_0 INTEGER NOT NULL,
        int_1 INTEGER NOT NULL,
        int_2 INTEGER NOT NULL,
        int_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO LiquidType (
        id,
        name,
        flags,
        sound_bank,
        sound_id,
        spell_id,
        max_darken_depth,
        fog_darken_intensity,
        amb_darken_intensity,
        dir_darken_intensity,
        light_id,
        particle_scale,
        particle_movement,
        particle_tex_slots,
        material_id,
        texture_0,
        texture_1,
        texture_2,
        texture_3,
        texture_4,
        texture_5,
        color_0,
        color_1,
        float_0,
        float_1,
        float_2,
        float_3,
        float_4,
        float_5,
        float_6,
        float_7,
        float_8,
        float_9,
        float_10,
        float_11,
        float_12,
        float_13,
        float_14,
        float_15,
        float_16,
        float_17,
        int_0,
        int_1,
        int_2,
        int_3
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpamMessages() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpamMessages (
        id INTEGER PRIMARY KEY NOT NULL,
        text TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpamMessages (
        id,
        text
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AnimationData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AnimationData (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        weaponflags INTEGER  NOT NULL,
        bodyflags INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        fallback INTEGER  NOT NULL,
        behavior_id INTEGER  NOT NULL,
        behavior_tier INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AnimationData (
        id,
        name,
        weaponflags,
        bodyflags,
        flags,
        fallback,
        behavior_id,
        behavior_tier
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldStateUI() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldStateUI (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        area_id INTEGER  NOT NULL,
        phase_shift INTEGER  NOT NULL,
        icon TEXT  NOT NULL,
        string_lang_en_gb TEXT NOT NULL,
        string_lang_ko_kr TEXT NOT NULL,
        string_lang_fr_fr TEXT NOT NULL,
        string_lang_de_de TEXT NOT NULL,
        string_lang_en_cn TEXT NOT NULL,
        string_lang_en_tw TEXT NOT NULL,
        string_lang_es_es TEXT NOT NULL,
        string_lang_es_mx TEXT NOT NULL,
        string_lang_ru_ru TEXT NOT NULL,
        string_lang_ja_jp TEXT NOT NULL,
        string_lang_pt_pt TEXT NOT NULL,
        string_lang_it_it TEXT NOT NULL,
        string_lang_unknown_12 TEXT NOT NULL,
        string_lang_unknown_13 TEXT NOT NULL,
        string_lang_unknown_14 TEXT NOT NULL,
        string_lang_unknown_15 TEXT NOT NULL,
        string_lang_flags TEXT NOT NULL,
        tooltip_lang_en_gb TEXT NOT NULL,
        tooltip_lang_ko_kr TEXT NOT NULL,
        tooltip_lang_fr_fr TEXT NOT NULL,
        tooltip_lang_de_de TEXT NOT NULL,
        tooltip_lang_en_cn TEXT NOT NULL,
        tooltip_lang_en_tw TEXT NOT NULL,
        tooltip_lang_es_es TEXT NOT NULL,
        tooltip_lang_es_mx TEXT NOT NULL,
        tooltip_lang_ru_ru TEXT NOT NULL,
        tooltip_lang_ja_jp TEXT NOT NULL,
        tooltip_lang_pt_pt TEXT NOT NULL,
        tooltip_lang_it_it TEXT NOT NULL,
        tooltip_lang_unknown_12 TEXT NOT NULL,
        tooltip_lang_unknown_13 TEXT NOT NULL,
        tooltip_lang_unknown_14 TEXT NOT NULL,
        tooltip_lang_unknown_15 TEXT NOT NULL,
        tooltip_lang_flags TEXT NOT NULL,
        state_variable INTEGER  NOT NULL,
        ty INTEGER  NOT NULL,
        dynamic_icon TEXT  NOT NULL,
        dynamic_tooltip_lang_en_gb TEXT NOT NULL,
        dynamic_tooltip_lang_ko_kr TEXT NOT NULL,
        dynamic_tooltip_lang_fr_fr TEXT NOT NULL,
        dynamic_tooltip_lang_de_de TEXT NOT NULL,
        dynamic_tooltip_lang_en_cn TEXT NOT NULL,
        dynamic_tooltip_lang_en_tw TEXT NOT NULL,
        dynamic_tooltip_lang_es_es TEXT NOT NULL,
        dynamic_tooltip_lang_es_mx TEXT NOT NULL,
        dynamic_tooltip_lang_ru_ru TEXT NOT NULL,
        dynamic_tooltip_lang_ja_jp TEXT NOT NULL,
        dynamic_tooltip_lang_pt_pt TEXT NOT NULL,
        dynamic_tooltip_lang_it_it TEXT NOT NULL,
        dynamic_tooltip_lang_unknown_12 TEXT NOT NULL,
        dynamic_tooltip_lang_unknown_13 TEXT NOT NULL,
        dynamic_tooltip_lang_unknown_14 TEXT NOT NULL,
        dynamic_tooltip_lang_unknown_15 TEXT NOT NULL,
        dynamic_tooltip_lang_flags TEXT NOT NULL,
        extended_u_i TEXT  NOT NULL,
        extended_u_i_state_variable_0 INTEGER NOT NULL,
        extended_u_i_state_variable_1 INTEGER NOT NULL,
        extended_u_i_state_variable_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO WorldStateUI (
        id,
        map_id,
        area_id,
        phase_shift,
        icon,
        string_lang_en_gb,
        string_lang_ko_kr,
        string_lang_fr_fr,
        string_lang_de_de,
        string_lang_en_cn,
        string_lang_en_tw,
        string_lang_es_es,
        string_lang_es_mx,
        string_lang_ru_ru,
        string_lang_ja_jp,
        string_lang_pt_pt,
        string_lang_it_it,
        string_lang_unknown_12,
        string_lang_unknown_13,
        string_lang_unknown_14,
        string_lang_unknown_15,
        string_lang_flags,
        tooltip_lang_en_gb,
        tooltip_lang_ko_kr,
        tooltip_lang_fr_fr,
        tooltip_lang_de_de,
        tooltip_lang_en_cn,
        tooltip_lang_en_tw,
        tooltip_lang_es_es,
        tooltip_lang_es_mx,
        tooltip_lang_ru_ru,
        tooltip_lang_ja_jp,
        tooltip_lang_pt_pt,
        tooltip_lang_it_it,
        tooltip_lang_unknown_12,
        tooltip_lang_unknown_13,
        tooltip_lang_unknown_14,
        tooltip_lang_unknown_15,
        tooltip_lang_flags,
        state_variable,
        ty,
        dynamic_icon,
        dynamic_tooltip_lang_en_gb,
        dynamic_tooltip_lang_ko_kr,
        dynamic_tooltip_lang_fr_fr,
        dynamic_tooltip_lang_de_de,
        dynamic_tooltip_lang_en_cn,
        dynamic_tooltip_lang_en_tw,
        dynamic_tooltip_lang_es_es,
        dynamic_tooltip_lang_es_mx,
        dynamic_tooltip_lang_ru_ru,
        dynamic_tooltip_lang_ja_jp,
        dynamic_tooltip_lang_pt_pt,
        dynamic_tooltip_lang_it_it,
        dynamic_tooltip_lang_unknown_12,
        dynamic_tooltip_lang_unknown_13,
        dynamic_tooltip_lang_unknown_14,
        dynamic_tooltip_lang_unknown_15,
        dynamic_tooltip_lang_flags,
        extended_u_i,
        extended_u_i_state_variable_0,
        extended_u_i_state_variable_1,
        extended_u_i_state_variable_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60,
        ?61,
        ?62,
        ?63
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ScalingStatDistribution() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ScalingStatDistribution (
        id INTEGER PRIMARY KEY NOT NULL,
        stat_id_0 INTEGER NOT NULL,
        stat_id_1 INTEGER NOT NULL,
        stat_id_2 INTEGER NOT NULL,
        stat_id_3 INTEGER NOT NULL,
        stat_id_4 INTEGER NOT NULL,
        stat_id_5 INTEGER NOT NULL,
        stat_id_6 INTEGER NOT NULL,
        stat_id_7 INTEGER NOT NULL,
        stat_id_8 INTEGER NOT NULL,
        stat_id_9 INTEGER NOT NULL,
        bonus_0 INTEGER NOT NULL,
        bonus_1 INTEGER NOT NULL,
        bonus_2 INTEGER NOT NULL,
        bonus_3 INTEGER NOT NULL,
        bonus_4 INTEGER NOT NULL,
        bonus_5 INTEGER NOT NULL,
        bonus_6 INTEGER NOT NULL,
        bonus_7 INTEGER NOT NULL,
        bonus_8 INTEGER NOT NULL,
        bonus_9 INTEGER NOT NULL,
        maxlevel INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ScalingStatDistribution (
        id,
        stat_id_0,
        stat_id_1,
        stat_id_2,
        stat_id_3,
        stat_id_4,
        stat_id_5,
        stat_id_6,
        stat_id_7,
        stat_id_8,
        stat_id_9,
        bonus_0,
        bonus_1,
        bonus_2,
        bonus_3,
        bonus_4,
        bonus_5,
        bonus_6,
        bonus_7,
        bonus_8,
        bonus_9,
        maxlevel
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DeclinedWord() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DeclinedWord (
        id INTEGER PRIMARY KEY NOT NULL,
        word TEXT  NOT NULL
    );"
    ,
    "INSERT INTO DeclinedWord (
        id,
        word
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundProviderPreferences() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundProviderPreferences (
        id INTEGER PRIMARY KEY NOT NULL,
        description TEXT  NOT NULL,
        flags INTEGER  NOT NULL,
        e_a_x_environment_selection INTEGER  NOT NULL,
        e_a_x_decay_time REAL  NOT NULL,
        e_a_x2_environment_size REAL  NOT NULL,
        e_a_x2_environment_diffusion REAL  NOT NULL,
        e_a_x2_room INTEGER  NOT NULL,
        e_a_x2_room_h_f INTEGER  NOT NULL,
        e_a_x2_decay_h_f_ratio REAL  NOT NULL,
        e_a_x2_reflections INTEGER  NOT NULL,
        e_a_x2_reflections_delay REAL  NOT NULL,
        e_a_x2_reverb INTEGER  NOT NULL,
        e_a_x2_reverb_delay REAL  NOT NULL,
        e_a_x2_room_rolloff REAL  NOT NULL,
        e_a_x2_air_absorption REAL  NOT NULL,
        e_a_x3_room_l_f INTEGER  NOT NULL,
        e_a_x3_decay_l_f_ratio REAL  NOT NULL,
        e_a_x3_echo_time REAL  NOT NULL,
        e_a_x3_echo_depth REAL  NOT NULL,
        e_a_x3_modulation_time REAL  NOT NULL,
        e_a_x3_modulation_depth REAL  NOT NULL,
        e_a_x3_h_f_reference REAL  NOT NULL,
        e_a_x3_l_f_reference REAL  NOT NULL
    );"
    ,
    "INSERT INTO SoundProviderPreferences (
        id,
        description,
        flags,
        e_a_x_environment_selection,
        e_a_x_decay_time,
        e_a_x2_environment_size,
        e_a_x2_environment_diffusion,
        e_a_x2_room,
        e_a_x2_room_h_f,
        e_a_x2_decay_h_f_ratio,
        e_a_x2_reflections,
        e_a_x2_reflections_delay,
        e_a_x2_reverb,
        e_a_x2_reverb_delay,
        e_a_x2_room_rolloff,
        e_a_x2_air_absorption,
        e_a_x3_room_l_f,
        e_a_x3_decay_l_f_ratio,
        e_a_x3_echo_time,
        e_a_x3_echo_depth,
        e_a_x3_modulation_time,
        e_a_x3_modulation_depth,
        e_a_x3_h_f_reference,
        e_a_x3_l_f_reference
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemPurchaseGroup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemPurchaseGroup (
        id INTEGER PRIMARY KEY NOT NULL,
        item_id_0 INTEGER NOT NULL,
        item_id_1 INTEGER NOT NULL,
        item_id_2 INTEGER NOT NULL,
        item_id_3 INTEGER NOT NULL,
        item_id_4 INTEGER NOT NULL,
        item_id_5 INTEGER NOT NULL,
        item_id_6 INTEGER NOT NULL,
        item_id_7 INTEGER NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemPurchaseGroup (
        id,
        item_id_0,
        item_id_1,
        item_id_2,
        item_id_3,
        item_id_4,
        item_id_5,
        item_id_6,
        item_id_7,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ChrRaces() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChrRaces (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        faction_id INTEGER  NOT NULL,
        exploration_sound_id INTEGER  NOT NULL,
        male_display_id INTEGER  NOT NULL,
        female_display_id INTEGER  NOT NULL,
        client_prefix TEXT  NOT NULL,
        base_language INTEGER  NOT NULL,
        creature_type INTEGER  NOT NULL,
        res_sickness_spell_id INTEGER  NOT NULL,
        splash_sound_id INTEGER  NOT NULL,
        client_file_string TEXT  NOT NULL,
        cinematic_sequence_id INTEGER  NOT NULL,
        alliance INTEGER  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        name_female_lang_en_gb TEXT NOT NULL,
        name_female_lang_ko_kr TEXT NOT NULL,
        name_female_lang_fr_fr TEXT NOT NULL,
        name_female_lang_de_de TEXT NOT NULL,
        name_female_lang_en_cn TEXT NOT NULL,
        name_female_lang_en_tw TEXT NOT NULL,
        name_female_lang_es_es TEXT NOT NULL,
        name_female_lang_es_mx TEXT NOT NULL,
        name_female_lang_ru_ru TEXT NOT NULL,
        name_female_lang_ja_jp TEXT NOT NULL,
        name_female_lang_pt_pt TEXT NOT NULL,
        name_female_lang_it_it TEXT NOT NULL,
        name_female_lang_unknown_12 TEXT NOT NULL,
        name_female_lang_unknown_13 TEXT NOT NULL,
        name_female_lang_unknown_14 TEXT NOT NULL,
        name_female_lang_unknown_15 TEXT NOT NULL,
        name_female_lang_flags TEXT NOT NULL,
        name_male_lang_en_gb TEXT NOT NULL,
        name_male_lang_ko_kr TEXT NOT NULL,
        name_male_lang_fr_fr TEXT NOT NULL,
        name_male_lang_de_de TEXT NOT NULL,
        name_male_lang_en_cn TEXT NOT NULL,
        name_male_lang_en_tw TEXT NOT NULL,
        name_male_lang_es_es TEXT NOT NULL,
        name_male_lang_es_mx TEXT NOT NULL,
        name_male_lang_ru_ru TEXT NOT NULL,
        name_male_lang_ja_jp TEXT NOT NULL,
        name_male_lang_pt_pt TEXT NOT NULL,
        name_male_lang_it_it TEXT NOT NULL,
        name_male_lang_unknown_12 TEXT NOT NULL,
        name_male_lang_unknown_13 TEXT NOT NULL,
        name_male_lang_unknown_14 TEXT NOT NULL,
        name_male_lang_unknown_15 TEXT NOT NULL,
        name_male_lang_flags TEXT NOT NULL,
        facial_hair_customization_0 TEXT NOT NULL,
        facial_hair_customization_1 TEXT NOT NULL,
        hair_customization TEXT  NOT NULL,
        required_expansion INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ChrRaces (
        id,
        flags,
        faction_id,
        exploration_sound_id,
        male_display_id,
        female_display_id,
        client_prefix,
        base_language,
        creature_type,
        res_sickness_spell_id,
        splash_sound_id,
        client_file_string,
        cinematic_sequence_id,
        alliance,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        name_female_lang_en_gb,
        name_female_lang_ko_kr,
        name_female_lang_fr_fr,
        name_female_lang_de_de,
        name_female_lang_en_cn,
        name_female_lang_en_tw,
        name_female_lang_es_es,
        name_female_lang_es_mx,
        name_female_lang_ru_ru,
        name_female_lang_ja_jp,
        name_female_lang_pt_pt,
        name_female_lang_it_it,
        name_female_lang_unknown_12,
        name_female_lang_unknown_13,
        name_female_lang_unknown_14,
        name_female_lang_unknown_15,
        name_female_lang_flags,
        name_male_lang_en_gb,
        name_male_lang_ko_kr,
        name_male_lang_fr_fr,
        name_male_lang_de_de,
        name_male_lang_en_cn,
        name_male_lang_en_tw,
        name_male_lang_es_es,
        name_male_lang_es_mx,
        name_male_lang_ru_ru,
        name_male_lang_ja_jp,
        name_male_lang_pt_pt,
        name_male_lang_it_it,
        name_male_lang_unknown_12,
        name_male_lang_unknown_13,
        name_male_lang_unknown_14,
        name_male_lang_unknown_15,
        name_male_lang_flags,
        facial_hair_customization_0,
        facial_hair_customization_1,
        hair_customization,
        required_expansion
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57,
        ?58,
        ?59,
        ?60,
        ?61,
        ?62,
        ?63,
        ?64,
        ?65,
        ?66,
        ?67,
        ?68,
        ?69
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SheatheSoundLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SheatheSoundLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        class_id INTEGER  NOT NULL,
        subclass_id INTEGER  NOT NULL,
        material INTEGER  NOT NULL,
        check_material INTEGER  NOT NULL,
        sheathe_sound INTEGER  NOT NULL,
        unsheathe_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SheatheSoundLookups (
        id,
        class_id,
        subclass_id,
        material,
        check_material,
        sheathe_sound,
        unsheathe_sound
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn UnitBloodLevels() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS UnitBloodLevels (
        id INTEGER PRIMARY KEY NOT NULL,
        violencelevel_0 INTEGER NOT NULL,
        violencelevel_1 INTEGER NOT NULL,
        violencelevel_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO UnitBloodLevels (
        id,
        violencelevel_0,
        violencelevel_1,
        violencelevel_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellIcon() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellIcon (
        id INTEGER PRIMARY KEY NOT NULL,
        texture_filename TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellIcon (
        id,
        texture_filename
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LFGDungeons() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LFGDungeons (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        target_level INTEGER  NOT NULL,
        target_level_min INTEGER  NOT NULL,
        target_level_max INTEGER  NOT NULL,
        map_id INTEGER  NOT NULL,
        difficulty INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        type_id INTEGER  NOT NULL,
        faction INTEGER  NOT NULL,
        texture_filename TEXT  NOT NULL,
        expansion_level INTEGER  NOT NULL,
        order_index INTEGER  NOT NULL,
        group_id INTEGER  NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO LFGDungeons (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        min_level,
        max_level,
        target_level,
        target_level_min,
        target_level_max,
        map_id,
        difficulty,
        flags,
        type_id,
        faction,
        texture_filename,
        expansion_level,
        order_index,
        group_id,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundFilter() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundFilter (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SoundFilter (
        id,
        name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn StringLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS StringLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        string TEXT  NOT NULL
    );"
    ,
    "INSERT INTO StringLookups (
        id,
        string
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn VocalUISounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS VocalUISounds (
        id INTEGER PRIMARY KEY NOT NULL,
        vocal_u_i_enum INTEGER  NOT NULL,
        race_id INTEGER  NOT NULL,
        normal_sound_id_0 INTEGER NOT NULL,
        normal_sound_id_1 INTEGER NOT NULL,
        pissed_sound_id_0 INTEGER NOT NULL,
        pissed_sound_id_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO VocalUISounds (
        id,
        vocal_u_i_enum,
        race_id,
        normal_sound_id_0,
        normal_sound_id_1,
        pissed_sound_id_0,
        pissed_sound_id_1
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LFGDungeonExpansion() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LFGDungeonExpansion (
        id INTEGER PRIMARY KEY NOT NULL,
        lfg_id INTEGER  NOT NULL,
        expansion_level INTEGER  NOT NULL,
        random_id INTEGER  NOT NULL,
        hard_level_min INTEGER  NOT NULL,
        hard_level_max INTEGER  NOT NULL,
        target_level_min INTEGER  NOT NULL,
        target_level_max INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LFGDungeonExpansion (
        id,
        lfg_id,
        expansion_level,
        random_id,
        hard_level_min,
        hard_level_max,
        target_level_min,
        target_level_max
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillRaceClassInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillRaceClassInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        skill_id INTEGER  NOT NULL,
        race_mask INTEGER  NOT NULL,
        class_mask INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        min_level INTEGER  NOT NULL,
        skill_tier_id INTEGER  NOT NULL,
        skill_cost_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SkillRaceClassInfo (
        id,
        skill_id,
        race_mask,
        class_mask,
        flags,
        min_level,
        skill_tier_id,
        skill_cost_index
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AreaTrigger() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaTrigger (
        id INTEGER PRIMARY KEY NOT NULL,
        continent_id INTEGER  NOT NULL,
        pos_0 REAL NOT NULL,
        pos_1 REAL NOT NULL,
        pos_2 REAL NOT NULL,
        radius REAL  NOT NULL,
        box_length REAL  NOT NULL,
        box_width REAL  NOT NULL,
        box_height REAL  NOT NULL,
        box_yaw REAL  NOT NULL
    );"
    ,
    "INSERT INTO AreaTrigger (
        id,
        continent_id,
        pos_0,
        pos_1,
        pos_2,
        radius,
        box_length,
        box_width,
        box_height,
        box_yaw
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LightParams() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LightParams (
        id INTEGER PRIMARY KEY NOT NULL,
        highlight_sky INTEGER  NOT NULL,
        light_skybox_id INTEGER  NOT NULL,
        glow REAL  NOT NULL,
        water_shallow_alpha REAL  NOT NULL,
        water_deep_alpha REAL  NOT NULL,
        ocean_shallow_alpha REAL  NOT NULL,
        ocean_deep_alpha REAL  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LightParams (
        id,
        highlight_sky,
        light_skybox_id,
        glow,
        water_shallow_alpha,
        water_deep_alpha,
        ocean_shallow_alpha,
        ocean_deep_alpha,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellVisualPrecastTransitions() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualPrecastTransitions (
        id INTEGER PRIMARY KEY NOT NULL,
        precast_load_anim_name TEXT  NOT NULL,
        precast_hold_anim_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualPrecastTransitions (
        id,
        precast_load_anim_name,
        precast_hold_anim_name
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NamesProfanity() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NamesProfanity (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        language INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO NamesProfanity (
        id,
        name,
        language
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Emotes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Emotes (
        id INTEGER PRIMARY KEY NOT NULL,
        emote_slash_command TEXT  NOT NULL,
        anim_id INTEGER  NOT NULL,
        emote_flags INTEGER  NOT NULL,
        emote_spec_proc INTEGER  NOT NULL,
        emote_spec_proc_param INTEGER  NOT NULL,
        event_sound_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Emotes (
        id,
        emote_slash_command,
        anim_id,
        emote_flags,
        emote_spec_proc,
        emote_spec_proc_param,
        event_sound_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CreatureType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureType (
        id INTEGER PRIMARY KEY NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CreatureType (
        id,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn BarberShopStyle() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS BarberShopStyle (
        id INTEGER PRIMARY KEY NOT NULL,
        ty INTEGER  NOT NULL,
        display_name_lang_en_gb TEXT NOT NULL,
        display_name_lang_ko_kr TEXT NOT NULL,
        display_name_lang_fr_fr TEXT NOT NULL,
        display_name_lang_de_de TEXT NOT NULL,
        display_name_lang_en_cn TEXT NOT NULL,
        display_name_lang_en_tw TEXT NOT NULL,
        display_name_lang_es_es TEXT NOT NULL,
        display_name_lang_es_mx TEXT NOT NULL,
        display_name_lang_ru_ru TEXT NOT NULL,
        display_name_lang_ja_jp TEXT NOT NULL,
        display_name_lang_pt_pt TEXT NOT NULL,
        display_name_lang_it_it TEXT NOT NULL,
        display_name_lang_unknown_12 TEXT NOT NULL,
        display_name_lang_unknown_13 TEXT NOT NULL,
        display_name_lang_unknown_14 TEXT NOT NULL,
        display_name_lang_unknown_15 TEXT NOT NULL,
        display_name_lang_flags TEXT NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL,
        cost_modifier REAL  NOT NULL,
        race INTEGER  NOT NULL,
        sex INTEGER  NOT NULL,
        data INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO BarberShopStyle (
        id,
        ty,
        display_name_lang_en_gb,
        display_name_lang_ko_kr,
        display_name_lang_fr_fr,
        display_name_lang_de_de,
        display_name_lang_en_cn,
        display_name_lang_en_tw,
        display_name_lang_es_es,
        display_name_lang_es_mx,
        display_name_lang_ru_ru,
        display_name_lang_ja_jp,
        display_name_lang_pt_pt,
        display_name_lang_it_it,
        display_name_lang_unknown_12,
        display_name_lang_unknown_13,
        display_name_lang_unknown_14,
        display_name_lang_unknown_15,
        display_name_lang_flags,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags,
        cost_modifier,
        race,
        sex,
        data
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn DungeonMapChunk() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DungeonMapChunk (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        w_m_o_group_id INTEGER  NOT NULL,
        dungeon_map_id INTEGER  NOT NULL,
        min_z REAL  NOT NULL
    );"
    ,
    "INSERT INTO DungeonMapChunk (
        id,
        map_id,
        w_m_o_group_id,
        dungeon_map_id,
        min_z
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn gtCombatRatings() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS gtCombatRatings (
        data REAL  NOT NULL
    );"
    ,
    "INSERT INTO gtCombatRatings (
        data
        ) VALUES (
        ?1
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Faction() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Faction (
        id INTEGER PRIMARY KEY NOT NULL,
        reputation_index INTEGER  NOT NULL,
        reputation_race_mask_0 INTEGER NOT NULL,
        reputation_race_mask_1 INTEGER NOT NULL,
        reputation_race_mask_2 INTEGER NOT NULL,
        reputation_race_mask_3 INTEGER NOT NULL,
        reputation_class_mask_0 INTEGER NOT NULL,
        reputation_class_mask_1 INTEGER NOT NULL,
        reputation_class_mask_2 INTEGER NOT NULL,
        reputation_class_mask_3 INTEGER NOT NULL,
        reputation_base_0 INTEGER NOT NULL,
        reputation_base_1 INTEGER NOT NULL,
        reputation_base_2 INTEGER NOT NULL,
        reputation_base_3 INTEGER NOT NULL,
        reputation_flags_0 INTEGER NOT NULL,
        reputation_flags_1 INTEGER NOT NULL,
        reputation_flags_2 INTEGER NOT NULL,
        reputation_flags_3 INTEGER NOT NULL,
        parent_faction_id INTEGER  NOT NULL,
        parent_faction_mod_0 REAL NOT NULL,
        parent_faction_mod_1 REAL NOT NULL,
        parent_faction_cap_0 INTEGER NOT NULL,
        parent_faction_cap_1 INTEGER NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        description_lang_en_gb TEXT NOT NULL,
        description_lang_ko_kr TEXT NOT NULL,
        description_lang_fr_fr TEXT NOT NULL,
        description_lang_de_de TEXT NOT NULL,
        description_lang_en_cn TEXT NOT NULL,
        description_lang_en_tw TEXT NOT NULL,
        description_lang_es_es TEXT NOT NULL,
        description_lang_es_mx TEXT NOT NULL,
        description_lang_ru_ru TEXT NOT NULL,
        description_lang_ja_jp TEXT NOT NULL,
        description_lang_pt_pt TEXT NOT NULL,
        description_lang_it_it TEXT NOT NULL,
        description_lang_unknown_12 TEXT NOT NULL,
        description_lang_unknown_13 TEXT NOT NULL,
        description_lang_unknown_14 TEXT NOT NULL,
        description_lang_unknown_15 TEXT NOT NULL,
        description_lang_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Faction (
        id,
        reputation_index,
        reputation_race_mask_0,
        reputation_race_mask_1,
        reputation_race_mask_2,
        reputation_race_mask_3,
        reputation_class_mask_0,
        reputation_class_mask_1,
        reputation_class_mask_2,
        reputation_class_mask_3,
        reputation_base_0,
        reputation_base_1,
        reputation_base_2,
        reputation_base_3,
        reputation_flags_0,
        reputation_flags_1,
        reputation_flags_2,
        reputation_flags_3,
        parent_faction_id,
        parent_faction_mod_0,
        parent_faction_mod_1,
        parent_faction_cap_0,
        parent_faction_cap_1,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        description_lang_en_gb,
        description_lang_ko_kr,
        description_lang_fr_fr,
        description_lang_de_de,
        description_lang_en_cn,
        description_lang_en_tw,
        description_lang_es_es,
        description_lang_es_mx,
        description_lang_ru_ru,
        description_lang_ja_jp,
        description_lang_pt_pt,
        description_lang_it_it,
        description_lang_unknown_12,
        description_lang_unknown_13,
        description_lang_unknown_14,
        description_lang_unknown_15,
        description_lang_flags
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34,
        ?35,
        ?36,
        ?37,
        ?38,
        ?39,
        ?40,
        ?41,
        ?42,
        ?43,
        ?44,
        ?45,
        ?46,
        ?47,
        ?48,
        ?49,
        ?50,
        ?51,
        ?52,
        ?53,
        ?54,
        ?55,
        ?56,
        ?57
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Exhaustion() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Exhaustion (
        id INTEGER PRIMARY KEY NOT NULL,
        xp INTEGER  NOT NULL,
        factor REAL  NOT NULL,
        outdoor_hours REAL  NOT NULL,
        inn_hours REAL  NOT NULL,
        name_lang_en_gb TEXT NOT NULL,
        name_lang_ko_kr TEXT NOT NULL,
        name_lang_fr_fr TEXT NOT NULL,
        name_lang_de_de TEXT NOT NULL,
        name_lang_en_cn TEXT NOT NULL,
        name_lang_en_tw TEXT NOT NULL,
        name_lang_es_es TEXT NOT NULL,
        name_lang_es_mx TEXT NOT NULL,
        name_lang_ru_ru TEXT NOT NULL,
        name_lang_ja_jp TEXT NOT NULL,
        name_lang_pt_pt TEXT NOT NULL,
        name_lang_it_it TEXT NOT NULL,
        name_lang_unknown_12 TEXT NOT NULL,
        name_lang_unknown_13 TEXT NOT NULL,
        name_lang_unknown_14 TEXT NOT NULL,
        name_lang_unknown_15 TEXT NOT NULL,
        name_lang_flags TEXT NOT NULL,
        threshold REAL  NOT NULL
    );"
    ,
    "INSERT INTO Exhaustion (
        id,
        xp,
        factor,
        outdoor_hours,
        inn_hours,
        name_lang_en_gb,
        name_lang_ko_kr,
        name_lang_fr_fr,
        name_lang_de_de,
        name_lang_en_cn,
        name_lang_en_tw,
        name_lang_es_es,
        name_lang_es_mx,
        name_lang_ru_ru,
        name_lang_ja_jp,
        name_lang_pt_pt,
        name_lang_it_it,
        name_lang_unknown_12,
        name_lang_unknown_13,
        name_lang_unknown_14,
        name_lang_unknown_15,
        name_lang_flags,
        threshold
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WeaponSwingSounds2() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WeaponSwingSounds2 (
        id INTEGER PRIMARY KEY NOT NULL,
        swing_type INTEGER  NOT NULL,
        crit INTEGER  NOT NULL,
        sound_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WeaponSwingSounds2 (
        id,
        swing_type,
        crit,
        sound_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundAmbience() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundAmbience (
        id INTEGER PRIMARY KEY NOT NULL,
        ambience_id_0 INTEGER NOT NULL,
        ambience_id_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SoundAmbience (
        id,
        ambience_id_0,
        ambience_id_1
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LightFloatBand() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LightFloatBand (
        id INTEGER PRIMARY KEY NOT NULL,
        num INTEGER  NOT NULL,
        time_0 INTEGER NOT NULL,
        time_1 INTEGER NOT NULL,
        time_2 INTEGER NOT NULL,
        time_3 INTEGER NOT NULL,
        time_4 INTEGER NOT NULL,
        time_5 INTEGER NOT NULL,
        time_6 INTEGER NOT NULL,
        time_7 INTEGER NOT NULL,
        time_8 INTEGER NOT NULL,
        time_9 INTEGER NOT NULL,
        time_10 INTEGER NOT NULL,
        time_11 INTEGER NOT NULL,
        time_12 INTEGER NOT NULL,
        time_13 INTEGER NOT NULL,
        time_14 INTEGER NOT NULL,
        time_15 INTEGER NOT NULL,
        data_0 REAL NOT NULL,
        data_1 REAL NOT NULL,
        data_2 REAL NOT NULL,
        data_3 REAL NOT NULL,
        data_4 REAL NOT NULL,
        data_5 REAL NOT NULL,
        data_6 REAL NOT NULL,
        data_7 REAL NOT NULL,
        data_8 REAL NOT NULL,
        data_9 REAL NOT NULL,
        data_10 REAL NOT NULL,
        data_11 REAL NOT NULL,
        data_12 REAL NOT NULL,
        data_13 REAL NOT NULL,
        data_14 REAL NOT NULL,
        data_15 REAL NOT NULL
    );"
    ,
    "INSERT INTO LightFloatBand (
        id,
        num,
        time_0,
        time_1,
        time_2,
        time_3,
        time_4,
        time_5,
        time_6,
        time_7,
        time_8,
        time_9,
        time_10,
        time_11,
        time_12,
        time_13,
        time_14,
        time_15,
        data_0,
        data_1,
        data_2,
        data_3,
        data_4,
        data_5,
        data_6,
        data_7,
        data_8,
        data_9,
        data_10,
        data_11,
        data_12,
        data_13,
        data_14,
        data_15
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7,
        ?8,
        ?9,
        ?10,
        ?11,
        ?12,
        ?13,
        ?14,
        ?15,
        ?16,
        ?17,
        ?18,
        ?19,
        ?20,
        ?21,
        ?22,
        ?23,
        ?24,
        ?25,
        ?26,
        ?27,
        ?28,
        ?29,
        ?30,
        ?31,
        ?32,
        ?33,
        ?34
    );"
    )
}

