use crate::SqliteError;
use rusqlite::{Connection, params};
use wow_dbc::{DbcTable, LocalizedString, ExtendedLocalizedString};
use wow_dbc::tbc_tables::*;

pub(crate) fn write_to_sqlite(conn: &mut Connection, file_name: &str, file_contents: &mut &[u8]) -> Result<(), SqliteError> {
    let tx = conn.transaction()?;

    match file_name {
        "AnimationData.dbc" => {
            let data = animation_data::AnimationData::read(file_contents)?;
            let (table, insert, _select) = AnimationData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.weaponflags,
                row.bodyflags,
                row.field_0_7_0_3694_004,
                row.flags,
                row.fallback.id,
                ])?;
            }
        }
        "AreaPOI.dbc" => {
            let data = area_poi::AreaPOI::read(file_contents)?;
            let (table, insert, _select) = AreaPOI();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.importance,
                row.icon,
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
                ])?;
            }
        }
        "AreaTable.dbc" => {
            let data = area_table::AreaTable::read(file_contents)?;
            let (table, insert, _select) = AreaTable();
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
                ])?;
            }
        }
        "AreaTrigger.dbc" => {
            let data = area_trigger::AreaTrigger::read(file_contents)?;
            let (table, insert, _select) = AreaTrigger();
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
        }
        "AttackAnimKits.dbc" => {
            let data = attack_anim_kits::AttackAnimKits::read(file_contents)?;
            let (table, insert, _select) = AttackAnimKits();
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
        }
        "AttackAnimTypes.dbc" => {
            let data = attack_anim_types::AttackAnimTypes::read(file_contents)?;
            let (table, insert, _select) = AttackAnimTypes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.anim_id,
                &row.anim_name,
                ])?;
            }
        }
        "AuctionHouse.dbc" => {
            let data = auction_house::AuctionHouse::read(file_contents)?;
            let (table, insert, _select) = AuctionHouse();
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
        }
        "BankBagSlotPrices.dbc" => {
            let data = bank_bag_slot_prices::BankBagSlotPrices::read(file_contents)?;
            let (table, insert, _select) = BankBagSlotPrices();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cost,
                ])?;
            }
        }
        "BattlemasterList.dbc" => {
            let data = battlemaster_list::BattlemasterList::read(file_contents)?;
            let (table, insert, _select) = BattlemasterList();
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
                row.min_level,
                row.max_level,
                row.field_2_0_0_5610_005,
                row.field_2_0_0_5610_006,
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
                row.field_2_4_0_8089_009,
                ])?;
            }
        }
        "CameraShakes.dbc" => {
            let data = camera_shakes::CameraShakes::read(file_contents)?;
            let (table, insert, _select) = CameraShakes();
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
        }
        "Cfg_Categories.dbc" => {
            let data = cfg_categories::Cfg_Categories::read(file_contents)?;
            let (table, insert, _select) = Cfg_Categories();
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
        }
        "Cfg_Configs.dbc" => {
            let data = cfg_configs::Cfg_Configs::read(file_contents)?;
            let (table, insert, _select) = Cfg_Configs();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.realm_type,
                row.player_killing_allowed,
                row.roleplaying,
                ])?;
            }
        }
        "CharBaseInfo.dbc" => {
            let data = char_base_info::CharBaseInfo::read(file_contents)?;
            let (table, insert, _select) = CharBaseInfo();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.race_id.id,
                row.class_id.id,
                ])?;
            }
        }
        "CharHairGeosets.dbc" => {
            let data = char_hair_geosets::CharHairGeosets::read(file_contents)?;
            let (table, insert, _select) = CharHairGeosets();
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
        }
        "CharHairTextures.dbc" => {
            let data = char_hair_textures::CharHairTextures::read(file_contents)?;
            let (table, insert, _select) = CharHairTextures();
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
        }
        "CharSections.dbc" => {
            let data = char_sections::CharSections::read(file_contents)?;
            let (table, insert, _select) = CharSections();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race_id.id,
                row.sex_id,
                row.base_section,
                row.variation_index,
                row.color_index,
                row.texture_name[0],
                row.texture_name[1],
                row.texture_name[2],
                row.flags,
                ])?;
            }
        }
        "CharStartOutfit.dbc" => {
            let data = char_start_outfit::CharStartOutfit::read(file_contents)?;
            let (table, insert, _select) = CharStartOutfit();
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
                ])?;
            }
        }
        "CharTitles.dbc" => {
            let data = char_titles::CharTitles::read(file_contents)?;
            let (table, insert, _select) = CharTitles();
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
        }
        "CharVariations.dbc" => {
            let data = char_variations::CharVariations::read(file_contents)?;
            let (table, insert, _select) = CharVariations();
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
        }
        "CharacterFacialHairStyles.dbc" => {
            let data = character_facial_hair_styles::CharacterFacialHairStyles::read(file_contents)?;
            let (table, insert, _select) = CharacterFacialHairStyles();
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
                row.geoset[5],
                row.geoset[6],
                row.geoset[7],
                ])?;
            }
        }
        "ChatChannels.dbc" => {
            let data = chat_channels::ChatChannels::read(file_contents)?;
            let (table, insert, _select) = ChatChannels();
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
        }
        "ChatProfanity.dbc" => {
            let data = chat_profanity::ChatProfanity::read(file_contents)?;
            let (table, insert, _select) = ChatProfanity();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text,
                row.language,
                ])?;
            }
        }
        "ChrClasses.dbc" => {
            let data = chr_classes::ChrClasses::read(file_contents)?;
            let (table, insert, _select) = ChrClasses();
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
                ])?;
            }
        }
        "ChrRaces.dbc" => {
            let data = chr_races::ChrRaces::read(file_contents)?;
            let (table, insert, _select) = ChrRaces();
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
                row.mount_scale,
                row.base_language.id,
                row.creature_type.id,
                row.res_sickness_spell_id.id,
                row.splash_sound_id.id,
                &row.client_file_string,
                row.cinematic_sequence_id.id,
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
        }
        "CinematicCamera.dbc" => {
            let data = cinematic_camera::CinematicCamera::read(file_contents)?;
            let (table, insert, _select) = CinematicCamera();
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
        }
        "CinematicSequences.dbc" => {
            let data = cinematic_sequences::CinematicSequences::read(file_contents)?;
            let (table, insert, _select) = CinematicSequences();
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
        }
        "CreatureDisplayInfo.dbc" => {
            let data = creature_display_info::CreatureDisplayInfo::read(file_contents)?;
            let (table, insert, _select) = CreatureDisplayInfo();
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
                ])?;
            }
        }
        "CreatureDisplayInfoExtra.dbc" => {
            let data = creature_display_info_extra::CreatureDisplayInfoExtra::read(file_contents)?;
            let (table, insert, _select) = CreatureDisplayInfoExtra();
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
        }
        "CreatureFamily.dbc" => {
            let data = creature_family::CreatureFamily::read(file_contents)?;
            let (table, insert, _select) = CreatureFamily();
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
        }
        "CreatureModelData.dbc" => {
            let data = creature_model_data::CreatureModelData::read(file_contents)?;
            let (table, insert, _select) = CreatureModelData();
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
                row.geo_box[0],
                row.geo_box[1],
                row.geo_box[2],
                row.geo_box[3],
                row.geo_box[4],
                row.geo_box[5],
                row.attached_effect_scale,
                ])?;
            }
        }
        "CreatureSoundData.dbc" => {
            let data = creature_sound_data::CreatureSoundData::read(file_contents)?;
            let (table, insert, _select) = CreatureSoundData();
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
                ])?;
            }
        }
        "CreatureSpellData.dbc" => {
            let data = creature_spell_data::CreatureSpellData::read(file_contents)?;
            let (table, insert, _select) = CreatureSpellData();
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
        }
        "CreatureType.dbc" => {
            let data = creature_type::CreatureType::read(file_contents)?;
            let (table, insert, _select) = CreatureType();
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
        }
        "DeathThudLookups.dbc" => {
            let data = death_thud_lookups::DeathThudLookups::read(file_contents)?;
            let (table, insert, _select) = DeathThudLookups();
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
        }
        "DeclinedWord.dbc" => {
            let data = declined_word::DeclinedWord::read(file_contents)?;
            let (table, insert, _select) = DeclinedWord();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.word,
                ])?;
            }
        }
        "DeclinedWordCases.dbc" => {
            let data = declined_word_cases::DeclinedWordCases::read(file_contents)?;
            let (table, insert, _select) = DeclinedWordCases();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.declined_word_id.id,
                row.case_index,
                &row.declined_word,
                ])?;
            }
        }
        "DurabilityCosts.dbc" => {
            let data = durability_costs::DurabilityCosts::read(file_contents)?;
            let (table, insert, _select) = DurabilityCosts();
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
        }
        "DurabilityQuality.dbc" => {
            let data = durability_quality::DurabilityQuality::read(file_contents)?;
            let (table, insert, _select) = DurabilityQuality();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.data,
                ])?;
            }
        }
        "Emotes.dbc" => {
            let data = emotes::Emotes::read(file_contents)?;
            let (table, insert, _select) = Emotes();
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
        }
        "EmotesText.dbc" => {
            let data = emotes_text::EmotesText::read(file_contents)?;
            let (table, insert, _select) = EmotesText();
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
        }
        "EmotesTextData.dbc" => {
            let data = emotes_text_data::EmotesTextData::read(file_contents)?;
            let (table, insert, _select) = EmotesTextData();
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
        }
        "EmotesTextSound.dbc" => {
            let data = emotes_text_sound::EmotesTextSound::read(file_contents)?;
            let (table, insert, _select) = EmotesTextSound();
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
        }
        "EnvironmentalDamage.dbc" => {
            let data = environmental_damage::EnvironmentalDamage::read(file_contents)?;
            let (table, insert, _select) = EnvironmentalDamage();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.enum_id,
                row.visualkit_id.id,
                ])?;
            }
        }
        "Exhaustion.dbc" => {
            let data = exhaustion::Exhaustion::read(file_contents)?;
            let (table, insert, _select) = Exhaustion();
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
        }
        "Faction.dbc" => {
            let data = faction::Faction::read(file_contents)?;
            let (table, insert, _select) = Faction();
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
        }
        "FactionGroup.dbc" => {
            let data = faction_group::FactionGroup::read(file_contents)?;
            let (table, insert, _select) = FactionGroup();
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
        }
        "FactionTemplate.dbc" => {
            let data = faction_template::FactionTemplate::read(file_contents)?;
            let (table, insert, _select) = FactionTemplate();
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
        }
        "FootprintTextures.dbc" => {
            let data = footprint_textures::FootprintTextures::read(file_contents)?;
            let (table, insert, _select) = FootprintTextures();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.footstep_filename,
                ])?;
            }
        }
        "FootstepTerrainLookup.dbc" => {
            let data = footstep_terrain_lookup::FootstepTerrainLookup::read(file_contents)?;
            let (table, insert, _select) = FootstepTerrainLookup();
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
        }
        "GMSurveyCurrentSurvey.dbc" => {
            let data = gm_survey_current_survey::GMSurveyCurrentSurvey::read(file_contents)?;
            let (table, insert, _select) = GMSurveyCurrentSurvey();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.gm_survey_id,
                ])?;
            }
        }
        "GMSurveyQuestions.dbc" => {
            let data = gm_survey_questions::GMSurveyQuestions::read(file_contents)?;
            let (table, insert, _select) = GMSurveyQuestions();
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
        }
        "GMSurveySurveys.dbc" => {
            let data = gm_survey_surveys::GMSurveySurveys::read(file_contents)?;
            let (table, insert, _select) = GMSurveySurveys();
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
        }
        "GMTicketCategory.dbc" => {
            let data = gm_ticket_category::GMTicketCategory::read(file_contents)?;
            let (table, insert, _select) = GMTicketCategory();
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
        }
        "GameObjectArtKit.dbc" => {
            let data = game_object_art_kit::GameObjectArtKit::read(file_contents)?;
            let (table, insert, _select) = GameObjectArtKit();
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
        }
        "GameObjectDisplayInfo.dbc" => {
            let data = game_object_display_info::GameObjectDisplayInfo::read(file_contents)?;
            let (table, insert, _select) = GameObjectDisplayInfo();
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
                ])?;
            }
        }
        "GameTables.dbc" => {
            let data = game_tables::GameTables::read(file_contents)?;
            let (table, insert, _select) = GameTables();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                &row.name,
                row.num_rows,
                row.num_columns,
                ])?;
            }
        }
        "GameTips.dbc" => {
            let data = game_tips::GameTips::read(file_contents)?;
            let (table, insert, _select) = GameTips();
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
        }
        "GemProperties.dbc" => {
            let data = gem_properties::GemProperties::read(file_contents)?;
            let (table, insert, _select) = GemProperties();
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
        }
        "GroundEffectDoodad.dbc" => {
            let data = ground_effect_doodad::GroundEffectDoodad::read(file_contents)?;
            let (table, insert, _select) = GroundEffectDoodad();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.doodad_id_tag,
                &row.doodadpath,
                ])?;
            }
        }
        "GroundEffectTexture.dbc" => {
            let data = ground_effect_texture::GroundEffectTexture::read(file_contents)?;
            let (table, insert, _select) = GroundEffectTexture();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.doodad_id[0],
                row.doodad_id[1],
                row.doodad_id[2],
                row.doodad_id[3],
                row.density,
                row.sound,
                ])?;
            }
        }
        "HelmetGeosetVisData.dbc" => {
            let data = helmet_geoset_vis_data::HelmetGeosetVisData::read(file_contents)?;
            let (table, insert, _select) = HelmetGeosetVisData();
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
        }
        "Item.dbc" => {
            let data = item::Item::read(file_contents)?;
            let (table, insert, _select) = Item();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.display_info_id,
                row.inventory_type,
                row.sheathe_type,
                ])?;
            }
        }
        "ItemBagFamily.dbc" => {
            let data = item_bag_family::ItemBagFamily::read(file_contents)?;
            let (table, insert, _select) = ItemBagFamily();
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
        }
        "ItemClass.dbc" => {
            let data = item_class::ItemClass::read(file_contents)?;
            let (table, insert, _select) = ItemClass();
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
        }
        "ItemCondExtCosts.dbc" => {
            let data = item_cond_ext_costs::ItemCondExtCosts::read(file_contents)?;
            let (table, insert, _select) = ItemCondExtCosts();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cond_extended_cost,
                row.item_extended_cost_entry.id,
                row.arena_season,
                ])?;
            }
        }
        "ItemDisplayInfo.dbc" => {
            let data = item_display_info::ItemDisplayInfo::read(file_contents)?;
            let (table, insert, _select) = ItemDisplayInfo();
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
        }
        "ItemExtendedCost.dbc" => {
            let data = item_extended_cost::ItemExtendedCost::read(file_contents)?;
            let (table, insert, _select) = ItemExtendedCost();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.honor_points,
                row.arena_points,
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
                ])?;
            }
        }
        "ItemGroupSounds.dbc" => {
            let data = item_group_sounds::ItemGroupSounds::read(file_contents)?;
            let (table, insert, _select) = ItemGroupSounds();
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
        }
        "ItemPetFood.dbc" => {
            let data = item_pet_food::ItemPetFood::read(file_contents)?;
            let (table, insert, _select) = ItemPetFood();
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
        }
        "ItemRandomProperties.dbc" => {
            let data = item_random_properties::ItemRandomProperties::read(file_contents)?;
            let (table, insert, _select) = ItemRandomProperties();
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
        }
        "ItemRandomSuffix.dbc" => {
            let data = item_random_suffix::ItemRandomSuffix::read(file_contents)?;
            let (table, insert, _select) = ItemRandomSuffix();
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
                row.allocation_pct[0],
                row.allocation_pct[1],
                row.allocation_pct[2],
                ])?;
            }
        }
        "ItemSet.dbc" => {
            let data = item_set::ItemSet::read(file_contents)?;
            let (table, insert, _select) = ItemSet();
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
        }
        "ItemSubClass.dbc" => {
            let data = item_sub_class::ItemSubClass::read(file_contents)?;
            let (table, insert, _select) = ItemSubClass();
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
        }
        "ItemSubClassMask.dbc" => {
            let data = item_sub_class_mask::ItemSubClassMask::read(file_contents)?;
            let (table, insert, _select) = ItemSubClassMask();
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
        }
        "ItemVisualEffects.dbc" => {
            let data = item_visual_effects::ItemVisualEffects::read(file_contents)?;
            let (table, insert, _select) = ItemVisualEffects();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model,
                ])?;
            }
        }
        "ItemVisuals.dbc" => {
            let data = item_visuals::ItemVisuals::read(file_contents)?;
            let (table, insert, _select) = ItemVisuals();
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
        }
        "LFGDungeons.dbc" => {
            let data = lfg_dungeons::LFGDungeons::read(file_contents)?;
            let (table, insert, _select) = LFGDungeons();
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
                row.type_id,
                row.faction.id,
                &row.texture_filename,
                row.expansion_level,
                ])?;
            }
        }
        "LanguageWords.dbc" => {
            let data = language_words::LanguageWords::read(file_contents)?;
            let (table, insert, _select) = LanguageWords();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.language_id.id,
                &row.word,
                ])?;
            }
        }
        "Languages.dbc" => {
            let data = languages::Languages::read(file_contents)?;
            let (table, insert, _select) = Languages();
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
        }
        "Light.dbc" => {
            let data = light::Light::read(file_contents)?;
            let (table, insert, _select) = Light();
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
                ])?;
            }
        }
        "LightFloatBand.dbc" => {
            let data = light_float_band::LightFloatBand::read(file_contents)?;
            let (table, insert, _select) = LightFloatBand();
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
        }
        "LightIntBand.dbc" => {
            let data = light_int_band::LightIntBand::read(file_contents)?;
            let (table, insert, _select) = LightIntBand();
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
        }
        "LightParams.dbc" => {
            let data = light_params::LightParams::read(file_contents)?;
            let (table, insert, _select) = LightParams();
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
        }
        "LightSkybox.dbc" => {
            let data = light_skybox::LightSkybox::read(file_contents)?;
            let (table, insert, _select) = LightSkybox();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.flags,
                ])?;
            }
        }
        "LiquidType.dbc" => {
            let data = liquid_type::LiquidType::read(file_contents)?;
            let (table, insert, _select) = LiquidType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.flags,
                row.spell_id.id,
                ])?;
            }
        }
        "LoadingScreenTaxiSplines.dbc" => {
            let data = loading_screen_taxi_splines::LoadingScreenTaxiSplines::read(file_contents)?;
            let (table, insert, _select) = LoadingScreenTaxiSplines();
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
        }
        "LoadingScreens.dbc" => {
            let data = loading_screens::LoadingScreens::read(file_contents)?;
            let (table, insert, _select) = LoadingScreens();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.file_name,
                ])?;
            }
        }
        "Lock.dbc" => {
            let data = lock::Lock::read(file_contents)?;
            let (table, insert, _select) = Lock();
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
        }
        "LockType.dbc" => {
            let data = lock_type::LockType::read(file_contents)?;
            let (table, insert, _select) = LockType();
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
        }
        "MailTemplate.dbc" => {
            let data = mail_template::MailTemplate::read(file_contents)?;
            let (table, insert, _select) = MailTemplate();
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
        }
        "Map.dbc" => {
            let data = map::Map::read(file_contents)?;
            let (table, insert, _select) = Map();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.directory,
                row.instance_type,
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
                row.min_level,
                row.max_level,
                row.max_players,
                row.field_0_7_0_3694_006,
                row.field_0_7_0_3694_007,
                row.field_0_7_0_3694_008,
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
                row.field_1_5_0_4442_014,
                row.field_1_7_0_4671_015,
                row.minimap_icon_scale,
                &row.field_2_0_0_5610_018_lang.en_gb,
                &row.field_2_0_0_5610_018_lang.ko_kr,
                &row.field_2_0_0_5610_018_lang.fr_fr,
                &row.field_2_0_0_5610_018_lang.de_de,
                &row.field_2_0_0_5610_018_lang.en_cn,
                &row.field_2_0_0_5610_018_lang.en_tw,
                &row.field_2_0_0_5610_018_lang.es_es,
                &row.field_2_0_0_5610_018_lang.es_mx,
                &row.field_2_0_0_5610_018_lang.ru_ru,
                &row.field_2_0_0_5610_018_lang.ja_jp,
                &row.field_2_0_0_5610_018_lang.pt_pt,
                &row.field_2_0_0_5610_018_lang.it_it,
                &row.field_2_0_0_5610_018_lang.unknown_12,
                &row.field_2_0_0_5610_018_lang.unknown_13,
                &row.field_2_0_0_5610_018_lang.unknown_14,
                &row.field_2_0_0_5610_018_lang.unknown_15,
                &row.field_2_0_0_5610_018_lang.flags,
                &row.field_2_0_0_5610_019_lang.en_gb,
                &row.field_2_0_0_5610_019_lang.ko_kr,
                &row.field_2_0_0_5610_019_lang.fr_fr,
                &row.field_2_0_0_5610_019_lang.de_de,
                &row.field_2_0_0_5610_019_lang.en_cn,
                &row.field_2_0_0_5610_019_lang.en_tw,
                &row.field_2_0_0_5610_019_lang.es_es,
                &row.field_2_0_0_5610_019_lang.es_mx,
                &row.field_2_0_0_5610_019_lang.ru_ru,
                &row.field_2_0_0_5610_019_lang.ja_jp,
                &row.field_2_0_0_5610_019_lang.pt_pt,
                &row.field_2_0_0_5610_019_lang.it_it,
                &row.field_2_0_0_5610_019_lang.unknown_12,
                &row.field_2_0_0_5610_019_lang.unknown_13,
                &row.field_2_0_0_5610_019_lang.unknown_14,
                &row.field_2_0_0_5610_019_lang.unknown_15,
                &row.field_2_0_0_5610_019_lang.flags,
                &row.field_2_0_0_5610_020_lang.en_gb,
                &row.field_2_0_0_5610_020_lang.ko_kr,
                &row.field_2_0_0_5610_020_lang.fr_fr,
                &row.field_2_0_0_5610_020_lang.de_de,
                &row.field_2_0_0_5610_020_lang.en_cn,
                &row.field_2_0_0_5610_020_lang.en_tw,
                &row.field_2_0_0_5610_020_lang.es_es,
                &row.field_2_0_0_5610_020_lang.es_mx,
                &row.field_2_0_0_5610_020_lang.ru_ru,
                &row.field_2_0_0_5610_020_lang.ja_jp,
                &row.field_2_0_0_5610_020_lang.pt_pt,
                &row.field_2_0_0_5610_020_lang.it_it,
                &row.field_2_0_0_5610_020_lang.unknown_12,
                &row.field_2_0_0_5610_020_lang.unknown_13,
                &row.field_2_0_0_5610_020_lang.unknown_14,
                &row.field_2_0_0_5610_020_lang.unknown_15,
                &row.field_2_0_0_5610_020_lang.flags,
                row.corpse_map_id.id,
                row.corpse[0],
                row.corpse[1],
                row.field_2_0_3_6299_023,
                row.field_2_0_3_6299_024,
                row.field_2_0_3_6299_025,
                row.time_of_day_override,
                row.expansion_id,
                ])?;
            }
        }
        "Material.dbc" => {
            let data = material::Material::read(file_contents)?;
            let (table, insert, _select) = Material();
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
        }
        "NPCSounds.dbc" => {
            let data = npc_sounds::NPCSounds::read(file_contents)?;
            let (table, insert, _select) = NPCSounds();
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
        }
        "NameGen.dbc" => {
            let data = name_gen::NameGen::read(file_contents)?;
            let (table, insert, _select) = NameGen();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.race_id.id,
                row.sex,
                ])?;
            }
        }
        "NamesProfanity.dbc" => {
            let data = names_profanity::NamesProfanity::read(file_contents)?;
            let (table, insert, _select) = NamesProfanity();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.language,
                ])?;
            }
        }
        "NamesReserved.dbc" => {
            let data = names_reserved::NamesReserved::read(file_contents)?;
            let (table, insert, _select) = NamesReserved();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.language,
                ])?;
            }
        }
        "Package.dbc" => {
            let data = package::Package::read(file_contents)?;
            let (table, insert, _select) = Package();
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
        }
        "PageTextMaterial.dbc" => {
            let data = page_text_material::PageTextMaterial::read(file_contents)?;
            let (table, insert, _select) = PageTextMaterial();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
        }
        "PaperDollItemFrame.dbc" => {
            let data = paper_doll_item_frame::PaperDollItemFrame::read(file_contents)?;
            let (table, insert, _select) = PaperDollItemFrame();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                &row.item_button_name,
                &row.slot_icon,
                row.slot_number,
                ])?;
            }
        }
        "ParticleColor.dbc" => {
            let data = particle_color::ParticleColor::read(file_contents)?;
            let (table, insert, _select) = ParticleColor();
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
        }
        "PetLoyalty.dbc" => {
            let data = pet_loyalty::PetLoyalty::read(file_contents)?;
            let (table, insert, _select) = PetLoyalty();
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
        }
        "PetPersonality.dbc" => {
            let data = pet_personality::PetPersonality::read(file_contents)?;
            let (table, insert, _select) = PetPersonality();
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
                row.damage_modifier[0],
                row.damage_modifier[1],
                row.damage_modifier[2],
                ])?;
            }
        }
        "PetitionType.dbc" => {
            let data = petition_type::PetitionType::read(file_contents)?;
            let (table, insert, _select) = PetitionType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.ty,
                ])?;
            }
        }
        "QuestInfo.dbc" => {
            let data = quest_info::QuestInfo::read(file_contents)?;
            let (table, insert, _select) = QuestInfo();
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
        }
        "QuestSort.dbc" => {
            let data = quest_sort::QuestSort::read(file_contents)?;
            let (table, insert, _select) = QuestSort();
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
        }
        "RandPropPoints.dbc" => {
            let data = rand_prop_points::RandPropPoints::read(file_contents)?;
            let (table, insert, _select) = RandPropPoints();
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
        }
        "Resistances.dbc" => {
            let data = resistances::Resistances::read(file_contents)?;
            let (table, insert, _select) = Resistances();
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
        }
        "ServerMessages.dbc" => {
            let data = server_messages::ServerMessages::read(file_contents)?;
            let (table, insert, _select) = ServerMessages();
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
        }
        "SheatheSoundLookups.dbc" => {
            let data = sheathe_sound_lookups::SheatheSoundLookups::read(file_contents)?;
            let (table, insert, _select) = SheatheSoundLookups();
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
        }
        "SkillCostsData.dbc" => {
            let data = skill_costs_data::SkillCostsData::read(file_contents)?;
            let (table, insert, _select) = SkillCostsData();
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
        }
        "SkillLine.dbc" => {
            let data = skill_line::SkillLine::read(file_contents)?;
            let (table, insert, _select) = SkillLine();
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
                ])?;
            }
        }
        "SkillLineAbility.dbc" => {
            let data = skill_line_ability::SkillLineAbility::read(file_contents)?;
            let (table, insert, _select) = SkillLineAbility();
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
                row.abandonable,
                row.character_points[0],
                row.character_points[1],
                ])?;
            }
        }
        "SkillLineCategory.dbc" => {
            let data = skill_line_category::SkillLineCategory::read(file_contents)?;
            let (table, insert, _select) = SkillLineCategory();
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
        }
        "SkillRaceClassInfo.dbc" => {
            let data = skill_race_class_info::SkillRaceClassInfo::read(file_contents)?;
            let (table, insert, _select) = SkillRaceClassInfo();
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
        }
        "SkillTiers.dbc" => {
            let data = skill_tiers::SkillTiers::read(file_contents)?;
            let (table, insert, _select) = SkillTiers();
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
        }
        "SoundAmbience.dbc" => {
            let data = sound_ambience::SoundAmbience::read(file_contents)?;
            let (table, insert, _select) = SoundAmbience();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ambience_id[0],
                row.ambience_id[1],
                ])?;
            }
        }
        "SoundEntries.dbc" => {
            let data = sound_entries::SoundEntries::read(file_contents)?;
            let (table, insert, _select) = SoundEntries();
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
                ])?;
            }
        }
        "SoundProviderPreferences.dbc" => {
            let data = sound_provider_preferences::SoundProviderPreferences::read(file_contents)?;
            let (table, insert, _select) = SoundProviderPreferences();
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
        }
        "SoundSamplePreferences.dbc" => {
            let data = sound_sample_preferences::SoundSamplePreferences::read(file_contents)?;
            let (table, insert, _select) = SoundSamplePreferences();
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
        }
        "SoundWaterType.dbc" => {
            let data = sound_water_type::SoundWaterType::read(file_contents)?;
            let (table, insert, _select) = SoundWaterType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_type,
                row.sound_subtype,
                row.sound_id.id,
                ])?;
            }
        }
        "SpamMessages.dbc" => {
            let data = spam_messages::SpamMessages::read(file_contents)?;
            let (table, insert, _select) = SpamMessages();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text,
                ])?;
            }
        }
        "Spell.dbc" => {
            let data = spell::Spell::read(file_contents)?;
            let (table, insert, _select) = Spell();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.category.id,
                row.cast_u_i,
                row.dispel_type.id,
                row.mechanic.id,
                row.attributes,
                row.attributes_ex,
                row.attributes_ex_b,
                row.attributes_ex_c,
                row.attributes_ex_d,
                row.attributes_ex_e,
                row.attributes_ex_f,
                row.shapeshift_mask,
                row.shapeshift_exclude,
                row.targets,
                row.target_creature_type,
                row.requires_spell_focus.id,
                row.facing_caster_flags,
                row.caster_aura_state,
                row.target_aura_state,
                row.exclude_caster_aura_state,
                row.exclude_target_aura_state,
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
                row.effect_base_dice[0],
                row.effect_base_dice[1],
                row.effect_base_dice[2],
                row.effect_dice_per_level[0],
                row.effect_dice_per_level[1],
                row.effect_dice_per_level[2],
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
                row.effect_aura[0].as_int(),
                row.effect_aura[1].as_int(),
                row.effect_aura[2].as_int(),
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
                row.required_areas_id,
                row.school_mask,
                ])?;
            }
        }
        "SpellCastTimes.dbc" => {
            let data = spell_cast_times::SpellCastTimes::read(file_contents)?;
            let (table, insert, _select) = SpellCastTimes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.base,
                row.per_level,
                row.minimum,
                ])?;
            }
        }
        "SpellCategory.dbc" => {
            let data = spell_category::SpellCategory::read(file_contents)?;
            let (table, insert, _select) = SpellCategory();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                ])?;
            }
        }
        "SpellChainEffects.dbc" => {
            let data = spell_chain_effects::SpellChainEffects::read(file_contents)?;
            let (table, insert, _select) = SpellChainEffects();
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
                ])?;
            }
        }
        "SpellDispelType.dbc" => {
            let data = spell_dispel_type::SpellDispelType::read(file_contents)?;
            let (table, insert, _select) = SpellDispelType();
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
        }
        "SpellDuration.dbc" => {
            let data = spell_duration::SpellDuration::read(file_contents)?;
            let (table, insert, _select) = SpellDuration();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.duration,
                row.duration_per_level,
                row.max_duration,
                ])?;
            }
        }
        "SpellEffectCameraShakes.dbc" => {
            let data = spell_effect_camera_shakes::SpellEffectCameraShakes::read(file_contents)?;
            let (table, insert, _select) = SpellEffectCameraShakes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.camera_shake[0],
                row.camera_shake[1],
                row.camera_shake[2],
                ])?;
            }
        }
        "SpellFocusObject.dbc" => {
            let data = spell_focus_object::SpellFocusObject::read(file_contents)?;
            let (table, insert, _select) = SpellFocusObject();
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
        }
        "SpellIcon.dbc" => {
            let data = spell_icon::SpellIcon::read(file_contents)?;
            let (table, insert, _select) = SpellIcon();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.texture_filename,
                ])?;
            }
        }
        "SpellItemEnchantment.dbc" => {
            let data = spell_item_enchantment::SpellItemEnchantment::read(file_contents)?;
            let (table, insert, _select) = SpellItemEnchantment();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
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
                ])?;
            }
        }
        "SpellItemEnchantmentCondition.dbc" => {
            let data = spell_item_enchantment_condition::SpellItemEnchantmentCondition::read(file_contents)?;
            let (table, insert, _select) = SpellItemEnchantmentCondition();
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
        }
        "SpellMechanic.dbc" => {
            let data = spell_mechanic::SpellMechanic::read(file_contents)?;
            let (table, insert, _select) = SpellMechanic();
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
        }
        "SpellMissileMotion.dbc" => {
            let data = spell_missile_motion::SpellMissileMotion::read(file_contents)?;
            let (table, insert, _select) = SpellMissileMotion();
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
        }
        "SpellRadius.dbc" => {
            let data = spell_radius::SpellRadius::read(file_contents)?;
            let (table, insert, _select) = SpellRadius();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.radius,
                row.radius_per_level,
                row.radius_max,
                ])?;
            }
        }
        "SpellRange.dbc" => {
            let data = spell_range::SpellRange::read(file_contents)?;
            let (table, insert, _select) = SpellRange();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.range_min,
                row.range_max,
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
        }
        "SpellShapeshiftForm.dbc" => {
            let data = spell_shapeshift_form::SpellShapeshiftForm::read(file_contents)?;
            let (table, insert, _select) = SpellShapeshiftForm();
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
        }
        "SpellVisual.dbc" => {
            let data = spell_visual::SpellVisual::read(file_contents)?;
            let (table, insert, _select) = SpellVisual();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.precast_kit,
                row.cast_kit,
                row.impact_kit,
                row.state_kit,
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
                ])?;
            }
        }
        "SpellVisualEffectName.dbc" => {
            let data = spell_visual_effect_name::SpellVisualEffectName::read(file_contents)?;
            let (table, insert, _select) = SpellVisualEffectName();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.file_name,
                row.area_effect_size,
                row.scale,
                ])?;
            }
        }
        "SpellVisualKit.dbc" => {
            let data = spell_visual_kit::SpellVisualKit::read(file_contents)?;
            let (table, insert, _select) = SpellVisualKit();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.kit_type,
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
        }
        "SpellVisualPrecastTransitions.dbc" => {
            let data = spell_visual_precast_transitions::SpellVisualPrecastTransitions::read(file_contents)?;
            let (table, insert, _select) = SpellVisualPrecastTransitions();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.precast_load_anim_name,
                &row.precast_hold_anim_name,
                ])?;
            }
        }
        "StableSlotPrices.dbc" => {
            let data = stable_slot_prices::StableSlotPrices::read(file_contents)?;
            let (table, insert, _select) = StableSlotPrices();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cost,
                ])?;
            }
        }
        "Startup_Strings.dbc" => {
            let data = startup_strings::Startup_Strings::read(file_contents)?;
            let (table, insert, _select) = Startup_Strings();
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
        }
        "Stationery.dbc" => {
            let data = stationery::Stationery::read(file_contents)?;
            let (table, insert, _select) = Stationery();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_id.id,
                &row.texture,
                row.flags,
                ])?;
            }
        }
        "StringLookups.dbc" => {
            let data = string_lookups::StringLookups::read(file_contents)?;
            let (table, insert, _select) = StringLookups();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.string,
                ])?;
            }
        }
        "SummonProperties.dbc" => {
            let data = summon_properties::SummonProperties::read(file_contents)?;
            let (table, insert, _select) = SummonProperties();
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
        }
        "Talent.dbc" => {
            let data = talent::Talent::read(file_contents)?;
            let (table, insert, _select) = Talent();
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
                ])?;
            }
        }
        "TalentTab.dbc" => {
            let data = talent_tab::TalentTab::read(file_contents)?;
            let (table, insert, _select) = TalentTab();
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
                row.order_index,
                &row.background_file,
                ])?;
            }
        }
        "TaxiNodes.dbc" => {
            let data = taxi_nodes::TaxiNodes::read(file_contents)?;
            let (table, insert, _select) = TaxiNodes();
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
        }
        "TaxiPath.dbc" => {
            let data = taxi_path::TaxiPath::read(file_contents)?;
            let (table, insert, _select) = TaxiPath();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.from_taxi_node.id,
                row.to_taxi_node.id,
                row.cost,
                ])?;
            }
        }
        "TaxiPathNode.dbc" => {
            let data = taxi_path_node::TaxiPathNode::read(file_contents)?;
            let (table, insert, _select) = TaxiPathNode();
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
        }
        "TerrainType.dbc" => {
            let data = terrain_type::TerrainType::read(file_contents)?;
            let (table, insert, _select) = TerrainType();
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
        }
        "TerrainTypeSounds.dbc" => {
            let data = terrain_type_sounds::TerrainTypeSounds::read(file_contents)?;
            let (table, insert, _select) = TerrainTypeSounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                ])?;
            }
        }
        "TotemCategory.dbc" => {
            let data = totem_category::TotemCategory::read(file_contents)?;
            let (table, insert, _select) = TotemCategory();
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
        }
        "TransportAnimation.dbc" => {
            let data = transport_animation::TransportAnimation::read(file_contents)?;
            let (table, insert, _select) = TransportAnimation();
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
        }
        "TransportPhysics.dbc" => {
            let data = transport_physics::TransportPhysics::read(file_contents)?;
            let (table, insert, _select) = TransportPhysics();
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
        }
        "UISoundLookups.dbc" => {
            let data = ui_sound_lookups::UISoundLookups::read(file_contents)?;
            let (table, insert, _select) = UISoundLookups();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_id.id,
                &row.sound_name,
                ])?;
            }
        }
        "UnitBlood.dbc" => {
            let data = unit_blood::UnitBlood::read(file_contents)?;
            let (table, insert, _select) = UnitBlood();
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
        }
        "UnitBloodLevels.dbc" => {
            let data = unit_blood_levels::UnitBloodLevels::read(file_contents)?;
            let (table, insert, _select) = UnitBloodLevels();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.violencelevel[0],
                row.violencelevel[1],
                row.violencelevel[2],
                ])?;
            }
        }
        "VideoHardware.dbc" => {
            let data = video_hardware::VideoHardware::read(file_contents)?;
            let (table, insert, _select) = VideoHardware();
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
        }
        "VocalUISounds.dbc" => {
            let data = vocal_ui_sounds::VocalUISounds::read(file_contents)?;
            let (table, insert, _select) = VocalUISounds();
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
        }
        "WMOAreaTable.dbc" => {
            let data = wmo_area_table::WMOAreaTable::read(file_contents)?;
            let (table, insert, _select) = WMOAreaTable();
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
        }
        "WeaponImpactSounds.dbc" => {
            let data = weapon_impact_sounds::WeaponImpactSounds::read(file_contents)?;
            let (table, insert, _select) = WeaponImpactSounds();
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
        }
        "WeaponSwingSounds2.dbc" => {
            let data = weapon_swing_sounds2::WeaponSwingSounds2::read(file_contents)?;
            let (table, insert, _select) = WeaponSwingSounds2();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.swing_type,
                row.crit,
                row.sound_id.id,
                ])?;
            }
        }
        "Weather.dbc" => {
            let data = weather::Weather::read(file_contents)?;
            let (table, insert, _select) = Weather();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ambience_id.id,
                row.effect_type,
                row.effect_color[0],
                row.effect_color[1],
                row.effect_color[2],
                &row.effect_texture,
                ])?;
            }
        }
        "WorldMapArea.dbc" => {
            let data = world_map_area::WorldMapArea::read(file_contents)?;
            let (table, insert, _select) = WorldMapArea();
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
                ])?;
            }
        }
        "WorldMapContinent.dbc" => {
            let data = world_map_continent::WorldMapContinent::read(file_contents)?;
            let (table, insert, _select) = WorldMapContinent();
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
                ])?;
            }
        }
        "WorldMapOverlay.dbc" => {
            let data = world_map_overlay::WorldMapOverlay::read(file_contents)?;
            let (table, insert, _select) = WorldMapOverlay();
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
        }
        "WorldMapTransforms.dbc" => {
            let data = world_map_transforms::WorldMapTransforms::read(file_contents)?;
            let (table, insert, _select) = WorldMapTransforms();
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
                ])?;
            }
        }
        "WorldSafeLocs.dbc" => {
            let data = world_safe_locs::WorldSafeLocs::read(file_contents)?;
            let (table, insert, _select) = WorldSafeLocs();
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
        }
        "WorldStateUI.dbc" => {
            let data = world_state_ui::WorldStateUI::read(file_contents)?;
            let (table, insert, _select) = WorldStateUI();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map_id.id,
                row.area_id.id,
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
                row.faction_id.id,
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
        }
        "WorldStateZoneSounds.dbc" => {
            let data = world_state_zone_sounds::WorldStateZoneSounds::read(file_contents)?;
            let (table, insert, _select) = WorldStateZoneSounds();
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
        }
        "WowError_Strings.dbc" => {
            let data = wow_error_strings::WowError_Strings::read(file_contents)?;
            let (table, insert, _select) = WowError_Strings();
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
        }
        "ZoneIntroMusicTable.dbc" => {
            let data = zone_intro_music_table::ZoneIntroMusicTable::read(file_contents)?;
            let (table, insert, _select) = ZoneIntroMusicTable();
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
        }
        "ZoneMusic.dbc" => {
            let data = zone_music::ZoneMusic::read(file_contents)?;
            let (table, insert, _select) = ZoneMusic();
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
        }
        "gtChanceToMeleeCrit.dbc" => {
            let data = gt_chance_to_melee_crit::gtChanceToMeleeCrit::read(file_contents)?;
            let (table, insert, _select) = gtChanceToMeleeCrit();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtChanceToMeleeCritBase.dbc" => {
            let data = gt_chance_to_melee_crit_base::gtChanceToMeleeCritBase::read(file_contents)?;
            let (table, insert, _select) = gtChanceToMeleeCritBase();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtChanceToSpellCrit.dbc" => {
            let data = gt_chance_to_spell_crit::gtChanceToSpellCrit::read(file_contents)?;
            let (table, insert, _select) = gtChanceToSpellCrit();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtChanceToSpellCritBase.dbc" => {
            let data = gt_chance_to_spell_crit_base::gtChanceToSpellCritBase::read(file_contents)?;
            let (table, insert, _select) = gtChanceToSpellCritBase();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtCombatRatings.dbc" => {
            let data = gt_combat_ratings::gtCombatRatings::read(file_contents)?;
            let (table, insert, _select) = gtCombatRatings();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtNPCManaCostScaler.dbc" => {
            let data = gt_npc_mana_cost_scaler::gtNPCManaCostScaler::read(file_contents)?;
            let (table, insert, _select) = gtNPCManaCostScaler();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtOCTRegenHP.dbc" => {
            let data = gt_oct_regen_hp::gtOCTRegenHP::read(file_contents)?;
            let (table, insert, _select) = gtOCTRegenHP();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtOCTRegenMP.dbc" => {
            let data = gt_oct_regen_mp::gtOCTRegenMP::read(file_contents)?;
            let (table, insert, _select) = gtOCTRegenMP();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtRegenHPPerSpt.dbc" => {
            let data = gt_regen_hp_per_spt::gtRegenHPPerSpt::read(file_contents)?;
            let (table, insert, _select) = gtRegenHPPerSpt();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        "gtRegenMPPerSpt.dbc" => {
            let data = gt_regen_mp_per_spt::gtRegenMPPerSpt::read(file_contents)?;
            let (table, insert, _select) = gtRegenMPPerSpt();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.data,
                ])?;
            }
        }
        v => return Err(SqliteError::FilenameNotFound { name: v.to_string() }),
    }

    tx.commit()?;

    Ok(())
}
#[allow(non_snake_case)]
pub(crate) fn AnimationData() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AnimationData (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        weaponflags INTEGER  NOT NULL,
        bodyflags INTEGER  NOT NULL,
        field_0_7_0_3694_004 INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        fallback INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AnimationData (
        id,
        name,
        weaponflags,
        bodyflags,
        field_0_7_0_3694_004,
        flags,
        fallback
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    ,
    "SELECT
        id,
        name,
        weaponflags,
        bodyflags,
        field_0_7_0_3694_004,
        flags,
        fallback
    FROM `AnimationData`;"
    )
}


