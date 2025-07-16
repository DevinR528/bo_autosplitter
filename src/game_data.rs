use asr::{game_engine::unity::mono::Class, Address64,};
use asr::string::ArrayCString;
use bytemuck::{Pod, Zeroable};

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum BossKind {
    Placeholder = 0,
    KiriKiriBozu = 1,
    PUA = 2,
    Hashihime = 3,
    Yuki = 4,
    Yokozuna = 5,
    Jorogumo = 6,
    KarasuTengu = 7,
    DaiTengu = 8,
    Gasha = 9,
    Asahi = 10,
    Shogun = 11,
    Amaterasu = 12,
}

unsafe impl Zeroable for BossKind {}
unsafe impl Pod for BossKind {}

#[derive(Debug, Class, Copy, Clone, PartialEq, PartialOrd)]
pub struct BossData {
    #[rename = "<Boss>k__BackingField"]
    pub boss_kind: BossKind,
    #[rename = "<Defeated>k__BackingField"]
    pub defeated: bool,
    #[rename = "<InProgress>k__BackingField"]
    pub in_progress: bool,
    #[rename = "<TotalHealth>k__BackingField"]
    pub total_health: f32,
    #[rename = "<OverrideInProgress>k__BackingField"]
    pub override_in_progress: bool,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, PartialOrd)]
pub struct EnemiesManager {
    #[rename = "<CurrentStaffDamage>k__BackingField"]
    pub staff_damage: f32,
    #[rename = "bosses"]
    pub bosses: Address64,
}

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum DarumaType {
    /// Chomper
    Bite = 0,
    /// Mamori
    Parry = 1,
    /// Toge-Chan
    Thorns = 2,
    /// Jingu
    Spirits = 3,
    /// Kaboomaru
    Bomb = 4,
    /// Not sure
    SpinAttack = 5,
    /// Not sure
    Deprecated1 = 6,
    /// Pyro-Kun
    FireWall = 7,
    /// Yuki
    Ice = 8,
    /// Ken
    Boomerang = 9,
}

unsafe impl Zeroable for DarumaType {}
unsafe impl Pod for DarumaType {}

#[derive(Debug, Class, Copy, Clone, PartialEq, PartialOrd)]
pub struct Daruma {
    #[rename = "<Type>k__BackingField"]
    pub daruma_type: DarumaType,
    #[rename = "<Available>k__BackingField"]
    pub available: bool,
    #[rename = "<isActive>k__BackingField"]
    pub is_active: bool,
    #[rename = "<TwoEyes>k__BackingField"]
    pub two_eyes: bool,

    #[rename = "<Stage1TeaCost>k__BackingField"]
    pub stage_one_tea_cost: i32,
    #[rename = "<Stage2TeaCost>k__BackingField"]
    pub stage_two_tea_cost: i32,
    #[rename = "<Stage3TeaCost>k__BackingField"]
    pub stage_three_tea_cost: i32,

    #[rename = "<Stage1Damage>k__BackingField"]
    pub stage_one_damage: f32,
    #[rename = "<Stage2Damage>k__BackingField"]
    pub stage_two_damage: f32,
    #[rename = "<Stage3Damage>k__BackingField"]
    pub stage_three_damage: f32,

    #[rename = "<Stage1Duration>k__BackingField"]
    pub stage_one_duration: f32,
    #[rename = "<Stage2Duration>k__BackingField"]
    pub stage_two_duration: f32,
    #[rename = "<Stage3Duration>k__BackingField"]
    pub stage_three_duration: f32,

    #[rename = "<TimeBetweenHits>k__BackingField"]
    pub time_between_hits: f32,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, PartialOrd)]
pub struct DarumaManager {
    #[rename = "<DarumaBoostDamageIncrease>k__BackingField"]
    pub daruma_boost_damage: f32,
    #[rename = "allDarumas"]
    pub all_darumas: Address64,
}

#[derive(Debug, Class, Copy, Clone, PartialEq)]
pub struct InventoryContainer {
    #[rename = "<FeatherKeys>k__BackingField"]
    pub feather_keys: i32,

    #[rename = "<MusicSheet>k__BackingField"]
    pub music_sheets: i32,

    #[rename = "<OmamoriStraps>k__BackingField"]
    pub omamori_straps: i32,

