use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::fmt::Debug;

/// A trait to unify [`Block`]s and [`Item`]s.
pub trait BlockItemID: Debug + Clone + PartialEq {
    /// The ID of the Block/Item
    fn id(&self) -> i16;

    /// The Block/Item corresponding to an ID
    fn from_id(id: i16) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockItem {
    Unknown,
    Block(Block),
    Item(Item),
}

impl BlockItemID for BlockItem {
    fn id(&self) -> i16 {
        match self {
            Self::Unknown => -1,
            BlockItem::Block(b) => *b as i16,
            BlockItem::Item(i) => *i as i16 + 255,
        }
    }

    fn from_id(id: i16) -> Self {
        if id <= 255 {
            if let Some(b) = Block::from_i16(id) {
                Self::Block(b)
            } else {
                Self::Unknown
            }
        } else {
            if let Some(b) = Item::from_i16(id - 255) {
                Self::Item(b)
            } else {
                Self::Unknown
            }
        }
    }
}

#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(FromPrimitive)]
pub enum Block {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
    Cobblestone = 4,
    Planks = 5,
    Sapling = 6,
    Bedrock = 7,
    WaterStill = 8,
    WaterMoving = 9,
    LavaStill = 10,
    LavaMoving = 11,
    Sand = 12,
    Gravel = 13,
    OreGold = 14,
    OreIron = 15,
    OreCoal = 16,
    Wood = 17,
    Leaves = 18,
    Sponge = 19,
    Glass = 20,
    Cloth = 35,
    PlantYellow = 37,
    PlantRed = 38,
    MushroomBrown = 39,
    MushroomRed = 40,
    BlockGold = 41,
    BlockSteel = 42,
    StairDouble = 43,
    StairSingle = 44,
    Brick = 45,
    Tnt = 46,
    BookShelf = 47,
    CobblestoneMossy = 48,
    Obsidian = 49,
    TorchWood = 50,
    Fire = 51,
    MobSpawner = 52,
    StairCompactPlanks = 53,
    Crate = 54,
    RedstoneWire = 55,
    OreDiamond = 56,
    BlockDiamond = 57,
    Workbench = 58,
    Crops = 59,
    TilledField = 60,
    StoneOvenIdle = 61,
    StoneOvenActive = 62,
    SignPost = 63,
    DoorWood = 64,
    Ladder = 65,
    MinecartTrack = 66,
    StairCompactCobblestone = 67,
    SignWall = 68,
    Lever = 69,
    PressurePlateStone = 70,
    DoorSteel = 71,
    PressurePlatePlanks = 72,
    OreRedstone = 73,
    OreRedstoneGlowing = 74,
    TorchRedstoneIdle = 75,
    TorchRedstoneActive = 76,
    Button = 77,
    Snow = 78,
    BlockIce = 79,
    BlockSnow = 80,
    Cactus = 81,
    BlockClay = 82,
    Reed = 83,
    Jukebox = 84,
    Fence = 85,
    Pumpkin = 86,
    BloodStone = 87,
    SlowSand = 88,
    LightStone = 89,
    Portal = 90,
    PumpkinLantern = 91,
}



#[repr(i16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(FromPrimitive)]
pub enum Item {
    ShovelSteel = 0,
    PickaxeSteel = 1,
    AxeSteel = 2,
    FlintAndSteel = 3,
    AppleRed = 4,
    Bow = 5,
    Arrow = 6,
    Coal = 7,
    Diamond = 8,
    IngotIron = 9,
    IngotGold = 10,
    SwordSteel = 11,
    SwordWood = 12,
    ShovelWood = 13,
    PickaxeWood = 14,
    AxeWood = 15,
    SwordStone = 16,
    ShovelStone = 17,
    PickaxeStone = 18,
    AxeStone = 19,
    SwordDiamond = 20,
    ShovelDiamond = 21,
    PickaxeDiamond = 22,
    AxeDiamond = 23,
    Stick = 24,
    BowlEmpty = 25,
    BowlSoup = 26,
    SwordGold = 27,
    ShovelGold = 28,
    PickaxeGold = 29,
    AxeGold = 30,
    Silk = 31,
    Feather = 32,
    Gunpowder = 33,
    HoeWood = 34,
    HoeStone = 35,
    HoeSteel = 36,
    HoeDiamond = 37,
    HoeGold = 38,
    Seeds = 39,
    Wheat = 40,
    Bread = 41,
    HelmetLeather = 42,
    PlateLeather = 43,
    LegsLeather = 44,
    BootsLeather = 45,
    HelmetChain = 46,
    PlateChain = 47,
    LegsChain = 48,
    BootsChain = 49,
    HelmetSteel = 50,
    PlateSteel = 51,
    LegsSteel = 52,
    BootsSteel = 53,
    HelmetDiamond = 54,
    PlateDiamond = 55,
    LegsDiamond = 56,
    BootsDiamond = 57,
    HelmetGold = 58,
    PlateGold = 59,
    LegsGold = 60,
    BootsGold = 61,
    Flint = 62,
    PorkRaw = 63,
    PorkCooked = 64,
    Painting = 65,
    AppleGold = 66,
    Sign = 67,
    DoorWood = 68,
    BucketEmpty = 69,
    BucketWater = 70,
    BucketLava = 71,
    MinecartEmpty = 72,
    Saddle = 73,
    DoorSteel = 74,
    Redstone = 75,
    Snowball = 76,
    Boat = 77,
    Leather = 78,
    BucketMilk = 79,
    Brick = 80,
    Clay = 81,
    Reed = 82,
    Paper = 83,
    Book = 84,
    SlimeBall = 85,
    MinecartCrate = 86,
    MinecartPowered = 87,
    Egg = 88,
    Compass = 89,
    FishingRod = 90,
    PocketSundial = 91,
    LightStoneDust = 92,
    FishRaw = 93,
    FishCooked = 94,
    Record13 = 2000,
    RecordCat = 2001,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemStack {
    pub stack_size: i32,
    pub animations_to_go: i32,
    pub item_id: BlockItem,
    pub item_damage: i32,
}

impl ItemStack {
    pub fn new(item_id: i32, stack_size: i32, item_damage: i32) -> Self {
        Self {
            stack_size,
            item_id: BlockItem::from_id(item_id as i16),
            item_damage,
            animations_to_go: -1,
        }
    }
}
