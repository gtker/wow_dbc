use crate::SqliteError;
use rusqlite::{Connection, params};
use wow_dbc::{DbcTable, LocalizedString, ExtendedLocalizedString};
use wow_dbc::vanilla_tables::*;

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
            let (table, insert, _select) = AreaPOI();
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
            let (table, insert, _select) = AreaTable();
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
            let (table, insert, _select) = AreaTrigger();
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
            let (table, insert, _select) = AttackAnimKits();
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
            let (table, insert, _select) = AttackAnimTypes();
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
            let (table, insert, _select) = AuctionHouse();
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
            let (table, insert, _select) = BankBagSlotPrices();
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
            let (table, insert, _select) = Cfg_Configs();
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
            let (table, insert, _select) = CharBaseInfo();
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
            let (table, insert, _select) = CharHairGeosets();
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
            let (table, insert, _select) = CharHairTextures();
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
            let (table, insert, _select) = CharSections();
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
            let (table, insert, _select) = CharStartOutfit();
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
            let (table, insert, _select) = CharVariations();
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
            let (table, insert, _select) = CharacterCreateCameras();
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
            let (table, insert, _select) = CharacterFacialHairStyles();
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
            let (table, insert, _select) = ChatChannels();
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
            let (table, insert, _select) = ChatProfanity();
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
            let (table, insert, _select) = ChrClasses();
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
            let (table, insert, _select) = ChrRaces();
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
            let (table, insert, _select) = CinematicCamera();
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
            let (table, insert, _select) = CinematicSequences();
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
            let (table, insert, _select) = CreatureDisplayInfo();
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
            let (table, insert, _select) = CreatureDisplayInfoExtra();
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
            let (table, insert, _select) = CreatureFamily();
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
            let (table, insert, _select) = CreatureModelData();
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
            let (table, insert, _select) = CreatureSoundData();
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
            let (table, insert, _select) = CreatureSpellData();
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
            let (table, insert, _select) = CreatureType();
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
            let (table, insert, _select) = DeathThudLookups();
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
            let (table, insert, _select) = DurabilityCosts();
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
            let (table, insert, _select) = EmotesText();
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
            let (table, insert, _select) = EmotesTextData();
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
            let (table, insert, _select) = EmotesTextSound();
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
            let (table, insert, _select) = EnvironmentalDamage();
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
            let (table, insert, _select) = Exhaustion();
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
            let (table, insert, _select) = Faction();
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
            let (table, insert, _select) = FactionGroup();
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
            let (table, insert, _select) = FactionTemplate();
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
            let (table, insert, _select) = FootprintTextures();
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
            let (table, insert, _select) = FootstepTerrainLookup();
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
            let (table, insert, _select) = GMSurveyCurrentSurvey();
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
            let (table, insert, _select) = GMSurveyQuestions();
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
            let (table, insert, _select) = GMSurveySurveys();
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
            let (table, insert, _select) = GMTicketCategory();
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
            let (table, insert, _select) = GameTips();
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
            let (table, insert, _select) = GroundEffectDoodad();
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
            let (table, insert, _select) = GroundEffectTexture();
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
            let (table, insert, _select) = ItemClass();
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
            let (table, insert, _select) = ItemGroupSounds();
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
            let (table, insert, _select) = ItemPetFood();
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
            let (table, insert, _select) = ItemRandomProperties();
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
            let (table, insert, _select) = ItemSet();
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
            let (table, insert, _select) = ItemSubClass();
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
            let (table, insert, _select) = ItemSubClassMask();
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
            let (table, insert, _select) = ItemVisualEffects();
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
            let (table, insert, _select) = ItemVisuals();
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
            let (table, insert, _select) = LFGDungeons();
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
            let (table, insert, _select) = LanguageWords();
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
            let (table, insert, _select) = Languages();
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
            let (table, insert, _select) = Light();
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
            let (table, insert, _select) = LightSkybox();
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
            let (table, insert, _select) = LiquidType();
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
            let (table, insert, _select) = LoadingScreenTaxiSplines();
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
            let (table, insert, _select) = LoadingScreens();
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
            let (table, insert, _select) = Lock();
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
            let (table, insert, _select) = LockType();
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
            let (table, insert, _select) = MailTemplate();
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
            let (table, insert, _select) = Map();
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
            let (table, insert, _select) = Material();
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
            let (table, insert, _select) = NPCSounds();
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
            let (table, insert, _select) = NameGen();
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
            let (table, insert, _select) = NamesProfanity();
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
            let (table, insert, _select) = NamesReserved();
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
            let (table, insert, _select) = Package();
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
        "PetLoyalty.dbc" => {
            let data = pet_loyalty::PetLoyalty::read(file_contents)?;
            let (table, insert, _select) = PetLoyalty();
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
            let (table, insert, _select) = PetPersonality();
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
            let (table, insert, _select) = QuestInfo();
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
            let (table, insert, _select) = QuestSort();
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
            let (table, insert, _select) = Resistances();
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
            let (table, insert, _select) = ServerMessages();
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
            let (table, insert, _select) = SheatheSoundLookups();
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
            let (table, insert, _select) = SkillCostsData();
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
            let (table, insert, _select) = SkillLine();
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
            let (table, insert, _select) = SkillLineAbility();
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
            let (table, insert, _select) = SkillLineCategory();
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
            let (table, insert, _select) = SkillRaceClassInfo();
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
                row.day_sound.id,
                row.night_sound.id,
                ])?;
            }
        }
        "SoundCharacterMacroLines.dbc" => {
            let data = sound_character_macro_lines::SoundCharacterMacroLines::read(file_contents)?;
            let (table, insert, _select) = SoundCharacterMacroLines();
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
            let (table, insert, _select) = SoundEntries();
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
            let (table, insert, _select) = SoundProviderPreferences();
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
            let (table, insert, _select) = SoundSamplePreferences();
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
            let (table, insert, _select) = SoundWaterType();
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
                row.interrupt_flags,
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
            let (table, insert, _select) = SpellAuraNames();
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
            let (table, insert, _select) = SpellCastTimes();
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
            let (table, insert, _select) = SpellDispelType();
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
        "SpellEffectNames.dbc" => {
            let data = spell_effect_names::SpellEffectNames::read(file_contents)?;
            let (table, insert, _select) = SpellEffectNames();
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
            let (table, insert, _select) = SpellFocusObject();
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
            let (table, insert, _select) = SpellIcon();
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
            let (table, insert, _select) = SpellItemEnchantment();
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
            let (table, insert, _select) = SpellMechanic();
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
            let (table, insert, _select) = SpellShapeshiftForm();
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
            let (table, insert, _select) = SpellVisual();
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
            let (table, insert, _select) = SpellVisualEffectName();
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
            let (table, insert, _select) = SpellVisualKit();
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
            let (table, insert, _select) = SpellVisualPrecastTransitions();
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
            let (table, insert, _select) = Stationery();
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
            let (table, insert, _select) = StringLookups();
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
            let (table, insert, _select) = Talent();
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
            let (table, insert, _select) = TalentTab();
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
            let (table, insert, _select) = TaxiNodes();
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
            let (table, insert, _select) = TaxiPath();
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
            let (table, insert, _select) = TaxiPathNode();
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
            let (table, insert, _select) = TerrainType();
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
            let (table, insert, _select) = TerrainTypeSounds();
            tx.execute(table, ())?;

            for row in data.rows() {
                tx.execute(insert, params![
                row.id.id,
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
            let (table, insert, _select) = UISoundLookups();
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
            let (table, insert, _select) = UnitBlood();
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
            let (table, insert, _select) = UnitBloodLevels();
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
            let (table, insert, _select) = WMOAreaTable();
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
            let (table, insert, _select) = WeaponImpactSounds();
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
            let (table, insert, _select) = WeaponSwingSounds2();
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
            let (table, insert, _select) = WorldMapArea();
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
            let (table, insert, _select) = WorldMapContinent();
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
            let (table, insert, _select) = WorldMapOverlay();
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
            let (table, insert, _select) = WorldSafeLocs();
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
            let (table, insert, _select) = WorldStateUI();
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
            let (table, insert, _select) = WowError_Strings();
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
            let (table, insert, _select) = ZoneIntroMusicTable();
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
            let (table, insert, _select) = ZoneMusic();
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
pub(crate) fn AnimationData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name,
        weapon_flags,
        body_flags,
        unknown,
        fallback,
        behaviour
    FROM `AnimationData`;"
    )
}


pub(crate) fn animation_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<animation_data::AnimationData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(animation_data::AnimationDataRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            weapon_flags: row.get::<_, i32>(2)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for weapon_flags")))?,
            body_flags: row.get::<_, i32>(3)?.into(),
            unknown: row.get::<_, i32>(4)?.into(),
            fallback: row.get::<_, u32>(5)?.into(),
            behaviour: row.get::<_, u32>(6)?.into(),
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
        name_flags INTEGER NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `AreaPOI`;"
    )
}


pub(crate) fn area_poi_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<area_poi::AreaPOI, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(area_poi::AreaPOIRow {
            id: row.get::<_, u32>(0)?.into(),
            importance: row.get::<_, i32>(1)?.into(),
            icon: row.get::<_, i32>(2)?.into(),
            faction: row.get::<_, u32>(3)?.into(),
            location_x: row.get::<_, f32>(4)?.into(),
            location_y: row.get::<_, f32>(5)?.into(),
            location_z: row.get::<_, f32>(6)?.into(),
            map: row.get::<_, u32>(7)?.into(),
            flags: row.get::<_, i32>(8)?.into(),
            area_table: row.get::<_, u32>(9)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(10)?.into(),
                ko_kr: row.get::<_, String>(11)?.into(),
                fr_fr: row.get::<_, String>(12)?.into(),
                de_de: row.get::<_, String>(13)?.into(),
                en_cn: row.get::<_, String>(14)?.into(),
                en_tw: row.get::<_, String>(15)?.into(),
                es_es: row.get::<_, String>(16)?.into(),
                es_mx: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
            description: LocalizedString {
                en_gb: row.get::<_, String>(19)?.into(),
                ko_kr: row.get::<_, String>(20)?.into(),
                fr_fr: row.get::<_, String>(21)?.into(),
                de_de: row.get::<_, String>(22)?.into(),
                en_cn: row.get::<_, String>(23)?.into(),
                en_tw: row.get::<_, String>(24)?.into(),
                es_es: row.get::<_, String>(25)?.into(),
                es_mx: row.get::<_, String>(26)?.into(),
                flags: row.get::<_, u32>(27)?.into(),
            },
            world_state: row.get::<_, u32>(28)?.into(),
        });
    }
    Ok(area_poi::AreaPOI { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AreaTable() -> (&'static str, &'static str, &'static str) {
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
        area_name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `AreaTable`;"
    )
}


pub(crate) fn area_table_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<area_table::AreaTable, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(area_table::AreaTableRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            parent_area_table: row.get::<_, u32>(2)?.into(),
            area_bit: row.get::<_, i32>(3)?.into(),
            flags: row.get::<_, i32>(4)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for flags")))?,
            sound_preferences: row.get::<_, u32>(5)?.into(),
            sound_preferences_underwater: row.get::<_, u32>(6)?.into(),
            sound_ambience: row.get::<_, u32>(7)?.into(),
            zone_music: row.get::<_, u32>(8)?.into(),
            zone_music_intro: row.get::<_, u32>(9)?.into(),
            exploration_level: row.get::<_, i32>(10)?.into(),
            area_name: LocalizedString {
                en_gb: row.get::<_, String>(11)?.into(),
                ko_kr: row.get::<_, String>(12)?.into(),
                fr_fr: row.get::<_, String>(13)?.into(),
                de_de: row.get::<_, String>(14)?.into(),
                en_cn: row.get::<_, String>(15)?.into(),
                en_tw: row.get::<_, String>(16)?.into(),
                es_es: row.get::<_, String>(17)?.into(),
                es_mx: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
            },
            faction_group: row.get::<_, u32>(20)?.into(),
            liquid_type: row.get::<_, u32>(21)?.into(),
            min_elevation: row.get::<_, i32>(22)?.into(),
            ambient_multiplier: row.get::<_, f32>(23)?.into(),
            light: row.get::<_, u32>(24)?.into(),
        });
    }
    Ok(area_table::AreaTable { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AreaTrigger() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `AreaTrigger`;"
    )
}


pub(crate) fn area_trigger_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<area_trigger::AreaTrigger, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(area_trigger::AreaTriggerRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            location_x: row.get::<_, f32>(2)?.into(),
            location_y: row.get::<_, f32>(3)?.into(),
            location_z: row.get::<_, f32>(4)?.into(),
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
    ,
    "SELECT
        id,
        animation_data,
        attack_anim_type,
        animation_frequency,
        flags
    FROM `AttackAnimKits`;"
    )
}


pub(crate) fn attack_anim_kits_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<attack_anim_kits::AttackAnimKits, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(attack_anim_kits::AttackAnimKitsRow {
            id: row.get::<_, u32>(0)?.into(),
            animation_data: row.get::<_, u32>(1)?.into(),
            attack_anim_type: row.get::<_, u32>(2)?.into(),
            animation_frequency: row.get::<_, u32>(3)?.into(),
            flags: attack_anim_kits::AttackHand::from_int(row.get::<_, i32>(4)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
        });
    }
    Ok(attack_anim_kits::AttackAnimKits { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AttackAnimTypes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name
    FROM `AttackAnimTypes`;"
    )
}


pub(crate) fn attack_anim_types_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<attack_anim_types::AttackAnimTypes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(attack_anim_types::AttackAnimTypesRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(attack_anim_types::AttackAnimTypes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn AuctionHouse() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `AuctionHouse`;"
    )
}


pub(crate) fn auction_house_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<auction_house::AuctionHouse, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(auction_house::AuctionHouseRow {
            id: row.get::<_, u32>(0)?.into(),
            faction: row.get::<_, u32>(1)?.into(),
            deposit_rate: row.get::<_, i32>(2)?.into(),
            consignment_rate: row.get::<_, i32>(3)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                flags: row.get::<_, u32>(12)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
            cost: row.get::<_, i32>(1)?.into(),
        });
    }
    Ok(bank_bag_slot_prices::BankBagSlotPrices { rows: data })
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
            id: row.get::<_, u32>(0)?.into(),
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Cfg_Categories`;"
    )
}


pub(crate) fn cfg_categories_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cfg_categories::Cfg_Categories, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cfg_categories::Cfg_CategoriesRow {
            category: cfg_categories::ServerCategory::from_int(row.get::<_, i32>(0)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            region: cfg_categories::ServerRegion::from_int(row.get::<_, i32>(1)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            name: LocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                flags: row.get::<_, u32>(10)?.into(),
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
    ,
    "SELECT
        id,
        realm_type,
        pvp_allowed,
        roleplaying
    FROM `Cfg_Configs`;"
    )
}


pub(crate) fn cfg_configs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cfg_configs::Cfg_Configs, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cfg_configs::Cfg_ConfigsRow {
            id: row.get::<_, u32>(0)?.into(),
            realm_type: row.get::<_, i32>(1)?.into(),
            pvp_allowed: row.get::<_, i32>(2)?.into(),
            roleplaying: row.get::<_, i32>(3)?.into(),
        });
    }
    Ok(cfg_configs::Cfg_Configs { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharBaseInfo() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        race,
        class
    FROM `CharBaseInfo`;"
    )
}


pub(crate) fn char_base_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_base_info::CharBaseInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_base_info::CharBaseInfoRow {
            race: row.get::<_, u8>(0)?.into(),
            class: row.get::<_, u8>(1)?.into(),
        });
    }
    Ok(char_base_info::CharBaseInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharHairGeosets() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        race,
        gender,
        variation,
        geoset,
        show_scalp
    FROM `CharHairGeosets`;"
    )
}


pub(crate) fn char_hair_geosets_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_hair_geosets::CharHairGeosets, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_hair_geosets::CharHairGeosetsRow {
            id: row.get::<_, u32>(0)?.into(),
            race: row.get::<_, u32>(1)?.into(),
            gender: char_hair_geosets::Gender::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            variation: row.get::<_, u32>(3)?.into(),
            geoset: row.get::<_, i32>(4)?.into(),
            show_scalp: char_hair_geosets::Scalp::from_int(row.get::<_, i32>(5)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
        });
    }
    Ok(char_hair_geosets::CharHairGeosets { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharHairTextures() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        race,
        gender,
        variation,
        geoset,
        show_scalp,
        unknown_padding_0,
        unknown_padding_1
    FROM `CharHairTextures`;"
    )
}


pub(crate) fn char_hair_textures_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_hair_textures::CharHairTextures, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_hair_textures::CharHairTexturesRow {
            id: row.get::<_, u32>(0)?.into(),
            race: row.get::<_, u32>(1)?.into(),
            gender: char_hair_textures::Gender::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            variation: row.get::<_, u32>(3)?.into(),
            geoset: row.get::<_, i32>(4)?.into(),
            show_scalp: row.get::<_, u32>(5)?.into(),
            unknown_padding: [row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(),             ],
        });
    }
    Ok(char_hair_textures::CharHairTextures { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharSections() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CharSections`;"
    )
}


