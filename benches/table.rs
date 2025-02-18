#![feature(test)]

extern crate test;
extern crate mech_core;

use test::Bencher;
use mech_core::{Table, Value, Index};

#[bench]
fn make_table(b: &mut Bencher) {
    b.iter(|| {
        let mut table = Table::new(0, 16, 16);
    });
}

#[bench]
fn make_table_100_x_100(b: &mut Bencher) {
    b.iter(|| {
        let mut table = Table::new(0, 100, 100);
    });
}

#[bench]
fn set_cell(b: &mut Bencher) {
    let mut table = Table::new(0, 1, 1);
    b.iter(|| {
        table.set_cell(&Index::Index(1), &Index::Index(1), Value::from_u64(100));
    });
}

#[bench]
fn set_clear_cell(b: &mut Bencher) {
    let mut table = Table::new(0, 1, 1);
    b.iter(|| {
        table.set_cell(&Index::Index(1), &Index::Index(1), Value::from_u64(100));
        table.clear_cell(&Index::Index(1), &Index::Index(1));
    });
}