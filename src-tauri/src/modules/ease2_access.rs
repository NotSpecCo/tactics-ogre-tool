use crate::modules::types::*;

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "ease2_access".to_string(),
        name: "Ease II Access Editor".to_string(),
        description: "Edit Ease II spell class access".to_string(),
        base_offset: 0x4B2C70,
        entry_count: 1,
        entry_size: 64,
        entry_names: vec!["Ease II".to_string()],
        fields: vec![
            FieldDefinition {
                name: "All Magic Set".to_string(),
                offset: 0,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Wizard Magic Set".to_string(),
                offset: 1,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cleric Magic Set".to_string(),
                offset: 2,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Valkyrie Magic Set".to_string(),
                offset: 3,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Knight Magic Set".to_string(),
                offset: 4,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Terror Knight Magic Set".to_string(),
                offset: 5,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Swordmaster Magic Set".to_string(),
                offset: 6,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Ninja Magic Set".to_string(),
                offset: 7,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Warlock Magic Set".to_string(),
                offset: 8,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Necromancer Magic Set".to_string(),
                offset: 9,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Lich Magic Set".to_string(),
                offset: 10,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Divine Knight Magic Set".to_string(),
                offset: 11,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Patriarch Magic Set".to_string(),
                offset: 12,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Familiar Magic Set".to_string(),
                offset: 13,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cyclops Magic Set".to_string(),
                offset: 14,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Boggart Magic Set".to_string(),
                offset: 15,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "None? Magic Set".to_string(),
                offset: 16,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Lord Magic Set".to_string(),
                offset: 17,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Priest Magic Set".to_string(),
                offset: 18,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Dark Priest Magic Set".to_string(),
                offset: 19,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Princess Magic Set".to_string(),
                offset: 20,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Paladin Magic Set".to_string(),
                offset: 21,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Astromancer Magic Set".to_string(),
                offset: 22,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Vartan Magic Set".to_string(),
                offset: 23,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "White Knight Magic Set".to_string(),
                offset: 24,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Shaman Magic Set".to_string(),
                offset: 25,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Wicce Magic Set".to_string(),
                offset: 26,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Songstress Magic Set".to_string(),
                offset: 27,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Knight Commander Magic Set".to_string(),
                offset: 28,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Phoenix Magic Set".to_string(),
                offset: 29,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Rukh Magic Set".to_string(),
                offset: 30,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let def = definition();
        assert_eq!(def.id, "ease2_access");
        assert_eq!(def.base_offset, 0x4B2C70);
        assert_eq!(def.entry_count, 1);
        assert_eq!(def.entry_size, 64);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 31);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 1);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "All Magic Set");
        assert_eq!(f.offset, 0);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[30];
        assert_eq!(f.name, "Rukh Magic Set");
        assert_eq!(f.offset, 30);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn fields_within_entry_bounds() {
        let def = definition();
        for field in &def.fields {
            assert!(
                field.offset + field.size <= def.entry_size,
                "field '{}' at offset {} + size {} exceeds entry_size {}",
                field.name,
                field.offset,
                field.size,
                def.entry_size
            );
        }
    }
}