pub(crate) fn char_sections_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_sections::CharSections, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_sections::CharSectionsRow {
            id: row.get::<_, u32>(0)?.into(),
            race: row.get::<_, u32>(1)?.into(),
            gender: char_sections::Gender::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            ty: char_sections::SelectionType::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            variation_index: row.get::<_, i32>(4)?.into(),
            colour_index: row.get::<_, i32>(5)?.into(),
            texture_name: [row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(),             ],
            npc_only: row.get::<_, bool>(9)?.into(),
        });
    }
    Ok(char_sections::CharSections { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharStartOutfit() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CharStartOutfit`;"
    )
}


pub(crate) fn char_start_outfit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_start_outfit::CharStartOutfit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_start_outfit::CharStartOutfitRow {
            id: row.get::<_, u32>(0)?.into(),
            race: row.get::<_, u8>(1)?.into(),
            class: row.get::<_, u8>(2)?.into(),
            gender: char_start_outfit::Gender::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            outfit_id: row.get::<_, i8>(4)?.into(),
            item_id: [row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(),             ],
            display_id: [row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(),             ],
            inv_slot_id: [row.get::<_, i32>(29)?.into(), row.get::<_, i32>(30)?.into(), row.get::<_, i32>(31)?.into(), row.get::<_, i32>(32)?.into(), row.get::<_, i32>(33)?.into(), row.get::<_, i32>(34)?.into(), row.get::<_, i32>(35)?.into(), row.get::<_, i32>(36)?.into(), row.get::<_, i32>(37)?.into(), row.get::<_, i32>(38)?.into(), row.get::<_, i32>(39)?.into(), row.get::<_, i32>(40)?.into(),             ],
        });
    }
    Ok(char_start_outfit::CharStartOutfit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharVariations() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        gender,
        unknown_1,
        mask_0,
        mask_1,
        unknown_2
    FROM `CharVariations`;"
    )
}


pub(crate) fn char_variations_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<char_variations::CharVariations, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(char_variations::CharVariationsRow {
            id: row.get::<_, u32>(0)?.into(),
            gender: char_variations::Gender::from_int(row.get::<_, i32>(1)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            unknown_1: row.get::<_, i32>(2)?.into(),
            mask: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(),             ],
            unknown_2: row.get::<_, i32>(5)?.into(),
        });
    }
    Ok(char_variations::CharVariations { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharacterCreateCameras() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        unknown_0,
        unknown_1,
        unknown_2_0,
        unknown_2_1,
        unknown_2_2
    FROM `CharacterCreateCameras`;"
    )
}


pub(crate) fn character_create_cameras_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<character_create_cameras::CharacterCreateCameras, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(character_create_cameras::CharacterCreateCamerasRow {
            id: row.get::<_, u32>(0)?.into(),
            unknown: [row.get::<_, bool>(1)?.into(), row.get::<_, bool>(2)?.into(),             ],
            unknown_2: [row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(),             ],
        });
    }
    Ok(character_create_cameras::CharacterCreateCameras { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CharacterFacialHairStyles() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        race,
        gender,
        variation_id,
        geoset_0,
        geoset_1,
        geoset_2,
        geoset_3,
        geoset_4,
        geoset_5
    FROM `CharacterFacialHairStyles`;"
    )
}


pub(crate) fn character_facial_hair_styles_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<character_facial_hair_styles::CharacterFacialHairStyles, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(character_facial_hair_styles::CharacterFacialHairStylesRow {
            race: row.get::<_, u32>(0)?.into(),
            gender: character_facial_hair_styles::Gender::from_int(row.get::<_, i32>(1)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            variation_id: row.get::<_, u32>(2)?.into(),
            geoset: [row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(),             ],
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL,
        shortcut_en_gb TEXT NOT NULL,
        shortcut_ko_kr TEXT NOT NULL,
        shortcut_fr_fr TEXT NOT NULL,
        shortcut_de_de TEXT NOT NULL,
        shortcut_en_cn TEXT NOT NULL,
        shortcut_en_tw TEXT NOT NULL,
        shortcut_es_es TEXT NOT NULL,
        shortcut_es_mx TEXT NOT NULL,
        shortcut_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ChatChannels`;"
    )
}


pub(crate) fn chat_channels_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chat_channels::ChatChannels, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chat_channels::ChatChannelsRow {
            id: row.get::<_, u32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for flags")))?,
            faction_group: row.get::<_, u32>(2)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
            },
            shortcut: LocalizedString {
                en_gb: row.get::<_, String>(12)?.into(),
                ko_kr: row.get::<_, String>(13)?.into(),
                fr_fr: row.get::<_, String>(14)?.into(),
                de_de: row.get::<_, String>(15)?.into(),
                en_cn: row.get::<_, String>(16)?.into(),
                en_tw: row.get::<_, String>(17)?.into(),
                es_es: row.get::<_, String>(18)?.into(),
                es_mx: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
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
    ,
    "SELECT
        id,
        text
    FROM `ChatProfanity`;"
    )
}


pub(crate) fn chat_profanity_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chat_profanity::ChatProfanity, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chat_profanity::ChatProfanityRow {
            id: row.get::<_, u32>(0)?.into(),
            text: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(chat_profanity::ChatProfanity { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ChrClasses() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `ChrClasses`;"
    )
}


pub(crate) fn chr_classes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chr_classes::ChrClasses, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chr_classes::ChrClassesRow {
            id: row.get::<_, u32>(0)?.into(),
            player_class: row.get::<_, u32>(1)?.into(),
            damage_bonus_stat: row.get::<_, i32>(2)?.into(),
            power_type: chr_classes::Power::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            pet_name_token: row.get::<_, String>(4)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                flags: row.get::<_, u32>(13)?.into(),
            },
            filename: row.get::<_, String>(14)?.into(),
            class_mask: row.get::<_, i32>(15)?.into(),
            hybrid_class: row.get::<_, bool>(16)?.into(),
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `ChrRaces`;"
    )
}


pub(crate) fn chr_races_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<chr_races::ChrRaces, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(chr_races::ChrRacesRow {
            id: row.get::<_, u32>(0)?.into(),
            flags: row.get::<_, u32>(1)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for flags")))?,
            faction: row.get::<_, u32>(2)?.into(),
            exploration_sound: row.get::<_, u32>(3)?.into(),
            male_display: row.get::<_, u32>(4)?.into(),
            female_display: row.get::<_, u32>(5)?.into(),
            client_prefix: row.get::<_, String>(6)?.into(),
            speed_modifier: row.get::<_, f32>(7)?.into(),
            base_lang: chr_races::Language::from_int(row.get::<_, i32>(8)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            creature_type: row.get::<_, u32>(9)?.into(),
            login_effect: row.get::<_, u32>(10)?.into(),
            unknown1: row.get::<_, i32>(11)?.into(),
            res_sickness_spell: row.get::<_, u32>(12)?.into(),
            splash_sound_entry: row.get::<_, u32>(13)?.into(),
            unknown2: row.get::<_, i32>(14)?.into(),
            client_file_path: row.get::<_, String>(15)?.into(),
            cinematic_sequence: row.get::<_, u32>(16)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(17)?.into(),
                ko_kr: row.get::<_, String>(18)?.into(),
                fr_fr: row.get::<_, String>(19)?.into(),
                de_de: row.get::<_, String>(20)?.into(),
                en_cn: row.get::<_, String>(21)?.into(),
                en_tw: row.get::<_, String>(22)?.into(),
                es_es: row.get::<_, String>(23)?.into(),
                es_mx: row.get::<_, String>(24)?.into(),
                flags: row.get::<_, u32>(25)?.into(),
            },
            facial_hair_customisation: [row.get::<_, String>(26)?.into(), row.get::<_, String>(27)?.into(),             ],
            hair_customisation: row.get::<_, String>(28)?.into(),
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
    ,
    "SELECT
        id,
        model,
        sound_entry,
        location_x,
        location_y,
        location_z,
        rotation
    FROM `CinematicCamera`;"
    )
}


pub(crate) fn cinematic_camera_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cinematic_camera::CinematicCamera, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cinematic_camera::CinematicCameraRow {
            id: row.get::<_, u32>(0)?.into(),
            model: row.get::<_, String>(1)?.into(),
            sound_entry: row.get::<_, u32>(2)?.into(),
            location_x: row.get::<_, f32>(3)?.into(),
            location_y: row.get::<_, f32>(4)?.into(),
            location_z: row.get::<_, f32>(5)?.into(),
            rotation: row.get::<_, f32>(6)?.into(),
        });
    }
    Ok(cinematic_camera::CinematicCamera { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CinematicSequences() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CinematicSequences`;"
    )
}


pub(crate) fn cinematic_sequences_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<cinematic_sequences::CinematicSequences, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(cinematic_sequences::CinematicSequencesRow {
            id: row.get::<_, u32>(0)?.into(),
            sound_entry: row.get::<_, u32>(1)?.into(),
            cinematic_camera_1: [row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(),             ],
        });
    }
    Ok(cinematic_sequences::CinematicSequences { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureDisplayInfo() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CreatureDisplayInfo`;"
    )
}


