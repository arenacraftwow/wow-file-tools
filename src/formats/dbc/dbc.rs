use crate::common::R;
use crate::formats::dbc::area_table::AreaTableDbcRow;
use crate::formats::dbc::battle_master_list::BattleMasterListDbcRow;
use crate::formats::dbc::game_object_display_info::GameObjectDisplayInfo;
use crate::formats::dbc::ground_effect_doodad::GroundEffectDoodadDbcRow;
use crate::formats::dbc::ground_effect_texture::GroundEffectTextureDbcRow;
use crate::formats::dbc::light::LightDbcRow;
use crate::formats::dbc::light_params::LightParamsDbcRow;
use crate::formats::dbc::light_sky_box::LightSkyBoxDbcRow;
use crate::formats::dbc::loading_screens::LoadingScreenDbcRow;
use crate::formats::dbc::map::MapDbcRow;
use crate::formats::dbc::pvp_difficulty::PvpDifficulty;
use crate::formats::dbc::spell::SpellDbcRow;
use crate::formats::dbc::spell_category::SpellCategoryDbcRow;
use crate::formats::dbc::spell_icon::SpellIconDbcRow;
use crate::formats::dbc::spell_visual::SpellVisualDbcRow;
use crate::formats::dbc::spell_visual_effect_name::SpellVisualEffectNameDbcRow;
use crate::formats::dbc::spell_visual_kit::SpellVisualKitDbcRow;
use crate::formats::dbc::talent::TalentDbcRow;
use crate::formats::dbc::talent_tab::TalentTabRow;
use crate::formats::dbc::{DbcFile, DbcHeader};
use serde::{Deserialize, Serialize};
use std::path::Path;

use super::DbcFileIteratorRow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dbc<T> {
    pub header: DbcHeader,
    pub rows: Vec<T>,
}

pub trait DbcRowMapper
where
    Self: Sized,
{
    fn map_dbc_row(row: &DbcFileIteratorRow) -> R<Self>;
}

fn load_dbc<T: DbcRowMapper, P: AsRef<Path>>(path: P) -> R<Dbc<T>> {
    let dbc = DbcFile::new(path)?;
    let mut row_builder = Vec::with_capacity(dbc.header.field_count as usize);
    for row in &dbc {
        let record = T::map_dbc_row(&row)?;
        row_builder.push(record);
    }
    Ok(Dbc {
        header: dbc.header,
        rows: row_builder,
    })
}

pub fn load_map_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<MapDbcRow>> {
    load_dbc(path)
}

pub fn load_loading_screens_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<LoadingScreenDbcRow>> {
    load_dbc(path)
}

pub fn load_area_table_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<AreaTableDbcRow>> {
    load_dbc(path)
}

pub fn load_light_sky_box_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<LightSkyBoxDbcRow>> {
    load_dbc(path)
}

pub fn load_light_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<LightDbcRow>> {
    load_dbc(path)
}

pub fn load_light_params_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<LightParamsDbcRow>> {
    load_dbc(path)
}

pub fn load_battle_master_list_from_path<P: AsRef<Path>>(
    path: P,
) -> R<Dbc<BattleMasterListDbcRow>> {
    load_dbc(path)
}

pub fn load_ground_effect_texture_from_path<P: AsRef<Path>>(
    path: P,
) -> R<Dbc<GroundEffectTextureDbcRow>> {
    load_dbc(path)
}

pub fn load_ground_effect_doodad_from_path<P: AsRef<Path>>(
    path: P,
) -> R<Dbc<GroundEffectDoodadDbcRow>> {
    load_dbc(path)
}

pub fn load_pvp_difficulty_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<PvpDifficulty>> {
    load_dbc(path)
}

pub fn load_game_object_display_info_from_path<P: AsRef<Path>>(
    path: P,
) -> R<Dbc<GameObjectDisplayInfo>> {
    load_dbc(path)
}

pub fn load_spell_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<SpellDbcRow>> {
    load_dbc(path)
}

pub fn load_spell_category_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<SpellCategoryDbcRow>> {
    load_dbc(path)
}

pub fn load_spell_visual_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<SpellVisualDbcRow>> {
    load_dbc(path)
}

pub fn load_spell_visual_effect_name_dbc_from_path<P: AsRef<Path>>(
    path: P,
) -> R<Dbc<SpellVisualEffectNameDbcRow>> {
    load_dbc(path)
}

pub fn load_spell_visual_kit_dbc_from_path<P: AsRef<Path>>(
    path: P,
) -> R<Dbc<SpellVisualKitDbcRow>> {
    load_dbc(path)
}

pub fn load_talent_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<TalentDbcRow>> {
    load_dbc(path)
}

pub fn load_talent_tab_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<TalentTabRow>> {
    load_dbc(path)
}

pub fn load_spell_icon_dbc_from_path<P: AsRef<Path>>(path: P) -> R<Dbc<SpellIconDbcRow>> {
    load_dbc(path)
}
