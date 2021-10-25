use std::fs::File;
use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Recipe {
    recipes: Vec<serde_json::Value>
}

// fn main() {
//     let file = File::open("src/data.json").expect("Error opening file.");
//     let json : Recipe = serde_json::from_reader(file).expect("Error reading JSON");
//     for ele in json.recipes {
//         println!("{}", ele["name"]);
//     }
// }
#[macro_use] extern crate rocket;

#[derive(Serialize)]
struct Recipes {
    recipe_names: Vec<String>
}
#[get("/recipes")]
fn get_recipes() -> Json<Recipes> {
    let file = File::open("src/data.json").expect("Error opening file.");
    let json : Recipe = serde_json::from_reader(file).expect("Error reading JSON");
    let mut recipes : Vec<String> = Vec::new();
    for ele in json.recipes {
        recipes.push(String::from(ele["name"].as_str().unwrap()));
    }
    Json(Recipes {recipe_names : recipes})
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_recipes])
}