pub(crate) fn animation_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<animation_data::AnimationData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(animation_data::AnimationDataRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            weaponflags: row.get::<_, i32>(2)?.into(),
            bodyflags: row.get::<_, i32>(3)?.into(),
            field_0_7_0_3694_004: row.get::<_, i32>(4)?.into(),
            flags: row.get::<_, i32>(5)?.into(),
            fallback: row.get::<_, i32>(6)?.into(),
        });
    }
    Ok(animation_data::AnimationData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AreaPOI() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaPOI (
        id INTEGER PRIMARY KEY NOT NULL,
        importance INTEGER  NOT NULL,
        icon INTEGER  NOT NULL,
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
        name_lang_flags INTEGER NOT NULL,
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
        description_lang_flags INTEGER NOT NULL,
        world_state_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AreaPOI (
        id,
        importance,
        icon,
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
        world_state_id
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
    ,
    "SELECT
        id,
        importance,
        icon,
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
        world_state_id
    FROM `AreaPOI`;"
    )
}


pub(crate) fn area_poi_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<area_poi::AreaPOI, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(area_poi::AreaPOIRow {
            id: row.get::<_, i32>(0)?.into(),
            importance: row.get::<_, i32>(1)?.into(),
            icon: row.get::<_, i32>(2)?.into(),
            faction_id: row.get::<_, i32>(3)?.into(),
            pos: [row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(), row.get::<_, f32>(6)?.into(),             ],
            continent_id: row.get::<_, i32>(7)?.into(),
            flags: row.get::<_, i32>(8)?.into(),
            area_id: row.get::<_, i32>(9)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(10)?.into(),
                ko_kr: row.get::<_, String>(11)?.into(),
                fr_fr: row.get::<_, String>(12)?.into(),
                de_de: row.get::<_, String>(13)?.into(),
                en_cn: row.get::<_, String>(14)?.into(),
                en_tw: row.get::<_, String>(15)?.into(),
                es_es: row.get::<_, String>(16)?.into(),
                es_mx: row.get::<_, String>(17)?.into(),
                ru_ru: row.get::<_, String>(18)?.into(),
                ja_jp: row.get::<_, String>(19)?.into(),
                pt_pt: row.get::<_, String>(20)?.into(),
                it_it: row.get::<_, String>(21)?.into(),
                unknown_12: row.get::<_, String>(22)?.into(),
                unknown_13: row.get::<_, String>(23)?.into(),
                unknown_14: row.get::<_, String>(24)?.into(),
                unknown_15: row.get::<_, String>(25)?.into(),
                flags: row.get::<_, u32>(26)?.into(),
            },
            description_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(27)?.into(),
                ko_kr: row.get::<_, String>(28)?.into(),
                fr_fr: row.get::<_, String>(29)?.into(),
                de_de: row.get::<_, String>(30)?.into(),
                en_cn: row.get::<_, String>(31)?.into(),
                en_tw: row.get::<_, String>(32)?.into(),
                es_es: row.get::<_, String>(33)?.into(),
                es_mx: row.get::<_, String>(34)?.into(),
                ru_ru: row.get::<_, String>(35)?.into(),
                ja_jp: row.get::<_, String>(36)?.into(),
                pt_pt: row.get::<_, String>(37)?.into(),
                it_it: row.get::<_, String>(38)?.into(),
                unknown_12: row.get::<_, String>(39)?.into(),
                unknown_13: row.get::<_, String>(40)?.into(),
                unknown_14: row.get::<_, String>(41)?.into(),
                unknown_15: row.get::<_, String>(42)?.into(),
                flags: row.get::<_, u32>(43)?.into(),
            },
            world_state_id: row.get::<_, i32>(44)?.into(),
        });
    }
    Ok(area_poi::AreaPOI { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AreaTable() -> (&'static str, &'static str, &'static str) {
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
        area_name_lang_flags INTEGER NOT NULL,
        faction_group_mask INTEGER  NOT NULL,
        liquid_type_id_0 INTEGER NOT NULL,
        liquid_type_id_1 INTEGER NOT NULL,
        liquid_type_id_2 INTEGER NOT NULL,
        liquid_type_id_3 INTEGER NOT NULL,
        min_elevation REAL  NOT NULL,
        ambient_multiplier REAL  NOT NULL
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
        ambient_multiplier
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
    ,
    "SELECT
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
        ambient_multiplier
    FROM `AreaTable`;"
    )
}


pub(crate) fn area_table_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<area_table::AreaTable, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(area_table::AreaTableRow {
            id: row.get::<_, i32>(0)?.into(),
            continent_id: row.get::<_, i32>(1)?.into(),
            parent_area_id: row.get::<_, i32>(2)?.into(),
            area_bit: row.get::<_, i32>(3)?.into(),
            flags: row.get::<_, i32>(4)?.into(),
            sound_provider_pref: row.get::<_, i32>(5)?.into(),
            sound_provider_pref_underwater: row.get::<_, i32>(6)?.into(),
            ambience_id: row.get::<_, i32>(7)?.into(),
            zone_music: row.get::<_, i32>(8)?.into(),
            intro_sound: row.get::<_, i32>(9)?.into(),
            exploration_level: row.get::<_, i32>(10)?.into(),
            area_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(11)?.into(),
                ko_kr: row.get::<_, String>(12)?.into(),
                fr_fr: row.get::<_, String>(13)?.into(),
                de_de: row.get::<_, String>(14)?.into(),
                en_cn: row.get::<_, String>(15)?.into(),
                en_tw: row.get::<_, String>(16)?.into(),
                es_es: row.get::<_, String>(17)?.into(),
                es_mx: row.get::<_, String>(18)?.into(),
                ru_ru: row.get::<_, String>(19)?.into(),
                ja_jp: row.get::<_, String>(20)?.into(),
                pt_pt: row.get::<_, String>(21)?.into(),
                it_it: row.get::<_, String>(22)?.into(),
                unknown_12: row.get::<_, String>(23)?.into(),
                unknown_13: row.get::<_, String>(24)?.into(),
                unknown_14: row.get::<_, String>(25)?.into(),
                unknown_15: row.get::<_, String>(26)?.into(),
                flags: row.get::<_, u32>(27)?.into(),
            },
            faction_group_mask: row.get::<_, i32>(28)?.into(),
            liquid_type_id: [row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(),             ],
            min_elevation: row.get::<_, f32>(33)?.into(),
            ambient_multiplier: row.get::<_, f32>(34)?.into(),
        });
    }
    Ok(area_table::AreaTable { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AreaTrigger() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `AreaTrigger`;"
    )
}


pub(crate) fn area_trigger_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<area_trigger::AreaTrigger, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(area_trigger::AreaTriggerRow {
            id: row.get::<_, i32>(0)?.into(),
            continent_id: row.get::<_, i32>(1)?.into(),
            pos: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(),             ],
            radius: row.get::<_, f32>(5)?.into(),
            box_length: row.get::<_, f32>(6)?.into(),
            box_width: row.get::<_, f32>(7)?.into(),
            box_height: row.get::<_, f32>(8)?.into(),
            box_yaw: row.get::<_, f32>(9)?.into(),
        });
    }
    Ok(area_trigger::AreaTrigger { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AttackAnimKits() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        item_subclass_id,
        anim_type_id,
        anim_frequency,
        which_hand
    FROM `AttackAnimKits`;"
    )
}


pub(crate) fn attack_anim_kits_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<attack_anim_kits::AttackAnimKits, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(attack_anim_kits::AttackAnimKitsRow {
            id: row.get::<_, i32>(0)?.into(),
            item_subclass_id: row.get::<_, i32>(1)?.into(),
            anim_type_id: row.get::<_, i32>(2)?.into(),
            anim_frequency: row.get::<_, i32>(3)?.into(),
            which_hand: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(attack_anim_kits::AttackAnimKits { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AttackAnimTypes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        anim_id,
        anim_name
    FROM `AttackAnimTypes`;"
    )
}


pub(crate) fn attack_anim_types_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<attack_anim_types::AttackAnimTypes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(attack_anim_types::AttackAnimTypesRow {
            anim_id: row.get::<_, i32>(0)?.into(),
            anim_name: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(attack_anim_types::AttackAnimTypes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AuctionHouse() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `AuctionHouse`;"
    )
}


pub(crate) fn auction_house_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<auction_house::AuctionHouse, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(auction_house::AuctionHouseRow {
            id: row.get::<_, i32>(0)?.into(),
            faction_id: row.get::<_, i32>(1)?.into(),
            deposit_rate: row.get::<_, i32>(2)?.into(),
            consignment_rate: row.get::<_, i32>(3)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                ru_ru: row.get::<_, String>(12)?.into(),
                ja_jp: row.get::<_, String>(13)?.into(),
                pt_pt: row.get::<_, String>(14)?.into(),
                it_it: row.get::<_, String>(15)?.into(),
                unknown_12: row.get::<_, String>(16)?.into(),
                unknown_13: row.get::<_, String>(17)?.into(),
                unknown_14: row.get::<_, String>(18)?.into(),
                unknown_15: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
        });
    }
    Ok(auction_house::AuctionHouse { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn BankBagSlotPrices() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        cost
    FROM `BankBagSlotPrices`;"
    )
}


pub(crate) fn bank_bag_slot_prices_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<bank_bag_slot_prices::BankBagSlotPrices, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(bank_bag_slot_prices::BankBagSlotPricesRow {
            id: row.get::<_, i32>(0)?.into(),
            cost: row.get::<_, i32>(1)?.into(),
        });
    }
    Ok(bank_bag_slot_prices::BankBagSlotPrices { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn BattlemasterList() -> (&'static str, &'static str, &'static str) {
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
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        field_2_0_0_5610_005 INTEGER  NOT NULL,
        field_2_0_0_5610_006 INTEGER  NOT NULL,
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
        name_lang_flags INTEGER NOT NULL,
        field_2_4_0_8089_009 INTEGER  NOT NULL
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
        min_level,
        max_level,
        field_2_0_0_5610_005,
        field_2_0_0_5610_006,
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
        field_2_4_0_8089_009
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
    ,
    "SELECT
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
        min_level,
        max_level,
        field_2_0_0_5610_005,
        field_2_0_0_5610_006,
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
        field_2_4_0_8089_009
    FROM `BattlemasterList`;"
    )
}


pub(crate) fn battlemaster_list_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<battlemaster_list::BattlemasterList, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(battlemaster_list::BattlemasterListRow {
            id: row.get::<_, i32>(0)?.into(),
            map_id: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(),             ],
            instance_type: row.get::<_, i32>(9)?.into(),
            min_level: row.get::<_, i32>(10)?.into(),
            max_level: row.get::<_, i32>(11)?.into(),
            field_2_0_0_5610_005: row.get::<_, i32>(12)?.into(),
            field_2_0_0_5610_006: row.get::<_, i32>(13)?.into(),
            groups_allowed: row.get::<_, i32>(14)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(15)?.into(),
                ko_kr: row.get::<_, String>(16)?.into(),
                fr_fr: row.get::<_, String>(17)?.into(),
                de_de: row.get::<_, String>(18)?.into(),
                en_cn: row.get::<_, String>(19)?.into(),
                en_tw: row.get::<_, String>(20)?.into(),
                es_es: row.get::<_, String>(21)?.into(),
                es_mx: row.get::<_, String>(22)?.into(),
                ru_ru: row.get::<_, String>(23)?.into(),
                ja_jp: row.get::<_, String>(24)?.into(),
                pt_pt: row.get::<_, String>(25)?.into(),
                it_it: row.get::<_, String>(26)?.into(),
                unknown_12: row.get::<_, String>(27)?.into(),
                unknown_13: row.get::<_, String>(28)?.into(),
                unknown_14: row.get::<_, String>(29)?.into(),
                unknown_15: row.get::<_, String>(30)?.into(),
                flags: row.get::<_, u32>(31)?.into(),
            },
            field_2_4_0_8089_009: row.get::<_, i32>(32)?.into(),
        });
    }
    Ok(battlemaster_list::BattlemasterList { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CameraShakes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        shake_type,
        direction,
        amplitude,
        frequency,
        duration,
        phase,
        coefficient
    FROM `CameraShakes`;"
    )
}


pub(crate) fn camera_shakes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<camera_shakes::CameraShakes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(camera_shakes::CameraShakesRow {
            id: row.get::<_, i32>(0)?.into(),
            shake_type: row.get::<_, i32>(1)?.into(),
            direction: row.get::<_, i32>(2)?.into(),
            amplitude: row.get::<_, f32>(3)?.into(),
            frequency: row.get::<_, f32>(4)?.into(),
            duration: row.get::<_, f32>(5)?.into(),
            phase: row.get::<_, f32>(6)?.into(),
            coefficient: row.get::<_, f32>(7)?.into(),
        });
    }
    Ok(camera_shakes::CameraShakes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Cfg_Categories() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Cfg_Categories`;"
    )
}


pub(crate) fn cfg_categories_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cfg_categories::Cfg_Categories, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cfg_categories::Cfg_CategoriesRow {
            id: row.get::<_, i32>(0)?.into(),
            locale_mask: row.get::<_, i32>(1)?.into(),
            create_charset_mask: row.get::<_, i32>(2)?.into(),
            flags: row.get::<_, i32>(3)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                ru_ru: row.get::<_, String>(12)?.into(),
                ja_jp: row.get::<_, String>(13)?.into(),
                pt_pt: row.get::<_, String>(14)?.into(),
                it_it: row.get::<_, String>(15)?.into(),
                unknown_12: row.get::<_, String>(16)?.into(),
                unknown_13: row.get::<_, String>(17)?.into(),
                unknown_14: row.get::<_, String>(18)?.into(),
                unknown_15: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
        });
    }
    Ok(cfg_categories::Cfg_Categories { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Cfg_Configs() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        realm_type,
        player_killing_allowed,
        roleplaying
    FROM `Cfg_Configs`;"
    )
}


pub(crate) fn cfg_configs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cfg_configs::Cfg_Configs, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cfg_configs::Cfg_ConfigsRow {
            id: row.get::<_, i32>(0)?.into(),
            realm_type: row.get::<_, i32>(1)?.into(),
            player_killing_allowed: row.get::<_, i32>(2)?.into(),
            roleplaying: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(cfg_configs::Cfg_Configs { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharBaseInfo() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        race_id,
        class_id
    FROM `CharBaseInfo`;"
    )
}


pub(crate) fn char_base_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_base_info::CharBaseInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_base_info::CharBaseInfoRow {
            race_id: row.get::<_, i8>(0)?.into(),
            class_id: row.get::<_, i8>(1)?.into(),
        });
    }
    Ok(char_base_info::CharBaseInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharHairGeosets() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        race_id,
        sex_id,
        variation_id,
        geoset_id,
        showscalp
    FROM `CharHairGeosets`;"
    )
}


pub(crate) fn char_hair_geosets_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_hair_geosets::CharHairGeosets, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_hair_geosets::CharHairGeosetsRow {
            id: row.get::<_, i32>(0)?.into(),
            race_id: row.get::<_, i32>(1)?.into(),
            sex_id: row.get::<_, i32>(2)?.into(),
            variation_id: row.get::<_, i32>(3)?.into(),
            geoset_id: row.get::<_, i32>(4)?.into(),
            showscalp: row.get::<_, i32>(5)?.into(),
        });
    }
    Ok(char_hair_geosets::CharHairGeosets { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharHairTextures() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        field_0_5_3_3368_001_race,
        field_0_5_3_3368_002_gender,
        field_0_5_3_3368_003,
        field_0_5_3_3368_004_mayberacemask,
        field_0_5_3_3368_005_the_x_in_hair_xy_blp,
        field_0_5_3_3368_006,
        field_0_5_3_3368_007
    FROM `CharHairTextures`;"
    )
}


pub(crate) fn char_hair_textures_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_hair_textures::CharHairTextures, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_hair_textures::CharHairTexturesRow {
            id: row.get::<_, i32>(0)?.into(),
            field_0_5_3_3368_001_race: row.get::<_, i32>(1)?.into(),
            field_0_5_3_3368_002_gender: row.get::<_, i32>(2)?.into(),
            field_0_5_3_3368_003: row.get::<_, i32>(3)?.into(),
            field_0_5_3_3368_004_mayberacemask: row.get::<_, i32>(4)?.into(),
            field_0_5_3_3368_005_the_x_in_hair_xy_blp: row.get::<_, i32>(5)?.into(),
            field_0_5_3_3368_006: row.get::<_, i32>(6)?.into(),
            field_0_5_3_3368_007: row.get::<_, i32>(7)?.into(),
        });
    }
    Ok(char_hair_textures::CharHairTextures { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharSections() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharSections (
        id INTEGER PRIMARY KEY NOT NULL,
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        base_section INTEGER  NOT NULL,
        variation_index INTEGER  NOT NULL,
        color_index INTEGER  NOT NULL,
        texture_name_0 TEXT NOT NULL,
        texture_name_1 TEXT NOT NULL,
        texture_name_2 TEXT NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharSections (
        id,
        race_id,
        sex_id,
        base_section,
        variation_index,
        color_index,
        texture_name_0,
        texture_name_1,
        texture_name_2,
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
        ?10
    );"
    ,
    "SELECT
        id,
        race_id,
        sex_id,
        base_section,
        variation_index,
        color_index,
        texture_name_0,
        texture_name_1,
        texture_name_2,
        flags
    FROM `CharSections`;"
    )
}


pub(crate) fn char_sections_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_sections::CharSections, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_sections::CharSectionsRow {
            id: row.get::<_, i32>(0)?.into(),
            race_id: row.get::<_, i32>(1)?.into(),
            sex_id: row.get::<_, i32>(2)?.into(),
            base_section: row.get::<_, i32>(3)?.into(),
            variation_index: row.get::<_, i32>(4)?.into(),
            color_index: row.get::<_, i32>(5)?.into(),
            texture_name: [row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(),             ],
            flags: row.get::<_, i32>(9)?.into(),
        });
    }
    Ok(char_sections::CharSections { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharStartOutfit() -> (&'static str, &'static str, &'static str) {
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
        inventory_type_11 INTEGER NOT NULL
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
        inventory_type_11
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
        ?41
    );"
    ,
    "SELECT
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
        inventory_type_11
    FROM `CharStartOutfit`;"
    )
}


pub(crate) fn char_start_outfit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_start_outfit::CharStartOutfit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_start_outfit::CharStartOutfitRow {
            id: row.get::<_, i32>(0)?.into(),
            race_id: row.get::<_, i8>(1)?.into(),
            class_id: row.get::<_, i8>(2)?.into(),
            sex_id: row.get::<_, i8>(3)?.into(),
            outfit_id: row.get::<_, i8>(4)?.into(),
            item_id: [row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(),             ],
            display_item_id: [row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(),             ],
            inventory_type: [row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(), row.get::<_, i32>(33)?.into(), row.get::<_, i32>(34)?.into(), row.get::<_, i32>(35)?.into(), row.get::<_, i32>(36)?.into(), row.get::<_, i32>(37)?.into(), row.get::<_, i32>(38)?.into(), row.get::<_, i32>(39)?.into(), row.get::<_, i32>(40)?.into(),             ],
        });
    }
    Ok(char_start_outfit::CharStartOutfit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharTitles() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
        name1_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `CharTitles`;"
    )
}


pub(crate) fn char_titles_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_titles::CharTitles, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_titles::CharTitlesRow {
            id: row.get::<_, i32>(0)?.into(),
            condition_id: row.get::<_, i32>(1)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                ru_ru: row.get::<_, String>(10)?.into(),
                ja_jp: row.get::<_, String>(11)?.into(),
                pt_pt: row.get::<_, String>(12)?.into(),
                it_it: row.get::<_, String>(13)?.into(),
                unknown_12: row.get::<_, String>(14)?.into(),
                unknown_13: row.get::<_, String>(15)?.into(),
                unknown_14: row.get::<_, String>(16)?.into(),
                unknown_15: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
            name1_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(19)?.into(),
                ko_kr: row.get::<_, String>(20)?.into(),
                fr_fr: row.get::<_, String>(21)?.into(),
                de_de: row.get::<_, String>(22)?.into(),
                en_cn: row.get::<_, String>(23)?.into(),
                en_tw: row.get::<_, String>(24)?.into(),
                es_es: row.get::<_, String>(25)?.into(),
                es_mx: row.get::<_, String>(26)?.into(),
                ru_ru: row.get::<_, String>(27)?.into(),
                ja_jp: row.get::<_, String>(28)?.into(),
                pt_pt: row.get::<_, String>(29)?.into(),
                it_it: row.get::<_, String>(30)?.into(),
                unknown_12: row.get::<_, String>(31)?.into(),
                unknown_13: row.get::<_, String>(32)?.into(),
                unknown_14: row.get::<_, String>(33)?.into(),
                unknown_15: row.get::<_, String>(34)?.into(),
                flags: row.get::<_, u32>(35)?.into(),
            },
            mask_id: row.get::<_, i32>(36)?.into(),
        });
    }
    Ok(char_titles::CharTitles { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharVariations() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        race_id,
        sex_id,
        texture_hold_layer_0,
        texture_hold_layer_1,
        texture_hold_layer_2,
        texture_hold_layer_3
    FROM `CharVariations`;"
    )
}


pub(crate) fn char_variations_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_variations::CharVariations, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_variations::CharVariationsRow {
            race_id: row.get::<_, i32>(0)?.into(),
            sex_id: row.get::<_, i32>(1)?.into(),
            texture_hold_layer: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
        });
    }
    Ok(char_variations::CharVariations { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharacterFacialHairStyles() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharacterFacialHairStyles (
        race_id INTEGER  NOT NULL,
        sex_id INTEGER  NOT NULL,
        variation_id INTEGER  NOT NULL,
        geoset_0 INTEGER NOT NULL,
        geoset_1 INTEGER NOT NULL,
        geoset_2 INTEGER NOT NULL,
        geoset_3 INTEGER NOT NULL,
        geoset_4 INTEGER NOT NULL,
        geoset_5 INTEGER NOT NULL,
        geoset_6 INTEGER NOT NULL,
        geoset_7 INTEGER NOT NULL
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
        geoset_4,
        geoset_5,
        geoset_6,
        geoset_7
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
    ,
    "SELECT
        race_id,
        sex_id,
        variation_id,
        geoset_0,
        geoset_1,
        geoset_2,
        geoset_3,
        geoset_4,
        geoset_5,
        geoset_6,
        geoset_7
    FROM `CharacterFacialHairStyles`;"
    )
}


pub(crate) fn character_facial_hair_styles_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<character_facial_hair_styles::CharacterFacialHairStyles, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(character_facial_hair_styles::CharacterFacialHairStylesRow {
            race_id: row.get::<_, i32>(0)?.into(),
            sex_id: row.get::<_, i32>(1)?.into(),
            variation_id: row.get::<_, i32>(2)?.into(),
            geoset: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(),             ],
        });
    }
    Ok(character_facial_hair_styles::CharacterFacialHairStyles { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ChatChannels() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
        shortcut_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ChatChannels`;"
    )
}


