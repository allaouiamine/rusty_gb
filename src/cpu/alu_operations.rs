use anyhow;

use super::{
    registers::{CpuRegisters, Flags},
    util::{
        add_relative, add_with_carry, check_carry_relative, check_half_carry_relative,
        sub_with_carry,
    },
    AluOutput, RegisterType, ValueEnum,
};

pub trait AluOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput>;
}

pub struct AddOperation {
    pub register_type: Option<RegisterType>,
}

impl AddOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self {
            register_type: Some(*register_type),
        }
    }
}

impl AluOperation for AddOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let (sum, z, n, h, c) = add_with_carry(
            cpu_registers.get_register(
                &self
                    .register_type
                    .ok_or(anyhow::anyhow!("No register provided for AddOpration"))?,
            ),
            fetched_data.try_into()?,
            0,
        );
        Ok(AluOutput {
            value: ValueEnum::Data8(sum),
            z: Some(z),
            n: Some(n),
            h: Some(h),
            c: Some(c),
        })
    }
}

pub struct AddWithCarryOperation {
    pub register_type: Option<RegisterType>,
}

impl AddWithCarryOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self {
            register_type: Some(*register_type),
        }
    }
}
impl AluOperation for AddWithCarryOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        if self.register_type.ok_or(anyhow::anyhow!(
            "Register A must be set for AddWithCarryOperation"
        ))? != RegisterType::A
        {
            anyhow::bail!("Only A register is allowed for ADC");
        }
        let carry = match cpu_registers.f.get_flag(Flags::C) {
            true => 1,
            false => 0,
        };
        let (sum, z, n, h, c) = add_with_carry(cpu_registers.a, fetched_data.try_into()?, carry);
        Ok(AluOutput {
            value: ValueEnum::Data8(sum),
            z: Some(z),
            n: Some(n),
            h: Some(h),
            c: Some(c),
        })
    }
}

pub struct AddRelativeOperation {
    pub register_type: Option<RegisterType>,
}

impl AddRelativeOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self {
            register_type: Some(*register_type),
        }
    }
}

impl AluOperation for AddRelativeOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        // only used in 0xE8 - ADD SP, r8
        if self.register_type.ok_or(anyhow::anyhow!(
            "Register SP must be set for AddRelativeOperation"
        ))? != RegisterType::SP
        {
            anyhow::bail!("Only SP register is allowed for ADD SP, r8");
        }
        let h = Some(check_half_carry_relative(
            cpu_registers.sp,
            fetched_data.try_into()?,
        ));
        let c = Some(check_carry_relative(
            cpu_registers.sp,
            fetched_data.try_into()?,
        ));
        let sum = add_relative(cpu_registers.sp, fetched_data.try_into()?);
        Ok(AluOutput {
            value: ValueEnum::Data16(sum),
            z: Some(false),
            n: Some(false),
            h,
            c,
        })
    }
}

pub struct AddOperation16 {
    pub register_type: Option<RegisterType>,
}
impl AddOperation16 {
    pub fn new(register_type: &RegisterType) -> Self {
        Self {
            register_type: Some(*register_type),
        }
    }
}
impl AluOperation for AddOperation16 {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let register_value = cpu_registers.get_register_16(
            &self
                .register_type
                .ok_or(anyhow::anyhow!("No register provided for AddOperation16"))?,
        );
        let fetched_data: u16 = fetched_data.try_into()?;
        let sum = register_value.wrapping_add(fetched_data);
        let h = Some((register_value & 0x0FFF) + (fetched_data & 0x0FFF) > 0x0FFF);
        let c = match register_value.checked_add(fetched_data) {
            Some(_) => Some(false),
            None => Some(true),
        };
        Ok(AluOutput {
            value: ValueEnum::Data16(sum),
            z: None,
            n: Some(false),
            h,
            c,
        })
    }
}

pub struct SbcOperation {
    pub register_type: Option<RegisterType>,
}
impl SbcOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self {
            register_type: Some(*register_type),
        }
    }
}
impl AluOperation for SbcOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        if self
            .register_type
            .ok_or(anyhow::anyhow!("No register provided for SbcOperation"))?
            != RegisterType::A
        {
            anyhow::bail!("Only A register is allowed for SBC");
        }
        let carry = match cpu_registers.f.get_flag(Flags::C) {
            true => 1,
            false => 0,
        };
        let (result, z, n, h, c) = sub_with_carry(cpu_registers.a, fetched_data.try_into()?, carry);
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(z),
            n: Some(n),
            h: Some(h),
            c: Some(c),
        })
    }
}

