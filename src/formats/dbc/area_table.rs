use super::dbc::DbcRowMapper;
use crate::common::R;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaTableDbcRow {
    id: u32,
    map_id: u32,
    area_id: u32,
    explore_flag: u32,
    flags: u32,
    sound_ambience_ref_id: u32,
    zone_music_ref_id: u32,
    zone_intro_music_ref_id: u32,
    area_level: u32,
    area_name: String,
    faction_group_id: u32,
}

impl DbcRowMapper for AreaTableDbcRow {
    fn map_dbc_row(row: &super::DbcFileIteratorRow) -> R<Self> {
        let id = row.get_number_column(1)?;
        let map_id = row.get_number_column(2)?;
        let area_id = row.get_number_column(3)?;
        let explore_flag = row.get_number_column(4)?;
        let flags = row.get_number_column(5)?;
        let sound_ambience_ref_id = row.get_number_column(8)?;
        let zone_music_ref_id = row.get_number_column(9)?;
        let zone_intro_music_ref_id = row.get_number_column(10)?;
        let area_level = row.get_number_column(11)?;
        let area_name = row.get_string_column(12)?;
        let faction_group_id = row.get_number_column(29)?;
        Ok(AreaTableDbcRow {
            id,
            map_id,
            area_id,
            explore_flag,
            flags,
            sound_ambience_ref_id,
            zone_music_ref_id,
            zone_intro_music_ref_id,
            area_level,
            area_name,
            faction_group_id,
        })
    }
}