pub(crate) fn chat_channels_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chat_channels::ChatChannels, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chat_channels::ChatChannelsRow {
            id: row.get::<_, i32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            faction_group: row.get::<_, i32>(2)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                ru_ru: row.get::<_, String>(11)?.into(),
                ja_jp: row.get::<_, String>(12)?.into(),
                pt_pt: row.get::<_, String>(13)?.into(),
                it_it: row.get::<_, String>(14)?.into(),
                unknown_12: row.get::<_, String>(15)?.into(),
                unknown_13: row.get::<_, String>(16)?.into(),
                unknown_14: row.get::<_, String>(17)?.into(),
                unknown_15: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
            shortcut_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(20)?.into(),
                ko_kr: row.get::<_, String>(21)?.into(),
                fr_fr: row.get::<_, String>(22)?.into(),
                de_de: row.get::<_, String>(23)?.into(),
                en_cn: row.get::<_, String>(24)?.into(),
                en_tw: row.get::<_, String>(25)?.into(),
                es_es: row.get::<_, String>(26)?.into(),
                es_mx: row.get::<_, String>(27)?.into(),
                ru_ru: row.get::<_, String>(28)?.into(),
                ja_jp: row.get::<_, String>(29)?.into(),
                pt_pt: row.get::<_, String>(30)?.into(),
                it_it: row.get::<_, String>(31)?.into(),
                unknown_12: row.get::<_, String>(32)?.into(),
                unknown_13: row.get::<_, String>(33)?.into(),
                unknown_14: row.get::<_, String>(34)?.into(),
                unknown_15: row.get::<_, String>(35)?.into(),
                flags: row.get::<_, u32>(36)?.into(),
            },
        });
    }
    Ok(chat_channels::ChatChannels { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ChatProfanity() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        text,
        language
    FROM `ChatProfanity`;"
    )
}


pub(crate) fn chat_profanity_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chat_profanity::ChatProfanity, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chat_profanity::ChatProfanityRow {
            id: row.get::<_, i32>(0)?.into(),
            text: row.get::<_, String>(1)?.into(),
            language: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(chat_profanity::ChatProfanity { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ChrClasses() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
        name_female_lang_flags INTEGER NOT NULL,
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
        name_male_lang_flags INTEGER NOT NULL,
        filename TEXT  NOT NULL,
        spell_class_set INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
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
        ?55,
        ?56,
        ?57,
        ?58
    );"
    ,
    "SELECT
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
        flags
    FROM `ChrClasses`;"
    )
}


pub(crate) fn chr_classes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chr_classes::ChrClasses, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chr_classes::ChrClassesRow {
            id: row.get::<_, i32>(0)?.into(),
            damage_bonus_stat: row.get::<_, i32>(1)?.into(),
            display_power: row.get::<_, i32>(2)?.into(),
            pet_name_token: row.get::<_, String>(3)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                ru_ru: row.get::<_, String>(12)?.into(),
                ja_jp: row.get::<_, String>(13)?.into(),
                pt_pt: row.get::<_, String>(14)?.into(),
                it_it: row.get::<_, String>(15)?.into(),
                unknown_12: row.get::<_, String>(16)?.into(),
                unknown_13: row.get::<_, String>(17)?.into(),
                unknown_14: row.get::<_, String>(18)?.into(),
                unknown_15: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
            name_female_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(21)?.into(),
                ko_kr: row.get::<_, String>(22)?.into(),
                fr_fr: row.get::<_, String>(23)?.into(),
                de_de: row.get::<_, String>(24)?.into(),
                en_cn: row.get::<_, String>(25)?.into(),
                en_tw: row.get::<_, String>(26)?.into(),
                es_es: row.get::<_, String>(27)?.into(),
                es_mx: row.get::<_, String>(28)?.into(),
                ru_ru: row.get::<_, String>(29)?.into(),
                ja_jp: row.get::<_, String>(30)?.into(),
                pt_pt: row.get::<_, String>(31)?.into(),
                it_it: row.get::<_, String>(32)?.into(),
                unknown_12: row.get::<_, String>(33)?.into(),
                unknown_13: row.get::<_, String>(34)?.into(),
                unknown_14: row.get::<_, String>(35)?.into(),
                unknown_15: row.get::<_, String>(36)?.into(),
                flags: row.get::<_, u32>(37)?.into(),
            },
            name_male_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(38)?.into(),
                ko_kr: row.get::<_, String>(39)?.into(),
                fr_fr: row.get::<_, String>(40)?.into(),
                de_de: row.get::<_, String>(41)?.into(),
                en_cn: row.get::<_, String>(42)?.into(),
                en_tw: row.get::<_, String>(43)?.into(),
                es_es: row.get::<_, String>(44)?.into(),
                es_mx: row.get::<_, String>(45)?.into(),
                ru_ru: row.get::<_, String>(46)?.into(),
                ja_jp: row.get::<_, String>(47)?.into(),
                pt_pt: row.get::<_, String>(48)?.into(),
                it_it: row.get::<_, String>(49)?.into(),
                unknown_12: row.get::<_, String>(50)?.into(),
                unknown_13: row.get::<_, String>(51)?.into(),
                unknown_14: row.get::<_, String>(52)?.into(),
                unknown_15: row.get::<_, String>(53)?.into(),
                flags: row.get::<_, u32>(54)?.into(),
            },
            filename: row.get::<_, String>(55)?.into(),
            spell_class_set: row.get::<_, i32>(56)?.into(),
            flags: row.get::<_, i32>(57)?.into(),
        });
    }
    Ok(chr_classes::ChrClasses { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ChrRaces() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChrRaces (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        faction_id INTEGER  NOT NULL,
        exploration_sound_id INTEGER  NOT NULL,
        male_display_id INTEGER  NOT NULL,
        female_display_id INTEGER  NOT NULL,
        client_prefix TEXT  NOT NULL,
        mount_scale REAL  NOT NULL,
        base_language INTEGER  NOT NULL,
        creature_type INTEGER  NOT NULL,
        res_sickness_spell_id INTEGER  NOT NULL,
        splash_sound_id INTEGER  NOT NULL,
        client_file_string TEXT  NOT NULL,
        cinematic_sequence_id INTEGER  NOT NULL,
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
        name_lang_flags INTEGER NOT NULL,
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
        name_female_lang_flags INTEGER NOT NULL,
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
        name_male_lang_flags INTEGER NOT NULL,
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
        mount_scale,
        base_language,
        creature_type,
        res_sickness_spell_id,
        splash_sound_id,
        client_file_string,
        cinematic_sequence_id,
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
    ,
    "SELECT
        id,
        flags,
        faction_id,
        exploration_sound_id,
        male_display_id,
        female_display_id,
        client_prefix,
        mount_scale,
        base_language,
        creature_type,
        res_sickness_spell_id,
        splash_sound_id,
        client_file_string,
        cinematic_sequence_id,
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
    FROM `ChrRaces`;"
    )
}


pub(crate) fn chr_races_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chr_races::ChrRaces, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chr_races::ChrRacesRow {
            id: row.get::<_, i32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            faction_id: row.get::<_, i32>(2)?.into(),
            exploration_sound_id: row.get::<_, i32>(3)?.into(),
            male_display_id: row.get::<_, i32>(4)?.into(),
            female_display_id: row.get::<_, i32>(5)?.into(),
            client_prefix: row.get::<_, String>(6)?.into(),
            mount_scale: row.get::<_, f32>(7)?.into(),
            base_language: row.get::<_, i32>(8)?.into(),
            creature_type: row.get::<_, i32>(9)?.into(),
            res_sickness_spell_id: row.get::<_, i32>(10)?.into(),
            splash_sound_id: row.get::<_, i32>(11)?.into(),
            client_file_string: row.get::<_, String>(12)?.into(),
            cinematic_sequence_id: row.get::<_, i32>(13)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(14)?.into(),
                ko_kr: row.get::<_, String>(15)?.into(),
                fr_fr: row.get::<_, String>(16)?.into(),
                de_de: row.get::<_, String>(17)?.into(),
                en_cn: row.get::<_, String>(18)?.into(),
                en_tw: row.get::<_, String>(19)?.into(),
                es_es: row.get::<_, String>(20)?.into(),
                es_mx: row.get::<_, String>(21)?.into(),
                ru_ru: row.get::<_, String>(22)?.into(),
                ja_jp: row.get::<_, String>(23)?.into(),
                pt_pt: row.get::<_, String>(24)?.into(),
                it_it: row.get::<_, String>(25)?.into(),
                unknown_12: row.get::<_, String>(26)?.into(),
                unknown_13: row.get::<_, String>(27)?.into(),
                unknown_14: row.get::<_, String>(28)?.into(),
                unknown_15: row.get::<_, String>(29)?.into(),
                flags: row.get::<_, u32>(30)?.into(),
            },
            name_female_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(31)?.into(),
                ko_kr: row.get::<_, String>(32)?.into(),
                fr_fr: row.get::<_, String>(33)?.into(),
                de_de: row.get::<_, String>(34)?.into(),
                en_cn: row.get::<_, String>(35)?.into(),
                en_tw: row.get::<_, String>(36)?.into(),
                es_es: row.get::<_, String>(37)?.into(),
                es_mx: row.get::<_, String>(38)?.into(),
                ru_ru: row.get::<_, String>(39)?.into(),
                ja_jp: row.get::<_, String>(40)?.into(),
                pt_pt: row.get::<_, String>(41)?.into(),
                it_it: row.get::<_, String>(42)?.into(),
                unknown_12: row.get::<_, String>(43)?.into(),
                unknown_13: row.get::<_, String>(44)?.into(),
                unknown_14: row.get::<_, String>(45)?.into(),
                unknown_15: row.get::<_, String>(46)?.into(),
                flags: row.get::<_, u32>(47)?.into(),
            },
            name_male_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(48)?.into(),
                ko_kr: row.get::<_, String>(49)?.into(),
                fr_fr: row.get::<_, String>(50)?.into(),
                de_de: row.get::<_, String>(51)?.into(),
                en_cn: row.get::<_, String>(52)?.into(),
                en_tw: row.get::<_, String>(53)?.into(),
                es_es: row.get::<_, String>(54)?.into(),
                es_mx: row.get::<_, String>(55)?.into(),
                ru_ru: row.get::<_, String>(56)?.into(),
                ja_jp: row.get::<_, String>(57)?.into(),
                pt_pt: row.get::<_, String>(58)?.into(),
                it_it: row.get::<_, String>(59)?.into(),
                unknown_12: row.get::<_, String>(60)?.into(),
                unknown_13: row.get::<_, String>(61)?.into(),
                unknown_14: row.get::<_, String>(62)?.into(),
                unknown_15: row.get::<_, String>(63)?.into(),
                flags: row.get::<_, u32>(64)?.into(),
            },
            facial_hair_customization: [row.get::<_, String>(65)?.into(), row.get::<_, String>(66)?.into(),             ],
            hair_customization: row.get::<_, String>(67)?.into(),
            required_expansion: row.get::<_, i32>(68)?.into(),
        });
    }
    Ok(chr_races::ChrRaces { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CinematicCamera() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        model,
        sound_id,
        origin_0,
        origin_1,
        origin_2,
        origin_facing
    FROM `CinematicCamera`;"
    )
}


pub(crate) fn cinematic_camera_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cinematic_camera::CinematicCamera, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cinematic_camera::CinematicCameraRow {
            id: row.get::<_, i32>(0)?.into(),
            model: row.get::<_, String>(1)?.into(),
            sound_id: row.get::<_, i32>(2)?.into(),
            origin: [row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(),             ],
            origin_facing: row.get::<_, f32>(6)?.into(),
        });
    }
    Ok(cinematic_camera::CinematicCamera { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CinematicSequences() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CinematicSequences`;"
    )
}


pub(crate) fn cinematic_sequences_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cinematic_sequences::CinematicSequences, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cinematic_sequences::CinematicSequencesRow {
            id: row.get::<_, i32>(0)?.into(),
            sound_id: row.get::<_, i32>(1)?.into(),
            camera: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
        });
    }
    Ok(cinematic_sequences::CinematicSequences { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureDisplayInfo() -> (&'static str, &'static str, &'static str) {
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
        particle_color_id INTEGER  NOT NULL
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
        ?14
    );"
    ,
    "SELECT
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
        particle_color_id
    FROM `CreatureDisplayInfo`;"
    )
}


pub(crate) fn creature_display_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_display_info::CreatureDisplayInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_display_info::CreatureDisplayInfoRow {
            id: row.get::<_, i32>(0)?.into(),
            model_id: row.get::<_, i32>(1)?.into(),
            sound_id: row.get::<_, i32>(2)?.into(),
            extended_display_info_id: row.get::<_, i32>(3)?.into(),
            creature_model_scale: row.get::<_, f32>(4)?.into(),
            creature_model_alpha: row.get::<_, i32>(5)?.into(),
            texture_variation: [row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(),             ],
            portrait_texture_name: row.get::<_, String>(9)?.into(),
            size_class: row.get::<_, i32>(10)?.into(),
            blood_id: row.get::<_, i32>(11)?.into(),
            n_p_c_sound_id: row.get::<_, i32>(12)?.into(),
            particle_color_id: row.get::<_, i32>(13)?.into(),
        });
    }
    Ok(creature_display_info::CreatureDisplayInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureDisplayInfoExtra() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CreatureDisplayInfoExtra`;"
    )
}


pub(crate) fn creature_display_info_extra_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_display_info_extra::CreatureDisplayInfoExtra, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_display_info_extra::CreatureDisplayInfoExtraRow {
            id: row.get::<_, i32>(0)?.into(),
            display_race_id: row.get::<_, i32>(1)?.into(),
            display_sex_id: row.get::<_, i32>(2)?.into(),
            skin_id: row.get::<_, i32>(3)?.into(),
            face_id: row.get::<_, i32>(4)?.into(),
            hair_style_id: row.get::<_, i32>(5)?.into(),
            hair_color_id: row.get::<_, i32>(6)?.into(),
            facial_hair_id: row.get::<_, i32>(7)?.into(),
            n_p_c_item_display: [row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(),             ],
            flags: row.get::<_, i32>(19)?.into(),
            bake_name: row.get::<_, String>(20)?.into(),
        });
    }
    Ok(creature_display_info_extra::CreatureDisplayInfoExtra { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureFamily() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
        ?26
    );"
    ,
    "SELECT
        id,
        min_scale,
        min_scale_level,
        max_scale,
        max_scale_level,
        skill_line_0,
        skill_line_1,
        pet_food_mask,
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
    FROM `CreatureFamily`;"
    )
}


pub(crate) fn creature_family_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_family::CreatureFamily, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_family::CreatureFamilyRow {
            id: row.get::<_, i32>(0)?.into(),
            min_scale: row.get::<_, f32>(1)?.into(),
            min_scale_level: row.get::<_, i32>(2)?.into(),
            max_scale: row.get::<_, f32>(3)?.into(),
            max_scale_level: row.get::<_, i32>(4)?.into(),
            skill_line: [row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(),             ],
            pet_food_mask: row.get::<_, i32>(7)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(8)?.into(),
                ko_kr: row.get::<_, String>(9)?.into(),
                fr_fr: row.get::<_, String>(10)?.into(),
                de_de: row.get::<_, String>(11)?.into(),
                en_cn: row.get::<_, String>(12)?.into(),
                en_tw: row.get::<_, String>(13)?.into(),
                es_es: row.get::<_, String>(14)?.into(),
                es_mx: row.get::<_, String>(15)?.into(),
                ru_ru: row.get::<_, String>(16)?.into(),
                ja_jp: row.get::<_, String>(17)?.into(),
                pt_pt: row.get::<_, String>(18)?.into(),
                it_it: row.get::<_, String>(19)?.into(),
                unknown_12: row.get::<_, String>(20)?.into(),
                unknown_13: row.get::<_, String>(21)?.into(),
                unknown_14: row.get::<_, String>(22)?.into(),
                unknown_15: row.get::<_, String>(23)?.into(),
                flags: row.get::<_, u32>(24)?.into(),
            },
            icon_file: row.get::<_, String>(25)?.into(),
        });
    }
    Ok(creature_family::CreatureFamily { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureModelData() -> (&'static str, &'static str, &'static str) {
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
        geo_box_0 REAL NOT NULL,
        geo_box_1 REAL NOT NULL,
        geo_box_2 REAL NOT NULL,
        geo_box_3 REAL NOT NULL,
        geo_box_4 REAL NOT NULL,
        geo_box_5 REAL NOT NULL,
        attached_effect_scale REAL  NOT NULL
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
        geo_box_0,
        geo_box_1,
        geo_box_2,
        geo_box_3,
        geo_box_4,
        geo_box_5,
        attached_effect_scale
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
    ,
    "SELECT
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
        geo_box_0,
        geo_box_1,
        geo_box_2,
        geo_box_3,
        geo_box_4,
        geo_box_5,
        attached_effect_scale
    FROM `CreatureModelData`;"
    )
}


pub(crate) fn creature_model_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_model_data::CreatureModelData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_model_data::CreatureModelDataRow {
            id: row.get::<_, i32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            model_name: row.get::<_, String>(2)?.into(),
            size_class: row.get::<_, i32>(3)?.into(),
            model_scale: row.get::<_, f32>(4)?.into(),
            blood_id: row.get::<_, i32>(5)?.into(),
            footprint_texture_id: row.get::<_, i32>(6)?.into(),
            footprint_texture_length: row.get::<_, f32>(7)?.into(),
            footprint_texture_width: row.get::<_, f32>(8)?.into(),
            footprint_particle_scale: row.get::<_, f32>(9)?.into(),
            foley_material_id: row.get::<_, i32>(10)?.into(),
            footstep_shake_size: row.get::<_, i32>(11)?.into(),
            death_thud_shake_size: row.get::<_, i32>(12)?.into(),
            sound_id: row.get::<_, i32>(13)?.into(),
            collision_width: row.get::<_, f32>(14)?.into(),
            collision_height: row.get::<_, f32>(15)?.into(),
            mount_height: row.get::<_, f32>(16)?.into(),
            geo_box: [row.get::<_, f32>(17)?.into(), row.get::<_, f32>(18)?.into(), row.get::<_, f32>(19)?.into(), row.get::<_, f32>(20)?.into(), row.get::<_, f32>(21)?.into(), row.get::<_, f32>(22)?.into(),             ],
            attached_effect_scale: row.get::<_, f32>(23)?.into(),
        });
    }
    Ok(creature_model_data::CreatureModelData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureSoundData() -> (&'static str, &'static str, &'static str) {
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
        submerged_sound_id INTEGER  NOT NULL
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
        submerged_sound_id
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
    ,
    "SELECT
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
        submerged_sound_id
    FROM `CreatureSoundData`;"
    )
}


pub(crate) fn creature_sound_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_sound_data::CreatureSoundData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_sound_data::CreatureSoundDataRow {
            id: row.get::<_, i32>(0)?.into(),
            sound_exertion_id: row.get::<_, i32>(1)?.into(),
            sound_exertion_critical_id: row.get::<_, i32>(2)?.into(),
            sound_injury_id: row.get::<_, i32>(3)?.into(),
            sound_injury_critical_id: row.get::<_, i32>(4)?.into(),
            sound_injury_crushing_blow_id: row.get::<_, i32>(5)?.into(),
            sound_death_id: row.get::<_, i32>(6)?.into(),
            sound_stun_id: row.get::<_, i32>(7)?.into(),
            sound_stand_id: row.get::<_, i32>(8)?.into(),
            sound_footstep_id: row.get::<_, i32>(9)?.into(),
            sound_aggro_id: row.get::<_, i32>(10)?.into(),
            sound_wing_flap_id: row.get::<_, i32>(11)?.into(),
            sound_wing_glide_id: row.get::<_, i32>(12)?.into(),
            sound_alert_id: row.get::<_, i32>(13)?.into(),
            sound_fidget: [row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(),             ],
            custom_attack: [row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(),             ],
            n_p_c_sound_id: row.get::<_, i32>(23)?.into(),
            loop_sound_id: row.get::<_, i32>(24)?.into(),
            creature_impact_type: row.get::<_, i32>(25)?.into(),
            sound_jump_start_id: row.get::<_, i32>(26)?.into(),
            sound_jump_end_id: row.get::<_, i32>(27)?.into(),
            sound_pet_attack_id: row.get::<_, i32>(28)?.into(),
            sound_pet_order_id: row.get::<_, i32>(29)?.into(),
            sound_pet_dismiss_id: row.get::<_, i32>(30)?.into(),
            fidget_delay_seconds_min: row.get::<_, f32>(31)?.into(),
            fidget_delay_seconds_max: row.get::<_, f32>(32)?.into(),
            birth_sound_id: row.get::<_, i32>(33)?.into(),
            spell_cast_directed_sound_id: row.get::<_, i32>(34)?.into(),
            submerge_sound_id: row.get::<_, i32>(35)?.into(),
            submerged_sound_id: row.get::<_, i32>(36)?.into(),
        });
    }
    Ok(creature_sound_data::CreatureSoundData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureSpellData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        spells_0,
        spells_1,
        spells_2,
        spells_3,
        availability_0,
        availability_1,
        availability_2,
        availability_3
    FROM `CreatureSpellData`;"
    )
}


pub(crate) fn creature_spell_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_spell_data::CreatureSpellData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_spell_data::CreatureSpellDataRow {
            id: row.get::<_, i32>(0)?.into(),
            spells: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
            availability: [row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(),             ],
        });
    }
    Ok(creature_spell_data::CreatureSpellData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureType() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `CreatureType`;"
    )
}


pub(crate) fn creature_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_type::CreatureType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_type::CreatureTypeRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            flags: row.get::<_, i32>(18)?.into(),
        });
    }
    Ok(creature_type::CreatureType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DeathThudLookups() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        size_class,
        terrain_type_sound_id,
        sound_entry_id,
        sound_entry_id_water
    FROM `DeathThudLookups`;"
    )
}


pub(crate) fn death_thud_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<death_thud_lookups::DeathThudLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(death_thud_lookups::DeathThudLookupsRow {
            id: row.get::<_, i32>(0)?.into(),
            size_class: row.get::<_, i32>(1)?.into(),
            terrain_type_sound_id: row.get::<_, i32>(2)?.into(),
            sound_entry_id: row.get::<_, i32>(3)?.into(),
            sound_entry_id_water: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(death_thud_lookups::DeathThudLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DeclinedWord() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        word
    FROM `DeclinedWord`;"
    )
}


pub(crate) fn declined_word_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<declined_word::DeclinedWord, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(declined_word::DeclinedWordRow {
            id: row.get::<_, i32>(0)?.into(),
            word: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(declined_word::DeclinedWord { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DeclinedWordCases() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        declined_word_id,
        case_index,
        declined_word
    FROM `DeclinedWordCases`;"
    )
}


pub(crate) fn declined_word_cases_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<declined_word_cases::DeclinedWordCases, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(declined_word_cases::DeclinedWordCasesRow {
            id: row.get::<_, i32>(0)?.into(),
            declined_word_id: row.get::<_, i32>(1)?.into(),
            case_index: row.get::<_, i32>(2)?.into(),
            declined_word: row.get::<_, String>(3)?.into(),
        });
    }
    Ok(declined_word_cases::DeclinedWordCases { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DurabilityCosts() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `DurabilityCosts`;"
    )
}


pub(crate) fn durability_costs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<durability_costs::DurabilityCosts, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(durability_costs::DurabilityCostsRow {
            id: row.get::<_, i32>(0)?.into(),
            weapon_sub_class_cost: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(),             ],
            armor_sub_class_cost: [row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(),             ],
        });
    }
    Ok(durability_costs::DurabilityCosts { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DurabilityQuality() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        data
    FROM `DurabilityQuality`;"
    )
}


pub(crate) fn durability_quality_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<durability_quality::DurabilityQuality, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(durability_quality::DurabilityQualityRow {
            id: row.get::<_, i32>(0)?.into(),
            data: row.get::<_, f32>(1)?.into(),
        });
    }
    Ok(durability_quality::DurabilityQuality { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Emotes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        emote_slash_command,
        anim_id,
        emote_flags,
        emote_spec_proc,
        emote_spec_proc_param,
        event_sound_id
    FROM `Emotes`;"
    )
}


pub(crate) fn emotes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes::Emotes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes::EmotesRow {
            id: row.get::<_, i32>(0)?.into(),
            emote_slash_command: row.get::<_, String>(1)?.into(),
            anim_id: row.get::<_, i32>(2)?.into(),
            emote_flags: row.get::<_, i32>(3)?.into(),
            emote_spec_proc: row.get::<_, i32>(4)?.into(),
            emote_spec_proc_param: row.get::<_, i32>(5)?.into(),
            event_sound_id: row.get::<_, i32>(6)?.into(),
        });
    }
    Ok(emotes::Emotes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn EmotesText() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `EmotesText`;"
    )
}


pub(crate) fn emotes_text_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes_text::EmotesText, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes_text::EmotesTextRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            emote_id: row.get::<_, i32>(2)?.into(),
            emote_text: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(),             ],
        });
    }
    Ok(emotes_text::EmotesText { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn EmotesTextData() -> (&'static str, &'static str, &'static str) {
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
        text_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `EmotesTextData`;"
    )
}


pub(crate) fn emotes_text_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes_text_data::EmotesTextData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes_text_data::EmotesTextDataRow {
            id: row.get::<_, i32>(0)?.into(),
            text_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(emotes_text_data::EmotesTextData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn EmotesTextSound() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        emotes_text_id,
        race_id,
        sex_id,
        sound_id
    FROM `EmotesTextSound`;"
    )
}


pub(crate) fn emotes_text_sound_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes_text_sound::EmotesTextSound, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes_text_sound::EmotesTextSoundRow {
            id: row.get::<_, i32>(0)?.into(),
            emotes_text_id: row.get::<_, i32>(1)?.into(),
            race_id: row.get::<_, i32>(2)?.into(),
            sex_id: row.get::<_, i32>(3)?.into(),
            sound_id: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(emotes_text_sound::EmotesTextSound { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn EnvironmentalDamage() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        enum_id,
        visualkit_id
    FROM `EnvironmentalDamage`;"
    )
}


pub(crate) fn environmental_damage_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<environmental_damage::EnvironmentalDamage, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(environmental_damage::EnvironmentalDamageRow {
            id: row.get::<_, i32>(0)?.into(),
            enum_id: row.get::<_, i32>(1)?.into(),
            visualkit_id: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(environmental_damage::EnvironmentalDamage { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Exhaustion() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `Exhaustion`;"
    )
}


pub(crate) fn exhaustion_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<exhaustion::Exhaustion, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(exhaustion::ExhaustionRow {
            id: row.get::<_, i32>(0)?.into(),
            xp: row.get::<_, i32>(1)?.into(),
            factor: row.get::<_, f32>(2)?.into(),
            outdoor_hours: row.get::<_, f32>(3)?.into(),
            inn_hours: row.get::<_, f32>(4)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                ru_ru: row.get::<_, String>(13)?.into(),
                ja_jp: row.get::<_, String>(14)?.into(),
                pt_pt: row.get::<_, String>(15)?.into(),
                it_it: row.get::<_, String>(16)?.into(),
                unknown_12: row.get::<_, String>(17)?.into(),
                unknown_13: row.get::<_, String>(18)?.into(),
                unknown_14: row.get::<_, String>(19)?.into(),
                unknown_15: row.get::<_, String>(20)?.into(),
                flags: row.get::<_, u32>(21)?.into(),
            },
            threshold: row.get::<_, f32>(22)?.into(),
        });
    }
    Ok(exhaustion::Exhaustion { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Faction() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
        description_lang_flags INTEGER NOT NULL
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
        ?53
    );"
    ,
    "SELECT
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
    FROM `Faction`;"
    )
}


pub(crate) fn faction_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<faction::Faction, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(faction::FactionRow {
            id: row.get::<_, i32>(0)?.into(),
            reputation_index: row.get::<_, i32>(1)?.into(),
            reputation_race_mask: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
            reputation_class_mask: [row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
            reputation_base: [row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(),             ],
            reputation_flags: [row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(),             ],
            parent_faction_id: row.get::<_, i32>(18)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(19)?.into(),
                ko_kr: row.get::<_, String>(20)?.into(),
                fr_fr: row.get::<_, String>(21)?.into(),
                de_de: row.get::<_, String>(22)?.into(),
                en_cn: row.get::<_, String>(23)?.into(),
                en_tw: row.get::<_, String>(24)?.into(),
                es_es: row.get::<_, String>(25)?.into(),
                es_mx: row.get::<_, String>(26)?.into(),
                ru_ru: row.get::<_, String>(27)?.into(),
                ja_jp: row.get::<_, String>(28)?.into(),
                pt_pt: row.get::<_, String>(29)?.into(),
                it_it: row.get::<_, String>(30)?.into(),
                unknown_12: row.get::<_, String>(31)?.into(),
                unknown_13: row.get::<_, String>(32)?.into(),
                unknown_14: row.get::<_, String>(33)?.into(),
                unknown_15: row.get::<_, String>(34)?.into(),
                flags: row.get::<_, u32>(35)?.into(),
            },
            description_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(36)?.into(),
                ko_kr: row.get::<_, String>(37)?.into(),
                fr_fr: row.get::<_, String>(38)?.into(),
                de_de: row.get::<_, String>(39)?.into(),
                en_cn: row.get::<_, String>(40)?.into(),
                en_tw: row.get::<_, String>(41)?.into(),
                es_es: row.get::<_, String>(42)?.into(),
                es_mx: row.get::<_, String>(43)?.into(),
                ru_ru: row.get::<_, String>(44)?.into(),
                ja_jp: row.get::<_, String>(45)?.into(),
                pt_pt: row.get::<_, String>(46)?.into(),
                it_it: row.get::<_, String>(47)?.into(),
                unknown_12: row.get::<_, String>(48)?.into(),
                unknown_13: row.get::<_, String>(49)?.into(),
                unknown_14: row.get::<_, String>(50)?.into(),
                unknown_15: row.get::<_, String>(51)?.into(),
                flags: row.get::<_, u32>(52)?.into(),
            },
        });
    }
    Ok(faction::Faction { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn FactionGroup() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `FactionGroup`;"
    )
}


