use std::fmt::{self, Debug};

use asr::{game_engine::unity::mono::Class, Address64};

#[derive(Debug, Class, Copy, Clone, PartialEq)]
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

#[derive(Debug, Class, Copy, Clone, PartialEq)]
pub struct BetaPlayerDataManager {
    #[rename = "<TimePlayed>k__BackingField"]
    pub time_played: f32,
}

#[derive(Debug, Class, Copy, Clone, PartialEq, Eq)]
pub struct QuestManager {
    #[rename = "<AsahiBambooStaffQuestStarted>k__BackingField"]
    pub asahi_staff_start: bool,
    #[rename = "<AsahiBambooStaffQuestCompleted>k__BackingField"]
    pub asahi_staff_end: bool,
    #[rename = "<AsahiEyeOfTheBeastQuestStarted>k__BackingField"]
    pub asahi_eye_of_beast_start: bool,
    #[rename = "<AsahiEyeOfTheBeastQuestCompleted>k__BackingField"]
    pub asahi_eye_of_beast_end: bool,
    #[rename = "<AsahiAfterArmapilloBoss>k__BackingField"]
    pub asahi_post_armapillo_boss: bool,
    #[rename = "<ToriBumpProphecyTold>k__BackingField"]
    pub tori_bump_told: bool,
    #[rename = "<ToriFulfilledBumpProphecy>k__BackingField"]
    pub tori_bump_end: bool,
    #[rename = "<ToriBatProphecyTold>k__BackingField"]
    pub tori_bat_told: bool,
    #[rename = "<ToriFulfilledBatProphecy>k__BackingField"]
    pub tori_bat_end: bool,
    #[rename = "<ToriDashProphecyTold>k__BackingField"]
    pub tori_dash_told: bool,
    #[rename = "<ToriFulfilledDashProphecy>k__BackingField"]
    pub tori_dash_end: bool,
    #[rename = "<ShimejiArmapillosCollected>k__BackingField"]
    pub shimeji_armapillos_collect: i32,
    #[rename = "<ShimejiQuestStarted>k__BackingField"]
    pub shimeji_quest_start: bool,
    #[rename = "<ShimejiQuestCompleted>k__BackingField"]
    pub shimeji_quest_end: bool,

    /// The Vermilion Stranger quest (this gives fast travel)
    #[rename = "<VSQuestCompleted>k__BackingField"]
    pub vermilion_stranger_quest_end: bool,

    /// This is the second boss of the game "Particularly Unmanageable Armadillo"
    #[rename = "<DefeatedPUABoss>k__BackingField"]
    pub defeat_pua_boss: bool,

    /// This is the third boss of the game "Hashihime" (bridge wave lady)
    #[rename = "<HashihimeDefeated>k__BackingField"]
    pub defeat_hashihime_boss: bool,

    // TODO: missing Kitsura (the nine tails thing)
    /// This is the fourth boss in the game "Kaboto Yokozuma" (the beatle)
    #[rename = "<YokozumaCompleted>k__BackingField"]
    pub defeat_kaboto_boss: bool,

    /// This is the fith boss in the game "Jorogumo" (the spider)
    #[rename = "<DefeatedJorogumo>k__BackingField"]
    pub defeat_spider_boss: bool,

    /// This is the sixth boss in the game "Tengu" (the 3 bird warriors)
    #[rename = "<TenguTrialQuestCompleted>k__BackingField"]
    pub defeat_tengu_boss: bool,

    /// Inserting the feather from the east side of white --- I mean ice palace into the keyhole
    #[rename = "<FirstFeatherKeyEntered>k__BackingField"]
    pub keyhole_east_feather: bool,

    /// Inserting the second feather from the west side of white--- I mean ice palace
    #[rename = "<SecondFeatherKeyEntered>k__BackingField"]
    pub keyhole_west_feather: bool,

    /// This is the seventh boss in the game "Gashadokuro" (giant skeleton)
    #[rename = "<GashaDefeated>k__BackingField"]
    pub defeat_gash_boss: bool,

    /// This is the eigth boss in the game "Asahi" (Your buddy)
    #[rename = "<AsahiDefeated>k__BackingField"]
    pub defeat_asahi_boss: bool,

    /// This is the ninth boss in the game "Sakura Shogun" (Final Boss)
    #[rename = "<ShogunDefeated>k__BackingField"]
    pub defeat_shogun_boss: bool,
    // TODO: more of these...
}

#[derive(Class, Copy, Clone, PartialEq)]
pub struct GameManager {
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
    #[rename = "<BossPercentage>k__BackingField"]
    pub boss_percentage: f32,
    #[rename = "<QuestManager>k__BackingField"]
    pub quest_pointer: Address64,
    #[rename = "abilityManager"]
    pub ability_pointer: Address64,
    #[rename = "betaDataManager"]
    pub player_data_pointer: Address64,
}

impl Debug for GameManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameManager")
            .field("elevator_e_up", &self.elevator_e_up)
            .field("elevator_1_up", &self.elevator_1_up)
            .field("elevator_1_down", &self.elevator_1_down)
            .field("elevator_2_up", &self.elevator_2_up)
            .field("elevator_2_down", &self.elevator_2_down)
            .field("elevator_3_up", &self.elevator_3_up)
            .field("elevator_3_down", &self.elevator_3_down)
            .field("vertical_chase_start", &self.vertical_chase_start)
            .field("load_game", &self.load_game)
            .field("from_in_game", &self.from_in_game)
            .field("boss_percentage", &self.boss_percentage)
            .field("quest_pointer", &self.quest_pointer)
            .field("ability_pointer", &self.ability_pointer)
            .field("player_data_pointer", &self.player_data_pointer)
            .finish()
    }
}
