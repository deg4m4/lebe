fn main() {

    let v1 = 4194480; 
    
    let le1 = i32tole(v1);
    let le2 = i32tolealt(v1);

    println!("{:x?}\n{:x?}", le1, le2);

    let be1 = i32tobe(v1);
    let be2 = i32tobealt(v1);

    println!("{:x?}\n{:x?}", be1, be2);


    let v1 = 4194480;

    let le3 = i64tole(v1);
    let le4 = i64tolealt(v1);

    println!("{:?}\n{:?}", le3, le4);

    let be3 = i64tobe(v1);
    let be4 = i64tobealt(v1);

    println!("{:?}\n{:?}", be3, be4); 
 

}

fn i32tole(value: i32) -> [u8; 4] {
     
     let b = value.to_le_bytes();
    
     b

}

fn i32tobe(value: i32) -> [u8; 4] {

    let b = value.to_be_bytes();

    b

}

fn i32tolealt(value: i32) -> [u8; 4] {

    let mut b: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

    b[0] = u8::try_from(value & 0xff).unwrap();
    b[1] = u8::try_from(value >> 8 & 0xff).unwrap();
    b[2] = u8::try_from(value >> 16 & 0xff).unwrap();
    b[3] = u8::try_from(value >> 24 & 0xff).unwrap();

    b

}

fn i32tobealt(value: i32) -> [u8; 4] {

    let mut b: [u8; 4] = [0x00, 0x00, 0x00, 0x00];

    b[0] = u8::try_from(value >> 24 & 0xff).unwrap();
    b[1] = u8::try_from(value >> 16 & 0xff).unwrap();
    b[2] = u8::try_from(value >> 8 & 0xff).unwrap();
    b[3] = u8::try_from(value & 0xff).unwrap();

    b

}

fn i64tole(value: i64) -> [u8; 8] {
     
     let b = value.to_le_bytes();
    
     b

}

fn i64tobe(value: i64) -> [u8; 8] {

    let b = value.to_be_bytes();

    b

}

fn i64tolealt(value: i64) -> [u8; 8] {

    let mut b: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

    b[0] = u8::try_from(value & 0xff).unwrap();
    b[1] = u8::try_from(value >> 8 & 0xff).unwrap();
    b[2] = u8::try_from(value >> 16 & 0xff).unwrap();
    b[3] = u8::try_from(value >> 24 & 0xff).unwrap();
    b[4] = u8::try_from(value >> 32 & 0xff).unwrap();
    b[5] = u8::try_from(value >> 40 & 0xff).unwrap();
    b[6] = u8::try_from(value >> 48 & 0xff).unwrap();
    b[7] = u8::try_from(value >> 56 & 0xff).unwrap();

    b

}

fn i64tobealt(value: i64) -> [u8; 8] {

    let mut b: [u8; 8] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
    

    b[0] = u8::try_from(value >> 56 & 0xff).unwrap();
    b[1] = u8::try_from(value >> 48 & 0xff).unwrap();
    b[2] = u8::try_from(value >> 40 & 0xff).unwrap();
    b[3] = u8::try_from(value >> 32 & 0xff).unwrap();
    b[4] = u8::try_from(value >> 24 & 0xff).unwrap();
    b[5] = u8::try_from(value >> 16 & 0xff).unwrap();
    b[6] = u8::try_from(value >> 8 & 0xff).unwrap();
    b[7] = u8::try_from(value & 0xff).unwrap();

    b

}


