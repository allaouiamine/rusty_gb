use anyhow;


use super::{
    registers::{CpuRegisters, Flags}, util::{
        add_relative, add_with_carry, check_carry_relative, check_half_carry_relative,
        sub_with_carry,
    }, AluOutput, RegisterType, ValueEnum
};

pub trait AluOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput>;
}

pub struct NoOpearation;

impl NoOpearation {
    pub fn new() -> Self {
        Self {}
    }
}

impl AluOperation for NoOpearation {
    fn execute(
        &self,
        _fetched_data: ValueEnum,
        _cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        Ok(AluOutput {
            value: ValueEnum::None,
            z: None,
            n: None,
            h: None,
            c: None,
            additional_cpu_cycles: 0,
        })
    }
}

pub struct AddOperation{
    pub register_type: Option<RegisterType>,
}

impl AddOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self { register_type: Some(*register_type) }
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
                &self.register_type.ok_or(anyhow::anyhow!("No register provided for AddOpration"))?,
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
            additional_cpu_cycles: 0,
        })
    }
}

pub struct AddWithCarryOperation{
    pub register_type: Option<RegisterType>,
}

impl AddWithCarryOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self { register_type: Some(*register_type) }
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
            additional_cpu_cycles: 0,
        })
    }
}

pub struct AddRelativeOperation{
    pub register_type: Option<RegisterType>,
}

impl AddRelativeOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self { register_type: Some(*register_type) }
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
            additional_cpu_cycles: 1,
        })
    }
}

pub struct AndOperation16{
    pub register_type: Option<RegisterType>,
}
impl AndOperation16 {
    pub fn new(register_type: &RegisterType) -> Self {
        Self { register_type: Some(*register_type) }
    }
}
impl AluOperation for AndOperation16 {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        let register_value = cpu_registers.get_register_16(
            &self.register_type.ok_or(anyhow::anyhow!("No register provided for AndOperation16"))?,
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
            additional_cpu_cycles: 1,
        })
    }
}

pub struct SbcOperation{
    pub register_type: Option<RegisterType>,
}
impl SbcOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self { register_type: Some(*register_type) }
    }
}
impl AluOperation for SbcOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        if self.register_type.ok_or(anyhow::anyhow!("No register provided for SbcOperation"))?
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
            additional_cpu_cycles: 0,
        })
    }
}

pub struct SubOperation{
    pub register_type: Option<RegisterType>,
}

impl SubOperation {
    pub fn new(register_type: &RegisterType) -> Self {
        Self { register_type: Some(*register_type) }
    }
}
impl AluOperation for SubOperation {
    fn execute(
        &self,
        fetched_data: ValueEnum,
        cpu_registers: &CpuRegisters,
    ) -> anyhow::Result<AluOutput> {
        if self.register_type.ok_or(anyhow::anyhow!("No register provided for SubOperation"))? != RegisterType::A {
            anyhow::bail!("Only A register is allowed for SBC");
        }
        let (result, z, n, h, c) = sub_with_carry(cpu_registers.a, fetched_data.try_into()?, 0);
        Ok(AluOutput {
            value: ValueEnum::Data8(result),
            z: Some(z),
            n: Some(n),
            h: Some(h),
            c: Some(c),
            additional_cpu_cycles: 0,
        })
    }
}
