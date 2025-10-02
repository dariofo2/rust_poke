mod classes;
mod http;
mod gtkGraphic;

use std::{
    self,
    fs::{self}
};

use crate::{
    classes::{binaryConvers::{BinaryConvers}, classroom::Classroom, pokemonImg::PokemonImg}, http::http::HttpPokemon
};

#[tokio::main]
async fn main() {
    BinaryConvers::test();
    gtkGraphic::gtkGraphic::gtkGraphic::runGraphicGTK();

    Classroom::classroomMenu().await;

    HttpPokemon::pokemon_menu().await;
}
