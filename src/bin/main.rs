extern crate mech;
extern crate core;

use mech::database::{Database, Transaction, Change, AddChange, NewTableChange};
use mech::table::{Value, Table};
use mech::indexes::Hasher;

fn main() {


  

  let tag: u64 = Hasher::hash_str("students");  
  let teachers: u64 = Hasher::hash_str("teachers");
  let student1: u64 = Hasher::hash_str("Mark");
  let student2: u64 = Hasher::hash_str("Sabra");
  let first: u64 = Hasher::hash_str("first name");
  let last: u64 = Hasher::hash_str("last name");
  let test1: u64 = Hasher::hash_str("test1");
  let test2: u64 = Hasher::hash_str("test2");

  let mut table = Table::new(tag, 16, 16);

  table.set(student1, first, Value::from_str("Mark"));
  table.set(student1, last, Value::from_str("Laughlin"));
  table.set(student1, test1, Value::from_u64(83));
  table.set(student1, test2, Value::from_u64(76));

  table.set(student2, first, Value::from_str("Sabra"));
  table.set(student2, last, Value::from_str("Kindar"));
  table.set(student2, test1, Value::from_u64(99));
  table.set(student2, test2, Value::from_u64(95));
  table.set(student2, first, Value::from_u64(100));

  //println!("{:?}", table);

  //println!("{:?}", table.get_rows(vec![student1]));
  //println!("{:?}", table.get_cols(vec![first, test1, last, 3]));

  table.index(student1, test1);

  table.clear(student1, last);
  table.clear(student2, test2);

  //println!("{:?}", table);


  //println!("{:?}", foo);
  //let mut my_value = Value::from_u64(100);
  //foo = &mut my_value;
  

  let mut db = Database::new(1000, 1000, 1000);
  let c1 = AddChange::new(tag, student1, first, Value::from_str("Mark"));
  let c2 = AddChange::new(tag, student1, last, Value::from_str("Laughlin"));
  let c3 = AddChange::new(tag, student1, test1, Value::from_u64(83));
  let c4 = AddChange::new(tag, student1, test1, Value::from_u64(76));
  let c5 = AddChange::new(teachers, student2, first, Value::from_str("Sabra"));
  let t1= NewTableChange::new(String::from("students"), vec![], vec![], 10, 10);
  let t2= NewTableChange::new(String::from("teachers"), vec![], vec![], 10, 10);
  let txn = Transaction::from_changeset(vec![
    Change::Add(c1), 
    Change::NewTable(t1), 
    Change::Add(c3), 
    Change::Add(c2)]);
  db.register_transaction(txn);
  println!("{:?}", db);
  let txn = Transaction::from_changeset(vec![
    Change::Add(c4),
    Change::Add(c5),
    Change::NewTable(t2)]);
  db.register_transaction(txn);
  println!("{:?}", db);
  for i in 0 .. 1_000_000 {
    let c = AddChange::new(tag, student1, test1, Value::from_u64(i as u64));
    let t = Transaction::from_change(Change::Add(c));
    db.register_transaction(t);
  }

  println!("{:?}", db);

}