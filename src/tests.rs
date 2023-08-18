use crate::*;

#[test]
fn boolean_initialize()
{
    let value = Boolean::new();
    assert_eq!(false, value.get(), "Boolean::new() produces value of false.");
}

#[test]
fn boolean_with()
{
    assert_eq!(false, Boolean::with(false).get(), "Boolean::with(false) produces value of false.");
    assert_eq!(true, Boolean::with(true).get(), "Boolean::with(true) produces value of true.");
}

#[test]
fn boolean_set_get()
{
    let mut value = Boolean::new();
    value.set(true);
    assert_eq!(true, value.get(), "Boolean.set(true) produces value of true.");
    value.set(false);
    assert_eq!(false, value.get(), "Boolean.set(false) produces value of false.");
}

#[test]
fn natural_initialize()
{
    let value = Natural::new();
    assert_eq!(0, value.get(), "Natural::new() produces value of 0.");
}

#[test]
fn natural_set_get()
{
    let mut value = Natural::new();
    value.set(12);
    assert_eq!(12, value.get(), "Natural.set(12) produces value of 12.");
    value.set(38695);
    assert_eq!(38695, value.get(), "Natural.set(38695) produces value of 38695.");
}

#[test]
fn integer_initialize()
{
    let value = Integer::new();
    assert_eq!(0, value.get(), "Integer::new() produces value of 0.");
}

#[test]
fn integer_set_get()
{
    let mut value = Integer::new();
    value.set(-273);
    assert_eq!(-273, value.get(), "Integer.set(-273) produces value of -273.");
    value.set(100);
    assert_eq!(100, value.get(), "Integer.set(100) produces value of 100.");
}

#[test]
fn block_initialize()
{
    let value = Block::new(8);
    assert_eq!(8, value.size(), "Block::new(8) has length of 8.");
    assert_eq!(vec![0;8], value.get(), "Block::new(8) has value of 0.");
}

#[test]
fn block_set_get()
{
    let mut value = Block::new(4);
    value.set(vec![1, 2, 3, 4]);
    assert_eq!(vec![1, 2, 3, 4], value.get(), "Block::set([1,2,3,4]) has value of [1,2,3,4].");
}

#[test]
fn sequence_initialize()
{
    let value = Sequence::new();
    assert_eq!(0, value.size(), "Sequence::new() has length 0.");
}

#[test]
fn sequence_set_get_raw()
{
    let mut value = Sequence::new();
    value.set_raw(vec![0, 1, 2, 3, 2, 1]);
    assert_eq!(6, value.size(), "Sequence.set_raw([0,1,2,3,2,1]) has length 6.");
    assert_eq!(vec![0,1,2,3,2,1], value.get_raw(), "Sequence.set_raw([0,1,2,3,2,1]) produces value [0,1,2,3,2,1].");
}

#[test]
fn sequence_set_get_str()
{
    let mut value = Sequence::new();
    value.set("hello");
    assert_eq!(5, value.size(), "Sequence.set(hello) has length 5.");
    assert_eq!(String::from("hello"), value.get(), "Sequence.set(hello) produces value hello.");
}

#[test]
fn array_initialize()
{

}

#[test]
fn array_from()
{
    
}

#[test]
fn array_with()
{
    
}

#[test]
fn array_length()
{
    
}

#[test]
fn array_at()
{
    
}

#[test]
fn array_set()
{
    
}

#[test]
fn list_initialize()
{
    let list = List::new(natural());
    assert_eq!(0, list.capacity(), "New list has capacity of 0.");
    assert_eq!(0, list.length(), "New list has length of 0.");
}

#[test]
fn list_from()
{
    let list = List::new(natural());
    let mut list_ref = List::from(*list).unwrap();
    list_ref.append(*Natural::with(10));

    assert_eq!(1, list.length(), "List has length 1 after append to reference.");
    assert_eq!(10, Natural::from(list.at(0)).unwrap().get());
}

#[test]
fn list_with()
{
    let list = List::with(natural(), vec![
        *Natural::with(5),
        *Natural::with(10),
        *Natural::with(15),
        *Natural::with(20),
    ]);
    assert_eq!(4, list.length());
    assert_eq!(15, Natural::from(list.at(2)).unwrap().get());
}