pub struct SubOperation {
    pub register_type: Option<RegisterType>,
}

impl SubOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self {
            register_type: Some(*register_type),
        }
    }
}
impl AluOperation for SubOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        if self
            .register_type
            .ok_or(anyhow::anyhow!("No register provided for SubOperation"))?
            != RegisterType::A
        {
            anyhow::bail!("Only A register is allowed for SBC");
        }
        let (result, z, n, h, c) = sub_with_carry(cpu_registers.a, fetched_data.try_into()?, 0);
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(z),
            n: Some(n),
            h: Some(h),
            c: Some(c),
        })
    }
}

pub struct IncOperation;

impl IncOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for IncOperation {
    fn execute(&self, fetched_data: ValueEnum, _: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let (result, z, _, h, _) = add_with_carry(fetched_data.try_into()?, 1, 0);
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(z),
            n: Some(false),
            h: Some(h),
            c: None,
        })
    }
}

pub struct IncOperation16;

impl IncOperation16 {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for IncOperation16 {
    fn execute(&self, fetched_data: ValueEnum, _: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let result: u16 = fetched_data.try_into()?;
        Ok(AluOutput {
            value: ValueEnum::Data16(result.wrapping_add(1)),
            z: None,
            n: None,
            h: None,
            c: None,
        })
    }
}

pub struct DecOperation;

impl DecOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for DecOperation {
    fn execute(&self, fetched_data: ValueEnum, _: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let (result, z, _, h, _) = sub_with_carry(fetched_data.try_into()?, 1, 0);
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(z),
            n: Some(true),
            h: Some(h),
            c: None,
        })
    }
}

pub struct DecOperation16;
impl DecOperation16 {
    pub fn new() -> Self {
        Self {}
    }
}
impl AluOperation for DecOperation16 {
    fn execute(&self, fetched_data: ValueEnum, _: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let result: u16 = fetched_data.try_into()?;
        Ok(AluOutput {
            value: ValueEnum::Data16(result.wrapping_sub(1)),
            z: None,
            n: None,
            h: None,
            c: None,
        })
    }
}

pub struct OrOperation;

impl OrOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for OrOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let data: u8 = fetched_data.try_into()?;
        let result = cpu_registers.a | data;
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(result == 0),
            n: Some(false),
            h: Some(false),
            c: Some(false),
        })
    }
}

pub struct XorOperation;
impl XorOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for XorOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let data: u8 = fetched_data.try_into()?;
        let result = cpu_registers.a ^ data;
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(result == 0),
            n: Some(false),
            h: Some(false),
            c: Some(false),
        })
    }
}
pub struct CpOperation;

impl CpOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for CpOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let data: u8 = fetched_data.try_into()?;
        Ok(AluOutput {
            value: ValueEnum::None,
            z: Some(cpu_registers.a == data),
            n: Some(true),
            h: Some((cpu_registers.a & 0x0F) < (data & 0x0F)),
            c: Some(cpu_registers.a < data),
        })
    }
}

pub struct AndOperation;

impl AndOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for AndOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let data: u8 = fetched_data.try_into()?;
        let result = cpu_registers.a & data;
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(result == 0),
            n: Some(false),
            h: Some(true),
            c: Some(false),
        })
    }
}

pub struct DaaOperation;