pub(crate) fn faction_group_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<faction_group::FactionGroup, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(faction_group::FactionGroupRow {
            id: row.get::<_, i32>(0)?.into(),
            mask_id: row.get::<_, i32>(1)?.into(),
            internal_name: row.get::<_, String>(2)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                ru_ru: row.get::<_, String>(11)?.into(),
                ja_jp: row.get::<_, String>(12)?.into(),
                pt_pt: row.get::<_, String>(13)?.into(),
                it_it: row.get::<_, String>(14)?.into(),
                unknown_12: row.get::<_, String>(15)?.into(),
                unknown_13: row.get::<_, String>(16)?.into(),
                unknown_14: row.get::<_, String>(17)?.into(),
                unknown_15: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
        });
    }
    Ok(faction_group::FactionGroup { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn FactionTemplate() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `FactionTemplate`;"
    )
}


pub(crate) fn faction_template_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<faction_template::FactionTemplate, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(faction_template::FactionTemplateRow {
            id: row.get::<_, i32>(0)?.into(),
            faction: row.get::<_, i32>(1)?.into(),
            flags: row.get::<_, i32>(2)?.into(),
            faction_group: row.get::<_, i32>(3)?.into(),
            friend_group: row.get::<_, i32>(4)?.into(),
            enemy_group: row.get::<_, i32>(5)?.into(),
            enemies: [row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
            friend: [row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(),             ],
        });
    }
    Ok(faction_template::FactionTemplate { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn FootprintTextures() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        footstep_filename
    FROM `FootprintTextures`;"
    )
}


pub(crate) fn footprint_textures_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<footprint_textures::FootprintTextures, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(footprint_textures::FootprintTexturesRow {
            id: row.get::<_, i32>(0)?.into(),
            footstep_filename: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(footprint_textures::FootprintTextures { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn FootstepTerrainLookup() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        creature_footstep_id,
        terrain_sound_id,
        sound_id,
        sound_id_splash
    FROM `FootstepTerrainLookup`;"
    )
}


pub(crate) fn footstep_terrain_lookup_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<footstep_terrain_lookup::FootstepTerrainLookup, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(footstep_terrain_lookup::FootstepTerrainLookupRow {
            id: row.get::<_, i32>(0)?.into(),
            creature_footstep_id: row.get::<_, i32>(1)?.into(),
            terrain_sound_id: row.get::<_, i32>(2)?.into(),
            sound_id: row.get::<_, i32>(3)?.into(),
            sound_id_splash: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(footstep_terrain_lookup::FootstepTerrainLookup { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMSurveyCurrentSurvey() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        gm_survey_id
    FROM `GMSurveyCurrentSurvey`;"
    )
}


pub(crate) fn gm_survey_current_survey_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_survey_current_survey::GMSurveyCurrentSurvey, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_survey_current_survey::GMSurveyCurrentSurveyRow {
            id: row.get::<_, i32>(0)?.into(),
            gm_survey_id: row.get::<_, i32>(1)?.into(),
        });
    }
    Ok(gm_survey_current_survey::GMSurveyCurrentSurvey { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMSurveyQuestions() -> (&'static str, &'static str, &'static str) {
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
        question_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `GMSurveyQuestions`;"
    )
}


pub(crate) fn gm_survey_questions_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_survey_questions::GMSurveyQuestions, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_survey_questions::GMSurveyQuestionsRow {
            id: row.get::<_, i32>(0)?.into(),
            question_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(gm_survey_questions::GMSurveyQuestions { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMSurveySurveys() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `GMSurveySurveys`;"
    )
}


pub(crate) fn gm_survey_surveys_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_survey_surveys::GMSurveySurveys, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_survey_surveys::GMSurveySurveysRow {
            id: row.get::<_, i32>(0)?.into(),
            q: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(),             ],
        });
    }
    Ok(gm_survey_surveys::GMSurveySurveys { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMTicketCategory() -> (&'static str, &'static str, &'static str) {
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
        category_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `GMTicketCategory`;"
    )
}


pub(crate) fn gm_ticket_category_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_ticket_category::GMTicketCategory, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_ticket_category::GMTicketCategoryRow {
            id: row.get::<_, i32>(0)?.into(),
            category_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(gm_ticket_category::GMTicketCategory { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GameObjectArtKit() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        texture_variation_0,
        texture_variation_1,
        texture_variation_2,
        attach_model_0,
        attach_model_1,
        attach_model_2,
        attach_model_3
    FROM `GameObjectArtKit`;"
    )
}


pub(crate) fn game_object_art_kit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<game_object_art_kit::GameObjectArtKit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(game_object_art_kit::GameObjectArtKitRow {
            id: row.get::<_, i32>(0)?.into(),
            texture_variation: [row.get::<_, String>(1)?.into(), row.get::<_, String>(2)?.into(), row.get::<_, String>(3)?.into(),             ],
            attach_model: [row.get::<_, String>(4)?.into(), row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(),             ],
        });
    }
    Ok(game_object_art_kit::GameObjectArtKit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GameObjectDisplayInfo() -> (&'static str, &'static str, &'static str) {
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
        geo_box_max_2 REAL NOT NULL
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
        geo_box_max_2
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
    ,
    "SELECT
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
        geo_box_max_2
    FROM `GameObjectDisplayInfo`;"
    )
}


pub(crate) fn game_object_display_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<game_object_display_info::GameObjectDisplayInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(game_object_display_info::GameObjectDisplayInfoRow {
            id: row.get::<_, i32>(0)?.into(),
            model_name: row.get::<_, String>(1)?.into(),
            sound: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(),             ],
            geo_box_min: [row.get::<_, f32>(12)?.into(), row.get::<_, f32>(13)?.into(), row.get::<_, f32>(14)?.into(),             ],
            geo_box_max: [row.get::<_, f32>(15)?.into(), row.get::<_, f32>(16)?.into(), row.get::<_, f32>(17)?.into(),             ],
        });
    }
    Ok(game_object_display_info::GameObjectDisplayInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GameTables() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        name,
        num_rows,
        num_columns
    FROM `GameTables`;"
    )
}


pub(crate) fn game_tables_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<game_tables::GameTables, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(game_tables::GameTablesRow {
            name: row.get::<_, String>(0)?.into(),
            num_rows: row.get::<_, i32>(1)?.into(),
            num_columns: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(game_tables::GameTables { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GameTips() -> (&'static str, &'static str, &'static str) {
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
        text_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `GameTips`;"
    )
}


pub(crate) fn game_tips_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<game_tips::GameTips, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(game_tips::GameTipsRow {
            id: row.get::<_, i32>(0)?.into(),
            text_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(game_tips::GameTips { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GemProperties() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        enchant_id,
        maxcount_inv,
        maxcount_item,
        ty
    FROM `GemProperties`;"
    )
}


pub(crate) fn gem_properties_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gem_properties::GemProperties, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gem_properties::GemPropertiesRow {
            id: row.get::<_, i32>(0)?.into(),
            enchant_id: row.get::<_, i32>(1)?.into(),
            maxcount_inv: row.get::<_, i32>(2)?.into(),
            maxcount_item: row.get::<_, i32>(3)?.into(),
            ty: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(gem_properties::GemProperties { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GroundEffectDoodad() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GroundEffectDoodad (
        id INTEGER PRIMARY KEY NOT NULL,
        doodad_id_tag INTEGER  NOT NULL,
        doodadpath TEXT  NOT NULL
    );"
    ,
    "INSERT INTO GroundEffectDoodad (
        id,
        doodad_id_tag,
        doodadpath
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    ,
    "SELECT
        id,
        doodad_id_tag,
        doodadpath
    FROM `GroundEffectDoodad`;"
    )
}


pub(crate) fn ground_effect_doodad_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<ground_effect_doodad::GroundEffectDoodad, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(ground_effect_doodad::GroundEffectDoodadRow {
            id: row.get::<_, i32>(0)?.into(),
            doodad_id_tag: row.get::<_, i32>(1)?.into(),
            doodadpath: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(ground_effect_doodad::GroundEffectDoodad { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GroundEffectTexture() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GroundEffectTexture (
        id INTEGER PRIMARY KEY NOT NULL,
        doodad_id_0 INTEGER NOT NULL,
        doodad_id_1 INTEGER NOT NULL,
        doodad_id_2 INTEGER NOT NULL,
        doodad_id_3 INTEGER NOT NULL,
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
        density,
        sound
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6,
        ?7
    );"
    ,
    "SELECT
        id,
        doodad_id_0,
        doodad_id_1,
        doodad_id_2,
        doodad_id_3,
        density,
        sound
    FROM `GroundEffectTexture`;"
    )
}


pub(crate) fn ground_effect_texture_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<ground_effect_texture::GroundEffectTexture, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(ground_effect_texture::GroundEffectTextureRow {
            id: row.get::<_, i32>(0)?.into(),
            doodad_id: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
            density: row.get::<_, i32>(5)?.into(),
            sound: row.get::<_, i32>(6)?.into(),
        });
    }
    Ok(ground_effect_texture::GroundEffectTexture { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn HelmetGeosetVisData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        hide_geoset_0,
        hide_geoset_1,
        hide_geoset_2,
        hide_geoset_3,
        hide_geoset_4,
        hide_geoset_5,
        hide_geoset_6
    FROM `HelmetGeosetVisData`;"
    )
}


pub(crate) fn helmet_geoset_vis_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<helmet_geoset_vis_data::HelmetGeosetVisData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(helmet_geoset_vis_data::HelmetGeosetVisDataRow {
            id: row.get::<_, i32>(0)?.into(),
            hide_geoset: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(),             ],
        });
    }
    Ok(helmet_geoset_vis_data::HelmetGeosetVisData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Item() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Item (
        id INTEGER PRIMARY KEY NOT NULL,
        display_info_id INTEGER  NOT NULL,
        inventory_type INTEGER  NOT NULL,
        sheathe_type INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Item (
        id,
        display_info_id,
        inventory_type,
        sheathe_type
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    ,
    "SELECT
        id,
        display_info_id,
        inventory_type,
        sheathe_type
    FROM `Item`;"
    )
}


pub(crate) fn item_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item::Item, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item::ItemRow {
            id: row.get::<_, i32>(0)?.into(),
            display_info_id: row.get::<_, i32>(1)?.into(),
            inventory_type: row.get::<_, i32>(2)?.into(),
            sheathe_type: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(item::Item { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemBagFamily() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemBagFamily`;"
    )
}


pub(crate) fn item_bag_family_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_bag_family::ItemBagFamily, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_bag_family::ItemBagFamilyRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(item_bag_family::ItemBagFamily { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemClass() -> (&'static str, &'static str, &'static str) {
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
        class_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemClass`;"
    )
}


pub(crate) fn item_class_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_class::ItemClass, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_class::ItemClassRow {
            class_id: row.get::<_, i32>(0)?.into(),
            subclass_map_id: row.get::<_, i32>(1)?.into(),
            flags: row.get::<_, i32>(2)?.into(),
            class_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                ru_ru: row.get::<_, String>(11)?.into(),
                ja_jp: row.get::<_, String>(12)?.into(),
                pt_pt: row.get::<_, String>(13)?.into(),
                it_it: row.get::<_, String>(14)?.into(),
                unknown_12: row.get::<_, String>(15)?.into(),
                unknown_13: row.get::<_, String>(16)?.into(),
                unknown_14: row.get::<_, String>(17)?.into(),
                unknown_15: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
        });
    }
    Ok(item_class::ItemClass { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemCondExtCosts() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        cond_extended_cost,
        item_extended_cost_entry,
        arena_season
    FROM `ItemCondExtCosts`;"
    )
}


pub(crate) fn item_cond_ext_costs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_cond_ext_costs::ItemCondExtCosts, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_cond_ext_costs::ItemCondExtCostsRow {
            id: row.get::<_, i32>(0)?.into(),
            cond_extended_cost: row.get::<_, i32>(1)?.into(),
            item_extended_cost_entry: row.get::<_, i32>(2)?.into(),
            arena_season: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(item_cond_ext_costs::ItemCondExtCosts { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemDisplayInfo() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `ItemDisplayInfo`;"
    )
}


pub(crate) fn item_display_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_display_info::ItemDisplayInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_display_info::ItemDisplayInfoRow {
            id: row.get::<_, i32>(0)?.into(),
            model_name: [row.get::<_, String>(1)?.into(), row.get::<_, String>(2)?.into(),             ],
            model_texture: [row.get::<_, String>(3)?.into(), row.get::<_, String>(4)?.into(),             ],
            inventory_icon: [row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(),             ],
            geoset_group: [row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
            flags: row.get::<_, i32>(10)?.into(),
            spell_visual_id: row.get::<_, i32>(11)?.into(),
            group_sound_index: row.get::<_, i32>(12)?.into(),
            helmet_geoset_vis_id: [row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(),             ],
            texture: [row.get::<_, String>(15)?.into(), row.get::<_, String>(16)?.into(), row.get::<_, String>(17)?.into(), row.get::<_, String>(18)?.into(), row.get::<_, String>(19)?.into(), row.get::<_, String>(20)?.into(), row.get::<_, String>(21)?.into(), row.get::<_, String>(22)?.into(),             ],
            item_visual: row.get::<_, i32>(23)?.into(),
            particle_color_id: row.get::<_, i32>(24)?.into(),
        });
    }
    Ok(item_display_info::ItemDisplayInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemExtendedCost() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemExtendedCost (
        id INTEGER PRIMARY KEY NOT NULL,
        honor_points INTEGER  NOT NULL,
        arena_points INTEGER  NOT NULL,
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
        required_arena_rating INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ItemExtendedCost (
        id,
        honor_points,
        arena_points,
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
        required_arena_rating
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
    ,
    "SELECT
        id,
        honor_points,
        arena_points,
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
        required_arena_rating
    FROM `ItemExtendedCost`;"
    )
}


pub(crate) fn item_extended_cost_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_extended_cost::ItemExtendedCost, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_extended_cost::ItemExtendedCostRow {
            id: row.get::<_, i32>(0)?.into(),
            honor_points: row.get::<_, i32>(1)?.into(),
            arena_points: row.get::<_, i32>(2)?.into(),
            item_id: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(),             ],
            item_count: [row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(),             ],
            required_arena_rating: row.get::<_, i32>(13)?.into(),
        });
    }
    Ok(item_extended_cost::ItemExtendedCost { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemGroupSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_0,
        sound_1,
        sound_2,
        sound_3
    FROM `ItemGroupSounds`;"
    )
}


pub(crate) fn item_group_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_group_sounds::ItemGroupSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_group_sounds::ItemGroupSoundsRow {
            id: row.get::<_, i32>(0)?.into(),
            sound: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
        });
    }
    Ok(item_group_sounds::ItemGroupSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemPetFood() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemPetFood`;"
    )
}


pub(crate) fn item_pet_food_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_pet_food::ItemPetFood, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_pet_food::ItemPetFoodRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(item_pet_food::ItemPetFood { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemRandomProperties() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemRandomProperties`;"
    )
}


pub(crate) fn item_random_properties_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_random_properties::ItemRandomProperties, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_random_properties::ItemRandomPropertiesRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            enchantment: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(),             ],
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(7)?.into(),
                ko_kr: row.get::<_, String>(8)?.into(),
                fr_fr: row.get::<_, String>(9)?.into(),
                de_de: row.get::<_, String>(10)?.into(),
                en_cn: row.get::<_, String>(11)?.into(),
                en_tw: row.get::<_, String>(12)?.into(),
                es_es: row.get::<_, String>(13)?.into(),
                es_mx: row.get::<_, String>(14)?.into(),
                ru_ru: row.get::<_, String>(15)?.into(),
                ja_jp: row.get::<_, String>(16)?.into(),
                pt_pt: row.get::<_, String>(17)?.into(),
                it_it: row.get::<_, String>(18)?.into(),
                unknown_12: row.get::<_, String>(19)?.into(),
                unknown_13: row.get::<_, String>(20)?.into(),
                unknown_14: row.get::<_, String>(21)?.into(),
                unknown_15: row.get::<_, String>(22)?.into(),
                flags: row.get::<_, u32>(23)?.into(),
            },
        });
    }
    Ok(item_random_properties::ItemRandomProperties { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemRandomSuffix() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
        internal_name TEXT  NOT NULL,
        enchantment_0 INTEGER NOT NULL,
        enchantment_1 INTEGER NOT NULL,
        enchantment_2 INTEGER NOT NULL,
        allocation_pct_0 INTEGER NOT NULL,
        allocation_pct_1 INTEGER NOT NULL,
        allocation_pct_2 INTEGER NOT NULL
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
        allocation_pct_0,
        allocation_pct_1,
        allocation_pct_2
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
    ,
    "SELECT
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
        allocation_pct_0,
        allocation_pct_1,
        allocation_pct_2
    FROM `ItemRandomSuffix`;"
    )
}


pub(crate) fn item_random_suffix_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_random_suffix::ItemRandomSuffix, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_random_suffix::ItemRandomSuffixRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            internal_name: row.get::<_, String>(18)?.into(),
            enchantment: [row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(),             ],
            allocation_pct: [row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(),             ],
        });
    }
    Ok(item_random_suffix::ItemRandomSuffix { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemSet() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `ItemSet`;"
    )
}


pub(crate) fn item_set_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_set::ItemSet, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_set::ItemSetRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            item_id: [row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(), row.get::<_, i32>(33)?.into(), row.get::<_, i32>(34)?.into(),             ],
            set_spell_id: [row.get::<_, i32>(35)?.into(), row.get::<_, i32>(36)?.into(), row.get::<_, i32>(37)?.into(), row.get::<_, i32>(38)?.into(), row.get::<_, i32>(39)?.into(), row.get::<_, i32>(40)?.into(), row.get::<_, i32>(41)?.into(), row.get::<_, i32>(42)?.into(),             ],
            set_threshold: [row.get::<_, i32>(43)?.into(), row.get::<_, i32>(44)?.into(), row.get::<_, i32>(45)?.into(), row.get::<_, i32>(46)?.into(), row.get::<_, i32>(47)?.into(), row.get::<_, i32>(48)?.into(), row.get::<_, i32>(49)?.into(), row.get::<_, i32>(50)?.into(),             ],
            required_skill: row.get::<_, i32>(51)?.into(),
            required_skill_rank: row.get::<_, i32>(52)?.into(),
        });
    }
    Ok(item_set::ItemSet { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemSubClass() -> (&'static str, &'static str, &'static str) {
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
        display_name_lang_flags INTEGER NOT NULL,
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
        verbose_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemSubClass`;"
    )
}


pub(crate) fn item_sub_class_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_sub_class::ItemSubClass, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_sub_class::ItemSubClassRow {
            class_id: row.get::<_, i32>(0)?.into(),
            sub_class_id: row.get::<_, i32>(1)?.into(),
            prerequisite_proficiency: row.get::<_, i32>(2)?.into(),
            postrequisite_proficiency: row.get::<_, i32>(3)?.into(),
            flags: row.get::<_, i32>(4)?.into(),
            display_flags: row.get::<_, i32>(5)?.into(),
            weapon_parry_seq: row.get::<_, i32>(6)?.into(),
            weapon_ready_seq: row.get::<_, i32>(7)?.into(),
            weapon_attack_seq: row.get::<_, i32>(8)?.into(),
            weapon_swing_size: row.get::<_, i32>(9)?.into(),
            display_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(10)?.into(),
                ko_kr: row.get::<_, String>(11)?.into(),
                fr_fr: row.get::<_, String>(12)?.into(),
                de_de: row.get::<_, String>(13)?.into(),
                en_cn: row.get::<_, String>(14)?.into(),
                en_tw: row.get::<_, String>(15)?.into(),
                es_es: row.get::<_, String>(16)?.into(),
                es_mx: row.get::<_, String>(17)?.into(),
                ru_ru: row.get::<_, String>(18)?.into(),
                ja_jp: row.get::<_, String>(19)?.into(),
                pt_pt: row.get::<_, String>(20)?.into(),
                it_it: row.get::<_, String>(21)?.into(),
                unknown_12: row.get::<_, String>(22)?.into(),
                unknown_13: row.get::<_, String>(23)?.into(),
                unknown_14: row.get::<_, String>(24)?.into(),
                unknown_15: row.get::<_, String>(25)?.into(),
                flags: row.get::<_, u32>(26)?.into(),
            },
            verbose_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(27)?.into(),
                ko_kr: row.get::<_, String>(28)?.into(),
                fr_fr: row.get::<_, String>(29)?.into(),
                de_de: row.get::<_, String>(30)?.into(),
                en_cn: row.get::<_, String>(31)?.into(),
                en_tw: row.get::<_, String>(32)?.into(),
                es_es: row.get::<_, String>(33)?.into(),
                es_mx: row.get::<_, String>(34)?.into(),
                ru_ru: row.get::<_, String>(35)?.into(),
                ja_jp: row.get::<_, String>(36)?.into(),
                pt_pt: row.get::<_, String>(37)?.into(),
                it_it: row.get::<_, String>(38)?.into(),
                unknown_12: row.get::<_, String>(39)?.into(),
                unknown_13: row.get::<_, String>(40)?.into(),
                unknown_14: row.get::<_, String>(41)?.into(),
                unknown_15: row.get::<_, String>(42)?.into(),
                flags: row.get::<_, u32>(43)?.into(),
            },
        });
    }
    Ok(item_sub_class::ItemSubClass { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemSubClassMask() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemSubClassMask`;"
    )
}


pub(crate) fn item_sub_class_mask_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_sub_class_mask::ItemSubClassMask, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_sub_class_mask::ItemSubClassMaskRow {
            class_id: row.get::<_, i32>(0)?.into(),
            mask: row.get::<_, i32>(1)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                ru_ru: row.get::<_, String>(10)?.into(),
                ja_jp: row.get::<_, String>(11)?.into(),
                pt_pt: row.get::<_, String>(12)?.into(),
                it_it: row.get::<_, String>(13)?.into(),
                unknown_12: row.get::<_, String>(14)?.into(),
                unknown_13: row.get::<_, String>(15)?.into(),
                unknown_14: row.get::<_, String>(16)?.into(),
                unknown_15: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
        });
    }
    Ok(item_sub_class_mask::ItemSubClassMask { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemVisualEffects() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        model
    FROM `ItemVisualEffects`;"
    )
}


pub(crate) fn item_visual_effects_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_visual_effects::ItemVisualEffects, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_visual_effects::ItemVisualEffectsRow {
            id: row.get::<_, i32>(0)?.into(),
            model: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(item_visual_effects::ItemVisualEffects { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemVisuals() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        slot_0,
        slot_1,
        slot_2,
        slot_3,
        slot_4
    FROM `ItemVisuals`;"
    )
}


pub(crate) fn item_visuals_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_visuals::ItemVisuals, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_visuals::ItemVisualsRow {
            id: row.get::<_, i32>(0)?.into(),
            slot: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
        });
    }
    Ok(item_visuals::ItemVisuals { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LFGDungeons() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        type_id INTEGER  NOT NULL,
        faction INTEGER  NOT NULL,
        texture_filename TEXT  NOT NULL,
        expansion_level INTEGER  NOT NULL
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
        type_id,
        faction,
        texture_filename,
        expansion_level
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
    ,
    "SELECT
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
        type_id,
        faction,
        texture_filename,
        expansion_level
    FROM `LFGDungeons`;"
    )
}


pub(crate) fn lfg_dungeons_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<lfg_dungeons::LFGDungeons, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(lfg_dungeons::LFGDungeonsRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            min_level: row.get::<_, i32>(18)?.into(),
            max_level: row.get::<_, i32>(19)?.into(),
            type_id: row.get::<_, i32>(20)?.into(),
            faction: row.get::<_, i32>(21)?.into(),
            texture_filename: row.get::<_, String>(22)?.into(),
            expansion_level: row.get::<_, i32>(23)?.into(),
        });
    }
    Ok(lfg_dungeons::LFGDungeons { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LanguageWords() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        language_id,
        word
    FROM `LanguageWords`;"
    )
}


