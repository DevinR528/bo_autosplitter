use asr::settings::{gui::Title, Gui};

#[derive(Gui, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Category {
    /// Any%
    #[default]
    AnyPercent,
    /// 100%
    HundredPercent,
}

#[derive(Gui)]
pub struct Settings {
    /// General Settings
    _general_settings: Title,

    /// Select your category, the default splits will be selected for you.
    pub category: Category,

    /// Split on starting Asahi's staff quest.
    ///
    /// This is the quest where you collect bamboo to get past Asahi (your first encounter).
    #[default = false]
    pub asahi_staff_start: bool,

    /// Split on completing Ashai's staff quest.
    ///
    /// This is the quest where you collect bamboo to get past Ashai (your first encounter).
    #[default = false]
    pub asahi_staff_end: bool,

    /// Split on starting Asahi's Eye of Beast quest.
    ///
    /// This is the quest where you collect an eye for your kettle (your second encounter).
    #[default = false]
    pub asahi_eye_of_beast_start: bool,

    /// Split on completing Ashai's Eye of Beast quest.
    ///
    /// This is the quest where you collect an eye for your kettle (your second encounter).
    #[default = false]
    pub asahi_eye_of_beast_end: bool,

    /// Split on starting Shimeji Armadillo collection quest.
    ///
    /// This is the quest where you collect 4 Armadillos.
    #[default = false]
    pub shimeji_quest_start: bool,

    /// Split on completing Shimeji Armadillo collection quest.
    ///
    /// This is the quest where you collect 4 Armadillos.
    #[default = false]
    pub shimeji_quest_end: bool,

    /// Split on inserting the first feather in keyhole.
    ///
    /// Upon inserting the first key into the west keyhole.
    #[default = false]
    pub west_feather_in_keyhole: bool,

    /// Split on inserting the second feather in keyhole.
    ///
    /// Upon inserting the second key into the east keyhole.
    #[default = false]
    pub east_feather_in_keyhole: bool,

    /// Split on obtaining the first feather.
    ///
    /// Upon obtaining the west side feather.
    #[default = false]
    pub first_feather_key: bool,

    /// Split on obtaining the second feather.
    ///
    /// Upon obtaining the east side feather.
    #[default = false]
    pub second_feather_key: bool,

    /// Split on starting the Vermilion Stranger quest.
    ///
    /// This quest gives fast travel.
    #[default = false]
    pub vermilion_stranger_quest_start: bool,

    /// Split on completing the Vermilion Stranger quest.
    ///
    /// This quest gives fast travel.
    #[default = false]
    pub vermilion_stranger_quest_end: bool,

    /// Split on entering the first palace elevator.
    ///
    /// This is the first elevator when entering the castle.
    #[default = false]
    pub first_elevator_up: bool,

    /// Split on starting the fox wedding quest.
    ///
    /// This splits on starting of the fox wedding quest (after burrows).
    #[default = false]
    pub fox_wedding_start: bool,

    /// Split on finishing the fox wedding quest.
    ///
    /// This splits on completion of the fox wedding quest (after burrows).
    #[default = false]
    pub fox_wedding_end: bool,

    /// Split on defeating Kiri Kiri Bozu.
    ///
    /// This is the first boss of the game KiriKiri Bozu.
    /// TODO: currently there is no split for this.
    #[default = false]
    pub defeated_kirikiri_boss: bool,

    /// Split on defeating Particularly Unmanageable Armadillo.
    ///
    /// This is the second boss of the game Particularly Unmanageable Armadillo.
    #[default = false]
    pub defeated_pua_boss: bool,

    /// Split on defeating Hashihime.
    ///
    /// This is the third boss of the game Hashihime.
    #[default = false]
    pub defeat_hashihime_boss: bool,

    /// Split on defeating Kaboto Yokozuma (the beatle).
    ///
    /// This is the fourth boss of the game Kaboto Yokozuma.
    #[default = false]
    pub defeat_kaboto_boss: bool,

    /// Split on defeating Jorogumo (the spider).
    ///
    /// This is the fifth boss of the game Jorogumo.
    #[default = false]
    pub defeat_jorogumo_boss: bool,

    /// Split on defeating KarasuTengu Trio.
    ///
    /// This is the sixth boss of the first Tengu of the Trio.
    #[default = false]
    pub defeat_karasu_tengu_boss: bool,

    /// Split on defeating DaiTengu Trio.
    ///
    /// This is the sixth boss of the second Tengu of the Trio.
    #[default = false]
    pub defeat_dai_tengu_boss: bool,

    /// Split on defeating Gashadokuro (the giant skeleton).
    ///
    /// This is the sixth boss of the game Gashadokuro.
    #[default = false]
    pub defeat_gash_boss: bool,

