#![allow(dead_code)]

mod tag;
mod util;
mod runtime;
mod interface; pub use interface::*;
mod encoding; pub use encoding::*;

pub fn test() {
    Schema::with(vec![
        ("nat", natural()),
        ("pass", block(6)),
    ]).bind(9);
    Schema::with(vec![
        ("str", sequence()),
        ("int", integer()),
        ("bool", boolean()),
        ("rec", record(9)),
    ]).bind(10);

    let rec = Record::with(10, vec![
        ("str", *Sequence::with("hello!")),
        ("int", *Integer::with(-70)),
        ("bool", *Boolean::with(true)),
        ("rec", *Record::with(9, vec![
            ("nat", *Natural::with(8)),
            ("pass", *Block::with(6, vec![1, 2, 3, 4, 5, 6])),
        ]).unwrap()),
    ]).unwrap();

    let out = encode(*rec);
    print!("[{}]: ", out.len());
    for byte in &out {
        print!("{:02x} ", *byte);
    } println!("\n");

    match decode(&out, &mut 0) {
        Ok(ty) => match ty {
            Type::Record(rec) => {
                println!("{}", Sequence::from(rec.get("str")).unwrap().get());
                println!("{}", Integer::from(rec.get("int")).unwrap().get());
                println!("{}", Boolean::from(rec.get("bool")).unwrap().get());
                let sub = Record::from(rec.get("rec")).unwrap();
                println!("{}", Natural::from(sub.get("nat")).unwrap().get());
                println!("{:?}", Block::from(sub.get("pass")).unwrap().get());
            }
            _ => { println!("Other"); }
        },
        Err(_) => { println!("Failure"); }
    }
}

#[cfg(test)]
mod tests;