pub(crate) fn language_words_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<language_words::LanguageWords, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(language_words::LanguageWordsRow {
            id: row.get::<_, i32>(0)?.into(),
            language_id: row.get::<_, i32>(1)?.into(),
            word: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(language_words::LanguageWords { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Languages() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Languages`;"
    )
}


pub(crate) fn languages_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<languages::Languages, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(languages::LanguagesRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(languages::Languages { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Light() -> (&'static str, &'static str, &'static str) {
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
        light_params_id_4 INTEGER NOT NULL
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
        light_params_id_4
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
    ,
    "SELECT
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
        light_params_id_4
    FROM `Light`;"
    )
}


pub(crate) fn light_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light::Light, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light::LightRow {
            id: row.get::<_, i32>(0)?.into(),
            continent_id: row.get::<_, i32>(1)?.into(),
            game_coords: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(),             ],
            game_falloff_start: row.get::<_, f32>(5)?.into(),
            game_falloff_end: row.get::<_, f32>(6)?.into(),
            light_params_id: [row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(),             ],
        });
    }
    Ok(light::Light { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LightFloatBand() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `LightFloatBand`;"
    )
}


pub(crate) fn light_float_band_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light_float_band::LightFloatBand, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light_float_band::LightFloatBandRow {
            id: row.get::<_, i32>(0)?.into(),
            num: row.get::<_, i32>(1)?.into(),
            time: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(),             ],
            data: [row.get::<_, f32>(18)?.into(), row.get::<_, f32>(19)?.into(), row.get::<_, f32>(20)?.into(), row.get::<_, f32>(21)?.into(), row.get::<_, f32>(22)?.into(), row.get::<_, f32>(23)?.into(), row.get::<_, f32>(24)?.into(), row.get::<_, f32>(25)?.into(), row.get::<_, f32>(26)?.into(), row.get::<_, f32>(27)?.into(), row.get::<_, f32>(28)?.into(), row.get::<_, f32>(29)?.into(), row.get::<_, f32>(30)?.into(), row.get::<_, f32>(31)?.into(), row.get::<_, f32>(32)?.into(), row.get::<_, f32>(33)?.into(),             ],
        });
    }
    Ok(light_float_band::LightFloatBand { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LightIntBand() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `LightIntBand`;"
    )
}


pub(crate) fn light_int_band_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light_int_band::LightIntBand, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light_int_band::LightIntBandRow {
            id: row.get::<_, i32>(0)?.into(),
            num: row.get::<_, i32>(1)?.into(),
            time: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(),             ],
            data: [row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(), row.get::<_, i32>(33)?.into(),             ],
        });
    }
    Ok(light_int_band::LightIntBand { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LightParams() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        highlight_sky,
        light_skybox_id,
        glow,
        water_shallow_alpha,
        water_deep_alpha,
        ocean_shallow_alpha,
        ocean_deep_alpha,
        flags
    FROM `LightParams`;"
    )
}


pub(crate) fn light_params_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light_params::LightParams, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light_params::LightParamsRow {
            id: row.get::<_, i32>(0)?.into(),
            highlight_sky: row.get::<_, i32>(1)?.into(),
            light_skybox_id: row.get::<_, i32>(2)?.into(),
            glow: row.get::<_, f32>(3)?.into(),
            water_shallow_alpha: row.get::<_, f32>(4)?.into(),
            water_deep_alpha: row.get::<_, f32>(5)?.into(),
            ocean_shallow_alpha: row.get::<_, f32>(6)?.into(),
            ocean_deep_alpha: row.get::<_, f32>(7)?.into(),
            flags: row.get::<_, i32>(8)?.into(),
        });
    }
    Ok(light_params::LightParams { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LightSkybox() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        flags
    FROM `LightSkybox`;"
    )
}


pub(crate) fn light_skybox_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light_skybox::LightSkybox, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light_skybox::LightSkyboxRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            flags: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(light_skybox::LightSkybox { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LiquidType() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LiquidType (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        flags INTEGER  NOT NULL,
        spell_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LiquidType (
        id,
        name,
        flags,
        spell_id
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    ,
    "SELECT
        id,
        name,
        flags,
        spell_id
    FROM `LiquidType`;"
    )
}


pub(crate) fn liquid_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<liquid_type::LiquidType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(liquid_type::LiquidTypeRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            flags: row.get::<_, i32>(2)?.into(),
            spell_id: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(liquid_type::LiquidType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LoadingScreenTaxiSplines() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `LoadingScreenTaxiSplines`;"
    )
}


pub(crate) fn loading_screen_taxi_splines_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<loading_screen_taxi_splines::LoadingScreenTaxiSplines, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(loading_screen_taxi_splines::LoadingScreenTaxiSplinesRow {
            id: row.get::<_, i32>(0)?.into(),
            path_id: row.get::<_, i32>(1)?.into(),
            locx: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(), row.get::<_, f32>(6)?.into(), row.get::<_, f32>(7)?.into(), row.get::<_, f32>(8)?.into(), row.get::<_, f32>(9)?.into(),             ],
            locy: [row.get::<_, f32>(10)?.into(), row.get::<_, f32>(11)?.into(), row.get::<_, f32>(12)?.into(), row.get::<_, f32>(13)?.into(), row.get::<_, f32>(14)?.into(), row.get::<_, f32>(15)?.into(), row.get::<_, f32>(16)?.into(), row.get::<_, f32>(17)?.into(),             ],
            leg_index: row.get::<_, i32>(18)?.into(),
        });
    }
    Ok(loading_screen_taxi_splines::LoadingScreenTaxiSplines { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LoadingScreens() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LoadingScreens (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        file_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LoadingScreens (
        id,
        name,
        file_name
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    ,
    "SELECT
        id,
        name,
        file_name
    FROM `LoadingScreens`;"
    )
}


pub(crate) fn loading_screens_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<loading_screens::LoadingScreens, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(loading_screens::LoadingScreensRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            file_name: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(loading_screens::LoadingScreens { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Lock() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `Lock`;"
    )
}


pub(crate) fn lock_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<lock::Lock, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(lock::LockRow {
            id: row.get::<_, i32>(0)?.into(),
            ty: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(),             ],
            index: [row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(),             ],
            skill: [row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(),             ],
            action: [row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(),             ],
        });
    }
    Ok(lock::Lock { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LockType() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
        resource_name_lang_flags INTEGER NOT NULL,
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
        verb_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `LockType`;"
    )
}


pub(crate) fn lock_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<lock_type::LockType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(lock_type::LockTypeRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            resource_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(18)?.into(),
                ko_kr: row.get::<_, String>(19)?.into(),
                fr_fr: row.get::<_, String>(20)?.into(),
                de_de: row.get::<_, String>(21)?.into(),
                en_cn: row.get::<_, String>(22)?.into(),
                en_tw: row.get::<_, String>(23)?.into(),
                es_es: row.get::<_, String>(24)?.into(),
                es_mx: row.get::<_, String>(25)?.into(),
                ru_ru: row.get::<_, String>(26)?.into(),
                ja_jp: row.get::<_, String>(27)?.into(),
                pt_pt: row.get::<_, String>(28)?.into(),
                it_it: row.get::<_, String>(29)?.into(),
                unknown_12: row.get::<_, String>(30)?.into(),
                unknown_13: row.get::<_, String>(31)?.into(),
                unknown_14: row.get::<_, String>(32)?.into(),
                unknown_15: row.get::<_, String>(33)?.into(),
                flags: row.get::<_, u32>(34)?.into(),
            },
            verb_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(35)?.into(),
                ko_kr: row.get::<_, String>(36)?.into(),
                fr_fr: row.get::<_, String>(37)?.into(),
                de_de: row.get::<_, String>(38)?.into(),
                en_cn: row.get::<_, String>(39)?.into(),
                en_tw: row.get::<_, String>(40)?.into(),
                es_es: row.get::<_, String>(41)?.into(),
                es_mx: row.get::<_, String>(42)?.into(),
                ru_ru: row.get::<_, String>(43)?.into(),
                ja_jp: row.get::<_, String>(44)?.into(),
                pt_pt: row.get::<_, String>(45)?.into(),
                it_it: row.get::<_, String>(46)?.into(),
                unknown_12: row.get::<_, String>(47)?.into(),
                unknown_13: row.get::<_, String>(48)?.into(),
                unknown_14: row.get::<_, String>(49)?.into(),
                unknown_15: row.get::<_, String>(50)?.into(),
                flags: row.get::<_, u32>(51)?.into(),
            },
            cursor_name: row.get::<_, String>(52)?.into(),
        });
    }
    Ok(lock_type::LockType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn MailTemplate() -> (&'static str, &'static str, &'static str) {
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
        subject_lang_flags INTEGER NOT NULL,
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
        body_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `MailTemplate`;"
    )
}


pub(crate) fn mail_template_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<mail_template::MailTemplate, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(mail_template::MailTemplateRow {
            id: row.get::<_, i32>(0)?.into(),
            subject_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            body_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(18)?.into(),
                ko_kr: row.get::<_, String>(19)?.into(),
                fr_fr: row.get::<_, String>(20)?.into(),
                de_de: row.get::<_, String>(21)?.into(),
                en_cn: row.get::<_, String>(22)?.into(),
                en_tw: row.get::<_, String>(23)?.into(),
                es_es: row.get::<_, String>(24)?.into(),
                es_mx: row.get::<_, String>(25)?.into(),
                ru_ru: row.get::<_, String>(26)?.into(),
                ja_jp: row.get::<_, String>(27)?.into(),
                pt_pt: row.get::<_, String>(28)?.into(),
                it_it: row.get::<_, String>(29)?.into(),
                unknown_12: row.get::<_, String>(30)?.into(),
                unknown_13: row.get::<_, String>(31)?.into(),
                unknown_14: row.get::<_, String>(32)?.into(),
                unknown_15: row.get::<_, String>(33)?.into(),
                flags: row.get::<_, u32>(34)?.into(),
            },
        });
    }
    Ok(mail_template::MailTemplate { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Map() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Map (
        id INTEGER PRIMARY KEY NOT NULL,
        directory TEXT  NOT NULL,
        instance_type INTEGER  NOT NULL,
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
        map_name_lang_flags INTEGER NOT NULL,
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        max_players INTEGER  NOT NULL,
        field_0_7_0_3694_006 INTEGER  NOT NULL,
        field_0_7_0_3694_007 REAL  NOT NULL,
        field_0_7_0_3694_008 REAL  NOT NULL,
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
        map_description0_lang_flags INTEGER NOT NULL,
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
        map_description1_lang_flags INTEGER NOT NULL,
        loading_screen_id INTEGER  NOT NULL,
        field_1_5_0_4442_014 INTEGER  NOT NULL,
        field_1_7_0_4671_015 INTEGER  NOT NULL,
        minimap_icon_scale REAL  NOT NULL,
        field_2_0_0_5610_018_lang_en_gb TEXT NOT NULL,
        field_2_0_0_5610_018_lang_ko_kr TEXT NOT NULL,
        field_2_0_0_5610_018_lang_fr_fr TEXT NOT NULL,
        field_2_0_0_5610_018_lang_de_de TEXT NOT NULL,
        field_2_0_0_5610_018_lang_en_cn TEXT NOT NULL,
        field_2_0_0_5610_018_lang_en_tw TEXT NOT NULL,
        field_2_0_0_5610_018_lang_es_es TEXT NOT NULL,
        field_2_0_0_5610_018_lang_es_mx TEXT NOT NULL,
        field_2_0_0_5610_018_lang_ru_ru TEXT NOT NULL,
        field_2_0_0_5610_018_lang_ja_jp TEXT NOT NULL,
        field_2_0_0_5610_018_lang_pt_pt TEXT NOT NULL,
        field_2_0_0_5610_018_lang_it_it TEXT NOT NULL,
        field_2_0_0_5610_018_lang_unknown_12 TEXT NOT NULL,
        field_2_0_0_5610_018_lang_unknown_13 TEXT NOT NULL,
        field_2_0_0_5610_018_lang_unknown_14 TEXT NOT NULL,
        field_2_0_0_5610_018_lang_unknown_15 TEXT NOT NULL,
        field_2_0_0_5610_018_lang_flags INTEGER NOT NULL,
        field_2_0_0_5610_019_lang_en_gb TEXT NOT NULL,
        field_2_0_0_5610_019_lang_ko_kr TEXT NOT NULL,
        field_2_0_0_5610_019_lang_fr_fr TEXT NOT NULL,
        field_2_0_0_5610_019_lang_de_de TEXT NOT NULL,
        field_2_0_0_5610_019_lang_en_cn TEXT NOT NULL,
        field_2_0_0_5610_019_lang_en_tw TEXT NOT NULL,
        field_2_0_0_5610_019_lang_es_es TEXT NOT NULL,
        field_2_0_0_5610_019_lang_es_mx TEXT NOT NULL,
        field_2_0_0_5610_019_lang_ru_ru TEXT NOT NULL,
        field_2_0_0_5610_019_lang_ja_jp TEXT NOT NULL,
        field_2_0_0_5610_019_lang_pt_pt TEXT NOT NULL,
        field_2_0_0_5610_019_lang_it_it TEXT NOT NULL,
        field_2_0_0_5610_019_lang_unknown_12 TEXT NOT NULL,
        field_2_0_0_5610_019_lang_unknown_13 TEXT NOT NULL,
        field_2_0_0_5610_019_lang_unknown_14 TEXT NOT NULL,
        field_2_0_0_5610_019_lang_unknown_15 TEXT NOT NULL,
        field_2_0_0_5610_019_lang_flags INTEGER NOT NULL,
        field_2_0_0_5610_020_lang_en_gb TEXT NOT NULL,
        field_2_0_0_5610_020_lang_ko_kr TEXT NOT NULL,
        field_2_0_0_5610_020_lang_fr_fr TEXT NOT NULL,
        field_2_0_0_5610_020_lang_de_de TEXT NOT NULL,
        field_2_0_0_5610_020_lang_en_cn TEXT NOT NULL,
        field_2_0_0_5610_020_lang_en_tw TEXT NOT NULL,
        field_2_0_0_5610_020_lang_es_es TEXT NOT NULL,
        field_2_0_0_5610_020_lang_es_mx TEXT NOT NULL,
        field_2_0_0_5610_020_lang_ru_ru TEXT NOT NULL,
        field_2_0_0_5610_020_lang_ja_jp TEXT NOT NULL,
        field_2_0_0_5610_020_lang_pt_pt TEXT NOT NULL,
        field_2_0_0_5610_020_lang_it_it TEXT NOT NULL,
        field_2_0_0_5610_020_lang_unknown_12 TEXT NOT NULL,
        field_2_0_0_5610_020_lang_unknown_13 TEXT NOT NULL,
        field_2_0_0_5610_020_lang_unknown_14 TEXT NOT NULL,
        field_2_0_0_5610_020_lang_unknown_15 TEXT NOT NULL,
        field_2_0_0_5610_020_lang_flags INTEGER NOT NULL,
        corpse_map_id INTEGER  NOT NULL,
        corpse_0 REAL NOT NULL,
        corpse_1 REAL NOT NULL,
        field_2_0_3_6299_023 INTEGER  NOT NULL,
        field_2_0_3_6299_024 INTEGER  NOT NULL,
        field_2_0_3_6299_025 INTEGER  NOT NULL,
        time_of_day_override INTEGER  NOT NULL,
        expansion_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Map (
        id,
        directory,
        instance_type,
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
        min_level,
        max_level,
        max_players,
        field_0_7_0_3694_006,
        field_0_7_0_3694_007,
        field_0_7_0_3694_008,
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
        field_1_5_0_4442_014,
        field_1_7_0_4671_015,
        minimap_icon_scale,
        field_2_0_0_5610_018_lang_en_gb,
        field_2_0_0_5610_018_lang_ko_kr,
        field_2_0_0_5610_018_lang_fr_fr,
        field_2_0_0_5610_018_lang_de_de,
        field_2_0_0_5610_018_lang_en_cn,
        field_2_0_0_5610_018_lang_en_tw,
        field_2_0_0_5610_018_lang_es_es,
        field_2_0_0_5610_018_lang_es_mx,
        field_2_0_0_5610_018_lang_ru_ru,
        field_2_0_0_5610_018_lang_ja_jp,
        field_2_0_0_5610_018_lang_pt_pt,
        field_2_0_0_5610_018_lang_it_it,
        field_2_0_0_5610_018_lang_unknown_12,
        field_2_0_0_5610_018_lang_unknown_13,
        field_2_0_0_5610_018_lang_unknown_14,
        field_2_0_0_5610_018_lang_unknown_15,
        field_2_0_0_5610_018_lang_flags,
        field_2_0_0_5610_019_lang_en_gb,
        field_2_0_0_5610_019_lang_ko_kr,
        field_2_0_0_5610_019_lang_fr_fr,
        field_2_0_0_5610_019_lang_de_de,
        field_2_0_0_5610_019_lang_en_cn,
        field_2_0_0_5610_019_lang_en_tw,
        field_2_0_0_5610_019_lang_es_es,
        field_2_0_0_5610_019_lang_es_mx,
        field_2_0_0_5610_019_lang_ru_ru,
        field_2_0_0_5610_019_lang_ja_jp,
        field_2_0_0_5610_019_lang_pt_pt,
        field_2_0_0_5610_019_lang_it_it,
        field_2_0_0_5610_019_lang_unknown_12,
        field_2_0_0_5610_019_lang_unknown_13,
        field_2_0_0_5610_019_lang_unknown_14,
        field_2_0_0_5610_019_lang_unknown_15,
        field_2_0_0_5610_019_lang_flags,
        field_2_0_0_5610_020_lang_en_gb,
        field_2_0_0_5610_020_lang_ko_kr,
        field_2_0_0_5610_020_lang_fr_fr,
        field_2_0_0_5610_020_lang_de_de,
        field_2_0_0_5610_020_lang_en_cn,
        field_2_0_0_5610_020_lang_en_tw,
        field_2_0_0_5610_020_lang_es_es,
        field_2_0_0_5610_020_lang_es_mx,
        field_2_0_0_5610_020_lang_ru_ru,
        field_2_0_0_5610_020_lang_ja_jp,
        field_2_0_0_5610_020_lang_pt_pt,
        field_2_0_0_5610_020_lang_it_it,
        field_2_0_0_5610_020_lang_unknown_12,
        field_2_0_0_5610_020_lang_unknown_13,
        field_2_0_0_5610_020_lang_unknown_14,
        field_2_0_0_5610_020_lang_unknown_15,
        field_2_0_0_5610_020_lang_flags,
        corpse_map_id,
        corpse_0,
        corpse_1,
        field_2_0_3_6299_023,
        field_2_0_3_6299_024,
        field_2_0_3_6299_025,
        time_of_day_override,
        expansion_id
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
        ?125
    );"
    ,
    "SELECT
        id,
        directory,
        instance_type,
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
        min_level,
        max_level,
        max_players,
        field_0_7_0_3694_006,
        field_0_7_0_3694_007,
        field_0_7_0_3694_008,
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
        field_1_5_0_4442_014,
        field_1_7_0_4671_015,
        minimap_icon_scale,
        field_2_0_0_5610_018_lang_en_gb,
        field_2_0_0_5610_018_lang_ko_kr,
        field_2_0_0_5610_018_lang_fr_fr,
        field_2_0_0_5610_018_lang_de_de,
        field_2_0_0_5610_018_lang_en_cn,
        field_2_0_0_5610_018_lang_en_tw,
        field_2_0_0_5610_018_lang_es_es,
        field_2_0_0_5610_018_lang_es_mx,
        field_2_0_0_5610_018_lang_ru_ru,
        field_2_0_0_5610_018_lang_ja_jp,
        field_2_0_0_5610_018_lang_pt_pt,
        field_2_0_0_5610_018_lang_it_it,
        field_2_0_0_5610_018_lang_unknown_12,
        field_2_0_0_5610_018_lang_unknown_13,
        field_2_0_0_5610_018_lang_unknown_14,
        field_2_0_0_5610_018_lang_unknown_15,
        field_2_0_0_5610_018_lang_flags,
        field_2_0_0_5610_019_lang_en_gb,
        field_2_0_0_5610_019_lang_ko_kr,
        field_2_0_0_5610_019_lang_fr_fr,
        field_2_0_0_5610_019_lang_de_de,
        field_2_0_0_5610_019_lang_en_cn,
        field_2_0_0_5610_019_lang_en_tw,
        field_2_0_0_5610_019_lang_es_es,
        field_2_0_0_5610_019_lang_es_mx,
        field_2_0_0_5610_019_lang_ru_ru,
        field_2_0_0_5610_019_lang_ja_jp,
        field_2_0_0_5610_019_lang_pt_pt,
        field_2_0_0_5610_019_lang_it_it,
        field_2_0_0_5610_019_lang_unknown_12,
        field_2_0_0_5610_019_lang_unknown_13,
        field_2_0_0_5610_019_lang_unknown_14,
        field_2_0_0_5610_019_lang_unknown_15,
        field_2_0_0_5610_019_lang_flags,
        field_2_0_0_5610_020_lang_en_gb,
        field_2_0_0_5610_020_lang_ko_kr,
        field_2_0_0_5610_020_lang_fr_fr,
        field_2_0_0_5610_020_lang_de_de,
        field_2_0_0_5610_020_lang_en_cn,
        field_2_0_0_5610_020_lang_en_tw,
        field_2_0_0_5610_020_lang_es_es,
        field_2_0_0_5610_020_lang_es_mx,
        field_2_0_0_5610_020_lang_ru_ru,
        field_2_0_0_5610_020_lang_ja_jp,
        field_2_0_0_5610_020_lang_pt_pt,
        field_2_0_0_5610_020_lang_it_it,
        field_2_0_0_5610_020_lang_unknown_12,
        field_2_0_0_5610_020_lang_unknown_13,
        field_2_0_0_5610_020_lang_unknown_14,
        field_2_0_0_5610_020_lang_unknown_15,
        field_2_0_0_5610_020_lang_flags,
        corpse_map_id,
        corpse_0,
        corpse_1,
        field_2_0_3_6299_023,
        field_2_0_3_6299_024,
        field_2_0_3_6299_025,
        time_of_day_override,
        expansion_id
    FROM `Map`;"
    )
}


pub(crate) fn map_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<map::Map, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(map::MapRow {
            id: row.get::<_, i32>(0)?.into(),
            directory: row.get::<_, String>(1)?.into(),
            instance_type: row.get::<_, i32>(2)?.into(),
            p_v_p: row.get::<_, i32>(3)?.into(),
            map_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                ru_ru: row.get::<_, String>(12)?.into(),
                ja_jp: row.get::<_, String>(13)?.into(),
                pt_pt: row.get::<_, String>(14)?.into(),
                it_it: row.get::<_, String>(15)?.into(),
                unknown_12: row.get::<_, String>(16)?.into(),
                unknown_13: row.get::<_, String>(17)?.into(),
                unknown_14: row.get::<_, String>(18)?.into(),
                unknown_15: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
            min_level: row.get::<_, i32>(21)?.into(),
            max_level: row.get::<_, i32>(22)?.into(),
            max_players: row.get::<_, i32>(23)?.into(),
            field_0_7_0_3694_006: row.get::<_, i32>(24)?.into(),
            field_0_7_0_3694_007: row.get::<_, f32>(25)?.into(),
            field_0_7_0_3694_008: row.get::<_, f32>(26)?.into(),
            area_table_id: row.get::<_, i32>(27)?.into(),
            map_description0_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(28)?.into(),
                ko_kr: row.get::<_, String>(29)?.into(),
                fr_fr: row.get::<_, String>(30)?.into(),
                de_de: row.get::<_, String>(31)?.into(),
                en_cn: row.get::<_, String>(32)?.into(),
                en_tw: row.get::<_, String>(33)?.into(),
                es_es: row.get::<_, String>(34)?.into(),
                es_mx: row.get::<_, String>(35)?.into(),
                ru_ru: row.get::<_, String>(36)?.into(),
                ja_jp: row.get::<_, String>(37)?.into(),
                pt_pt: row.get::<_, String>(38)?.into(),
                it_it: row.get::<_, String>(39)?.into(),
                unknown_12: row.get::<_, String>(40)?.into(),
                unknown_13: row.get::<_, String>(41)?.into(),
                unknown_14: row.get::<_, String>(42)?.into(),
                unknown_15: row.get::<_, String>(43)?.into(),
                flags: row.get::<_, u32>(44)?.into(),
            },
            map_description1_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(45)?.into(),
                ko_kr: row.get::<_, String>(46)?.into(),
                fr_fr: row.get::<_, String>(47)?.into(),
                de_de: row.get::<_, String>(48)?.into(),
                en_cn: row.get::<_, String>(49)?.into(),
                en_tw: row.get::<_, String>(50)?.into(),
                es_es: row.get::<_, String>(51)?.into(),
                es_mx: row.get::<_, String>(52)?.into(),
                ru_ru: row.get::<_, String>(53)?.into(),
                ja_jp: row.get::<_, String>(54)?.into(),
                pt_pt: row.get::<_, String>(55)?.into(),
                it_it: row.get::<_, String>(56)?.into(),
                unknown_12: row.get::<_, String>(57)?.into(),
                unknown_13: row.get::<_, String>(58)?.into(),
                unknown_14: row.get::<_, String>(59)?.into(),
                unknown_15: row.get::<_, String>(60)?.into(),
                flags: row.get::<_, u32>(61)?.into(),
            },
            loading_screen_id: row.get::<_, i32>(62)?.into(),
            field_1_5_0_4442_014: row.get::<_, i32>(63)?.into(),
            field_1_7_0_4671_015: row.get::<_, i32>(64)?.into(),
            minimap_icon_scale: row.get::<_, f32>(65)?.into(),
            field_2_0_0_5610_018_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(66)?.into(),
                ko_kr: row.get::<_, String>(67)?.into(),
                fr_fr: row.get::<_, String>(68)?.into(),
                de_de: row.get::<_, String>(69)?.into(),
                en_cn: row.get::<_, String>(70)?.into(),
                en_tw: row.get::<_, String>(71)?.into(),
                es_es: row.get::<_, String>(72)?.into(),
                es_mx: row.get::<_, String>(73)?.into(),
                ru_ru: row.get::<_, String>(74)?.into(),
                ja_jp: row.get::<_, String>(75)?.into(),
                pt_pt: row.get::<_, String>(76)?.into(),
                it_it: row.get::<_, String>(77)?.into(),
                unknown_12: row.get::<_, String>(78)?.into(),
                unknown_13: row.get::<_, String>(79)?.into(),
                unknown_14: row.get::<_, String>(80)?.into(),
                unknown_15: row.get::<_, String>(81)?.into(),
                flags: row.get::<_, u32>(82)?.into(),
            },
            field_2_0_0_5610_019_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(83)?.into(),
                ko_kr: row.get::<_, String>(84)?.into(),
                fr_fr: row.get::<_, String>(85)?.into(),
                de_de: row.get::<_, String>(86)?.into(),
                en_cn: row.get::<_, String>(87)?.into(),
                en_tw: row.get::<_, String>(88)?.into(),
                es_es: row.get::<_, String>(89)?.into(),
                es_mx: row.get::<_, String>(90)?.into(),
                ru_ru: row.get::<_, String>(91)?.into(),
                ja_jp: row.get::<_, String>(92)?.into(),
                pt_pt: row.get::<_, String>(93)?.into(),
                it_it: row.get::<_, String>(94)?.into(),
                unknown_12: row.get::<_, String>(95)?.into(),
                unknown_13: row.get::<_, String>(96)?.into(),
                unknown_14: row.get::<_, String>(97)?.into(),
                unknown_15: row.get::<_, String>(98)?.into(),
                flags: row.get::<_, u32>(99)?.into(),
            },
            field_2_0_0_5610_020_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(100)?.into(),
                ko_kr: row.get::<_, String>(101)?.into(),
                fr_fr: row.get::<_, String>(102)?.into(),
                de_de: row.get::<_, String>(103)?.into(),
                en_cn: row.get::<_, String>(104)?.into(),
                en_tw: row.get::<_, String>(105)?.into(),
                es_es: row.get::<_, String>(106)?.into(),
                es_mx: row.get::<_, String>(107)?.into(),
                ru_ru: row.get::<_, String>(108)?.into(),
                ja_jp: row.get::<_, String>(109)?.into(),
                pt_pt: row.get::<_, String>(110)?.into(),
                it_it: row.get::<_, String>(111)?.into(),
                unknown_12: row.get::<_, String>(112)?.into(),
                unknown_13: row.get::<_, String>(113)?.into(),
                unknown_14: row.get::<_, String>(114)?.into(),
                unknown_15: row.get::<_, String>(115)?.into(),
                flags: row.get::<_, u32>(116)?.into(),
            },
            corpse_map_id: row.get::<_, i32>(117)?.into(),
            corpse: [row.get::<_, f32>(118)?.into(), row.get::<_, f32>(119)?.into(),             ],
            field_2_0_3_6299_023: row.get::<_, i32>(120)?.into(),
            field_2_0_3_6299_024: row.get::<_, i32>(121)?.into(),
            field_2_0_3_6299_025: row.get::<_, i32>(122)?.into(),
            time_of_day_override: row.get::<_, i32>(123)?.into(),
            expansion_id: row.get::<_, i32>(124)?.into(),
        });
    }
    Ok(map::Map { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Material() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        flags,
        foley_sound_id,
        sheathe_sound_id,
        unsheathe_sound_id
    FROM `Material`;"
    )
}


pub(crate) fn material_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<material::Material, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(material::MaterialRow {
            id: row.get::<_, i32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            foley_sound_id: row.get::<_, i32>(2)?.into(),
            sheathe_sound_id: row.get::<_, i32>(3)?.into(),
            unsheathe_sound_id: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(material::Material { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NPCSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_id_0,
        sound_id_1,
        sound_id_2,
        sound_id_3
    FROM `NPCSounds`;"
    )
}


pub(crate) fn npc_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<npc_sounds::NPCSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(npc_sounds::NPCSoundsRow {
            id: row.get::<_, i32>(0)?.into(),
            sound_id: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
        });
    }
    Ok(npc_sounds::NPCSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NameGen() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        race_id,
        sex
    FROM `NameGen`;"
    )
}


pub(crate) fn name_gen_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<name_gen::NameGen, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(name_gen::NameGenRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            race_id: row.get::<_, i32>(2)?.into(),
            sex: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(name_gen::NameGen { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NamesProfanity() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        language
    FROM `NamesProfanity`;"
    )
}


pub(crate) fn names_profanity_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<names_profanity::NamesProfanity, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(names_profanity::NamesProfanityRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            language: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(names_profanity::NamesProfanity { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NamesReserved() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        language
    FROM `NamesReserved`;"
    )
}


pub(crate) fn names_reserved_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<names_reserved::NamesReserved, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(names_reserved::NamesReservedRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            language: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(names_reserved::NamesReserved { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Package() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Package`;"
    )
}


pub(crate) fn package_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<package::Package, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(package::PackageRow {
            id: row.get::<_, i32>(0)?.into(),
            icon: row.get::<_, String>(1)?.into(),
            cost: row.get::<_, i32>(2)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                ru_ru: row.get::<_, String>(11)?.into(),
                ja_jp: row.get::<_, String>(12)?.into(),
                pt_pt: row.get::<_, String>(13)?.into(),
                it_it: row.get::<_, String>(14)?.into(),
                unknown_12: row.get::<_, String>(15)?.into(),
                unknown_13: row.get::<_, String>(16)?.into(),
                unknown_14: row.get::<_, String>(17)?.into(),
                unknown_15: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
        });
    }
    Ok(package::Package { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn PageTextMaterial() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name
    FROM `PageTextMaterial`;"
    )
}


pub(crate) fn page_text_material_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<page_text_material::PageTextMaterial, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(page_text_material::PageTextMaterialRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(page_text_material::PageTextMaterial { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn PaperDollItemFrame() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        item_button_name,
        slot_icon,
        slot_number
    FROM `PaperDollItemFrame`;"
    )
}


pub(crate) fn paper_doll_item_frame_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<paper_doll_item_frame::PaperDollItemFrame, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(paper_doll_item_frame::PaperDollItemFrameRow {
            item_button_name: row.get::<_, String>(0)?.into(),
            slot_icon: row.get::<_, String>(1)?.into(),
            slot_number: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(paper_doll_item_frame::PaperDollItemFrame { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ParticleColor() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `ParticleColor`;"
    )
}


pub(crate) fn particle_color_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<particle_color::ParticleColor, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(particle_color::ParticleColorRow {
            id: row.get::<_, i32>(0)?.into(),
            start: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
            m_id: [row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(),             ],
            end: [row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
        });
    }
    Ok(particle_color::ParticleColor { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn PetLoyalty() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PetLoyalty (
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
        name_lang_flags INTEGER NOT NULL
    );"
    ,
    "INSERT INTO PetLoyalty (
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
    ,
    "SELECT
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
    FROM `PetLoyalty`;"
    )
}


pub(crate) fn pet_loyalty_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<pet_loyalty::PetLoyalty, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(pet_loyalty::PetLoyaltyRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(pet_loyalty::PetLoyalty { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn PetPersonality() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
        happiness_threshold_0 INTEGER NOT NULL,
        happiness_threshold_1 INTEGER NOT NULL,
        happiness_threshold_2 INTEGER NOT NULL,
        happiness_damage_0 REAL NOT NULL,
        happiness_damage_1 REAL NOT NULL,
        happiness_damage_2 REAL NOT NULL,
        damage_modifier_0 REAL NOT NULL,
        damage_modifier_1 REAL NOT NULL,
        damage_modifier_2 REAL NOT NULL
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
        happiness_damage_2,
        damage_modifier_0,
        damage_modifier_1,
        damage_modifier_2
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
        ?27
    );"
    ,
    "SELECT
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
        happiness_damage_2,
        damage_modifier_0,
        damage_modifier_1,
        damage_modifier_2
    FROM `PetPersonality`;"
    )
}


pub(crate) fn pet_personality_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<pet_personality::PetPersonality, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(pet_personality::PetPersonalityRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            happiness_threshold: [row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(),             ],
            happiness_damage: [row.get::<_, f32>(21)?.into(), row.get::<_, f32>(22)?.into(), row.get::<_, f32>(23)?.into(),             ],
            damage_modifier: [row.get::<_, f32>(24)?.into(), row.get::<_, f32>(25)?.into(), row.get::<_, f32>(26)?.into(),             ],
        });
    }
    Ok(pet_personality::PetPersonality { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn PetitionType() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        ty
    FROM `PetitionType`;"
    )
}


pub(crate) fn petition_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<petition_type::PetitionType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(petition_type::PetitionTypeRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            ty: row.get::<_, i32>(2)?.into(),
        });
    }
    Ok(petition_type::PetitionType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn QuestInfo() -> (&'static str, &'static str, &'static str) {
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
        info_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `QuestInfo`;"
    )
}


pub(crate) fn quest_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<quest_info::QuestInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(quest_info::QuestInfoRow {
            id: row.get::<_, i32>(0)?.into(),
            info_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(quest_info::QuestInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn QuestSort() -> (&'static str, &'static str, &'static str) {
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
        sort_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `QuestSort`;"
    )
}


pub(crate) fn quest_sort_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<quest_sort::QuestSort, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(quest_sort::QuestSortRow {
            id: row.get::<_, i32>(0)?.into(),
            sort_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(quest_sort::QuestSort { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn RandPropPoints() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `RandPropPoints`;"
    )
}


pub(crate) fn rand_prop_points_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<rand_prop_points::RandPropPoints, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(rand_prop_points::RandPropPointsRow {
            id: row.get::<_, i32>(0)?.into(),
            epic: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
            superior: [row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(),             ],
            good: [row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(),             ],
        });
    }
    Ok(rand_prop_points::RandPropPoints { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Resistances() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Resistances`;"
    )
}


pub(crate) fn resistances_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<resistances::Resistances, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(resistances::ResistancesRow {
            id: row.get::<_, i32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            fizzle_sound_id: row.get::<_, i32>(2)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                ru_ru: row.get::<_, String>(11)?.into(),
                ja_jp: row.get::<_, String>(12)?.into(),
                pt_pt: row.get::<_, String>(13)?.into(),
                it_it: row.get::<_, String>(14)?.into(),
                unknown_12: row.get::<_, String>(15)?.into(),
                unknown_13: row.get::<_, String>(16)?.into(),
                unknown_14: row.get::<_, String>(17)?.into(),
                unknown_15: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
        });
    }
    Ok(resistances::Resistances { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ServerMessages() -> (&'static str, &'static str, &'static str) {
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
        text_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ServerMessages`;"
    )
}


pub(crate) fn server_messages_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<server_messages::ServerMessages, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(server_messages::ServerMessagesRow {
            id: row.get::<_, i32>(0)?.into(),
            text_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(server_messages::ServerMessages { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SheatheSoundLookups() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        class_id,
        subclass_id,
        material,
        check_material,
        sheathe_sound,
        unsheathe_sound
    FROM `SheatheSoundLookups`;"
    )
}


pub(crate) fn sheathe_sound_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sheathe_sound_lookups::SheatheSoundLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sheathe_sound_lookups::SheatheSoundLookupsRow {
            id: row.get::<_, i32>(0)?.into(),
            class_id: row.get::<_, i32>(1)?.into(),
            subclass_id: row.get::<_, i32>(2)?.into(),
            material: row.get::<_, i32>(3)?.into(),
            check_material: row.get::<_, i32>(4)?.into(),
            sheathe_sound: row.get::<_, i32>(5)?.into(),
            unsheathe_sound: row.get::<_, i32>(6)?.into(),
        });
    }
    Ok(sheathe_sound_lookups::SheatheSoundLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillCostsData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        skill_costs_id,
        cost_0,
        cost_1,
        cost_2
    FROM `SkillCostsData`;"
    )
}


pub(crate) fn skill_costs_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_costs_data::SkillCostsData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_costs_data::SkillCostsDataRow {
            id: row.get::<_, i32>(0)?.into(),
            skill_costs_id: row.get::<_, i32>(1)?.into(),
            cost: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
        });
    }
    Ok(skill_costs_data::SkillCostsData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillLine() -> (&'static str, &'static str, &'static str) {
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
        display_name_lang_flags INTEGER NOT NULL,
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
        description_lang_flags INTEGER NOT NULL,
        spell_icon_id INTEGER  NOT NULL
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
    ,
    "SELECT
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
        spell_icon_id
    FROM `SkillLine`;"
    )
}


pub(crate) fn skill_line_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_line::SkillLine, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_line::SkillLineRow {
            id: row.get::<_, i32>(0)?.into(),
            category_id: row.get::<_, i32>(1)?.into(),
            skill_costs_id: row.get::<_, i32>(2)?.into(),
            display_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                ru_ru: row.get::<_, String>(11)?.into(),
                ja_jp: row.get::<_, String>(12)?.into(),
                pt_pt: row.get::<_, String>(13)?.into(),
                it_it: row.get::<_, String>(14)?.into(),
                unknown_12: row.get::<_, String>(15)?.into(),
                unknown_13: row.get::<_, String>(16)?.into(),
                unknown_14: row.get::<_, String>(17)?.into(),
                unknown_15: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
            description_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(20)?.into(),
                ko_kr: row.get::<_, String>(21)?.into(),
                fr_fr: row.get::<_, String>(22)?.into(),
                de_de: row.get::<_, String>(23)?.into(),
                en_cn: row.get::<_, String>(24)?.into(),
                en_tw: row.get::<_, String>(25)?.into(),
                es_es: row.get::<_, String>(26)?.into(),
                es_mx: row.get::<_, String>(27)?.into(),
                ru_ru: row.get::<_, String>(28)?.into(),
                ja_jp: row.get::<_, String>(29)?.into(),
                pt_pt: row.get::<_, String>(30)?.into(),
                it_it: row.get::<_, String>(31)?.into(),
                unknown_12: row.get::<_, String>(32)?.into(),
                unknown_13: row.get::<_, String>(33)?.into(),
                unknown_14: row.get::<_, String>(34)?.into(),
                unknown_15: row.get::<_, String>(35)?.into(),
                flags: row.get::<_, u32>(36)?.into(),
            },
            spell_icon_id: row.get::<_, i32>(37)?.into(),
        });
    }
    Ok(skill_line::SkillLine { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillLineAbility() -> (&'static str, &'static str, &'static str) {
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
        abandonable INTEGER  NOT NULL,
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
        abandonable,
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
        ?14,
        ?15
    );"
    ,
    "SELECT
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
        abandonable,
        character_points_0,
        character_points_1
    FROM `SkillLineAbility`;"
    )
}


