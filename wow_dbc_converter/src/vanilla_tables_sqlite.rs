use crate::SqliteError;
use rusqlite::{Connection, params};
use wow_dbc::DbcTable;
use wow_dbc::vanilla_tables::*;

pub(crate) fn write_to_sqlite(conn: &mut Connection, file_name: &str, file_contents: &mut &[u8]) -> Result<(), SqliteError> {
    let tx = conn.transaction()?;

    match file_name {
        "AnimationData.dbc" => {
            let data = animation_data::AnimationData::read(file_contents)?;
            let (table, insert) = AnimationData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.weapon_flags.as_int(),
                row.body_flags,
                row.unknown,
                row.fallback.id,
                row.behaviour.id,
                ])?;
            }
        }
        "AreaPOI.dbc" => {
            let data = area_poi::AreaPOI::read(file_contents)?;
            let (table, insert) = AreaPOI();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.importance,
                row.icon,
                row.faction.id,
                row.location_x,
                row.location_y,
                row.location_z,
                row.map.id,
                row.flags,
                row.area_table.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.description.en_gb,
                &row.description.ko_kr,
                &row.description.fr_fr,
                &row.description.de_de,
                &row.description.en_cn,
                &row.description.en_tw,
                &row.description.es_es,
                &row.description.es_mx,
                &row.description.flags,
                row.world_state.id,
                ])?;
            }
        }
        "AreaTable.dbc" => {
            let data = area_table::AreaTable::read(file_contents)?;
            let (table, insert) = AreaTable();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.parent_area_table.id,
                row.area_bit,
                row.flags.as_int(),
                row.sound_preferences.id,
                row.sound_preferences_underwater.id,
                row.sound_ambience.id,
                row.zone_music.id,
                row.zone_music_intro.id,
                row.exploration_level,
                &row.area_name.en_gb,
                &row.area_name.ko_kr,
                &row.area_name.fr_fr,
                &row.area_name.de_de,
                &row.area_name.en_cn,
                &row.area_name.en_tw,
                &row.area_name.es_es,
                &row.area_name.es_mx,
                &row.area_name.flags,
                row.faction_group.id,
                row.liquid_type.id,
                row.min_elevation,
                row.ambient_multiplier,
                row.light.id,
                ])?;
            }
        }
        "AreaTrigger.dbc" => {
            let data = area_trigger::AreaTrigger::read(file_contents)?;
            let (table, insert) = AreaTrigger();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.location_x,
                row.location_y,
                row.location_z,
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
            let (table, insert) = AttackAnimKits();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.animation_data.id,
                row.attack_anim_type.id,
                row.animation_frequency,
                row.flags.as_int(),
                ])?;
            }
        }
        "AttackAnimTypes.dbc" => {
            let data = attack_anim_types::AttackAnimTypes::read(file_contents)?;
            let (table, insert) = AttackAnimTypes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
        }
        "AuctionHouse.dbc" => {
            let data = auction_house::AuctionHouse::read(file_contents)?;
            let (table, insert) = AuctionHouse();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.faction.id,
                row.deposit_rate,
                row.consignment_rate,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "BankBagSlotPrices.dbc" => {
            let data = bank_bag_slot_prices::BankBagSlotPrices::read(file_contents)?;
            let (table, insert) = BankBagSlotPrices();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.cost,
                ])?;
            }
        }
        "CameraShakes.dbc" => {
            let data = camera_shakes::CameraShakes::read(file_contents)?;
            let (table, insert) = CameraShakes();
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
            let (table, insert) = Cfg_Categories();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.category.as_int(),
                row.region.as_int(),
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "Cfg_Configs.dbc" => {
            let data = cfg_configs::Cfg_Configs::read(file_contents)?;
            let (table, insert) = Cfg_Configs();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.realm_type,
                row.pvp_allowed,
                row.roleplaying,
                ])?;
            }
        }
        "CharBaseInfo.dbc" => {
            let data = char_base_info::CharBaseInfo::read(file_contents)?;
            let (table, insert) = CharBaseInfo();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.race.id,
                row.class.id,
                ])?;
            }
        }
        "CharHairGeosets.dbc" => {
            let data = char_hair_geosets::CharHairGeosets::read(file_contents)?;
            let (table, insert) = CharHairGeosets();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race.id,
                row.gender.as_int(),
                row.variation,
                row.geoset,
                row.show_scalp.as_int(),
                ])?;
            }
        }
        "CharHairTextures.dbc" => {
            let data = char_hair_textures::CharHairTextures::read(file_contents)?;
            let (table, insert) = CharHairTextures();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race.id,
                row.gender.as_int(),
                row.variation,
                row.geoset,
                row.show_scalp,
                row.unknown_padding[0],
                row.unknown_padding[1],
                ])?;
            }
        }
        "CharSections.dbc" => {
            let data = char_sections::CharSections::read(file_contents)?;
            let (table, insert) = CharSections();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race.id,
                row.gender.as_int(),
                row.ty.as_int(),
                row.variation_index,
                row.colour_index,
                row.texture_name[0],
                row.texture_name[1],
                row.texture_name[2],
                row.npc_only,
                ])?;
            }
        }
        "CharStartOutfit.dbc" => {
            let data = char_start_outfit::CharStartOutfit::read(file_contents)?;
            let (table, insert) = CharStartOutfit();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.race.id,
                row.class.id,
                row.gender.as_int(),
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
                row.display_id[0],
                row.display_id[1],
                row.display_id[2],
                row.display_id[3],
                row.display_id[4],
                row.display_id[5],
                row.display_id[6],
                row.display_id[7],
                row.display_id[8],
                row.display_id[9],
                row.display_id[10],
                row.display_id[11],
                row.inv_slot_id[0],
                row.inv_slot_id[1],
                row.inv_slot_id[2],
                row.inv_slot_id[3],
                row.inv_slot_id[4],
                row.inv_slot_id[5],
                row.inv_slot_id[6],
                row.inv_slot_id[7],
                row.inv_slot_id[8],
                row.inv_slot_id[9],
                row.inv_slot_id[10],
                row.inv_slot_id[11],
                ])?;
            }
        }
        "CharVariations.dbc" => {
            let data = char_variations::CharVariations::read(file_contents)?;
            let (table, insert) = CharVariations();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.gender.as_int(),
                row.unknown_1,
                row.mask[0],
                row.mask[1],
                row.unknown_2,
                ])?;
            }
        }
        "CharacterCreateCameras.dbc" => {
            let data = character_create_cameras::CharacterCreateCameras::read(file_contents)?;
            let (table, insert) = CharacterCreateCameras();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.unknown[0],
                row.unknown[1],
                row.unknown_2[0],
                row.unknown_2[1],
                row.unknown_2[2],
                ])?;
            }
        }
        "CharacterFacialHairStyles.dbc" => {
            let data = character_facial_hair_styles::CharacterFacialHairStyles::read(file_contents)?;
            let (table, insert) = CharacterFacialHairStyles();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.race.id,
                row.gender.as_int(),
                row.variation_id,
                row.geoset[0],
                row.geoset[1],
                row.geoset[2],
                row.geoset[3],
                row.geoset[4],
                row.geoset[5],
                ])?;
            }
        }
        "ChatChannels.dbc" => {
            let data = chat_channels::ChatChannels::read(file_contents)?;
            let (table, insert) = ChatChannels();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags.as_int(),
                row.faction_group.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.shortcut.en_gb,
                &row.shortcut.ko_kr,
                &row.shortcut.fr_fr,
                &row.shortcut.de_de,
                &row.shortcut.en_cn,
                &row.shortcut.en_tw,
                &row.shortcut.es_es,
                &row.shortcut.es_mx,
                &row.shortcut.flags,
                ])?;
            }
        }
        "ChatProfanity.dbc" => {
            let data = chat_profanity::ChatProfanity::read(file_contents)?;
            let (table, insert) = ChatProfanity();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text,
                ])?;
            }
        }
        "ChrClasses.dbc" => {
            let data = chr_classes::ChrClasses::read(file_contents)?;
            let (table, insert) = ChrClasses();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.player_class,
                row.damage_bonus_stat,
                row.power_type.as_int(),
                &row.pet_name_token,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.filename,
                row.class_mask,
                row.hybrid_class,
                ])?;
            }
        }
        "ChrRaces.dbc" => {
            let data = chr_races::ChrRaces::read(file_contents)?;
            let (table, insert) = ChrRaces();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags.as_int(),
                row.faction.id,
                row.exploration_sound.id,
                row.male_display.id,
                row.female_display.id,
                &row.client_prefix,
                row.speed_modifier,
                row.base_lang.as_int(),
                row.creature_type.id,
                row.login_effect.id,
                row.unknown1,
                row.res_sickness_spell.id,
                row.splash_sound_entry.id,
                row.unknown2,
                &row.client_file_path,
                row.cinematic_sequence.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.facial_hair_customisation[0],
                row.facial_hair_customisation[1],
                &row.hair_customisation,
                ])?;
            }
        }
        "CinematicCamera.dbc" => {
            let data = cinematic_camera::CinematicCamera::read(file_contents)?;
            let (table, insert) = CinematicCamera();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model,
                row.sound_entry.id,
                row.location_x,
                row.location_y,
                row.location_z,
                row.rotation,
                ])?;
            }
        }
        "CinematicSequences.dbc" => {
            let data = cinematic_sequences::CinematicSequences::read(file_contents)?;
            let (table, insert) = CinematicSequences();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_entry.id,
                row.cinematic_camera_1[0],
                row.cinematic_camera_1[1],
                row.cinematic_camera_1[2],
                row.cinematic_camera_1[3],
                row.cinematic_camera_1[4],
                row.cinematic_camera_1[5],
                row.cinematic_camera_1[6],
                row.cinematic_camera_1[7],
                ])?;
            }
        }
        "CreatureDisplayInfo.dbc" => {
            let data = creature_display_info::CreatureDisplayInfo::read(file_contents)?;
            let (table, insert) = CreatureDisplayInfo();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.model.id,
                row.sound.id,
                row.extended_display_info.id,
                row.creature_model_scale,
                row.creature_model_alpha,
                row.texture_variation[0],
                row.texture_variation[1],
                row.texture_variation[2],
                row.size.as_int(),
                row.blood.id,
                row.npc_sound.id,
                ])?;
            }
        }
        "CreatureDisplayInfoExtra.dbc" => {
            let data = creature_display_info_extra::CreatureDisplayInfoExtra::read(file_contents)?;
            let (table, insert) = CreatureDisplayInfoExtra();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.display_race.id,
                row.gender.as_int(),
                row.skin,
                row.face,
                row.hair_style,
                row.hair_colour,
                row.facial_hair,
                row.npc_item_display[0],
                row.npc_item_display[1],
                row.npc_item_display[2],
                row.npc_item_display[3],
                row.npc_item_display[4],
                row.npc_item_display[5],
                row.npc_item_display[6],
                row.npc_item_display[7],
                row.npc_item_display[8],
                row.flags,
                &row.bake_name,
                ])?;
            }
        }
        "CreatureFamily.dbc" => {
            let data = creature_family::CreatureFamily::read(file_contents)?;
            let (table, insert) = CreatureFamily();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.min_scale,
                row.min_scale_level,
                row.max_scale,
                row.max_scale_level,
                row.pet_food_mask,
                row.pet_talent_type,
                row.category,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.icon_path,
                ])?;
            }
        }
        "CreatureModelData.dbc" => {
            let data = creature_model_data::CreatureModelData::read(file_contents)?;
            let (table, insert) = CreatureModelData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                &row.model_path,
                row.size.as_int(),
                row.model_scale,
                row.blood.id,
                row.footprint_texture.id,
                row.footprint_texture_length,
                row.footprint_texture_width,
                row.footprint_texture_scale,
                row.foley_material,
                row.footstep_shake_size,
                row.death_thud_shake_size,
                row.collision_width,
                row.collision_height,
                row.mount_height,
                ])?;
            }
        }
        "CreatureSoundData.dbc" => {
            let data = creature_sound_data::CreatureSoundData::read(file_contents)?;
            let (table, insert) = CreatureSoundData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_exertion.id,
                row.sound_exertion_critical.id,
                row.sound_injury.id,
                row.sound_injury_critical.id,
                row.sound_injury_crushing_blow.id,
                row.sound_death.id,
                row.sound_stun.id,
                row.sound_stand.id,
                row.sound_footstep.id,
                row.sound_aggro.id,
                row.sound_wing_flap.id,
                row.sound_wing_glide.id,
                row.sound_alert.id,
                row.sound_fidget.id,
                row.custom_attack,
                row.npc_sound.id,
                row.loop_sound.id,
                row.creature_impact_type,
                row.sound_jump_start.id,
                row.sound_jump_end.id,
                row.sound_pet_attack.id,
                row.sound_pet_order.id,
                row.sound_pet_dismiss.id,
                row.fidget_delay_seconds_min,
                row.fidget_delay_seconds_max,
                row.birth_sound.id,
                row.spell_cast_directed_sound.id,
                row.submerge_sound.id,
                row.submerged_sound.id,
                ])?;
            }
        }
        "CreatureSpellData.dbc" => {
            let data = creature_spell_data::CreatureSpellData::read(file_contents)?;
            let (table, insert) = CreatureSpellData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.spell[0],
                row.spell[1],
                row.spell[2],
                row.spell[3],
                row.cooldown_time_1[0],
                row.cooldown_time_1[1],
                row.cooldown_time_1[2],
                row.cooldown_time_1[3],
                ])?;
            }
        }
        "CreatureType.dbc" => {
            let data = creature_type::CreatureType::read(file_contents)?;
            let (table, insert) = CreatureType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.flags,
                ])?;
            }
        }
        "DeathThudLookups.dbc" => {
            let data = death_thud_lookups::DeathThudLookups::read(file_contents)?;
            let (table, insert) = DeathThudLookups();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.size.as_int(),
                row.terrain_type.id,
                row.sound_entry.id,
                row.sound_entry_water.id,
                ])?;
            }
        }
        "DurabilityCosts.dbc" => {
            let data = durability_costs::DurabilityCosts::read(file_contents)?;
            let (table, insert) = DurabilityCosts();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.weapon_subclass_cost[0],
                row.weapon_subclass_cost[1],
                row.weapon_subclass_cost[2],
                row.weapon_subclass_cost[3],
                row.weapon_subclass_cost[4],
                row.weapon_subclass_cost[5],
                row.weapon_subclass_cost[6],
                row.weapon_subclass_cost[7],
                row.weapon_subclass_cost[8],
                row.weapon_subclass_cost[9],
                row.weapon_subclass_cost[10],
                row.weapon_subclass_cost[11],
                row.weapon_subclass_cost[12],
                row.weapon_subclass_cost[13],
                row.weapon_subclass_cost[14],
                row.weapon_subclass_cost[15],
                row.weapon_subclass_cost[16],
                row.weapon_subclass_cost[17],
                row.weapon_subclass_cost[18],
                row.weapon_subclass_cost[19],
                row.weapon_subclass_cost[20],
                row.armour_subclass_cost[0],
                row.armour_subclass_cost[1],
                row.armour_subclass_cost[2],
                row.armour_subclass_cost[3],
                row.armour_subclass_cost[4],
                row.armour_subclass_cost[5],
                row.armour_subclass_cost[6],
                row.armour_subclass_cost[7],
                ])?;
            }
        }
        "DurabilityQuality.dbc" => {
            let data = durability_quality::DurabilityQuality::read(file_contents)?;
            let (table, insert) = DurabilityQuality();
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
            let (table, insert) = Emotes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.emote_slash_command,
                row.animation_data.id,
                row.flags.as_int(),
                row.spec_proc.as_int(),
                row.emote_spec_proc_param,
                row.event_sound_entry.id,
                ])?;
            }
        }
        "EmotesText.dbc" => {
            let data = emotes_text::EmotesText::read(file_contents)?;
            let (table, insert) = EmotesText();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.emote.id,
                row.emote_text_data[0],
                row.emote_text_data[1],
                row.emote_text_data[2],
                row.emote_text_data[3],
                row.emote_text_data[4],
                row.emote_text_data[5],
                row.emote_text_data[6],
                row.emote_text_data[7],
                row.emote_text_data[8],
                row.emote_text_data[9],
                row.emote_text_data[10],
                row.emote_text_data[11],
                row.emote_text_data[12],
                row.emote_text_data[13],
                row.emote_text_data[14],
                row.emote_text_data[15],
                ])?;
            }
        }
        "EmotesTextData.dbc" => {
            let data = emotes_text_data::EmotesTextData::read(file_contents)?;
            let (table, insert) = EmotesTextData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text.en_gb,
                &row.text.ko_kr,
                &row.text.fr_fr,
                &row.text.de_de,
                &row.text.en_cn,
                &row.text.en_tw,
                &row.text.es_es,
                &row.text.es_mx,
                &row.text.flags,
                ])?;
            }
        }
        "EmotesTextSound.dbc" => {
            let data = emotes_text_sound::EmotesTextSound::read(file_contents)?;
            let (table, insert) = EmotesTextSound();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.emotes_text.id,
                row.race.id,
                row.gender.as_int(),
                row.sound.id,
                ])?;
            }
        }
        "EnvironmentalDamage.dbc" => {
            let data = environmental_damage::EnvironmentalDamage::read(file_contents)?;
            let (table, insert) = EnvironmentalDamage();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.en,
                row.spell_visual_kit.id,
                ])?;
            }
        }
        "Exhaustion.dbc" => {
            let data = exhaustion::Exhaustion::read(file_contents)?;
            let (table, insert) = Exhaustion();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.experience,
                row.factor,
                row.outdoor_hours,
                row.inn_hours,
                &row.state_name.en_gb,
                &row.state_name.ko_kr,
                &row.state_name.fr_fr,
                &row.state_name.de_de,
                &row.state_name.en_cn,
                &row.state_name.en_tw,
                &row.state_name.es_es,
                &row.state_name.es_mx,
                &row.state_name.flags,
                row.threshold,
                ])?;
            }
        }
        "Faction.dbc" => {
            let data = faction::Faction::read(file_contents)?;
            let (table, insert) = Faction();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.reputation_index,
                row.reputation_race_mask[0].as_int(),
                row.reputation_race_mask[1].as_int(),
                row.reputation_race_mask[2].as_int(),
                row.reputation_race_mask[3].as_int(),
                row.reputation_class_mask[0],
                row.reputation_class_mask[1],
                row.reputation_class_mask[2],
                row.reputation_class_mask[3],
                row.reputation_base[0],
                row.reputation_base[1],
                row.reputation_base[2],
                row.reputation_base[3],
                row.reputation_flags[0].as_int(),
                row.reputation_flags[1].as_int(),
                row.reputation_flags[2].as_int(),
                row.reputation_flags[3].as_int(),
                row.parent_faction.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.description.en_gb,
                &row.description.ko_kr,
                &row.description.fr_fr,
                &row.description.de_de,
                &row.description.en_cn,
                &row.description.en_tw,
                &row.description.es_es,
                &row.description.es_mx,
                &row.description.flags,
                ])?;
            }
        }
        "FactionGroup.dbc" => {
            let data = faction_group::FactionGroup::read(file_contents)?;
            let (table, insert) = FactionGroup();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.mask_id,
                &row.internal_name,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "FactionTemplate.dbc" => {
            let data = faction_template::FactionTemplate::read(file_contents)?;
            let (table, insert) = FactionTemplate();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.faction.id,
                row.flags.as_int(),
                row.faction_group.id,
                row.friend_group.id,
                row.enemy_group.id,
                row.enemies[0],
                row.enemies[1],
                row.enemies[2],
                row.enemies[3],
                row.friends[0],
                row.friends[1],
                row.friends[2],
                row.friends[3],
                ])?;
            }
        }
        "FootprintTextures.dbc" => {
            let data = footprint_textures::FootprintTextures::read(file_contents)?;
            let (table, insert) = FootprintTextures();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.footstep_file_path,
                ])?;
            }
        }
        "FootstepTerrainLookup.dbc" => {
            let data = footstep_terrain_lookup::FootstepTerrainLookup::read(file_contents)?;
            let (table, insert) = FootstepTerrainLookup();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.creature_footstep.id,
                row.terrain_type.id,
                row.sound_entry.id,
                row.sound_entry_splash.id,
                ])?;
            }
        }
        "GMSurveyCurrentSurvey.dbc" => {
            let data = gm_survey_current_survey::GMSurveyCurrentSurvey::read(file_contents)?;
            let (table, insert) = GMSurveyCurrentSurvey();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.language.as_int(),
                row.gm_survey.id,
                ])?;
            }
        }
        "GMSurveyQuestions.dbc" => {
            let data = gm_survey_questions::GMSurveyQuestions::read(file_contents)?;
            let (table, insert) = GMSurveyQuestions();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.question.en_gb,
                &row.question.ko_kr,
                &row.question.fr_fr,
                &row.question.de_de,
                &row.question.en_cn,
                &row.question.en_tw,
                &row.question.es_es,
                &row.question.es_mx,
                &row.question.flags,
                ])?;
            }
        }
        "GMSurveySurveys.dbc" => {
            let data = gm_survey_surveys::GMSurveySurveys::read(file_contents)?;
            let (table, insert) = GMSurveySurveys();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.questions[0],
                row.questions[1],
                row.questions[2],
                row.questions[3],
                row.questions[4],
                row.questions[5],
                row.questions[6],
                row.questions[7],
                row.questions[8],
                row.questions[9],
                ])?;
            }
        }
        "GMTicketCategory.dbc" => {
            let data = gm_ticket_category::GMTicketCategory::read(file_contents)?;
            let (table, insert) = GMTicketCategory();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "GameObjectArtKit.dbc" => {
            let data = game_object_art_kit::GameObjectArtKit::read(file_contents)?;
            let (table, insert) = GameObjectArtKit();
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
            let (table, insert) = GameObjectDisplayInfo();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model_name,
                row.sound_entry[0],
                row.sound_entry[1],
                row.sound_entry[2],
                row.sound_entry[3],
                row.sound_entry[4],
                row.sound_entry[5],
                row.sound_entry[6],
                row.sound_entry[7],
                row.sound_entry[8],
                row.sound_entry[9],
                ])?;
            }
        }
        "GameTips.dbc" => {
            let data = game_tips::GameTips::read(file_contents)?;
            let (table, insert) = GameTips();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text.en_gb,
                &row.text.ko_kr,
                &row.text.fr_fr,
                &row.text.de_de,
                &row.text.en_cn,
                &row.text.en_tw,
                &row.text.es_es,
                &row.text.es_mx,
                &row.text.flags,
                ])?;
            }
        }
        "GroundEffectDoodad.dbc" => {
            let data = ground_effect_doodad::GroundEffectDoodad::read(file_contents)?;
            let (table, insert) = GroundEffectDoodad();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.internal_id,
                &row.doodad_path,
                ])?;
            }
        }
        "GroundEffectTexture.dbc" => {
            let data = ground_effect_texture::GroundEffectTexture::read(file_contents)?;
            let (table, insert) = GroundEffectTexture();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.doodad[0],
                row.doodad[1],
                row.doodad[2],
                row.doodad[3],
                row.density,
                row.terrain_type.id,
                ])?;
            }
        }
        "HelmetGeosetVisData.dbc" => {
            let data = helmet_geoset_vis_data::HelmetGeosetVisData::read(file_contents)?;
            let (table, insert) = HelmetGeosetVisData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.hide_geoset[0],
                row.hide_geoset[1],
                row.hide_geoset[2],
                row.hide_geoset[3],
                row.hide_geoset[4],
                ])?;
            }
        }
        "ItemBagFamily.dbc" => {
            let data = item_bag_family::ItemBagFamily::read(file_contents)?;
            let (table, insert) = ItemBagFamily();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "ItemClass.dbc" => {
            let data = item_class::ItemClass::read(file_contents)?;
            let (table, insert) = ItemClass();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.subclass_map,
                row.item_class.as_int(),
                &row.class_name.en_gb,
                &row.class_name.ko_kr,
                &row.class_name.fr_fr,
                &row.class_name.de_de,
                &row.class_name.en_cn,
                &row.class_name.en_tw,
                &row.class_name.es_es,
                &row.class_name.es_mx,
                &row.class_name.flags,
                ])?;
            }
        }
        "ItemDisplayInfo.dbc" => {
            let data = item_display_info::ItemDisplayInfo::read(file_contents)?;
            let (table, insert) = ItemDisplayInfo();
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
                row.spell_visual.id,
                row.group_sound_index.id,
                row.helmet_geoset_vis[0],
                row.helmet_geoset_vis[1],
                row.textures[0],
                row.textures[1],
                row.textures[2],
                row.textures[3],
                row.textures[4],
                row.textures[5],
                row.textures[6],
                row.textures[7],
                row.item_visual.id,
                ])?;
            }
        }
        "ItemGroupSounds.dbc" => {
            let data = item_group_sounds::ItemGroupSounds::read(file_contents)?;
            let (table, insert) = ItemGroupSounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_entry[0],
                row.sound_entry[1],
                row.sound_entry[2],
                row.sound_entry[3],
                ])?;
            }
        }
        "ItemPetFood.dbc" => {
            let data = item_pet_food::ItemPetFood::read(file_contents)?;
            let (table, insert) = ItemPetFood();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "ItemRandomProperties.dbc" => {
            let data = item_random_properties::ItemRandomProperties::read(file_contents)?;
            let (table, insert) = ItemRandomProperties();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.spell_item_enchantment[0],
                row.spell_item_enchantment[1],
                row.spell_item_enchantment[2],
                row.spell_item_enchantment[3],
                row.spell_item_enchantment[4],
                &row.suffix.en_gb,
                &row.suffix.ko_kr,
                &row.suffix.fr_fr,
                &row.suffix.de_de,
                &row.suffix.en_cn,
                &row.suffix.en_tw,
                &row.suffix.es_es,
                &row.suffix.es_mx,
                &row.suffix.flags,
                ])?;
            }
        }
        "ItemSet.dbc" => {
            let data = item_set::ItemSet::read(file_contents)?;
            let (table, insert) = ItemSet();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.items[0],
                row.items[1],
                row.items[2],
                row.items[3],
                row.items[4],
                row.items[5],
                row.items[6],
                row.items[7],
                row.items[8],
                row.items[9],
                row.bank_item[0],
                row.bank_item[1],
                row.bank_item[2],
                row.bank_item[3],
                row.bank_item[4],
                row.bank_item[5],
                row.bank_item[6],
                row.set_spell[0],
                row.set_spell[1],
                row.set_spell[2],
                row.set_spell[3],
                row.set_spell[4],
                row.set_spell[5],
                row.set_spell[6],
                row.set_spell[7],
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
            let (table, insert) = ItemSubClass();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.item_class.id,
                row.subclass,
                row.prerequisite_proficiency,
                row.postrequisite_proficiency,
                row.flags,
                row.display_flags,
                row.weapon_parry_sequence,
                row.weapon_ready_sequence,
                row.weapon_attack_sequence,
                row.weapon_swing_size,
                &row.display_name.en_gb,
                &row.display_name.ko_kr,
                &row.display_name.fr_fr,
                &row.display_name.de_de,
                &row.display_name.en_cn,
                &row.display_name.en_tw,
                &row.display_name.es_es,
                &row.display_name.es_mx,
                &row.display_name.flags,
                &row.verbose_name.en_gb,
                &row.verbose_name.ko_kr,
                &row.verbose_name.fr_fr,
                &row.verbose_name.de_de,
                &row.verbose_name.en_cn,
                &row.verbose_name.en_tw,
                &row.verbose_name.es_es,
                &row.verbose_name.es_mx,
                &row.verbose_name.flags,
                ])?;
            }
        }
        "ItemSubClassMask.dbc" => {
            let data = item_sub_class_mask::ItemSubClassMask::read(file_contents)?;
            let (table, insert) = ItemSubClassMask();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.subclass,
                row.mask,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "ItemVisualEffects.dbc" => {
            let data = item_visual_effects::ItemVisualEffects::read(file_contents)?;
            let (table, insert) = ItemVisualEffects();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.model_path,
                ])?;
            }
        }
        "ItemVisuals.dbc" => {
            let data = item_visuals::ItemVisuals::read(file_contents)?;
            let (table, insert) = ItemVisuals();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_visual_effects[0],
                row.item_visual_effects[1],
                row.item_visual_effects[2],
                row.item_visual_effects[3],
                row.item_visual_effects[4],
                ])?;
            }
        }
        "LFGDungeons.dbc" => {
            let data = lfg_dungeons::LFGDungeons::read(file_contents)?;
            let (table, insert) = LFGDungeons();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.min_allowed_level,
                row.max_allowed_level,
                row.instance_type.as_int(),
                row.faction.as_int(),
                ])?;
            }
        }
        "LanguageWords.dbc" => {
            let data = language_words::LanguageWords::read(file_contents)?;
            let (table, insert) = LanguageWords();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.language.id,
                &row.word,
                ])?;
            }
        }
        "Languages.dbc" => {
            let data = languages::Languages::read(file_contents)?;
            let (table, insert) = Languages();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "Light.dbc" => {
            let data = light::Light::read(file_contents)?;
            let (table, insert) = Light();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.location_x,
                row.location_y,
                row.location_z,
                row.falloff_start,
                row.falloff_end,
                row.light_params[0],
                row.light_params[1],
                row.light_params[2],
                row.light_params[3],
                row.light_params[4],
                ])?;
            }
        }
        "LightFloatBand.dbc" => {
            let data = light_float_band::LightFloatBand::read(file_contents)?;
            let (table, insert) = LightFloatBand();
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
            let (table, insert) = LightIntBand();
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
            let (table, insert) = LightParams();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.highlight_sky,
                row.light_skybox.id,
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
            let (table, insert) = LightSkybox();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.skybox_model_path,
                ])?;
            }
        }
        "LiquidType.dbc" => {
            let data = liquid_type::LiquidType::read(file_contents)?;
            let (table, insert) = LiquidType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.ty.as_int(),
                row.spell.id,
                ])?;
            }
        }
        "LoadingScreenTaxiSplines.dbc" => {
            let data = loading_screen_taxi_splines::LoadingScreenTaxiSplines::read(file_contents)?;
            let (table, insert) = LoadingScreenTaxiSplines();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.taxi_path.id,
                row.location_x[0],
                row.location_x[1],
                row.location_x[2],
                row.location_x[3],
                row.location_x[4],
                row.location_x[5],
                row.location_x[6],
                row.location_x[7],
                row.location_y[0],
                row.location_y[1],
                row.location_y[2],
                row.location_y[3],
                row.location_y[4],
                row.location_y[5],
                row.location_y[6],
                row.location_y[7],
                row.leg_index,
                ])?;
            }
        }
        "LoadingScreens.dbc" => {
            let data = loading_screens::LoadingScreens::read(file_contents)?;
            let (table, insert) = LoadingScreens();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.file_path,
                ])?;
            }
        }
        "Lock.dbc" => {
            let data = lock::Lock::read(file_contents)?;
            let (table, insert) = Lock();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.ty[0].as_int(),
                row.ty[1].as_int(),
                row.ty[2].as_int(),
                row.ty[3].as_int(),
                row.ty[4].as_int(),
                row.ty[5].as_int(),
                row.ty[6].as_int(),
                row.ty[7].as_int(),
                row.property[0],
                row.property[1],
                row.property[2],
                row.property[3],
                row.property[4],
                row.property[5],
                row.property[6],
                row.property[7],
                row.required_skill[0],
                row.required_skill[1],
                row.required_skill[2],
                row.required_skill[3],
                row.required_skill[4],
                row.required_skill[5],
                row.required_skill[6],
                row.required_skill[7],
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
            let (table, insert) = LockType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.resource_name.en_gb,
                &row.resource_name.ko_kr,
                &row.resource_name.fr_fr,
                &row.resource_name.de_de,
                &row.resource_name.en_cn,
                &row.resource_name.en_tw,
                &row.resource_name.es_es,
                &row.resource_name.es_mx,
                &row.resource_name.flags,
                &row.verb.en_gb,
                &row.verb.ko_kr,
                &row.verb.fr_fr,
                &row.verb.de_de,
                &row.verb.en_cn,
                &row.verb.en_tw,
                &row.verb.es_es,
                &row.verb.es_mx,
                &row.verb.flags,
                &row.cursor_name,
                ])?;
            }
        }
        "MailTemplate.dbc" => {
            let data = mail_template::MailTemplate::read(file_contents)?;
            let (table, insert) = MailTemplate();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.body.en_gb,
                &row.body.ko_kr,
                &row.body.fr_fr,
                &row.body.de_de,
                &row.body.en_cn,
                &row.body.en_tw,
                &row.body.es_es,
                &row.body.es_mx,
                &row.body.flags,
                ])?;
            }
        }
        "Map.dbc" => {
            let data = map::Map::read(file_contents)?;
            let (table, insert) = Map();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.internal_name,
                row.instance_type.as_int(),
                row.battleground,
                &row.map_name.en_gb,
                &row.map_name.ko_kr,
                &row.map_name.fr_fr,
                &row.map_name.de_de,
                &row.map_name.en_cn,
                &row.map_name.en_tw,
                &row.map_name.es_es,
                &row.map_name.es_mx,
                &row.map_name.flags,
                row.min_level,
                row.max_level,
                row.max_players,
                row.unknown[0],
                row.unknown[1],
                row.unknown[2],
                row.area_table.id,
                &row.map_description_horde.en_gb,
                &row.map_description_horde.ko_kr,
                &row.map_description_horde.fr_fr,
                &row.map_description_horde.de_de,
                &row.map_description_horde.en_cn,
                &row.map_description_horde.en_tw,
                &row.map_description_horde.es_es,
                &row.map_description_horde.es_mx,
                &row.map_description_horde.flags,
                &row.map_description_alliance.en_gb,
                &row.map_description_alliance.ko_kr,
                &row.map_description_alliance.fr_fr,
                &row.map_description_alliance.de_de,
                &row.map_description_alliance.en_cn,
                &row.map_description_alliance.en_tw,
                &row.map_description_alliance.es_es,
                &row.map_description_alliance.es_mx,
                &row.map_description_alliance.flags,
                row.loading_screen.id,
                row.raid_offset,
                row.unknown_2[0],
                row.unknown_2[1],
                ])?;
            }
        }
        "Material.dbc" => {
            let data = material::Material::read(file_contents)?;
            let (table, insert) = Material();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.flags,
                row.foley_sound.id,
                ])?;
            }
        }
        "NPCSounds.dbc" => {
            let data = npc_sounds::NPCSounds::read(file_contents)?;
            let (table, insert) = NPCSounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_entries[0],
                row.sound_entries[1],
                row.sound_entries[2],
                row.sound_entries[3],
                ])?;
            }
        }
        "NameGen.dbc" => {
            let data = name_gen::NameGen::read(file_contents)?;
            let (table, insert) = NameGen();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.race.id,
                row.gender.as_int(),
                ])?;
            }
        }
        "NamesProfanity.dbc" => {
            let data = names_profanity::NamesProfanity::read(file_contents)?;
            let (table, insert) = NamesProfanity();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
        }
        "NamesReserved.dbc" => {
            let data = names_reserved::NamesReserved::read(file_contents)?;
            let (table, insert) = NamesReserved();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                ])?;
            }
        }
        "Package.dbc" => {
            let data = package::Package::read(file_contents)?;
            let (table, insert) = Package();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.icon,
                row.cost,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "PageTextMaterial.dbc" => {
            let data = page_text_material::PageTextMaterial::read(file_contents)?;
            let (table, insert) = PageTextMaterial();
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
            let (table, insert) = PaperDollItemFrame();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                &row.item_button_name,
                &row.slot_icon,
                row.slot_number,
                ])?;
            }
        }
        "PetLoyalty.dbc" => {
            let data = pet_loyalty::PetLoyalty::read(file_contents)?;
            let (table, insert) = PetLoyalty();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "PetPersonality.dbc" => {
            let data = pet_personality::PetPersonality::read(file_contents)?;
            let (table, insert) = PetPersonality();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.threshold_unhappy,
                row.threshold_content,
                row.threshold_happy,
                row.damage_unhappy,
                row.damage_content,
                row.damage_happy,
                row.modifier_unhappy,
                row.modifier_content,
                row.modifier_happy,
                ])?;
            }
        }
        "QuestInfo.dbc" => {
            let data = quest_info::QuestInfo::read(file_contents)?;
            let (table, insert) = QuestInfo();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "QuestSort.dbc" => {
            let data = quest_sort::QuestSort::read(file_contents)?;
            let (table, insert) = QuestSort();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "Resistances.dbc" => {
            let data = resistances::Resistances::read(file_contents)?;
            let (table, insert) = Resistances();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.physical_damage,
                row.fizzle_sound_entry.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "ServerMessages.dbc" => {
            let data = server_messages::ServerMessages::read(file_contents)?;
            let (table, insert) = ServerMessages();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.text.en_gb,
                &row.text.ko_kr,
                &row.text.fr_fr,
                &row.text.de_de,
                &row.text.en_cn,
                &row.text.en_tw,
                &row.text.es_es,
                &row.text.es_mx,
                &row.text.flags,
                ])?;
            }
        }
        "SheatheSoundLookups.dbc" => {
            let data = sheathe_sound_lookups::SheatheSoundLookups::read(file_contents)?;
            let (table, insert) = SheatheSoundLookups();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item_class.id,
                row.item_subclass,
                row.item_env_types.as_int(),
                row.not_shield,
                row.sheathe_sound.id,
                row.draw_sound.id,
                ])?;
            }
        }
        "SkillCostsData.dbc" => {
            let data = skill_costs_data::SkillCostsData::read(file_contents)?;
            let (table, insert) = SkillCostsData();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.skill_costs,
                row.cost[0],
                row.cost[1],
                row.cost[2],
                ])?;
            }
        }
        "SkillLine.dbc" => {
            let data = skill_line::SkillLine::read(file_contents)?;
            let (table, insert) = SkillLine();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.category.id,
                row.skill_costs.id,
                &row.display_name.en_gb,
                &row.display_name.ko_kr,
                &row.display_name.fr_fr,
                &row.display_name.de_de,
                &row.display_name.en_cn,
                &row.display_name.en_tw,
                &row.display_name.es_es,
                &row.display_name.es_mx,
                &row.display_name.flags,
                &row.description.en_gb,
                &row.description.ko_kr,
                &row.description.fr_fr,
                &row.description.de_de,
                &row.description.en_cn,
                &row.description.en_tw,
                &row.description.es_es,
                &row.description.es_mx,
                &row.description.flags,
                row.spell_icon.id,
                ])?;
            }
        }
        "SkillLineAbility.dbc" => {
            let data = skill_line_ability::SkillLineAbility::read(file_contents)?;
            let (table, insert) = SkillLineAbility();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.skill_line.id,
                row.spell.id,
                row.race_mask.id,
                row.class_mask.id,
                row.exclude_race.id,
                row.exclude_class.id,
                row.superseded_by.id,
                row.acquire_method,
                row.trivial_skill_line_rank_high,
                row.trivial_skill_line_rank_low,
                row.character_points[0],
                row.character_points[1],
                row.num_skills_up,
                row.unknown_padding,
                ])?;
            }
        }
        "SkillLineCategory.dbc" => {
            let data = skill_line_category::SkillLineCategory::read(file_contents)?;
            let (table, insert) = SkillLineCategory();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.sort_index,
                ])?;
            }
        }
        "SkillRaceClassInfo.dbc" => {
            let data = skill_race_class_info::SkillRaceClassInfo::read(file_contents)?;
            let (table, insert) = SkillRaceClassInfo();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.skill_line.id,
                row.race_mask.id,
                row.class_mask.id,
                row.flags,
                row.min_level,
                row.skill_tier.id,
                row.skill_cost.id,
                ])?;
            }
        }
        "SkillTiers.dbc" => {
            let data = skill_tiers::SkillTiers::read(file_contents)?;
            let (table, insert) = SkillTiers();
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
            let (table, insert) = SoundAmbience();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.day_sound.id,
                row.night_sound.id,
                ])?;
            }
        }
        "SoundCharacterMacroLines.dbc" => {
            let data = sound_character_macro_lines::SoundCharacterMacroLines::read(file_contents)?;
            let (table, insert) = SoundCharacterMacroLines();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.unknown,
                row.gender.as_int(),
                row.race.id,
                row.sound.id,
                ])?;
            }
        }
        "SoundEntries.dbc" => {
            let data = sound_entries::SoundEntries::read(file_contents)?;
            let (table, insert) = SoundEntries();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_type.as_int(),
                &row.name,
                row.files[0],
                row.files[1],
                row.files[2],
                row.files[3],
                row.files[4],
                row.files[5],
                row.files[6],
                row.files[7],
                row.files[8],
                row.files[9],
                row.frequency[0],
                row.frequency[1],
                row.frequency[2],
                row.frequency[3],
                row.frequency[4],
                row.frequency[5],
                row.frequency[6],
                row.frequency[7],
                row.frequency[8],
                row.frequency[9],
                &row.directory_base,
                row.volume,
                row.flags,
                row.min_distance,
                row.distance_cutoff,
                row.sound_entries_advanced,
                ])?;
            }
        }
        "SoundProviderPreferences.dbc" => {
            let data = sound_provider_preferences::SoundProviderPreferences::read(file_contents)?;
            let (table, insert) = SoundProviderPreferences();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.description,
                row.flags,
                row.eax_environment_selection,
                row.eax_decay_time,
                row.eax2_environment_size,
                row.eax_environment_diffusion,
                row.eax2_room,
                row.eax2_room_hf,
                row.eax2_decay_hf_ratio,
                row.eax2_reflections,
                row.eax2_reflections_delay,
                row.eax2_reverb,
                row.eax2_reverb_delay,
                row.eax2_room_rolloff,
                row.eax2_air_absorption,
                row.eax3_room_lf,
                row.eax3_delay_lf_ratio,
                row.eax3_echo_time,
                row.eax3_echo_depth,
                row.eax3_modulation_time,
                row.eax3_modulation_depth,
                row.eax3_hf_reference,
                row.eax3_lf_reference,
                ])?;
            }
        }
        "SoundSamplePreferences.dbc" => {
            let data = sound_sample_preferences::SoundSamplePreferences::read(file_contents)?;
            let (table, insert) = SoundSamplePreferences();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.unknown[0],
                row.unknown[1],
                row.unknown[2],
                row.unknown[3],
                row.unknown[4],
                row.unknown[5],
                row.unknown[6],
                row.unknown[7],
                row.unknown[8],
                row.unknown[9],
                row.unknown[10],
                row.unknown[11],
                row.unknown[12],
                row.unknown[13],
                row.unknown[14],
                row.unknown[15],
                ])?;
            }
        }
        "SoundWaterType.dbc" => {
            let data = sound_water_type::SoundWaterType::read(file_contents)?;
            let (table, insert) = SoundWaterType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.liquid_type.id,
                row.fluid_speed.as_int(),
                row.sound.id,
                ])?;
            }
        }
        "SpamMessages.dbc" => {
            let data = spam_messages::SpamMessages::read(file_contents)?;
            let (table, insert) = SpamMessages();
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
            let (table, insert) = Spell();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.school.id,
                row.category.id,
                row.cast_ui,
                row.dispel_type.id,
                row.mechanic.id,
                row.attributes.as_int(),
                row.attributes_ex1.as_int(),
                row.attributes_ex2.as_int(),
                row.attributes_ex3.as_int(),
                row.attributes_ex4.as_int(),
                row.shapeshift_mask.id,
                row.shapeshift_exclude.id,
                row.targets,
                row.target_creature_type.id,
                row.requires_spell_focus.id,
                row.caster_aura_state,
                row.target_aura_state,
                row.casting_time_index.id,
                row.recovery_time,
                row.category_recovery_time,
                row.aura_interrupt_flags,
                row.channel_interrupt_flags,
                row.proc_type_mask,
                row.proc_chance,
                row.proc_charges,
                row.max_level,
                row.base_level,
                row.spell_level,
                row.duration.id,
                row.power_type,
                row.mana_cost,
                row.mana_cost_per_level,
                row.mana_cost_per_second,
                row.mana_cost_per_second_per_level,
                row.range.id,
                row.speed,
                row.modal_next_spell.id,
                row.stack_amount,
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
                row.equipped_item_class.id,
                row.equipped_item_subclass,
                row.equipped_item_inventory_type,
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
                row.effect_radius[0],
                row.effect_radius[1],
                row.effect_radius[2],
                row.effect_aura[0].as_int(),
                row.effect_aura[1].as_int(),
                row.effect_aura[2].as_int(),
                row.effect_amplitude[0],
                row.effect_amplitude[1],
                row.effect_amplitude[2],
                row.effect_multiple_values[0],
                row.effect_multiple_values[1],
                row.effect_multiple_values[2],
                row.effect_chain_target[0],
                row.effect_chain_target[1],
                row.effect_chain_target[2],
                row.effect_item_type[0],
                row.effect_item_type[1],
                row.effect_item_type[2],
                row.effect_misc_value[0],
                row.effect_misc_value[1],
                row.effect_misc_value[2],
                row.effect_trigger_spell[0],
                row.effect_trigger_spell[1],
                row.effect_trigger_spell[2],
                row.effect_points_per_combo[0],
                row.effect_points_per_combo[1],
                row.effect_points_per_combo[2],
                row.spell_visual[0],
                row.spell_visual[1],
                row.spell_icon.id,
                row.active_icon,
                row.spell_priority,
                row.unknown_flag,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                &row.name_subtext.en_gb,
                &row.name_subtext.ko_kr,
                &row.name_subtext.fr_fr,
                &row.name_subtext.de_de,
                &row.name_subtext.en_cn,
                &row.name_subtext.en_tw,
                &row.name_subtext.es_es,
                &row.name_subtext.es_mx,
                &row.name_subtext.flags,
                &row.description.en_gb,
                &row.description.ko_kr,
                &row.description.fr_fr,
                &row.description.de_de,
                &row.description.en_cn,
                &row.description.en_tw,
                &row.description.es_es,
                &row.description.es_mx,
                &row.description.flags,
                &row.aura_description.en_gb,
                &row.aura_description.ko_kr,
                &row.aura_description.fr_fr,
                &row.aura_description.de_de,
                &row.aura_description.en_cn,
                &row.aura_description.en_tw,
                &row.aura_description.es_es,
                &row.aura_description.es_mx,
                &row.aura_description.flags,
                row.mana_cost_percent,
                row.start_recovery_category,
                row.start_recovery_time,
                row.max_target_level,
                row.spell_class_set.id,
                row.spell_class_mask[0],
                row.spell_class_mask[1],
                row.max_targets,
                row.defence_type,
                row.prevention_type,
                row.stance_bar_order,
                row.damage_multiplier[0],
                row.damage_multiplier[1],
                row.damage_multiplier[2],
                row.min_faction,
                row.min_reputation,
                row.required_aura_vision,
                ])?;
            }
        }
        "SpellAuraNames.dbc" => {
            let data = spell_aura_names::SpellAuraNames::read(file_contents)?;
            let (table, insert) = SpellAuraNames();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.unknown,
                &row.internal_name,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "SpellCastTimes.dbc" => {
            let data = spell_cast_times::SpellCastTimes::read(file_contents)?;
            let (table, insert) = SpellCastTimes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.base,
                row.per_level_increase,
                row.minimum,
                ])?;
            }
        }
        "SpellCategory.dbc" => {
            let data = spell_category::SpellCategory::read(file_contents)?;
            let (table, insert) = SpellCategory();
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
            let (table, insert) = SpellChainEffects();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.average_seg_len,
                row.width,
                row.noise_scale,
                row.tex_coord_scale,
                row.seg_duration,
                row.seg_delay,
                &row.texture,
                ])?;
            }
        }
        "SpellDispelType.dbc" => {
            let data = spell_dispel_type::SpellDispelType::read(file_contents)?;
            let (table, insert) = SpellDispelType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.mask,
                row.immunity_possible,
                ])?;
            }
        }
        "SpellDuration.dbc" => {
            let data = spell_duration::SpellDuration::read(file_contents)?;
            let (table, insert) = SpellDuration();
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
            let (table, insert) = SpellEffectCameraShakes();
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
        "SpellEffectNames.dbc" => {
            let data = spell_effect_names::SpellEffectNames::read(file_contents)?;
            let (table, insert) = SpellEffectNames();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "SpellFocusObject.dbc" => {
            let data = spell_focus_object::SpellFocusObject::read(file_contents)?;
            let (table, insert) = SpellFocusObject();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "SpellIcon.dbc" => {
            let data = spell_icon::SpellIcon::read(file_contents)?;
            let (table, insert) = SpellIcon();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.texture_file,
                ])?;
            }
        }
        "SpellItemEnchantment.dbc" => {
            let data = spell_item_enchantment::SpellItemEnchantment::read(file_contents)?;
            let (table, insert) = SpellItemEnchantment();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.enchantment_type[0],
                row.enchantment_type[1],
                row.enchantment_type[2],
                row.effect_points_min[0],
                row.effect_points_min[1],
                row.effect_points_min[2],
                row.effect_points_max[0],
                row.effect_points_max[1],
                row.effect_points_max[2],
                row.effect_arg[0],
                row.effect_arg[1],
                row.effect_arg[2],
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.item_visual.id,
                row.flags,
                ])?;
            }
        }
        "SpellMechanic.dbc" => {
            let data = spell_mechanic::SpellMechanic::read(file_contents)?;
            let (table, insert) = SpellMechanic();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.state_name.en_gb,
                &row.state_name.ko_kr,
                &row.state_name.fr_fr,
                &row.state_name.de_de,
                &row.state_name.en_cn,
                &row.state_name.en_tw,
                &row.state_name.es_es,
                &row.state_name.es_mx,
                &row.state_name.flags,
                ])?;
            }
        }
        "SpellRadius.dbc" => {
            let data = spell_radius::SpellRadius::read(file_contents)?;
            let (table, insert) = SpellRadius();
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
            let (table, insert) = SpellRange();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.range_min,
                row.range_max,
                row.flags,
                &row.display_name.en_gb,
                &row.display_name.ko_kr,
                &row.display_name.fr_fr,
                &row.display_name.de_de,
                &row.display_name.en_cn,
                &row.display_name.en_tw,
                &row.display_name.es_es,
                &row.display_name.es_mx,
                &row.display_name.flags,
                &row.display_name_short.en_gb,
                &row.display_name_short.ko_kr,
                &row.display_name_short.fr_fr,
                &row.display_name_short.de_de,
                &row.display_name_short.en_cn,
                &row.display_name_short.en_tw,
                &row.display_name_short.es_es,
                &row.display_name_short.es_mx,
                &row.display_name_short.flags,
                ])?;
            }
        }
        "SpellShapeshiftForm.dbc" => {
            let data = spell_shapeshift_form::SpellShapeshiftForm::read(file_contents)?;
            let (table, insert) = SpellShapeshiftForm();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.bonus_action_bar,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.flags,
                row.creature_type,
                row.spell_icon.id,
                ])?;
            }
        }
        "SpellVisual.dbc" => {
            let data = spell_visual::SpellVisual::read(file_contents)?;
            let (table, insert) = SpellVisual();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.precast_kit.id,
                row.cast_kit.id,
                row.impact_kit.id,
                row.state_kit.id,
                row.state_done_kit.id,
                row.channel_kit.id,
                row.has_missile,
                row.missile_model,
                row.missile_path_type,
                row.missile_destination_attachment,
                row.missile_sound.id,
                row.anim_event_sound.id,
                row.flags,
                row.caster_impact_kit.id,
                row.target_impact_kit.id,
                ])?;
            }
        }
        "SpellVisualEffectName.dbc" => {
            let data = spell_visual_effect_name::SpellVisualEffectName::read(file_contents)?;
            let (table, insert) = SpellVisualEffectName();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.filename,
                row.area_effect_size,
                row.scale,
                ])?;
            }
        }
        "SpellVisualKit.dbc" => {
            let data = spell_visual_kit::SpellVisualKit::read(file_contents)?;
            let (table, insert) = SpellVisualKit();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.start_anim.id,
                row.anim.id,
                row.head_effect.id,
                row.chest_effect.id,
                row.base_effect.id,
                row.left_hand_effect.id,
                row.right_hand_effect.id,
                row.breath_effect.id,
                row.left_weapon_effect.id,
                row.right_weapon_effect.id,
                row.special_effects[0],
                row.special_effects[1],
                row.special_effects[2],
                row.world_effect.id,
                row.sound.id,
                row.shake.id,
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
                row.unknown1_pad,
                row.unknown2_pad,
                ])?;
            }
        }
        "SpellVisualPrecastTransitions.dbc" => {
            let data = spell_visual_precast_transitions::SpellVisualPrecastTransitions::read(file_contents)?;
            let (table, insert) = SpellVisualPrecastTransitions();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.load_animation,
                &row.hold_animation,
                ])?;
            }
        }
        "StableSlotPrices.dbc" => {
            let data = stable_slot_prices::StableSlotPrices::read(file_contents)?;
            let (table, insert) = StableSlotPrices();
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
            let (table, insert) = Startup_Strings();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.startup_string.en_gb,
                &row.startup_string.ko_kr,
                &row.startup_string.fr_fr,
                &row.startup_string.de_de,
                &row.startup_string.en_cn,
                &row.startup_string.en_tw,
                &row.startup_string.es_es,
                &row.startup_string.es_mx,
                &row.startup_string.flags,
                ])?;
            }
        }
        "Stationery.dbc" => {
            let data = stationery::Stationery::read(file_contents)?;
            let (table, insert) = Stationery();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.item,
                &row.texture,
                row.flags,
                ])?;
            }
        }
        "StringLookups.dbc" => {
            let data = string_lookups::StringLookups::read(file_contents)?;
            let (table, insert) = StringLookups();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.path,
                ])?;
            }
        }
        "Talent.dbc" => {
            let data = talent::Talent::read(file_contents)?;
            let (table, insert) = Talent();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.tab.id,
                row.tier,
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
                row.prereq_talents[0],
                row.prereq_talents[1],
                row.prereq_talents[2],
                row.prereq_ranks[0],
                row.prereq_ranks[1],
                row.prereq_ranks[2],
                row.flags,
                row.required_spell.id,
                ])?;
            }
        }
        "TalentTab.dbc" => {
            let data = talent_tab::TalentTab::read(file_contents)?;
            let (table, insert) = TalentTab();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.spell_icon.id,
                row.race_mask.id,
                row.class_mask.id,
                row.order_index,
                &row.background_file,
                ])?;
            }
        }
        "TaxiNodes.dbc" => {
            let data = taxi_nodes::TaxiNodes::read(file_contents)?;
            let (table, insert) = TaxiNodes();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.location_x,
                row.location_y,
                row.location_z,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                row.mount_creature_display_info[0],
                row.mount_creature_display_info[1],
                ])?;
            }
        }
        "TaxiPath.dbc" => {
            let data = taxi_path::TaxiPath::read(file_contents)?;
            let (table, insert) = TaxiPath();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.source_taxi_node.id,
                row.destination_taxi_node.id,
                row.cost,
                ])?;
            }
        }
        "TaxiPathNode.dbc" => {
            let data = taxi_path_node::TaxiPathNode::read(file_contents)?;
            let (table, insert) = TaxiPathNode();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.taxi_path.id,
                row.node_index,
                row.map.id,
                row.location_x,
                row.location_y,
                row.location_z,
                row.flags,
                row.delay,
                ])?;
            }
        }
        "TerrainType.dbc" => {
            let data = terrain_type::TerrainType::read(file_contents)?;
            let (table, insert) = TerrainType();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.description,
                row.footstep_spray_run.id,
                row.footstep_spray_walk.id,
                row.sound.id,
                row.display_footsteps,
                ])?;
            }
        }
        "TerrainTypeSounds.dbc" => {
            let data = terrain_type_sounds::TerrainTypeSounds::read(file_contents)?;
            let (table, insert) = TerrainTypeSounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                ])?;
            }
        }
        "TransportAnimation.dbc" => {
            let data = transport_animation::TransportAnimation::read(file_contents)?;
            let (table, insert) = TransportAnimation();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.transport,
                row.time_index,
                row.location_x,
                row.location_y,
                row.location_z,
                row.sequence.id,
                ])?;
            }
        }
        "UISoundLookups.dbc" => {
            let data = ui_sound_lookups::UISoundLookups::read(file_contents)?;
            let (table, insert) = UISoundLookups();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.sound_entry.id,
                &row.internal_name,
                ])?;
            }
        }
        "UnitBlood.dbc" => {
            let data = unit_blood::UnitBlood::read(file_contents)?;
            let (table, insert) = UnitBlood();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.combat_blood_spurt_front_small,
                row.combat_blood_spurt_front_large,
                row.combat_blood_spurt_back_small,
                row.combat_blood_spurt_back_large,
                row.texture[0],
                row.texture[1],
                row.texture[2],
                row.texture[3],
                row.texture[4],
                ])?;
            }
        }
        "UnitBloodLevels.dbc" => {
            let data = unit_blood_levels::UnitBloodLevels::read(file_contents)?;
            let (table, insert) = UnitBloodLevels();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.violence_level[0],
                row.violence_level[1],
                row.violence_level[2],
                ])?;
            }
        }
        "VideoHardware.dbc" => {
            let data = video_hardware::VideoHardware::read(file_contents)?;
            let (table, insert) = VideoHardware();
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
                ])?;
            }
        }
        "VocalUISounds.dbc" => {
            let data = vocal_ui_sounds::VocalUISounds::read(file_contents)?;
            let (table, insert) = VocalUISounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.vocal_ui_enum,
                row.race.id,
                row.normal_male_sound.id,
                row.normal_female_sound.id,
                row.pissed_male_sound.id,
                row.pissed_female_sound.id,
                ])?;
            }
        }
        "WMOAreaTable.dbc" => {
            let data = wmo_area_table::WMOAreaTable::read(file_contents)?;
            let (table, insert) = WMOAreaTable();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.wmo_id,
                row.name_set_id,
                row.wmo_group_id,
                row.sound_provider_preferences.id,
                row.sound_provider_preferences_underwater.id,
                row.sound_ambience.id,
                row.zone_music.id,
                row.zone_intro_music.id,
                row.flags,
                row.area_table.id,
                &row.name.en_gb,
                &row.name.ko_kr,
                &row.name.fr_fr,
                &row.name.de_de,
                &row.name.en_cn,
                &row.name.en_tw,
                &row.name.es_es,
                &row.name.es_mx,
                &row.name.flags,
                ])?;
            }
        }
        "WeaponImpactSounds.dbc" => {
            let data = weapon_impact_sounds::WeaponImpactSounds::read(file_contents)?;
            let (table, insert) = WeaponImpactSounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.weapon_subclass,
                row.parry_sound_type,
                row.impact_sound[0],
                row.impact_sound[1],
                row.impact_sound[2],
                row.impact_sound[3],
                row.impact_sound[4],
                row.impact_sound[5],
                row.impact_sound[6],
                row.impact_sound[7],
                row.impact_sound[8],
                row.impact_sound[9],
                row.crit_impact_sound[0],
                row.crit_impact_sound[1],
                row.crit_impact_sound[2],
                row.crit_impact_sound[3],
                row.crit_impact_sound[4],
                row.crit_impact_sound[5],
                row.crit_impact_sound[6],
                row.crit_impact_sound[7],
                row.crit_impact_sound[8],
                row.crit_impact_sound[9],
                ])?;
            }
        }
        "WeaponSwingSounds2.dbc" => {
            let data = weapon_swing_sounds2::WeaponSwingSounds2::read(file_contents)?;
            let (table, insert) = WeaponSwingSounds2();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.swing_type.as_int(),
                row.critical,
                row.sound.id,
                ])?;
            }
        }
        "WorldMapArea.dbc" => {
            let data = world_map_area::WorldMapArea::read(file_contents)?;
            let (table, insert) = WorldMapArea();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.world_map_continent.id,
                row.area_table.id,
                &row.area_name,
                row.location_left,
                row.location_right,
                row.location_top,
                row.location_bottom,
                ])?;
            }
        }
        "WorldMapContinent.dbc" => {
            let data = world_map_continent::WorldMapContinent::read(file_contents)?;
            let (table, insert) = WorldMapContinent();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.left_boundary,
                row.right_boundary,
                row.top_boundary,
                row.bottom_boundary,
                row.continent_offset_x,
                row.continent_offset_y,
                row.scale,
                row.taxi_min_x,
                row.taxi_min_y,
                row.taxi_max_x,
                row.taxi_max_y,
                ])?;
            }
        }
        "WorldMapOverlay.dbc" => {
            let data = world_map_overlay::WorldMapOverlay::read(file_contents)?;
            let (table, insert) = WorldMapOverlay();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.world_map_area.id,
                row.area_table[0],
                row.area_table[1],
                row.area_table[2],
                row.area_table[3],
                row.location_x,
                row.location_y,
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
        "WorldSafeLocs.dbc" => {
            let data = world_safe_locs::WorldSafeLocs::read(file_contents)?;
            let (table, insert) = WorldSafeLocs();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.location_x,
                row.location_y,
                row.location_z,
                &row.area_name.en_gb,
                &row.area_name.ko_kr,
                &row.area_name.fr_fr,
                &row.area_name.de_de,
                &row.area_name.en_cn,
                &row.area_name.en_tw,
                &row.area_name.es_es,
                &row.area_name.es_mx,
                &row.area_name.flags,
                ])?;
            }
        }
        "WorldStateUI.dbc" => {
            let data = world_state_ui::WorldStateUI::read(file_contents)?;
            let (table, insert) = WorldStateUI();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                row.map.id,
                row.area_table.id,
                &row.icon,
                &row.state_variable.en_gb,
                &row.state_variable.ko_kr,
                &row.state_variable.fr_fr,
                &row.state_variable.de_de,
                &row.state_variable.en_cn,
                &row.state_variable.en_tw,
                &row.state_variable.es_es,
                &row.state_variable.es_mx,
                &row.state_variable.flags,
                &row.tooltip.en_gb,
                &row.tooltip.ko_kr,
                &row.tooltip.fr_fr,
                &row.tooltip.de_de,
                &row.tooltip.en_cn,
                &row.tooltip.en_tw,
                &row.tooltip.es_es,
                &row.tooltip.es_mx,
                &row.tooltip.flags,
                row.state,
                row.world_state,
                row.ty,
                &row.dynamic_icon,
                &row.dynamic_tooltip.en_gb,
                &row.dynamic_tooltip.ko_kr,
                &row.dynamic_tooltip.fr_fr,
                &row.dynamic_tooltip.de_de,
                &row.dynamic_tooltip.en_cn,
                &row.dynamic_tooltip.en_tw,
                &row.dynamic_tooltip.es_es,
                &row.dynamic_tooltip.es_mx,
                &row.dynamic_tooltip.flags,
                &row.extended_ui,
                row.unknown[0],
                row.unknown[1],
                row.unknown[2],
                ])?;
            }
        }
        "WowError_Strings.dbc" => {
            let data = wow_error_strings::WowError_Strings::read(file_contents)?;
            let (table, insert) = WowError_Strings();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                &row.text.en_gb,
                &row.text.ko_kr,
                &row.text.fr_fr,
                &row.text.de_de,
                &row.text.en_cn,
                &row.text.en_tw,
                &row.text.es_es,
                &row.text.es_mx,
                &row.text.flags,
                ])?;
            }
        }
        "ZoneIntroMusicTable.dbc" => {
            let data = zone_intro_music_table::ZoneIntroMusicTable::read(file_contents)?;
            let (table, insert) = ZoneIntroMusicTable();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.name,
                row.intro_sound.id,
                row.priority_over_ambience,
                row.min_delay,
                ])?;
            }
        }
        "ZoneMusic.dbc" => {
            let data = zone_music::ZoneMusic::read(file_contents)?;
            let (table, insert) = ZoneMusic();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
                &row.set_name,
                row.silence_interval_min_day,
                row.silence_interval_min_night,
                row.silence_interval_max_day,
                row.silence_interval_max_night,
                row.day_sound.id,
                row.night_sound.id,
                ])?;
            }
        }
        v => return Err(SqliteError::FilenameNotFound { name: v.to_string() }),
    }

    tx.commit()?;

    Ok(())
}
#[allow(non_snake_case)]
pub(crate) fn AnimationData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AnimationData (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        weapon_flags INTEGER  NOT NULL,
        body_flags INTEGER  NOT NULL,
        unknown INTEGER  NOT NULL,
        fallback INTEGER  NOT NULL,
        behaviour INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AnimationData (
        id,
        name,
        weapon_flags,
        body_flags,
        unknown,
        fallback,
        behaviour
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
pub(crate) fn AreaPOI() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaPOI (
        id INTEGER PRIMARY KEY NOT NULL,
        importance INTEGER  NOT NULL,
        icon INTEGER  NOT NULL,
        faction INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        map INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        area_table INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags TEXT NOT NULL,
        world_state INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AreaPOI (
        id,
        importance,
        icon,
        faction,
        location_x,
        location_y,
        location_z,
        map,
        flags,
        area_table,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        description_en_gb,
        description_ko_kr,
        description_fr_fr,
        description_de_de,
        description_en_cn,
        description_en_tw,
        description_es_es,
        description_es_mx,
        description_flags,
        world_state
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
pub(crate) fn AreaTable() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaTable (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        parent_area_table INTEGER  NOT NULL,
        area_bit INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        sound_preferences INTEGER  NOT NULL,
        sound_preferences_underwater INTEGER  NOT NULL,
        sound_ambience INTEGER  NOT NULL,
        zone_music INTEGER  NOT NULL,
        zone_music_intro INTEGER  NOT NULL,
        exploration_level INTEGER  NOT NULL,
        area_name_en_gb TEXT NOT NULL,
        area_name_ko_kr TEXT NOT NULL,
        area_name_fr_fr TEXT NOT NULL,
        area_name_de_de TEXT NOT NULL,
        area_name_en_cn TEXT NOT NULL,
        area_name_en_tw TEXT NOT NULL,
        area_name_es_es TEXT NOT NULL,
        area_name_es_mx TEXT NOT NULL,
        area_name_flags TEXT NOT NULL,
        faction_group INTEGER  NOT NULL,
        liquid_type INTEGER  NOT NULL,
        min_elevation INTEGER  NOT NULL,
        ambient_multiplier REAL  NOT NULL,
        light INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AreaTable (
        id,
        map,
        parent_area_table,
        area_bit,
        flags,
        sound_preferences,
        sound_preferences_underwater,
        sound_ambience,
        zone_music,
        zone_music_intro,
        exploration_level,
        area_name_en_gb,
        area_name_ko_kr,
        area_name_fr_fr,
        area_name_de_de,
        area_name_en_cn,
        area_name_en_tw,
        area_name_es_es,
        area_name_es_mx,
        area_name_flags,
        faction_group,
        liquid_type,
        min_elevation,
        ambient_multiplier,
        light
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
pub(crate) fn AreaTrigger() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AreaTrigger (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        radius REAL  NOT NULL,
        box_length REAL  NOT NULL,
        box_width REAL  NOT NULL,
        box_height REAL  NOT NULL,
        box_yaw REAL  NOT NULL
    );"
    ,
    "INSERT INTO AreaTrigger (
        id,
        map,
        location_x,
        location_y,
        location_z,
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
pub(crate) fn AttackAnimKits() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AttackAnimKits (
        id INTEGER PRIMARY KEY NOT NULL,
        animation_data INTEGER  NOT NULL,
        attack_anim_type INTEGER  NOT NULL,
        animation_frequency INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO AttackAnimKits (
        id,
        animation_data,
        attack_anim_type,
        animation_frequency,
        flags
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
pub(crate) fn AttackAnimTypes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AttackAnimTypes (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO AttackAnimTypes (
        id,
        name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn AuctionHouse() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS AuctionHouse (
        id INTEGER PRIMARY KEY NOT NULL,
        faction INTEGER  NOT NULL,
        deposit_rate INTEGER  NOT NULL,
        consignment_rate INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO AuctionHouse (
        id,
        faction,
        deposit_rate,
        consignment_rate,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn Cfg_Categories() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Cfg_Categories (
        category INTEGER  NOT NULL,
        region INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Cfg_Categories (
        category,
        region,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn Cfg_Configs() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Cfg_Configs (
        id INTEGER PRIMARY KEY NOT NULL,
        realm_type INTEGER  NOT NULL,
        pvp_allowed INTEGER  NOT NULL,
        roleplaying INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Cfg_Configs (
        id,
        realm_type,
        pvp_allowed,
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
pub(crate) fn CharBaseInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharBaseInfo (
        race INTEGER  NOT NULL,
        class INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharBaseInfo (
        race,
        class
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharHairGeosets() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharHairGeosets (
        id INTEGER PRIMARY KEY NOT NULL,
        race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        variation INTEGER  NOT NULL,
        geoset INTEGER  NOT NULL,
        show_scalp INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharHairGeosets (
        id,
        race,
        gender,
        variation,
        geoset,
        show_scalp
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
pub(crate) fn CharHairTextures() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharHairTextures (
        id INTEGER PRIMARY KEY NOT NULL,
        race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        variation INTEGER  NOT NULL,
        geoset INTEGER  NOT NULL,
        show_scalp INTEGER  NOT NULL,
        unknown_padding_0 INTEGER NOT NULL,
        unknown_padding_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CharHairTextures (
        id,
        race,
        gender,
        variation,
        geoset,
        show_scalp,
        unknown_padding_0,
        unknown_padding_1
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
pub(crate) fn CharSections() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharSections (
        id INTEGER PRIMARY KEY NOT NULL,
        race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        ty INTEGER  NOT NULL,
        variation_index INTEGER  NOT NULL,
        colour_index INTEGER  NOT NULL,
        texture_name_0 TEXT NOT NULL,
        texture_name_1 TEXT NOT NULL,
        texture_name_2 TEXT NOT NULL,
        npc_only INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharSections (
        id,
        race,
        gender,
        ty,
        variation_index,
        colour_index,
        texture_name_0,
        texture_name_1,
        texture_name_2,
        npc_only
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
pub(crate) fn CharStartOutfit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharStartOutfit (
        id INTEGER PRIMARY KEY NOT NULL,
        race INTEGER  NOT NULL,
        class INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
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
        display_id_0 INTEGER NOT NULL,
        display_id_1 INTEGER NOT NULL,
        display_id_2 INTEGER NOT NULL,
        display_id_3 INTEGER NOT NULL,
        display_id_4 INTEGER NOT NULL,
        display_id_5 INTEGER NOT NULL,
        display_id_6 INTEGER NOT NULL,
        display_id_7 INTEGER NOT NULL,
        display_id_8 INTEGER NOT NULL,
        display_id_9 INTEGER NOT NULL,
        display_id_10 INTEGER NOT NULL,
        display_id_11 INTEGER NOT NULL,
        inv_slot_id_0 INTEGER NOT NULL,
        inv_slot_id_1 INTEGER NOT NULL,
        inv_slot_id_2 INTEGER NOT NULL,
        inv_slot_id_3 INTEGER NOT NULL,
        inv_slot_id_4 INTEGER NOT NULL,
        inv_slot_id_5 INTEGER NOT NULL,
        inv_slot_id_6 INTEGER NOT NULL,
        inv_slot_id_7 INTEGER NOT NULL,
        inv_slot_id_8 INTEGER NOT NULL,
        inv_slot_id_9 INTEGER NOT NULL,
        inv_slot_id_10 INTEGER NOT NULL,
        inv_slot_id_11 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CharStartOutfit (
        id,
        race,
        class,
        gender,
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
        display_id_0,
        display_id_1,
        display_id_2,
        display_id_3,
        display_id_4,
        display_id_5,
        display_id_6,
        display_id_7,
        display_id_8,
        display_id_9,
        display_id_10,
        display_id_11,
        inv_slot_id_0,
        inv_slot_id_1,
        inv_slot_id_2,
        inv_slot_id_3,
        inv_slot_id_4,
        inv_slot_id_5,
        inv_slot_id_6,
        inv_slot_id_7,
        inv_slot_id_8,
        inv_slot_id_9,
        inv_slot_id_10,
        inv_slot_id_11
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
    )
}

#[allow(non_snake_case)]
pub(crate) fn CharVariations() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharVariations (
        id INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        unknown_1 INTEGER  NOT NULL,
        mask_0 INTEGER NOT NULL,
        mask_1 INTEGER NOT NULL,
        unknown_2 INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CharVariations (
        id,
        gender,
        unknown_1,
        mask_0,
        mask_1,
        unknown_2
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
pub(crate) fn CharacterCreateCameras() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharacterCreateCameras (
        id INTEGER PRIMARY KEY NOT NULL,
        unknown_0 INTEGER NOT NULL,
        unknown_1 INTEGER NOT NULL,
        unknown_2_0 REAL NOT NULL,
        unknown_2_1 REAL NOT NULL,
        unknown_2_2 REAL NOT NULL
    );"
    ,
    "INSERT INTO CharacterCreateCameras (
        id,
        unknown_0,
        unknown_1,
        unknown_2_0,
        unknown_2_1,
        unknown_2_2
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
pub(crate) fn CharacterFacialHairStyles() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CharacterFacialHairStyles (
        race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        variation_id INTEGER  NOT NULL,
        geoset_0 INTEGER NOT NULL,
        geoset_1 INTEGER NOT NULL,
        geoset_2 INTEGER NOT NULL,
        geoset_3 INTEGER NOT NULL,
        geoset_4 INTEGER NOT NULL,
        geoset_5 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CharacterFacialHairStyles (
        race,
        gender,
        variation_id,
        geoset_0,
        geoset_1,
        geoset_2,
        geoset_3,
        geoset_4,
        geoset_5
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
pub(crate) fn ChatChannels() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChatChannels (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        faction_group INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        shortcut_en_gb TEXT NOT NULL,
        shortcut_ko_kr TEXT NOT NULL,
        shortcut_fr_fr TEXT NOT NULL,
        shortcut_de_de TEXT NOT NULL,
        shortcut_en_cn TEXT NOT NULL,
        shortcut_en_tw TEXT NOT NULL,
        shortcut_es_es TEXT NOT NULL,
        shortcut_es_mx TEXT NOT NULL,
        shortcut_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ChatChannels (
        id,
        flags,
        faction_group,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        shortcut_en_gb,
        shortcut_ko_kr,
        shortcut_fr_fr,
        shortcut_de_de,
        shortcut_en_cn,
        shortcut_en_tw,
        shortcut_es_es,
        shortcut_es_mx,
        shortcut_flags
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
pub(crate) fn ChatProfanity() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChatProfanity (
        id INTEGER PRIMARY KEY NOT NULL,
        text TEXT  NOT NULL
    );"
    ,
    "INSERT INTO ChatProfanity (
        id,
        text
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ChrClasses() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChrClasses (
        id INTEGER PRIMARY KEY NOT NULL,
        player_class INTEGER  NOT NULL,
        damage_bonus_stat INTEGER  NOT NULL,
        power_type INTEGER  NOT NULL,
        pet_name_token TEXT  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        filename TEXT  NOT NULL,
        class_mask INTEGER  NOT NULL,
        hybrid_class INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ChrClasses (
        id,
        player_class,
        damage_bonus_stat,
        power_type,
        pet_name_token,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        filename,
        class_mask,
        hybrid_class
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
pub(crate) fn ChrRaces() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ChrRaces (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        faction INTEGER  NOT NULL,
        exploration_sound INTEGER  NOT NULL,
        male_display INTEGER  NOT NULL,
        female_display INTEGER  NOT NULL,
        client_prefix TEXT  NOT NULL,
        speed_modifier REAL  NOT NULL,
        base_lang INTEGER  NOT NULL,
        creature_type INTEGER  NOT NULL,
        login_effect INTEGER  NOT NULL,
        unknown1 INTEGER  NOT NULL,
        res_sickness_spell INTEGER  NOT NULL,
        splash_sound_entry INTEGER  NOT NULL,
        unknown2 INTEGER  NOT NULL,
        client_file_path TEXT  NOT NULL,
        cinematic_sequence INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        facial_hair_customisation_0 TEXT NOT NULL,
        facial_hair_customisation_1 TEXT NOT NULL,
        hair_customisation TEXT  NOT NULL
    );"
    ,
    "INSERT INTO ChrRaces (
        id,
        flags,
        faction,
        exploration_sound,
        male_display,
        female_display,
        client_prefix,
        speed_modifier,
        base_lang,
        creature_type,
        login_effect,
        unknown1,
        res_sickness_spell,
        splash_sound_entry,
        unknown2,
        client_file_path,
        cinematic_sequence,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        facial_hair_customisation_0,
        facial_hair_customisation_1,
        hair_customisation
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
pub(crate) fn CinematicCamera() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CinematicCamera (
        id INTEGER PRIMARY KEY NOT NULL,
        model TEXT  NOT NULL,
        sound_entry INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        rotation REAL  NOT NULL
    );"
    ,
    "INSERT INTO CinematicCamera (
        id,
        model,
        sound_entry,
        location_x,
        location_y,
        location_z,
        rotation
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
pub(crate) fn CinematicSequences() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CinematicSequences (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_entry INTEGER  NOT NULL,
        cinematic_camera_1_0 INTEGER NOT NULL,
        cinematic_camera_1_1 INTEGER NOT NULL,
        cinematic_camera_1_2 INTEGER NOT NULL,
        cinematic_camera_1_3 INTEGER NOT NULL,
        cinematic_camera_1_4 INTEGER NOT NULL,
        cinematic_camera_1_5 INTEGER NOT NULL,
        cinematic_camera_1_6 INTEGER NOT NULL,
        cinematic_camera_1_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CinematicSequences (
        id,
        sound_entry,
        cinematic_camera_1_0,
        cinematic_camera_1_1,
        cinematic_camera_1_2,
        cinematic_camera_1_3,
        cinematic_camera_1_4,
        cinematic_camera_1_5,
        cinematic_camera_1_6,
        cinematic_camera_1_7
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
pub(crate) fn CreatureDisplayInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureDisplayInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        model INTEGER  NOT NULL,
        sound INTEGER  NOT NULL,
        extended_display_info INTEGER  NOT NULL,
        creature_model_scale REAL  NOT NULL,
        creature_model_alpha INTEGER  NOT NULL,
        texture_variation_0 TEXT NOT NULL,
        texture_variation_1 TEXT NOT NULL,
        texture_variation_2 TEXT NOT NULL,
        size INTEGER  NOT NULL,
        blood INTEGER  NOT NULL,
        npc_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CreatureDisplayInfo (
        id,
        model,
        sound,
        extended_display_info,
        creature_model_scale,
        creature_model_alpha,
        texture_variation_0,
        texture_variation_1,
        texture_variation_2,
        size,
        blood,
        npc_sound
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
pub(crate) fn CreatureDisplayInfoExtra() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureDisplayInfoExtra (
        id INTEGER PRIMARY KEY NOT NULL,
        display_race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        skin INTEGER  NOT NULL,
        face INTEGER  NOT NULL,
        hair_style INTEGER  NOT NULL,
        hair_colour INTEGER  NOT NULL,
        facial_hair INTEGER  NOT NULL,
        npc_item_display_0 INTEGER NOT NULL,
        npc_item_display_1 INTEGER NOT NULL,
        npc_item_display_2 INTEGER NOT NULL,
        npc_item_display_3 INTEGER NOT NULL,
        npc_item_display_4 INTEGER NOT NULL,
        npc_item_display_5 INTEGER NOT NULL,
        npc_item_display_6 INTEGER NOT NULL,
        npc_item_display_7 INTEGER NOT NULL,
        npc_item_display_8 INTEGER NOT NULL,
        flags INTEGER  NOT NULL,
        bake_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO CreatureDisplayInfoExtra (
        id,
        display_race,
        gender,
        skin,
        face,
        hair_style,
        hair_colour,
        facial_hair,
        npc_item_display_0,
        npc_item_display_1,
        npc_item_display_2,
        npc_item_display_3,
        npc_item_display_4,
        npc_item_display_5,
        npc_item_display_6,
        npc_item_display_7,
        npc_item_display_8,
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
        ?19
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
        pet_food_mask INTEGER  NOT NULL,
        pet_talent_type INTEGER  NOT NULL,
        category INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        icon_path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO CreatureFamily (
        id,
        min_scale,
        min_scale_level,
        max_scale,
        max_scale_level,
        pet_food_mask,
        pet_talent_type,
        category,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        icon_path
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
pub(crate) fn CreatureModelData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureModelData (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        model_path TEXT  NOT NULL,
        size INTEGER  NOT NULL,
        model_scale REAL  NOT NULL,
        blood INTEGER  NOT NULL,
        footprint_texture INTEGER  NOT NULL,
        footprint_texture_length REAL  NOT NULL,
        footprint_texture_width REAL  NOT NULL,
        footprint_texture_scale REAL  NOT NULL,
        foley_material INTEGER  NOT NULL,
        footstep_shake_size INTEGER  NOT NULL,
        death_thud_shake_size INTEGER  NOT NULL,
        collision_width REAL  NOT NULL,
        collision_height REAL  NOT NULL,
        mount_height REAL  NOT NULL
    );"
    ,
    "INSERT INTO CreatureModelData (
        id,
        flags,
        model_path,
        size,
        model_scale,
        blood,
        footprint_texture,
        footprint_texture_length,
        footprint_texture_width,
        footprint_texture_scale,
        foley_material,
        footstep_shake_size,
        death_thud_shake_size,
        collision_width,
        collision_height,
        mount_height
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
pub(crate) fn CreatureSoundData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureSoundData (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_exertion INTEGER  NOT NULL,
        sound_exertion_critical INTEGER  NOT NULL,
        sound_injury INTEGER  NOT NULL,
        sound_injury_critical INTEGER  NOT NULL,
        sound_injury_crushing_blow INTEGER  NOT NULL,
        sound_death INTEGER  NOT NULL,
        sound_stun INTEGER  NOT NULL,
        sound_stand INTEGER  NOT NULL,
        sound_footstep INTEGER  NOT NULL,
        sound_aggro INTEGER  NOT NULL,
        sound_wing_flap INTEGER  NOT NULL,
        sound_wing_glide INTEGER  NOT NULL,
        sound_alert INTEGER  NOT NULL,
        sound_fidget INTEGER  NOT NULL,
        custom_attack INTEGER  NOT NULL,
        npc_sound INTEGER  NOT NULL,
        loop_sound INTEGER  NOT NULL,
        creature_impact_type INTEGER  NOT NULL,
        sound_jump_start INTEGER  NOT NULL,
        sound_jump_end INTEGER  NOT NULL,
        sound_pet_attack INTEGER  NOT NULL,
        sound_pet_order INTEGER  NOT NULL,
        sound_pet_dismiss INTEGER  NOT NULL,
        fidget_delay_seconds_min INTEGER  NOT NULL,
        fidget_delay_seconds_max INTEGER  NOT NULL,
        birth_sound INTEGER  NOT NULL,
        spell_cast_directed_sound INTEGER  NOT NULL,
        submerge_sound INTEGER  NOT NULL,
        submerged_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CreatureSoundData (
        id,
        sound_exertion,
        sound_exertion_critical,
        sound_injury,
        sound_injury_critical,
        sound_injury_crushing_blow,
        sound_death,
        sound_stun,
        sound_stand,
        sound_footstep,
        sound_aggro,
        sound_wing_flap,
        sound_wing_glide,
        sound_alert,
        sound_fidget,
        custom_attack,
        npc_sound,
        loop_sound,
        creature_impact_type,
        sound_jump_start,
        sound_jump_end,
        sound_pet_attack,
        sound_pet_order,
        sound_pet_dismiss,
        fidget_delay_seconds_min,
        fidget_delay_seconds_max,
        birth_sound,
        spell_cast_directed_sound,
        submerge_sound,
        submerged_sound
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
pub(crate) fn CreatureSpellData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureSpellData (
        id INTEGER PRIMARY KEY NOT NULL,
        spell_0 INTEGER NOT NULL,
        spell_1 INTEGER NOT NULL,
        spell_2 INTEGER NOT NULL,
        spell_3 INTEGER NOT NULL,
        cooldown_time_1_0 INTEGER NOT NULL,
        cooldown_time_1_1 INTEGER NOT NULL,
        cooldown_time_1_2 INTEGER NOT NULL,
        cooldown_time_1_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO CreatureSpellData (
        id,
        spell_0,
        spell_1,
        spell_2,
        spell_3,
        cooldown_time_1_0,
        cooldown_time_1_1,
        cooldown_time_1_2,
        cooldown_time_1_3
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
pub(crate) fn CreatureType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS CreatureType (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO CreatureType (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
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
pub(crate) fn DeathThudLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DeathThudLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        size INTEGER  NOT NULL,
        terrain_type INTEGER  NOT NULL,
        sound_entry INTEGER  NOT NULL,
        sound_entry_water INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO DeathThudLookups (
        id,
        size,
        terrain_type,
        sound_entry,
        sound_entry_water
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
pub(crate) fn DurabilityCosts() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS DurabilityCosts (
        id INTEGER PRIMARY KEY NOT NULL,
        weapon_subclass_cost_0 INTEGER NOT NULL,
        weapon_subclass_cost_1 INTEGER NOT NULL,
        weapon_subclass_cost_2 INTEGER NOT NULL,
        weapon_subclass_cost_3 INTEGER NOT NULL,
        weapon_subclass_cost_4 INTEGER NOT NULL,
        weapon_subclass_cost_5 INTEGER NOT NULL,
        weapon_subclass_cost_6 INTEGER NOT NULL,
        weapon_subclass_cost_7 INTEGER NOT NULL,
        weapon_subclass_cost_8 INTEGER NOT NULL,
        weapon_subclass_cost_9 INTEGER NOT NULL,
        weapon_subclass_cost_10 INTEGER NOT NULL,
        weapon_subclass_cost_11 INTEGER NOT NULL,
        weapon_subclass_cost_12 INTEGER NOT NULL,
        weapon_subclass_cost_13 INTEGER NOT NULL,
        weapon_subclass_cost_14 INTEGER NOT NULL,
        weapon_subclass_cost_15 INTEGER NOT NULL,
        weapon_subclass_cost_16 INTEGER NOT NULL,
        weapon_subclass_cost_17 INTEGER NOT NULL,
        weapon_subclass_cost_18 INTEGER NOT NULL,
        weapon_subclass_cost_19 INTEGER NOT NULL,
        weapon_subclass_cost_20 INTEGER NOT NULL,
        armour_subclass_cost_0 INTEGER NOT NULL,
        armour_subclass_cost_1 INTEGER NOT NULL,
        armour_subclass_cost_2 INTEGER NOT NULL,
        armour_subclass_cost_3 INTEGER NOT NULL,
        armour_subclass_cost_4 INTEGER NOT NULL,
        armour_subclass_cost_5 INTEGER NOT NULL,
        armour_subclass_cost_6 INTEGER NOT NULL,
        armour_subclass_cost_7 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO DurabilityCosts (
        id,
        weapon_subclass_cost_0,
        weapon_subclass_cost_1,
        weapon_subclass_cost_2,
        weapon_subclass_cost_3,
        weapon_subclass_cost_4,
        weapon_subclass_cost_5,
        weapon_subclass_cost_6,
        weapon_subclass_cost_7,
        weapon_subclass_cost_8,
        weapon_subclass_cost_9,
        weapon_subclass_cost_10,
        weapon_subclass_cost_11,
        weapon_subclass_cost_12,
        weapon_subclass_cost_13,
        weapon_subclass_cost_14,
        weapon_subclass_cost_15,
        weapon_subclass_cost_16,
        weapon_subclass_cost_17,
        weapon_subclass_cost_18,
        weapon_subclass_cost_19,
        weapon_subclass_cost_20,
        armour_subclass_cost_0,
        armour_subclass_cost_1,
        armour_subclass_cost_2,
        armour_subclass_cost_3,
        armour_subclass_cost_4,
        armour_subclass_cost_5,
        armour_subclass_cost_6,
        armour_subclass_cost_7
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
pub(crate) fn Emotes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Emotes (
        id INTEGER PRIMARY KEY NOT NULL,
        emote_slash_command TEXT  NOT NULL,
        animation_data INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        spec_proc INTEGER  NOT NULL,
        emote_spec_proc_param INTEGER  NOT NULL,
        event_sound_entry INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Emotes (
        id,
        emote_slash_command,
        animation_data,
        flags,
        spec_proc,
        emote_spec_proc_param,
        event_sound_entry
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
pub(crate) fn EmotesText() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EmotesText (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        emote INTEGER  NOT NULL,
        emote_text_data_0 INTEGER NOT NULL,
        emote_text_data_1 INTEGER NOT NULL,
        emote_text_data_2 INTEGER NOT NULL,
        emote_text_data_3 INTEGER NOT NULL,
        emote_text_data_4 INTEGER NOT NULL,
        emote_text_data_5 INTEGER NOT NULL,
        emote_text_data_6 INTEGER NOT NULL,
        emote_text_data_7 INTEGER NOT NULL,
        emote_text_data_8 INTEGER NOT NULL,
        emote_text_data_9 INTEGER NOT NULL,
        emote_text_data_10 INTEGER NOT NULL,
        emote_text_data_11 INTEGER NOT NULL,
        emote_text_data_12 INTEGER NOT NULL,
        emote_text_data_13 INTEGER NOT NULL,
        emote_text_data_14 INTEGER NOT NULL,
        emote_text_data_15 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO EmotesText (
        id,
        name,
        emote,
        emote_text_data_0,
        emote_text_data_1,
        emote_text_data_2,
        emote_text_data_3,
        emote_text_data_4,
        emote_text_data_5,
        emote_text_data_6,
        emote_text_data_7,
        emote_text_data_8,
        emote_text_data_9,
        emote_text_data_10,
        emote_text_data_11,
        emote_text_data_12,
        emote_text_data_13,
        emote_text_data_14,
        emote_text_data_15
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
        text_en_gb TEXT NOT NULL,
        text_ko_kr TEXT NOT NULL,
        text_fr_fr TEXT NOT NULL,
        text_de_de TEXT NOT NULL,
        text_en_cn TEXT NOT NULL,
        text_en_tw TEXT NOT NULL,
        text_es_es TEXT NOT NULL,
        text_es_mx TEXT NOT NULL,
        text_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO EmotesTextData (
        id,
        text_en_gb,
        text_ko_kr,
        text_fr_fr,
        text_de_de,
        text_en_cn,
        text_en_tw,
        text_es_es,
        text_es_mx,
        text_flags
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
pub(crate) fn EmotesTextSound() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EmotesTextSound (
        id INTEGER PRIMARY KEY NOT NULL,
        emotes_text INTEGER  NOT NULL,
        race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO EmotesTextSound (
        id,
        emotes_text,
        race,
        gender,
        sound
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
pub(crate) fn EnvironmentalDamage() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS EnvironmentalDamage (
        id INTEGER PRIMARY KEY NOT NULL,
        en INTEGER  NOT NULL,
        spell_visual_kit INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO EnvironmentalDamage (
        id,
        en,
        spell_visual_kit
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Exhaustion() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Exhaustion (
        id INTEGER PRIMARY KEY NOT NULL,
        experience INTEGER  NOT NULL,
        factor REAL  NOT NULL,
        outdoor_hours REAL  NOT NULL,
        inn_hours REAL  NOT NULL,
        state_name_en_gb TEXT NOT NULL,
        state_name_ko_kr TEXT NOT NULL,
        state_name_fr_fr TEXT NOT NULL,
        state_name_de_de TEXT NOT NULL,
        state_name_en_cn TEXT NOT NULL,
        state_name_en_tw TEXT NOT NULL,
        state_name_es_es TEXT NOT NULL,
        state_name_es_mx TEXT NOT NULL,
        state_name_flags TEXT NOT NULL,
        threshold REAL  NOT NULL
    );"
    ,
    "INSERT INTO Exhaustion (
        id,
        experience,
        factor,
        outdoor_hours,
        inn_hours,
        state_name_en_gb,
        state_name_ko_kr,
        state_name_fr_fr,
        state_name_de_de,
        state_name_en_cn,
        state_name_en_tw,
        state_name_es_es,
        state_name_es_mx,
        state_name_flags,
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
        ?15
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
        parent_faction INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags TEXT NOT NULL
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
        parent_faction,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        description_en_gb,
        description_ko_kr,
        description_fr_fr,
        description_de_de,
        description_en_cn,
        description_en_tw,
        description_es_es,
        description_es_mx,
        description_flags
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
pub(crate) fn FactionGroup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FactionGroup (
        id INTEGER PRIMARY KEY NOT NULL,
        mask_id INTEGER  NOT NULL,
        internal_name TEXT  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO FactionGroup (
        id,
        mask_id,
        internal_name,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
        friends_0 INTEGER NOT NULL,
        friends_1 INTEGER NOT NULL,
        friends_2 INTEGER NOT NULL,
        friends_3 INTEGER NOT NULL
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
        friends_0,
        friends_1,
        friends_2,
        friends_3
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
pub(crate) fn FootprintTextures() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FootprintTextures (
        id INTEGER PRIMARY KEY NOT NULL,
        footstep_file_path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO FootprintTextures (
        id,
        footstep_file_path
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn FootstepTerrainLookup() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS FootstepTerrainLookup (
        id INTEGER PRIMARY KEY NOT NULL,
        creature_footstep INTEGER  NOT NULL,
        terrain_type INTEGER  NOT NULL,
        sound_entry INTEGER  NOT NULL,
        sound_entry_splash INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO FootstepTerrainLookup (
        id,
        creature_footstep,
        terrain_type,
        sound_entry,
        sound_entry_splash
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
pub(crate) fn GMSurveyCurrentSurvey() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveyCurrentSurvey (
        language INTEGER  NOT NULL,
        gm_survey INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GMSurveyCurrentSurvey (
        language,
        gm_survey
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn GMSurveyQuestions() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveyQuestions (
        id INTEGER PRIMARY KEY NOT NULL,
        question_en_gb TEXT NOT NULL,
        question_ko_kr TEXT NOT NULL,
        question_fr_fr TEXT NOT NULL,
        question_de_de TEXT NOT NULL,
        question_en_cn TEXT NOT NULL,
        question_en_tw TEXT NOT NULL,
        question_es_es TEXT NOT NULL,
        question_es_mx TEXT NOT NULL,
        question_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GMSurveyQuestions (
        id,
        question_en_gb,
        question_ko_kr,
        question_fr_fr,
        question_de_de,
        question_en_cn,
        question_en_tw,
        question_es_es,
        question_es_mx,
        question_flags
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
pub(crate) fn GMSurveySurveys() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMSurveySurveys (
        id INTEGER PRIMARY KEY NOT NULL,
        questions_0 INTEGER NOT NULL,
        questions_1 INTEGER NOT NULL,
        questions_2 INTEGER NOT NULL,
        questions_3 INTEGER NOT NULL,
        questions_4 INTEGER NOT NULL,
        questions_5 INTEGER NOT NULL,
        questions_6 INTEGER NOT NULL,
        questions_7 INTEGER NOT NULL,
        questions_8 INTEGER NOT NULL,
        questions_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO GMSurveySurveys (
        id,
        questions_0,
        questions_1,
        questions_2,
        questions_3,
        questions_4,
        questions_5,
        questions_6,
        questions_7,
        questions_8,
        questions_9
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
pub(crate) fn GMTicketCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GMTicketCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GMTicketCategory (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn GameObjectDisplayInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GameObjectDisplayInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        model_name TEXT  NOT NULL,
        sound_entry_0 INTEGER NOT NULL,
        sound_entry_1 INTEGER NOT NULL,
        sound_entry_2 INTEGER NOT NULL,
        sound_entry_3 INTEGER NOT NULL,
        sound_entry_4 INTEGER NOT NULL,
        sound_entry_5 INTEGER NOT NULL,
        sound_entry_6 INTEGER NOT NULL,
        sound_entry_7 INTEGER NOT NULL,
        sound_entry_8 INTEGER NOT NULL,
        sound_entry_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO GameObjectDisplayInfo (
        id,
        model_name,
        sound_entry_0,
        sound_entry_1,
        sound_entry_2,
        sound_entry_3,
        sound_entry_4,
        sound_entry_5,
        sound_entry_6,
        sound_entry_7,
        sound_entry_8,
        sound_entry_9
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
pub(crate) fn GameTips() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GameTips (
        id INTEGER PRIMARY KEY NOT NULL,
        text_en_gb TEXT NOT NULL,
        text_ko_kr TEXT NOT NULL,
        text_fr_fr TEXT NOT NULL,
        text_de_de TEXT NOT NULL,
        text_en_cn TEXT NOT NULL,
        text_en_tw TEXT NOT NULL,
        text_es_es TEXT NOT NULL,
        text_es_mx TEXT NOT NULL,
        text_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO GameTips (
        id,
        text_en_gb,
        text_ko_kr,
        text_fr_fr,
        text_de_de,
        text_en_cn,
        text_en_tw,
        text_es_es,
        text_es_mx,
        text_flags
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
pub(crate) fn GroundEffectDoodad() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS GroundEffectDoodad (
        id INTEGER PRIMARY KEY NOT NULL,
        internal_id INTEGER  NOT NULL,
        doodad_path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO GroundEffectDoodad (
        id,
        internal_id,
        doodad_path
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
        doodad_0 INTEGER NOT NULL,
        doodad_1 INTEGER NOT NULL,
        doodad_2 INTEGER NOT NULL,
        doodad_3 INTEGER NOT NULL,
        density INTEGER  NOT NULL,
        terrain_type INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO GroundEffectTexture (
        id,
        doodad_0,
        doodad_1,
        doodad_2,
        doodad_3,
        density,
        terrain_type
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
pub(crate) fn HelmetGeosetVisData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS HelmetGeosetVisData (
        id INTEGER PRIMARY KEY NOT NULL,
        hide_geoset_0 INTEGER NOT NULL,
        hide_geoset_1 INTEGER NOT NULL,
        hide_geoset_2 INTEGER NOT NULL,
        hide_geoset_3 INTEGER NOT NULL,
        hide_geoset_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO HelmetGeosetVisData (
        id,
        hide_geoset_0,
        hide_geoset_1,
        hide_geoset_2,
        hide_geoset_3,
        hide_geoset_4
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
pub(crate) fn ItemBagFamily() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemBagFamily (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemBagFamily (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn ItemClass() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemClass (
        id INTEGER PRIMARY KEY NOT NULL,
        subclass_map INTEGER  NOT NULL,
        item_class INTEGER  NOT NULL,
        class_name_en_gb TEXT NOT NULL,
        class_name_ko_kr TEXT NOT NULL,
        class_name_fr_fr TEXT NOT NULL,
        class_name_de_de TEXT NOT NULL,
        class_name_en_cn TEXT NOT NULL,
        class_name_en_tw TEXT NOT NULL,
        class_name_es_es TEXT NOT NULL,
        class_name_es_mx TEXT NOT NULL,
        class_name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemClass (
        id,
        subclass_map,
        item_class,
        class_name_en_gb,
        class_name_ko_kr,
        class_name_fr_fr,
        class_name_de_de,
        class_name_en_cn,
        class_name_en_tw,
        class_name_es_es,
        class_name_es_mx,
        class_name_flags
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
        spell_visual INTEGER  NOT NULL,
        group_sound_index INTEGER  NOT NULL,
        helmet_geoset_vis_0 INTEGER NOT NULL,
        helmet_geoset_vis_1 INTEGER NOT NULL,
        textures_0 TEXT NOT NULL,
        textures_1 TEXT NOT NULL,
        textures_2 TEXT NOT NULL,
        textures_3 TEXT NOT NULL,
        textures_4 TEXT NOT NULL,
        textures_5 TEXT NOT NULL,
        textures_6 TEXT NOT NULL,
        textures_7 TEXT NOT NULL,
        item_visual INTEGER  NOT NULL
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
        spell_visual,
        group_sound_index,
        helmet_geoset_vis_0,
        helmet_geoset_vis_1,
        textures_0,
        textures_1,
        textures_2,
        textures_3,
        textures_4,
        textures_5,
        textures_6,
        textures_7,
        item_visual
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
pub(crate) fn ItemGroupSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemGroupSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_entry_0 INTEGER NOT NULL,
        sound_entry_1 INTEGER NOT NULL,
        sound_entry_2 INTEGER NOT NULL,
        sound_entry_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ItemGroupSounds (
        id,
        sound_entry_0,
        sound_entry_1,
        sound_entry_2,
        sound_entry_3
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
pub(crate) fn ItemPetFood() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemPetFood (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemPetFood (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn ItemRandomProperties() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemRandomProperties (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        spell_item_enchantment_0 INTEGER NOT NULL,
        spell_item_enchantment_1 INTEGER NOT NULL,
        spell_item_enchantment_2 INTEGER NOT NULL,
        spell_item_enchantment_3 INTEGER NOT NULL,
        spell_item_enchantment_4 INTEGER NOT NULL,
        suffix_en_gb TEXT NOT NULL,
        suffix_ko_kr TEXT NOT NULL,
        suffix_fr_fr TEXT NOT NULL,
        suffix_de_de TEXT NOT NULL,
        suffix_en_cn TEXT NOT NULL,
        suffix_en_tw TEXT NOT NULL,
        suffix_es_es TEXT NOT NULL,
        suffix_es_mx TEXT NOT NULL,
        suffix_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemRandomProperties (
        id,
        name,
        spell_item_enchantment_0,
        spell_item_enchantment_1,
        spell_item_enchantment_2,
        spell_item_enchantment_3,
        spell_item_enchantment_4,
        suffix_en_gb,
        suffix_ko_kr,
        suffix_fr_fr,
        suffix_de_de,
        suffix_en_cn,
        suffix_en_tw,
        suffix_es_es,
        suffix_es_mx,
        suffix_flags
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
pub(crate) fn ItemSet() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemSet (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        items_0 INTEGER NOT NULL,
        items_1 INTEGER NOT NULL,
        items_2 INTEGER NOT NULL,
        items_3 INTEGER NOT NULL,
        items_4 INTEGER NOT NULL,
        items_5 INTEGER NOT NULL,
        items_6 INTEGER NOT NULL,
        items_7 INTEGER NOT NULL,
        items_8 INTEGER NOT NULL,
        items_9 INTEGER NOT NULL,
        bank_item_0 INTEGER NOT NULL,
        bank_item_1 INTEGER NOT NULL,
        bank_item_2 INTEGER NOT NULL,
        bank_item_3 INTEGER NOT NULL,
        bank_item_4 INTEGER NOT NULL,
        bank_item_5 INTEGER NOT NULL,
        bank_item_6 INTEGER NOT NULL,
        set_spell_0 INTEGER NOT NULL,
        set_spell_1 INTEGER NOT NULL,
        set_spell_2 INTEGER NOT NULL,
        set_spell_3 INTEGER NOT NULL,
        set_spell_4 INTEGER NOT NULL,
        set_spell_5 INTEGER NOT NULL,
        set_spell_6 INTEGER NOT NULL,
        set_spell_7 INTEGER NOT NULL,
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
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        items_0,
        items_1,
        items_2,
        items_3,
        items_4,
        items_5,
        items_6,
        items_7,
        items_8,
        items_9,
        bank_item_0,
        bank_item_1,
        bank_item_2,
        bank_item_3,
        bank_item_4,
        bank_item_5,
        bank_item_6,
        set_spell_0,
        set_spell_1,
        set_spell_2,
        set_spell_3,
        set_spell_4,
        set_spell_5,
        set_spell_6,
        set_spell_7,
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
        ?45
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemSubClass() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemSubClass (
        item_class INTEGER  NOT NULL,
        subclass INTEGER  NOT NULL,
        prerequisite_proficiency INTEGER  NOT NULL,
        postrequisite_proficiency INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        display_flags INTEGER  NOT NULL,
        weapon_parry_sequence INTEGER  NOT NULL,
        weapon_ready_sequence INTEGER  NOT NULL,
        weapon_attack_sequence INTEGER  NOT NULL,
        weapon_swing_size INTEGER  NOT NULL,
        display_name_en_gb TEXT NOT NULL,
        display_name_ko_kr TEXT NOT NULL,
        display_name_fr_fr TEXT NOT NULL,
        display_name_de_de TEXT NOT NULL,
        display_name_en_cn TEXT NOT NULL,
        display_name_en_tw TEXT NOT NULL,
        display_name_es_es TEXT NOT NULL,
        display_name_es_mx TEXT NOT NULL,
        display_name_flags TEXT NOT NULL,
        verbose_name_en_gb TEXT NOT NULL,
        verbose_name_ko_kr TEXT NOT NULL,
        verbose_name_fr_fr TEXT NOT NULL,
        verbose_name_de_de TEXT NOT NULL,
        verbose_name_en_cn TEXT NOT NULL,
        verbose_name_en_tw TEXT NOT NULL,
        verbose_name_es_es TEXT NOT NULL,
        verbose_name_es_mx TEXT NOT NULL,
        verbose_name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemSubClass (
        item_class,
        subclass,
        prerequisite_proficiency,
        postrequisite_proficiency,
        flags,
        display_flags,
        weapon_parry_sequence,
        weapon_ready_sequence,
        weapon_attack_sequence,
        weapon_swing_size,
        display_name_en_gb,
        display_name_ko_kr,
        display_name_fr_fr,
        display_name_de_de,
        display_name_en_cn,
        display_name_en_tw,
        display_name_es_es,
        display_name_es_mx,
        display_name_flags,
        verbose_name_en_gb,
        verbose_name_ko_kr,
        verbose_name_fr_fr,
        verbose_name_de_de,
        verbose_name_en_cn,
        verbose_name_en_tw,
        verbose_name_es_es,
        verbose_name_es_mx,
        verbose_name_flags
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
pub(crate) fn ItemSubClassMask() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemSubClassMask (
        subclass INTEGER  NOT NULL,
        mask INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ItemSubClassMask (
        subclass,
        mask,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn ItemVisualEffects() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemVisualEffects (
        id INTEGER PRIMARY KEY NOT NULL,
        model_path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO ItemVisualEffects (
        id,
        model_path
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn ItemVisuals() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ItemVisuals (
        id INTEGER PRIMARY KEY NOT NULL,
        item_visual_effects_0 INTEGER NOT NULL,
        item_visual_effects_1 INTEGER NOT NULL,
        item_visual_effects_2 INTEGER NOT NULL,
        item_visual_effects_3 INTEGER NOT NULL,
        item_visual_effects_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO ItemVisuals (
        id,
        item_visual_effects_0,
        item_visual_effects_1,
        item_visual_effects_2,
        item_visual_effects_3,
        item_visual_effects_4
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
pub(crate) fn LFGDungeons() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LFGDungeons (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        min_allowed_level INTEGER  NOT NULL,
        max_allowed_level INTEGER  NOT NULL,
        instance_type INTEGER  NOT NULL,
        faction INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LFGDungeons (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        min_allowed_level,
        max_allowed_level,
        instance_type,
        faction
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
pub(crate) fn LanguageWords() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LanguageWords (
        id INTEGER PRIMARY KEY NOT NULL,
        language INTEGER  NOT NULL,
        word TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LanguageWords (
        id,
        language,
        word
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Languages (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn Light() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Light (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        falloff_start REAL  NOT NULL,
        falloff_end REAL  NOT NULL,
        light_params_0 INTEGER NOT NULL,
        light_params_1 INTEGER NOT NULL,
        light_params_2 INTEGER NOT NULL,
        light_params_3 INTEGER NOT NULL,
        light_params_4 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO Light (
        id,
        map,
        location_x,
        location_y,
        location_z,
        falloff_start,
        falloff_end,
        light_params_0,
        light_params_1,
        light_params_2,
        light_params_3,
        light_params_4
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
pub(crate) fn LightParams() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LightParams (
        id INTEGER PRIMARY KEY NOT NULL,
        highlight_sky INTEGER  NOT NULL,
        light_skybox INTEGER  NOT NULL,
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
        light_skybox,
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
pub(crate) fn LightSkybox() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LightSkybox (
        id INTEGER PRIMARY KEY NOT NULL,
        skybox_model_path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LightSkybox (
        id,
        skybox_model_path
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LiquidType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LiquidType (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        ty INTEGER  NOT NULL,
        spell INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LiquidType (
        id,
        name,
        ty,
        spell
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn LoadingScreenTaxiSplines() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LoadingScreenTaxiSplines (
        id INTEGER PRIMARY KEY NOT NULL,
        taxi_path INTEGER  NOT NULL,
        location_x_0 REAL NOT NULL,
        location_x_1 REAL NOT NULL,
        location_x_2 REAL NOT NULL,
        location_x_3 REAL NOT NULL,
        location_x_4 REAL NOT NULL,
        location_x_5 REAL NOT NULL,
        location_x_6 REAL NOT NULL,
        location_x_7 REAL NOT NULL,
        location_y_0 REAL NOT NULL,
        location_y_1 REAL NOT NULL,
        location_y_2 REAL NOT NULL,
        location_y_3 REAL NOT NULL,
        location_y_4 REAL NOT NULL,
        location_y_5 REAL NOT NULL,
        location_y_6 REAL NOT NULL,
        location_y_7 REAL NOT NULL,
        leg_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO LoadingScreenTaxiSplines (
        id,
        taxi_path,
        location_x_0,
        location_x_1,
        location_x_2,
        location_x_3,
        location_x_4,
        location_x_5,
        location_x_6,
        location_x_7,
        location_y_0,
        location_y_1,
        location_y_2,
        location_y_3,
        location_y_4,
        location_y_5,
        location_y_6,
        location_y_7,
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
pub(crate) fn LoadingScreens() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LoadingScreens (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        file_path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LoadingScreens (
        id,
        name,
        file_path
        ) VALUES (
        ?1,
        ?2,
        ?3
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
        property_0 INTEGER NOT NULL,
        property_1 INTEGER NOT NULL,
        property_2 INTEGER NOT NULL,
        property_3 INTEGER NOT NULL,
        property_4 INTEGER NOT NULL,
        property_5 INTEGER NOT NULL,
        property_6 INTEGER NOT NULL,
        property_7 INTEGER NOT NULL,
        required_skill_0 INTEGER NOT NULL,
        required_skill_1 INTEGER NOT NULL,
        required_skill_2 INTEGER NOT NULL,
        required_skill_3 INTEGER NOT NULL,
        required_skill_4 INTEGER NOT NULL,
        required_skill_5 INTEGER NOT NULL,
        required_skill_6 INTEGER NOT NULL,
        required_skill_7 INTEGER NOT NULL,
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
        property_0,
        property_1,
        property_2,
        property_3,
        property_4,
        property_5,
        property_6,
        property_7,
        required_skill_0,
        required_skill_1,
        required_skill_2,
        required_skill_3,
        required_skill_4,
        required_skill_5,
        required_skill_6,
        required_skill_7,
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
pub(crate) fn LockType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS LockType (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        resource_name_en_gb TEXT NOT NULL,
        resource_name_ko_kr TEXT NOT NULL,
        resource_name_fr_fr TEXT NOT NULL,
        resource_name_de_de TEXT NOT NULL,
        resource_name_en_cn TEXT NOT NULL,
        resource_name_en_tw TEXT NOT NULL,
        resource_name_es_es TEXT NOT NULL,
        resource_name_es_mx TEXT NOT NULL,
        resource_name_flags TEXT NOT NULL,
        verb_en_gb TEXT NOT NULL,
        verb_ko_kr TEXT NOT NULL,
        verb_fr_fr TEXT NOT NULL,
        verb_de_de TEXT NOT NULL,
        verb_en_cn TEXT NOT NULL,
        verb_en_tw TEXT NOT NULL,
        verb_es_es TEXT NOT NULL,
        verb_es_mx TEXT NOT NULL,
        verb_flags TEXT NOT NULL,
        cursor_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO LockType (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        resource_name_en_gb,
        resource_name_ko_kr,
        resource_name_fr_fr,
        resource_name_de_de,
        resource_name_en_cn,
        resource_name_en_tw,
        resource_name_es_es,
        resource_name_es_mx,
        resource_name_flags,
        verb_en_gb,
        verb_ko_kr,
        verb_fr_fr,
        verb_de_de,
        verb_en_cn,
        verb_en_tw,
        verb_es_es,
        verb_es_mx,
        verb_flags,
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
        ?29
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn MailTemplate() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS MailTemplate (
        id INTEGER PRIMARY KEY NOT NULL,
        body_en_gb TEXT NOT NULL,
        body_ko_kr TEXT NOT NULL,
        body_fr_fr TEXT NOT NULL,
        body_de_de TEXT NOT NULL,
        body_en_cn TEXT NOT NULL,
        body_en_tw TEXT NOT NULL,
        body_es_es TEXT NOT NULL,
        body_es_mx TEXT NOT NULL,
        body_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO MailTemplate (
        id,
        body_en_gb,
        body_ko_kr,
        body_fr_fr,
        body_de_de,
        body_en_cn,
        body_en_tw,
        body_es_es,
        body_es_mx,
        body_flags
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
pub(crate) fn Map() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Map (
        id INTEGER PRIMARY KEY NOT NULL,
        internal_name TEXT  NOT NULL,
        instance_type INTEGER  NOT NULL,
        battleground INTEGER  NOT NULL,
        map_name_en_gb TEXT NOT NULL,
        map_name_ko_kr TEXT NOT NULL,
        map_name_fr_fr TEXT NOT NULL,
        map_name_de_de TEXT NOT NULL,
        map_name_en_cn TEXT NOT NULL,
        map_name_en_tw TEXT NOT NULL,
        map_name_es_es TEXT NOT NULL,
        map_name_es_mx TEXT NOT NULL,
        map_name_flags TEXT NOT NULL,
        min_level INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        max_players INTEGER  NOT NULL,
        unknown_0 INTEGER NOT NULL,
        unknown_1 INTEGER NOT NULL,
        unknown_2 INTEGER NOT NULL,
        area_table INTEGER  NOT NULL,
        map_description_horde_en_gb TEXT NOT NULL,
        map_description_horde_ko_kr TEXT NOT NULL,
        map_description_horde_fr_fr TEXT NOT NULL,
        map_description_horde_de_de TEXT NOT NULL,
        map_description_horde_en_cn TEXT NOT NULL,
        map_description_horde_en_tw TEXT NOT NULL,
        map_description_horde_es_es TEXT NOT NULL,
        map_description_horde_es_mx TEXT NOT NULL,
        map_description_horde_flags TEXT NOT NULL,
        map_description_alliance_en_gb TEXT NOT NULL,
        map_description_alliance_ko_kr TEXT NOT NULL,
        map_description_alliance_fr_fr TEXT NOT NULL,
        map_description_alliance_de_de TEXT NOT NULL,
        map_description_alliance_en_cn TEXT NOT NULL,
        map_description_alliance_en_tw TEXT NOT NULL,
        map_description_alliance_es_es TEXT NOT NULL,
        map_description_alliance_es_mx TEXT NOT NULL,
        map_description_alliance_flags TEXT NOT NULL,
        loading_screen INTEGER  NOT NULL,
        raid_offset INTEGER  NOT NULL,
        unknown_2_0 INTEGER NOT NULL,
        unknown_2_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO Map (
        id,
        internal_name,
        instance_type,
        battleground,
        map_name_en_gb,
        map_name_ko_kr,
        map_name_fr_fr,
        map_name_de_de,
        map_name_en_cn,
        map_name_en_tw,
        map_name_es_es,
        map_name_es_mx,
        map_name_flags,
        min_level,
        max_level,
        max_players,
        unknown_0,
        unknown_1,
        unknown_2,
        area_table,
        map_description_horde_en_gb,
        map_description_horde_ko_kr,
        map_description_horde_fr_fr,
        map_description_horde_de_de,
        map_description_horde_en_cn,
        map_description_horde_en_tw,
        map_description_horde_es_es,
        map_description_horde_es_mx,
        map_description_horde_flags,
        map_description_alliance_en_gb,
        map_description_alliance_ko_kr,
        map_description_alliance_fr_fr,
        map_description_alliance_de_de,
        map_description_alliance_en_cn,
        map_description_alliance_en_tw,
        map_description_alliance_es_es,
        map_description_alliance_es_mx,
        map_description_alliance_flags,
        loading_screen,
        raid_offset,
        unknown_2_0,
        unknown_2_1
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
        ?42
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Material() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Material (
        id INTEGER PRIMARY KEY NOT NULL,
        flags INTEGER  NOT NULL,
        foley_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Material (
        id,
        flags,
        foley_sound
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NPCSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NPCSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_entries_0 INTEGER NOT NULL,
        sound_entries_1 INTEGER NOT NULL,
        sound_entries_2 INTEGER NOT NULL,
        sound_entries_3 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO NPCSounds (
        id,
        sound_entries_0,
        sound_entries_1,
        sound_entries_2,
        sound_entries_3
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
        race INTEGER  NOT NULL,
        gender INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO NameGen (
        id,
        name,
        race,
        gender
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NamesProfanity() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NamesProfanity (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO NamesProfanity (
        id,
        name
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn NamesReserved() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS NamesReserved (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO NamesReserved (
        id,
        name
        ) VALUES (
        ?1,
        ?2
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Package (
        id,
        icon,
        cost,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn PetLoyalty() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PetLoyalty (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO PetLoyalty (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn PetPersonality() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS PetPersonality (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        threshold_unhappy INTEGER  NOT NULL,
        threshold_content INTEGER  NOT NULL,
        threshold_happy INTEGER  NOT NULL,
        damage_unhappy REAL  NOT NULL,
        damage_content REAL  NOT NULL,
        damage_happy REAL  NOT NULL,
        modifier_unhappy REAL  NOT NULL,
        modifier_content REAL  NOT NULL,
        modifier_happy REAL  NOT NULL
    );"
    ,
    "INSERT INTO PetPersonality (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        threshold_unhappy,
        threshold_content,
        threshold_happy,
        damage_unhappy,
        damage_content,
        damage_happy,
        modifier_unhappy,
        modifier_content,
        modifier_happy
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
pub(crate) fn QuestInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS QuestInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO QuestInfo (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn QuestSort() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS QuestSort (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO QuestSort (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn Resistances() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Resistances (
        id INTEGER PRIMARY KEY NOT NULL,
        physical_damage INTEGER  NOT NULL,
        fizzle_sound_entry INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Resistances (
        id,
        physical_damage,
        fizzle_sound_entry,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn ServerMessages() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ServerMessages (
        id INTEGER PRIMARY KEY NOT NULL,
        text_en_gb TEXT NOT NULL,
        text_ko_kr TEXT NOT NULL,
        text_fr_fr TEXT NOT NULL,
        text_de_de TEXT NOT NULL,
        text_en_cn TEXT NOT NULL,
        text_en_tw TEXT NOT NULL,
        text_es_es TEXT NOT NULL,
        text_es_mx TEXT NOT NULL,
        text_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO ServerMessages (
        id,
        text_en_gb,
        text_ko_kr,
        text_fr_fr,
        text_de_de,
        text_en_cn,
        text_en_tw,
        text_es_es,
        text_es_mx,
        text_flags
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
pub(crate) fn SheatheSoundLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SheatheSoundLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        item_class INTEGER  NOT NULL,
        item_subclass INTEGER  NOT NULL,
        item_env_types INTEGER  NOT NULL,
        not_shield INTEGER  NOT NULL,
        sheathe_sound INTEGER  NOT NULL,
        draw_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SheatheSoundLookups (
        id,
        item_class,
        item_subclass,
        item_env_types,
        not_shield,
        sheathe_sound,
        draw_sound
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
pub(crate) fn SkillCostsData() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillCostsData (
        id INTEGER PRIMARY KEY NOT NULL,
        skill_costs INTEGER  NOT NULL,
        cost_0 INTEGER NOT NULL,
        cost_1 INTEGER NOT NULL,
        cost_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SkillCostsData (
        id,
        skill_costs,
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
pub(crate) fn SkillLine() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillLine (
        id INTEGER PRIMARY KEY NOT NULL,
        category INTEGER  NOT NULL,
        skill_costs INTEGER  NOT NULL,
        display_name_en_gb TEXT NOT NULL,
        display_name_ko_kr TEXT NOT NULL,
        display_name_fr_fr TEXT NOT NULL,
        display_name_de_de TEXT NOT NULL,
        display_name_en_cn TEXT NOT NULL,
        display_name_en_tw TEXT NOT NULL,
        display_name_es_es TEXT NOT NULL,
        display_name_es_mx TEXT NOT NULL,
        display_name_flags TEXT NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags TEXT NOT NULL,
        spell_icon INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SkillLine (
        id,
        category,
        skill_costs,
        display_name_en_gb,
        display_name_ko_kr,
        display_name_fr_fr,
        display_name_de_de,
        display_name_en_cn,
        display_name_en_tw,
        display_name_es_es,
        display_name_es_mx,
        display_name_flags,
        description_en_gb,
        description_ko_kr,
        description_fr_fr,
        description_de_de,
        description_en_cn,
        description_en_tw,
        description_es_es,
        description_es_mx,
        description_flags,
        spell_icon
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
        superseded_by INTEGER  NOT NULL,
        acquire_method INTEGER  NOT NULL,
        trivial_skill_line_rank_high INTEGER  NOT NULL,
        trivial_skill_line_rank_low INTEGER  NOT NULL,
        character_points_0 INTEGER NOT NULL,
        character_points_1 INTEGER NOT NULL,
        num_skills_up INTEGER  NOT NULL,
        unknown_padding INTEGER  NOT NULL
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
        superseded_by,
        acquire_method,
        trivial_skill_line_rank_high,
        trivial_skill_line_rank_low,
        character_points_0,
        character_points_1,
        num_skills_up,
        unknown_padding
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
pub(crate) fn SkillLineCategory() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillLineCategory (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        sort_index INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SkillLineCategory (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
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
        ?11
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SkillRaceClassInfo() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SkillRaceClassInfo (
        id INTEGER PRIMARY KEY NOT NULL,
        skill_line INTEGER  NOT NULL,
        race_mask INTEGER  NOT NULL,
        class_mask INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        min_level INTEGER  NOT NULL,
        skill_tier INTEGER  NOT NULL,
        skill_cost INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SkillRaceClassInfo (
        id,
        skill_line,
        race_mask,
        class_mask,
        flags,
        min_level,
        skill_tier,
        skill_cost
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
pub(crate) fn SoundAmbience() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundAmbience (
        id INTEGER PRIMARY KEY NOT NULL,
        day_sound INTEGER  NOT NULL,
        night_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundAmbience (
        id,
        day_sound,
        night_sound
        ) VALUES (
        ?1,
        ?2,
        ?3
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SoundCharacterMacroLines() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundCharacterMacroLines (
        id INTEGER PRIMARY KEY NOT NULL,
        unknown INTEGER  NOT NULL,
        gender INTEGER  NOT NULL,
        race INTEGER  NOT NULL,
        sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundCharacterMacroLines (
        id,
        unknown,
        gender,
        race,
        sound
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
pub(crate) fn SoundEntries() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundEntries (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_type INTEGER  NOT NULL,
        name TEXT  NOT NULL,
        files_0 TEXT NOT NULL,
        files_1 TEXT NOT NULL,
        files_2 TEXT NOT NULL,
        files_3 TEXT NOT NULL,
        files_4 TEXT NOT NULL,
        files_5 TEXT NOT NULL,
        files_6 TEXT NOT NULL,
        files_7 TEXT NOT NULL,
        files_8 TEXT NOT NULL,
        files_9 TEXT NOT NULL,
        frequency_0 INTEGER NOT NULL,
        frequency_1 INTEGER NOT NULL,
        frequency_2 INTEGER NOT NULL,
        frequency_3 INTEGER NOT NULL,
        frequency_4 INTEGER NOT NULL,
        frequency_5 INTEGER NOT NULL,
        frequency_6 INTEGER NOT NULL,
        frequency_7 INTEGER NOT NULL,
        frequency_8 INTEGER NOT NULL,
        frequency_9 INTEGER NOT NULL,
        directory_base TEXT  NOT NULL,
        volume REAL  NOT NULL,
        flags INTEGER  NOT NULL,
        min_distance REAL  NOT NULL,
        distance_cutoff REAL  NOT NULL,
        sound_entries_advanced INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundEntries (
        id,
        sound_type,
        name,
        files_0,
        files_1,
        files_2,
        files_3,
        files_4,
        files_5,
        files_6,
        files_7,
        files_8,
        files_9,
        frequency_0,
        frequency_1,
        frequency_2,
        frequency_3,
        frequency_4,
        frequency_5,
        frequency_6,
        frequency_7,
        frequency_8,
        frequency_9,
        directory_base,
        volume,
        flags,
        min_distance,
        distance_cutoff,
        sound_entries_advanced
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
pub(crate) fn SoundProviderPreferences() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundProviderPreferences (
        id INTEGER PRIMARY KEY NOT NULL,
        description TEXT  NOT NULL,
        flags INTEGER  NOT NULL,
        eax_environment_selection INTEGER  NOT NULL,
        eax_decay_time REAL  NOT NULL,
        eax2_environment_size REAL  NOT NULL,
        eax_environment_diffusion REAL  NOT NULL,
        eax2_room INTEGER  NOT NULL,
        eax2_room_hf INTEGER  NOT NULL,
        eax2_decay_hf_ratio REAL  NOT NULL,
        eax2_reflections INTEGER  NOT NULL,
        eax2_reflections_delay REAL  NOT NULL,
        eax2_reverb INTEGER  NOT NULL,
        eax2_reverb_delay REAL  NOT NULL,
        eax2_room_rolloff REAL  NOT NULL,
        eax2_air_absorption REAL  NOT NULL,
        eax3_room_lf INTEGER  NOT NULL,
        eax3_delay_lf_ratio REAL  NOT NULL,
        eax3_echo_time REAL  NOT NULL,
        eax3_echo_depth REAL  NOT NULL,
        eax3_modulation_time REAL  NOT NULL,
        eax3_modulation_depth REAL  NOT NULL,
        eax3_hf_reference REAL  NOT NULL,
        eax3_lf_reference REAL  NOT NULL
    );"
    ,
    "INSERT INTO SoundProviderPreferences (
        id,
        description,
        flags,
        eax_environment_selection,
        eax_decay_time,
        eax2_environment_size,
        eax_environment_diffusion,
        eax2_room,
        eax2_room_hf,
        eax2_decay_hf_ratio,
        eax2_reflections,
        eax2_reflections_delay,
        eax2_reverb,
        eax2_reverb_delay,
        eax2_room_rolloff,
        eax2_air_absorption,
        eax3_room_lf,
        eax3_delay_lf_ratio,
        eax3_echo_time,
        eax3_echo_depth,
        eax3_modulation_time,
        eax3_modulation_depth,
        eax3_hf_reference,
        eax3_lf_reference
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
pub(crate) fn SoundSamplePreferences() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundSamplePreferences (
        id INTEGER PRIMARY KEY NOT NULL,
        unknown_0 INTEGER NOT NULL,
        unknown_1 INTEGER NOT NULL,
        unknown_2 INTEGER NOT NULL,
        unknown_3 INTEGER NOT NULL,
        unknown_4 INTEGER NOT NULL,
        unknown_5 INTEGER NOT NULL,
        unknown_6 INTEGER NOT NULL,
        unknown_7 INTEGER NOT NULL,
        unknown_8 INTEGER NOT NULL,
        unknown_9 INTEGER NOT NULL,
        unknown_10 INTEGER NOT NULL,
        unknown_11 INTEGER NOT NULL,
        unknown_12 INTEGER NOT NULL,
        unknown_13 INTEGER NOT NULL,
        unknown_14 INTEGER NOT NULL,
        unknown_15 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO SoundSamplePreferences (
        id,
        unknown_0,
        unknown_1,
        unknown_2,
        unknown_3,
        unknown_4,
        unknown_5,
        unknown_6,
        unknown_7,
        unknown_8,
        unknown_9,
        unknown_10,
        unknown_11,
        unknown_12,
        unknown_13,
        unknown_14,
        unknown_15
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
pub(crate) fn SoundWaterType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SoundWaterType (
        id INTEGER PRIMARY KEY NOT NULL,
        liquid_type INTEGER  NOT NULL,
        fluid_speed INTEGER  NOT NULL,
        sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SoundWaterType (
        id,
        liquid_type,
        fluid_speed,
        sound
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
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
pub(crate) fn Spell() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Spell (
        id INTEGER PRIMARY KEY NOT NULL,
        school INTEGER  NOT NULL,
        category INTEGER  NOT NULL,
        cast_ui INTEGER  NOT NULL,
        dispel_type INTEGER  NOT NULL,
        mechanic INTEGER  NOT NULL,
        attributes INTEGER  NOT NULL,
        attributes_ex1 INTEGER  NOT NULL,
        attributes_ex2 INTEGER  NOT NULL,
        attributes_ex3 INTEGER  NOT NULL,
        attributes_ex4 INTEGER  NOT NULL,
        shapeshift_mask INTEGER  NOT NULL,
        shapeshift_exclude INTEGER  NOT NULL,
        targets INTEGER  NOT NULL,
        target_creature_type INTEGER  NOT NULL,
        requires_spell_focus INTEGER  NOT NULL,
        caster_aura_state INTEGER  NOT NULL,
        target_aura_state INTEGER  NOT NULL,
        casting_time_index INTEGER  NOT NULL,
        recovery_time INTEGER  NOT NULL,
        category_recovery_time INTEGER  NOT NULL,
        aura_interrupt_flags INTEGER  NOT NULL,
        channel_interrupt_flags INTEGER  NOT NULL,
        proc_type_mask INTEGER  NOT NULL,
        proc_chance INTEGER  NOT NULL,
        proc_charges INTEGER  NOT NULL,
        max_level INTEGER  NOT NULL,
        base_level INTEGER  NOT NULL,
        spell_level INTEGER  NOT NULL,
        duration INTEGER  NOT NULL,
        power_type INTEGER  NOT NULL,
        mana_cost INTEGER  NOT NULL,
        mana_cost_per_level INTEGER  NOT NULL,
        mana_cost_per_second INTEGER  NOT NULL,
        mana_cost_per_second_per_level INTEGER  NOT NULL,
        range INTEGER  NOT NULL,
        speed REAL  NOT NULL,
        modal_next_spell INTEGER  NOT NULL,
        stack_amount INTEGER  NOT NULL,
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
        equipped_item_inventory_type INTEGER  NOT NULL,
        effect_0 INTEGER NOT NULL,
        effect_1 INTEGER NOT NULL,
        effect_2 INTEGER NOT NULL,
        effect_die_sides_0 INTEGER NOT NULL,
        effect_die_sides_1 INTEGER NOT NULL,
        effect_die_sides_2 INTEGER NOT NULL,
        effect_base_dice_0 INTEGER NOT NULL,
        effect_base_dice_1 INTEGER NOT NULL,
        effect_base_dice_2 INTEGER NOT NULL,
        effect_dice_per_level_0 REAL NOT NULL,
        effect_dice_per_level_1 REAL NOT NULL,
        effect_dice_per_level_2 REAL NOT NULL,
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
        effect_radius_0 INTEGER NOT NULL,
        effect_radius_1 INTEGER NOT NULL,
        effect_radius_2 INTEGER NOT NULL,
        effect_aura_0 INTEGER NOT NULL,
        effect_aura_1 INTEGER NOT NULL,
        effect_aura_2 INTEGER NOT NULL,
        effect_amplitude_0 REAL NOT NULL,
        effect_amplitude_1 REAL NOT NULL,
        effect_amplitude_2 REAL NOT NULL,
        effect_multiple_values_0 REAL NOT NULL,
        effect_multiple_values_1 REAL NOT NULL,
        effect_multiple_values_2 REAL NOT NULL,
        effect_chain_target_0 INTEGER NOT NULL,
        effect_chain_target_1 INTEGER NOT NULL,
        effect_chain_target_2 INTEGER NOT NULL,
        effect_item_type_0 INTEGER NOT NULL,
        effect_item_type_1 INTEGER NOT NULL,
        effect_item_type_2 INTEGER NOT NULL,
        effect_misc_value_0 INTEGER NOT NULL,
        effect_misc_value_1 INTEGER NOT NULL,
        effect_misc_value_2 INTEGER NOT NULL,
        effect_trigger_spell_0 INTEGER NOT NULL,
        effect_trigger_spell_1 INTEGER NOT NULL,
        effect_trigger_spell_2 INTEGER NOT NULL,
        effect_points_per_combo_0 REAL NOT NULL,
        effect_points_per_combo_1 REAL NOT NULL,
        effect_points_per_combo_2 REAL NOT NULL,
        spell_visual_0 INTEGER NOT NULL,
        spell_visual_1 INTEGER NOT NULL,
        spell_icon INTEGER  NOT NULL,
        active_icon INTEGER  NOT NULL,
        spell_priority INTEGER  NOT NULL,
        unknown_flag INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        name_subtext_en_gb TEXT NOT NULL,
        name_subtext_ko_kr TEXT NOT NULL,
        name_subtext_fr_fr TEXT NOT NULL,
        name_subtext_de_de TEXT NOT NULL,
        name_subtext_en_cn TEXT NOT NULL,
        name_subtext_en_tw TEXT NOT NULL,
        name_subtext_es_es TEXT NOT NULL,
        name_subtext_es_mx TEXT NOT NULL,
        name_subtext_flags TEXT NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags TEXT NOT NULL,
        aura_description_en_gb TEXT NOT NULL,
        aura_description_ko_kr TEXT NOT NULL,
        aura_description_fr_fr TEXT NOT NULL,
        aura_description_de_de TEXT NOT NULL,
        aura_description_en_cn TEXT NOT NULL,
        aura_description_en_tw TEXT NOT NULL,
        aura_description_es_es TEXT NOT NULL,
        aura_description_es_mx TEXT NOT NULL,
        aura_description_flags TEXT NOT NULL,
        mana_cost_percent INTEGER  NOT NULL,
        start_recovery_category INTEGER  NOT NULL,
        start_recovery_time INTEGER  NOT NULL,
        max_target_level INTEGER  NOT NULL,
        spell_class_set INTEGER  NOT NULL,
        spell_class_mask_0 INTEGER NOT NULL,
        spell_class_mask_1 INTEGER NOT NULL,
        max_targets INTEGER  NOT NULL,
        defence_type INTEGER  NOT NULL,
        prevention_type INTEGER  NOT NULL,
        stance_bar_order INTEGER  NOT NULL,
        damage_multiplier_0 REAL NOT NULL,
        damage_multiplier_1 REAL NOT NULL,
        damage_multiplier_2 REAL NOT NULL,
        min_faction INTEGER  NOT NULL,
        min_reputation INTEGER  NOT NULL,
        required_aura_vision INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Spell (
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
        equipped_item_inventory_type,
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
        effect_radius_0,
        effect_radius_1,
        effect_radius_2,
        effect_aura_0,
        effect_aura_1,
        effect_aura_2,
        effect_amplitude_0,
        effect_amplitude_1,
        effect_amplitude_2,
        effect_multiple_values_0,
        effect_multiple_values_1,
        effect_multiple_values_2,
        effect_chain_target_0,
        effect_chain_target_1,
        effect_chain_target_2,
        effect_item_type_0,
        effect_item_type_1,
        effect_item_type_2,
        effect_misc_value_0,
        effect_misc_value_1,
        effect_misc_value_2,
        effect_trigger_spell_0,
        effect_trigger_spell_1,
        effect_trigger_spell_2,
        effect_points_per_combo_0,
        effect_points_per_combo_1,
        effect_points_per_combo_2,
        spell_visual_0,
        spell_visual_1,
        spell_icon,
        active_icon,
        spell_priority,
        unknown_flag,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        name_subtext_en_gb,
        name_subtext_ko_kr,
        name_subtext_fr_fr,
        name_subtext_de_de,
        name_subtext_en_cn,
        name_subtext_en_tw,
        name_subtext_es_es,
        name_subtext_es_mx,
        name_subtext_flags,
        description_en_gb,
        description_ko_kr,
        description_fr_fr,
        description_de_de,
        description_en_cn,
        description_en_tw,
        description_es_es,
        description_es_mx,
        description_flags,
        aura_description_en_gb,
        aura_description_ko_kr,
        aura_description_fr_fr,
        aura_description_de_de,
        aura_description_en_cn,
        aura_description_en_tw,
        aura_description_es_es,
        aura_description_es_mx,
        aura_description_flags,
        mana_cost_percent,
        start_recovery_category,
        start_recovery_time,
        max_target_level,
        spell_class_set,
        spell_class_mask_0,
        spell_class_mask_1,
        max_targets,
        defence_type,
        prevention_type,
        stance_bar_order,
        damage_multiplier_0,
        damage_multiplier_1,
        damage_multiplier_2,
        min_faction,
        min_reputation,
        required_aura_vision
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
        ?173
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellAuraNames() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellAuraNames (
        id INTEGER PRIMARY KEY NOT NULL,
        unknown INTEGER  NOT NULL,
        internal_name TEXT  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellAuraNames (
        id,
        unknown,
        internal_name,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn SpellCastTimes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellCastTimes (
        id INTEGER PRIMARY KEY NOT NULL,
        base INTEGER  NOT NULL,
        per_level_increase INTEGER  NOT NULL,
        minimum INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellCastTimes (
        id,
        base,
        per_level_increase,
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
pub(crate) fn SpellChainEffects() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellChainEffects (
        id INTEGER PRIMARY KEY NOT NULL,
        average_seg_len REAL  NOT NULL,
        width REAL  NOT NULL,
        noise_scale REAL  NOT NULL,
        tex_coord_scale REAL  NOT NULL,
        seg_duration INTEGER  NOT NULL,
        seg_delay INTEGER  NOT NULL,
        texture TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellChainEffects (
        id,
        average_seg_len,
        width,
        noise_scale,
        tex_coord_scale,
        seg_duration,
        seg_delay,
        texture
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
pub(crate) fn SpellDispelType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellDispelType (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        mask INTEGER  NOT NULL,
        immunity_possible INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellDispelType (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        mask,
        immunity_possible
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
pub(crate) fn SpellEffectNames() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellEffectNames (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellEffectNames (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn SpellFocusObject() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellFocusObject (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellFocusObject (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn SpellIcon() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellIcon (
        id INTEGER PRIMARY KEY NOT NULL,
        texture_file TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellIcon (
        id,
        texture_file
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
        enchantment_type_0 INTEGER NOT NULL,
        enchantment_type_1 INTEGER NOT NULL,
        enchantment_type_2 INTEGER NOT NULL,
        effect_points_min_0 INTEGER NOT NULL,
        effect_points_min_1 INTEGER NOT NULL,
        effect_points_min_2 INTEGER NOT NULL,
        effect_points_max_0 INTEGER NOT NULL,
        effect_points_max_1 INTEGER NOT NULL,
        effect_points_max_2 INTEGER NOT NULL,
        effect_arg_0 INTEGER NOT NULL,
        effect_arg_1 INTEGER NOT NULL,
        effect_arg_2 INTEGER NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        item_visual INTEGER  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellItemEnchantment (
        id,
        enchantment_type_0,
        enchantment_type_1,
        enchantment_type_2,
        effect_points_min_0,
        effect_points_min_1,
        effect_points_min_2,
        effect_points_max_0,
        effect_points_max_1,
        effect_points_max_2,
        effect_arg_0,
        effect_arg_1,
        effect_arg_2,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        item_visual,
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
        ?24
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn SpellMechanic() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellMechanic (
        id INTEGER PRIMARY KEY NOT NULL,
        state_name_en_gb TEXT NOT NULL,
        state_name_ko_kr TEXT NOT NULL,
        state_name_fr_fr TEXT NOT NULL,
        state_name_de_de TEXT NOT NULL,
        state_name_en_cn TEXT NOT NULL,
        state_name_en_tw TEXT NOT NULL,
        state_name_es_es TEXT NOT NULL,
        state_name_es_mx TEXT NOT NULL,
        state_name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellMechanic (
        id,
        state_name_en_gb,
        state_name_ko_kr,
        state_name_fr_fr,
        state_name_de_de,
        state_name_en_cn,
        state_name_en_tw,
        state_name_es_es,
        state_name_es_mx,
        state_name_flags
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
pub(crate) fn SpellRange() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellRange (
        id INTEGER PRIMARY KEY NOT NULL,
        range_min REAL  NOT NULL,
        range_max REAL  NOT NULL,
        flags INTEGER  NOT NULL,
        display_name_en_gb TEXT NOT NULL,
        display_name_ko_kr TEXT NOT NULL,
        display_name_fr_fr TEXT NOT NULL,
        display_name_de_de TEXT NOT NULL,
        display_name_en_cn TEXT NOT NULL,
        display_name_en_tw TEXT NOT NULL,
        display_name_es_es TEXT NOT NULL,
        display_name_es_mx TEXT NOT NULL,
        display_name_flags TEXT NOT NULL,
        display_name_short_en_gb TEXT NOT NULL,
        display_name_short_ko_kr TEXT NOT NULL,
        display_name_short_fr_fr TEXT NOT NULL,
        display_name_short_de_de TEXT NOT NULL,
        display_name_short_en_cn TEXT NOT NULL,
        display_name_short_en_tw TEXT NOT NULL,
        display_name_short_es_es TEXT NOT NULL,
        display_name_short_es_mx TEXT NOT NULL,
        display_name_short_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO SpellRange (
        id,
        range_min,
        range_max,
        flags,
        display_name_en_gb,
        display_name_ko_kr,
        display_name_fr_fr,
        display_name_de_de,
        display_name_en_cn,
        display_name_en_tw,
        display_name_es_es,
        display_name_es_mx,
        display_name_flags,
        display_name_short_en_gb,
        display_name_short_ko_kr,
        display_name_short_fr_fr,
        display_name_short_de_de,
        display_name_short_en_cn,
        display_name_short_en_tw,
        display_name_short_es_es,
        display_name_short_es_mx,
        display_name_short_flags
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
pub(crate) fn SpellShapeshiftForm() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellShapeshiftForm (
        id INTEGER PRIMARY KEY NOT NULL,
        bonus_action_bar INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        flags INTEGER  NOT NULL,
        creature_type INTEGER  NOT NULL,
        spell_icon INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellShapeshiftForm (
        id,
        bonus_action_bar,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        flags,
        creature_type,
        spell_icon
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
        anim_event_sound INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        caster_impact_kit INTEGER  NOT NULL,
        target_impact_kit INTEGER  NOT NULL
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
        anim_event_sound,
        flags,
        caster_impact_kit,
        target_impact_kit
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
pub(crate) fn SpellVisualEffectName() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualEffectName (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        filename TEXT  NOT NULL,
        area_effect_size REAL  NOT NULL,
        scale REAL  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualEffectName (
        id,
        name,
        filename,
        area_effect_size,
        scale
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
pub(crate) fn SpellVisualKit() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualKit (
        id INTEGER PRIMARY KEY NOT NULL,
        start_anim INTEGER  NOT NULL,
        anim INTEGER  NOT NULL,
        head_effect INTEGER  NOT NULL,
        chest_effect INTEGER  NOT NULL,
        base_effect INTEGER  NOT NULL,
        left_hand_effect INTEGER  NOT NULL,
        right_hand_effect INTEGER  NOT NULL,
        breath_effect INTEGER  NOT NULL,
        left_weapon_effect INTEGER  NOT NULL,
        right_weapon_effect INTEGER  NOT NULL,
        special_effects_0 INTEGER NOT NULL,
        special_effects_1 INTEGER NOT NULL,
        special_effects_2 INTEGER NOT NULL,
        world_effect INTEGER  NOT NULL,
        sound INTEGER  NOT NULL,
        shake INTEGER  NOT NULL,
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
        unknown1_pad INTEGER  NOT NULL,
        unknown2_pad INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualKit (
        id,
        start_anim,
        anim,
        head_effect,
        chest_effect,
        base_effect,
        left_hand_effect,
        right_hand_effect,
        breath_effect,
        left_weapon_effect,
        right_weapon_effect,
        special_effects_0,
        special_effects_1,
        special_effects_2,
        world_effect,
        sound,
        shake,
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
        unknown1_pad,
        unknown2_pad
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
pub(crate) fn SpellVisualPrecastTransitions() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS SpellVisualPrecastTransitions (
        id INTEGER PRIMARY KEY NOT NULL,
        load_animation TEXT  NOT NULL,
        hold_animation TEXT  NOT NULL
    );"
    ,
    "INSERT INTO SpellVisualPrecastTransitions (
        id,
        load_animation,
        hold_animation
        ) VALUES (
        ?1,
        ?2,
        ?3
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
pub(crate) fn Startup_Strings() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Startup_Strings (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        startup_string_en_gb TEXT NOT NULL,
        startup_string_ko_kr TEXT NOT NULL,
        startup_string_fr_fr TEXT NOT NULL,
        startup_string_de_de TEXT NOT NULL,
        startup_string_en_cn TEXT NOT NULL,
        startup_string_en_tw TEXT NOT NULL,
        startup_string_es_es TEXT NOT NULL,
        startup_string_es_mx TEXT NOT NULL,
        startup_string_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO Startup_Strings (
        id,
        name,
        startup_string_en_gb,
        startup_string_ko_kr,
        startup_string_fr_fr,
        startup_string_de_de,
        startup_string_en_cn,
        startup_string_en_tw,
        startup_string_es_es,
        startup_string_es_mx,
        startup_string_flags
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
pub(crate) fn Stationery() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Stationery (
        id INTEGER PRIMARY KEY NOT NULL,
        item INTEGER  NOT NULL,
        texture TEXT  NOT NULL,
        flags INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Stationery (
        id,
        item,
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
pub(crate) fn StringLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS StringLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        path TEXT  NOT NULL
    );"
    ,
    "INSERT INTO StringLookups (
        id,
        path
        ) VALUES (
        ?1,
        ?2
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn Talent() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS Talent (
        id INTEGER PRIMARY KEY NOT NULL,
        tab INTEGER  NOT NULL,
        tier INTEGER  NOT NULL,
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
        prereq_talents_0 INTEGER NOT NULL,
        prereq_talents_1 INTEGER NOT NULL,
        prereq_talents_2 INTEGER NOT NULL,
        prereq_ranks_0 INTEGER NOT NULL,
        prereq_ranks_1 INTEGER NOT NULL,
        prereq_ranks_2 INTEGER NOT NULL,
        flags INTEGER  NOT NULL,
        required_spell INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO Talent (
        id,
        tab,
        tier,
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
        prereq_talents_0,
        prereq_talents_1,
        prereq_talents_2,
        prereq_ranks_0,
        prereq_ranks_1,
        prereq_ranks_2,
        flags,
        required_spell
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
pub(crate) fn TalentTab() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TalentTab (
        id INTEGER PRIMARY KEY NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        spell_icon INTEGER  NOT NULL,
        race_mask INTEGER  NOT NULL,
        class_mask INTEGER  NOT NULL,
        order_index INTEGER  NOT NULL,
        background_file TEXT  NOT NULL
    );"
    ,
    "INSERT INTO TalentTab (
        id,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        spell_icon,
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
        ?15
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn TaxiNodes() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TaxiNodes (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL,
        mount_creature_display_info_0 INTEGER NOT NULL,
        mount_creature_display_info_1 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO TaxiNodes (
        id,
        map,
        location_x,
        location_y,
        location_z,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags,
        mount_creature_display_info_0,
        mount_creature_display_info_1
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
pub(crate) fn TaxiPath() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TaxiPath (
        id INTEGER PRIMARY KEY NOT NULL,
        source_taxi_node INTEGER  NOT NULL,
        destination_taxi_node INTEGER  NOT NULL,
        cost INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TaxiPath (
        id,
        source_taxi_node,
        destination_taxi_node,
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
pub(crate) fn TaxiPathNode() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TaxiPathNode (
        id INTEGER PRIMARY KEY NOT NULL,
        taxi_path INTEGER  NOT NULL,
        node_index INTEGER  NOT NULL,
        map INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        flags INTEGER  NOT NULL,
        delay INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TaxiPathNode (
        id,
        taxi_path,
        node_index,
        map,
        location_x,
        location_y,
        location_z,
        flags,
        delay
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
pub(crate) fn TerrainType() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TerrainType (
        id INTEGER PRIMARY KEY NOT NULL,
        description TEXT  NOT NULL,
        footstep_spray_run INTEGER  NOT NULL,
        footstep_spray_walk INTEGER  NOT NULL,
        sound INTEGER  NOT NULL,
        display_footsteps INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TerrainType (
        id,
        description,
        footstep_spray_run,
        footstep_spray_walk,
        sound,
        display_footsteps
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
pub(crate) fn TransportAnimation() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS TransportAnimation (
        id INTEGER PRIMARY KEY NOT NULL,
        transport INTEGER  NOT NULL,
        time_index INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        sequence INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO TransportAnimation (
        id,
        transport,
        time_index,
        location_x,
        location_y,
        location_z,
        sequence
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
pub(crate) fn UISoundLookups() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS UISoundLookups (
        id INTEGER PRIMARY KEY NOT NULL,
        sound_entry INTEGER  NOT NULL,
        internal_name TEXT  NOT NULL
    );"
    ,
    "INSERT INTO UISoundLookups (
        id,
        sound_entry,
        internal_name
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
        combat_blood_spurt_front_small INTEGER  NOT NULL,
        combat_blood_spurt_front_large INTEGER  NOT NULL,
        combat_blood_spurt_back_small INTEGER  NOT NULL,
        combat_blood_spurt_back_large INTEGER  NOT NULL,
        texture_0 TEXT NOT NULL,
        texture_1 TEXT NOT NULL,
        texture_2 TEXT NOT NULL,
        texture_3 TEXT NOT NULL,
        texture_4 TEXT NOT NULL
    );"
    ,
    "INSERT INTO UnitBlood (
        id,
        combat_blood_spurt_front_small,
        combat_blood_spurt_front_large,
        combat_blood_spurt_back_small,
        combat_blood_spurt_back_large,
        texture_0,
        texture_1,
        texture_2,
        texture_3,
        texture_4
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
pub(crate) fn UnitBloodLevels() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS UnitBloodLevels (
        id INTEGER PRIMARY KEY NOT NULL,
        violence_level_0 INTEGER NOT NULL,
        violence_level_1 INTEGER NOT NULL,
        violence_level_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO UnitBloodLevels (
        id,
        violence_level_0,
        violence_level_1,
        violence_level_2
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
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
        multisample INTEGER  NOT NULL
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
        multisample
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
pub(crate) fn VocalUISounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS VocalUISounds (
        id INTEGER PRIMARY KEY NOT NULL,
        vocal_ui_enum INTEGER  NOT NULL,
        race INTEGER  NOT NULL,
        normal_male_sound INTEGER  NOT NULL,
        normal_female_sound INTEGER  NOT NULL,
        pissed_male_sound INTEGER  NOT NULL,
        pissed_female_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO VocalUISounds (
        id,
        vocal_ui_enum,
        race,
        normal_male_sound,
        normal_female_sound,
        pissed_male_sound,
        pissed_female_sound
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
pub(crate) fn WMOAreaTable() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WMOAreaTable (
        id INTEGER PRIMARY KEY NOT NULL,
        wmo_id INTEGER  NOT NULL,
        name_set_id INTEGER  NOT NULL,
        wmo_group_id INTEGER  NOT NULL,
        sound_provider_preferences INTEGER  NOT NULL,
        sound_provider_preferences_underwater INTEGER  NOT NULL,
        sound_ambience INTEGER  NOT NULL,
        zone_music INTEGER  NOT NULL,
        zone_intro_music INTEGER  NOT NULL,
        flags INTEGER  NOT NULL,
        area_table INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO WMOAreaTable (
        id,
        wmo_id,
        name_set_id,
        wmo_group_id,
        sound_provider_preferences,
        sound_provider_preferences_underwater,
        sound_ambience,
        zone_music,
        zone_intro_music,
        flags,
        area_table,
        name_en_gb,
        name_ko_kr,
        name_fr_fr,
        name_de_de,
        name_en_cn,
        name_en_tw,
        name_es_es,
        name_es_mx,
        name_flags
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
pub(crate) fn WeaponImpactSounds() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WeaponImpactSounds (
        id INTEGER PRIMARY KEY NOT NULL,
        weapon_subclass INTEGER  NOT NULL,
        parry_sound_type INTEGER  NOT NULL,
        impact_sound_0 INTEGER NOT NULL,
        impact_sound_1 INTEGER NOT NULL,
        impact_sound_2 INTEGER NOT NULL,
        impact_sound_3 INTEGER NOT NULL,
        impact_sound_4 INTEGER NOT NULL,
        impact_sound_5 INTEGER NOT NULL,
        impact_sound_6 INTEGER NOT NULL,
        impact_sound_7 INTEGER NOT NULL,
        impact_sound_8 INTEGER NOT NULL,
        impact_sound_9 INTEGER NOT NULL,
        crit_impact_sound_0 INTEGER NOT NULL,
        crit_impact_sound_1 INTEGER NOT NULL,
        crit_impact_sound_2 INTEGER NOT NULL,
        crit_impact_sound_3 INTEGER NOT NULL,
        crit_impact_sound_4 INTEGER NOT NULL,
        crit_impact_sound_5 INTEGER NOT NULL,
        crit_impact_sound_6 INTEGER NOT NULL,
        crit_impact_sound_7 INTEGER NOT NULL,
        crit_impact_sound_8 INTEGER NOT NULL,
        crit_impact_sound_9 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO WeaponImpactSounds (
        id,
        weapon_subclass,
        parry_sound_type,
        impact_sound_0,
        impact_sound_1,
        impact_sound_2,
        impact_sound_3,
        impact_sound_4,
        impact_sound_5,
        impact_sound_6,
        impact_sound_7,
        impact_sound_8,
        impact_sound_9,
        crit_impact_sound_0,
        crit_impact_sound_1,
        crit_impact_sound_2,
        crit_impact_sound_3,
        crit_impact_sound_4,
        crit_impact_sound_5,
        crit_impact_sound_6,
        crit_impact_sound_7,
        crit_impact_sound_8,
        crit_impact_sound_9
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
        critical INTEGER  NOT NULL,
        sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO WeaponSwingSounds2 (
        id,
        swing_type,
        critical,
        sound
        ) VALUES (
        ?1,
        ?2,
        ?3,
        ?4
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WorldMapArea() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapArea (
        id INTEGER PRIMARY KEY NOT NULL,
        world_map_continent INTEGER  NOT NULL,
        area_table INTEGER  NOT NULL,
        area_name TEXT  NOT NULL,
        location_left REAL  NOT NULL,
        location_right REAL  NOT NULL,
        location_top REAL  NOT NULL,
        location_bottom REAL  NOT NULL
    );"
    ,
    "INSERT INTO WorldMapArea (
        id,
        world_map_continent,
        area_table,
        area_name,
        location_left,
        location_right,
        location_top,
        location_bottom
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
pub(crate) fn WorldMapContinent() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapContinent (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        left_boundary INTEGER  NOT NULL,
        right_boundary INTEGER  NOT NULL,
        top_boundary INTEGER  NOT NULL,
        bottom_boundary INTEGER  NOT NULL,
        continent_offset_x REAL  NOT NULL,
        continent_offset_y REAL  NOT NULL,
        scale REAL  NOT NULL,
        taxi_min_x REAL  NOT NULL,
        taxi_min_y REAL  NOT NULL,
        taxi_max_x REAL  NOT NULL,
        taxi_max_y REAL  NOT NULL
    );"
    ,
    "INSERT INTO WorldMapContinent (
        id,
        map,
        left_boundary,
        right_boundary,
        top_boundary,
        bottom_boundary,
        continent_offset_x,
        continent_offset_y,
        scale,
        taxi_min_x,
        taxi_min_y,
        taxi_max_x,
        taxi_max_y
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
pub(crate) fn WorldMapOverlay() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldMapOverlay (
        id INTEGER PRIMARY KEY NOT NULL,
        world_map_area INTEGER  NOT NULL,
        area_table_0 INTEGER NOT NULL,
        area_table_1 INTEGER NOT NULL,
        area_table_2 INTEGER NOT NULL,
        area_table_3 INTEGER NOT NULL,
        location_x INTEGER  NOT NULL,
        location_y INTEGER  NOT NULL,
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
        world_map_area,
        area_table_0,
        area_table_1,
        area_table_2,
        area_table_3,
        location_x,
        location_y,
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
pub(crate) fn WorldSafeLocs() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldSafeLocs (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        location_x REAL  NOT NULL,
        location_y REAL  NOT NULL,
        location_z REAL  NOT NULL,
        area_name_en_gb TEXT NOT NULL,
        area_name_ko_kr TEXT NOT NULL,
        area_name_fr_fr TEXT NOT NULL,
        area_name_de_de TEXT NOT NULL,
        area_name_en_cn TEXT NOT NULL,
        area_name_en_tw TEXT NOT NULL,
        area_name_es_es TEXT NOT NULL,
        area_name_es_mx TEXT NOT NULL,
        area_name_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO WorldSafeLocs (
        id,
        map,
        location_x,
        location_y,
        location_z,
        area_name_en_gb,
        area_name_ko_kr,
        area_name_fr_fr,
        area_name_de_de,
        area_name_en_cn,
        area_name_en_tw,
        area_name_es_es,
        area_name_es_mx,
        area_name_flags
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
pub(crate) fn WorldStateUI() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WorldStateUI (
        id INTEGER PRIMARY KEY NOT NULL,
        map INTEGER  NOT NULL,
        area_table INTEGER  NOT NULL,
        icon TEXT  NOT NULL,
        state_variable_en_gb TEXT NOT NULL,
        state_variable_ko_kr TEXT NOT NULL,
        state_variable_fr_fr TEXT NOT NULL,
        state_variable_de_de TEXT NOT NULL,
        state_variable_en_cn TEXT NOT NULL,
        state_variable_en_tw TEXT NOT NULL,
        state_variable_es_es TEXT NOT NULL,
        state_variable_es_mx TEXT NOT NULL,
        state_variable_flags TEXT NOT NULL,
        tooltip_en_gb TEXT NOT NULL,
        tooltip_ko_kr TEXT NOT NULL,
        tooltip_fr_fr TEXT NOT NULL,
        tooltip_de_de TEXT NOT NULL,
        tooltip_en_cn TEXT NOT NULL,
        tooltip_en_tw TEXT NOT NULL,
        tooltip_es_es TEXT NOT NULL,
        tooltip_es_mx TEXT NOT NULL,
        tooltip_flags TEXT NOT NULL,
        state INTEGER  NOT NULL,
        world_state INTEGER  NOT NULL,
        ty INTEGER  NOT NULL,
        dynamic_icon TEXT  NOT NULL,
        dynamic_tooltip_en_gb TEXT NOT NULL,
        dynamic_tooltip_ko_kr TEXT NOT NULL,
        dynamic_tooltip_fr_fr TEXT NOT NULL,
        dynamic_tooltip_de_de TEXT NOT NULL,
        dynamic_tooltip_en_cn TEXT NOT NULL,
        dynamic_tooltip_en_tw TEXT NOT NULL,
        dynamic_tooltip_es_es TEXT NOT NULL,
        dynamic_tooltip_es_mx TEXT NOT NULL,
        dynamic_tooltip_flags TEXT NOT NULL,
        extended_ui TEXT  NOT NULL,
        unknown_0 INTEGER NOT NULL,
        unknown_1 INTEGER NOT NULL,
        unknown_2 INTEGER NOT NULL
    );"
    ,
    "INSERT INTO WorldStateUI (
        id,
        map,
        area_table,
        icon,
        state_variable_en_gb,
        state_variable_ko_kr,
        state_variable_fr_fr,
        state_variable_de_de,
        state_variable_en_cn,
        state_variable_en_tw,
        state_variable_es_es,
        state_variable_es_mx,
        state_variable_flags,
        tooltip_en_gb,
        tooltip_ko_kr,
        tooltip_fr_fr,
        tooltip_de_de,
        tooltip_en_cn,
        tooltip_en_tw,
        tooltip_es_es,
        tooltip_es_mx,
        tooltip_flags,
        state,
        world_state,
        ty,
        dynamic_icon,
        dynamic_tooltip_en_gb,
        dynamic_tooltip_ko_kr,
        dynamic_tooltip_fr_fr,
        dynamic_tooltip_de_de,
        dynamic_tooltip_en_cn,
        dynamic_tooltip_en_tw,
        dynamic_tooltip_es_es,
        dynamic_tooltip_es_mx,
        dynamic_tooltip_flags,
        extended_ui,
        unknown_0,
        unknown_1,
        unknown_2
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
        ?39
    );"
    )
}

#[allow(non_snake_case)]
pub(crate) fn WowError_Strings() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS WowError_Strings (
        id INTEGER PRIMARY KEY NOT NULL,
        name TEXT  NOT NULL,
        text_en_gb TEXT NOT NULL,
        text_ko_kr TEXT NOT NULL,
        text_fr_fr TEXT NOT NULL,
        text_de_de TEXT NOT NULL,
        text_en_cn TEXT NOT NULL,
        text_en_tw TEXT NOT NULL,
        text_es_es TEXT NOT NULL,
        text_es_mx TEXT NOT NULL,
        text_flags TEXT NOT NULL
    );"
    ,
    "INSERT INTO WowError_Strings (
        id,
        name,
        text_en_gb,
        text_ko_kr,
        text_fr_fr,
        text_de_de,
        text_en_cn,
        text_en_tw,
        text_es_es,
        text_es_mx,
        text_flags
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
        intro_sound INTEGER  NOT NULL,
        priority_over_ambience INTEGER  NOT NULL,
        min_delay INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ZoneIntroMusicTable (
        id,
        name,
        intro_sound,
        priority_over_ambience,
        min_delay
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
pub(crate) fn ZoneMusic() -> (&'static str, &'static str) {
    (
    "CREATE TABLE IF NOT EXISTS ZoneMusic (
        id INTEGER PRIMARY KEY NOT NULL,
        set_name TEXT  NOT NULL,
        silence_interval_min_day INTEGER  NOT NULL,
        silence_interval_min_night INTEGER  NOT NULL,
        silence_interval_max_day INTEGER  NOT NULL,
        silence_interval_max_night INTEGER  NOT NULL,
        day_sound INTEGER  NOT NULL,
        night_sound INTEGER  NOT NULL
    );"
    ,
    "INSERT INTO ZoneMusic (
        id,
        set_name,
        silence_interval_min_day,
        silence_interval_min_night,
        silence_interval_max_day,
        silence_interval_max_night,
        day_sound,
        night_sound
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