pub(crate) fn creature_display_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_display_info::CreatureDisplayInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_display_info::CreatureDisplayInfoRow {
            id: row.get::<_, u32>(0)?.into(),
            model: row.get::<_, u32>(1)?.into(),
            sound: row.get::<_, u32>(2)?.into(),
            extended_display_info: row.get::<_, u32>(3)?.into(),
            creature_model_scale: row.get::<_, f32>(4)?.into(),
            creature_model_alpha: row.get::<_, i32>(5)?.into(),
            texture_variation: [row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(),             ],
            size: creature_display_info::SizeClass::from_int(row.get::<_, i32>(9)? as i8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            blood: row.get::<_, u32>(10)?.into(),
            npc_sound: row.get::<_, u32>(11)?.into(),
        });
    }
    Ok(creature_display_info::CreatureDisplayInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureDisplayInfoExtra() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CreatureDisplayInfoExtra`;"
    )
}


pub(crate) fn creature_display_info_extra_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_display_info_extra::CreatureDisplayInfoExtra, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_display_info_extra::CreatureDisplayInfoExtraRow {
            id: row.get::<_, u32>(0)?.into(),
            display_race: row.get::<_, u32>(1)?.into(),
            gender: creature_display_info_extra::Gender::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            skin: row.get::<_, i32>(3)?.into(),
            face: row.get::<_, i32>(4)?.into(),
            hair_style: row.get::<_, i32>(5)?.into(),
            hair_colour: row.get::<_, i32>(6)?.into(),
            facial_hair: row.get::<_, i32>(7)?.into(),
            npc_item_display: [row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(), row.get::<_, u32>(16)?.into(),             ],
            flags: row.get::<_, i32>(17)?.into(),
            bake_name: row.get::<_, String>(18)?.into(),
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `CreatureFamily`;"
    )
}


pub(crate) fn creature_family_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_family::CreatureFamily, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_family::CreatureFamilyRow {
            id: row.get::<_, u32>(0)?.into(),
            min_scale: row.get::<_, f32>(1)?.into(),
            min_scale_level: row.get::<_, i32>(2)?.into(),
            max_scale: row.get::<_, f32>(3)?.into(),
            max_scale_level: row.get::<_, i32>(4)?.into(),
            pet_food_mask: row.get::<_, i32>(5)?.into(),
            pet_talent_type: row.get::<_, i32>(6)?.into(),
            category: row.get::<_, i32>(7)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(8)?.into(),
                ko_kr: row.get::<_, String>(9)?.into(),
                fr_fr: row.get::<_, String>(10)?.into(),
                de_de: row.get::<_, String>(11)?.into(),
                en_cn: row.get::<_, String>(12)?.into(),
                en_tw: row.get::<_, String>(13)?.into(),
                es_es: row.get::<_, String>(14)?.into(),
                es_mx: row.get::<_, String>(15)?.into(),
                flags: row.get::<_, u32>(16)?.into(),
            },
            icon_path: row.get::<_, String>(17)?.into(),
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
    ,
    "SELECT
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
    FROM `CreatureModelData`;"
    )
}


pub(crate) fn creature_model_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_model_data::CreatureModelData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_model_data::CreatureModelDataRow {
            id: row.get::<_, u32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            model_path: row.get::<_, String>(2)?.into(),
            size: creature_model_data::SizeClass::from_int(row.get::<_, i32>(3)? as i8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            model_scale: row.get::<_, f32>(4)?.into(),
            blood: row.get::<_, u32>(5)?.into(),
            footprint_texture: row.get::<_, u32>(6)?.into(),
            footprint_texture_length: row.get::<_, f32>(7)?.into(),
            footprint_texture_width: row.get::<_, f32>(8)?.into(),
            footprint_texture_scale: row.get::<_, f32>(9)?.into(),
            foley_material: row.get::<_, i32>(10)?.into(),
            footstep_shake_size: row.get::<_, i32>(11)?.into(),
            death_thud_shake_size: row.get::<_, i32>(12)?.into(),
            collision_width: row.get::<_, f32>(13)?.into(),
            collision_height: row.get::<_, f32>(14)?.into(),
            mount_height: row.get::<_, f32>(15)?.into(),
        });
    }
    Ok(creature_model_data::CreatureModelData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureSoundData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `CreatureSoundData`;"
    )
}


pub(crate) fn creature_sound_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_sound_data::CreatureSoundData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_sound_data::CreatureSoundDataRow {
            id: row.get::<_, u32>(0)?.into(),
            sound_exertion: row.get::<_, u32>(1)?.into(),
            sound_exertion_critical: row.get::<_, u32>(2)?.into(),
            sound_injury: row.get::<_, u32>(3)?.into(),
            sound_injury_critical: row.get::<_, u32>(4)?.into(),
            sound_injury_crushing_blow: row.get::<_, u32>(5)?.into(),
            sound_death: row.get::<_, u32>(6)?.into(),
            sound_stun: row.get::<_, u32>(7)?.into(),
            sound_stand: row.get::<_, u32>(8)?.into(),
            sound_footstep: row.get::<_, u32>(9)?.into(),
            sound_aggro: row.get::<_, u32>(10)?.into(),
            sound_wing_flap: row.get::<_, u32>(11)?.into(),
            sound_wing_glide: row.get::<_, u32>(12)?.into(),
            sound_alert: row.get::<_, u32>(13)?.into(),
            sound_fidget: row.get::<_, u32>(14)?.into(),
            custom_attack: row.get::<_, u32>(15)?.into(),
            npc_sound: row.get::<_, u32>(16)?.into(),
            loop_sound: row.get::<_, u32>(17)?.into(),
            creature_impact_type: row.get::<_, i32>(18)?.into(),
            sound_jump_start: row.get::<_, u32>(19)?.into(),
            sound_jump_end: row.get::<_, u32>(20)?.into(),
            sound_pet_attack: row.get::<_, u32>(21)?.into(),
            sound_pet_order: row.get::<_, u32>(22)?.into(),
            sound_pet_dismiss: row.get::<_, u32>(23)?.into(),
            fidget_delay_seconds_min: row.get::<_, i32>(24)?.into(),
            fidget_delay_seconds_max: row.get::<_, i32>(25)?.into(),
            birth_sound: row.get::<_, u32>(26)?.into(),
            spell_cast_directed_sound: row.get::<_, u32>(27)?.into(),
            submerge_sound: row.get::<_, u32>(28)?.into(),
            submerged_sound: row.get::<_, u32>(29)?.into(),
        });
    }
    Ok(creature_sound_data::CreatureSoundData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureSpellData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        spell_0,
        spell_1,
        spell_2,
        spell_3,
        cooldown_time_1_0,
        cooldown_time_1_1,
        cooldown_time_1_2,
        cooldown_time_1_3
    FROM `CreatureSpellData`;"
    )
}


pub(crate) fn creature_spell_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_spell_data::CreatureSpellData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_spell_data::CreatureSpellDataRow {
            id: row.get::<_, u32>(0)?.into(),
            spell: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(),             ],
            cooldown_time_1: [row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(),             ],
        });
    }
    Ok(creature_spell_data::CreatureSpellData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn CreatureType() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `CreatureType`;"
    )
}


pub(crate) fn creature_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<creature_type::CreatureType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(creature_type::CreatureTypeRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            flags: row.get::<_, bool>(10)?.into(),
        });
    }
    Ok(creature_type::CreatureType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DeathThudLookups() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        size,
        terrain_type,
        sound_entry,
        sound_entry_water
    FROM `DeathThudLookups`;"
    )
}


pub(crate) fn death_thud_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<death_thud_lookups::DeathThudLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(death_thud_lookups::DeathThudLookupsRow {
            id: row.get::<_, u32>(0)?.into(),
            size: death_thud_lookups::SizeClass::from_int(row.get::<_, i32>(1)? as i8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            terrain_type: row.get::<_, u32>(2)?.into(),
            sound_entry: row.get::<_, u32>(3)?.into(),
            sound_entry_water: row.get::<_, u32>(4)?.into(),
        });
    }
    Ok(death_thud_lookups::DeathThudLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn DurabilityCosts() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `DurabilityCosts`;"
    )
}


pub(crate) fn durability_costs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<durability_costs::DurabilityCosts, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(durability_costs::DurabilityCostsRow {
            id: row.get::<_, u32>(0)?.into(),
            weapon_subclass_cost: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(),             ],
            armour_subclass_cost: [row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(), row.get::<_, i32>(25)?.into(), row.get::<_, i32>(26)?.into(), row.get::<_, i32>(27)?.into(), row.get::<_, i32>(28)?.into(), row.get::<_, i32>(29)?.into(),             ],
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
            id: row.get::<_, u32>(0)?.into(),
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
    ,
    "SELECT
        id,
        emote_slash_command,
        animation_data,
        flags,
        spec_proc,
        emote_spec_proc_param,
        event_sound_entry
    FROM `Emotes`;"
    )
}


pub(crate) fn emotes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes::Emotes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes::EmotesRow {
            id: row.get::<_, u32>(0)?.into(),
            emote_slash_command: row.get::<_, String>(1)?.into(),
            animation_data: row.get::<_, u32>(2)?.into(),
            flags: row.get::<_, i32>(3)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for flags")))?,
            spec_proc: emotes::EmoteSpecProc::from_int(row.get::<_, i32>(4)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            emote_spec_proc_param: row.get::<_, i32>(5)?.into(),
            event_sound_entry: row.get::<_, u32>(6)?.into(),
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
    ,
    "SELECT
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
    FROM `EmotesText`;"
    )
}


pub(crate) fn emotes_text_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes_text::EmotesText, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes_text::EmotesTextRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            emote: row.get::<_, u32>(2)?.into(),
            emote_text_data: [row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(), row.get::<_, u32>(16)?.into(), row.get::<_, u32>(17)?.into(), row.get::<_, u32>(18)?.into(),             ],
        });
    }
    Ok(emotes_text::EmotesText { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn EmotesTextData() -> (&'static str, &'static str, &'static str) {
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
        text_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `EmotesTextData`;"
    )
}


pub(crate) fn emotes_text_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes_text_data::EmotesTextData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes_text_data::EmotesTextDataRow {
            id: row.get::<_, u32>(0)?.into(),
            text: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
    ,
    "SELECT
        id,
        emotes_text,
        race,
        gender,
        sound
    FROM `EmotesTextSound`;"
    )
}


pub(crate) fn emotes_text_sound_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<emotes_text_sound::EmotesTextSound, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(emotes_text_sound::EmotesTextSoundRow {
            id: row.get::<_, u32>(0)?.into(),
            emotes_text: row.get::<_, u32>(1)?.into(),
            race: row.get::<_, u32>(2)?.into(),
            gender: emotes_text_sound::Gender::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            sound: row.get::<_, u32>(4)?.into(),
        });
    }
    Ok(emotes_text_sound::EmotesTextSound { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn EnvironmentalDamage() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        en,
        spell_visual_kit
    FROM `EnvironmentalDamage`;"
    )
}


pub(crate) fn environmental_damage_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<environmental_damage::EnvironmentalDamage, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(environmental_damage::EnvironmentalDamageRow {
            id: row.get::<_, u32>(0)?.into(),
            en: row.get::<_, i32>(1)?.into(),
            spell_visual_kit: row.get::<_, u32>(2)?.into(),
        });
    }
    Ok(environmental_damage::EnvironmentalDamage { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Exhaustion() -> (&'static str, &'static str, &'static str) {
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
        state_name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `Exhaustion`;"
    )
}


pub(crate) fn exhaustion_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<exhaustion::Exhaustion, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(exhaustion::ExhaustionRow {
            id: row.get::<_, u32>(0)?.into(),
            experience: row.get::<_, i32>(1)?.into(),
            factor: row.get::<_, f32>(2)?.into(),
            outdoor_hours: row.get::<_, f32>(3)?.into(),
            inn_hours: row.get::<_, f32>(4)?.into(),
            state_name: LocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                flags: row.get::<_, u32>(13)?.into(),
            },
            threshold: row.get::<_, f32>(14)?.into(),
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
        parent_faction INTEGER  NOT NULL,
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags INTEGER NOT NULL
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
    FROM `Faction`;"
    )
}


pub(crate) fn faction_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<faction::Faction, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(faction::FactionRow {
            id: row.get::<_, u32>(0)?.into(),
            reputation_index: row.get::<_, u32>(1)?.into(),
            reputation_race_mask: [row.get::<_, i32>(2)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_race_mask")))?, row.get::<_, i32>(3)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_race_mask")))?, row.get::<_, i32>(4)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_race_mask")))?, row.get::<_, i32>(5)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_race_mask")))?,             ],
            reputation_class_mask: [row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(),             ],
            reputation_base: [row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(),             ],
            reputation_flags: [row.get::<_, i32>(14)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_flags")))?, row.get::<_, i32>(15)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_flags")))?, row.get::<_, i32>(16)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_flags")))?, row.get::<_, i32>(17)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for reputation_flags")))?,             ],
            parent_faction: row.get::<_, u32>(18)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(19)?.into(),
                ko_kr: row.get::<_, String>(20)?.into(),
                fr_fr: row.get::<_, String>(21)?.into(),
                de_de: row.get::<_, String>(22)?.into(),
                en_cn: row.get::<_, String>(23)?.into(),
                en_tw: row.get::<_, String>(24)?.into(),
                es_es: row.get::<_, String>(25)?.into(),
                es_mx: row.get::<_, String>(26)?.into(),
                flags: row.get::<_, u32>(27)?.into(),
            },
            description: LocalizedString {
                en_gb: row.get::<_, String>(28)?.into(),
                ko_kr: row.get::<_, String>(29)?.into(),
                fr_fr: row.get::<_, String>(30)?.into(),
                de_de: row.get::<_, String>(31)?.into(),
                en_cn: row.get::<_, String>(32)?.into(),
                en_tw: row.get::<_, String>(33)?.into(),
                es_es: row.get::<_, String>(34)?.into(),
                es_mx: row.get::<_, String>(35)?.into(),
                flags: row.get::<_, u32>(36)?.into(),
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `FactionGroup`;"
    )
}


pub(crate) fn faction_group_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<faction_group::FactionGroup, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(faction_group::FactionGroupRow {
            id: row.get::<_, u32>(0)?.into(),
            mask_id: row.get::<_, u32>(1)?.into(),
            internal_name: row.get::<_, String>(2)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
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
        friends_0,
        friends_1,
        friends_2,
        friends_3
    FROM `FactionTemplate`;"
    )
}


pub(crate) fn faction_template_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<faction_template::FactionTemplate, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(faction_template::FactionTemplateRow {
            id: row.get::<_, u32>(0)?.into(),
            faction: row.get::<_, u32>(1)?.into(),
            flags: row.get::<_, u32>(2)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for flags")))?,
            faction_group: row.get::<_, u32>(3)?.into(),
            friend_group: row.get::<_, u32>(4)?.into(),
            enemy_group: row.get::<_, u32>(5)?.into(),
            enemies: [row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(),             ],
            friends: [row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(),             ],
        });
    }
    Ok(faction_template::FactionTemplate { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn FootprintTextures() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        footstep_file_path
    FROM `FootprintTextures`;"
    )
}


pub(crate) fn footprint_textures_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<footprint_textures::FootprintTextures, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(footprint_textures::FootprintTexturesRow {
            id: row.get::<_, u32>(0)?.into(),
            footstep_file_path: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(footprint_textures::FootprintTextures { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn FootstepTerrainLookup() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        creature_footstep,
        terrain_type,
        sound_entry,
        sound_entry_splash
    FROM `FootstepTerrainLookup`;"
    )
}


pub(crate) fn footstep_terrain_lookup_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<footstep_terrain_lookup::FootstepTerrainLookup, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(footstep_terrain_lookup::FootstepTerrainLookupRow {
            id: row.get::<_, u32>(0)?.into(),
            creature_footstep: row.get::<_, u32>(1)?.into(),
            terrain_type: row.get::<_, u32>(2)?.into(),
            sound_entry: row.get::<_, u32>(3)?.into(),
            sound_entry_splash: row.get::<_, u32>(4)?.into(),
        });
    }
    Ok(footstep_terrain_lookup::FootstepTerrainLookup { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMSurveyCurrentSurvey() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        language,
        gm_survey
    FROM `GMSurveyCurrentSurvey`;"
    )
}


pub(crate) fn gm_survey_current_survey_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_survey_current_survey::GMSurveyCurrentSurvey, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_survey_current_survey::GMSurveyCurrentSurveyRow {
            language: gm_survey_current_survey::ClientLanguage::from_int(row.get::<_, i32>(0)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            gm_survey: row.get::<_, u32>(1)?.into(),
        });
    }
    Ok(gm_survey_current_survey::GMSurveyCurrentSurvey { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMSurveyQuestions() -> (&'static str, &'static str, &'static str) {
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
        question_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `GMSurveyQuestions`;"
    )
}


pub(crate) fn gm_survey_questions_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_survey_questions::GMSurveyQuestions, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_survey_questions::GMSurveyQuestionsRow {
            id: row.get::<_, u32>(0)?.into(),
            question: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
    ,
    "SELECT
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
    FROM `GMSurveySurveys`;"
    )
}


pub(crate) fn gm_survey_surveys_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_survey_surveys::GMSurveySurveys, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_survey_surveys::GMSurveySurveysRow {
            id: row.get::<_, u32>(0)?.into(),
            questions: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(),             ],
        });
    }
    Ok(gm_survey_surveys::GMSurveySurveys { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GMTicketCategory() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `GMTicketCategory`;"
    )
}


pub(crate) fn gm_ticket_category_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<gm_ticket_category::GMTicketCategory, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(gm_ticket_category::GMTicketCategoryRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
    ,
    "SELECT
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
    FROM `GameObjectDisplayInfo`;"
    )
}


pub(crate) fn game_object_display_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<game_object_display_info::GameObjectDisplayInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(game_object_display_info::GameObjectDisplayInfoRow {
            id: row.get::<_, u32>(0)?.into(),
            model_name: row.get::<_, String>(1)?.into(),
            sound_entry: [row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(),             ],
        });
    }
    Ok(game_object_display_info::GameObjectDisplayInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GameTips() -> (&'static str, &'static str, &'static str) {
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
        text_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `GameTips`;"
    )
}


pub(crate) fn game_tips_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<game_tips::GameTips, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(game_tips::GameTipsRow {
            id: row.get::<_, u32>(0)?.into(),
            text: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
        });
    }
    Ok(game_tips::GameTips { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GroundEffectDoodad() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        internal_id,
        doodad_path
    FROM `GroundEffectDoodad`;"
    )
}


pub(crate) fn ground_effect_doodad_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<ground_effect_doodad::GroundEffectDoodad, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(ground_effect_doodad::GroundEffectDoodadRow {
            id: row.get::<_, u32>(0)?.into(),
            internal_id: row.get::<_, i32>(1)?.into(),
            doodad_path: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(ground_effect_doodad::GroundEffectDoodad { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn GroundEffectTexture() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        doodad_0,
        doodad_1,
        doodad_2,
        doodad_3,
        density,
        terrain_type
    FROM `GroundEffectTexture`;"
    )
}


pub(crate) fn ground_effect_texture_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<ground_effect_texture::GroundEffectTexture, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(ground_effect_texture::GroundEffectTextureRow {
            id: row.get::<_, u32>(0)?.into(),
            doodad: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(),             ],
            density: row.get::<_, i32>(5)?.into(),
            terrain_type: row.get::<_, u32>(6)?.into(),
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
    ,
    "SELECT
        id,
        hide_geoset_0,
        hide_geoset_1,
        hide_geoset_2,
        hide_geoset_3,
        hide_geoset_4
    FROM `HelmetGeosetVisData`;"
    )
}


pub(crate) fn helmet_geoset_vis_data_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<helmet_geoset_vis_data::HelmetGeosetVisData, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(helmet_geoset_vis_data::HelmetGeosetVisDataRow {
            id: row.get::<_, u32>(0)?.into(),
            hide_geoset: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(),             ],
        });
    }
    Ok(helmet_geoset_vis_data::HelmetGeosetVisData { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemBagFamily() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemBagFamily`;"
    )
}


pub(crate) fn item_bag_family_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_bag_family::ItemBagFamily, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_bag_family::ItemBagFamilyRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
        });
    }
    Ok(item_bag_family::ItemBagFamily { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemClass() -> (&'static str, &'static str, &'static str) {
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
        class_name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemClass`;"
    )
}


pub(crate) fn item_class_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_class::ItemClass, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_class::ItemClassRow {
            id: row.get::<_, u32>(0)?.into(),
            subclass_map: row.get::<_, u32>(1)?.into(),
            item_class: item_class::ItemWeaponClass::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            class_name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
            },
        });
    }
    Ok(item_class::ItemClass { rows: data })
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
    FROM `ItemDisplayInfo`;"
    )
}


pub(crate) fn item_display_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_display_info::ItemDisplayInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_display_info::ItemDisplayInfoRow {
            id: row.get::<_, u32>(0)?.into(),
            model_name: [row.get::<_, String>(1)?.into(), row.get::<_, String>(2)?.into(),             ],
            model_texture: [row.get::<_, String>(3)?.into(), row.get::<_, String>(4)?.into(),             ],
            inventory_icon: [row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(),             ],
            geoset_group: [row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
            spell_visual: row.get::<_, u32>(10)?.into(),
            group_sound_index: row.get::<_, u32>(11)?.into(),
            helmet_geoset_vis: [row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(),             ],
            textures: [row.get::<_, String>(14)?.into(), row.get::<_, String>(15)?.into(), row.get::<_, String>(16)?.into(), row.get::<_, String>(17)?.into(), row.get::<_, String>(18)?.into(), row.get::<_, String>(19)?.into(), row.get::<_, String>(20)?.into(), row.get::<_, String>(21)?.into(),             ],
            item_visual: row.get::<_, u32>(22)?.into(),
        });
    }
    Ok(item_display_info::ItemDisplayInfo { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemGroupSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_entry_0,
        sound_entry_1,
        sound_entry_2,
        sound_entry_3
    FROM `ItemGroupSounds`;"
    )
}


pub(crate) fn item_group_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_group_sounds::ItemGroupSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_group_sounds::ItemGroupSoundsRow {
            id: row.get::<_, u32>(0)?.into(),
            sound_entry: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(),             ],
        });
    }
    Ok(item_group_sounds::ItemGroupSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemPetFood() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemPetFood`;"
    )
}


pub(crate) fn item_pet_food_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_pet_food::ItemPetFood, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_pet_food::ItemPetFoodRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
        suffix_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemRandomProperties`;"
    )
}


pub(crate) fn item_random_properties_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_random_properties::ItemRandomProperties, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_random_properties::ItemRandomPropertiesRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            spell_item_enchantment: [row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(),             ],
            suffix: LocalizedString {
                en_gb: row.get::<_, String>(7)?.into(),
                ko_kr: row.get::<_, String>(8)?.into(),
                fr_fr: row.get::<_, String>(9)?.into(),
                de_de: row.get::<_, String>(10)?.into(),
                en_cn: row.get::<_, String>(11)?.into(),
                en_tw: row.get::<_, String>(12)?.into(),
                es_es: row.get::<_, String>(13)?.into(),
                es_mx: row.get::<_, String>(14)?.into(),
                flags: row.get::<_, u32>(15)?.into(),
            },
        });
    }
    Ok(item_random_properties::ItemRandomProperties { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemSet() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `ItemSet`;"
    )
}


pub(crate) fn item_set_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_set::ItemSet, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_set::ItemSetRow {
            id: row.get::<_, i32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            items: [row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(), row.get::<_, u32>(16)?.into(), row.get::<_, u32>(17)?.into(), row.get::<_, u32>(18)?.into(), row.get::<_, u32>(19)?.into(),             ],
            bank_item: [row.get::<_, u32>(20)?.into(), row.get::<_, u32>(21)?.into(), row.get::<_, u32>(22)?.into(), row.get::<_, u32>(23)?.into(), row.get::<_, u32>(24)?.into(), row.get::<_, u32>(25)?.into(), row.get::<_, u32>(26)?.into(),             ],
            set_spell: [row.get::<_, u32>(27)?.into(), row.get::<_, u32>(28)?.into(), row.get::<_, u32>(29)?.into(), row.get::<_, u32>(30)?.into(), row.get::<_, u32>(31)?.into(), row.get::<_, u32>(32)?.into(), row.get::<_, u32>(33)?.into(), row.get::<_, u32>(34)?.into(),             ],
            set_threshold: [row.get::<_, u32>(35)?.into(), row.get::<_, u32>(36)?.into(), row.get::<_, u32>(37)?.into(), row.get::<_, u32>(38)?.into(), row.get::<_, u32>(39)?.into(), row.get::<_, u32>(40)?.into(), row.get::<_, u32>(41)?.into(), row.get::<_, u32>(42)?.into(),             ],
            required_skill: row.get::<_, u32>(43)?.into(),
            required_skill_rank: row.get::<_, u32>(44)?.into(),
        });
    }
    Ok(item_set::ItemSet { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemSubClass() -> (&'static str, &'static str, &'static str) {
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
        display_name_flags INTEGER NOT NULL,
        verbose_name_en_gb TEXT NOT NULL,
        verbose_name_ko_kr TEXT NOT NULL,
        verbose_name_fr_fr TEXT NOT NULL,
        verbose_name_de_de TEXT NOT NULL,
        verbose_name_en_cn TEXT NOT NULL,
        verbose_name_en_tw TEXT NOT NULL,
        verbose_name_es_es TEXT NOT NULL,
        verbose_name_es_mx TEXT NOT NULL,
        verbose_name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemSubClass`;"
    )
}