pub(crate) fn skill_line_ability_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_line_ability::SkillLineAbility, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_line_ability::SkillLineAbilityRow {
            id: row.get::<_, i32>(0)?.into(),
            skill_line: row.get::<_, i32>(1)?.into(),
            spell: row.get::<_, i32>(2)?.into(),
            race_mask: row.get::<_, i32>(3)?.into(),
            class_mask: row.get::<_, i32>(4)?.into(),
            exclude_race: row.get::<_, i32>(5)?.into(),
            exclude_class: row.get::<_, i32>(6)?.into(),
            min_skill_line_rank: row.get::<_, i32>(7)?.into(),
            superceded_by_spell: row.get::<_, i32>(8)?.into(),
            acquire_method: row.get::<_, i32>(9)?.into(),
            trivial_skill_line_rank_high: row.get::<_, i32>(10)?.into(),
            trivial_skill_line_rank_low: row.get::<_, i32>(11)?.into(),
            abandonable: row.get::<_, i32>(12)?.into(),
            character_points: [row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(),             ],
        });
    }
    Ok(skill_line_ability::SkillLineAbility { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillLineCategory() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SkillLineCategory`;"
    )
}


pub(crate) fn skill_line_category_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_line_category::SkillLineCategory, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_line_category::SkillLineCategoryRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            sort_index: row.get::<_, i32>(18)?.into(),
        });
    }
    Ok(skill_line_category::SkillLineCategory { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillRaceClassInfo() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        skill_id,
        race_mask,
        class_mask,
        flags,
        min_level,
        skill_tier_id,
        skill_cost_index
    FROM `SkillRaceClassInfo`;"
    )
}


pub(crate) fn skill_race_class_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_race_class_info::SkillRaceClassInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_race_class_info::SkillRaceClassInfoRow {
            id: row.get::<_, i32>(0)?.into(),
            skill_id: row.get::<_, i32>(1)?.into(),
            race_mask: row.get::<_, i32>(2)?.into(),
            class_mask: row.get::<_, i32>(3)?.into(),
            flags: row.get::<_, i32>(4)?.into(),
            min_level: row.get::<_, i32>(5)?.into(),
            skill_tier_id: row.get::<_, i32>(6)?.into(),
            skill_cost_index: row.get::<_, i32>(7)?.into(),
        });
    }
    Ok(skill_race_class_info::SkillRaceClassInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillTiers() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `SkillTiers`;"
    )
}


pub(crate) fn skill_tiers_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_tiers::SkillTiers, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_tiers::SkillTiersRow {
            id: row.get::<_, i32>(0)?.into(),
            cost: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(),             ],
            value: [row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(),             ],
        });
    }
    Ok(skill_tiers::SkillTiers { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundAmbience() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        ambience_id_0,
        ambience_id_1
    FROM `SoundAmbience`;"
    )
}


pub(crate) fn sound_ambience_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_ambience::SoundAmbience, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_ambience::SoundAmbienceRow {
            id: row.get::<_, i32>(0)?.into(),
            ambience_id: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(),             ],
        });
    }
    Ok(sound_ambience::SoundAmbience { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundEntries() -> (&'static str, &'static str, &'static str) {
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
        e_a_x_def INTEGER  NOT NULL
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
        e_a_x_def
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
    ,
    "SELECT
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
        e_a_x_def
    FROM `SoundEntries`;"
    )
}


pub(crate) fn sound_entries_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_entries::SoundEntries, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_entries::SoundEntriesRow {
            id: row.get::<_, i32>(0)?.into(),
            sound_type: row.get::<_, i32>(1)?.into(),
            name: row.get::<_, String>(2)?.into(),
            file: [row.get::<_, String>(3)?.into(), row.get::<_, String>(4)?.into(), row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(), row.get::<_, String>(9)?.into(), row.get::<_, String>(10)?.into(), row.get::<_, String>(11)?.into(), row.get::<_, String>(12)?.into(),             ],
            freq: [row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(),             ],
            directory_base: row.get::<_, String>(23)?.into(),
            volume_float: row.get::<_, f32>(24)?.into(),
            flags: row.get::<_, i32>(25)?.into(),
            min_distance: row.get::<_, f32>(26)?.into(),
            distance_cutoff: row.get::<_, f32>(27)?.into(),
            e_a_x_def: row.get::<_, i32>(28)?.into(),
        });
    }
    Ok(sound_entries::SoundEntries { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundProviderPreferences() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `SoundProviderPreferences`;"
    )
}


pub(crate) fn sound_provider_preferences_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_provider_preferences::SoundProviderPreferences, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_provider_preferences::SoundProviderPreferencesRow {
            id: row.get::<_, i32>(0)?.into(),
            description: row.get::<_, String>(1)?.into(),
            flags: row.get::<_, i32>(2)?.into(),
            e_a_x_environment_selection: row.get::<_, i32>(3)?.into(),
            e_a_x_decay_time: row.get::<_, f32>(4)?.into(),
            e_a_x2_environment_size: row.get::<_, f32>(5)?.into(),
            e_a_x2_environment_diffusion: row.get::<_, f32>(6)?.into(),
            e_a_x2_room: row.get::<_, i32>(7)?.into(),
            e_a_x2_room_h_f: row.get::<_, i32>(8)?.into(),
            e_a_x2_decay_h_f_ratio: row.get::<_, f32>(9)?.into(),
            e_a_x2_reflections: row.get::<_, i32>(10)?.into(),
            e_a_x2_reflections_delay: row.get::<_, f32>(11)?.into(),
            e_a_x2_reverb: row.get::<_, i32>(12)?.into(),
            e_a_x2_reverb_delay: row.get::<_, f32>(13)?.into(),
            e_a_x2_room_rolloff: row.get::<_, f32>(14)?.into(),
            e_a_x2_air_absorption: row.get::<_, f32>(15)?.into(),
            e_a_x3_room_l_f: row.get::<_, i32>(16)?.into(),
            e_a_x3_decay_l_f_ratio: row.get::<_, f32>(17)?.into(),
            e_a_x3_echo_time: row.get::<_, f32>(18)?.into(),
            e_a_x3_echo_depth: row.get::<_, f32>(19)?.into(),
            e_a_x3_modulation_time: row.get::<_, f32>(20)?.into(),
            e_a_x3_modulation_depth: row.get::<_, f32>(21)?.into(),
            e_a_x3_h_f_reference: row.get::<_, f32>(22)?.into(),
            e_a_x3_l_f_reference: row.get::<_, f32>(23)?.into(),
        });
    }
    Ok(sound_provider_preferences::SoundProviderPreferences { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundSamplePreferences() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `SoundSamplePreferences`;"
    )
}


pub(crate) fn sound_sample_preferences_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_sample_preferences::SoundSamplePreferences, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_sample_preferences::SoundSamplePreferencesRow {
            id: row.get::<_, i32>(0)?.into(),
            field_0_6_0_3592_001: row.get::<_, i32>(1)?.into(),
            field_0_6_0_3592_002: row.get::<_, i32>(2)?.into(),
            e_a_x2_sample_room: row.get::<_, i32>(3)?.into(),
            field_0_6_0_3592_004: row.get::<_, i32>(4)?.into(),
            field_0_6_0_3592_005: row.get::<_, i32>(5)?.into(),
            field_0_6_0_3592_006: row.get::<_, f32>(6)?.into(),
            field_0_6_0_3592_007: row.get::<_, i32>(7)?.into(),
            e_a_x2_sample_occlusion_l_f_ratio: row.get::<_, f32>(8)?.into(),
            e_a_x2_sample_occlusion_room_ratio: row.get::<_, f32>(9)?.into(),
            field_0_6_0_3592_010: row.get::<_, i32>(10)?.into(),
            e_a_x1_effect_level: row.get::<_, f32>(11)?.into(),
            field_0_6_0_3592_012: row.get::<_, i32>(12)?.into(),
            field_0_6_0_3592_013: row.get::<_, f32>(13)?.into(),
            e_a_x3_sample_exclusion: row.get::<_, f32>(14)?.into(),
            field_0_6_0_3592_015: row.get::<_, f32>(15)?.into(),
            field_0_6_0_3592_016: row.get::<_, i32>(16)?.into(),
        });
    }
    Ok(sound_sample_preferences::SoundSamplePreferences { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundWaterType() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_type,
        sound_subtype,
        sound_id
    FROM `SoundWaterType`;"
    )
}


pub(crate) fn sound_water_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_water_type::SoundWaterType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_water_type::SoundWaterTypeRow {
            id: row.get::<_, i32>(0)?.into(),
            sound_type: row.get::<_, i32>(1)?.into(),
            sound_subtype: row.get::<_, i32>(2)?.into(),
            sound_id: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(sound_water_type::SoundWaterType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpamMessages() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        text
    FROM `SpamMessages`;"
    )
}


pub(crate) fn spam_messages_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spam_messages::SpamMessages, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spam_messages::SpamMessagesRow {
            id: row.get::<_, i32>(0)?.into(),
            text: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(spam_messages::SpamMessages { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Spell() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Spell (
        id INTEGER PRIMARY KEY NOT NULL,
        category INTEGER  NOT NULL,
        cast_u_i INTEGER  NOT NULL,
        dispel_type INTEGER  NOT NULL,
        mechanic INTEGER  NOT NULL,
        attributes INTEGER  NOT NULL,
        attributes_ex INTEGER  NOT NULL,
        attributes_ex_b INTEGER  NOT NULL,
        attributes_ex_c INTEGER  NOT NULL,
        attributes_ex_d INTEGER  NOT NULL,
        attributes_ex_e INTEGER  NOT NULL,
        attributes_ex_f INTEGER  NOT NULL,
        shapeshift_mask INTEGER  NOT NULL,
        shapeshift_exclude INTEGER  NOT NULL,
        targets INTEGER  NOT NULL,
        target_creature_type INTEGER  NOT NULL,
        requires_spell_focus INTEGER  NOT NULL,
        facing_caster_flags INTEGER  NOT NULL,
        caster_aura_state INTEGER  NOT NULL,
        target_aura_state INTEGER  NOT NULL,
        exclude_caster_aura_state INTEGER  NOT NULL,
        exclude_target_aura_state INTEGER  NOT NULL,
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
        effect_base_dice_0 INTEGER NOT NULL,
        effect_base_dice_1 INTEGER NOT NULL,
        effect_base_dice_2 INTEGER NOT NULL,
        effect_dice_per_level_0 INTEGER NOT NULL,
        effect_dice_per_level_1 INTEGER NOT NULL,
        effect_dice_per_level_2 INTEGER NOT NULL,
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
        name_lang_flags INTEGER NOT NULL,
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
        name_subtext_lang_flags INTEGER NOT NULL,
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
        description_lang_flags INTEGER NOT NULL,
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
        aura_description_lang_flags INTEGER NOT NULL,
        mana_cost_pct INTEGER  NOT NULL,
        start_recovery_category INTEGER  NOT NULL,
        start_recovery_time INTEGER  NOT NULL,
        max_target_level INTEGER  NOT NULL,
        spell_class_set INTEGER  NOT NULL,
        spell_class_mask_0 INTEGER NOT NULL,
        spell_class_mask_1 INTEGER NOT NULL,
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
        school_mask INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Spell (
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
        effect_base_dice_0,
        effect_base_dice_1,
        effect_base_dice_2,
        effect_dice_per_level_0,
        effect_dice_per_level_1,
        effect_dice_per_level_2,
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
        school_mask
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
        ?216
    );"
    ,
    "SELECT
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
        effect_base_dice_0,
        effect_base_dice_1,
        effect_base_dice_2,
        effect_dice_per_level_0,
        effect_dice_per_level_1,
        effect_dice_per_level_2,
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
        school_mask
    FROM `Spell`;"
    )
}


pub(crate) fn spell_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell::Spell, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell::SpellRow {
            id: row.get::<_, i32>(0)?.into(),
            category: row.get::<_, i32>(1)?.into(),
            cast_u_i: row.get::<_, i32>(2)?.into(),
            dispel_type: row.get::<_, i32>(3)?.into(),
            mechanic: row.get::<_, i32>(4)?.into(),
            attributes: row.get::<_, i32>(5)?.into(),
            attributes_ex: row.get::<_, i32>(6)?.into(),
            attributes_ex_b: row.get::<_, i32>(7)?.into(),
            attributes_ex_c: row.get::<_, i32>(8)?.into(),
            attributes_ex_d: row.get::<_, i32>(9)?.into(),
            attributes_ex_e: row.get::<_, i32>(10)?.into(),
            attributes_ex_f: row.get::<_, i32>(11)?.into(),
            shapeshift_mask: row.get::<_, i32>(12)?.into(),
            shapeshift_exclude: row.get::<_, i32>(13)?.into(),
            targets: row.get::<_, i32>(14)?.into(),
            target_creature_type: row.get::<_, i32>(15)?.into(),
            requires_spell_focus: row.get::<_, i32>(16)?.into(),
            facing_caster_flags: row.get::<_, i32>(17)?.into(),
            caster_aura_state: row.get::<_, i32>(18)?.into(),
            target_aura_state: row.get::<_, i32>(19)?.into(),
            exclude_caster_aura_state: row.get::<_, i32>(20)?.into(),
            exclude_target_aura_state: row.get::<_, i32>(21)?.into(),
            casting_time_index: row.get::<_, i32>(22)?.into(),
            recovery_time: row.get::<_, i32>(23)?.into(),
            category_recovery_time: row.get::<_, i32>(24)?.into(),
            interrupt_flags: row.get::<_, i32>(25)?.into(),
            aura_interrupt_flags: row.get::<_, i32>(26)?.into(),
            channel_interrupt_flags: row.get::<_, i32>(27)?.into(),
            proc_type_mask: row.get::<_, i32>(28)?.into(),
            proc_chance: row.get::<_, i32>(29)?.into(),
            proc_charges: row.get::<_, i32>(30)?.into(),
            max_level: row.get::<_, i32>(31)?.into(),
            base_level: row.get::<_, i32>(32)?.into(),
            spell_level: row.get::<_, i32>(33)?.into(),
            duration_index: row.get::<_, i32>(34)?.into(),
            power_type: row.get::<_, i32>(35)?.into(),
            mana_cost: row.get::<_, i32>(36)?.into(),
            mana_cost_per_level: row.get::<_, i32>(37)?.into(),
            mana_per_second: row.get::<_, i32>(38)?.into(),
            mana_per_second_per_level: row.get::<_, i32>(39)?.into(),
            range_index: row.get::<_, i32>(40)?.into(),
            speed: row.get::<_, f32>(41)?.into(),
            modal_next_spell: row.get::<_, i32>(42)?.into(),
            cumulative_aura: row.get::<_, i32>(43)?.into(),
            totem: [row.get::<_, i32>(44)?.into(), row.get::<_, i32>(45)?.into(),             ],
            reagent: [row.get::<_, i32>(46)?.into(), row.get::<_, i32>(47)?.into(), row.get::<_, i32>(48)?.into(), row.get::<_, i32>(49)?.into(), row.get::<_, i32>(50)?.into(), row.get::<_, i32>(51)?.into(), row.get::<_, i32>(52)?.into(), row.get::<_, i32>(53)?.into(),             ],
            reagent_count: [row.get::<_, i32>(54)?.into(), row.get::<_, i32>(55)?.into(), row.get::<_, i32>(56)?.into(), row.get::<_, i32>(57)?.into(), row.get::<_, i32>(58)?.into(), row.get::<_, i32>(59)?.into(), row.get::<_, i32>(60)?.into(), row.get::<_, i32>(61)?.into(),             ],
            equipped_item_class: row.get::<_, i32>(62)?.into(),
            equipped_item_subclass: row.get::<_, i32>(63)?.into(),
            equipped_item_inv_types: row.get::<_, i32>(64)?.into(),
            effect: [row.get::<_, i32>(65)?.into(), row.get::<_, i32>(66)?.into(), row.get::<_, i32>(67)?.into(),             ],
            effect_die_sides: [row.get::<_, i32>(68)?.into(), row.get::<_, i32>(69)?.into(), row.get::<_, i32>(70)?.into(),             ],
            effect_base_dice: [row.get::<_, i32>(71)?.into(), row.get::<_, i32>(72)?.into(), row.get::<_, i32>(73)?.into(),             ],
            effect_dice_per_level: [row.get::<_, i32>(74)?.into(), row.get::<_, i32>(75)?.into(), row.get::<_, i32>(76)?.into(),             ],
            effect_real_points_per_level: [row.get::<_, f32>(77)?.into(), row.get::<_, f32>(78)?.into(), row.get::<_, f32>(79)?.into(),             ],
            effect_base_points: [row.get::<_, i32>(80)?.into(), row.get::<_, i32>(81)?.into(), row.get::<_, i32>(82)?.into(),             ],
            effect_mechanic: [row.get::<_, i32>(83)?.into(), row.get::<_, i32>(84)?.into(), row.get::<_, i32>(85)?.into(),             ],
            implicit_target_a: [row.get::<_, i32>(86)?.into(), row.get::<_, i32>(87)?.into(), row.get::<_, i32>(88)?.into(),             ],
            implicit_target_b: [row.get::<_, i32>(89)?.into(), row.get::<_, i32>(90)?.into(), row.get::<_, i32>(91)?.into(),             ],
            effect_radius_index: [row.get::<_, i32>(92)?.into(), row.get::<_, i32>(93)?.into(), row.get::<_, i32>(94)?.into(),             ],
            effect_aura: [            spell::AuraMod::from_int(row.get::<_, i32>(95)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            spell::AuraMod::from_int(row.get::<_, i32>(96)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            spell::AuraMod::from_int(row.get::<_, i32>(97)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            ],
            effect_aura_period: [row.get::<_, i32>(98)?.into(), row.get::<_, i32>(99)?.into(), row.get::<_, i32>(100)?.into(),             ],
            effect_amplitude: [row.get::<_, f32>(101)?.into(), row.get::<_, f32>(102)?.into(), row.get::<_, f32>(103)?.into(),             ],
            effect_chain_targets: [row.get::<_, i32>(104)?.into(), row.get::<_, i32>(105)?.into(), row.get::<_, i32>(106)?.into(),             ],
            effect_item_type: [row.get::<_, i32>(107)?.into(), row.get::<_, i32>(108)?.into(), row.get::<_, i32>(109)?.into(),             ],
            effect_misc_value: [row.get::<_, i32>(110)?.into(), row.get::<_, i32>(111)?.into(), row.get::<_, i32>(112)?.into(),             ],
            effect_misc_value_b: [row.get::<_, i32>(113)?.into(), row.get::<_, i32>(114)?.into(), row.get::<_, i32>(115)?.into(),             ],
            effect_trigger_spell: [row.get::<_, i32>(116)?.into(), row.get::<_, i32>(117)?.into(), row.get::<_, i32>(118)?.into(),             ],
            effect_points_per_combo: [row.get::<_, f32>(119)?.into(), row.get::<_, f32>(120)?.into(), row.get::<_, f32>(121)?.into(),             ],
            spell_visual_id: [row.get::<_, i32>(122)?.into(), row.get::<_, i32>(123)?.into(),             ],
            spell_icon_id: row.get::<_, i32>(124)?.into(),
            active_icon_id: row.get::<_, i32>(125)?.into(),
            spell_priority: row.get::<_, i32>(126)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(127)?.into(),
                ko_kr: row.get::<_, String>(128)?.into(),
                fr_fr: row.get::<_, String>(129)?.into(),
                de_de: row.get::<_, String>(130)?.into(),
                en_cn: row.get::<_, String>(131)?.into(),
                en_tw: row.get::<_, String>(132)?.into(),
                es_es: row.get::<_, String>(133)?.into(),
                es_mx: row.get::<_, String>(134)?.into(),
                ru_ru: row.get::<_, String>(135)?.into(),
                ja_jp: row.get::<_, String>(136)?.into(),
                pt_pt: row.get::<_, String>(137)?.into(),
                it_it: row.get::<_, String>(138)?.into(),
                unknown_12: row.get::<_, String>(139)?.into(),
                unknown_13: row.get::<_, String>(140)?.into(),
                unknown_14: row.get::<_, String>(141)?.into(),
                unknown_15: row.get::<_, String>(142)?.into(),
                flags: row.get::<_, u32>(143)?.into(),
            },
            name_subtext_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(144)?.into(),
                ko_kr: row.get::<_, String>(145)?.into(),
                fr_fr: row.get::<_, String>(146)?.into(),
                de_de: row.get::<_, String>(147)?.into(),
                en_cn: row.get::<_, String>(148)?.into(),
                en_tw: row.get::<_, String>(149)?.into(),
                es_es: row.get::<_, String>(150)?.into(),
                es_mx: row.get::<_, String>(151)?.into(),
                ru_ru: row.get::<_, String>(152)?.into(),
                ja_jp: row.get::<_, String>(153)?.into(),
                pt_pt: row.get::<_, String>(154)?.into(),
                it_it: row.get::<_, String>(155)?.into(),
                unknown_12: row.get::<_, String>(156)?.into(),
                unknown_13: row.get::<_, String>(157)?.into(),
                unknown_14: row.get::<_, String>(158)?.into(),
                unknown_15: row.get::<_, String>(159)?.into(),
                flags: row.get::<_, u32>(160)?.into(),
            },
            description_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(161)?.into(),
                ko_kr: row.get::<_, String>(162)?.into(),
                fr_fr: row.get::<_, String>(163)?.into(),
                de_de: row.get::<_, String>(164)?.into(),
                en_cn: row.get::<_, String>(165)?.into(),
                en_tw: row.get::<_, String>(166)?.into(),
                es_es: row.get::<_, String>(167)?.into(),
                es_mx: row.get::<_, String>(168)?.into(),
                ru_ru: row.get::<_, String>(169)?.into(),
                ja_jp: row.get::<_, String>(170)?.into(),
                pt_pt: row.get::<_, String>(171)?.into(),
                it_it: row.get::<_, String>(172)?.into(),
                unknown_12: row.get::<_, String>(173)?.into(),
                unknown_13: row.get::<_, String>(174)?.into(),
                unknown_14: row.get::<_, String>(175)?.into(),
                unknown_15: row.get::<_, String>(176)?.into(),
                flags: row.get::<_, u32>(177)?.into(),
            },
            aura_description_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(178)?.into(),
                ko_kr: row.get::<_, String>(179)?.into(),
                fr_fr: row.get::<_, String>(180)?.into(),
                de_de: row.get::<_, String>(181)?.into(),
                en_cn: row.get::<_, String>(182)?.into(),
                en_tw: row.get::<_, String>(183)?.into(),
                es_es: row.get::<_, String>(184)?.into(),
                es_mx: row.get::<_, String>(185)?.into(),
                ru_ru: row.get::<_, String>(186)?.into(),
                ja_jp: row.get::<_, String>(187)?.into(),
                pt_pt: row.get::<_, String>(188)?.into(),
                it_it: row.get::<_, String>(189)?.into(),
                unknown_12: row.get::<_, String>(190)?.into(),
                unknown_13: row.get::<_, String>(191)?.into(),
                unknown_14: row.get::<_, String>(192)?.into(),
                unknown_15: row.get::<_, String>(193)?.into(),
                flags: row.get::<_, u32>(194)?.into(),
            },
            mana_cost_pct: row.get::<_, i32>(195)?.into(),
            start_recovery_category: row.get::<_, i32>(196)?.into(),
            start_recovery_time: row.get::<_, i32>(197)?.into(),
            max_target_level: row.get::<_, i32>(198)?.into(),
            spell_class_set: row.get::<_, i32>(199)?.into(),
            spell_class_mask: [row.get::<_, i32>(200)?.into(), row.get::<_, i32>(201)?.into(),             ],
            max_targets: row.get::<_, i32>(202)?.into(),
            defense_type: row.get::<_, i32>(203)?.into(),
            prevention_type: row.get::<_, i32>(204)?.into(),
            stance_bar_order: row.get::<_, i32>(205)?.into(),
            effect_chain_amplitude: [row.get::<_, f32>(206)?.into(), row.get::<_, f32>(207)?.into(), row.get::<_, f32>(208)?.into(),             ],
            min_faction_id: row.get::<_, i32>(209)?.into(),
            min_reputation: row.get::<_, i32>(210)?.into(),
            required_aura_vision: row.get::<_, i32>(211)?.into(),
            required_totem_category_id: [row.get::<_, i32>(212)?.into(), row.get::<_, i32>(213)?.into(),             ],
            required_areas_id: row.get::<_, i32>(214)?.into(),
            school_mask: row.get::<_, i32>(215)?.into(),
        });
    }
    Ok(spell::Spell { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellCastTimes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        base,
        per_level,
        minimum
    FROM `SpellCastTimes`;"
    )
}


pub(crate) fn spell_cast_times_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_cast_times::SpellCastTimes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_cast_times::SpellCastTimesRow {
            id: row.get::<_, i32>(0)?.into(),
            base: row.get::<_, i32>(1)?.into(),
            per_level: row.get::<_, i32>(2)?.into(),
            minimum: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(spell_cast_times::SpellCastTimes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellCategory() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        flags
    FROM `SpellCategory`;"
    )
}


pub(crate) fn spell_category_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_category::SpellCategory, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_category::SpellCategoryRow {
            id: row.get::<_, i32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
        });
    }
    Ok(spell_category::SpellCategory { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellChainEffects() -> (&'static str, &'static str, &'static str) {
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
        texture_length REAL  NOT NULL
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
        texture_length
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
        ?47
    );"
    ,
    "SELECT
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
        texture_length
    FROM `SpellChainEffects`;"
    )
}


pub(crate) fn spell_chain_effects_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_chain_effects::SpellChainEffects, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_chain_effects::SpellChainEffectsRow {
            id: row.get::<_, i32>(0)?.into(),
            avg_seg_len: row.get::<_, f32>(1)?.into(),
            width: row.get::<_, f32>(2)?.into(),
            noise_scale: row.get::<_, f32>(3)?.into(),
            tex_coord_scale: row.get::<_, f32>(4)?.into(),
            seg_duration: row.get::<_, i32>(5)?.into(),
            seg_delay: row.get::<_, i32>(6)?.into(),
            texture: row.get::<_, String>(7)?.into(),
            flags: row.get::<_, i32>(8)?.into(),
            joint_count: row.get::<_, i32>(9)?.into(),
            joint_offset_radius: row.get::<_, f32>(10)?.into(),
            joints_per_minor_joint: row.get::<_, i32>(11)?.into(),
            minor_joints_per_major_joint: row.get::<_, i32>(12)?.into(),
            minor_joint_scale: row.get::<_, f32>(13)?.into(),
            major_joint_scale: row.get::<_, f32>(14)?.into(),
            joint_move_speed: row.get::<_, f32>(15)?.into(),
            joint_smoothness: row.get::<_, f32>(16)?.into(),
            min_duration_between_joint_jumps: row.get::<_, f32>(17)?.into(),
            max_duration_between_joint_jumps: row.get::<_, f32>(18)?.into(),
            wave_height: row.get::<_, f32>(19)?.into(),
            wave_freq: row.get::<_, f32>(20)?.into(),
            wave_speed: row.get::<_, f32>(21)?.into(),
            min_wave_angle: row.get::<_, f32>(22)?.into(),
            max_wave_angle: row.get::<_, f32>(23)?.into(),
            min_wave_spin: row.get::<_, f32>(24)?.into(),
            max_wave_spin: row.get::<_, f32>(25)?.into(),
            arc_height: row.get::<_, f32>(26)?.into(),
            min_arc_angle: row.get::<_, f32>(27)?.into(),
            max_arc_angle: row.get::<_, f32>(28)?.into(),
            min_arc_spin: row.get::<_, f32>(29)?.into(),
            max_arc_spin: row.get::<_, f32>(30)?.into(),
            delay_between_effects: row.get::<_, f32>(31)?.into(),
            min_flicker_on_duration: row.get::<_, f32>(32)?.into(),
            max_flicker_on_duration: row.get::<_, f32>(33)?.into(),
            min_flicker_off_duration: row.get::<_, f32>(34)?.into(),
            max_flicker_off_duration: row.get::<_, f32>(35)?.into(),
            pulse_speed: row.get::<_, f32>(36)?.into(),
            pulse_on_length: row.get::<_, f32>(37)?.into(),
            pulse_fade_length: row.get::<_, f32>(38)?.into(),
            alpha: row.get::<_, i8>(39)?.into(),
            red: row.get::<_, i8>(40)?.into(),
            green: row.get::<_, i8>(41)?.into(),
            blue: row.get::<_, i8>(42)?.into(),
            blend_mode: row.get::<_, i8>(43)?.into(),
            combo: row.get::<_, String>(44)?.into(),
            render_layer: row.get::<_, i32>(45)?.into(),
            texture_length: row.get::<_, f32>(46)?.into(),
        });
    }
    Ok(spell_chain_effects::SpellChainEffects { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellDispelType() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SpellDispelType`;"
    )
}


pub(crate) fn spell_dispel_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_dispel_type::SpellDispelType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_dispel_type::SpellDispelTypeRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            mask: row.get::<_, i32>(18)?.into(),
            immunity_possible: row.get::<_, i32>(19)?.into(),
            internal_name: row.get::<_, String>(20)?.into(),
        });
    }
    Ok(spell_dispel_type::SpellDispelType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellDuration() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        duration,
        duration_per_level,
        max_duration
    FROM `SpellDuration`;"
    )
}


pub(crate) fn spell_duration_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_duration::SpellDuration, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_duration::SpellDurationRow {
            id: row.get::<_, i32>(0)?.into(),
            duration: row.get::<_, i32>(1)?.into(),
            duration_per_level: row.get::<_, i32>(2)?.into(),
            max_duration: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(spell_duration::SpellDuration { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellEffectCameraShakes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        camera_shake_0,
        camera_shake_1,
        camera_shake_2
    FROM `SpellEffectCameraShakes`;"
    )
}


pub(crate) fn spell_effect_camera_shakes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_effect_camera_shakes::SpellEffectCameraShakes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_effect_camera_shakes::SpellEffectCameraShakesRow {
            id: row.get::<_, i32>(0)?.into(),
            camera_shake: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
        });
    }
    Ok(spell_effect_camera_shakes::SpellEffectCameraShakes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellFocusObject() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellFocusObject`;"
    )
}


pub(crate) fn spell_focus_object_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_focus_object::SpellFocusObject, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_focus_object::SpellFocusObjectRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(spell_focus_object::SpellFocusObject { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellIcon() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        texture_filename
    FROM `SpellIcon`;"
    )
}


pub(crate) fn spell_icon_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_icon::SpellIcon, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_icon::SpellIconRow {
            id: row.get::<_, i32>(0)?.into(),
            texture_filename: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(spell_icon::SpellIcon { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellItemEnchantment() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellItemEnchantment (
        id INTEGER PRIMARY KEY NOT NULL,
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
        name_lang_flags INTEGER NOT NULL,
        item_visual INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        src_item_id INTEGER  NOT NULL,
        condition_id INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellItemEnchantment (
        id,
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
        condition_id
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
    ,
    "SELECT
        id,
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
        condition_id
    FROM `SpellItemEnchantment`;"
    )
}


pub(crate) fn spell_item_enchantment_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_item_enchantment::SpellItemEnchantment, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_item_enchantment::SpellItemEnchantmentRow {
            id: row.get::<_, i32>(0)?.into(),
            effect: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
            effect_points_min: [row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(),             ],
            effect_points_max: [row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
            effect_arg: [row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(),             ],
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(13)?.into(),
                ko_kr: row.get::<_, String>(14)?.into(),
                fr_fr: row.get::<_, String>(15)?.into(),
                de_de: row.get::<_, String>(16)?.into(),
                en_cn: row.get::<_, String>(17)?.into(),
                en_tw: row.get::<_, String>(18)?.into(),
                es_es: row.get::<_, String>(19)?.into(),
                es_mx: row.get::<_, String>(20)?.into(),
                ru_ru: row.get::<_, String>(21)?.into(),
                ja_jp: row.get::<_, String>(22)?.into(),
                pt_pt: row.get::<_, String>(23)?.into(),
                it_it: row.get::<_, String>(24)?.into(),
                unknown_12: row.get::<_, String>(25)?.into(),
                unknown_13: row.get::<_, String>(26)?.into(),
                unknown_14: row.get::<_, String>(27)?.into(),
                unknown_15: row.get::<_, String>(28)?.into(),
                flags: row.get::<_, u32>(29)?.into(),
            },
            item_visual: row.get::<_, i32>(30)?.into(),
            flags: row.get::<_, i32>(31)?.into(),
            src_item_id: row.get::<_, i32>(32)?.into(),
            condition_id: row.get::<_, i32>(33)?.into(),
        });
    }
    Ok(spell_item_enchantment::SpellItemEnchantment { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellItemEnchantmentCondition() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `SpellItemEnchantmentCondition`;"
    )
}


pub(crate) fn spell_item_enchantment_condition_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_item_enchantment_condition::SpellItemEnchantmentCondition, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_item_enchantment_condition::SpellItemEnchantmentConditionRow {
            id: row.get::<_, i32>(0)?.into(),
            lt_operand_type: [row.get::<_, i8>(1)?.into(), row.get::<_, i8>(2)?.into(), row.get::<_, i8>(3)?.into(), row.get::<_, i8>(4)?.into(), row.get::<_, i8>(5)?.into(),             ],
            lt_operand: [row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(),             ],
            operator: [row.get::<_, i8>(11)?.into(), row.get::<_, i8>(12)?.into(), row.get::<_, i8>(13)?.into(), row.get::<_, i8>(14)?.into(), row.get::<_, i8>(15)?.into(),             ],
            rt_operand_type: [row.get::<_, i8>(16)?.into(), row.get::<_, i8>(17)?.into(), row.get::<_, i8>(18)?.into(), row.get::<_, i8>(19)?.into(), row.get::<_, i8>(20)?.into(),             ],
            rt_operand: [row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(),             ],
            logic: [row.get::<_, i8>(26)?.into(), row.get::<_, i8>(27)?.into(), row.get::<_, i8>(28)?.into(), row.get::<_, i8>(29)?.into(), row.get::<_, i8>(30)?.into(),             ],
        });
    }
    Ok(spell_item_enchantment_condition::SpellItemEnchantmentCondition { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellMechanic() -> (&'static str, &'static str, &'static str) {
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
        state_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellMechanic`;"
    )
}


