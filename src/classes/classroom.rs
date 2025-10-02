use std::sync::{LazyLock, Mutex};

use tokio::io::{stdin, AsyncBufReadExt, BufReader};

use crate::classes::person::Person;

pub struct Classroom {
    pub persons: Vec<Person>,
}

impl Classroom {
    fn new() -> Self {
        Self { persons: vec![] }
    }

    pub fn deletePerson(&mut self, name: String) {
        let mut indexToDelete = 0;
        let mut found = false;

        for person in self.persons.iter().enumerate() {
            if person.1.name == name {
                indexToDelete = person.0;
                found = true;
            }
        }

        if (found) {
            &self.persons.remove(indexToDelete);
        }
    }

    /**
     * Find Persons in ClassRoom
     */
    pub fn findPersons(&mut self, name: String) -> Vec<&mut Person> {
        return self.persons.iter_mut().filter(|x| x.name == name).collect();
    }

    pub fn findPerson(&mut self, name: String) -> Option<&mut Person> {
        let person = self.persons.iter_mut().find(|x| x.name == name);
        return person;
    }

    pub fn addPerson(&mut self, person: Person) {
        self.persons.push(person);
    }

    pub fn print(&self) {
        for person in &self.persons {
            println!(
                "Person |Name: {} |Surname: {} |Age: {}",
                person.name, person.surname, person.age
            );
        }
    }

    pub fn printSelected(vec: &Vec<&mut Person>) {
        for person in vec.iter() {
            println!(
                "Person |Name: {} |Surname: {} |Age: {}",
                person.name, person.surname, person.age
            );
        }
    }

    pub async fn classroomMenu() {
        while true {
            print!(
                "Select Option To Proceed\n1.Add New Person to ClassRoom\n2. Delete a Person in ClassRoom\n3.Find A Person in ClassRoom\n4.Find persons in ClassRoom\n5. List all Persons in Classroom\n6. Exit\n"
            );

            let mut input = String::new();
            let mut bufRead = BufReader::new(stdin());
            bufRead.read_line(&mut input).await.unwrap();

            println!("You Selected Option {}", input);

            let num_used = u32::from_str_radix(&input.trim(), 10).unwrap();

            let mut newInput = String::new();

            let mut classroomGlobal = CLASSROOM.lock().unwrap();

            println!("Welcome to ClassRoom Menu");
            match num_used {
                1 => {
                    println!("Name");
                    newInput = String::new();
                    bufRead.read_line(&mut newInput).await.unwrap();
                    let name = newInput.trim().to_string();

                    println!("Surname");
                    newInput = String::new();
                    bufRead.read_line(&mut newInput).await.unwrap();
                    let surname = newInput.trim().to_string();

                    println!("Age");
                    newInput = String::new();
                    bufRead.read_line(&mut newInput).await.unwrap();
                    let age = u32::from_str_radix(&newInput.trim(), 10).unwrap();

                    classroomGlobal.addPerson(Person::new(name, surname, age));
                }

                2 => {
                    println!("Name To Delete: ");
                    newInput = String::new();
                    bufRead.read_line(&mut newInput).await.unwrap();

                    classroomGlobal.deletePerson(newInput.trim().to_string());
                }

                3 => {
                    println!("Name To Search: ");
                    newInput = String::new();
                    bufRead.read_line(&mut newInput).await.unwrap();

                    let personFound = classroomGlobal.findPerson(newInput.trim().to_string());

                    if let Some(p) = personFound {
                        println!("{} {} is {} Years Old", p.name, p.surname, p.age);
                    } else {
                        println!("No Persons Found");
                    }
                }
                4 => {
                    println!("Names To Search: ");
                    newInput = String::new();
                    bufRead.read_line(&mut newInput).await.unwrap();

                    let mut personsFound = classroomGlobal.findPersons(newInput.trim().to_string());

                    Classroom::printSelected(&personsFound);
                }
                5 => {
                    classroomGlobal.print();
                }
                6 => {
                    break;
                }
                _ => {}
            }
        }
    }
}

pub static CLASSROOM: LazyLock<Mutex<Classroom>> = LazyLock::new(|| Mutex::new(Classroom::new()));