pub(crate) fn item_sub_class_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_sub_class::ItemSubClass, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_sub_class::ItemSubClassRow {
            item_class: row.get::<_, u32>(0)?.into(),
            subclass: row.get::<_, i32>(1)?.into(),
            prerequisite_proficiency: row.get::<_, i32>(2)?.into(),
            postrequisite_proficiency: row.get::<_, i32>(3)?.into(),
            flags: row.get::<_, i32>(4)?.into(),
            display_flags: row.get::<_, i32>(5)?.into(),
            weapon_parry_sequence: row.get::<_, i32>(6)?.into(),
            weapon_ready_sequence: row.get::<_, i32>(7)?.into(),
            weapon_attack_sequence: row.get::<_, i32>(8)?.into(),
            weapon_swing_size: row.get::<_, i32>(9)?.into(),
            display_name: LocalizedString {
                en_gb: row.get::<_, String>(10)?.into(),
                ko_kr: row.get::<_, String>(11)?.into(),
                fr_fr: row.get::<_, String>(12)?.into(),
                de_de: row.get::<_, String>(13)?.into(),
                en_cn: row.get::<_, String>(14)?.into(),
                en_tw: row.get::<_, String>(15)?.into(),
                es_es: row.get::<_, String>(16)?.into(),
                es_mx: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
            verbose_name: LocalizedString {
                en_gb: row.get::<_, String>(19)?.into(),
                ko_kr: row.get::<_, String>(20)?.into(),
                fr_fr: row.get::<_, String>(21)?.into(),
                de_de: row.get::<_, String>(22)?.into(),
                en_cn: row.get::<_, String>(23)?.into(),
                en_tw: row.get::<_, String>(24)?.into(),
                es_es: row.get::<_, String>(25)?.into(),
                es_mx: row.get::<_, String>(26)?.into(),
                flags: row.get::<_, u32>(27)?.into(),
            },
        });
    }
    Ok(item_sub_class::ItemSubClass { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemSubClassMask() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ItemSubClassMask`;"
    )
}


pub(crate) fn item_sub_class_mask_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_sub_class_mask::ItemSubClassMask, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_sub_class_mask::ItemSubClassMaskRow {
            subclass: row.get::<_, u32>(0)?.into(),
            mask: row.get::<_, i32>(1)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                flags: row.get::<_, u32>(10)?.into(),
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
    ,
    "SELECT
        id,
        model_path
    FROM `ItemVisualEffects`;"
    )
}


pub(crate) fn item_visual_effects_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_visual_effects::ItemVisualEffects, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_visual_effects::ItemVisualEffectsRow {
            id: row.get::<_, u32>(0)?.into(),
            model_path: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(item_visual_effects::ItemVisualEffects { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn ItemVisuals() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        item_visual_effects_0,
        item_visual_effects_1,
        item_visual_effects_2,
        item_visual_effects_3,
        item_visual_effects_4
    FROM `ItemVisuals`;"
    )
}


pub(crate) fn item_visuals_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<item_visuals::ItemVisuals, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(item_visuals::ItemVisualsRow {
            id: row.get::<_, u32>(0)?.into(),
            item_visual_effects: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(),             ],
        });
    }
    Ok(item_visuals::ItemVisuals { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LFGDungeons() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `LFGDungeons`;"
    )
}


pub(crate) fn lfg_dungeons_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<lfg_dungeons::LFGDungeons, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(lfg_dungeons::LFGDungeonsRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            min_allowed_level: row.get::<_, u32>(10)?.into(),
            max_allowed_level: row.get::<_, u32>(11)?.into(),
            instance_type: lfg_dungeons::InstanceType::from_int(row.get::<_, i32>(12)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            faction: lfg_dungeons::LfgFaction::from_int(row.get::<_, i32>(13)? as i8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
        });
    }
    Ok(lfg_dungeons::LFGDungeons { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LanguageWords() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        language,
        word
    FROM `LanguageWords`;"
    )
}


pub(crate) fn language_words_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<language_words::LanguageWords, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(language_words::LanguageWordsRow {
            id: row.get::<_, u32>(0)?.into(),
            language: row.get::<_, u32>(1)?.into(),
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Languages`;"
    )
}


pub(crate) fn languages_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<languages::Languages, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(languages::LanguagesRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
    ,
    "SELECT
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
    FROM `Light`;"
    )
}


pub(crate) fn light_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light::Light, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light::LightRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            location_x: row.get::<_, f32>(2)?.into(),
            location_y: row.get::<_, f32>(3)?.into(),
            location_z: row.get::<_, f32>(4)?.into(),
            falloff_start: row.get::<_, f32>(5)?.into(),
            falloff_end: row.get::<_, f32>(6)?.into(),
            light_params: [row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(),             ],
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
            id: row.get::<_, u32>(0)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
    ,
    "SELECT
        id,
        highlight_sky,
        light_skybox,
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
            id: row.get::<_, u32>(0)?.into(),
            highlight_sky: row.get::<_, bool>(1)?.into(),
            light_skybox: row.get::<_, u32>(2)?.into(),
            glow: row.get::<_, f32>(3)?.into(),
            water_shallow_alpha: row.get::<_, f32>(4)?.into(),
            water_deep_alpha: row.get::<_, f32>(5)?.into(),
            ocean_shallow_alpha: row.get::<_, f32>(6)?.into(),
            ocean_deep_alpha: row.get::<_, f32>(7)?.into(),
            flags: row.get::<_, u32>(8)?.into(),
        });
    }
    Ok(light_params::LightParams { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LightSkybox() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        skybox_model_path
    FROM `LightSkybox`;"
    )
}


pub(crate) fn light_skybox_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<light_skybox::LightSkybox, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(light_skybox::LightSkyboxRow {
            id: row.get::<_, u32>(0)?.into(),
            skybox_model_path: row.get::<_, String>(1)?.into(),
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
    ,
    "SELECT
        id,
        name,
        ty,
        spell
    FROM `LiquidType`;"
    )
}


pub(crate) fn liquid_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<liquid_type::LiquidType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(liquid_type::LiquidTypeRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            ty: liquid_type::OceanType::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            spell: row.get::<_, u32>(3)?.into(),
        });
    }
    Ok(liquid_type::LiquidType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn LoadingScreenTaxiSplines() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `LoadingScreenTaxiSplines`;"
    )
}


pub(crate) fn loading_screen_taxi_splines_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<loading_screen_taxi_splines::LoadingScreenTaxiSplines, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(loading_screen_taxi_splines::LoadingScreenTaxiSplinesRow {
            id: row.get::<_, u32>(0)?.into(),
            taxi_path: row.get::<_, u32>(1)?.into(),
            location_x: [row.get::<_, f32>(2)?.into(), row.get::<_, f32>(3)?.into(), row.get::<_, f32>(4)?.into(), row.get::<_, f32>(5)?.into(), row.get::<_, f32>(6)?.into(), row.get::<_, f32>(7)?.into(), row.get::<_, f32>(8)?.into(), row.get::<_, f32>(9)?.into(),             ],
            location_y: [row.get::<_, f32>(10)?.into(), row.get::<_, f32>(11)?.into(), row.get::<_, f32>(12)?.into(), row.get::<_, f32>(13)?.into(), row.get::<_, f32>(14)?.into(), row.get::<_, f32>(15)?.into(), row.get::<_, f32>(16)?.into(), row.get::<_, f32>(17)?.into(),             ],
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
    ,
    "SELECT
        id,
        name,
        file_path
    FROM `LoadingScreens`;"
    )
}


pub(crate) fn loading_screens_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<loading_screens::LoadingScreens, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(loading_screens::LoadingScreensRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            file_path: row.get::<_, String>(2)?.into(),
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
    FROM `Lock`;"
    )
}


pub(crate) fn lock_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<lock::Lock, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(lock::LockRow {
            id: row.get::<_, u32>(0)?.into(),
            ty: [            lock::LockType::from_int(row.get::<_, i32>(1)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(4)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(5)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(6)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(7)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            lock::LockType::from_int(row.get::<_, i32>(8)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            ],
            property: [row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(), row.get::<_, u32>(16)?.into(),             ],
            required_skill: [row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(), row.get::<_, i32>(19)?.into(), row.get::<_, i32>(20)?.into(), row.get::<_, i32>(21)?.into(), row.get::<_, i32>(22)?.into(), row.get::<_, i32>(23)?.into(), row.get::<_, i32>(24)?.into(),             ],
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL,
        resource_name_en_gb TEXT NOT NULL,
        resource_name_ko_kr TEXT NOT NULL,
        resource_name_fr_fr TEXT NOT NULL,
        resource_name_de_de TEXT NOT NULL,
        resource_name_en_cn TEXT NOT NULL,
        resource_name_en_tw TEXT NOT NULL,
        resource_name_es_es TEXT NOT NULL,
        resource_name_es_mx TEXT NOT NULL,
        resource_name_flags INTEGER NOT NULL,
        verb_en_gb TEXT NOT NULL,
        verb_ko_kr TEXT NOT NULL,
        verb_fr_fr TEXT NOT NULL,
        verb_de_de TEXT NOT NULL,
        verb_en_cn TEXT NOT NULL,
        verb_en_tw TEXT NOT NULL,
        verb_es_es TEXT NOT NULL,
        verb_es_mx TEXT NOT NULL,
        verb_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `LockType`;"
    )
}


pub(crate) fn lock_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<lock_type::LockType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(lock_type::LockTypeRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            resource_name: LocalizedString {
                en_gb: row.get::<_, String>(10)?.into(),
                ko_kr: row.get::<_, String>(11)?.into(),
                fr_fr: row.get::<_, String>(12)?.into(),
                de_de: row.get::<_, String>(13)?.into(),
                en_cn: row.get::<_, String>(14)?.into(),
                en_tw: row.get::<_, String>(15)?.into(),
                es_es: row.get::<_, String>(16)?.into(),
                es_mx: row.get::<_, String>(17)?.into(),
                flags: row.get::<_, u32>(18)?.into(),
            },
            verb: LocalizedString {
                en_gb: row.get::<_, String>(19)?.into(),
                ko_kr: row.get::<_, String>(20)?.into(),
                fr_fr: row.get::<_, String>(21)?.into(),
                de_de: row.get::<_, String>(22)?.into(),
                en_cn: row.get::<_, String>(23)?.into(),
                en_tw: row.get::<_, String>(24)?.into(),
                es_es: row.get::<_, String>(25)?.into(),
                es_mx: row.get::<_, String>(26)?.into(),
                flags: row.get::<_, u32>(27)?.into(),
            },
            cursor_name: row.get::<_, String>(28)?.into(),
        });
    }
    Ok(lock_type::LockType { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn MailTemplate() -> (&'static str, &'static str, &'static str) {
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
        body_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `MailTemplate`;"
    )
}


pub(crate) fn mail_template_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<mail_template::MailTemplate, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(mail_template::MailTemplateRow {
            id: row.get::<_, u32>(0)?.into(),
            body: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
        map_name_flags INTEGER NOT NULL,
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
        map_description_horde_flags INTEGER NOT NULL,
        map_description_alliance_en_gb TEXT NOT NULL,
        map_description_alliance_ko_kr TEXT NOT NULL,
        map_description_alliance_fr_fr TEXT NOT NULL,
        map_description_alliance_de_de TEXT NOT NULL,
        map_description_alliance_en_cn TEXT NOT NULL,
        map_description_alliance_en_tw TEXT NOT NULL,
        map_description_alliance_es_es TEXT NOT NULL,
        map_description_alliance_es_mx TEXT NOT NULL,
        map_description_alliance_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `Map`;"
    )
}


pub(crate) fn map_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<map::Map, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(map::MapRow {
            id: row.get::<_, u32>(0)?.into(),
            internal_name: row.get::<_, String>(1)?.into(),
            instance_type: map::InstanceType::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            battleground: row.get::<_, bool>(3)?.into(),
            map_name: LocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                flags: row.get::<_, u32>(12)?.into(),
            },
            min_level: row.get::<_, i32>(13)?.into(),
            max_level: row.get::<_, i32>(14)?.into(),
            max_players: row.get::<_, i32>(15)?.into(),
            unknown: [row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(),             ],
            area_table: row.get::<_, u32>(19)?.into(),
            map_description_horde: LocalizedString {
                en_gb: row.get::<_, String>(20)?.into(),
                ko_kr: row.get::<_, String>(21)?.into(),
                fr_fr: row.get::<_, String>(22)?.into(),
                de_de: row.get::<_, String>(23)?.into(),
                en_cn: row.get::<_, String>(24)?.into(),
                en_tw: row.get::<_, String>(25)?.into(),
                es_es: row.get::<_, String>(26)?.into(),
                es_mx: row.get::<_, String>(27)?.into(),
                flags: row.get::<_, u32>(28)?.into(),
            },
            map_description_alliance: LocalizedString {
                en_gb: row.get::<_, String>(29)?.into(),
                ko_kr: row.get::<_, String>(30)?.into(),
                fr_fr: row.get::<_, String>(31)?.into(),
                de_de: row.get::<_, String>(32)?.into(),
                en_cn: row.get::<_, String>(33)?.into(),
                en_tw: row.get::<_, String>(34)?.into(),
                es_es: row.get::<_, String>(35)?.into(),
                es_mx: row.get::<_, String>(36)?.into(),
                flags: row.get::<_, u32>(37)?.into(),
            },
            loading_screen: row.get::<_, u32>(38)?.into(),
            raid_offset: row.get::<_, i32>(39)?.into(),
            unknown_2: [row.get::<_, i32>(40)?.into(), row.get::<_, i32>(41)?.into(),             ],
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
    ,
    "SELECT
        id,
        flags,
        foley_sound
    FROM `Material`;"
    )
}


pub(crate) fn material_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<material::Material, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(material::MaterialRow {
            id: row.get::<_, u32>(0)?.into(),
            flags: row.get::<_, i32>(1)?.into(),
            foley_sound: row.get::<_, u32>(2)?.into(),
        });
    }
    Ok(material::Material { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NPCSounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_entries_0,
        sound_entries_1,
        sound_entries_2,
        sound_entries_3
    FROM `NPCSounds`;"
    )
}


pub(crate) fn npc_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<npc_sounds::NPCSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(npc_sounds::NPCSoundsRow {
            id: row.get::<_, u32>(0)?.into(),
            sound_entries: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(),             ],
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
    ,
    "SELECT
        id,
        name,
        race,
        gender
    FROM `NameGen`;"
    )
}


pub(crate) fn name_gen_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<name_gen::NameGen, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(name_gen::NameGenRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            race: row.get::<_, u32>(2)?.into(),
            gender: name_gen::Gender::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
        });
    }
    Ok(name_gen::NameGen { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NamesProfanity() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name
    FROM `NamesProfanity`;"
    )
}


pub(crate) fn names_profanity_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<names_profanity::NamesProfanity, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(names_profanity::NamesProfanityRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(names_profanity::NamesProfanity { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn NamesReserved() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        name
    FROM `NamesReserved`;"
    )
}


pub(crate) fn names_reserved_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<names_reserved::NamesReserved, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(names_reserved::NamesReservedRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Package`;"
    )
}


pub(crate) fn package_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<package::Package, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(package::PackageRow {
            id: row.get::<_, u32>(0)?.into(),
            icon: row.get::<_, String>(1)?.into(),
            cost: row.get::<_, i32>(2)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
pub(crate) fn PetLoyalty() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `PetLoyalty`;"
    )
}


pub(crate) fn pet_loyalty_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<pet_loyalty::PetLoyalty, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(pet_loyalty::PetLoyaltyRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `PetPersonality`;"
    )
}


pub(crate) fn pet_personality_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<pet_personality::PetPersonality, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(pet_personality::PetPersonalityRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            threshold_unhappy: row.get::<_, i32>(10)?.into(),
            threshold_content: row.get::<_, i32>(11)?.into(),
            threshold_happy: row.get::<_, i32>(12)?.into(),
            damage_unhappy: row.get::<_, f32>(13)?.into(),
            damage_content: row.get::<_, f32>(14)?.into(),
            damage_happy: row.get::<_, f32>(15)?.into(),
            modifier_unhappy: row.get::<_, f32>(16)?.into(),
            modifier_content: row.get::<_, f32>(17)?.into(),
            modifier_happy: row.get::<_, f32>(18)?.into(),
        });
    }
    Ok(pet_personality::PetPersonality { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn QuestInfo() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `QuestInfo`;"
    )
}


pub(crate) fn quest_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<quest_info::QuestInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(quest_info::QuestInfoRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `QuestSort`;"
    )
}


pub(crate) fn quest_sort_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<quest_sort::QuestSort, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(quest_sort::QuestSortRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
        });
    }
    Ok(quest_sort::QuestSort { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Resistances() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Resistances`;"
    )
}


pub(crate) fn resistances_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<resistances::Resistances, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(resistances::ResistancesRow {
            id: row.get::<_, u32>(0)?.into(),
            physical_damage: row.get::<_, bool>(1)?.into(),
            fizzle_sound_entry: row.get::<_, u32>(2)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
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
        text_en_gb TEXT NOT NULL,
        text_ko_kr TEXT NOT NULL,
        text_fr_fr TEXT NOT NULL,
        text_de_de TEXT NOT NULL,
        text_en_cn TEXT NOT NULL,
        text_en_tw TEXT NOT NULL,
        text_es_es TEXT NOT NULL,
        text_es_mx TEXT NOT NULL,
        text_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `ServerMessages`;"
    )
}


pub(crate) fn server_messages_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<server_messages::ServerMessages, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(server_messages::ServerMessagesRow {
            id: row.get::<_, u32>(0)?.into(),
            text: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
    ,
    "SELECT
        id,
        item_class,
        item_subclass,
        item_env_types,
        not_shield,
        sheathe_sound,
        draw_sound
    FROM `SheatheSoundLookups`;"
    )
}


