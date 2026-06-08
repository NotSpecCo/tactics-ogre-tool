pub mod characters;
pub mod classes;
pub mod ease2;
pub mod ease2_access;
pub mod entry_unit;
pub mod equipment;
pub mod finishers;
pub mod items;
pub mod shop;
pub mod skill_access;
pub mod skills;
pub mod spell_access;
pub mod spells;
pub mod types;

use types::ModuleDefinition;

pub fn all_modules() -> Vec<ModuleDefinition> {
    vec![
        entry_unit::definition(),
        equipment::definition(),
        items::definition(),
        classes::definition(),
        spells::definition(),
        finishers::definition(),
        skills::definition(),
        ease2::definition(),
        characters::definition(),
        skill_access::definition(),
        spell_access::definition(),
        ease2_access::definition(),
        shop::definition(),
    ]
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
