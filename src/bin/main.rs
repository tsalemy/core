extern crate mech;
extern crate core;

use std::time::SystemTime;
use mech::database::{Database, Transaction, Change, NewTableChange};
use mech::table::{Value, Table};
use mech::indexes::Hasher;
use mech::operations::{Function, Plan};
use mech::runtime::{Runtime, Block, Constraint, Register};

fn main() {

  

  let mut db = Database::new(1000, 1000, 1000);
  let students: u64 = Hasher::hash_str("students");  
  let student1: u64 = Hasher::hash_str("Mark");
  let student2: u64 = Hasher::hash_str("Sabra");
  let test1: u64 = Hasher::hash_str("test1");
  let test2: u64 = Hasher::hash_str("test2");
  let result: u64 = Hasher::hash_str("result");

//  let c2 = AddChange::new(students, student1, test2, Value::from_u64(76));
//  let c3 = AddChange::new(students, student2, test1, Value::from_u64(99));
//  let c4 = AddChange::new(students, student2, test2, Value::from_u64(88));
  let t1= NewTableChange::new(String::from("students"), vec![], vec![], 10, 10);
  let txn = Transaction::from_changeset(vec![
    Change::Add{ix: 0, table: students, entity: student1, attribute: test1, value: Value::from_u64(83)}, 
    Change::NewTable(t1), 
    //Change::Add(c2),
    //Change::Add(c3),
    //Change::Add(c4)
  ]);


  let mut block = Block::new();
  block.add_constraint(Constraint::Scan {table: students, attribute: test1, register: 1});
  block.add_constraint(Constraint::Scan {table: students, attribute: test2, register: 2});
  block.add_constraint(Constraint::Function {operation: Function::Add, parameters: vec![1, 2], output: vec![1]});
  block.add_constraint(Constraint::Insert {table: students, attribute: result, register: 1});
  let plan = vec![
    Constraint::Function {operation: Function::Add, parameters: vec![1, 2], output: vec![1]},
    Constraint::Insert {table: students, attribute: result, register: 1}
  ];
  block.plan = plan;
  let mut block2 = Block::new();
  
  let begin = SystemTime::now();


  println!("{:?}", txn);

  db.register_transaction(txn);  
  let foo = db.runtime.register_block(block.clone(), &db.store);
  let foo2 = db.runtime.register_block(block2.clone(), &db.store);

  let txn2 = Transaction::from_changeset(foo);
  db.register_transaction(txn2);
  
  println!("{:?}", db);
  println!("{:?}", db.runtime);




  let end = SystemTime::now();
  let delta = end.duration_since(begin);

  
  println!("{:?}", delta);
  loop{}
}