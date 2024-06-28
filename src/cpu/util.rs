use super::types::Bits;

pub fn is_bit_set(register: u8, bits: Bits) -> bool {
    let mask: u8 = 1 << (bits as u8);
    register & mask == mask
}

pub fn combine(reg_left: u8, reg_right: u8) -> u16 {
    let hi = (reg_left as u16) << 8;
    let lo = reg_right as u16;
    hi | lo
}

pub fn add_relative(n: u16, r: i8) -> u16 {
    n.wrapping_add(r as u16)
}

pub fn check_carry_relative(n: u16, r: i8) -> bool {
    add_relative(n & 0xFF, r) > 0xFF
}

pub fn check_half_carry_relative(n: u16, r: i8) -> bool {
    // TODO: Make sure this is correct
    add_relative(n & 0x0F, r & 0x0F) > 0x0F
}

/**
 * Adds two 8-bit values and a carry (0 or 1) and then returns the result along with the flags
 * Z: Set if result is zero
 * N: always false for all 8bit ADD and ADC
 * H: Set if carry from bit 3
 * C: Set if carry from bit 7
 */
pub fn add_with_carry(left_value: u8, right_value: u8, carry: u8) -> (u8, bool, bool, bool, bool) {
    let sum = left_value.wrapping_add(right_value).wrapping_add(carry);
    let z = sum == 0;
    let h = (left_value & 0x0F) + (right_value & 0x0F) + carry > 0x0F;
    let c = (left_value as u16) + (right_value as u16) + (carry as u16) > 0xFF;
    (sum, z, false, h, c)
}

/**
 * Subtracts two 8-bit values and a carry (0 or 1) and then returns the result along with the flags
 * Z: Set if result is zero
 * N: always true for all 8bit SUB and SBC
 * H: Set if borrow from bit 4
 * C: Set if borrow
 */
pub fn sub_with_carry(left: u8, right: u8, carry: u8) -> (u8, bool, bool, bool, bool) {
    let diff = left - right - carry;
    let z = diff == 0;
    let c = left < right + carry;
    let h = (left & 0x0F) < (right & 0x0F) + carry;
    (diff, z, true, h, c)
}


#[cfg(test)]
mod tests {
    use crate::cpu::types::ValueEnum;

    use super::*;

    #[test]
    fn test_add_relative_positive() {
        let n = 0x1234;
        let r: i8 = 0x12;
        assert_eq!(add_relative(n, r), 0x1246);
    }

    #[test]
    fn test_add_relative_negative() {
        let n = 0x1234;
        let r: i8 = (0x88 as u8) as i8;
        assert_eq!(add_relative(n, r), 0x11BC);
    }

    #[test]
    fn test_add_with_carry() {
        let (result, z, n, h, c) = add_with_carry(0x16, 0x35, 0);
        assert_eq!(result, 0x4b);
        assert_eq!(z, false);
        assert_eq!(h, false);
        assert_eq!(c, false);
        assert_eq!(n, false);
    }

    #[test]
    fn test_add_with_carry_half_carry() {
        let (result, z, n, h, c) = add_with_carry(0x16, 0x35, 1);
        assert_eq!(result, 0x4c);
        assert_eq!(z, false);
        assert_eq!(h, true);
        assert_eq!(c, false);
        assert_eq!(n, false);
    }

    #[test]
    fn test_add_with_carry_carry() {
        let (result, z, n, h, c) = add_with_carry(0x36, 0xca, 1);
        assert_eq!(result, 0x1);
        assert_eq!(z, false);
        assert_eq!(h, true);
        assert_eq!(c, true);
        assert_eq!(n, false);
    }

    #[test]
    fn test_add_with_carry_carry_zero() {
        let (result, z, n, h, c) = add_with_carry(0x36, 0xca, 0);
        assert_eq!(result, 0);
        assert_eq!(z, true);
        assert_eq!(h, true);
        assert_eq!(c, true);
        assert_eq!(n, false);
    }
    #[test]
    fn test_sub_with_carry() {
        let (result, z, n, h, c) = sub_with_carry(0x16, 0x15, 0);
        assert_eq!(result, 0x1);
        assert_eq!(z, false);
        assert_eq!(h, true);
        assert_eq!(c, false);
        assert_eq!(n, true);
    }

    #[test]
    fn test_sub_with_carry_half_carry() {
        let (result, z, n, h, c) = sub_with_carry(0x16, 0x15, 1);
        assert_eq!(result, 0x0);
        assert_eq!(z, true);
        assert_eq!(h, false);
        assert_eq!(c, false);
        assert_eq!(n, true);
    }

    #[test]
    fn test_sub_with_carry_carry() {
        let (result, z, n, h, c) = sub_with_carry(0x16, 0x16, 0);
        assert_eq!(result, 0x0);
        assert_eq!(z, true);
        assert_eq!(h, false);
        assert_eq!(c, false);
        assert_eq!(n, true);
    }

    #[test]
    fn test_sub_with_carry_carry_zero() {
        let (result, z, n, h, c) = sub_with_carry(0x16, 0x16, 1);
        assert_eq!(result, 0xFF);
        assert_eq!(z, false);
        assert_eq!(h, true);
        assert_eq!(c, true);
        assert_eq!(n, true);
    }
}
