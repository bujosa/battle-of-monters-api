use crate::models::monster::Monster;
use crate::repository::battle_repository;
use crate::repository::monster_repository;
use crate::{models::battle::Battle, repository::database::Database};
use actix_web::{delete, get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBattleRequest {
    monster_a: Option<String>,
    monster_b: Option<String>,
}

#[get("/battles")]
pub async fn get_battles(db: web::Data<Database>) -> HttpResponse {
    let battles = battle_repository::get_battles(&db);
    HttpResponse::Ok().json(battles)
}

#[post("/battles")]
pub async fn create_battle(
    db: web::Data<Database>,
    battle_request: web::Json<CreateBattleRequest>,
) -> HttpResponse {
    let monster_a_id = match &battle_request.monster_a {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json("Monster A ID is missing"),
    };

    let monster_b_id = match &battle_request.monster_b {
        Some(id) => id,
        None => return HttpResponse::BadRequest().json("Monster B ID is missing"),
    };

    let monster_a = match monster_repository::get_monster_by_id(&db, monster_a_id) {
        Some(monster) => monster,
        None => return HttpResponse::NotFound().json("Monster B not found"),
    };

    let monster_b = match monster_repository::get_monster_by_id(&db, monster_b_id) {
        Some(monster) => monster,
        None => return HttpResponse::NotFound().json("Monster B not found"),
    };

    let battle_result = battle(monster_a, monster_b);

    match battle_repository::create_battle(&db, &battle_result) {
        Ok(battle) => HttpResponse::Created().json(battle),
        Err(_) => HttpResponse::InternalServerError().json("Error creating battle"),
    }
}

#[delete("/battles/{battle_id}")]
pub async fn delete_battle(db: web::Data<Database>, battle_id: web::Path<String>) -> HttpResponse {
    match battle_repository::delete_battle_by_id(&db, &battle_id) {
        Some(_) => HttpResponse::NoContent().finish(),
        None => HttpResponse::NotFound().json("Battle not found"),
    }
}

#[get("/battles/{battle_id}")]
pub async fn get_battle(db: web::Data<Database>, battle_id: web::Path<String>) -> HttpResponse {
    match battle_repository::get_battle_by_id(&db, &battle_id) {
        Some(battle) => HttpResponse::Ok().json(battle),
        None => HttpResponse::NotFound().json("Battle not found"),
    }
}

fn battle(mut monster_a: Monster, mut monster_b: Monster) -> Battle {
    let mut turn = if monster_a.speed > monster_b.speed
        || (monster_a.speed == monster_b.speed && monster_a.attack > monster_b.attack)
    {
        'a'
    } else {
        'b'
    };

    loop {
        if turn == 'a' {
            let damage = if monster_a.attack > monster_b.defense {
                monster_a.attack - monster_b.defense
            } else {
                1
            };
            monster_b.hp -= damage;
            if monster_b.hp <= 0 {
                return Battle {
                    id: "".to_string(),
                    monster_a: monster_a.id.clone(),
                    monster_b: monster_b.id,
                    winner: monster_a.id.clone(),
                    created_at: None,
                    updated_at: None,
                };
            }
            turn = 'b';
        } else {
            let damage = if monster_b.attack > monster_a.defense {
                monster_b.attack - monster_a.defense
            } else {
                1
            };
            monster_a.hp -= damage;
            if monster_a.hp <= 0 {
                return Battle {
                    id: "".to_string(),
                    monster_a: monster_a.id,
                    monster_b: monster_b.id.clone(),
                    winner: monster_b.id.clone(),
                    created_at: None,
                    updated_at: None,
                };
            }
            turn = 'a';
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{utils::test_utils::init_test_battle, utils::test_utils::init_test_monsters};
    use actix_web::web::Data;
    use actix_web::{http, test, App};
    use serde_json;

    use super::*;

    #[actix_rt::test]
    async fn test_should_get_all_battles_correctly() {
        let db = Database::new();
        let app = App::new().app_data(Data::new(db)).service(get_battles);

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/battles").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_should_get_404_error_if_battle_does_not_exists() {
        let db = Database::new();
        let app = App::new().app_data(Data::new(db)).service(get_battle);

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/battles/123").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_should_get_a_single_battle_correctly() {
        let db = Database::new();
        let battle = init_test_battle(&db).await;
        let app = App::new().app_data(Data::new(db)).service(get_battle);

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get()
            .uri(&format!("/battles/{}", battle.id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_should_delete_a_battle_correctly() {
        let db = Database::new();
        let battle = init_test_battle(&db).await;
        let app = App::new().app_data(Data::new(db)).service(delete_battle);

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::delete()
            .uri(&format!("/battles/{}", battle.id))
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_should_delete_with_404_error_if_battle_does_not_exists() {
        let db = Database::new();
        let app = App::new().app_data(Data::new(db)).service(delete_battle);

        let mut app = test::init_service(app).await;

        let req = test::TestRequest::delete().uri("/battles/123").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_should_create_a_battle_with_404_error_if_one_parameter_has_a_monster_id_does_not_exists(
    ) {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[0].id.clone()),
            monster_b: Some("123".to_string()),
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }

    #[actix_rt::test]
    async fn test_should_create_a_battle_with_a_bad_request_response_if_one_parameter_is_null() {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[0].id.clone()),
            monster_b: None,
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning() {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[1].id.clone()),
            monster_b: Some(test_monsters[0].id.clone()),
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        let battle_result: Battle =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        assert_eq!(battle_result.winner, test_monsters[1].id.clone());
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_b_winning() {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[0].id.clone()),
            monster_b: Some(test_monsters[1].id.clone()),
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;

        let battle_result: Battle =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        assert_eq!(battle_result.winner, test_monsters[1].id.clone());
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning_if_theirs_speeds_same_and_monster_a_has_higher_attack(
    ) {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[2].id.clone()),
            monster_b: Some(test_monsters[5].id.clone()),
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;

        let battle_result: Battle =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        assert_eq!(battle_result.winner, test_monsters[2].id.clone());
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_b_winning_if_theirs_speeds_same_and_monster_b_has_higher_attack(
    ) {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[5].id.clone()),
            monster_b: Some(test_monsters[2].id.clone()),
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;

        let battle_result: Battle =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        assert_eq!(battle_result.winner, test_monsters[2].id.clone());
    }

    #[actix_rt::test]
    async fn test_should_create_battle_correctly_with_monster_a_winning_if_theirs_defense_same_and_monster_a_has_higher_speed(
    ) {
        let db = Database::new();
        let test_monsters = init_test_monsters(&db).await;
        let app = App::new().app_data(Data::new(db)).service(create_battle);

        let mut app = test::init_service(app).await;

        let battle_request = CreateBattleRequest {
            monster_a: Some(test_monsters[6].id.clone()),
            monster_b: Some(test_monsters[7].id.clone()),
        };

        let req = test::TestRequest::post()
            .uri("/battles")
            .set_json(&battle_request)
            .to_request();

        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;

        let battle_result: Battle =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        assert_eq!(battle_result.winner, test_monsters[6].id.clone());
    }
}