    #[rename = "<HasFragileEgg>k__BackingField"]
    pub fragile_egg: bool,

    #[rename = "<ResetForNewGame>k__BackingField"]
    pub reset_new_game: bool,

    #[rename = "<HasKitsuneKifuda>k__BackingField"]
    pub has_kitsune_kifuda: bool,

    #[rename = "<HasScuffedGunbai>k__BackingField"]
    pub has_scuffed_gunbai: bool,

    #[rename = "<BaseDamage>k__BackingField"]
    pub base_damage: f32,

    #[rename = "<InscrutableTableFragment>k__BackingField"]
    pub tablets: i32,

    #[rename = "<Kodamas>k__BackingField"]
    pub number_of_kodamas: i32,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, Eq)]
pub struct AbilityManager {
    #[rename = "<CanAttack>k__BackingField"]
    pub can_attack: bool,

    #[rename = "<CanBat>k__BackingField"]
    pub can_bat: bool,

    #[rename = "<CanDash>k__BackingField"]
    pub can_dash: bool,

    #[rename = "<CanHover>k__BackingField"]
    pub can_hover: bool,

    #[rename = "<CanIDash>k__BackingField"]
    pub can_idash: bool,

    #[rename = "<CanGrapple>k__BackingField"]
    pub can_grapple: bool,

    #[rename = "<CanHammerDash>k__BackingField"]
    pub can_hammer_dash: bool,

    #[rename = "<CanWallJump>k__BackingField"]
    pub can_wall_jump: bool,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, PartialOrd)]
pub struct BetaPlayerDataManager {
    #[rename = "<TimePlayed>k__BackingField"]
    pub time_played: f32,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, Eq)]
pub struct QuestManager {
    /// Asahi bulks your bamboo sword start
    #[rename = "<AsahiBambooStaffQuestStarted>k__BackingField"]
    pub asahi_staff_start: bool,
    /// Asahi bulks your bamboo sword end
    #[rename = "<AsahiBambooStaffQuestCompleted>k__BackingField"]
    pub asahi_staff_end: bool,

    /// Asahi gives quest for dash when entering caves???
    #[rename = "<AsahiEyeOfTheBeastQuestStarted>k__BackingField"]
    pub asahi_eye_of_beast_start: bool,
    /// Defeate PUA and get eye of beast
    #[rename = "<AsahiEyeOfTheBeastQuestCompleted>k__BackingField"]
    pub asahi_eye_of_beast_end: bool,

    /// Not really sure when this splits
    #[rename = "<AsahiAfterArmapilloBoss>k__BackingField"]
    pub asahi_post_armapillo_boss: bool,

    // No idea what bump (start)
    #[rename = "<ToriBumpProphecyTold>k__BackingField"]
    pub tori_bump_told: bool,
    // No idea what bump (end)
    #[rename = "<ToriFulfilledBumpProphecy>k__BackingField"]
    pub tori_bump_end: bool,

    /// Bird bat prophecy start (not really sure)
    #[rename = "<ToriBatProphecyTold>k__BackingField"]
    pub tori_bat_told: bool,
    /// Bird bat prophecy start (not really sure)
    #[rename = "<ToriFulfilledBatProphecy>k__BackingField"]
    pub tori_bat_end: bool,

    /// Bird dash prophecy start (not really sure)
    #[rename = "<ToriDashProphecyTold>k__BackingField"]
    pub tori_dash_told: bool,
    /// Bird dash prophecy end (not really sure)
    #[rename = "<ToriFulfilledDashProphecy>k__BackingField"]
    pub tori_dash_end: bool,

    /// Number of armadillos collected for Shimeji's quest.
    #[rename = "<ShimejiArmapillosCollected>k__BackingField"]
    pub shimeji_armapillos_collect: i32,
    /// Collect 4 armadillos quest start
    #[rename = "<ShimejiQuestStarted>k__BackingField"]
    pub shimeji_quest_start: bool,
    /// Collect 4 armadillos quest end
    #[rename = "<ShimejiQuestCompleted>k__BackingField"]
    pub shimeji_quest_end: bool,

    /// Rozu's Requiem quest start
    #[rename = "<RozusRequiemQuestStarted>k__BackingField"]
    pub rozus_requiem_start: bool,
    /// Rozu's Requiem quest end
    #[rename = "<RozusRequiemQuestCompleted>k__BackingField"]
    pub rozus_requiem_end: bool,

