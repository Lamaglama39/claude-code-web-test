use axum::{
    routing::get,
    Json, Router,
};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Shrimp {
    id: u32,
    name: String,
    japanese_name: String,
    species: String,
    size_cm: f32,
    habitat: String,
    color: String,
    diet: String,
    lifespan_years: f32,
    fun_fact: String,
}

fn get_shrimp_data() -> Vec<Shrimp> {
    vec![
        Shrimp {
            id: 1,
            name: "Cherry Shrimp".to_string(),
            japanese_name: "チェリーシュリンプ".to_string(),
            species: "Neocaridina davidi".to_string(),
            size_cm: 2.5,
            habitat: "Freshwater streams and ponds in Taiwan".to_string(),
            color: "Bright red".to_string(),
            diet: "Algae, biofilm, and detritus".to_string(),
            lifespan_years: 1.5,
            fun_fact: "They can change color based on their diet and environment!".to_string(),
        },
        Shrimp {
            id: 2,
            name: "Mantis Shrimp".to_string(),
            japanese_name: "シャコ".to_string(),
            species: "Odontodactylus scyllarus".to_string(),
            size_cm: 18.0,
            habitat: "Coral reefs in Indo-Pacific".to_string(),
            color: "Rainbow (green, blue, red, orange)".to_string(),
            diet: "Mollusks, crabs, and fish".to_string(),
            lifespan_years: 20.0,
            fun_fact: "Has the fastest punch in the animal kingdom at 50 mph!".to_string(),
        },
        Shrimp {
            id: 3,
            name: "Amano Shrimp".to_string(),
            japanese_name: "ヤマトヌマエビ".to_string(),
            species: "Caridina multidentata".to_string(),
            size_cm: 5.0,
            habitat: "Rivers and streams in Japan".to_string(),
            color: "Translucent with brown spots".to_string(),
            diet: "Algae and plant matter".to_string(),
            lifespan_years: 3.0,
            fun_fact: "Named after Takashi Amano, famous aquascaper!".to_string(),
        },
        Shrimp {
            id: 4,
            name: "Ghost Shrimp".to_string(),
            japanese_name: "ゴーストシュリンプ".to_string(),
            species: "Palaemonetes paludosus".to_string(),
            size_cm: 4.0,
            habitat: "Freshwater streams in North America".to_string(),
            color: "Transparent".to_string(),
            diet: "Omnivore - algae, plants, and small organisms".to_string(),
            lifespan_years: 1.0,
            fun_fact: "You can see their internal organs through their transparent body!".to_string(),
        },
        Shrimp {
            id: 5,
            name: "Crystal Red Shrimp".to_string(),
            japanese_name: "クリスタルレッドシュリンプ".to_string(),
            species: "Caridina cantonensis".to_string(),
            size_cm: 3.0,
            habitat: "Selective breeding from bees shrimp in Japan".to_string(),
            color: "Red and white stripes".to_string(),
            diet: "Biofilm, algae, and specialized shrimp food".to_string(),
            lifespan_years: 2.0,
            fun_fact: "One of the most expensive aquarium shrimp varieties!".to_string(),
        },
        Shrimp {
            id: 6,
            name: "Pistol Shrimp".to_string(),
            japanese_name: "テッポウエビ".to_string(),
            species: "Alpheus heterochaelis".to_string(),
            size_cm: 5.0,
            habitat: "Tropical seas worldwide".to_string(),
            color: "Brown to olive green".to_string(),
            diet: "Small fish and worms".to_string(),
            lifespan_years: 4.0,
            fun_fact: "Creates a bubble that collapses with a sound louder than a gunshot!".to_string(),
        },
        Shrimp {
            id: 7,
            name: "Blue Velvet Shrimp".to_string(),
            japanese_name: "ブルーベルベットシュリンプ".to_string(),
            species: "Neocaridina davidi".to_string(),
            size_cm: 2.5,
            habitat: "Freshwater (selectively bred)".to_string(),
            color: "Deep blue".to_string(),
            diet: "Algae, biofilm, and vegetables".to_string(),
            lifespan_years: 2.0,
            fun_fact: "A color variation of cherry shrimp bred for deep blue color!".to_string(),
        },
        Shrimp {
            id: 8,
            name: "Kuruma Prawn".to_string(),
            japanese_name: "クルマエビ".to_string(),
            species: "Marsupenaeus japonicus".to_string(),
            size_cm: 20.0,
            habitat: "Sandy bottoms in shallow seas around Japan".to_string(),
            color: "Brown with dark bands".to_string(),
            diet: "Worms, mollusks, and small crustaceans".to_string(),
            lifespan_years: 2.5,
            fun_fact: "A prized delicacy in Japanese cuisine!".to_string(),
        },
        Shrimp {
            id: 9,
            name: "Bumblebee Shrimp".to_string(),
            japanese_name: "バンブルビーシュリンプ".to_string(),
            species: "Caridina breviata".to_string(),
            size_cm: 3.5,
            habitat: "Mountain streams in China".to_string(),
            color: "Black and white stripes".to_string(),
            diet: "Biofilm and algae".to_string(),
            lifespan_years: 2.0,
            fun_fact: "Their striped pattern resembles a bumblebee!".to_string(),
        },
        Shrimp {
            id: 10,
            name: "Spot Prawn".to_string(),
            japanese_name: "ボタンエビ".to_string(),
            species: "Pandalus platyceros".to_string(),
            size_cm: 23.0,
            habitat: "Deep waters of the Pacific Ocean".to_string(),
            color: "Reddish-brown with white spots".to_string(),
            diet: "Plankton and small organisms".to_string(),
            lifespan_years: 4.0,
            fun_fact: "They are born male and become female as they mature!".to_string(),
        },
    ]
}

async fn health_check() -> &'static str {
    "Shrimp API is running!"
}

async fn get_random_shrimp() -> Json<Shrimp> {
    let shrimps = get_shrimp_data();
    let mut rng = rand::thread_rng();
    let random_shrimp = shrimps.choose(&mut rng).unwrap().clone();
    Json(random_shrimp)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(health_check))
        .route("/api/shrimp/random", get(get_random_shrimp));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🦐 Shrimp API server listening on http://{}", addr);
    println!("📍 Endpoints:");
    println!("   - GET /                    - Health check");
    println!("   - GET /api/shrimp/random   - Get random shrimp");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
