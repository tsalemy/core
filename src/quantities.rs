// |T|DDDDDDD|RRRRRRR|SMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM|
// T: Type Extension Flag (0 For Typed Math numbers)
// D: Domain [0, 127]
// R: Range [-64, 63]
// S: mantissa Sign bit
// M: Mantissa [-2^48, 2^48 - 1]
// Credit: Josh Cole, who implemented this for Eve v0.4
// Adapted and Extended for Mech by Corey Montella

extern crate num;
use self::num::Float;

const EXTENSION_MASK:u64 = 1 << 63;
const MANTISSA_MASK:u64 = (((1 as u64) << 49) as u64 - 1); // 49 bits at the end
const META_MASK:u64 = ((1 << 15) as u64 - 1) << 49; // 15 1s at the front
const OVERFLOW_MASK:u64 = ((1 << 16) as u64 - 1) << 48; // 15 1s at the front
const RANGE_MASK:u64 = ((1 << 7) as u64 - 1) << 49;
const SHIFTED_RANGE_DOMAIN_MASK:u64 = ((1 << 7) as u64 - 1);
const SHIFTED_FILL:u64 = ((((1 as u64) << 57) as u64 - 1) << 7);
const SIGN_MASK:u64 = 1 << 48;

pub type Quantity = u64;

pub trait ToQuantity {
    fn to_quantity(&self) -> u64;
}

pub trait FromQuantity<T> {
    fn get_value(self) -> T;
}

impl ToQuantity for u32 {
    #[inline(always)]
    fn to_quantity(&self) -> u64 {
        let result:u64 = (*self).into();
        result | (1 << 63)
    }
}

impl ToQuantity for i32 {
    #[inline(always)]
    fn to_quantity(&self) -> u64 {
        let me = *self;
        if me.is_negative() {
            me as u64 & MANTISSA_MASK | EXTENSION_MASK
        } else {
            me as u64 | EXTENSION_MASK
        }
    }
}

impl ToQuantity for u64 {
    #[inline(always)]
    fn to_quantity(&self) -> u64 {
        let me = *self;
        if me & META_MASK != 0 {
            let (mantissa, range) = overflow_handler(me);
            (mantissa as u64) & MANTISSA_MASK | shifted_range(range) |  EXTENSION_MASK
        } else {
            me & MANTISSA_MASK | EXTENSION_MASK
        }
    }
}

impl ToQuantity for i64 {
    #[inline(always)]
    fn to_quantity(&self) -> u64 {
        let me = *self;
        if me.is_negative() {
            if (me as u64) & META_MASK != META_MASK {
                let (mantissa, range) = overflow_handler(me.abs() as u64);
                !(mantissa - 1) & MANTISSA_MASK | shifted_range(range) |  EXTENSION_MASK
            } else {
                (me as u64) & MANTISSA_MASK | EXTENSION_MASK
            }
        } else if (me as u64) & OVERFLOW_MASK != 0 {
            let (mantissa, range) = overflow_handler(me as u64);
            (mantissa as u64) & MANTISSA_MASK | shifted_range(range) |  EXTENSION_MASK
        } else {
            (me as u64) & MANTISSA_MASK | EXTENSION_MASK
        }
    }
}

impl ToQuantity for f64 {
    #[inline(always)]
    fn to_quantity(&self) -> u64 {
        let me = *self;
        let (mantissa, exponent, sign) = Float::integer_decode(me);
        let exp_log = 2f64.powi(exponent as i32).log10();
        let real_exponent = exp_log.floor() as i64 + 1;
        let real_mantissa = (sign as f64 * ((mantissa as f64) * 10f64.powf(exp_log.fract()))) as i64;
        let mut result = real_mantissa.to_quantity();
        let cur = result.range();
        result.set_range(cur + real_exponent);
        result
    }
}


#[inline(always)]
pub fn overflow_handler(me:u64) -> (u64, u64) {
    let hi = 64 - me.leading_zeros() - 48;
    let r = (2u64.pow(hi) as f64).log10().ceil() as u32;
    let result = me / 10u64.pow(r) as u64;
    (result, r as u64)
}

pub fn decrease_range(mantissa:i64, range_delta:u64) -> (i64, u64) {
    let remaining_space = mantissa.leading_zeros();
    let thing:u64 = (1 as u64) << remaining_space;
    let remaining_10 = (thing as f64).log10().floor() as u64;
    if range_delta <= remaining_10 {
        (mantissa * 10u64.pow(range_delta as u32) as i64, range_delta)
    } else {
        (mantissa * 10u64.pow(remaining_10 as u32) as i64, range_delta)
    }
}

pub fn increase_range(mantissa:i64, range_delta:u64) -> (i64, bool) {
    let range = 10u64.pow(range_delta as u32) as i64;
    (mantissa / range, mantissa % range != 0)
}

#[inline(always)]
pub fn shifted_range(range:u64) -> u64 {
    range << 49
}