pub(crate) fn sheathe_sound_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sheathe_sound_lookups::SheatheSoundLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sheathe_sound_lookups::SheatheSoundLookupsRow {
            id: row.get::<_, u32>(0)?.into(),
            item_class: row.get::<_, u32>(1)?.into(),
            item_subclass: row.get::<_, u32>(2)?.into(),
            item_env_types: sheathe_sound_lookups::ItemEnvTypes::from_int(row.get::<_, i32>(3)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            not_shield: row.get::<_, bool>(4)?.into(),
            sheathe_sound: row.get::<_, u32>(5)?.into(),
            draw_sound: row.get::<_, u32>(6)?.into(),
        });
    }
    Ok(sheathe_sound_lookups::SheatheSoundLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillCostsData() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        skill_costs,
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
            id: row.get::<_, u32>(0)?.into(),
            skill_costs: row.get::<_, i32>(1)?.into(),
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
        display_name_flags INTEGER NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SkillLine`;"
    )
}


pub(crate) fn skill_line_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_line::SkillLine, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_line::SkillLineRow {
            id: row.get::<_, u32>(0)?.into(),
            category: row.get::<_, u32>(1)?.into(),
            skill_costs: row.get::<_, u32>(2)?.into(),
            display_name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
            },
            description: LocalizedString {
                en_gb: row.get::<_, String>(12)?.into(),
                ko_kr: row.get::<_, String>(13)?.into(),
                fr_fr: row.get::<_, String>(14)?.into(),
                de_de: row.get::<_, String>(15)?.into(),
                en_cn: row.get::<_, String>(16)?.into(),
                en_tw: row.get::<_, String>(17)?.into(),
                es_es: row.get::<_, String>(18)?.into(),
                es_mx: row.get::<_, String>(19)?.into(),
                flags: row.get::<_, u32>(20)?.into(),
            },
            spell_icon: row.get::<_, u32>(21)?.into(),
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
    ,
    "SELECT
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
    FROM `SkillLineAbility`;"
    )
}


pub(crate) fn skill_line_ability_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_line_ability::SkillLineAbility, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_line_ability::SkillLineAbilityRow {
            id: row.get::<_, u32>(0)?.into(),
            skill_line: row.get::<_, u32>(1)?.into(),
            spell: row.get::<_, u32>(2)?.into(),
            race_mask: row.get::<_, u32>(3)?.into(),
            class_mask: row.get::<_, u32>(4)?.into(),
            exclude_race: row.get::<_, u32>(5)?.into(),
            exclude_class: row.get::<_, u32>(6)?.into(),
            superseded_by: row.get::<_, u32>(7)?.into(),
            acquire_method: row.get::<_, i32>(8)?.into(),
            trivial_skill_line_rank_high: row.get::<_, i32>(9)?.into(),
            trivial_skill_line_rank_low: row.get::<_, i32>(10)?.into(),
            character_points: [row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(),             ],
            num_skills_up: row.get::<_, i32>(13)?.into(),
            unknown_padding: row.get::<_, i32>(14)?.into(),
        });
    }
    Ok(skill_line_ability::SkillLineAbility { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillLineCategory() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SkillLineCategory`;"
    )
}


pub(crate) fn skill_line_category_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_line_category::SkillLineCategory, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_line_category::SkillLineCategoryRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            sort_index: row.get::<_, i32>(10)?.into(),
        });
    }
    Ok(skill_line_category::SkillLineCategory { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SkillRaceClassInfo() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        skill_line,
        race_mask,
        class_mask,
        flags,
        min_level,
        skill_tier,
        skill_cost
    FROM `SkillRaceClassInfo`;"
    )
}


pub(crate) fn skill_race_class_info_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<skill_race_class_info::SkillRaceClassInfo, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(skill_race_class_info::SkillRaceClassInfoRow {
            id: row.get::<_, u32>(0)?.into(),
            skill_line: row.get::<_, u32>(1)?.into(),
            race_mask: row.get::<_, u32>(2)?.into(),
            class_mask: row.get::<_, u32>(3)?.into(),
            flags: row.get::<_, i32>(4)?.into(),
            min_level: row.get::<_, i32>(5)?.into(),
            skill_tier: row.get::<_, u32>(6)?.into(),
            skill_cost: row.get::<_, u32>(7)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
    ,
    "SELECT
        id,
        day_sound,
        night_sound
    FROM `SoundAmbience`;"
    )
}


pub(crate) fn sound_ambience_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_ambience::SoundAmbience, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_ambience::SoundAmbienceRow {
            id: row.get::<_, u32>(0)?.into(),
            day_sound: row.get::<_, u32>(1)?.into(),
            night_sound: row.get::<_, u32>(2)?.into(),
        });
    }
    Ok(sound_ambience::SoundAmbience { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundCharacterMacroLines() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        unknown,
        gender,
        race,
        sound
    FROM `SoundCharacterMacroLines`;"
    )
}


pub(crate) fn sound_character_macro_lines_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_character_macro_lines::SoundCharacterMacroLines, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_character_macro_lines::SoundCharacterMacroLinesRow {
            id: row.get::<_, u32>(0)?.into(),
            unknown: row.get::<_, u32>(1)?.into(),
            gender: sound_character_macro_lines::Gender::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            race: row.get::<_, u32>(3)?.into(),
            sound: row.get::<_, u32>(4)?.into(),
        });
    }
    Ok(sound_character_macro_lines::SoundCharacterMacroLines { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundEntries() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `SoundEntries`;"
    )
}


pub(crate) fn sound_entries_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_entries::SoundEntries, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_entries::SoundEntriesRow {
            id: row.get::<_, u32>(0)?.into(),
            sound_type: sound_entries::SoundType::from_int(row.get::<_, i32>(1)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            name: row.get::<_, String>(2)?.into(),
            files: [row.get::<_, String>(3)?.into(), row.get::<_, String>(4)?.into(), row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(), row.get::<_, String>(9)?.into(), row.get::<_, String>(10)?.into(), row.get::<_, String>(11)?.into(), row.get::<_, String>(12)?.into(),             ],
            frequency: [row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(), row.get::<_, u32>(16)?.into(), row.get::<_, u32>(17)?.into(), row.get::<_, u32>(18)?.into(), row.get::<_, u32>(19)?.into(), row.get::<_, u32>(20)?.into(), row.get::<_, u32>(21)?.into(), row.get::<_, u32>(22)?.into(),             ],
            directory_base: row.get::<_, String>(23)?.into(),
            volume: row.get::<_, f32>(24)?.into(),
            flags: row.get::<_, i32>(25)?.into(),
            min_distance: row.get::<_, f32>(26)?.into(),
            distance_cutoff: row.get::<_, f32>(27)?.into(),
            sound_entries_advanced: row.get::<_, i32>(28)?.into(),
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
    ,
    "SELECT
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
    FROM `SoundProviderPreferences`;"
    )
}


pub(crate) fn sound_provider_preferences_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_provider_preferences::SoundProviderPreferences, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_provider_preferences::SoundProviderPreferencesRow {
            id: row.get::<_, u32>(0)?.into(),
            description: row.get::<_, String>(1)?.into(),
            flags: row.get::<_, i32>(2)?.into(),
            eax_environment_selection: row.get::<_, i32>(3)?.into(),
            eax_decay_time: row.get::<_, f32>(4)?.into(),
            eax2_environment_size: row.get::<_, f32>(5)?.into(),
            eax_environment_diffusion: row.get::<_, f32>(6)?.into(),
            eax2_room: row.get::<_, i32>(7)?.into(),
            eax2_room_hf: row.get::<_, i32>(8)?.into(),
            eax2_decay_hf_ratio: row.get::<_, f32>(9)?.into(),
            eax2_reflections: row.get::<_, i32>(10)?.into(),
            eax2_reflections_delay: row.get::<_, f32>(11)?.into(),
            eax2_reverb: row.get::<_, i32>(12)?.into(),
            eax2_reverb_delay: row.get::<_, f32>(13)?.into(),
            eax2_room_rolloff: row.get::<_, f32>(14)?.into(),
            eax2_air_absorption: row.get::<_, f32>(15)?.into(),
            eax3_room_lf: row.get::<_, i32>(16)?.into(),
            eax3_delay_lf_ratio: row.get::<_, f32>(17)?.into(),
            eax3_echo_time: row.get::<_, f32>(18)?.into(),
            eax3_echo_depth: row.get::<_, f32>(19)?.into(),
            eax3_modulation_time: row.get::<_, f32>(20)?.into(),
            eax3_modulation_depth: row.get::<_, f32>(21)?.into(),
            eax3_hf_reference: row.get::<_, f32>(22)?.into(),
            eax3_lf_reference: row.get::<_, f32>(23)?.into(),
        });
    }
    Ok(sound_provider_preferences::SoundProviderPreferences { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundSamplePreferences() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `SoundSamplePreferences`;"
    )
}


pub(crate) fn sound_sample_preferences_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_sample_preferences::SoundSamplePreferences, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_sample_preferences::SoundSamplePreferencesRow {
            id: row.get::<_, u32>(0)?.into(),
            unknown: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(), row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(), row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(), row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(), row.get::<_, i32>(13)?.into(), row.get::<_, i32>(14)?.into(), row.get::<_, i32>(15)?.into(), row.get::<_, i32>(16)?.into(),             ],
        });
    }
    Ok(sound_sample_preferences::SoundSamplePreferences { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SoundWaterType() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        liquid_type,
        fluid_speed,
        sound
    FROM `SoundWaterType`;"
    )
}


pub(crate) fn sound_water_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<sound_water_type::SoundWaterType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(sound_water_type::SoundWaterTypeRow {
            id: row.get::<_, u32>(0)?.into(),
            liquid_type: row.get::<_, u32>(1)?.into(),
            fluid_speed: sound_water_type::FluidSpeed::from_int(row.get::<_, i32>(2)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            sound: row.get::<_, u32>(3)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
        interrupt_flags INTEGER  NOT NULL,
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL,
        name_subtext_en_gb TEXT NOT NULL,
        name_subtext_ko_kr TEXT NOT NULL,
        name_subtext_fr_fr TEXT NOT NULL,
        name_subtext_de_de TEXT NOT NULL,
        name_subtext_en_cn TEXT NOT NULL,
        name_subtext_en_tw TEXT NOT NULL,
        name_subtext_es_es TEXT NOT NULL,
        name_subtext_es_mx TEXT NOT NULL,
        name_subtext_flags INTEGER NOT NULL,
        description_en_gb TEXT NOT NULL,
        description_ko_kr TEXT NOT NULL,
        description_fr_fr TEXT NOT NULL,
        description_de_de TEXT NOT NULL,
        description_en_cn TEXT NOT NULL,
        description_en_tw TEXT NOT NULL,
        description_es_es TEXT NOT NULL,
        description_es_mx TEXT NOT NULL,
        description_flags INTEGER NOT NULL,
        aura_description_en_gb TEXT NOT NULL,
        aura_description_ko_kr TEXT NOT NULL,
        aura_description_fr_fr TEXT NOT NULL,
        aura_description_de_de TEXT NOT NULL,
        aura_description_en_cn TEXT NOT NULL,
        aura_description_en_tw TEXT NOT NULL,
        aura_description_es_es TEXT NOT NULL,
        aura_description_es_mx TEXT NOT NULL,
        aura_description_flags INTEGER NOT NULL,
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
        interrupt_flags,
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
    ,
    "SELECT
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
    FROM `Spell`;"
    )
}


pub(crate) fn spell_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell::Spell, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell::SpellRow {
            id: row.get::<_, u32>(0)?.into(),
            school: row.get::<_, u32>(1)?.into(),
            category: row.get::<_, u32>(2)?.into(),
            cast_ui: row.get::<_, i32>(3)?.into(),
            dispel_type: row.get::<_, u32>(4)?.into(),
            mechanic: row.get::<_, u32>(5)?.into(),
            attributes: row.get::<_, u32>(6)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for attributes")))?,
            attributes_ex1: row.get::<_, u32>(7)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for attributes_ex1")))?,
            attributes_ex2: row.get::<_, u32>(8)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for attributes_ex2")))?,
            attributes_ex3: row.get::<_, u32>(9)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for attributes_ex3")))?,
            attributes_ex4: row.get::<_, u32>(10)?.try_into().map_err(|e| SqliteError::EnumError(format!("Invalid flag {e:?} for attributes_ex4")))?,
            shapeshift_mask: row.get::<_, u32>(11)?.into(),
            shapeshift_exclude: row.get::<_, u32>(12)?.into(),
            targets: row.get::<_, i32>(13)?.into(),
            target_creature_type: row.get::<_, u32>(14)?.into(),
            requires_spell_focus: row.get::<_, u32>(15)?.into(),
            caster_aura_state: row.get::<_, i32>(16)?.into(),
            target_aura_state: row.get::<_, i32>(17)?.into(),
            casting_time_index: row.get::<_, u32>(18)?.into(),
            recovery_time: row.get::<_, i32>(19)?.into(),
            category_recovery_time: row.get::<_, i32>(20)?.into(),
            aura_interrupt_flags: row.get::<_, i32>(21)?.into(),
            channel_interrupt_flags: row.get::<_, i32>(22)?.into(),
            proc_type_mask: row.get::<_, i32>(23)?.into(),
            proc_chance: row.get::<_, i32>(24)?.into(),
            proc_charges: row.get::<_, i32>(25)?.into(),
            max_level: row.get::<_, i32>(26)?.into(),
            base_level: row.get::<_, i32>(27)?.into(),
            spell_level: row.get::<_, i32>(28)?.into(),
            duration: row.get::<_, u32>(29)?.into(),
            power_type: row.get::<_, i32>(30)?.into(),
            mana_cost: row.get::<_, i32>(31)?.into(),
            mana_cost_per_level: row.get::<_, i32>(32)?.into(),
            mana_cost_per_second: row.get::<_, i32>(33)?.into(),
            mana_cost_per_second_per_level: row.get::<_, i32>(34)?.into(),
            range: row.get::<_, u32>(35)?.into(),
            speed: row.get::<_, f32>(36)?.into(),
            modal_next_spell: row.get::<_, u32>(37)?.into(),
            stack_amount: row.get::<_, i32>(38)?.into(),
            totem: [row.get::<_, i32>(39)?.into(), row.get::<_, i32>(40)?.into(),             ],
            reagent: [row.get::<_, i32>(41)?.into(), row.get::<_, i32>(42)?.into(), row.get::<_, i32>(43)?.into(), row.get::<_, i32>(44)?.into(), row.get::<_, i32>(45)?.into(), row.get::<_, i32>(46)?.into(), row.get::<_, i32>(47)?.into(), row.get::<_, i32>(48)?.into(),             ],
            reagent_count: [row.get::<_, i32>(49)?.into(), row.get::<_, i32>(50)?.into(), row.get::<_, i32>(51)?.into(), row.get::<_, i32>(52)?.into(), row.get::<_, i32>(53)?.into(), row.get::<_, i32>(54)?.into(), row.get::<_, i32>(55)?.into(), row.get::<_, i32>(56)?.into(),             ],
            equipped_item_class: row.get::<_, u32>(57)?.into(),
            equipped_item_subclass: row.get::<_, u32>(58)?.into(),
            equipped_item_inventory_type: row.get::<_, i32>(59)?.into(),
            effect: [row.get::<_, i32>(60)?.into(), row.get::<_, i32>(61)?.into(), row.get::<_, i32>(62)?.into(),             ],
            effect_die_sides: [row.get::<_, i32>(63)?.into(), row.get::<_, i32>(64)?.into(), row.get::<_, i32>(65)?.into(),             ],
            effect_base_dice: [row.get::<_, i32>(66)?.into(), row.get::<_, i32>(67)?.into(), row.get::<_, i32>(68)?.into(),             ],
            effect_dice_per_level: [row.get::<_, f32>(69)?.into(), row.get::<_, f32>(70)?.into(), row.get::<_, f32>(71)?.into(),             ],
            effect_real_points_per_level: [row.get::<_, f32>(72)?.into(), row.get::<_, f32>(73)?.into(), row.get::<_, f32>(74)?.into(),             ],
            effect_base_points: [row.get::<_, i32>(75)?.into(), row.get::<_, i32>(76)?.into(), row.get::<_, i32>(77)?.into(),             ],
            effect_mechanic: [row.get::<_, u32>(78)?.into(), row.get::<_, u32>(79)?.into(), row.get::<_, u32>(80)?.into(),             ],
            implicit_target_a: [row.get::<_, i32>(81)?.into(), row.get::<_, i32>(82)?.into(), row.get::<_, i32>(83)?.into(),             ],
            implicit_target_b: [row.get::<_, i32>(84)?.into(), row.get::<_, i32>(85)?.into(), row.get::<_, i32>(86)?.into(),             ],
            effect_radius: [row.get::<_, u32>(87)?.into(), row.get::<_, u32>(88)?.into(), row.get::<_, u32>(89)?.into(),             ],
            effect_aura: [            spell::AuraMod::from_int(row.get::<_, i32>(90)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            spell::AuraMod::from_int(row.get::<_, i32>(91)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            spell::AuraMod::from_int(row.get::<_, i32>(92)? as u32).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            ],
            effect_amplitude: [row.get::<_, f32>(93)?.into(), row.get::<_, f32>(94)?.into(), row.get::<_, f32>(95)?.into(),             ],
            effect_multiple_values: [row.get::<_, f32>(96)?.into(), row.get::<_, f32>(97)?.into(), row.get::<_, f32>(98)?.into(),             ],
            effect_chain_target: [row.get::<_, i32>(99)?.into(), row.get::<_, i32>(100)?.into(), row.get::<_, i32>(101)?.into(),             ],
            effect_item_type: [row.get::<_, i32>(102)?.into(), row.get::<_, i32>(103)?.into(), row.get::<_, i32>(104)?.into(),             ],
            effect_misc_value: [row.get::<_, u32>(105)?.into(), row.get::<_, u32>(106)?.into(), row.get::<_, u32>(107)?.into(),             ],
            effect_trigger_spell: [row.get::<_, u32>(108)?.into(), row.get::<_, u32>(109)?.into(), row.get::<_, u32>(110)?.into(),             ],
            effect_points_per_combo: [row.get::<_, f32>(111)?.into(), row.get::<_, f32>(112)?.into(), row.get::<_, f32>(113)?.into(),             ],
            spell_visual: [row.get::<_, i32>(114)?.into(), row.get::<_, i32>(115)?.into(),             ],
            spell_icon: row.get::<_, u32>(116)?.into(),
            active_icon: row.get::<_, i32>(117)?.into(),
            spell_priority: row.get::<_, i32>(118)?.into(),
            unknown_flag: row.get::<_, i32>(119)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(120)?.into(),
                ko_kr: row.get::<_, String>(121)?.into(),
                fr_fr: row.get::<_, String>(122)?.into(),
                de_de: row.get::<_, String>(123)?.into(),
                en_cn: row.get::<_, String>(124)?.into(),
                en_tw: row.get::<_, String>(125)?.into(),
                es_es: row.get::<_, String>(126)?.into(),
                es_mx: row.get::<_, String>(127)?.into(),
                flags: row.get::<_, u32>(128)?.into(),
            },
            name_subtext: LocalizedString {
                en_gb: row.get::<_, String>(129)?.into(),
                ko_kr: row.get::<_, String>(130)?.into(),
                fr_fr: row.get::<_, String>(131)?.into(),
                de_de: row.get::<_, String>(132)?.into(),
                en_cn: row.get::<_, String>(133)?.into(),
                en_tw: row.get::<_, String>(134)?.into(),
                es_es: row.get::<_, String>(135)?.into(),
                es_mx: row.get::<_, String>(136)?.into(),
                flags: row.get::<_, u32>(137)?.into(),
            },
            description: LocalizedString {
                en_gb: row.get::<_, String>(138)?.into(),
                ko_kr: row.get::<_, String>(139)?.into(),
                fr_fr: row.get::<_, String>(140)?.into(),
                de_de: row.get::<_, String>(141)?.into(),
                en_cn: row.get::<_, String>(142)?.into(),
                en_tw: row.get::<_, String>(143)?.into(),
                es_es: row.get::<_, String>(144)?.into(),
                es_mx: row.get::<_, String>(145)?.into(),
                flags: row.get::<_, u32>(146)?.into(),
            },
            aura_description: LocalizedString {
                en_gb: row.get::<_, String>(147)?.into(),
                ko_kr: row.get::<_, String>(148)?.into(),
                fr_fr: row.get::<_, String>(149)?.into(),
                de_de: row.get::<_, String>(150)?.into(),
                en_cn: row.get::<_, String>(151)?.into(),
                en_tw: row.get::<_, String>(152)?.into(),
                es_es: row.get::<_, String>(153)?.into(),
                es_mx: row.get::<_, String>(154)?.into(),
                flags: row.get::<_, u32>(155)?.into(),
            },
            mana_cost_percent: row.get::<_, i32>(156)?.into(),
            start_recovery_category: row.get::<_, i32>(157)?.into(),
            start_recovery_time: row.get::<_, i32>(158)?.into(),
            max_target_level: row.get::<_, i32>(159)?.into(),
            spell_class_set: row.get::<_, u32>(160)?.into(),
            spell_class_mask: [row.get::<_, i32>(161)?.into(), row.get::<_, i32>(162)?.into(),             ],
            max_targets: row.get::<_, i32>(163)?.into(),
            defence_type: row.get::<_, i32>(164)?.into(),
            prevention_type: row.get::<_, i32>(165)?.into(),
            stance_bar_order: row.get::<_, i32>(166)?.into(),
            damage_multiplier: [row.get::<_, f32>(167)?.into(), row.get::<_, f32>(168)?.into(), row.get::<_, f32>(169)?.into(),             ],
            min_faction: row.get::<_, i32>(170)?.into(),
            min_reputation: row.get::<_, i32>(171)?.into(),
            required_aura_vision: row.get::<_, i32>(172)?.into(),
        });
    }
    Ok(spell::Spell { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellAuraNames() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellAuraNames`;"
    )
}


pub(crate) fn spell_aura_names_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_aura_names::SpellAuraNames, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_aura_names::SpellAuraNamesRow {
            id: row.get::<_, u32>(0)?.into(),
            unknown: row.get::<_, u32>(1)?.into(),
            internal_name: row.get::<_, String>(2)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(3)?.into(),
                ko_kr: row.get::<_, String>(4)?.into(),
                fr_fr: row.get::<_, String>(5)?.into(),
                de_de: row.get::<_, String>(6)?.into(),
                en_cn: row.get::<_, String>(7)?.into(),
                en_tw: row.get::<_, String>(8)?.into(),
                es_es: row.get::<_, String>(9)?.into(),
                es_mx: row.get::<_, String>(10)?.into(),
                flags: row.get::<_, u32>(11)?.into(),
            },
        });
    }
    Ok(spell_aura_names::SpellAuraNames { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellCastTimes() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        base,
        per_level_increase,
        minimum
    FROM `SpellCastTimes`;"
    )
}


pub(crate) fn spell_cast_times_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_cast_times::SpellCastTimes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_cast_times::SpellCastTimesRow {
            id: row.get::<_, u32>(0)?.into(),
            base: row.get::<_, i32>(1)?.into(),
            per_level_increase: row.get::<_, i32>(2)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
    ,
    "SELECT
        id,
        average_seg_len,
        width,
        noise_scale,
        tex_coord_scale,
        seg_duration,
        seg_delay,
        texture
    FROM `SpellChainEffects`;"
    )
}


pub(crate) fn spell_chain_effects_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_chain_effects::SpellChainEffects, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_chain_effects::SpellChainEffectsRow {
            id: row.get::<_, u32>(0)?.into(),
            average_seg_len: row.get::<_, f32>(1)?.into(),
            width: row.get::<_, f32>(2)?.into(),
            noise_scale: row.get::<_, f32>(3)?.into(),
            tex_coord_scale: row.get::<_, f32>(4)?.into(),
            seg_duration: row.get::<_, i32>(5)?.into(),
            seg_delay: row.get::<_, i32>(6)?.into(),
            texture: row.get::<_, String>(7)?.into(),
        });
    }
    Ok(spell_chain_effects::SpellChainEffects { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellDispelType() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SpellDispelType`;"
    )
}


