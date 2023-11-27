use super::battle_apis::get_battles;
use super::monster_apis::{
    create_monster, delete_monster_by_id, get_monster_by_id, get_monsters, import_csv,
    update_monster_by_id,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_monsters)
            .service(create_monster)
            .service(get_monster_by_id)
            .service(delete_monster_by_id)
            .service(update_monster_by_id)
            .service(import_csv)
            .service(get_battles),
    );
}
