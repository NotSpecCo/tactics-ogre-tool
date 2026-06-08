pub mod battle_data;
pub mod menu_data;
pub mod types;

use types::{DatFile, ModuleDefinition};

pub fn all_modules() -> Vec<ModuleDefinition> {
    vec![
        battle_data::entry_unit::definition(),
        battle_data::equipment::definition(),
        battle_data::items::definition(),
        battle_data::classes::definition(),
        battle_data::spells::definition(),
        battle_data::finishers::definition(),
        battle_data::skills::definition(),
        battle_data::ease2::definition(),
        battle_data::characters::definition(),
        battle_data::skill_access::definition(),
        battle_data::spell_access::definition(),
        battle_data::ease2_access::definition(),
        menu_data::shop::definition(),
    ]
}

pub fn dat_file_for_path(dat_path: &str) -> Option<DatFile> {
    let file_name = std::path::Path::new(dat_path)
        .file_name()
        .and_then(|f| f.to_str());

    match file_name {
        Some("battle_data_release.dat") => Some(DatFile::BattleData),
        Some("menu_data.dat") => Some(DatFile::MenuData),
        _ => None,
    }
}

pub fn modules_for_dat(dat_path: &str) -> Vec<ModuleDefinition> {
    match dat_file_for_path(dat_path) {
        Some(df) => all_modules()
            .into_iter()
            .filter(|m| m.dat_file == df)
            .collect(),
        None => vec![],
    }
}

pub fn find_module(module_id: &str) -> Option<ModuleDefinition> {
    all_modules().into_iter().find(|m| m.id == module_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_modules_returns_13() {
        assert_eq!(all_modules().len(), 13);
    }

    #[test]
    fn battle_data_has_12_modules() {
        let mods = modules_for_dat("battle/battle_data_release.dat");
        assert_eq!(mods.len(), 12);
        assert_eq!(
            dat_file_for_path("battle/battle_data_release.dat"),
            Some(DatFile::BattleData)
        );
    }

    #[test]
    fn battle_data_matches_at_root() {
        let mods = modules_for_dat("battle_data_release.dat");
        assert_eq!(mods.len(), 12);
    }

    #[test]
    fn battle_data_matches_nested() {
        let mods = modules_for_dat("some/deep/path/battle_data_release.dat");
        assert_eq!(mods.len(), 12);
    }

    #[test]
    fn menu_data_has_1_module() {
        let mods = modules_for_dat("menu/menu_data.dat");
        assert_eq!(mods.len(), 1);
        assert_eq!(mods[0].id, "shop");
    }

    #[test]
    fn does_not_match_suffix() {
        let mods = modules_for_dat("fake_battle_data_release.dat");
        assert!(mods.is_empty());
    }

    #[test]
    fn unknown_dat_returns_empty() {
        let mods = modules_for_dat("some/unknown.dat");
        assert!(mods.is_empty());
    }

    #[test]
    fn find_module_by_id() {
        assert!(find_module("equipment").is_some());
        assert!(find_module("shop").is_some());
        assert!(find_module("nonexistent").is_none());
    }

    #[test]
    fn all_module_ids_unique() {
        let mods = all_modules();
        let mut ids: Vec<&str> = mods.iter().map(|m| m.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), mods.len());
    }
}