pub(crate) fn spell_dispel_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_dispel_type::SpellDispelType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_dispel_type::SpellDispelTypeRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            mask: row.get::<_, i32>(10)?.into(),
            immunity_possible: row.get::<_, u32>(11)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
            camera_shake: [row.get::<_, u32>(1)?.into(), row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(),             ],
        });
    }
    Ok(spell_effect_camera_shakes::SpellEffectCameraShakes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellEffectNames() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellEffectNames`;"
    )
}


pub(crate) fn spell_effect_names_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_effect_names::SpellEffectNames, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_effect_names::SpellEffectNamesRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
        });
    }
    Ok(spell_effect_names::SpellEffectNames { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellFocusObject() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellFocusObject`;"
    )
}


pub(crate) fn spell_focus_object_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_focus_object::SpellFocusObject, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_focus_object::SpellFocusObjectRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
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
    ,
    "SELECT
        id,
        texture_file
    FROM `SpellIcon`;"
    )
}


pub(crate) fn spell_icon_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_icon::SpellIcon, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_icon::SpellIconRow {
            id: row.get::<_, u32>(0)?.into(),
            texture_file: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(spell_icon::SpellIcon { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellItemEnchantment() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SpellItemEnchantment`;"
    )
}


pub(crate) fn spell_item_enchantment_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_item_enchantment::SpellItemEnchantment, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_item_enchantment::SpellItemEnchantmentRow {
            id: row.get::<_, u32>(0)?.into(),
            enchantment_type: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
            effect_points_min: [row.get::<_, i32>(4)?.into(), row.get::<_, i32>(5)?.into(), row.get::<_, i32>(6)?.into(),             ],
            effect_points_max: [row.get::<_, i32>(7)?.into(), row.get::<_, i32>(8)?.into(), row.get::<_, i32>(9)?.into(),             ],
            effect_arg: [row.get::<_, i32>(10)?.into(), row.get::<_, i32>(11)?.into(), row.get::<_, i32>(12)?.into(),             ],
            name: LocalizedString {
                en_gb: row.get::<_, String>(13)?.into(),
                ko_kr: row.get::<_, String>(14)?.into(),
                fr_fr: row.get::<_, String>(15)?.into(),
                de_de: row.get::<_, String>(16)?.into(),
                en_cn: row.get::<_, String>(17)?.into(),
                en_tw: row.get::<_, String>(18)?.into(),
                es_es: row.get::<_, String>(19)?.into(),
                es_mx: row.get::<_, String>(20)?.into(),
                flags: row.get::<_, u32>(21)?.into(),
            },
            item_visual: row.get::<_, u32>(22)?.into(),
            flags: row.get::<_, i32>(23)?.into(),
        });
    }
    Ok(spell_item_enchantment::SpellItemEnchantment { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellMechanic() -> (&'static str, &'static str, &'static str) {
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
        state_name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellMechanic`;"
    )
}


pub(crate) fn spell_mechanic_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_mechanic::SpellMechanic, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_mechanic::SpellMechanicRow {
            id: row.get::<_, u32>(0)?.into(),
            state_name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
        });
    }
    Ok(spell_mechanic::SpellMechanic { rows: data })
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
            id: row.get::<_, u32>(0)?.into(),
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
        display_name_en_gb TEXT NOT NULL,
        display_name_ko_kr TEXT NOT NULL,
        display_name_fr_fr TEXT NOT NULL,
        display_name_de_de TEXT NOT NULL,
        display_name_en_cn TEXT NOT NULL,
        display_name_en_tw TEXT NOT NULL,
        display_name_es_es TEXT NOT NULL,
        display_name_es_mx TEXT NOT NULL,
        display_name_flags INTEGER NOT NULL,
        display_name_short_en_gb TEXT NOT NULL,
        display_name_short_ko_kr TEXT NOT NULL,
        display_name_short_fr_fr TEXT NOT NULL,
        display_name_short_de_de TEXT NOT NULL,
        display_name_short_en_cn TEXT NOT NULL,
        display_name_short_en_tw TEXT NOT NULL,
        display_name_short_es_es TEXT NOT NULL,
        display_name_short_es_mx TEXT NOT NULL,
        display_name_short_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `SpellRange`;"
    )
}


pub(crate) fn spell_range_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_range::SpellRange, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_range::SpellRangeRow {
            id: row.get::<_, u32>(0)?.into(),
            range_min: row.get::<_, f32>(1)?.into(),
            range_max: row.get::<_, f32>(2)?.into(),
            flags: row.get::<_, i32>(3)?.into(),
            display_name: LocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                flags: row.get::<_, u32>(12)?.into(),
            },
            display_name_short: LocalizedString {
                en_gb: row.get::<_, String>(13)?.into(),
                ko_kr: row.get::<_, String>(14)?.into(),
                fr_fr: row.get::<_, String>(15)?.into(),
                de_de: row.get::<_, String>(16)?.into(),
                en_cn: row.get::<_, String>(17)?.into(),
                en_tw: row.get::<_, String>(18)?.into(),
                es_es: row.get::<_, String>(19)?.into(),
                es_mx: row.get::<_, String>(20)?.into(),
                flags: row.get::<_, u32>(21)?.into(),
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
        name_en_gb TEXT NOT NULL,
        name_ko_kr TEXT NOT NULL,
        name_fr_fr TEXT NOT NULL,
        name_de_de TEXT NOT NULL,
        name_en_cn TEXT NOT NULL,
        name_en_tw TEXT NOT NULL,
        name_es_es TEXT NOT NULL,
        name_es_mx TEXT NOT NULL,
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `SpellShapeshiftForm`;"
    )
}


pub(crate) fn spell_shapeshift_form_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_shapeshift_form::SpellShapeshiftForm, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_shapeshift_form::SpellShapeshiftFormRow {
            id: row.get::<_, u32>(0)?.into(),
            bonus_action_bar: row.get::<_, i32>(1)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                flags: row.get::<_, u32>(10)?.into(),
            },
            flags: row.get::<_, i32>(11)?.into(),
            creature_type: row.get::<_, i32>(12)?.into(),
            spell_icon: row.get::<_, u32>(13)?.into(),
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
    ,
    "SELECT
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
    FROM `SpellVisual`;"
    )
}


pub(crate) fn spell_visual_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual::SpellVisual, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual::SpellVisualRow {
            id: row.get::<_, u32>(0)?.into(),
            precast_kit: row.get::<_, u32>(1)?.into(),
            cast_kit: row.get::<_, u32>(2)?.into(),
            impact_kit: row.get::<_, u32>(3)?.into(),
            state_kit: row.get::<_, u32>(4)?.into(),
            state_done_kit: row.get::<_, u32>(5)?.into(),
            channel_kit: row.get::<_, u32>(6)?.into(),
            has_missile: row.get::<_, u32>(7)?.into(),
            missile_model: row.get::<_, i32>(8)?.into(),
            missile_path_type: row.get::<_, i32>(9)?.into(),
            missile_destination_attachment: row.get::<_, i32>(10)?.into(),
            missile_sound: row.get::<_, u32>(11)?.into(),
            anim_event_sound: row.get::<_, u32>(12)?.into(),
            flags: row.get::<_, i32>(13)?.into(),
            caster_impact_kit: row.get::<_, u32>(14)?.into(),
            target_impact_kit: row.get::<_, u32>(15)?.into(),
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
    ,
    "SELECT
        id,
        name,
        filename,
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
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            filename: row.get::<_, String>(2)?.into(),
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
    ,
    "SELECT
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
    FROM `SpellVisualKit`;"
    )
}


pub(crate) fn spell_visual_kit_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual_kit::SpellVisualKit, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual_kit::SpellVisualKitRow {
            id: row.get::<_, u32>(0)?.into(),
            start_anim: row.get::<_, u32>(1)?.into(),
            anim: row.get::<_, u32>(2)?.into(),
            head_effect: row.get::<_, u32>(3)?.into(),
            chest_effect: row.get::<_, u32>(4)?.into(),
            base_effect: row.get::<_, u32>(5)?.into(),
            left_hand_effect: row.get::<_, u32>(6)?.into(),
            right_hand_effect: row.get::<_, u32>(7)?.into(),
            breath_effect: row.get::<_, u32>(8)?.into(),
            left_weapon_effect: row.get::<_, u32>(9)?.into(),
            right_weapon_effect: row.get::<_, u32>(10)?.into(),
            special_effects: [row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(), row.get::<_, u32>(13)?.into(),             ],
            world_effect: row.get::<_, u32>(14)?.into(),
            sound: row.get::<_, u32>(15)?.into(),
            shake: row.get::<_, u32>(16)?.into(),
            char_proc: [row.get::<_, u32>(17)?.into(), row.get::<_, u32>(18)?.into(), row.get::<_, u32>(19)?.into(), row.get::<_, u32>(20)?.into(),             ],
            char_param_zero: [row.get::<_, f32>(21)?.into(), row.get::<_, f32>(22)?.into(), row.get::<_, f32>(23)?.into(), row.get::<_, f32>(24)?.into(),             ],
            char_param_one: [row.get::<_, f32>(25)?.into(), row.get::<_, f32>(26)?.into(), row.get::<_, f32>(27)?.into(), row.get::<_, f32>(28)?.into(),             ],
            char_param_two: [row.get::<_, f32>(29)?.into(), row.get::<_, f32>(30)?.into(), row.get::<_, f32>(31)?.into(), row.get::<_, f32>(32)?.into(),             ],
            unknown1_pad: row.get::<_, u32>(33)?.into(),
            unknown2_pad: row.get::<_, u32>(34)?.into(),
        });
    }
    Ok(spell_visual_kit::SpellVisualKit { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn SpellVisualPrecastTransitions() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        load_animation,
        hold_animation
    FROM `SpellVisualPrecastTransitions`;"
    )
}


pub(crate) fn spell_visual_precast_transitions_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<spell_visual_precast_transitions::SpellVisualPrecastTransitions, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(spell_visual_precast_transitions::SpellVisualPrecastTransitionsRow {
            id: row.get::<_, u32>(0)?.into(),
            load_animation: row.get::<_, String>(1)?.into(),
            hold_animation: row.get::<_, String>(2)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
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
        startup_string_en_gb TEXT NOT NULL,
        startup_string_ko_kr TEXT NOT NULL,
        startup_string_fr_fr TEXT NOT NULL,
        startup_string_de_de TEXT NOT NULL,
        startup_string_en_cn TEXT NOT NULL,
        startup_string_en_tw TEXT NOT NULL,
        startup_string_es_es TEXT NOT NULL,
        startup_string_es_mx TEXT NOT NULL,
        startup_string_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `Startup_Strings`;"
    )
}


pub(crate) fn startup_strings_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<startup_strings::Startup_Strings, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(startup_strings::Startup_StringsRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            startup_string: LocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                flags: row.get::<_, u32>(10)?.into(),
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
    ,
    "SELECT
        id,
        item,
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
            id: row.get::<_, u32>(0)?.into(),
            item: row.get::<_, u32>(1)?.into(),
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
    ,
    "SELECT
        id,
        path
    FROM `StringLookups`;"
    )
}


pub(crate) fn string_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<string_lookups::StringLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(string_lookups::StringLookupsRow {
            id: row.get::<_, u32>(0)?.into(),
            path: row.get::<_, String>(1)?.into(),
        });
    }
    Ok(string_lookups::StringLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn Talent() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `Talent`;"
    )
}


