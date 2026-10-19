use crate::profile::{Equipped, Profile};
use serde::{Deserialize, Serialize};
use shared::items::{self, ItemId, ItemRef};

const SLOT_KEY_PREFIX: &str = "save_slot_";

fn slot_key(slot: u8) -> String {
    format!("{}{}", SLOT_KEY_PREFIX, slot)
}

#[derive(Serialize, Deserialize)]
struct SavedEquipped {
    pickaxe: ItemId,
    hat: ItemId,
    backpack: ItemId,
}

#[derive(Serialize, Deserialize)]
struct SavedProfile {
    name: String,
    slot: u8,
    gold: u16,
    items: Vec<ItemId>,
    equipped: SavedEquipped,
    #[serde(default)]
    latest_completed_level: u8,
}

impl From<&Profile> for SavedProfile {
    fn from(p: &Profile) -> Self {
        Self {
            name: p.name.clone(),
            slot: p.slot,
            gold: p.gold,
            items: p.items.iter().map(|i| i.id()).collect(),
            equipped: SavedEquipped {
                pickaxe: ItemRef::Pickaxe(p.equipped.pickaxe).id(),
                hat: ItemRef::Hat(p.equipped.hat).id(),
                backpack: ItemRef::Backpack(p.equipped.backpack).id(),
            },
            latest_completed_level: p.latest_completed_level,
        }
    }
}

impl TryFrom<SavedProfile> for Profile {
    type Error = String;

    fn try_from(s: SavedProfile) -> Result<Self, Self::Error> {
        let equipped = Equipped {
            pickaxe: items::get_item(s.equipped.pickaxe)
                .and_then(|i| i.pickaxe())
                .ok_or("Invalid pickaxe ID in save file")?,
            hat: items::get_item(s.equipped.hat)
                .and_then(|i| i.hat())
                .ok_or("Invalid hat ID in save file")?,
            backpack: items::get_item(s.equipped.backpack)
                .and_then(|i| i.backpack())
                .ok_or("Invalid backpack ID in save file")?,
        };

        let items = s.items.into_iter().filter_map(items::get_item).collect();

        Ok(Profile {
            name: s.name,
            slot: s.slot,
            gold: s.gold,
            items,
            equipped,
            latest_completed_level: s.latest_completed_level,
        })
    }
}

pub fn save_profile(profile: &Profile) -> Result<(), String> {
    let saved = SavedProfile::from(profile);
    let json = serde_json::to_string(&saved).map_err(|e| e.to_string())?;
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    storage.set(&slot_key(profile.slot), &json);
    Ok(())
}

pub fn delete_profile(slot: u8) {
    let mut storage = quad_storage::STORAGE.lock().unwrap();
    storage.remove(&slot_key(slot.into()));
}

pub fn load_profile(slot: u8) -> Result<Option<Profile>, String> {
    let storage = quad_storage::STORAGE.lock().unwrap();
    let Some(json) = storage.get(&slot_key(slot.into())) else {
        return Ok(None);
    };

    let saved: SavedProfile = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Profile::try_from(saved)
        .map(Some)
        .map_err(|e| format!("slot {}: {}", slot, e))
}