    /// The Fox wedding quest start, get Kitsune scroll and Ingenuity Omamori.
    #[rename = "<FoxWeddingQuestStarted>k__BackingField"]
    pub fox_wedding_start: bool,
    /// The Fox wedding quest, saving groom from spider.
    #[rename = "<GroomAscentCompleted>k__BackingField"]
    pub fox_wedding_save_groom: bool,
    /// The Fox wedding quest end, get Kitsune scroll and Ingenuity Omamori.
    #[rename = "<FoxWeddingQuestCompleted>k__BackingField"]
    pub fox_wedding_end: bool,

    /// The Vermilion Stranger quest start (this gives fast travel)
    #[rename = "<VermillionStrangerQuestStarted>k__BackingField"]
    pub vermilion_stranger_quest_start: bool,
    /// The Vermilion Stranger quest end (this gives fast travel)
    #[rename = "<VSQuestCompleted>k__BackingField"]
    pub vermilion_stranger_quest_end: bool,

    /// The Kitsune Kifuda quest (DaiTangu and Gashadoku)
    #[rename = "<KitsuneKifudaQuestStarted>k__BackingField"]
    pub kitsune_kifuda_start: bool,
    /// The Kitsune Kifuda quest (DaiTangu and Gashadoku)
    #[rename = "<KitsuneKifudaQuestCompleted>k__BackingField"]
    pub kitsune_kifuda_end: bool,

    /// The Infinite Tea Kettle Quest.
    #[rename = "<InfiniteKettleQuestStarted>k__BackingField"]
    pub infinite_kettle_start: bool,
    /// The Infinite Tea Kettle Quest.
    #[rename = "<InfiniteKettleQuestCompleted>k__BackingField"]
    pub infinite_kettle_end: bool,

    /// Inserting the feather from the west side of white --- I mean ice palace into the keyhole
    #[rename = "<FirstFeatherKeyEntered>k__BackingField"]
    pub west_feather_in_keyhole: bool,

    /// Inserting the second feather from the east side of white--- I mean ice palace
    #[rename = "<SecondFeatherKeyEntered>k__BackingField"]
    pub east_feather_in_keyhole: bool,

    /// This is the second boss of the game "Particularly Unmanageable Armadillo"
    #[rename = "<DefeatedPUABoss>k__BackingField"]
    pub defeat_pua_boss: bool,

    /// This is the third boss of the game "Hashihime" (bridge wave lady)
    #[rename = "<HashihimeDefeated>k__BackingField"]
    pub defeat_hashihime_boss: bool,

    /// This is the fourth boss in the game "Kaboto Yokozuma" (the beetle)
    #[rename = "<YokozumaCompleted>k__BackingField"]
    pub defeat_kaboto_boss: bool,

    /// This is the fith boss in the game "Jorogumo" (the spider)
    #[rename = "<DefeatedJorogumo>k__BackingField"]
    pub defeat_spider_boss: bool,

    /// This is the sixth boss in the game "Tengu" (the 3 bird warriors)
    #[rename = "<TenguTrialQuestCompleted>k__BackingField"]
    pub defeat_tengu_boss: bool,

    /// This is the seventh boss in the game "Gashadokuro" (giant skeleton)
    #[rename = "<GashaDefeated>k__BackingField"]
    pub defeat_gash_boss: bool,

    /// This is the eigth boss in the game "Asahi" (Your buddy)
    #[rename = "<AsahiDefeated>k__BackingField"]
    pub defeat_asahi_boss: bool,

    /// This is the ninth boss in the game "Sakura Shogun" (Final Boss)
    #[rename = "<ShogunDefeated>k__BackingField"]
    pub defeat_sakura_boss: bool,

    /// This is as the credits roll, you finished GG
    ///
    /// TODO: confirm
    #[rename = "<PostGame>k__BackingField"]
    pub credits_roll: bool,
    // TODO: more of these...
}

#[derive(Class, Copy, Clone, PartialEq, Eq)]
pub struct SceneNameManager {
    /// Active scene seems to determine if we are in game, main menu, starting screen.
    #[rename = "<activeSceneName>k__BackingField"]
    pub active_scene_name: ArrayCString<32>,
    
    #[rename = "<EndCreditsk__BackingField"]
    pub end_credits: ArrayCString<32>,