pub(crate) fn talent_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<talent::Talent, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(talent::TalentRow {
            id: row.get::<_, u32>(0)?.into(),
            tab: row.get::<_, u32>(1)?.into(),
            tier: row.get::<_, i32>(2)?.into(),
            column_index: row.get::<_, i32>(3)?.into(),
            spell_rank: [row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(),             ],
            prereq_talents: [row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(),             ],
            prereq_ranks: [row.get::<_, i32>(16)?.into(), row.get::<_, i32>(17)?.into(), row.get::<_, i32>(18)?.into(),             ],
            flags: row.get::<_, i32>(19)?.into(),
            required_spell: row.get::<_, u32>(20)?.into(),
        });
    }
    Ok(talent::Talent { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TalentTab() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `TalentTab`;"
    )
}


pub(crate) fn talent_tab_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<talent_tab::TalentTab, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(talent_tab::TalentTabRow {
            id: row.get::<_, u32>(0)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(1)?.into(),
                ko_kr: row.get::<_, String>(2)?.into(),
                fr_fr: row.get::<_, String>(3)?.into(),
                de_de: row.get::<_, String>(4)?.into(),
                en_cn: row.get::<_, String>(5)?.into(),
                en_tw: row.get::<_, String>(6)?.into(),
                es_es: row.get::<_, String>(7)?.into(),
                es_mx: row.get::<_, String>(8)?.into(),
                flags: row.get::<_, u32>(9)?.into(),
            },
            spell_icon: row.get::<_, u32>(10)?.into(),
            race_mask: row.get::<_, u32>(11)?.into(),
            class_mask: row.get::<_, u32>(12)?.into(),
            order_index: row.get::<_, u32>(13)?.into(),
            background_file: row.get::<_, String>(14)?.into(),
        });
    }
    Ok(talent_tab::TalentTab { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TaxiNodes() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `TaxiNodes`;"
    )
}


pub(crate) fn taxi_nodes_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<taxi_nodes::TaxiNodes, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(taxi_nodes::TaxiNodesRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            location_x: row.get::<_, f32>(2)?.into(),
            location_y: row.get::<_, f32>(3)?.into(),
            location_z: row.get::<_, f32>(4)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                flags: row.get::<_, u32>(13)?.into(),
            },
            mount_creature_display_info: [row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(),             ],
        });
    }
    Ok(taxi_nodes::TaxiNodes { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TaxiPath() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        source_taxi_node,
        destination_taxi_node,
        cost
    FROM `TaxiPath`;"
    )
}


pub(crate) fn taxi_path_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<taxi_path::TaxiPath, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(taxi_path::TaxiPathRow {
            id: row.get::<_, u32>(0)?.into(),
            source_taxi_node: row.get::<_, u32>(1)?.into(),
            destination_taxi_node: row.get::<_, u32>(2)?.into(),
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
    ,
    "SELECT
        id,
        taxi_path,
        node_index,
        map,
        location_x,
        location_y,
        location_z,
        flags,
        delay
    FROM `TaxiPathNode`;"
    )
}


pub(crate) fn taxi_path_node_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<taxi_path_node::TaxiPathNode, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(taxi_path_node::TaxiPathNodeRow {
            id: row.get::<_, u32>(0)?.into(),
            taxi_path: row.get::<_, u32>(1)?.into(),
            node_index: row.get::<_, i32>(2)?.into(),
            map: row.get::<_, u32>(3)?.into(),
            location_x: row.get::<_, f32>(4)?.into(),
            location_y: row.get::<_, f32>(5)?.into(),
            location_z: row.get::<_, f32>(6)?.into(),
            flags: row.get::<_, i32>(7)?.into(),
            delay: row.get::<_, i32>(8)?.into(),
        });
    }
    Ok(taxi_path_node::TaxiPathNode { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TerrainType() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        description,
        footstep_spray_run,
        footstep_spray_walk,
        sound,
        display_footsteps
    FROM `TerrainType`;"
    )
}


pub(crate) fn terrain_type_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<terrain_type::TerrainType, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(terrain_type::TerrainTypeRow {
            id: row.get::<_, u32>(0)?.into(),
            description: row.get::<_, String>(1)?.into(),
            footstep_spray_run: row.get::<_, u32>(2)?.into(),
            footstep_spray_walk: row.get::<_, u32>(3)?.into(),
            sound: row.get::<_, u32>(4)?.into(),
            display_footsteps: row.get::<_, bool>(5)?.into(),
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
            id: row.get::<_, u32>(0)?.into(),
        });
    }
    Ok(terrain_type_sounds::TerrainTypeSounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn TransportAnimation() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        transport,
        time_index,
        location_x,
        location_y,
        location_z,
        sequence
    FROM `TransportAnimation`;"
    )
}


pub(crate) fn transport_animation_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<transport_animation::TransportAnimation, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(transport_animation::TransportAnimationRow {
            id: row.get::<_, u32>(0)?.into(),
            transport: row.get::<_, u32>(1)?.into(),
            time_index: row.get::<_, i32>(2)?.into(),
            location_x: row.get::<_, f32>(3)?.into(),
            location_y: row.get::<_, f32>(4)?.into(),
            location_z: row.get::<_, f32>(5)?.into(),
            sequence: row.get::<_, u32>(6)?.into(),
        });
    }
    Ok(transport_animation::TransportAnimation { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn UISoundLookups() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        sound_entry,
        internal_name
    FROM `UISoundLookups`;"
    )
}


pub(crate) fn ui_sound_lookups_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<ui_sound_lookups::UISoundLookups, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(ui_sound_lookups::UISoundLookupsRow {
            id: row.get::<_, u32>(0)?.into(),
            sound_entry: row.get::<_, u32>(1)?.into(),
            internal_name: row.get::<_, String>(2)?.into(),
        });
    }
    Ok(ui_sound_lookups::UISoundLookups { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn UnitBlood() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `UnitBlood`;"
    )
}


pub(crate) fn unit_blood_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<unit_blood::UnitBlood, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(unit_blood::UnitBloodRow {
            id: row.get::<_, u32>(0)?.into(),
            combat_blood_spurt_front_small: row.get::<_, i32>(1)?.into(),
            combat_blood_spurt_front_large: row.get::<_, i32>(2)?.into(),
            combat_blood_spurt_back_small: row.get::<_, i32>(3)?.into(),
            combat_blood_spurt_back_large: row.get::<_, i32>(4)?.into(),
            texture: [row.get::<_, String>(5)?.into(), row.get::<_, String>(6)?.into(), row.get::<_, String>(7)?.into(), row.get::<_, String>(8)?.into(), row.get::<_, String>(9)?.into(),             ],
        });
    }
    Ok(unit_blood::UnitBlood { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn UnitBloodLevels() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        violence_level_0,
        violence_level_1,
        violence_level_2
    FROM `UnitBloodLevels`;"
    )
}


pub(crate) fn unit_blood_levels_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<unit_blood_levels::UnitBloodLevels, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(unit_blood_levels::UnitBloodLevelsRow {
            id: row.get::<_, u32>(0)?.into(),
            violence_level: [row.get::<_, i32>(1)?.into(), row.get::<_, i32>(2)?.into(), row.get::<_, i32>(3)?.into(),             ],
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
        multisample
    FROM `VideoHardware`;"
    )
}


pub(crate) fn video_hardware_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<video_hardware::VideoHardware, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(video_hardware::VideoHardwareRow {
            id: row.get::<_, u32>(0)?.into(),
            vendor_id: row.get::<_, u32>(1)?.into(),
            device_id: row.get::<_, u32>(2)?.into(),
            farclip_idx: row.get::<_, u32>(3)?.into(),
            terrain_l_o_d_dist_idx: row.get::<_, u32>(4)?.into(),
            terrain_shadow_l_o_d: row.get::<_, u32>(5)?.into(),
            detail_doodad_density_idx: row.get::<_, u32>(6)?.into(),
            detail_doodad_alpha: row.get::<_, u32>(7)?.into(),
            animating_doodad_idx: row.get::<_, u32>(8)?.into(),
            trilinear: row.get::<_, u32>(9)?.into(),
            num_lights: row.get::<_, u32>(10)?.into(),
            specularity: row.get::<_, u32>(11)?.into(),
            water_l_o_d_idx: row.get::<_, u32>(12)?.into(),
            particle_density_idx: row.get::<_, u32>(13)?.into(),
            unit_draw_dist_idx: row.get::<_, u32>(14)?.into(),
            small_cull_dist_idx: row.get::<_, u32>(15)?.into(),
            resolution_idx: row.get::<_, u32>(16)?.into(),
            base_mip_level: row.get::<_, u32>(17)?.into(),
            ogl_overrides: row.get::<_, String>(18)?.into(),
            d3d_overrides: row.get::<_, String>(19)?.into(),
            fix_lag: row.get::<_, u32>(20)?.into(),
            multisample: row.get::<_, u32>(21)?.into(),
        });
    }
    Ok(video_hardware::VideoHardware { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn VocalUISounds() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        vocal_ui_enum,
        race,
        normal_male_sound,
        normal_female_sound,
        pissed_male_sound,
        pissed_female_sound
    FROM `VocalUISounds`;"
    )
}


pub(crate) fn vocal_ui_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<vocal_ui_sounds::VocalUISounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(vocal_ui_sounds::VocalUISoundsRow {
            id: row.get::<_, u32>(0)?.into(),
            vocal_ui_enum: row.get::<_, i32>(1)?.into(),
            race: row.get::<_, u32>(2)?.into(),
            normal_male_sound: row.get::<_, u32>(3)?.into(),
            normal_female_sound: row.get::<_, u32>(4)?.into(),
            pissed_male_sound: row.get::<_, u32>(5)?.into(),
            pissed_female_sound: row.get::<_, u32>(6)?.into(),
        });
    }
    Ok(vocal_ui_sounds::VocalUISounds { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WMOAreaTable() -> (&'static str, &'static str, &'static str) {
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
        name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `WMOAreaTable`;"
    )
}


pub(crate) fn wmo_area_table_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<wmo_area_table::WMOAreaTable, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(wmo_area_table::WMOAreaTableRow {
            id: row.get::<_, u32>(0)?.into(),
            wmo_id: row.get::<_, u32>(1)?.into(),
            name_set_id: row.get::<_, u32>(2)?.into(),
            wmo_group_id: row.get::<_, i32>(3)?.into(),
            sound_provider_preferences: row.get::<_, u32>(4)?.into(),
            sound_provider_preferences_underwater: row.get::<_, u32>(5)?.into(),
            sound_ambience: row.get::<_, u32>(6)?.into(),
            zone_music: row.get::<_, u32>(7)?.into(),
            zone_intro_music: row.get::<_, u32>(8)?.into(),
            flags: row.get::<_, u32>(9)?.into(),
            area_table: row.get::<_, u32>(10)?.into(),
            name: LocalizedString {
                en_gb: row.get::<_, String>(11)?.into(),
                ko_kr: row.get::<_, String>(12)?.into(),
                fr_fr: row.get::<_, String>(13)?.into(),
                de_de: row.get::<_, String>(14)?.into(),
                en_cn: row.get::<_, String>(15)?.into(),
                en_tw: row.get::<_, String>(16)?.into(),
                es_es: row.get::<_, String>(17)?.into(),
                es_mx: row.get::<_, String>(18)?.into(),
                flags: row.get::<_, u32>(19)?.into(),
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
    ,
    "SELECT
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
    FROM `WeaponImpactSounds`;"
    )
}


pub(crate) fn weapon_impact_sounds_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<weapon_impact_sounds::WeaponImpactSounds, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(weapon_impact_sounds::WeaponImpactSoundsRow {
            id: row.get::<_, u32>(0)?.into(),
            weapon_subclass: row.get::<_, i32>(1)?.into(),
            parry_sound_type: row.get::<_, i32>(2)?.into(),
            impact_sound: [row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(), row.get::<_, u32>(6)?.into(), row.get::<_, u32>(7)?.into(), row.get::<_, u32>(8)?.into(), row.get::<_, u32>(9)?.into(), row.get::<_, u32>(10)?.into(), row.get::<_, u32>(11)?.into(), row.get::<_, u32>(12)?.into(),             ],
            crit_impact_sound: [row.get::<_, u32>(13)?.into(), row.get::<_, u32>(14)?.into(), row.get::<_, u32>(15)?.into(), row.get::<_, u32>(16)?.into(), row.get::<_, u32>(17)?.into(), row.get::<_, u32>(18)?.into(), row.get::<_, u32>(19)?.into(), row.get::<_, u32>(20)?.into(), row.get::<_, u32>(21)?.into(), row.get::<_, u32>(22)?.into(),             ],
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
    ,
    "SELECT
        id,
        swing_type,
        critical,
        sound
    FROM `WeaponSwingSounds2`;"
    )
}


pub(crate) fn weapon_swing_sounds2_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<weapon_swing_sounds2::WeaponSwingSounds2, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(weapon_swing_sounds2::WeaponSwingSounds2Row {
            id: row.get::<_, u32>(0)?.into(),
            swing_type: weapon_swing_sounds2::SwingType::from_int(row.get::<_, i32>(1)? as u8).map_err(|e| SqliteError::EnumError(e.to_string()))?,
            critical: row.get::<_, bool>(2)?.into(),
            sound: row.get::<_, u32>(3)?.into(),
        });
    }
    Ok(weapon_swing_sounds2::WeaponSwingSounds2 { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapArea() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
        id,
        world_map_continent,
        area_table,
        area_name,
        location_left,
        location_right,
        location_top,
        location_bottom
    FROM `WorldMapArea`;"
    )
}


pub(crate) fn world_map_area_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_area::WorldMapArea, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_area::WorldMapAreaRow {
            id: row.get::<_, u32>(0)?.into(),
            world_map_continent: row.get::<_, u32>(1)?.into(),
            area_table: row.get::<_, u32>(2)?.into(),
            area_name: row.get::<_, String>(3)?.into(),
            location_left: row.get::<_, f32>(4)?.into(),
            location_right: row.get::<_, f32>(5)?.into(),
            location_top: row.get::<_, f32>(6)?.into(),
            location_bottom: row.get::<_, f32>(7)?.into(),
        });
    }
    Ok(world_map_area::WorldMapArea { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapContinent() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `WorldMapContinent`;"
    )
}


pub(crate) fn world_map_continent_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_continent::WorldMapContinent, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_continent::WorldMapContinentRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            left_boundary: row.get::<_, u32>(2)?.into(),
            right_boundary: row.get::<_, u32>(3)?.into(),
            top_boundary: row.get::<_, u32>(4)?.into(),
            bottom_boundary: row.get::<_, u32>(5)?.into(),
            continent_offset_x: row.get::<_, f32>(6)?.into(),
            continent_offset_y: row.get::<_, f32>(7)?.into(),
            scale: row.get::<_, f32>(8)?.into(),
            taxi_min_x: row.get::<_, f32>(9)?.into(),
            taxi_min_y: row.get::<_, f32>(10)?.into(),
            taxi_max_x: row.get::<_, f32>(11)?.into(),
            taxi_max_y: row.get::<_, f32>(12)?.into(),
        });
    }
    Ok(world_map_continent::WorldMapContinent { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldMapOverlay() -> (&'static str, &'static str, &'static str) {
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
    ,
    "SELECT
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
    FROM `WorldMapOverlay`;"
    )
}


pub(crate) fn world_map_overlay_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_map_overlay::WorldMapOverlay, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_map_overlay::WorldMapOverlayRow {
            id: row.get::<_, u32>(0)?.into(),
            world_map_area: row.get::<_, u32>(1)?.into(),
            area_table: [row.get::<_, u32>(2)?.into(), row.get::<_, u32>(3)?.into(), row.get::<_, u32>(4)?.into(), row.get::<_, u32>(5)?.into(),             ],
            location_x: row.get::<_, u32>(6)?.into(),
            location_y: row.get::<_, u32>(7)?.into(),
            texture_name: row.get::<_, String>(8)?.into(),
            texture_width: row.get::<_, u32>(9)?.into(),
            texture_height: row.get::<_, u32>(10)?.into(),
            offset_x: row.get::<_, u32>(11)?.into(),
            offset_y: row.get::<_, u32>(12)?.into(),
            hit_rect_top: row.get::<_, u32>(13)?.into(),
            hit_rect_left: row.get::<_, u32>(14)?.into(),
            hit_rect_bottom: row.get::<_, u32>(15)?.into(),
            hit_rect_right: row.get::<_, u32>(16)?.into(),
        });
    }
    Ok(world_map_overlay::WorldMapOverlay { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WorldSafeLocs() -> (&'static str, &'static str, &'static str) {
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
        area_name_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `WorldSafeLocs`;"
    )
}


pub(crate) fn world_safe_locs_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_safe_locs::WorldSafeLocs, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_safe_locs::WorldSafeLocsRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            location_x: row.get::<_, f32>(2)?.into(),
            location_y: row.get::<_, f32>(3)?.into(),
            location_z: row.get::<_, f32>(4)?.into(),
            area_name: LocalizedString {
                en_gb: row.get::<_, String>(5)?.into(),
                ko_kr: row.get::<_, String>(6)?.into(),
                fr_fr: row.get::<_, String>(7)?.into(),
                de_de: row.get::<_, String>(8)?.into(),
                en_cn: row.get::<_, String>(9)?.into(),
                en_tw: row.get::<_, String>(10)?.into(),
                es_es: row.get::<_, String>(11)?.into(),
                es_mx: row.get::<_, String>(12)?.into(),
                flags: row.get::<_, u32>(13)?.into(),
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
        state_variable_flags INTEGER NOT NULL,
        tooltip_en_gb TEXT NOT NULL,
        tooltip_ko_kr TEXT NOT NULL,
        tooltip_fr_fr TEXT NOT NULL,
        tooltip_de_de TEXT NOT NULL,
        tooltip_en_cn TEXT NOT NULL,
        tooltip_en_tw TEXT NOT NULL,
        tooltip_es_es TEXT NOT NULL,
        tooltip_es_mx TEXT NOT NULL,
        tooltip_flags INTEGER NOT NULL,
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
        dynamic_tooltip_flags INTEGER NOT NULL,
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
    ,
    "SELECT
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
    FROM `WorldStateUI`;"
    )
}


