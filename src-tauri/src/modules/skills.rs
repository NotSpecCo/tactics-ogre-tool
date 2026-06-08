use crate::modules::types::*;

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "skills".to_string(),
        name: "Action Skill Editor".to_string(),
        description: "Edit action skill properties".to_string(),
        base_offset: 0x3E713C,
        entry_count: 158,
        entry_size: 180,
        entry_names: vec![
            "Recruit".to_string(),
            "Subdue".to_string(),
            "Coax".to_string(),
            "Tame".to_string(),
            "Seraph's Pact".to_string(),
            "Demon's Pact".to_string(),
            "Fey Pact".to_string(),
            "Master Undead".to_string(),
            "Control Golem".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Gordian Key".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Animate Dead".to_string(),
            "Absolution".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Evanescece".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Evil Deeds".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Sublime Sacrifice".to_string(),
            "None".to_string(),
            "Sanctuary Shadow".to_string(),
            "Jack-o'-Lantern".to_string(),
            "Shadowbreak".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Holy Water".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Nature's Whisper".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Dragon's Wound".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Consecrate Dead".to_string(),
            "Condemn".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Rampart Shadow".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Aqua Bubble".to_string(),
            "Agonal Scream".to_string(),
            "Acid Breath".to_string(),
            "Acid Breath II".to_string(),
            "Sweaty Palms".to_string(),
            "Wind Shot".to_string(),
            "Virtuous Dance".to_string(),
            "Cruelist Cut".to_string(),
            "Granite Fist".to_string(),
            "Crystal Pumpkin".to_string(),
            "Reeking Meetballs".to_string(),
            "Coquettish Kiss (Faerie)".to_string(),
            "Coquettish Kiss (Gremlin)".to_string(),
            "Silent Song".to_string(),
            "Pirate Breath".to_string(),
            "Requiem".to_string(),
            "Thunder Breath".to_string(),
            "Thunder Breath II".to_string(),
            "Stun Breath".to_string(),
            "Stun Breath II".to_string(),
            "Sparagmos".to_string(),
            "Sand Breath".to_string(),
            "Sand Breath II".to_string(),
            "Lingering Kiss (Faerie)".to_string(),
            "Lingering Kiss (Gremlin)".to_string(),
            "Day of Reckoning".to_string(),
            "Divine Breath".to_string(),
            "Divine Breath II".to_string(),
            "Tail Lash".to_string(),
            "Toxic Breath".to_string(),
            "Toxic Breath II".to_string(),
            "Vortex Breath".to_string(),
            "Vortex Breath II".to_string(),
            "Numbing Hook".to_string(),
            "Stinky Feet".to_string(),
            "Pumpkin Strike".to_string(),
            "Pumpkin Pie".to_string(),
            "Pumpkin Bomb".to_string(),
            "Flame Breath".to_string(),
            "Flame Breath II".to_string(),
            "Blood Syphon".to_string(),
            "Selfless Kiss (Faerie)".to_string(),
            "Selfless Kiss (Gremlin)".to_string(),
            "Blue Spiral".to_string(),
            "Frost Breath".to_string(),
            "Frost Breath II".to_string(),
            "Heaven's Tear".to_string(),
            "Petro Breath".to_string(),
            "Petro Breath II".to_string(),
            "Poison Rain".to_string(),
            "Poison Breath".to_string(),
            "Poison Breath II".to_string(),
            "Poignant Melody".to_string(),
            "Maelstrom".to_string(),
            "Stirring Kiss (Faerie)".to_string(),
            "Stirring Kiss (Gremlin)".to_string(),
            "Raven Eye".to_string(),
            "Corpse Breath".to_string(),
            "Corpse Breath II".to_string(),
            "Evil Eye".to_string(),
            "Celestial Song".to_string(),
        ],
        fields: vec![
            FieldDefinition {
                name: "RT".to_string(),
                offset: 3,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cost Type".to_string(),
                offset: 7,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cost".to_string(),
                offset: 8,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Range Short".to_string(),
                offset: 13,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Range Long/-1".to_string(),
                offset: 14,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Range Lower Limit".to_string(),
                offset: 15,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Area 0=1,1=1".to_string(),
                offset: 16,
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
        assert_eq!(def.id, "skills");
        assert_eq!(def.base_offset, 0x3E713C);
        assert_eq!(def.entry_count, 158);
        assert_eq!(def.entry_size, 180);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 7);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 158);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "RT");
        assert_eq!(f.offset, 3);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[6];
        assert_eq!(f.name, "Area 0=1,1=1");
        assert_eq!(f.offset, 16);
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
