#[repr(i16)]
pub enum Block {
    Unknown = -1,
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
pub enum Item {
    Unknown = -1,
    ShovelSteel = 256,
    PickaxeSteel = 257,
    AxeSteel = 258,
    FlintAndSteel = 259,
    AppleRed = 260,
    Bow = 261,
    Arrow = 262,
    Coal = 263,
    Diamond = 264,
    IngotIron = 265,
    IngotGold = 266,
    SwordSteel = 267,
    SwordWood = 268,
    ShovelWood = 269,
    PickaxeWood = 270,
    AxeWood = 271,
    SwordStone = 272,
    ShovelStone = 273,
    PickaxeStone = 274,
    AxeStone = 275,
    SwordDiamond = 276,
    ShovelDiamond = 277,
    PickaxeDiamond = 278,
    AxeDiamond = 279,
    Stick = 280,
    BowlEmpty = 281,
    BowlSoup = 282,
    SwordGold = 283,
    ShovelGold = 284,
    PickaxeGold = 285,
    AxeGold = 286,
    Silk = 287,
    Feather = 288,
    Gunpowder = 289,
    HoeWood = 290,
    HoeStone = 291,
    HoeSteel = 292,
    HoeDiamond = 293,
    HoeGold = 294,
    Seeds = 295,
    Wheat = 296,
    Bread = 297,
    HelmetLeather = 298,
    PlateLeather = 299,
    LegsLeather = 300,
    BootsLeather = 301,
    HelmetChain = 302,
    PlateChain = 303,
    LegsChain = 304,
    BootsChain = 305,
    HelmetSteel = 306,
    PlateSteel = 307,
    LegsSteel = 308,
    BootsSteel = 309,
    HelmetDiamond = 310,
    PlateDiamond = 311,
    LegsDiamond = 312,
    BootsDiamond = 313,
    HelmetGold = 314,
    PlateGold = 315,
    LegsGold = 316,
    BootsGold = 317,
    Flint = 318,
    PorkRaw = 319,
    PorkCooked = 320,
    Painting = 321,
    AppleGold = 322,
    Sign = 323,
    DoorWood = 324,
    BucketEmpty = 325,
    BucketWater = 326,
    BucketLava = 327,
    MinecartEmpty = 328,
    Saddle = 329,
    DoorSteel = 330,
    Redstone = 331,
    Snowball = 332,
    Boat = 333,
    Leather = 334,
    BucketMilk = 335,
    Brick = 336,
    Clay = 337,
    Reed = 338,
    Paper = 339,
    Book = 340,
    SlimeBall = 341,
    MinecartCrate = 342,
    MinecartPowered = 343,
    Egg = 344,
    Compass = 345,
    FishingRod = 346,
    PocketSundial = 347,
    LightStoneDust = 348,
    FishRaw = 349,
    FishCooked = 350,
    Record13 = 2000,
    RecordCat = 2001,
}

impl Item {
    fn index() -> i16 {
        256
    }
}