pub(crate) fn world_state_ui_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<world_state_ui::WorldStateUI, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(world_state_ui::WorldStateUIRow {
            id: row.get::<_, u32>(0)?.into(),
            map: row.get::<_, u32>(1)?.into(),
            area_table: row.get::<_, u32>(2)?.into(),
            icon: row.get::<_, String>(3)?.into(),
            state_variable: LocalizedString {
                en_gb: row.get::<_, String>(4)?.into(),
                ko_kr: row.get::<_, String>(5)?.into(),
                fr_fr: row.get::<_, String>(6)?.into(),
                de_de: row.get::<_, String>(7)?.into(),
                en_cn: row.get::<_, String>(8)?.into(),
                en_tw: row.get::<_, String>(9)?.into(),
                es_es: row.get::<_, String>(10)?.into(),
                es_mx: row.get::<_, String>(11)?.into(),
                flags: row.get::<_, u32>(12)?.into(),
            },
            tooltip: LocalizedString {
                en_gb: row.get::<_, String>(13)?.into(),
                ko_kr: row.get::<_, String>(14)?.into(),
                fr_fr: row.get::<_, String>(15)?.into(),
                de_de: row.get::<_, String>(16)?.into(),
                en_cn: row.get::<_, String>(17)?.into(),
                en_tw: row.get::<_, String>(18)?.into(),
                es_es: row.get::<_, String>(19)?.into(),
                es_mx: row.get::<_, String>(20)?.into(),
                flags: row.get::<_, u32>(21)?.into(),
            },
            state: row.get::<_, i32>(22)?.into(),
            world_state: row.get::<_, u32>(23)?.into(),
            ty: row.get::<_, i32>(24)?.into(),
            dynamic_icon: row.get::<_, String>(25)?.into(),
            dynamic_tooltip: LocalizedString {
                en_gb: row.get::<_, String>(26)?.into(),
                ko_kr: row.get::<_, String>(27)?.into(),
                fr_fr: row.get::<_, String>(28)?.into(),
                de_de: row.get::<_, String>(29)?.into(),
                en_cn: row.get::<_, String>(30)?.into(),
                en_tw: row.get::<_, String>(31)?.into(),
                es_es: row.get::<_, String>(32)?.into(),
                es_mx: row.get::<_, String>(33)?.into(),
                flags: row.get::<_, u32>(34)?.into(),
            },
            extended_ui: row.get::<_, String>(35)?.into(),
            unknown: [row.get::<_, u32>(36)?.into(), row.get::<_, u32>(37)?.into(), row.get::<_, u32>(38)?.into(),             ],
        });
    }
    Ok(world_state_ui::WorldStateUI { rows: data })
}
#[allow(non_snake_case)]
pub(crate) fn WowError_Strings() -> (&'static str, &'static str, &'static str) {
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
        text_flags INTEGER NOT NULL
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
    ,
    "SELECT
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
    FROM `WowError_Strings`;"
    )
}


pub(crate) fn wow_error_strings_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<wow_error_strings::WowError_Strings, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(wow_error_strings::WowError_StringsRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            text: LocalizedString {
                en_gb: row.get::<_, String>(2)?.into(),
                ko_kr: row.get::<_, String>(3)?.into(),
                fr_fr: row.get::<_, String>(4)?.into(),
                de_de: row.get::<_, String>(5)?.into(),
                en_cn: row.get::<_, String>(6)?.into(),
                en_tw: row.get::<_, String>(7)?.into(),
                es_es: row.get::<_, String>(8)?.into(),
                es_mx: row.get::<_, String>(9)?.into(),
                flags: row.get::<_, u32>(10)?.into(),
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
    ,
    "SELECT
        id,
        name,
        intro_sound,
        priority_over_ambience,
        min_delay
    FROM `ZoneIntroMusicTable`;"
    )
}


pub(crate) fn zone_intro_music_table_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<zone_intro_music_table::ZoneIntroMusicTable, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(zone_intro_music_table::ZoneIntroMusicTableRow {
            id: row.get::<_, u32>(0)?.into(),
            name: row.get::<_, String>(1)?.into(),
            intro_sound: row.get::<_, u32>(2)?.into(),
            priority_over_ambience: row.get::<_, bool>(3)?.into(),
            min_delay: row.get::<_, i32>(4)?.into(),
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
    ,
    "SELECT
        id,
        set_name,
        silence_interval_min_day,
        silence_interval_min_night,
        silence_interval_max_day,
        silence_interval_max_night,
        day_sound,
        night_sound
    FROM `ZoneMusic`;"
    )
}


pub(crate) fn zone_music_from_rows(rows: &mut rusqlite::Rows<'_>) -> Result<zone_music::ZoneMusic, SqliteError> {
    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        let row = row;
        data.push(zone_music::ZoneMusicRow {
            id: row.get::<_, u32>(0)?.into(),
            set_name: row.get::<_, String>(1)?.into(),
            silence_interval_min_day: row.get::<_, i32>(2)?.into(),
            silence_interval_min_night: row.get::<_, i32>(3)?.into(),
            silence_interval_max_day: row.get::<_, i32>(4)?.into(),
            silence_interval_max_night: row.get::<_, i32>(5)?.into(),
            day_sound: row.get::<_, u32>(6)?.into(),
            night_sound: row.get::<_, u32>(7)?.into(),
        });
    }
    Ok(zone_music::ZoneMusic { rows: data })
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
        "CharVariations" => {
            let (_create, _insert, select) = CharVariations();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = char_variations_from_rows(&mut rows)?;
            data.write(&mut writer).map_err(|e| SqliteError::DbcError(wow_dbc::DbcError::Io(e)))?;
            Ok(())
        }
        "CharacterCreateCameras" => {
            let (_create, _insert, select) = CharacterCreateCameras();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = character_create_cameras_from_rows(&mut rows)?;
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
        "GameTips" => {
            let (_create, _insert, select) = GameTips();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = game_tips_from_rows(&mut rows)?;
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
        "ItemDisplayInfo" => {
            let (_create, _insert, select) = ItemDisplayInfo();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = item_display_info_from_rows(&mut rows)?;
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
        "SoundCharacterMacroLines" => {
            let (_create, _insert, select) = SoundCharacterMacroLines();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = sound_character_macro_lines_from_rows(&mut rows)?;
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
        "SpellAuraNames" => {
            let (_create, _insert, select) = SpellAuraNames();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_aura_names_from_rows(&mut rows)?;
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
        "SpellEffectNames" => {
            let (_create, _insert, select) = SpellEffectNames();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_effect_names_from_rows(&mut rows)?;
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
        "SpellMechanic" => {
            let (_create, _insert, select) = SpellMechanic();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = spell_mechanic_from_rows(&mut rows)?;
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
        "TransportAnimation" => {
            let (_create, _insert, select) = TransportAnimation();
            let mut stmt = conn.prepare(select)?;
            let mut rows = stmt.query([])?;
            let data = transport_animation_from_rows(&mut rows)?;
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
        _ => Err(SqliteError::FilenameNotFound { name: name.to_string() }),
    }
}

pub(crate) fn get_sql_for(name: &str) -> Result<(&'static str, &'static str, &'static str), SqliteError> {
    match name {
        "AnimationData" => Ok(AnimationData()),
        "AreaPOI" => Ok(AreaPOI()),
        "AreaTable" => Ok(AreaTable()),
        "AreaTrigger" => Ok(AreaTrigger()),
        "AttackAnimKits" => Ok(AttackAnimKits()),
        "AttackAnimTypes" => Ok(AttackAnimTypes()),
        "AuctionHouse" => Ok(AuctionHouse()),
        "BankBagSlotPrices" => Ok(BankBagSlotPrices()),
        "CameraShakes" => Ok(CameraShakes()),
        "Cfg_Categories" => Ok(Cfg_Categories()),
        "Cfg_Configs" => Ok(Cfg_Configs()),
        "CharBaseInfo" => Ok(CharBaseInfo()),
        "CharHairGeosets" => Ok(CharHairGeosets()),
        "CharHairTextures" => Ok(CharHairTextures()),
        "CharSections" => Ok(CharSections()),
        "CharStartOutfit" => Ok(CharStartOutfit()),
        "CharVariations" => Ok(CharVariations()),
        "CharacterCreateCameras" => Ok(CharacterCreateCameras()),
        "CharacterFacialHairStyles" => Ok(CharacterFacialHairStyles()),
        "ChatChannels" => Ok(ChatChannels()),
        "ChatProfanity" => Ok(ChatProfanity()),
        "ChrClasses" => Ok(ChrClasses()),
        "ChrRaces" => Ok(ChrRaces()),
        "CinematicCamera" => Ok(CinematicCamera()),
        "CinematicSequences" => Ok(CinematicSequences()),
        "CreatureDisplayInfo" => Ok(CreatureDisplayInfo()),
        "CreatureDisplayInfoExtra" => Ok(CreatureDisplayInfoExtra()),
        "CreatureFamily" => Ok(CreatureFamily()),
        "CreatureModelData" => Ok(CreatureModelData()),
        "CreatureSoundData" => Ok(CreatureSoundData()),
        "CreatureSpellData" => Ok(CreatureSpellData()),
        "CreatureType" => Ok(CreatureType()),
        "DeathThudLookups" => Ok(DeathThudLookups()),
        "DurabilityCosts" => Ok(DurabilityCosts()),
        "DurabilityQuality" => Ok(DurabilityQuality()),
        "Emotes" => Ok(Emotes()),
        "EmotesText" => Ok(EmotesText()),
        "EmotesTextData" => Ok(EmotesTextData()),
        "EmotesTextSound" => Ok(EmotesTextSound()),
        "EnvironmentalDamage" => Ok(EnvironmentalDamage()),
        "Exhaustion" => Ok(Exhaustion()),
        "Faction" => Ok(Faction()),
        "FactionGroup" => Ok(FactionGroup()),
        "FactionTemplate" => Ok(FactionTemplate()),
        "FootprintTextures" => Ok(FootprintTextures()),
        "FootstepTerrainLookup" => Ok(FootstepTerrainLookup()),
        "GMSurveyCurrentSurvey" => Ok(GMSurveyCurrentSurvey()),
        "GMSurveyQuestions" => Ok(GMSurveyQuestions()),
        "GMSurveySurveys" => Ok(GMSurveySurveys()),
        "GMTicketCategory" => Ok(GMTicketCategory()),
        "GameObjectArtKit" => Ok(GameObjectArtKit()),
        "GameObjectDisplayInfo" => Ok(GameObjectDisplayInfo()),
        "GameTips" => Ok(GameTips()),
        "GroundEffectDoodad" => Ok(GroundEffectDoodad()),
        "GroundEffectTexture" => Ok(GroundEffectTexture()),
        "HelmetGeosetVisData" => Ok(HelmetGeosetVisData()),
        "ItemBagFamily" => Ok(ItemBagFamily()),
        "ItemClass" => Ok(ItemClass()),
        "ItemDisplayInfo" => Ok(ItemDisplayInfo()),
        "ItemGroupSounds" => Ok(ItemGroupSounds()),
        "ItemPetFood" => Ok(ItemPetFood()),
        "ItemRandomProperties" => Ok(ItemRandomProperties()),
        "ItemSet" => Ok(ItemSet()),
        "ItemSubClass" => Ok(ItemSubClass()),
        "ItemSubClassMask" => Ok(ItemSubClassMask()),
        "ItemVisualEffects" => Ok(ItemVisualEffects()),
        "ItemVisuals" => Ok(ItemVisuals()),
        "LFGDungeons" => Ok(LFGDungeons()),
        "LanguageWords" => Ok(LanguageWords()),
        "Languages" => Ok(Languages()),
        "Light" => Ok(Light()),
        "LightFloatBand" => Ok(LightFloatBand()),
        "LightIntBand" => Ok(LightIntBand()),
        "LightParams" => Ok(LightParams()),
        "LightSkybox" => Ok(LightSkybox()),
        "LiquidType" => Ok(LiquidType()),
        "LoadingScreenTaxiSplines" => Ok(LoadingScreenTaxiSplines()),
        "LoadingScreens" => Ok(LoadingScreens()),
        "Lock" => Ok(Lock()),
        "LockType" => Ok(LockType()),
        "MailTemplate" => Ok(MailTemplate()),
        "Map" => Ok(Map()),
        "Material" => Ok(Material()),
        "NPCSounds" => Ok(NPCSounds()),
        "NameGen" => Ok(NameGen()),
        "NamesProfanity" => Ok(NamesProfanity()),
        "NamesReserved" => Ok(NamesReserved()),
        "Package" => Ok(Package()),
        "PageTextMaterial" => Ok(PageTextMaterial()),
        "PaperDollItemFrame" => Ok(PaperDollItemFrame()),
        "PetLoyalty" => Ok(PetLoyalty()),
        "PetPersonality" => Ok(PetPersonality()),
        "QuestInfo" => Ok(QuestInfo()),
        "QuestSort" => Ok(QuestSort()),
        "Resistances" => Ok(Resistances()),
        "ServerMessages" => Ok(ServerMessages()),
        "SheatheSoundLookups" => Ok(SheatheSoundLookups()),
        "SkillCostsData" => Ok(SkillCostsData()),
        "SkillLine" => Ok(SkillLine()),
        "SkillLineAbility" => Ok(SkillLineAbility()),
        "SkillLineCategory" => Ok(SkillLineCategory()),
        "SkillRaceClassInfo" => Ok(SkillRaceClassInfo()),
        "SkillTiers" => Ok(SkillTiers()),
        "SoundAmbience" => Ok(SoundAmbience()),
        "SoundCharacterMacroLines" => Ok(SoundCharacterMacroLines()),
        "SoundEntries" => Ok(SoundEntries()),
        "SoundProviderPreferences" => Ok(SoundProviderPreferences()),
        "SoundSamplePreferences" => Ok(SoundSamplePreferences()),
        "SoundWaterType" => Ok(SoundWaterType()),
        "SpamMessages" => Ok(SpamMessages()),
        "Spell" => Ok(Spell()),
        "SpellAuraNames" => Ok(SpellAuraNames()),
        "SpellCastTimes" => Ok(SpellCastTimes()),
        "SpellCategory" => Ok(SpellCategory()),
        "SpellChainEffects" => Ok(SpellChainEffects()),
        "SpellDispelType" => Ok(SpellDispelType()),
        "SpellDuration" => Ok(SpellDuration()),
        "SpellEffectCameraShakes" => Ok(SpellEffectCameraShakes()),
        "SpellEffectNames" => Ok(SpellEffectNames()),
        "SpellFocusObject" => Ok(SpellFocusObject()),
        "SpellIcon" => Ok(SpellIcon()),
        "SpellItemEnchantment" => Ok(SpellItemEnchantment()),
        "SpellMechanic" => Ok(SpellMechanic()),
        "SpellRadius" => Ok(SpellRadius()),
        "SpellRange" => Ok(SpellRange()),
        "SpellShapeshiftForm" => Ok(SpellShapeshiftForm()),
        "SpellVisual" => Ok(SpellVisual()),
        "SpellVisualEffectName" => Ok(SpellVisualEffectName()),
        "SpellVisualKit" => Ok(SpellVisualKit()),
        "SpellVisualPrecastTransitions" => Ok(SpellVisualPrecastTransitions()),
        "StableSlotPrices" => Ok(StableSlotPrices()),
        "Startup_Strings" => Ok(Startup_Strings()),
        "Stationery" => Ok(Stationery()),
        "StringLookups" => Ok(StringLookups()),
        "Talent" => Ok(Talent()),
        "TalentTab" => Ok(TalentTab()),
        "TaxiNodes" => Ok(TaxiNodes()),
        "TaxiPath" => Ok(TaxiPath()),
        "TaxiPathNode" => Ok(TaxiPathNode()),
        "TerrainType" => Ok(TerrainType()),
        "TerrainTypeSounds" => Ok(TerrainTypeSounds()),
        "TransportAnimation" => Ok(TransportAnimation()),
        "UISoundLookups" => Ok(UISoundLookups()),
        "UnitBlood" => Ok(UnitBlood()),
        "UnitBloodLevels" => Ok(UnitBloodLevels()),
        "VideoHardware" => Ok(VideoHardware()),
        "VocalUISounds" => Ok(VocalUISounds()),
        "WMOAreaTable" => Ok(WMOAreaTable()),
        "WeaponImpactSounds" => Ok(WeaponImpactSounds()),
        "WeaponSwingSounds2" => Ok(WeaponSwingSounds2()),
        "WorldMapArea" => Ok(WorldMapArea()),
        "WorldMapContinent" => Ok(WorldMapContinent()),
        "WorldMapOverlay" => Ok(WorldMapOverlay()),
        "WorldSafeLocs" => Ok(WorldSafeLocs()),
        "WorldStateUI" => Ok(WorldStateUI()),
        "WowError_Strings" => Ok(WowError_Strings()),
        "ZoneIntroMusicTable" => Ok(ZoneIntroMusicTable()),
        "ZoneMusic" => Ok(ZoneMusic()),
        _ => Err(SqliteError::FilenameNotFound { name: name.to_string() }),
    }
}
