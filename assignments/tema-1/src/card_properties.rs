use multiversx_sc::derive_imports::*;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq)]
pub struct CardProperties {
    pub class: Class,
    pub rarity: Rarity,
    pub power: Power,
}

impl CardProperties {
    pub fn new(class: u8, power: u8, rarity: u8) -> Result<Self, &'static str> {
        Ok(Self {
            class: Class::from_int(class)?,
            power: Power::from_int(power)?,
            rarity: Rarity::from_int(rarity)?,
        })
    }
}

#[derive(TypeAbi, TopEncode, NestedEncode, NestedDecode, PartialEq)]
pub enum Rarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl Rarity {
    pub fn from_int(value: u8) -> Result<Self, &'static str> {
        match value {
            0 => Ok(Rarity::Common),
            1 => Ok(Rarity::Rare),
            2 => Ok(Rarity::Epic),
            3 => Ok(Rarity::Legendary),
            _ => Err("Invalid integer for Rarity"),
        }
    }
}

#[derive(TypeAbi, TopEncode, NestedEncode, NestedDecode, PartialEq)]
pub enum Class {
    Warrior,
    Mage,
    Rogue,
    Priest,
    Hunter,
    Warlock,
    Shaman,
    Druid,
    Paladin,
}

impl Class {
    pub fn from_int(value: u8) -> Result<Self, &'static str> {
        match value {
            0 => Ok(Class::Warrior),
            1 => Ok(Class::Mage),
            2 => Ok(Class::Rogue),
            3 => Ok(Class::Priest),
            4 => Ok(Class::Hunter),
            5 => Ok(Class::Warlock),
            6 => Ok(Class::Shaman),
            7 => Ok(Class::Druid),
            8 => Ok(Class::Paladin),
            _ => Err("Invalid integer for Class"),
        }
    }
}

#[derive(TypeAbi, TopEncode, NestedEncode, NestedDecode, PartialEq)]
pub enum Power {
    Low,
    Medium,
    High,
}

impl Power {
    pub fn from_int(value: u8) -> Result<Self, &'static str> {
        match value {
            0 => Ok(Power::Low),
            1 => Ok(Power::Medium),
            2 => Ok(Power::High),
            _ => Err("Invalid integer for Power"),
        }
    }
}
