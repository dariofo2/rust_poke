use std::any::Any;
pub struct BinaryConvers {}

pub enum types {
    text(Vec<u8>),
    u16(Vec<u8>),
    u32(Vec<u8>),
    u64(Vec<u8>),
    u128(Vec<u8>),
    i16(Vec<u8>),
    i32(Vec<u8>),
    i64(Vec<u8>),
    i128(Vec<u8>)
}


impl BinaryConvers {
    pub fn encode (input:Box<dyn Any>) -> Option<Vec<u8>> {
        if let Some(x)=input.downcast_ref::<u16>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<u32>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<u64>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<u128>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<i16>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<i32>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<i64>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<i128>() {
            return Some(x.to_le_bytes().to_vec());
        }
        if let Some(x)=input.downcast_ref::<String>() {
            return Some(x.as_bytes().to_vec());
        }
        return None;
        
    }

    pub fn decode (input:types) -> Box<dyn Any> {
        let value: Box<dyn Any>=match input {
            types::text(x) => Box::new(String::from_utf8(x)),
            types::i16(x) => Box::new(i16::from_le_bytes(x.try_into().unwrap())),
            types::i32(x) => Box::new(i32::from_le_bytes(x.try_into().unwrap())),
            types::i64(x) => Box::new(i64::from_le_bytes(x.try_into().unwrap())),
            types::i128(x) => Box::new(i128::from_le_bytes(x.try_into().unwrap())),
            types::u16(x) => Box::new(u16::from_le_bytes(x.try_into().unwrap())),
            types::u32(x) => Box::new(u32::from_le_bytes(x.try_into().unwrap())),
            types::u64(x) => Box::new(u64::from_le_bytes(x.try_into().unwrap())),
            types::u128(x) => Box::new(u128::from_le_bytes(x.try_into().unwrap())),
        };

        return value;
    }

    pub fn test () {
        let encoded1=BinaryConvers::encode(Box::new(574647574 as u128)).unwrap();
        let encoded2=BinaryConvers::encode(Box::new(574647574 as u128)).unwrap();
        let encoded3=BinaryConvers::encode(Box::new(574647574 as u128)).unwrap();

        let array=[encoded1,encoded2,encoded3].concat();

        print!("0x");
        for i in &array {
            print!("{:X} ",i);
        }

        println!("");

        let decoded1=BinaryConvers::decode(types::u128(array[0..16].to_vec()));
        let decoded2=BinaryConvers::decode(types::u128(array[16..32].to_vec()));
        let decoded3=BinaryConvers::decode(types::u128(array[32..48].to_vec()));

    //let decoded=BinaryConvers::decode(types::u128(encoded));
    println!("{} {} {}",decoded1.downcast::<u128>().unwrap(),decoded2.downcast::<u128>().unwrap(),decoded3.downcast::<u128>().unwrap());
    }
}
