#![allow(unreachable_code)]

use crate::float::Float;
use crate::int::MinInt;

#[derive(Clone, Copy)]
enum Result {
    Less,
    Equal,
    Greater,
    Unordered,
}

impl Result {
    fn to_le_abi(self) -> i32 {
        match self {
            Result::Less => -1,
            Result::Equal => 0,
            Result::Greater => 1,
            Result::Unordered => 1,
        }
    }

    fn to_ge_abi(self) -> i32 {
        match self {
            Result::Less => -1,
            Result::Equal => 0,
            Result::Greater => 1,
            Result::Unordered => -1,
        }
    }
}

fn cmp<F: Float>(a: F, b: F) -> Result {
    let one = F::Int::ONE;
    let zero = F::Int::ZERO;
    let szero = F::SignedInt::ZERO;

    let sign_bit = F::SIGN_MASK as F::Int;
    let abs_mask = sign_bit - one;
    let exponent_mask = F::EXPONENT_MASK;
    let inf_rep = exponent_mask;

    let a_rep = a.repr();
    let b_rep = b.repr();
    let a_abs = a_rep & abs_mask;
    let b_abs = b_rep & abs_mask;

    // If either a or b is NaN, they are unordered.
    if a_abs > inf_rep || b_abs > inf_rep {
        return Result::Unordered;
    }

    // If a and b are both zeros, they are equal.
    if a_abs | b_abs == zero {
        return Result::Equal;
    }

    let a_srep = a.signed_repr();
    let b_srep = b.signed_repr();

    // If at least one of a and b is positive, we get the same result comparing
    // a and b as signed integers as we would with a fp_ting-point compare.
    if a_srep & b_srep >= szero {
        if a_srep < b_srep {
            Result::Less
        } else if a_srep == b_srep {
            Result::Equal
        } else {
            Result::Greater
        }
    // Otherwise, both are negative, so we need to flip the sense of the
    // comparison to get the correct result.  (This assumes a twos- or ones-
    // complement integer representation; if integers are represented in a
    // sign-magnitude representation, then this flip is incorrect).
    } else if a_srep > b_srep {
        Result::Less
    } else if a_srep == b_srep {
        Result::Equal
    } else {
        Result::Greater
    }
}

fn unord<F: Float>(a: F, b: F) -> bool {
    let one = F::Int::ONE;

    let sign_bit = F::SIGN_MASK as F::Int;
    let abs_mask = sign_bit - one;
    let exponent_mask = F::EXPONENT_MASK;
    let inf_rep = exponent_mask;

    let a_rep = a.repr();
    let b_rep = b.repr();
    let a_abs = a_rep & abs_mask;
    let b_abs = b_rep & abs_mask;

    a_abs > inf_rep || b_abs > inf_rep
}

intrinsics! {
    #[avr_skip]
    pub extern "C" fn __lesf2(a: f32, b: f32) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gesf2(a: f32, b: f32) -> i32 {
        cmp(a, b).to_ge_abi()
    }

    #[avr_skip]
    #[arm_aeabi_alias = __aeabi_fcmpun]
    pub extern "C" fn __unordsf2(a: f32, b: f32) -> i32 {
        unord(a, b) as i32
    }

    #[avr_skip]
    pub extern "C" fn __eqsf2(a: f32, b: f32) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __ltsf2(a: f32, b: f32) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __nesf2(a: f32, b: f32) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gtsf2(a: f32, b: f32) -> i32 {
        cmp(a, b).to_ge_abi()
    }

    #[avr_skip]
    pub extern "C" fn __ledf2(a: f64, b: f64) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gedf2(a: f64, b: f64) -> i32 {
        cmp(a, b).to_ge_abi()
    }

    #[avr_skip]
    #[arm_aeabi_alias = __aeabi_dcmpun]
    pub extern "C" fn __unorddf2(a: f64, b: f64) -> i32 {
        unord(a, b) as i32
    }

    #[avr_skip]
    pub extern "C" fn __eqdf2(a: f64, b: f64) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __ltdf2(a: f64, b: f64) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __nedf2(a: f64, b: f64) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gtdf2(a: f64, b: f64) -> i32 {
        cmp(a, b).to_ge_abi()
    }
}