    /// Split on defeating Ashai.
    ///
    /// This is the second to last boss of the game.
    #[default = false]
    pub defeat_ashai_boss: bool,

    /// Split on defeating Sakura Shogun.
    ///
    /// This is the final boss.
    #[default = false]
    pub defeat_sakura_boss: bool,

     /// Split on gaining atack ability.
    ///
    /// This is given once bamboo is collected for Asahi.
    #[default = false]
    pub can_attack: bool,

    /// Split on gaining bat ability.
    ///
    /// This is given once KiriKiri Bozu is defeated.
    #[default = false]
    pub can_bat: bool,

    /// Split on gaining dash ability.
    ///
    /// This is given once Asahi get eye of beast.
    #[default = false]
    pub can_dash: bool,

    /// Split on gaining hover ability.
    ///
    /// This is given after beating Kaboto.
    #[default = false]
    pub can_hover: bool,

    /// Split on gaining shade cloak dash ability.
    ///
    /// This is given after destroying nests in spider layer.
    #[default = false]
    pub can_idash: bool,

    /// Split on gaining grapple ability.
    ///
    /// This is given after getting 3 music sheets.
    #[default = false]
    pub can_grapple: bool,

    /// Split on gaining hammer dash ability.
    ///
    /// This is given after destroying nests in spider layer.
    #[default = false]
    pub can_hammer_dash: bool,

    /// Split on gaining wall jump ability.
    ///
    /// This is given after west side ice palace.
    #[default = false]
    pub can_wall_jump: bool,

    /// Split on gaining Chomper Daruma.
    ///
    /// This is given early in Caves.
    #[default = false]
    pub got_chomper_daruma: bool,

    /// Split on gaining Kaboomaru.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_kaboomaru_daruma: bool,

    /// Split on gaining Yuki.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_yuki_daruma: bool,

    /// Split on gaining Jingu.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_jingu_daruma: bool,

    /// Split on gaining Mamori.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_mamori_daruma: bool,

    /// Split on gaining Ken.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_ken_daruma: bool,

    /// Split on gaining PyroKun.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_pyro_daruma: bool,

    /// Split on gaining TogiChan.
    ///
    /// This is given at the shop.
    #[default = false]
    pub got_togichan_daruma: bool,

     /// Split on credits appearing.
    ///
    /// This is once the game has been beaten.
    #[default = false]
    pub credits_roll: bool,
}

pub const ANY_PERCENT: &[&str] = &[
    "defeated_kirikiri_boss", // KiriKiri Bozu split
    "defeated_pua_boss", // PUA defeated (armadillo) split
    "defeat_hashihime_boss", // Hashihime defeated (bridge) split
    "vermilion_stranger_quest_end", // Fast Travel split
    "defeat_kaboto_boss", // Yokozuna Kaboto (hover) split
    "can_grapple", // Sheet music split
    "can_idash", // Shade Cloak (i-dash) split
    "can_hammer_dash", // Dive (mallet dive) split
    "defeat_jorogumo_boss", // Spider boss split
    "fox_wedding_end", // Fox wedding split (Kitsune Kifuda, Ingenuity Omamori)
    "can_wall_jump", // Wall jump split (West ice palace)
    // "first_feather_key", // First feather (West ice palace)
    "second_feather_key", // DaiTangu second feather key (East ice palace)
    "defeat_gash_boss", // Defeat Gashadokuro skeleton
    "first_elevator_up", // Palace then enter first elevator
    "defeat_ashai_boss", // Defeat Asahi boss
    "defeat_sakura_boss", // Defeat Sakura Shogun final boss
    "credits_roll", // Defeat Sakura Shogun final boss
];

#[allow(dead_code)]
// TODO: Make this actually have the 100% splits
pub const HUNDRED_PERCENT: &[&str] = &[
    "can_bat", // KiriKiri Bozu split
    "defeated_pua_boss", // PUA defeated (armadillo) split
    "defeat_hashihime_boss", // Hashihime defeated (bridge) split
    "vermilion_stranger_quest_end", // Fast Travel split
    "defeat_kaboto_boss", // Yokozuna Kaboto (hover) split
    "can_grapple", // Sheet music split
    "can_idash", // Shade Cloak (i-dash) split
    "can_hammer_dash", // Dive (mallet dive) split
    "defeat_jorogumo_boss", // Spider boss split
    "fox_wedding_end", // Fox wedding split (Kitsune Kifuda, Ingenuity Omamori)
    "can_wall_jump", // Wall jump split (West ice palace)
    "second_feather_key", // DaiTangu second feather key (East ice palace)
    "defeat_gash_boss", // Defeat Gashadokuro skeleton
    "first_elevator_up", // Palace then enter first elevator
    "defeat_ashai_boss", // Defeat Asahi boss
    "defeat_sakura_boss", // Defeat Sakura Shogun final boss
];