pub(crate) fn spell_mechanic_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_mechanic::SpellMechanic, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_mechanic::SpellMechanicRow {
            id: row.get::<_, i32>(0)?.into(),
            state_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
        });
    }
    Ok(spell_mechanic::SpellMechanic { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellMissileMotion() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        script_body,
        flags,
        missile_count
    FROM `SpellMissileMotion`;"
    )
}


pub(crate) fn spell_missile_motion_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_missile_motion::SpellMissileMotion, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_missile_motion::SpellMissileMotionRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            script_body: row.get::<_, String>(2)?.into(),
            flags: row.get::<_, i32>(3)?.into(),
            missile_count: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(spell_missile_motion::SpellMissileMotion { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellRadius() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        radius,
        radius_per_level,
        radius_max
    FROM `SpellRadius`;"
    )
}


pub(crate) fn spell_radius_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_radius::SpellRadius, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_radius::SpellRadiusRow {
            id: row.get::<_, i32>(0)?.into(),
            radius: row.get::<_, f32>(1)?.into(),
            radius_per_level: row.get::<_, f32>(2)?.into(),
            radius_max: row.get::<_, f32>(3)?.into(),
        });
    }
    Ok(spell_radius::SpellRadius { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellRange() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellRange (
        id INTEGER PRIMARY KEY NOT NULL,
        range_min REAL  NOT NULL,
        range_max REAL  NOT NULL,
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
        display_name_lang_flags INTEGER NOT NULL,
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
        display_name_short_lang_flags INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SpellRange (
        id,
        range_min,
        range_max,
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
        ?38
    );"
    ,
    "SELECT
        id,
        range_min,
        range_max,
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
    FROM `SpellRange`;"
    )
}


pub(crate) fn spell_range_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_range::SpellRange, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_range::SpellRangeRow {
            id: row.get::<_, i32>(0)?.into(),
            range_min: row.get::<_, f32>(1)?.into(),
            range_max: row.get::<_, f32>(2)?.into(),
            flags: row.get::<_, i32>(3)?.into(),
            display_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                ru_ru: row.get::<_, String>(12)?.into(),
                ja_jp: row.get::<_, String>(13)?.into(),
                pt_pt: row.get::<_, String>(14)?.into(),
                it_it: row.get::<_, String>(15)?.into(),
                unknown_12: row.get::<_, String>(16)?.into(),
                unknown_13: row.get::<_, String>(17)?.into(),
                unknown_14: row.get::<_, String>(18)?.into(),
                unknown_15: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
            display_name_short_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(21)?.into(),
                ko_kr: row.get::<_, String>(22)?.into(),
                fr_fr: row.get::<_, String>(23)?.into(),
                de_de: row.get::<_, String>(24)?.into(),
                en_cn: row.get::<_, String>(25)?.into(),
                en_tw: row.get::<_, String>(26)?.into(),
                es_es: row.get::<_, String>(27)?.into(),
                es_mx: row.get::<_, String>(28)?.into(),
                ru_ru: row.get::<_, String>(29)?.into(),
                ja_jp: row.get::<_, String>(30)?.into(),
                pt_pt: row.get::<_, String>(31)?.into(),
                it_it: row.get::<_, String>(32)?.into(),
                unknown_12: row.get::<_, String>(33)?.into(),
                unknown_13: row.get::<_, String>(34)?.into(),
                unknown_14: row.get::<_, String>(35)?.into(),
                unknown_15: row.get::<_, String>(36)?.into(),
                flags: row.get::<_, u32>(37)?.into(),
            },
        });
    }
    Ok(spell_range::SpellRange { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellShapeshiftForm() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SpellShapeshiftForm`;"
    )
}


pub(crate) fn spell_shapeshift_form_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_shapeshift_form::SpellShapeshiftForm, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_shapeshift_form::SpellShapeshiftFormRow {
            id: row.get::<_, i32>(0)?.into(),
            bonus_action_bar: row.get::<_, i32>(1)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                ru_ru: row.get::<_, String>(10)?.into(),
                ja_jp: row.get::<_, String>(11)?.into(),
                pt_pt: row.get::<_, String>(12)?.into(),
                it_it: row.get::<_, String>(13)?.into(),
                unknown_12: row.get::<_, String>(14)?.into(),
                unknown_13: row.get::<_, String>(15)?.into(),
                unknown_14: row.get::<_, String>(16)?.into(),
                unknown_15: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
            flags: row.get::<_, i32>(19)?.into(),
            creature_type: row.get::<_, i32>(20)?.into(),
            attack_icon_id: row.get::<_, i32>(21)?.into(),
            combat_round_time: row.get::<_, i32>(22)?.into(),
            creature_display_id: [row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(),             ],
            preset_spell_id: [row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(), row.get::<_, i32>(33)?.into(), row.get::<_, i32>(34)?.into(),             ],
        });
    }
    Ok(spell_shapeshift_form::SpellShapeshiftForm { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellVisual() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisual (
        id INTEGER PRIMARY KEY NOT NULL,
        precast_kit INTEGER  NOT NULL,
        cast_kit INTEGER  NOT NULL,
        impact_kit INTEGER  NOT NULL,
        state_kit INTEGER  NOT NULL,
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
        persistent_area_kit INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisual (
        id,
        precast_kit,
        cast_kit,
        impact_kit,
        state_kit,
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
        persistent_area_kit
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
    ,
    "SELECT
        id,
        precast_kit,
        cast_kit,
        impact_kit,
        state_kit,
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
        persistent_area_kit
    FROM `SpellVisual`;"
    )
}


pub(crate) fn spell_visual_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual::SpellVisual, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual::SpellVisualRow {
            id: row.get::<_, i32>(0)?.into(),
            precast_kit: row.get::<_, i32>(1)?.into(),
            cast_kit: row.get::<_, i32>(2)?.into(),
            impact_kit: row.get::<_, i32>(3)?.into(),
            state_kit: row.get::<_, i32>(4)?.into(),
            channel_kit: row.get::<_, i32>(5)?.into(),
            has_missile: row.get::<_, i32>(6)?.into(),
            missile_model: row.get::<_, i32>(7)?.into(),
            missile_path_type: row.get::<_, i32>(8)?.into(),
            missile_destination_attachment: row.get::<_, i32>(9)?.into(),
            missile_sound: row.get::<_, i32>(10)?.into(),
            anim_event_sound_id: row.get::<_, i32>(11)?.into(),
            flags: row.get::<_, i32>(12)?.into(),
            caster_impact_kit: row.get::<_, i32>(13)?.into(),
            target_impact_kit: row.get::<_, i32>(14)?.into(),
            missile_attachment: row.get::<_, i32>(15)?.into(),
            missile_follow_ground_height: row.get::<_, i32>(16)?.into(),
            missile_follow_ground_drop_speed: row.get::<_, i32>(17)?.into(),
            missile_follow_ground_approach: row.get::<_, i32>(18)?.into(),
            missile_follow_ground_flags: row.get::<_, i32>(19)?.into(),
            missile_motion: row.get::<_, i32>(20)?.into(),
            missile_targeting_kit: row.get::<_, i32>(21)?.into(),
            instant_area_kit: row.get::<_, i32>(22)?.into(),
            impact_area_kit: row.get::<_, i32>(23)?.into(),
            persistent_area_kit: row.get::<_, i32>(24)?.into(),
        });
    }
    Ok(spell_visual::SpellVisual { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellVisualEffectName() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualEffectName (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        file_name TEXT  NOT NULL,
        area_effect_size REAL  NOT NULL,
        scale REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualEffectName (
        id,
        name,
        file_name,
        area_effect_size,
        scale
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5
    );"
    ,
    "SELECT
        id,
        name,
        file_name,
        area_effect_size,
        scale
    FROM `SpellVisualEffectName`;"
    )
}


pub(crate) fn spell_visual_effect_name_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual_effect_name::SpellVisualEffectName, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual_effect_name::SpellVisualEffectNameRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            file_name: row.get::<_, String>(2)?.into(),
            area_effect_size: row.get::<_, f32>(3)?.into(),
            scale: row.get::<_, f32>(4)?.into(),
        });
    }
    Ok(spell_visual_effect_name::SpellVisualEffectName { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellVisualKit() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualKit (
        id INTEGER PRIMARY KEY NOT NULL,
        kit_type INTEGER  NOT NULL,
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
        kit_type,
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
    ,
    "SELECT
        id,
        kit_type,
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
    FROM `SpellVisualKit`;"
    )
}


pub(crate) fn spell_visual_kit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual_kit::SpellVisualKit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual_kit::SpellVisualKitRow {
            id: row.get::<_, i32>(0)?.into(),
            kit_type: row.get::<_, i32>(1)?.into(),
            anim_id: row.get::<_, i32>(2)?.into(),
            head_effect: row.get::<_, i32>(3)?.into(),
            chest_effect: row.get::<_, i32>(4)?.into(),
            base_effect: row.get::<_, i32>(5)?.into(),
            left_hand_effect: row.get::<_, i32>(6)?.into(),
            right_hand_effect: row.get::<_, i32>(7)?.into(),
            breath_effect: row.get::<_, i32>(8)?.into(),
            left_weapon_effect: row.get::<_, i32>(9)?.into(),
            right_weapon_effect: row.get::<_, i32>(10)?.into(),
            special_effect: [row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(),             ],
            world_effect: row.get::<_, i32>(14)?.into(),
            sound_id: row.get::<_, i32>(15)?.into(),
            shake_id: row.get::<_, i32>(16)?.into(),
            char_proc: [row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(),             ],
            char_param_zero: [row.get::<_, f32>(21)?.into(), row.get::<_, f32>(22)?.into(), row.get::<_, f32>(23)?.into(), row.get::<_, f32>(24)?.into(),             ],
            char_param_one: [row.get::<_, f32>(25)?.into(), row.get::<_, f32>(26)?.into(), row.get::<_, f32>(27)?.into(), row.get::<_, f32>(28)?.into(),             ],
            char_param_two: [row.get::<_, f32>(29)?.into(), row.get::<_, f32>(30)?.into(), row.get::<_, f32>(31)?.into(), row.get::<_, f32>(32)?.into(),             ],
            char_param_three: [row.get::<_, f32>(33)?.into(), row.get::<_, f32>(34)?.into(), row.get::<_, f32>(35)?.into(), row.get::<_, f32>(36)?.into(),             ],
            flags: row.get::<_, i32>(37)?.into(),
        });
    }
    Ok(spell_visual_kit::SpellVisualKit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellVisualPrecastTransitions() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        precast_load_anim_name,
        precast_hold_anim_name
    FROM `SpellVisualPrecastTransitions`;"
    )
}


pub(crate) fn spell_visual_precast_transitions_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual_precast_transitions::SpellVisualPrecastTransitions, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual_precast_transitions::SpellVisualPrecastTransitionsRow {
            id: row.get::<_, i32>(0)?.into(),
            precast_load_anim_name: row.get::<_, String>(1)?.into(),
            precast_hold_anim_name: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(spell_visual_precast_transitions::SpellVisualPrecastTransitions { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn StableSlotPrices() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        cost
    FROM `StableSlotPrices`;"
    )
}


pub(crate) fn stable_slot_prices_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<stable_slot_prices::StableSlotPrices, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(stable_slot_prices::StableSlotPricesRow {
            id: row.get::<_, i32>(0)?.into(),
            cost: row.get::<_, i32>(1)?.into(),
        });
    }
    Ok(stable_slot_prices::StableSlotPrices { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Startup_Strings() -> (&'static str, &'static str, &'static str) {
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
        message_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Startup_Strings`;"
    )
}


pub(crate) fn startup_strings_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<startup_strings::Startup_Strings, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(startup_strings::Startup_StringsRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            message_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                ru_ru: row.get::<_, String>(10)?.into(),
                ja_jp: row.get::<_, String>(11)?.into(),
                pt_pt: row.get::<_, String>(12)?.into(),
                it_it: row.get::<_, String>(13)?.into(),
                unknown_12: row.get::<_, String>(14)?.into(),
                unknown_13: row.get::<_, String>(15)?.into(),
                unknown_14: row.get::<_, String>(16)?.into(),
                unknown_15: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
        });
    }
    Ok(startup_strings::Startup_Strings { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Stationery() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        item_id,
        texture,
        flags
    FROM `Stationery`;"
    )
}


pub(crate) fn stationery_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<stationery::Stationery, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(stationery::StationeryRow {
            id: row.get::<_, i32>(0)?.into(),
            item_id: row.get::<_, i32>(1)?.into(),
            texture: row.get::<_, String>(2)?.into(),
            flags: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(stationery::Stationery { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn StringLookups() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        string
    FROM `StringLookups`;"
    )
}


pub(crate) fn string_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<string_lookups::StringLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(string_lookups::StringLookupsRow {
            id: row.get::<_, i32>(0)?.into(),
            string: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(string_lookups::StringLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SummonProperties() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        control,
        faction,
        title,
        slot,
        flags
    FROM `SummonProperties`;"
    )
}


pub(crate) fn summon_properties_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<summon_properties::SummonProperties, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(summon_properties::SummonPropertiesRow {
            id: row.get::<_, i32>(0)?.into(),
            control: row.get::<_, i32>(1)?.into(),
            faction: row.get::<_, i32>(2)?.into(),
            title: row.get::<_, i32>(3)?.into(),
            slot: row.get::<_, i32>(4)?.into(),
            flags: row.get::<_, i32>(5)?.into(),
        });
    }
    Ok(summon_properties::SummonProperties { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Talent() -> (&'static str, &'static str, &'static str) {
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
        required_spell_id INTEGER  NOT NULL
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
        required_spell_id
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
    ,
    "SELECT
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
        required_spell_id
    FROM `Talent`;"
    )
}


pub(crate) fn talent_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<talent::Talent, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(talent::TalentRow {
            id: row.get::<_, i32>(0)?.into(),
            tab_id: row.get::<_, i32>(1)?.into(),
            tier_id: row.get::<_, i32>(2)?.into(),
            column_index: row.get::<_, i32>(3)?.into(),
            spell_rank: [row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(),             ],
            prereq_talent: [row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(),             ],
            prereq_rank: [row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(),             ],
            flags: row.get::<_, i32>(19)?.into(),
            required_spell_id: row.get::<_, i32>(20)?.into(),
        });
    }
    Ok(talent::Talent { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TalentTab() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
        spell_icon_id INTEGER  NOT NULL,
        race_mask INTEGER  NOT NULL,
        class_mask INTEGER  NOT NULL,
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
        ?23
    );"
    ,
    "SELECT
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
        order_index,
        background_file
    FROM `TalentTab`;"
    )
}


pub(crate) fn talent_tab_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<talent_tab::TalentTab, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(talent_tab::TalentTabRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            spell_icon_id: row.get::<_, i32>(18)?.into(),
            race_mask: row.get::<_, i32>(19)?.into(),
            class_mask: row.get::<_, i32>(20)?.into(),
            order_index: row.get::<_, i32>(21)?.into(),
            background_file: row.get::<_, String>(22)?.into(),
        });
    }
    Ok(talent_tab::TalentTab { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TaxiNodes() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `TaxiNodes`;"
    )
}


pub(crate) fn taxi_nodes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<taxi_nodes::TaxiNodes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(taxi_nodes::TaxiNodesRow {
            id: row.get::<_, i32>(0)?.into(),
            continent_id: row.get::<_, i32>(1)?.into(),
            pos: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(),             ],
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                ru_ru: row.get::<_, String>(13)?.into(),
                ja_jp: row.get::<_, String>(14)?.into(),
                pt_pt: row.get::<_, String>(15)?.into(),
                it_it: row.get::<_, String>(16)?.into(),
                unknown_12: row.get::<_, String>(17)?.into(),
                unknown_13: row.get::<_, String>(18)?.into(),
                unknown_14: row.get::<_, String>(19)?.into(),
                unknown_15: row.get::<_, String>(20)?.into(),
                flags: row.get::<_, u32>(21)?.into(),
            },
            mount_creature_id: [row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(),             ],
        });
    }
    Ok(taxi_nodes::TaxiNodes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TaxiPath() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        from_taxi_node,
        to_taxi_node,
        cost
    FROM `TaxiPath`;"
    )
}


pub(crate) fn taxi_path_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<taxi_path::TaxiPath, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(taxi_path::TaxiPathRow {
            id: row.get::<_, i32>(0)?.into(),
            from_taxi_node: row.get::<_, i32>(1)?.into(),
            to_taxi_node: row.get::<_, i32>(2)?.into(),
            cost: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(taxi_path::TaxiPath { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TaxiPathNode() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `TaxiPathNode`;"
    )
}


pub(crate) fn taxi_path_node_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<taxi_path_node::TaxiPathNode, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(taxi_path_node::TaxiPathNodeRow {
            id: row.get::<_, i32>(0)?.into(),
            path_id: row.get::<_, i32>(1)?.into(),
            node_index: row.get::<_, i32>(2)?.into(),
            continent_id: row.get::<_, i32>(3)?.into(),
            loc: [row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(), row.get::<_, f32>(6)?.into(),             ],
            flags: row.get::<_, i32>(7)?.into(),
            delay: row.get::<_, i32>(8)?.into(),
            arrival_event_id: row.get::<_, i32>(9)?.into(),
            departure_event_id: row.get::<_, i32>(10)?.into(),
        });
    }
    Ok(taxi_path_node::TaxiPathNode { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TerrainType() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        terrain_id,
        terrain_desc,
        footstep_spray_run,
        footstep_spray_walk,
        sound_id,
        flags
    FROM `TerrainType`;"
    )
}


pub(crate) fn terrain_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<terrain_type::TerrainType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(terrain_type::TerrainTypeRow {
            terrain_id: row.get::<_, i32>(0)?.into(),
            terrain_desc: row.get::<_, String>(1)?.into(),
            footstep_spray_run: row.get::<_, i32>(2)?.into(),
            footstep_spray_walk: row.get::<_, i32>(3)?.into(),
            sound_id: row.get::<_, i32>(4)?.into(),
            flags: row.get::<_, i32>(5)?.into(),
        });
    }
    Ok(terrain_type::TerrainType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TerrainTypeSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id
    FROM `TerrainTypeSounds`;"
    )
}


pub(crate) fn terrain_type_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<terrain_type_sounds::TerrainTypeSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(terrain_type_sounds::TerrainTypeSoundsRow {
            id: row.get::<_, i32>(0)?.into(),
        });
    }
    Ok(terrain_type_sounds::TerrainTypeSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TotemCategory() -> (&'static str, &'static str, &'static str) {
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
        name_lang_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `TotemCategory`;"
    )
}


pub(crate) fn totem_category_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<totem_category::TotemCategory, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(totem_category::TotemCategoryRow {
            id: row.get::<_, i32>(0)?.into(),
            name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                ru_ru: row.get::<_, String>(9)?.into(),
                ja_jp: row.get::<_, String>(10)?.into(),
                pt_pt: row.get::<_, String>(11)?.into(),
                it_it: row.get::<_, String>(12)?.into(),
                unknown_12: row.get::<_, String>(13)?.into(),
                unknown_13: row.get::<_, String>(14)?.into(),
                unknown_14: row.get::<_, String>(15)?.into(),
                unknown_15: row.get::<_, String>(16)?.into(),
                flags: row.get::<_, u32>(17)?.into(),
            },
            totem_category_type: row.get::<_, i32>(18)?.into(),
            totem_category_mask: row.get::<_, i32>(19)?.into(),
        });
    }
    Ok(totem_category::TotemCategory { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TransportAnimation() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        transport_id,
        time_index,
        pos_0,
        pos_1,
        pos_2,
        sequence_id
    FROM `TransportAnimation`;"
    )
}


pub(crate) fn transport_animation_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<transport_animation::TransportAnimation, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(transport_animation::TransportAnimationRow {
            id: row.get::<_, i32>(0)?.into(),
            transport_id: row.get::<_, i32>(1)?.into(),
            time_index: row.get::<_, i32>(2)?.into(),
            pos: [row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(),             ],
            sequence_id: row.get::<_, i32>(6)?.into(),
        });
    }
    Ok(transport_animation::TransportAnimation { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TransportPhysics() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `TransportPhysics`;"
    )
}


pub(crate) fn transport_physics_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<transport_physics::TransportPhysics, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(transport_physics::TransportPhysicsRow {
            id: row.get::<_, i32>(0)?.into(),
            wave_amp: row.get::<_, f32>(1)?.into(),
            wave_time_scale: row.get::<_, f32>(2)?.into(),
            roll_amp: row.get::<_, f32>(3)?.into(),
            roll_time_scale: row.get::<_, f32>(4)?.into(),
            pitch_amp: row.get::<_, f32>(5)?.into(),
            pitch_time_scale: row.get::<_, f32>(6)?.into(),
            max_bank: row.get::<_, f32>(7)?.into(),
            max_bank_turn_speed: row.get::<_, f32>(8)?.into(),
            speed_damp_thresh: row.get::<_, f32>(9)?.into(),
            speed_damp: row.get::<_, f32>(10)?.into(),
        });
    }
    Ok(transport_physics::TransportPhysics { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn UISoundLookups() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_id,
        sound_name
    FROM `UISoundLookups`;"
    )
}


pub(crate) fn ui_sound_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<ui_sound_lookups::UISoundLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(ui_sound_lookups::UISoundLookupsRow {
            id: row.get::<_, i32>(0)?.into(),
            sound_id: row.get::<_, i32>(1)?.into(),
            sound_name: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(ui_sound_lookups::UISoundLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn UnitBlood() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `UnitBlood`;"
    )
}


pub(crate) fn unit_blood_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<unit_blood::UnitBlood, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(unit_blood::UnitBloodRow {
            id: row.get::<_, i32>(0)?.into(),
            combat_blood_spurt_front: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(),             ],
            combat_blood_spurt_back: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
            ground_blood: [row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(), row.get::<_, String>(9)?.into(),             ],
        });
    }
    Ok(unit_blood::UnitBlood { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn UnitBloodLevels() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        violencelevel_0,
        violencelevel_1,
        violencelevel_2
    FROM `UnitBloodLevels`;"
    )
}


pub(crate) fn unit_blood_levels_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<unit_blood_levels::UnitBloodLevels, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(unit_blood_levels::UnitBloodLevelsRow {
            id: row.get::<_, i32>(0)?.into(),
            violencelevel: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
        });
    }
    Ok(unit_blood_levels::UnitBloodLevels { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn VideoHardware() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `VideoHardware`;"
    )
}


pub(crate) fn video_hardware_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<video_hardware::VideoHardware, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(video_hardware::VideoHardwareRow {
            id: row.get::<_, i32>(0)?.into(),
            vendor_id: row.get::<_, i32>(1)?.into(),
            device_id: row.get::<_, i32>(2)?.into(),
            farclip_idx: row.get::<_, i32>(3)?.into(),
            terrain_l_o_d_dist_idx: row.get::<_, i32>(4)?.into(),
            terrain_shadow_l_o_d: row.get::<_, i32>(5)?.into(),
            detail_doodad_density_idx: row.get::<_, i32>(6)?.into(),
            detail_doodad_alpha: row.get::<_, i32>(7)?.into(),
            animating_doodad_idx: row.get::<_, i32>(8)?.into(),
            trilinear: row.get::<_, i32>(9)?.into(),
            num_lights: row.get::<_, i32>(10)?.into(),
            specularity: row.get::<_, i32>(11)?.into(),
            water_l_o_d_idx: row.get::<_, i32>(12)?.into(),
            particle_density_idx: row.get::<_, i32>(13)?.into(),
            unit_draw_dist_idx: row.get::<_, i32>(14)?.into(),
            small_cull_dist_idx: row.get::<_, i32>(15)?.into(),
            resolution_idx: row.get::<_, i32>(16)?.into(),
            base_mip_level: row.get::<_, i32>(17)?.into(),
            ogl_overrides: row.get::<_, String>(18)?.into(),
            d3d_overrides: row.get::<_, String>(19)?.into(),
            fix_lag: row.get::<_, i32>(20)?.into(),
            multisample: row.get::<_, i32>(21)?.into(),
            atlasdisable: row.get::<_, i32>(22)?.into(),
        });
    }
    Ok(video_hardware::VideoHardware { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn VocalUISounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        vocal_u_i_enum,
        race_id,
        normal_sound_id_0,
        normal_sound_id_1,
        pissed_sound_id_0,
        pissed_sound_id_1
    FROM `VocalUISounds`;"
    )
}


pub(crate) fn vocal_ui_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<vocal_ui_sounds::VocalUISounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(vocal_ui_sounds::VocalUISoundsRow {
            id: row.get::<_, i32>(0)?.into(),
            vocal_u_i_enum: row.get::<_, i32>(1)?.into(),
            race_id: row.get::<_, i32>(2)?.into(),
            normal_sound_id: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
            pissed_sound_id: [row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(),             ],
        });
    }
    Ok(vocal_ui_sounds::VocalUISounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WMOAreaTable() -> (&'static str, &'static str, &'static str) {
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
        area_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `WMOAreaTable`;"
    )
}


pub(crate) fn wmo_area_table_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<wmo_area_table::WMOAreaTable, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(wmo_area_table::WMOAreaTableRow {
            id: row.get::<_, i32>(0)?.into(),
            w_m_o_id: row.get::<_, i32>(1)?.into(),
            name_set_id: row.get::<_, i32>(2)?.into(),
            w_m_o_group_id: row.get::<_, i32>(3)?.into(),
            sound_provider_pref: row.get::<_, i32>(4)?.into(),
            sound_provider_pref_underwater: row.get::<_, i32>(5)?.into(),
            ambience_id: row.get::<_, i32>(6)?.into(),
            zone_music: row.get::<_, i32>(7)?.into(),
            intro_sound: row.get::<_, i32>(8)?.into(),
            flags: row.get::<_, i32>(9)?.into(),
            area_table_id: row.get::<_, i32>(10)?.into(),
            area_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(11)?.into(),
                ko_kr: row.get::<_, String>(12)?.into(),
                fr_fr: row.get::<_, String>(13)?.into(),
                de_de: row.get::<_, String>(14)?.into(),
                en_cn: row.get::<_, String>(15)?.into(),
                en_tw: row.get::<_, String>(16)?.into(),
                es_es: row.get::<_, String>(17)?.into(),
                es_mx: row.get::<_, String>(18)?.into(),
                ru_ru: row.get::<_, String>(19)?.into(),
                ja_jp: row.get::<_, String>(20)?.into(),
                pt_pt: row.get::<_, String>(21)?.into(),
                it_it: row.get::<_, String>(22)?.into(),
                unknown_12: row.get::<_, String>(23)?.into(),
                unknown_13: row.get::<_, String>(24)?.into(),
                unknown_14: row.get::<_, String>(25)?.into(),
                unknown_15: row.get::<_, String>(26)?.into(),
                flags: row.get::<_, u32>(27)?.into(),
            },
        });
    }
    Ok(wmo_area_table::WMOAreaTable { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WeaponImpactSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `WeaponImpactSounds`;"
    )
}


pub(crate) fn weapon_impact_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<weapon_impact_sounds::WeaponImpactSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(weapon_impact_sounds::WeaponImpactSoundsRow {
            id: row.get::<_, i32>(0)?.into(),
            weapon_sub_class_id: row.get::<_, i32>(1)?.into(),
            parry_sound_type: row.get::<_, i32>(2)?.into(),
            impact_sound_id: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(),             ],
            crit_impact_sound_id: [row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(),             ],
        });
    }
    Ok(weapon_impact_sounds::WeaponImpactSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WeaponSwingSounds2() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        swing_type,
        crit,
        sound_id
    FROM `WeaponSwingSounds2`;"
    )
}


pub(crate) fn weapon_swing_sounds2_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<weapon_swing_sounds2::WeaponSwingSounds2, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(weapon_swing_sounds2::WeaponSwingSounds2Row {
            id: row.get::<_, i32>(0)?.into(),
            swing_type: row.get::<_, i32>(1)?.into(),
            crit: row.get::<_, i32>(2)?.into(),
            sound_id: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(weapon_swing_sounds2::WeaponSwingSounds2 { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Weather() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Weather (
        id INTEGER PRIMARY KEY NOT NULL,
        ambience_id INTEGER  NOT NULL,
        effect_type INTEGER  NOT NULL,
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
        ?7
    );"
    ,
    "SELECT
        id,
        ambience_id,
        effect_type,
        effect_color_0,
        effect_color_1,
        effect_color_2,
        effect_texture
    FROM `Weather`;"
    )
}


pub(crate) fn weather_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<weather::Weather, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(weather::WeatherRow {
            id: row.get::<_, i32>(0)?.into(),
            ambience_id: row.get::<_, i32>(1)?.into(),
            effect_type: row.get::<_, i32>(2)?.into(),
            effect_color: [row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(),             ],
            effect_texture: row.get::<_, String>(6)?.into(),
        });
    }
    Ok(weather::Weather { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapArea() -> (&'static str, &'static str, &'static str) {
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
        display_map_id INTEGER  NOT NULL
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
        display_map_id
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
    ,
    "SELECT
        id,
        map_id,
        area_id,
        area_name,
        loc_left,
        loc_right,
        loc_top,
        loc_bottom,
        display_map_id
    FROM `WorldMapArea`;"
    )
}


pub(crate) fn world_map_area_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_area::WorldMapArea, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_area::WorldMapAreaRow {
            id: row.get::<_, i32>(0)?.into(),
            map_id: row.get::<_, i32>(1)?.into(),
            area_id: row.get::<_, i32>(2)?.into(),
            area_name: row.get::<_, String>(3)?.into(),
            loc_left: row.get::<_, f32>(4)?.into(),
            loc_right: row.get::<_, f32>(5)?.into(),
            loc_top: row.get::<_, f32>(6)?.into(),
            loc_bottom: row.get::<_, f32>(7)?.into(),
            display_map_id: row.get::<_, i32>(8)?.into(),
        });
    }
    Ok(world_map_area::WorldMapArea { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapContinent() -> (&'static str, &'static str, &'static str) {
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
        taxi_max_1 REAL NOT NULL
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
        taxi_max_1
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
    ,
    "SELECT
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
        taxi_max_1
    FROM `WorldMapContinent`;"
    )
}


pub(crate) fn world_map_continent_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_continent::WorldMapContinent, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_continent::WorldMapContinentRow {
            id: row.get::<_, i32>(0)?.into(),
            map_id: row.get::<_, i32>(1)?.into(),
            left_boundary: row.get::<_, i32>(2)?.into(),
            right_boundary: row.get::<_, i32>(3)?.into(),
            top_boundary: row.get::<_, i32>(4)?.into(),
            bottom_boundary: row.get::<_, i32>(5)?.into(),
            continent_offset: [row.get::<_, f32>(6)?.into(), row.get::<_, f32>(7)?.into(),             ],
            scale: row.get::<_, f32>(8)?.into(),
            taxi_min: [row.get::<_, f32>(9)?.into(), row.get::<_, f32>(10)?.into(),             ],
            taxi_max: [row.get::<_, f32>(11)?.into(), row.get::<_, f32>(12)?.into(),             ],
        });
    }
    Ok(world_map_continent::WorldMapContinent { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapOverlay() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `WorldMapOverlay`;"
    )
}