    #[rename = "<MainMenu>k__BackingField"]
    pub main_menu: ArrayCString<32>,

    #[rename = "<HumblePlayground>k__BackingField"]
    pub humble_playground: ArrayCString<32>,

    /// Current world map area is what loading zone you are in in-game.
    #[rename = "<CurrentWorldMapArea>k__BackingField"]
    pub current_world_map_area: ArrayCString<32>,

    /// cbf = crimson bamboo forest
    #[rename = "<CBFIntro>k__BackingField"]
    pub cbf_intro: ArrayCString<32>,

    #[rename = "<CBFBumpIntro>k__BackingField"]
    pub cbf_bump_intro: ArrayCString<32>,

    #[rename = "<CBFCaveEntrance>k__BackingField"]
    pub cbf_cave_entrance: ArrayCString<32>,

    #[rename = "<Coast1>k__BackingField"]
    pub coast1: ArrayCString<32>,

    /// uc = uzumaki cave
    #[rename = "<UCEntrance>k__BackingField"]
    pub uc_entrance: ArrayCString<32>,

    #[rename = "<UCShortcut>k__BackingField"]
    pub uc_shortcut: ArrayCString<32>,

    #[rename = "<UCArenaPuzzle>k__BackingField"]
    pub uc_arena_puzzle: ArrayCString<32>,

    #[rename = "<UCMainChamber>k__BackingField"]
    pub uc_main_chamber: ArrayCString<32>,

    #[rename = "<UCHerder>k__BackingField"]
    pub uc_herder: ArrayCString<32>,

    #[rename = "<UCAmaterasu>k__BackingField"]
    pub uc_amaterasu: ArrayCString<32>,

    #[rename = "<UCNorthPuzzle>k__BackingField"]
    pub uc_north_puzzle: ArrayCString<32>,

    #[rename = "<UCBoss>k__BackingField"]
    pub uc_boss: ArrayCString<32>,

    #[rename = "<UCAmaterasuDwelling>k__BackingField"]
    pub uc_amaterasu_dwelling: ArrayCString<32>,

    /// sb = sencho bridge
    #[rename = "<SBEntrance>k__BackingField"]
    pub sb_entrance: ArrayCString<32>,
    
    #[rename = "<SBEscalation>k__BackingField"]
    pub sb_escalation: ArrayCString<32>,

    #[rename = "<SBWave>k__BackingField"]
    pub sb_wave: ArrayCString<32>,

    #[rename = "<SBSS>k__BackingField"]
    pub sbss: ArrayCString<32>,

    #[rename = "<SBHH>k__BackingField"]
    pub sbhh: ArrayCString<32>,

    #[rename = "<SBNightToDay>k__BackingField"]
    pub sb_night_to_day: ArrayCString<32>,

    /// saci = sakura city
    #[rename = "<Coast2>k__BackingField"]
    pub coast2: ArrayCString<32>,

    #[rename = "<SACIWalls>k__BackingField"]
    pub saci_walls: ArrayCString<32>,

    #[rename = "<SACITier1>k__BackingField"]
    pub saci_tier1: ArrayCString<32>,

    #[rename = "<SACILibraryFloor1>k__BackingField"]
    pub saci_library_floor1: ArrayCString<32>,

    #[rename = "<SACILibraryFloor2>k__BackingField"]
    pub saci_library_floor2: ArrayCString<32>,

    #[rename = "<HallOfHaikus>k__BackingField"]
    pub hall_of_haikus: ArrayCString<32>,

    #[rename = "<ShrineOfTheLillies>k__BackingField"]
    pub shrine_of_the_lillies: ArrayCString<32>,

    #[rename = "<SACISumo>k__BackingField"]
    pub saci_sumo: ArrayCString<32>,

    #[rename = "<SACITargetsMiniGame>k__BackingField"]
    pub saci_targets_mini_game: ArrayCString<32>,
    
    #[rename = "<SACIFoxHouse>k__BackingField"]
    pub saci_fox_house: ArrayCString<32>,

    #[rename = "<SACITier2>k__BackingField"]
    pub saci_tier2: ArrayCString<32>,

    #[rename = "<SACIArmory>k__BackingField"]
    pub saci_armory: ArrayCString<32>,

    #[rename = "<SACIDarumaShop>k__BackingField"]
    pub saci_daruma_shop: ArrayCString<32>,

