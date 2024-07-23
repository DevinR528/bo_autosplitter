use asr::settings::{gui::Title, Gui};

#[derive(Gui)]
pub struct Settings {
    /// General Settings
    _general_settings: Title,

    /// Split on starting Asahi's staff quest.
    ///
    /// This is the quest where you collect bamboo to get past Asahi (your first encounter).
    #[default = true]
    pub asahi_staff_start: bool,

    /// Split on completing Ashai's staff quest.
    ///
    /// This is the quest where you collect bamboo to get past Ashai (your first encounter).
    #[default = true]
    pub asahi_staff_end: bool,

    /// Split on starting Asahi's Eye of Beast quest.
    ///
    /// This is the quest where you collect an eye for your kettle (your second encounter).
    #[default = true]
    pub asahi_eye_of_beast_start: bool,

    /// Split on completing Ashai's Eye of Beast quest.
    ///
    /// This is the quest where you collect an eye for your kettle (your second encounter).
    #[default = true]
    pub asahi_eye_of_beast_end: bool,

    /// Split on completing Shimeji Armadillo collection quest.
    ///
    /// This is the quest where you collect 4 Armadillos.
    #[default = true]
    pub shimeji_quest_end: bool,

    /// Split on defeating Kiri Kiri Bozu.
    ///
    /// This is the first boss of the game KiriKiri Bozu.
    /// TODO: currently there is no split for this.
    #[default = false]
    pub defeated_kirikiri_boss: bool,

    /// Split on defeating Particularly Unmanageable Armadillo.
    ///
    /// This is the second boss of the game Particularly Unmanageable Armadillo.
    #[default = true]
    pub defeated_pua_boss: bool,

    /// Split on defeating Hashihime.
    ///
    /// This is the third boss of the game Hashihime.
    #[default = true]
    pub defeat_hashihime_boss: bool,

    /// Split on completing the Vermilion Stranger quest.
    ///
    /// This quest gives fast travel.
    #[default = true]
    pub vermilion_stranger_quest_end: bool,

    /// Split on defeating Kaboto Yokozuma (the beatle).
    ///
    /// This is the fourth boss of the game Kaboto Yokozuma.
    #[default = true]
    pub defeat_kaboto_boss: bool,

    /// Split on defeating Jorogumo (the spider).
    ///
    /// This is the fifth boss of the game Jorogumo.
    #[default = true]
    pub defeat_jorogumo_boss: bool,

    /// Split on defeating Tengu Trio.
    ///
    /// This is the sixth boss of the game Tengu Trio.
    /// TODO: this split is unconfirmed, it is based on `QuestManager::TenguTrialQuestCompleted`.
    #[default = true]
    pub defeat_tengu_boss: bool,

    /// Split on inserting the first feather in keyhole.
    ///
    /// Upon inserting the first key into the west keyhole.
    #[default = true]
    pub west_feather_in_keyhole: bool,

    /// Split on inserting the second feather in keyhole.
    ///
    /// Upon inserting the second key into the east keyhole.
    #[default = true]
    pub east_feather_in_keyhole: bool,

    /// Split on defeating Gashadokuro (the giant skeleton).
    ///
    /// This is the third boss of the game Gashadokuro.
    #[default = true]
    pub defeat_gash_boss: bool,

    /// Split on defeating Gashadokuro (the giant skeleton).
    ///
    /// This is the third boss of the game Gashadokuro.
    #[default = true]
    pub defeat_ashai_boss: bool,

     /// Split on gaining atack ability.
    ///
    /// This is given once bamboo is collected for Asahi.
    #[default = true]
    pub can_attack: bool,

    /// Split on gaining bat ability.
    ///
    /// This is given once KiriKiri Bozu is defeated.
    #[default = true]
    pub can_bat: bool,

    /// Split on gaining dash ability.
    ///
    /// This is given once Asahi get eye of beast.
    #[default = true]
    pub can_dash: bool,

    /// Split on gaining hover ability.
    ///
    /// This is given after beating Kaboto.
    #[default = true]
    pub can_hover: bool,

    /// Split on gaining shade cloak dash ability.
    ///
    /// This is given after destroying nests in spider layer.
    #[default = true]
    pub can_idash: bool,

    /// Split on gaining grapple ability.
    ///
    /// This is given after getting 3 music sheets.
    #[default = true]
    pub can_grapple: bool,

    /// Split on gaining hammer dash ability.
    ///
    /// This is given after destroying nests in spider layer.
    #[default = true]
    pub can_hammer_dash: bool,

    /// Split on gaining wall jump ability.
    ///
    /// This is given after west side ice palace.
    #[default = true]
    pub can_wall_jump: bool,
}
