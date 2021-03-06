// run-pass

#![feature(saturating_neg)]
#![feature(const_checked_int_methods)]
#![feature(const_euclidean_int_methods)]
#![feature(const_overflowing_int_methods)]
#![feature(const_saturating_int_methods)]
#![feature(const_wrapping_int_methods)]

use std::i8;

macro_rules! suite {
    ($(
        $fn:ident -> $ty:ty { $( $label:ident : $expr:expr, $result:expr; )* }
    )*) => { $(
        fn $fn() {
            $(
                const $label: $ty = $expr;
                assert_eq!($label, $result);
            )*
        }
    )* }
}

suite!(
    checked -> Option<i8> {
        // `const_checked_int_methods`
        C1: 5i8.checked_add(2), Some(7);
        C2: 127i8.checked_add(2), None;

        C3: 5i8.checked_sub(2), Some(3);
        C4: (-127i8).checked_sub(2), None;

        C5: 1i8.checked_mul(3), Some(3);
        C6: 5i8.checked_mul(122), None;
        C7: (-127i8).checked_mul(-99), None;

        C8: (i8::min_value() + 1).checked_div(-1), Some(127);
        C9: i8::min_value().checked_div(-1), None;
        C10: 1i8.checked_div(0), None;

        C11: 5i8.checked_rem(2), Some(1);
        C12: 5i8.checked_rem(0), None;
        C13: i8::MIN.checked_rem(-1), None;

        C14: 5i8.checked_neg(), Some(-5);
        C15: i8::MIN.checked_neg(), None;

        C16: 0x1i8.checked_shl(4), Some(0x10);
        C17: 0x1i8.checked_shl(129), None;

        C18: 0x10i8.checked_shr(4), Some(0x1);
        C19: 0x10i8.checked_shr(128), None;


        C20: (-5i8).checked_abs(), Some(5);
        C21: i8::MIN.checked_abs(), None;

        // `const_euclidean_int_methods`
        C22: (i8::min_value() + 1).checked_div_euclid(-1), Some(127);
        C23: i8::min_value().checked_div_euclid(-1), None;
        C24: (1i8).checked_div_euclid(0), None;

        C25: 5i8.checked_rem_euclid(2), Some(1);
        C26: 5i8.checked_rem_euclid(0), None;
        C27: i8::MIN.checked_rem_euclid(-1), None;
    }

    saturating_and_wrapping -> i8 {
        // `const_saturating_int_methods`
        C28: 100i8.saturating_add(1), 101;
        C29: i8::max_value().saturating_add(100), i8::max_value();
        C30: i8::min_value().saturating_add(-1), i8::min_value();

        C31: 100i8.saturating_sub(127), -27;
        C32: i8::min_value().saturating_sub(100), i8::min_value();
        C33: i8::max_value().saturating_sub(-1), i8::max_value();

        C34: 10i8.saturating_mul(12), 120;
        C35: i8::MAX.saturating_mul(10), i8::MAX;
        C36: i8::MIN.saturating_mul(10), i8::MIN;

        C37: 100i8.saturating_neg(), -100;
        C38: (-100i8).saturating_neg(), 100;
        C39: i8::min_value().saturating_neg(), i8::max_value();
        C40: i8::max_value().saturating_neg(), i8::min_value() + 1;

        C57: 100i8.saturating_abs(), 100;
        C58: (-100i8).saturating_abs(), 100;
        C59: i8::min_value().saturating_abs(), i8::max_value();
        C60: (i8::min_value() + 1).saturating_abs(), i8::max_value();

        // `const_wrapping_int_methods`
        C41: 100i8.wrapping_div(10), 10;
        C42: (-128i8).wrapping_div(-1), -128;

        C43: 100i8.wrapping_rem(10), 0;
        C44: (-128i8).wrapping_rem(-1), 0;

        // `const_euclidean_int_methods`
        C45: 100i8.wrapping_div_euclid(10), 10;
        C46: (-128i8).wrapping_div_euclid(-1), -128;

        C47: 100i8.wrapping_rem_euclid(10), 0;
        C48: (-128i8).wrapping_rem_euclid(-1), 0;
    }

    overflowing -> (i8, bool) {
        // `const_overflowing_int_methods`
        C49: 5i8.overflowing_div(2), (2, false);
        C50: i8::MIN.overflowing_div(-1), (i8::MIN, true);

        C51: 5i8.overflowing_rem(2), (1, false);
        C52: i8::MIN.overflowing_rem(-1), (0, true);

        // `const_euclidean_int_methods`
        C53: 5i8.overflowing_div_euclid(2), (2, false);
        C54: i8::MIN.overflowing_div_euclid(-1), (i8::MIN, true);

        C55: 5i8.overflowing_rem_euclid(2), (1, false);
        C56: i8::MIN.overflowing_rem_euclid(-1), (0, true);

    }
);

fn main() {
   checked();
   saturating_and_wrapping();
   overflowing();
}
