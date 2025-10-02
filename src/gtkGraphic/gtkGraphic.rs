
use gtk::{gio::{prelude::ApplicationExtManual, traits::ApplicationExt}, traits::*};

use crate::classes::pokemonImg::PokemonImg;

pub struct gtkGraphic {}

impl gtkGraphic {
    pub fn runGraphicGTK() {
        let pokemonImgs=PokemonImg::get();
        let app = gtk::Application::new(Some("com.example.rustApp"), Default::default());
        app.connect_activate(move |app| {
            let window = gtk::ApplicationWindow::new(app);

            window.set_default_size(500, 500);
            window.set_title("holaaa");

            //Graph Box
            let mut vBox = gtk::Box::new(gtk::Orientation::Horizontal, 50);

            //Take Imgs to GTK
            let mut imgsToGTK: Vec<gtk::Image> = vec![];
            for pokImg in &pokemonImgs {
                let vboxPok = gtk::Box::new(gtk::Orientation::Vertical, 50);
                let labelTitle = gtk::Label::new(Some(pokImg.pokemonName.as_str()));
                vboxPok.add(&labelTitle);

                for img in pokImg.imgPaths.iter().enumerate() {
                    let labelType = gtk::Label::new(Some(pokImg.typePaths[img.0].as_str()));

                    vboxPok.add(&labelType);
                    let imgTo = gtk::Image::from_file(img.1);
                    vboxPok.add(&imgTo);
                    imgsToGTK.push(imgTo);

                    vBox.add(&vboxPok);
                }
            }

            let button = gtk::Button::new();
            button.connect_clicked(|x| {
                println!("hola");
            });

            let scrolledWindow = gtk::ScrolledWindow::default();

            scrolledWindow.add(&vBox);
            window.add(&scrolledWindow);

            window.show_all();
        });

        app.run();
    }
}