    #[rename = "<SACITier3>k__BackingField"]
    pub saci_tier3: ArrayCString<32>,

    #[rename = "<SACITeaHouse>k__BackingField"]
    pub saci_tea_house: ArrayCString<32>,

    #[rename = "<GashaBossFight>k__BackingField"]
    pub gasha_boss_fight: ArrayCString<32>,

    #[rename = "<GashaSkull>k__BackingField"]
    pub gasha_skull: ArrayCString<32>,

    /// mf = Midori Forest
    #[rename = "<MFTeaField>k__BackingField"]
    pub mf_tea_field: ArrayCString<32>,

    #[rename = "<MFTeaFieldDestroyed>k__BackingField"]
    pub mf_tea_field_destroyed: ArrayCString<32>,
    
    #[rename = "<MFAsahi>k__BackingField"]
    pub mf_asahi: ArrayCString<32>,

    #[rename = "<MFArena>k__BackingField"]
    pub mf_arena: ArrayCString<32>,

    #[rename = "<MFEscort>k__BackingField"]
    pub mf_escort: ArrayCString<32>,

    #[rename = "<MFShrine>k__BackingField"]
    pub mf_shrine: ArrayCString<32>,

    #[rename = "<MFTreetops>k__BackingField"]
    pub mf_treetops: ArrayCString<32>,

    #[rename = "<MFMaze>k__BackingField"]
    pub mf_maze: ArrayCString<32>,

    #[rename = "<MFBurrowEntry>k__BackingField"]
    pub mf_burrow_entry: ArrayCString<32>,

    #[rename = "<MFBoss>k__BackingField"]
    pub mf_boss: ArrayCString<32>,

    #[rename = "<MFWedding>k__BackingField"]
    pub mf_wedding: ArrayCString<32>,
    
    /// mm = megumi mountain
    #[rename = "<MMBase>k__BackingField"]
    pub mm_base: ArrayCString<32>,
    
    #[rename = "<MMCaveAscent>k__BackingField"]
    pub mm_cave_ascent: ArrayCString<32>,

    #[rename = "<MMVillage>k__BackingField"]
    pub mm_village: ArrayCString<32>,

    #[rename = "<MMVista>k__BackingField"]
    pub mm_vista: ArrayCString<32>,

    #[rename = "<MMMainAscent>k__BackingField"]
    pub mm_main_ascent: ArrayCString<32>,

    #[rename = "<MMTrainingGrounds>k__BackingField"]
    pub mm_training_grounds: ArrayCString<32>,

    #[rename = "<MMSumoLobby>k__BackingField"]
    pub mm_sumo_lobby: ArrayCString<32>,

    #[rename = "<MMSumoStage1>k__BackingField"]
    pub mm_sumo_stage1: ArrayCString<32>,

    #[rename = "<MMSumoStage2>k__BackingField"]
    pub mm_sumo_stage2: ArrayCString<32>,

    #[rename = "<MMYokozuna>k__BackingField"]
    pub mm_yokozuna: ArrayCString<32>,

    #[rename = "<MMSummit>k__BackingField"]
    pub mm_summit: ArrayCString<32>,

    #[rename = "<MMUnderground>k__BackingField"]
    pub mm_underground: ArrayCString<32>,

    #[rename = "<MMGunbai>k__BackingField"]
    pub mm_gunbai: ArrayCString<32>,

    #[rename = "<MMTeaHouse>k__BackingField"]
    pub mm_teahouse: ArrayCString<32>,

    #[rename = "<MMOptionalPuzzle>k__BackingField"]
    pub mm_optional_puzzle: ArrayCString<32>,
    
    /// kb = kitsune burrows
    #[rename = "<KBJorogumo>k__BackingField"]
    pub kb_jorogumo: ArrayCString<32>,
    
    #[rename = "<KBWest>k__BackingField"]
    pub kb_west: ArrayCString<32>,

    #[rename = "<KBSouth>k__BackingField"]
    pub kb_south: ArrayCString<32>,
    
    #[rename = "<KBEntrance>k__BackingField"]
    pub kb_entrance: ArrayCString<32>,

    #[rename = "<KBBugNest>k__BackingField"]
    pub kb_bug_nest: ArrayCString<32>,

    #[rename = "<KBAscent>k__BackingField"]
    pub kb_ascent: ArrayCString<32>,

