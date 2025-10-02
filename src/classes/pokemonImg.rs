use std::fs;

#[derive(Clone, Debug)]
pub struct PokemonImg {
    pub pokemonName:String,
    pub typePaths:Vec<String>,
    pub imgPaths:Vec<String>
}

impl PokemonImg {
    pub fn new () -> Self {
        let pokName=String::new();
        let vec1:Vec<String>=vec![];
        let vec2:Vec<String>=vec![];

        return Self {pokemonName:pokName,typePaths:vec1,imgPaths:vec2};
    }

    pub fn get () -> Vec<PokemonImg> {
        let dir = fs::read_dir("files").unwrap();

        let mut pokemonImgs: Vec<PokemonImg> = vec![];
        //Get Pokemon Imgs
        for file in dir.into_iter() {
            if let Ok(file) = file {
                let itemPath = file.file_name().into_string();
                if let Ok(itemPath) = itemPath {
                    let mut pok = PokemonImg::new();
                    pok.pokemonName = itemPath;

                    let dirlist = fs::read_dir(format!("files/{}", pok.pokemonName));

                    if let Ok(dirlist) = dirlist {
                        for file2 in dirlist {
                            if let Ok(file2) = file2 {
                                let fileName=file2.file_name().into_string();
                                let filePath=file2.path().into_os_string().into_string();
                                if let Ok(fileName) = fileName && let Ok(filePath) =filePath {
                                    pok.imgPaths.push(format!("files/{}/{}/img.png",pok.pokemonName,fileName));
                                    pok.typePaths.push(fileName);
                                }
                            }
                        }
                        pokemonImgs.push(pok);
                    }
                }
            }
        }

        return pokemonImgs;
    } 

    
}