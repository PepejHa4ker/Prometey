pub struct Settings {

    // Aim
    pub silent_enabled: bool,
    pub silent_players: bool,
    pub silent_zombies: bool,
    pub silent_draw_fov: bool,
    pub silent_fov: i32,
    pub silent_dist: i32,
    pub silent_bone: i32,

    // Loot
    pub loot_enabled: bool,
    pub ammo_enabled: bool,
    pub items_enabled: bool,
    pub clothes_enabled: bool,
    pub backpack_enabled: bool,
    pub medicines_enabled: bool,
    pub explosives_enabled: bool,
    pub food_enabled: bool,
    pub weapons_enabled: bool,
    pub building_enabled: bool,
    pub car_items_enabled: bool,
    pub attachments_enabled: bool,
    pub consumables_enabled: bool,
    pub loot_distance: i32,
    //Visuals
    // Misc
    pub noclip_enabled: bool,
    pub noclip_range: i32,
    pub debug_camera: bool,
    pub camera_speed: f32,
    pub remove_grass: bool,
    pub recoil_control: bool,
    pub anti_spread: bool,
}

impl Settings {

    pub const fn new() -> Self {
        Settings {
            silent_enabled: false,
            silent_players: false,
            silent_zombies: false,
            silent_draw_fov: false,
            silent_fov: 45,
            silent_dist: 900,
            silent_bone: 0,
            loot_enabled: false,
            ammo_enabled: false,
            items_enabled: false,
            clothes_enabled: false,
            backpack_enabled: false,
            medicines_enabled: false,
            explosives_enabled: false,
            food_enabled: false,
            weapons_enabled: false,
            building_enabled: false,
            car_items_enabled: false,
            attachments_enabled: false,
            consumables_enabled: false,
            loot_distance: 150,
            noclip_enabled: false,
            debug_camera: false,
            camera_speed: 0.150,
            remove_grass: false,
            recoil_control: false,
            anti_spread: false,
            noclip_range: 750
        }
    }

}