pub(crate) fn world_map_overlay_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_overlay::WorldMapOverlay, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_overlay::WorldMapOverlayRow {
            id: row.get::<_, i32>(0)?.into(),
            map_area_id: row.get::<_, i32>(1)?.into(),
            area_id: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
            map_point_x: row.get::<_, i32>(6)?.into(),
            map_point_y: row.get::<_, i32>(7)?.into(),
            texture_name: row.get::<_, String>(8)?.into(),
            texture_width: row.get::<_, i32>(9)?.into(),
            texture_height: row.get::<_, i32>(10)?.into(),
            offset_x: row.get::<_, i32>(11)?.into(),
            offset_y: row.get::<_, i32>(12)?.into(),
            hit_rect_top: row.get::<_, i32>(13)?.into(),
            hit_rect_left: row.get::<_, i32>(14)?.into(),
            hit_rect_bottom: row.get::<_, i32>(15)?.into(),
            hit_rect_right: row.get::<_, i32>(16)?.into(),
        });
    }
    Ok(world_map_overlay::WorldMapOverlay { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapTransforms() -> (&'static str, &'static str, &'static str) {
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
        region_offset_1 REAL NOT NULL
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
        region_offset_1
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
    ,
    "SELECT
        id,
        map_id,
        region_min_0,
        region_min_1,
        region_max_0,
        region_max_1,
        new_map_id,
        region_offset_0,
        region_offset_1
    FROM `WorldMapTransforms`;"
    )
}


pub(crate) fn world_map_transforms_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_transforms::WorldMapTransforms, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_transforms::WorldMapTransformsRow {
            id: row.get::<_, i32>(0)?.into(),
            map_id: row.get::<_, i32>(1)?.into(),
            region_min: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(),             ],
            region_max: [row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(),             ],
            new_map_id: row.get::<_, i32>(6)?.into(),
            region_offset: [row.get::<_, f32>(7)?.into(), row.get::<_, f32>(8)?.into(),             ],
        });
    }
    Ok(world_map_transforms::WorldMapTransforms { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldSafeLocs() -> (&'static str, &'static str, &'static str) {
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
        area_name_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `WorldSafeLocs`;"
    )
}


pub(crate) fn world_safe_locs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_safe_locs::WorldSafeLocs, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_safe_locs::WorldSafeLocsRow {
            id: row.get::<_, i32>(0)?.into(),
            continent: row.get::<_, i32>(1)?.into(),
            loc: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(),             ],
            area_name_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                ru_ru: row.get::<_, String>(13)?.into(),
                ja_jp: row.get::<_, String>(14)?.into(),
                pt_pt: row.get::<_, String>(15)?.into(),
                it_it: row.get::<_, String>(16)?.into(),
                unknown_12: row.get::<_, String>(17)?.into(),
                unknown_13: row.get::<_, String>(18)?.into(),
                unknown_14: row.get::<_, String>(19)?.into(),
                unknown_15: row.get::<_, String>(20)?.into(),
                flags: row.get::<_, u32>(21)?.into(),
            },
        });
    }
    Ok(world_safe_locs::WorldSafeLocs { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldStateUI() -> (&'static str, &'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldStateUI (
        id INTEGER PRIMARY KEY NOT NULL,
        map_id INTEGER  NOT NULL,
        area_id INTEGER  NOT NULL,
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
        string_lang_flags INTEGER NOT NULL,
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
        tooltip_lang_flags INTEGER NOT NULL,
        faction_id INTEGER  NOT NULL,
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
        dynamic_tooltip_lang_flags INTEGER NOT NULL,
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
        faction_id,
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
    ,
    "SELECT
        id,
        map_id,
        area_id,
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
        faction_id,
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
    FROM `WorldStateUI`;"
    )
}


pub(crate) fn world_state_ui_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_state_ui::WorldStateUI, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_state_ui::WorldStateUIRow {
            id: row.get::<_, i32>(0)?.into(),
            map_id: row.get::<_, i32>(1)?.into(),
            area_id: row.get::<_, i32>(2)?.into(),
            icon: row.get::<_, String>(3)?.into(),
            string_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                ru_ru: row.get::<_, String>(12)?.into(),
                ja_jp: row.get::<_, String>(13)?.into(),
                pt_pt: row.get::<_, String>(14)?.into(),
                it_it: row.get::<_, String>(15)?.into(),
                unknown_12: row.get::<_, String>(16)?.into(),
                unknown_13: row.get::<_, String>(17)?.into(),
                unknown_14: row.get::<_, String>(18)?.into(),
                unknown_15: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
            tooltip_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(21)?.into(),
                ko_kr: row.get::<_, String>(22)?.into(),
                fr_fr: row.get::<_, String>(23)?.into(),
                de_de: row.get::<_, String>(24)?.into(),
                en_cn: row.get::<_, String>(25)?.into(),
                en_tw: row.get::<_, String>(26)?.into(),
                es_es: row.get::<_, String>(27)?.into(),
                es_mx: row.get::<_, String>(28)?.into(),
                ru_ru: row.get::<_, String>(29)?.into(),
                ja_jp: row.get::<_, String>(30)?.into(),
                pt_pt: row.get::<_, String>(31)?.into(),
                it_it: row.get::<_, String>(32)?.into(),
                unknown_12: row.get::<_, String>(33)?.into(),
                unknown_13: row.get::<_, String>(34)?.into(),
                unknown_14: row.get::<_, String>(35)?.into(),
                unknown_15: row.get::<_, String>(36)?.into(),
                flags: row.get::<_, u32>(37)?.into(),
            },
            faction_id: row.get::<_, i32>(38)?.into(),
            state_variable: row.get::<_, i32>(39)?.into(),
            ty: row.get::<_, i32>(40)?.into(),
            dynamic_icon: row.get::<_, String>(41)?.into(),
            dynamic_tooltip_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(42)?.into(),
                ko_kr: row.get::<_, String>(43)?.into(),
                fr_fr: row.get::<_, String>(44)?.into(),
                de_de: row.get::<_, String>(45)?.into(),
                en_cn: row.get::<_, String>(46)?.into(),
                en_tw: row.get::<_, String>(47)?.into(),
                es_es: row.get::<_, String>(48)?.into(),
                es_mx: row.get::<_, String>(49)?.into(),
                ru_ru: row.get::<_, String>(50)?.into(),
                ja_jp: row.get::<_, String>(51)?.into(),
                pt_pt: row.get::<_, String>(52)?.into(),
                it_it: row.get::<_, String>(53)?.into(),
                unknown_12: row.get::<_, String>(54)?.into(),
                unknown_13: row.get::<_, String>(55)?.into(),
                unknown_14: row.get::<_, String>(56)?.into(),
                unknown_15: row.get::<_, String>(57)?.into(),
                flags: row.get::<_, u32>(58)?.into(),
            },
            extended_u_i: row.get::<_, String>(59)?.into(),
            extended_u_i_state_variable: [row.get::<_, i32>(60)?.into(), row.get::<_, i32>(61)?.into(), row.get::<_, i32>(62)?.into(),             ],
        });
    }
    Ok(world_state_ui::WorldStateUI { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldStateZoneSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        world_state_id,
        world_state_value,
        area_id,
        w_m_o_area_id,
        zone_intro_music_id,
        zone_music_id,
        sound_ambience_id,
        sound_provider_preferences_id
    FROM `WorldStateZoneSounds`;"
    )
}


pub(crate) fn world_state_zone_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_state_zone_sounds::WorldStateZoneSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_state_zone_sounds::WorldStateZoneSoundsRow {
            world_state_id: row.get::<_, i32>(0)?.into(),
            world_state_value: row.get::<_, i32>(1)?.into(),
            area_id: row.get::<_, i32>(2)?.into(),
            w_m_o_area_id: row.get::<_, i32>(3)?.into(),
            zone_intro_music_id: row.get::<_, i32>(4)?.into(),
            zone_music_id: row.get::<_, i32>(5)?.into(),
            sound_ambience_id: row.get::<_, i32>(6)?.into(),
            sound_provider_preferences_id: row.get::<_, i32>(7)?.into(),
        });
    }
    Ok(world_state_zone_sounds::WorldStateZoneSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WowError_Strings() -> (&'static str, &'static str, &'static str) {
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
        description_lang_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `WowError_Strings`;"
    )
}


pub(crate) fn wow_error_strings_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<wow_error_strings::WowError_Strings, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(wow_error_strings::WowError_StringsRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            description_lang: ExtendedLocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                ru_ru: row.get::<_, String>(10)?.into(),
                ja_jp: row.get::<_, String>(11)?.into(),
                pt_pt: row.get::<_, String>(12)?.into(),
                it_it: row.get::<_, String>(13)?.into(),
                unknown_12: row.get::<_, String>(14)?.into(),
                unknown_13: row.get::<_, String>(15)?.into(),
                unknown_14: row.get::<_, String>(16)?.into(),
                unknown_15: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
        });
    }
    Ok(wow_error_strings::WowError_Strings { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ZoneIntroMusicTable() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        sound_id,
        priority,
        min_delay_minutes
    FROM `ZoneIntroMusicTable`;"
    )
}


pub(crate) fn zone_intro_music_table_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<zone_intro_music_table::ZoneIntroMusicTable, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(zone_intro_music_table::ZoneIntroMusicTableRow {
            id: row.get::<_, i32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            sound_id: row.get::<_, i32>(2)?.into(),
            priority: row.get::<_, i32>(3)?.into(),
            min_delay_minutes: row.get::<_, i32>(4)?.into(),
        });
    }
    Ok(zone_intro_music_table::ZoneIntroMusicTable { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ZoneMusic() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        set_name,
        silence_interval_min_0,
        silence_interval_min_1,
        silence_interval_max_0,
        silence_interval_max_1,
        sounds_0,
        sounds_1
    FROM `ZoneMusic`;"
    )
}


pub(crate) fn zone_music_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<zone_music::ZoneMusic, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(zone_music::ZoneMusicRow {
            id: row.get::<_, i32>(0)?.into(),
            set_name: row.get::<_, String>(1)?.into(),
            silence_interval_min: [row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
            silence_interval_max: [row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
            sounds: [row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(),             ],
        });
    }
    Ok(zone_music::ZoneMusic { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtChanceToMeleeCrit() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtChanceToMeleeCrit`;"
    )
}


pub(crate) fn gt_chance_to_melee_crit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_chance_to_melee_crit::gtChanceToMeleeCrit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_chance_to_melee_crit::gtChanceToMeleeCritRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_chance_to_melee_crit::gtChanceToMeleeCrit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtChanceToMeleeCritBase() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtChanceToMeleeCritBase`;"
    )
}


pub(crate) fn gt_chance_to_melee_crit_base_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_chance_to_melee_crit_base::gtChanceToMeleeCritBase, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_chance_to_melee_crit_base::gtChanceToMeleeCritBaseRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_chance_to_melee_crit_base::gtChanceToMeleeCritBase { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtChanceToSpellCrit() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtChanceToSpellCrit`;"
    )
}


pub(crate) fn gt_chance_to_spell_crit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_chance_to_spell_crit::gtChanceToSpellCrit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_chance_to_spell_crit::gtChanceToSpellCritRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_chance_to_spell_crit::gtChanceToSpellCrit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtChanceToSpellCritBase() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtChanceToSpellCritBase`;"
    )
}


pub(crate) fn gt_chance_to_spell_crit_base_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_chance_to_spell_crit_base::gtChanceToSpellCritBase, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_chance_to_spell_crit_base::gtChanceToSpellCritBaseRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_chance_to_spell_crit_base::gtChanceToSpellCritBase { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtCombatRatings() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtCombatRatings`;"
    )
}


pub(crate) fn gt_combat_ratings_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_combat_ratings::gtCombatRatings, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_combat_ratings::gtCombatRatingsRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_combat_ratings::gtCombatRatings { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtNPCManaCostScaler() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtNPCManaCostScaler`;"
    )
}


pub(crate) fn gt_npc_mana_cost_scaler_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_npc_mana_cost_scaler::gtNPCManaCostScaler, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_npc_mana_cost_scaler::gtNPCManaCostScalerRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_npc_mana_cost_scaler::gtNPCManaCostScaler { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtOCTRegenHP() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtOCTRegenHP`;"
    )
}


pub(crate) fn gt_oct_regen_hp_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_oct_regen_hp::gtOCTRegenHP, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_oct_regen_hp::gtOCTRegenHPRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_oct_regen_hp::gtOCTRegenHP { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtOCTRegenMP() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtOCTRegenMP`;"
    )
}


pub(crate) fn gt_oct_regen_mp_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_oct_regen_mp::gtOCTRegenMP, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_oct_regen_mp::gtOCTRegenMPRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_oct_regen_mp::gtOCTRegenMP { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtRegenHPPerSpt() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtRegenHPPerSpt`;"
    )
}


pub(crate) fn gt_regen_hp_per_spt_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_regen_hp_per_spt::gtRegenHPPerSpt, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_regen_hp_per_spt::gtRegenHPPerSptRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_regen_hp_per_spt::gtRegenHPPerSpt { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn gtRegenMPPerSpt() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        data
    FROM `gtRegenMPPerSpt`;"
    )
}


pub(crate) fn gt_regen_mp_per_spt_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gt_regen_mp_per_spt::gtRegenMPPerSpt, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gt_regen_mp_per_spt::gtRegenMPPerSptRow {
            data: row.get::<_, f32>(0)?.into(),
        });
    }
    Ok(gt_regen_mp_per_spt::gtRegenMPPerSpt { rows: data })
}
pub(crate) fn generate_dbc_for(name: &str, conn: &rusqlite::Connection, mut writer: impl std::io::Write) -> Result<(), SqliteError> {
    match name {
        "AnimationData" => {
            let (_create, _insert, select) = AnimationData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = animation_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "AreaPOI" => {
            let (_create, _insert, select) = AreaPOI();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = area_poi_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "AreaTable" => {
            let (_create, _insert, select) = AreaTable();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = area_table_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "AreaTrigger" => {
            let (_create, _insert, select) = AreaTrigger();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = area_trigger_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "AttackAnimKits" => {
            let (_create, _insert, select) = AttackAnimKits();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = attack_anim_kits_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "AttackAnimTypes" => {
            let (_create, _insert, select) = AttackAnimTypes();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = attack_anim_types_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "AuctionHouse" => {
            let (_create, _insert, select) = AuctionHouse();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = auction_house_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "BankBagSlotPrices" => {
            let (_create, _insert, select) = BankBagSlotPrices();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = bank_bag_slot_prices_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "BattlemasterList" => {
            let (_create, _insert, select) = BattlemasterList();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = battlemaster_list_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CameraShakes" => {
            let (_create, _insert, select) = CameraShakes();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = camera_shakes_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Cfg_Categories" => {
            let (_create, _insert, select) = Cfg_Categories();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = cfg_categories_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Cfg_Configs" => {
            let (_create, _insert, select) = Cfg_Configs();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = cfg_configs_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharBaseInfo" => {
            let (_create, _insert, select) = CharBaseInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_base_info_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharHairGeosets" => {
            let (_create, _insert, select) = CharHairGeosets();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_hair_geosets_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharHairTextures" => {
            let (_create, _insert, select) = CharHairTextures();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_hair_textures_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharSections" => {
            let (_create, _insert, select) = CharSections();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_sections_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharStartOutfit" => {
            let (_create, _insert, select) = CharStartOutfit();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_start_outfit_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharTitles" => {
            let (_create, _insert, select) = CharTitles();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_titles_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharVariations" => {
            let (_create, _insert, select) = CharVariations();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_variations_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharacterFacialHairStyles" => {
            let (_create, _insert, select) = CharacterFacialHairStyles();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = character_facial_hair_styles_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ChatChannels" => {
            let (_create, _insert, select) = ChatChannels();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = chat_channels_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ChatProfanity" => {
            let (_create, _insert, select) = ChatProfanity();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = chat_profanity_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ChrClasses" => {
            let (_create, _insert, select) = ChrClasses();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = chr_classes_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ChrRaces" => {
            let (_create, _insert, select) = ChrRaces();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = chr_races_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CinematicCamera" => {
            let (_create, _insert, select) = CinematicCamera();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = cinematic_camera_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CinematicSequences" => {
            let (_create, _insert, select) = CinematicSequences();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = cinematic_sequences_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureDisplayInfo" => {
            let (_create, _insert, select) = CreatureDisplayInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_display_info_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureDisplayInfoExtra" => {
            let (_create, _insert, select) = CreatureDisplayInfoExtra();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_display_info_extra_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureFamily" => {
            let (_create, _insert, select) = CreatureFamily();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_family_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureModelData" => {
            let (_create, _insert, select) = CreatureModelData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_model_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureSoundData" => {
            let (_create, _insert, select) = CreatureSoundData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_sound_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureSpellData" => {
            let (_create, _insert, select) = CreatureSpellData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_spell_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CreatureType" => {
            let (_create, _insert, select) = CreatureType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = creature_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "DeathThudLookups" => {
            let (_create, _insert, select) = DeathThudLookups();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = death_thud_lookups_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "DeclinedWord" => {
            let (_create, _insert, select) = DeclinedWord();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = declined_word_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "DeclinedWordCases" => {
            let (_create, _insert, select) = DeclinedWordCases();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = declined_word_cases_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "DurabilityCosts" => {
            let (_create, _insert, select) = DurabilityCosts();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = durability_costs_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "DurabilityQuality" => {
            let (_create, _insert, select) = DurabilityQuality();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = durability_quality_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Emotes" => {
            let (_create, _insert, select) = Emotes();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = emotes_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "EmotesText" => {
            let (_create, _insert, select) = EmotesText();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = emotes_text_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "EmotesTextData" => {
            let (_create, _insert, select) = EmotesTextData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = emotes_text_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "EmotesTextSound" => {
            let (_create, _insert, select) = EmotesTextSound();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = emotes_text_sound_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "EnvironmentalDamage" => {
            let (_create, _insert, select) = EnvironmentalDamage();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = environmental_damage_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Exhaustion" => {
            let (_create, _insert, select) = Exhaustion();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = exhaustion_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Faction" => {
            let (_create, _insert, select) = Faction();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = faction_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "FactionGroup" => {
            let (_create, _insert, select) = FactionGroup();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = faction_group_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "FactionTemplate" => {
            let (_create, _insert, select) = FactionTemplate();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = faction_template_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "FootprintTextures" => {
            let (_create, _insert, select) = FootprintTextures();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = footprint_textures_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "FootstepTerrainLookup" => {
            let (_create, _insert, select) = FootstepTerrainLookup();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = footstep_terrain_lookup_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GMSurveyCurrentSurvey" => {
            let (_create, _insert, select) = GMSurveyCurrentSurvey();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gm_survey_current_survey_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GMSurveyQuestions" => {
            let (_create, _insert, select) = GMSurveyQuestions();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gm_survey_questions_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GMSurveySurveys" => {
            let (_create, _insert, select) = GMSurveySurveys();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gm_survey_surveys_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GMTicketCategory" => {
            let (_create, _insert, select) = GMTicketCategory();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gm_ticket_category_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GameObjectArtKit" => {
            let (_create, _insert, select) = GameObjectArtKit();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = game_object_art_kit_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GameObjectDisplayInfo" => {
            let (_create, _insert, select) = GameObjectDisplayInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = game_object_display_info_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GameTables" => {
            let (_create, _insert, select) = GameTables();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = game_tables_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GameTips" => {
            let (_create, _insert, select) = GameTips();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = game_tips_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GemProperties" => {
            let (_create, _insert, select) = GemProperties();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gem_properties_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GroundEffectDoodad" => {
            let (_create, _insert, select) = GroundEffectDoodad();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = ground_effect_doodad_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "GroundEffectTexture" => {
            let (_create, _insert, select) = GroundEffectTexture();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = ground_effect_texture_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "HelmetGeosetVisData" => {
            let (_create, _insert, select) = HelmetGeosetVisData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = helmet_geoset_vis_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Item" => {
            let (_create, _insert, select) = Item();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemBagFamily" => {
            let (_create, _insert, select) = ItemBagFamily();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_bag_family_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemClass" => {
            let (_create, _insert, select) = ItemClass();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_class_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemCondExtCosts" => {
            let (_create, _insert, select) = ItemCondExtCosts();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_cond_ext_costs_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemDisplayInfo" => {
            let (_create, _insert, select) = ItemDisplayInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_display_info_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemExtendedCost" => {
            let (_create, _insert, select) = ItemExtendedCost();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_extended_cost_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemGroupSounds" => {
            let (_create, _insert, select) = ItemGroupSounds();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_group_sounds_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemPetFood" => {
            let (_create, _insert, select) = ItemPetFood();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_pet_food_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemRandomProperties" => {
            let (_create, _insert, select) = ItemRandomProperties();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_random_properties_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemRandomSuffix" => {
            let (_create, _insert, select) = ItemRandomSuffix();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_random_suffix_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemSet" => {
            let (_create, _insert, select) = ItemSet();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_set_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemSubClass" => {
            let (_create, _insert, select) = ItemSubClass();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_sub_class_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemSubClassMask" => {
            let (_create, _insert, select) = ItemSubClassMask();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_sub_class_mask_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemVisualEffects" => {
            let (_create, _insert, select) = ItemVisualEffects();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_visual_effects_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ItemVisuals" => {
            let (_create, _insert, select) = ItemVisuals();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_visuals_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LFGDungeons" => {
            let (_create, _insert, select) = LFGDungeons();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = lfg_dungeons_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LanguageWords" => {
            let (_create, _insert, select) = LanguageWords();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = language_words_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Languages" => {
            let (_create, _insert, select) = Languages();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = languages_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Light" => {
            let (_create, _insert, select) = Light();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = light_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LightFloatBand" => {
            let (_create, _insert, select) = LightFloatBand();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = light_float_band_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LightIntBand" => {
            let (_create, _insert, select) = LightIntBand();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = light_int_band_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LightParams" => {
            let (_create, _insert, select) = LightParams();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = light_params_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LightSkybox" => {
            let (_create, _insert, select) = LightSkybox();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = light_skybox_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LiquidType" => {
            let (_create, _insert, select) = LiquidType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = liquid_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LoadingScreenTaxiSplines" => {
            let (_create, _insert, select) = LoadingScreenTaxiSplines();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = loading_screen_taxi_splines_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LoadingScreens" => {
            let (_create, _insert, select) = LoadingScreens();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = loading_screens_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Lock" => {
            let (_create, _insert, select) = Lock();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = lock_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "LockType" => {
            let (_create, _insert, select) = LockType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = lock_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "MailTemplate" => {
            let (_create, _insert, select) = MailTemplate();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = mail_template_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Map" => {
            let (_create, _insert, select) = Map();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = map_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Material" => {
            let (_create, _insert, select) = Material();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = material_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "NPCSounds" => {
            let (_create, _insert, select) = NPCSounds();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = npc_sounds_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "NameGen" => {
            let (_create, _insert, select) = NameGen();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = name_gen_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "NamesProfanity" => {
            let (_create, _insert, select) = NamesProfanity();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = names_profanity_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "NamesReserved" => {
            let (_create, _insert, select) = NamesReserved();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = names_reserved_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Package" => {
            let (_create, _insert, select) = Package();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = package_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "PageTextMaterial" => {
            let (_create, _insert, select) = PageTextMaterial();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = page_text_material_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "PaperDollItemFrame" => {
            let (_create, _insert, select) = PaperDollItemFrame();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = paper_doll_item_frame_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ParticleColor" => {
            let (_create, _insert, select) = ParticleColor();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = particle_color_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "PetLoyalty" => {
            let (_create, _insert, select) = PetLoyalty();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = pet_loyalty_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "PetPersonality" => {
            let (_create, _insert, select) = PetPersonality();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = pet_personality_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "PetitionType" => {
            let (_create, _insert, select) = PetitionType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = petition_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "QuestInfo" => {
            let (_create, _insert, select) = QuestInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = quest_info_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "QuestSort" => {
            let (_create, _insert, select) = QuestSort();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = quest_sort_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "RandPropPoints" => {
            let (_create, _insert, select) = RandPropPoints();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = rand_prop_points_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Resistances" => {
            let (_create, _insert, select) = Resistances();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = resistances_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ServerMessages" => {
            let (_create, _insert, select) = ServerMessages();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = server_messages_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SheatheSoundLookups" => {
            let (_create, _insert, select) = SheatheSoundLookups();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sheathe_sound_lookups_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SkillCostsData" => {
            let (_create, _insert, select) = SkillCostsData();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = skill_costs_data_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SkillLine" => {
            let (_create, _insert, select) = SkillLine();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = skill_line_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SkillLineAbility" => {
            let (_create, _insert, select) = SkillLineAbility();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = skill_line_ability_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SkillLineCategory" => {
            let (_create, _insert, select) = SkillLineCategory();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = skill_line_category_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SkillRaceClassInfo" => {
            let (_create, _insert, select) = SkillRaceClassInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = skill_race_class_info_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SkillTiers" => {
            let (_create, _insert, select) = SkillTiers();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = skill_tiers_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SoundAmbience" => {
            let (_create, _insert, select) = SoundAmbience();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sound_ambience_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SoundEntries" => {
            let (_create, _insert, select) = SoundEntries();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sound_entries_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SoundProviderPreferences" => {
            let (_create, _insert, select) = SoundProviderPreferences();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sound_provider_preferences_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SoundSamplePreferences" => {
            let (_create, _insert, select) = SoundSamplePreferences();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sound_sample_preferences_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SoundWaterType" => {
            let (_create, _insert, select) = SoundWaterType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sound_water_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpamMessages" => {
            let (_create, _insert, select) = SpamMessages();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spam_messages_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Spell" => {
            let (_create, _insert, select) = Spell();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellCastTimes" => {
            let (_create, _insert, select) = SpellCastTimes();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_cast_times_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellCategory" => {
            let (_create, _insert, select) = SpellCategory();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_category_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellChainEffects" => {
            let (_create, _insert, select) = SpellChainEffects();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_chain_effects_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellDispelType" => {
            let (_create, _insert, select) = SpellDispelType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_dispel_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellDuration" => {
            let (_create, _insert, select) = SpellDuration();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_duration_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellEffectCameraShakes" => {
            let (_create, _insert, select) = SpellEffectCameraShakes();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_effect_camera_shakes_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellFocusObject" => {
            let (_create, _insert, select) = SpellFocusObject();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_focus_object_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellIcon" => {
            let (_create, _insert, select) = SpellIcon();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_icon_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellItemEnchantment" => {
            let (_create, _insert, select) = SpellItemEnchantment();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_item_enchantment_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellItemEnchantmentCondition" => {
            let (_create, _insert, select) = SpellItemEnchantmentCondition();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_item_enchantment_condition_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellMechanic" => {
            let (_create, _insert, select) = SpellMechanic();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_mechanic_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellMissileMotion" => {
            let (_create, _insert, select) = SpellMissileMotion();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_missile_motion_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellRadius" => {
            let (_create, _insert, select) = SpellRadius();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_radius_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellRange" => {
            let (_create, _insert, select) = SpellRange();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_range_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellShapeshiftForm" => {
            let (_create, _insert, select) = SpellShapeshiftForm();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_shapeshift_form_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellVisual" => {
            let (_create, _insert, select) = SpellVisual();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_visual_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellVisualEffectName" => {
            let (_create, _insert, select) = SpellVisualEffectName();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_visual_effect_name_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellVisualKit" => {
            let (_create, _insert, select) = SpellVisualKit();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_visual_kit_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SpellVisualPrecastTransitions" => {
            let (_create, _insert, select) = SpellVisualPrecastTransitions();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_visual_precast_transitions_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "StableSlotPrices" => {
            let (_create, _insert, select) = StableSlotPrices();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = stable_slot_prices_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Startup_Strings" => {
            let (_create, _insert, select) = Startup_Strings();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = startup_strings_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Stationery" => {
            let (_create, _insert, select) = Stationery();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = stationery_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "StringLookups" => {
            let (_create, _insert, select) = StringLookups();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = string_lookups_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "SummonProperties" => {
            let (_create, _insert, select) = SummonProperties();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = summon_properties_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Talent" => {
            let (_create, _insert, select) = Talent();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = talent_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TalentTab" => {
            let (_create, _insert, select) = TalentTab();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = talent_tab_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TaxiNodes" => {
            let (_create, _insert, select) = TaxiNodes();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = taxi_nodes_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TaxiPath" => {
            let (_create, _insert, select) = TaxiPath();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = taxi_path_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TaxiPathNode" => {
            let (_create, _insert, select) = TaxiPathNode();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = taxi_path_node_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TerrainType" => {
            let (_create, _insert, select) = TerrainType();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = terrain_type_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TerrainTypeSounds" => {
            let (_create, _insert, select) = TerrainTypeSounds();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = terrain_type_sounds_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TotemCategory" => {
            let (_create, _insert, select) = TotemCategory();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = totem_category_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TransportAnimation" => {
            let (_create, _insert, select) = TransportAnimation();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = transport_animation_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "TransportPhysics" => {
            let (_create, _insert, select) = TransportPhysics();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = transport_physics_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "UISoundLookups" => {
            let (_create, _insert, select) = UISoundLookups();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = ui_sound_lookups_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "UnitBlood" => {
            let (_create, _insert, select) = UnitBlood();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = unit_blood_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "UnitBloodLevels" => {
            let (_create, _insert, select) = UnitBloodLevels();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = unit_blood_levels_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "VideoHardware" => {
            let (_create, _insert, select) = VideoHardware();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = video_hardware_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "VocalUISounds" => {
            let (_create, _insert, select) = VocalUISounds();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = vocal_ui_sounds_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WMOAreaTable" => {
            let (_create, _insert, select) = WMOAreaTable();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = wmo_area_table_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WeaponImpactSounds" => {
            let (_create, _insert, select) = WeaponImpactSounds();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = weapon_impact_sounds_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WeaponSwingSounds2" => {
            let (_create, _insert, select) = WeaponSwingSounds2();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = weapon_swing_sounds2_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "Weather" => {
            let (_create, _insert, select) = Weather();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = weather_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldMapArea" => {
            let (_create, _insert, select) = WorldMapArea();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_map_area_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldMapContinent" => {
            let (_create, _insert, select) = WorldMapContinent();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_map_continent_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldMapOverlay" => {
            let (_create, _insert, select) = WorldMapOverlay();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_map_overlay_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldMapTransforms" => {
            let (_create, _insert, select) = WorldMapTransforms();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_map_transforms_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldSafeLocs" => {
            let (_create, _insert, select) = WorldSafeLocs();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_safe_locs_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldStateUI" => {
            let (_create, _insert, select) = WorldStateUI();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_state_ui_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WorldStateZoneSounds" => {
            let (_create, _insert, select) = WorldStateZoneSounds();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = world_state_zone_sounds_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "WowError_Strings" => {
            let (_create, _insert, select) = WowError_Strings();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = wow_error_strings_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ZoneIntroMusicTable" => {
            let (_create, _insert, select) = ZoneIntroMusicTable();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = zone_intro_music_table_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "ZoneMusic" => {
            let (_create, _insert, select) = ZoneMusic();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = zone_music_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtChanceToMeleeCrit" => {
            let (_create, _insert, select) = gtChanceToMeleeCrit();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_chance_to_melee_crit_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtChanceToMeleeCritBase" => {
            let (_create, _insert, select) = gtChanceToMeleeCritBase();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_chance_to_melee_crit_base_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtChanceToSpellCrit" => {
            let (_create, _insert, select) = gtChanceToSpellCrit();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_chance_to_spell_crit_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtChanceToSpellCritBase" => {
            let (_create, _insert, select) = gtChanceToSpellCritBase();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_chance_to_spell_crit_base_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtCombatRatings" => {
            let (_create, _insert, select) = gtCombatRatings();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_combat_ratings_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtNPCManaCostScaler" => {
            let (_create, _insert, select) = gtNPCManaCostScaler();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_npc_mana_cost_scaler_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtOCTRegenHP" => {
            let (_create, _insert, select) = gtOCTRegenHP();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_oct_regen_hp_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtOCTRegenMP" => {
            let (_create, _insert, select) = gtOCTRegenMP();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_oct_regen_mp_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtRegenHPPerSpt" => {
            let (_create, _insert, select) = gtRegenHPPerSpt();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_regen_hp_per_spt_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "gtRegenMPPerSpt" => {
            let (_create, _insert, select) = gtRegenMPPerSpt();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = gt_regen_mp_per_spt_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        _ => Err(SqliteError::FilenameNotFound { name: name.to_string() }),
    }
}

