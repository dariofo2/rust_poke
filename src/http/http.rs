use std::{
    collections::HashMap,
    fs,
    io::{self, Bytes, Error, Read},
};

use reqwest::Client;
use serde_json::Value;

pub struct HttpPokemon {}

impl HttpPokemon {
    pub async fn getPokemon(name: String) -> Result<String, reqwest::Error> {
        let client = Client::new();

        let url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
        let res = client.get(url).send().await;
        let text = res?.text().await?;
        return Ok(text);
    }

    pub async fn getImage(url: String) -> Result<Vec<u8>, reqwest::Error> {
        let client = Client::new();

        let res = client.get(url).send().await?;
        let bytes = res.bytes().await?;

        return Ok(bytes.to_vec());
    }

    /**
     * Get Pokemon Images and Write em To files/pok_name
     */
    pub async fn get_pokemon_images(poke_name: String) -> Result<bool, Error> {
        let pokemon: Result<String, reqwest::Error> = HttpPokemon::getPokemon(poke_name).await;
        if let Ok(pok) = pokemon {
            //println!("{}",pok);
            let json_obj: Value = serde_json::from_str(&pok).unwrap();

            let name = json_obj["species"]["name"]
                .as_str()
                .unwrap_or("unknown")
                .to_string();
            
            for spriteMap in &json_obj["sprites"].as_object() {
                for sprite in spriteMap.iter() {
                    let sprite_name = sprite.0;
                    let sprite_value = sprite.1;

                    if !sprite_value.is_null() && sprite_value.is_string() {
                        //println!("{} {}", sprite_name, sprite_value);

                        let img = HttpPokemon::getImage(sprite_value.as_str().unwrap().to_string()).await;
                        if let Ok(im) = img {
                            let path = format!("files/{}/{}", name,sprite_name);
                            println!("{path}");
                            if !fs::exists(&path)? {
                                fs::create_dir_all(&path)?;
                            }
                            fs::write(format!("{}/img.png", path), im)?;
                        }
                    }
                }
            }

            return Ok(true);
        }
        return Ok(false);
    }

    pub async fn pokemon_menu() {
        while true {
            let mut output = String::new();
            let inLine = io::stdin();

            println!("\nEnter a Pokemon Name\n2.Exit");
            inLine.read_line(&mut output);

            HttpPokemon::get_pokemon_images(output.trim().to_string()).await;
        }
    }
}