pub fn make_quantity(mantissa:u64, range:i64, domain:u64) -> Quantity {
    let value = mantissa.to_quantity();
    let cur_range = (value.range() + range) as u64;
    value & !RANGE_MASK | ((cur_range << 49) & RANGE_MASK) | (domain << 56)
}

pub trait QuantityMath {
    fn is_number(self) -> bool;
    fn is_other(self) -> bool;
    fn domain(self) -> u64;
    fn range(self) -> i64;
    fn set_range(&mut self, range:i64);
    fn mantissa(self) -> i64;
    fn is_negative(self) -> bool;
    fn negate(self) -> Quantity;
    fn add(self, Quantity) -> Quantity;
    fn sub(self, Quantity) -> Quantity;
    fn multiply(self, Quantity) -> Quantity;
    fn divide(self, Quantity) -> Quantity;
    fn to_string(self) -> String;
    fn to_float(self) -> f64;
}

impl QuantityMath for Quantity {
    #[inline(always)]
    fn is_number(self) -> bool {
        self & EXTENSION_MASK == EXTENSION_MASK
    }

    #[inline(always)]
    fn is_other(self) -> bool {
        self & EXTENSION_MASK == 0
    }

    #[inline(always)]
    fn domain(self) -> u64 {
        (self >> 56) & SHIFTED_RANGE_DOMAIN_MASK
    }

    #[inline(always)]
    fn range(self) -> i64 {
        let range = (self >> 49) & SHIFTED_RANGE_DOMAIN_MASK;
        if range & (1 << 6) == 0 {
            range as i64
        } else {
            (range | SHIFTED_FILL) as i64
        }
    }

    fn set_range(&mut self, range:i64) {
        let range_fill = ((range << 49) as u64) & RANGE_MASK;
        *self &= !RANGE_MASK;
        *self |= range_fill;
    }

    #[inline(always)]
    fn mantissa(self) -> i64 {
        if self & SIGN_MASK == SIGN_MASK {
            let a = self & MANTISSA_MASK;
            (a as i64) | (META_MASK as i64)
        } else {
            (self & MANTISSA_MASK) as i64
        }
    }

    fn negate(self) -> Quantity {
        let value = ((self.mantissa() * -1) as u64 & MANTISSA_MASK) as u64;
        self & META_MASK | value
    }

    #[inline(always)]
    fn is_negative(self) -> bool {
        (self & SIGN_MASK) == SIGN_MASK
    }

    fn to_string(self) -> String {
        format!("{}r{}", self.mantissa(), self.range())
    }

    fn to_float(self) -> f64 {
        (self.mantissa() as f64) * 10f64.powi(self.range() as i32)
    }

    #[inline(always)]
    fn add(self, other:Quantity) -> Quantity {
        let my_range = self.range();
        let other_range = other.range();
        if my_range == other_range {
            let added = self.mantissa() + other.mantissa();
            let mut added_quantity = added.to_quantity();
            added_quantity.set_range(added_quantity.range() + self.range());
            added_quantity
        } else {
            let my_mant = self.mantissa();
            let other_mant = other.mantissa();
            let (a_range, b_range, a_mant, b_mant) = if my_range > other_range {
                (my_range, other_range, my_mant, other_mant)
            } else {
                (other_range, my_range, other_mant, my_mant)
            };
            let range_delta = (a_range - b_range) as u64;
            let (neue, actual_delta) = decrease_range(a_mant, range_delta);
            if actual_delta == range_delta {
                let added = neue + b_mant;
                let mut added_quantity = added.to_quantity();
                added_quantity.set_range(b_range + added_quantity.range());
                added_quantity
            } else {
                let (b_neue, _) = increase_range(b_mant, actual_delta);
                let mut added = (neue + b_neue).to_quantity();
                added.set_range(a_range - actual_delta as i64);
                added
            }
        }
    }

    fn sub(self, other:Quantity) -> Quantity {
        self.add(other.negate())
    }

    fn multiply(self, other:Quantity) -> Quantity {
        let result = match self.mantissa().checked_mul(other.mantissa()) {
           Some(result) => { result },
           None => { panic!("QuantityMultiply overflow") }
        };
        let mut quantity = result.to_quantity();
        quantity.set_range(self.range() + other.range());
        quantity
    }

    fn divide(self, other:Quantity) -> Quantity {
        (self.to_float() / other.to_float()).to_quantity()
    }
}

// extern crate test;
// use self::test::{Bencher};
// #[bench]
// fn bench_numerics_add(b:&mut Bencher) {
//     let y:i32 = -1;
//     // let xs = (0..10000).map(|x| x.to_tagged()).collect::<Vec<_>>();
//     let y_tagged = y.to_tagged();
//     b.iter(|| {
//         for x in (0..10000).map(|x| x.to_tagged()) {
//             test::black_box(x.add(y_tagged));
//         }
//     });
// }

// #[bench]
// fn bench_numerics_normal_add(b:&mut Bencher) {
//     let y:i32 = -1;
//     b.iter(|| {
//         for x in 0..10000 {
//             test::black_box(x + y);
//         }
//     });
// }