impl DaaOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for DaaOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        /*
         * DAA - Decimal Adjust A
         * This instruction adjusts the register A so that the correct representation of Binary
         * Coded Decimal (BCD) is obtained.
         * Which means that each nibble is treated as a decimal digit. for example
         *  45 is 0100 0101 in BCD
         */
        let a = cpu_registers.a;

        // If the last operation was addition, the N flag is 0
        // If the last operation was subtraction, the N flag is 1
        let n_flag = cpu_registers.f.get_flag(Flags::N);
        let c_flag = cpu_registers.f.get_flag(Flags::C);
        let h_flag = cpu_registers.f.get_flag(Flags::H);

        let mut new_c_flag = false;

        let mut adjust = 0;
        // if the half carry flag is set, it means that the lower nibble of the result is greater
        // than 9, which cannot be represented in a single digit in BCD
        // If we add 6 to any hex digit higher than 9 (0xA to 0xF) to the lower nibble, it will overflow to the upper
        // nibble and the lower nibble stays in the range of 0-9.
        // !n_flag means the last operation was an addition
        if h_flag || (!n_flag && ((a & 0x0F) > 9)) {
            adjust = 0x06;
        }

        // Same as above, we will add 6 to the upper nibble if the carry flag is set or the upper
        // nibble is greater than 9 if the last operation was an addition
        if c_flag || (!n_flag && (a > 0x99)) {
            adjust |= 0x60;
            new_c_flag = true;
        }

        let result = if n_flag {
            // substraction
            a.wrapping_sub(adjust)
        } else {
            // addition
            a.wrapping_add(adjust)
        };
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(result == 0),
            n: None,
            h: Some(false),
            c: Some(new_c_flag),
        })
    }
}

pub struct CplOperation;

impl CplOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for CplOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let result = cpu_registers.a ^ 0xFF; // Invert all the bits
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: None,
            n: Some(true),
            h: Some(true),
            c: None,
        })
    }
}

pub struct ScfOperation;

impl ScfOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for ScfOperation {
    fn execute(&self, _: ValueEnum, _: &CpuRegisters) -> anyhow::Result<AluOutput> {
        Ok(AluOutput {
            value: ValueEnum::None,
            z: None,
            n: Some(false),
            h: Some(false),
            c: Some(true),
        })
    }
}

pub struct CcfOperation;

impl CcfOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for CcfOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let c = !cpu_registers.f.get_flag(Flags::C);
        Ok(AluOutput {
            value: ValueEnum::None,
            z: None,
            n: Some(false),
            h: Some(false),
            c: Some(c),
        })
    }
}

pub struct RlcaOperation;

impl RlcaOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for RlcaOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let carry = cpu_registers.a >> 7;
        let result = (cpu_registers.a << 1) | carry;
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(false),
            n: Some(false),
            h: Some(false),
            c: Some(carry == 1),
        })
    }
}

pub struct RlaOperation;

impl RlaOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for RlaOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let carry_flag = cpu_registers.f.get_flag_as_u8(Flags::C);
        let carry = cpu_registers.a >> 7;
        let result = (cpu_registers.a << 1) | carry_flag;
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(false),
            n: Some(false),
            h: Some(false),
            c: Some(carry == 1),
        })
    }
}

pub struct RrcaOperation;

impl RrcaOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for RrcaOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let carry = cpu_registers.a & 0x01;
        let result = (cpu_registers.a >> 1) | (carry << 7);
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(false),
            n: Some(false),
            h: Some(false),
            c: Some(carry == 1),
        })
    }
}
pub struct RraOperation;

impl RraOperation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for RraOperation {
    fn execute(&self, _: ValueEnum, cpu_registers: &CpuRegisters) -> anyhow::Result<AluOutput> {
        let carry = cpu_registers.f.get_flag_as_u8(Flags::C);
        let result = (cpu_registers.a >> 1) | (carry << 7);
        let c = cpu_registers.a & 0x01 == 1;
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(false),
            n: Some(false),
            h: Some(false),
            c: Some(c),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_set_half_carry() {
        let inc = IncOperation::new();
        let cpu_registers = CpuRegisters::new();
        let fetched_data = ValueEnum::Data8(0x0F);
        let alu_output = inc.execute(fetched_data, &cpu_registers).unwrap();
        assert_eq!(alu_output.h, Some(true));
        assert_eq!(alu_output.value, ValueEnum::Data8(0x10));
    }
    #[test]
    fn test_inc_reset_half_carry() {
        let inc = IncOperation::new();
        let cpu_registers = CpuRegisters::new();
        let fetched_data = ValueEnum::Data8(0x10);
        let alu_output = inc.execute(fetched_data, &cpu_registers).unwrap();
        assert_eq!(alu_output.h, Some(false));
        assert_eq!(alu_output.value, ValueEnum::Data8(0x11));
    }
}
