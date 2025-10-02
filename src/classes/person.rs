pub struct Person {
    pub name:String,
    pub surname:String,
    pub age:u32
}

impl Person {
    pub fn new (name:String,surname:String,age:u32) -> Self {
        return Self { name,surname,age } ;
    }

    pub fn saludar (&self) {
        println!("Hola, me llamo {} {}, y tengo {} años",self.name,self.surname,self.age);
    }
}