    /// ic = ice caverns
    #[rename = "<ICTengu1>k__BackingField"]
    pub ic_tengu1: ArrayCString<32>,

    #[rename = "<ICTengu2>k__BackingField"]
    pub ic_tengu2: ArrayCString<32>,

    #[rename = "<ICDaiTengu>k__BackingField"]
    pub ic_daitengu: ArrayCString<32>,
    
    #[rename = "<ICCentral>k__BackingField"]
    pub ic_central: ArrayCString<32>,

    #[rename = "<ICWest>k__BackingField"]
    pub ic_west: ArrayCString<32>,

    #[rename = "<ICEastRedesign>k__BackingField"]
    pub ic_east: ArrayCString<32>,

    #[rename = "<ICVolcano>k__BackingField"]
    pub ic_volcano: ArrayCString<32>,
    
    /// imc = imperial castle
    #[rename = "<IMCEntrance>k__BackingField"]
    pub imc_entrance: ArrayCString<32>,
    
    #[rename = "<IMCFloor1>k__BackingField"]
    pub imc_floor1: ArrayCString<32>,

    #[rename = "<IMCFloor2>k__BackingField"]
    pub imc_floor2: ArrayCString<32>,

    #[rename = "<IMCFloor3>k__BackingField"]
    pub imc_floor3: ArrayCString<32>,

    #[rename = "<IMCElevator>k__BackingField"]
    pub imc_elevator: ArrayCString<32>,

    #[rename = "<IMCAsahi1>k__BackingField"]
    pub imc_asahi1: ArrayCString<32>,

    #[rename = "<IMCAsahi2>k__BackingField"]
    pub imc_asahi2: ArrayCString<32>,

    #[rename = "<IMCAsahi3>k__BackingField"]
    pub imc_asahi3: ArrayCString<32>,

    #[rename = "<IMCFinalAscent>k__BackingField"]
    pub imc_final_ascent: ArrayCString<32>,

    #[rename = "<IMCShogun>k__BackingField"]
    pub imc_shogun: ArrayCString<32>,

    #[rename = "<CelestialRealm>k__BackingField"]
    pub celestial_realm: ArrayCString<32>,

}
#[derive(Class, Copy, Clone, Debug, PartialEq)]
pub struct GameManager {
    #[rename = "<FromMainMenu>k__BackingField"]
    pub from_main_menu: bool,
    #[rename = "<ElevatorEntranceUp>k__BackingField"]
    pub elevator_e_up: bool,
    #[rename = "<ElevatorFloor1Up>k__BackingField"]
    pub elevator_1_up: bool,
    #[rename = "<ElevatorFloor1Down>k__BackingField"]
    pub elevator_1_down: bool,
    #[rename = "<ElevatorFloor2Up>k__BackingField"]
    pub elevator_2_up: bool,
    #[rename = "<ElevatorFloor2Down>k__BackingField"]
    pub elevator_2_down: bool,
    #[rename = "<ElevatorFloor3Up>k__BackingField"]
    pub elevator_3_up: bool,
    #[rename = "<ElevatorFloor3Down>k__BackingField"]
    pub elevator_3_down: bool,
    #[rename = "<VerticalChaseStarted>k__BackingField"]
    pub vertical_chase_start: bool,
    #[rename = "<loadGame>k__BackingField"]
    pub load_game: bool,
    #[rename = "<fromInGame>k__BackingField"]
    pub from_in_game: bool,
    #[rename = "<isQuittingGame>k__BackingField"]
    pub is_quitting: bool,
    #[rename = "<BossPercentage>k__BackingField"]
    pub boss_percentage: f32,
    #[rename = "<QuestManager>k__BackingField"]
    pub quest_pointer: Address64,
    #[rename = "abilityManager"]
    pub ability_pointer: Address64,
    #[rename = "betaDataManager"]
    pub player_data_pointer: Address64,
    #[rename = "inventoryContainer"]
    pub inventory_pointer: Address64,
    #[rename = "enemiesManager"]
    pub enemies_pointer: Address64,
    #[rename = "darumaManager"]
    pub daruma_pointer: Address64,
    #[rename = "omamoriManager"]
    pub omamori_pointer: Address64,
    #[rename = "sceneNameManager"]
    pub scene_name_pointer: Address64,
}