#[test]
fn list_clear()
{
    let mut list = List::with(natural(), vec![
        *Natural::with(33),
        *Natural::with(66),
        *Natural::with(99),
    ]);
    assert_eq!(3, list.length(), "List initialized with 3 elements has length of 3.");
    list.clear();
    assert_eq!(0, list.length(), "Cleared list has length of 0.");
}

#[test]
fn list_insert()
{
    let mut list = List::with(natural(), vec![
        *Natural::with(21),
        *Natural::with(23),
        *Natural::with(24),
    ]);
    assert_eq!(3, list.length(), "List initialized with 3 elements has length of 3.");
    list.insert(1, *Natural::with(22));
    assert_eq!(21, list.length(), "First element has value of 21.");
    assert_eq!(22, list.length(), "Second element has value of 22.");
    assert_eq!(23, list.length(), "Third element has value of 23.");
}

#[test]
fn list_prepend()
{
    let mut list = List::with(natural(), vec![
        *Natural::with(1000),
        *Natural::with(2000),
        *Natural::with(3000),
    ]);
    assert_eq!(3, list.length(), "List initialized with 3 elements has length of 3.");
    list.prepend(*Natural::with(0));
    assert_eq!(4, list.length(), "List of 3 elements has length of 4 after prepend.");
    assert_eq!(0, Natural::from(list.at(0)).unwrap().get(), "First element in list has value 0.");
    assert_eq!(1000, Natural::from(list.at(1)).unwrap().get(), "Second element in list has value 1000.");
}

#[test]
fn list_append()
{
    let mut list = List::with(natural(), vec![
        *Natural::with(1000),
        *Natural::with(2000),
        *Natural::with(3000),
    ]);
    assert_eq!(3, list.length(), "List initialized with 3 elements has length of 3.");
    list.append(*Natural::with(4000));
    assert_eq!(4, list.length(), "List of 3 elements has length of 4 after prepend.");
    assert_eq!(3000, Natural::from(list.at(2)).unwrap().get(), "Third element in list has value 3000.");
    assert_eq!(4000, Natural::from(list.at(3)).unwrap().get(), "Last element in list has value 4000.");}

#[test]
fn list_set()
{
    let mut list = List::with(natural(), vec![
        *Natural::with(1),
        *Natural::with(1),
        *Natural::with(3),
    ]);
    assert_eq!(3, list.length(), "List initialized with 3 elements has length of 3.");
    list.set(1, *Natural::with(2));
    assert_eq!(2, Natural::from(list.at(1)).unwrap().get(), "Second element in list has value 2.");
}

#[test]
fn list_remove()
{
    let mut list = List::with(natural(), vec![
        *Natural::with(0),
        *Natural::with(1),
        *Natural::with(2),
        *Natural::with(3),
        *Natural::with(4),
        *Natural::with(5),
    ]);
    assert_eq!(6, list.length(), "List initialized with 6 elements has length of 6.");
    list.remove(5);
    list.remove(0);
    list.remove(2);
    assert_eq!(3, list.length(), "List with 3/6 elements removed has length of 3.");
    assert_eq!(1, Natural::from(list.at(0)).unwrap().get(), "First element in list is 1.");
    assert_eq!(2, Natural::from(list.at(1)).unwrap().get(), "Second element in list is 2.");
    assert_eq!(4, Natural::from(list.at(2)).unwrap().get(), "Last element in list is 4.");
}

#[test]
fn list_reserve()
{
    let capacity :usize = 10;
    let mut list = List::new(natural());
    assert_eq!(0, list.capacity(), "List has initial capacity of 0.");
    list.reserve(capacity);
    assert_eq!(capacity, list.capacity(), "List has updated capacity.");
}

#[test]
fn record_initialize()
{

}

#[test]
fn record_from()
{
    
}

#[test]
fn record_with()
{
    
}

#[test]
fn record_with_values()
{
    
}

#[test]
fn record_at()
{
    
}

#[test]
fn record_set()
{
    
}

#[test]
fn schema_initialize()
{
    
}