#[cfg(not(any(
    feature = "no-f16-f128",
    target_arch = "powerpc",
    target_arch = "powerpc64"
)))]
intrinsics! {
    #[avr_skip]
    pub extern "C" fn __letf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __getf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_ge_abi()
    }

    #[avr_skip]
    pub extern "C" fn __unordtf2(a: f128, b: f128) -> i32 {
        unord(a, b) as i32
    }

    #[avr_skip]
    pub extern "C" fn __eqtf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __lttf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __netf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gttf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_ge_abi()
    }
}

#[cfg(all(
    not(feature = "no-f16-f128"),
    any(target_arch = "powerpc", target_arch = "powerpc64")
))]
intrinsics! {
    #[avr_skip]
    pub extern "C" fn __lekf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gekf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_ge_abi()
    }

    #[avr_skip]
    pub extern "C" fn __unordkf2(a: f128, b: f128) -> i32 {
        unord(a, b) as i32
    }

    #[avr_skip]
    pub extern "C" fn __eqkf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __ltkf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __nekf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_le_abi()
    }

    #[avr_skip]
    pub extern "C" fn __gtkf2(a: f128, b: f128) -> i32 {
        cmp(a, b).to_ge_abi()
    }
}

#[cfg(target_arch = "arm")]
intrinsics! {
    pub extern "aapcs" fn __aeabi_fcmple(a: f32, b: f32) -> i32 {
        (__lesf2(a, b) <= 0) as i32
    }

    pub extern "aapcs" fn __aeabi_fcmpge(a: f32, b: f32) -> i32 {
        (__gesf2(a, b) >= 0) as i32
    }

    pub extern "aapcs" fn __aeabi_fcmpeq(a: f32, b: f32) -> i32 {
        (__eqsf2(a, b) == 0) as i32
    }

    pub extern "aapcs" fn __aeabi_fcmplt(a: f32, b: f32) -> i32 {
        (__ltsf2(a, b) < 0) as i32
    }

    pub extern "aapcs" fn __aeabi_fcmpgt(a: f32, b: f32) -> i32 {
        (__gtsf2(a, b) > 0) as i32
    }

    pub extern "aapcs" fn __aeabi_dcmple(a: f64, b: f64) -> i32 {
        (__ledf2(a, b) <= 0) as i32
    }

    pub extern "aapcs" fn __aeabi_dcmpge(a: f64, b: f64) -> i32 {
        (__gedf2(a, b) >= 0) as i32
    }

    pub extern "aapcs" fn __aeabi_dcmpeq(a: f64, b: f64) -> i32 {
        (__eqdf2(a, b) == 0) as i32
    }

    pub extern "aapcs" fn __aeabi_dcmplt(a: f64, b: f64) -> i32 {
        (__ltdf2(a, b) < 0) as i32
    }

    pub extern "aapcs" fn __aeabi_dcmpgt(a: f64, b: f64) -> i32 {
        (__gtdf2(a, b) > 0) as i32
    }

    // On hard-float targets LLVM will use native instructions
    // for all VFP intrinsics below

    pub extern "C" fn __gesf2vfp(a: f32, b: f32) -> i32 {
        (a >= b) as i32
    }

    pub extern "C" fn __gedf2vfp(a: f64, b: f64) -> i32 {
        (a >= b) as i32
    }

    pub extern "C" fn __gtsf2vfp(a: f32, b: f32) -> i32 {
        (a > b) as i32
    }

    pub extern "C" fn __gtdf2vfp(a: f64, b: f64) -> i32 {
        (a > b) as i32
    }

    pub extern "C" fn __ltsf2vfp(a: f32, b: f32) -> i32 {
        (a < b) as i32
    }

    pub extern "C" fn __ltdf2vfp(a: f64, b: f64) -> i32 {
        (a < b) as i32
    }

    pub extern "C" fn __lesf2vfp(a: f32, b: f32) -> i32 {
        (a <= b) as i32
    }

    pub extern "C" fn __ledf2vfp(a: f64, b: f64) -> i32 {
        (a <= b) as i32
    }

    pub extern "C" fn __nesf2vfp(a: f32, b: f32) -> i32 {
        (a != b) as i32
    }

    pub extern "C" fn __nedf2vfp(a: f64, b: f64) -> i32 {
        (a != b) as i32
    }

    pub extern "C" fn __eqsf2vfp(a: f32, b: f32) -> i32 {
        (a == b) as i32
    }

    pub extern "C" fn __eqdf2vfp(a: f64, b: f64) -> i32 {
        (a == b) as i32
    }
}
