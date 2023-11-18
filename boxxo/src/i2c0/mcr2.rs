#[doc = "Register `MCR2` reader"]
pub type R = crate::R<MCR2_SPEC>;
#[doc = "Register `MCR2` writer"]
pub type W = crate::W<MCR2_SPEC>;
#[doc = "Field `I2C_MCR2_GFPW` reader - I2C Glitch Filter Pulse Width"]
pub type I2C_MCR2_GFPW_R = crate::FieldReader<I2C_MCR2_GFPW_A>;
#[doc = "I2C Glitch Filter Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C_MCR2_GFPW_A {
    #[doc = "0: Bypass"]
    I2C_MCR2_GFPW_BYPASS = 0,
    #[doc = "1: 1 clock"]
    I2C_MCR2_GFPW_1 = 1,
    #[doc = "2: 2 clocks"]
    I2C_MCR2_GFPW_2 = 2,
    #[doc = "3: 3 clocks"]
    I2C_MCR2_GFPW_3 = 3,
    #[doc = "4: 4 clocks"]
    I2C_MCR2_GFPW_4 = 4,
    #[doc = "5: 8 clocks"]
    I2C_MCR2_GFPW_8 = 5,
    #[doc = "6: 16 clocks"]
    I2C_MCR2_GFPW_16 = 6,
    #[doc = "7: 32 clocks"]
    I2C_MCR2_GFPW_32 = 7,
}
impl From<I2C_MCR2_GFPW_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C_MCR2_GFPW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C_MCR2_GFPW_A {
    type Ux = u8;
}
impl I2C_MCR2_GFPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2C_MCR2_GFPW_A {
        match self.bits {
            0 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_BYPASS,
            1 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_1,
            2 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_2,
            3 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_3,
            4 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_4,
            5 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_8,
            6 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_16,
            7 => I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_32,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_bypass(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_BYPASS
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_1(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_1
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_2(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_2
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_3(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_3
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_4(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_4
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_8(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_8
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_16(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_16
    }
    #[doc = "32 clocks"]
    #[inline(always)]
    pub fn is_i2c_mcr2_gfpw_32(&self) -> bool {
        *self == I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_32
    }
}
#[doc = "Field `I2C_MCR2_GFPW` writer - I2C Glitch Filter Pulse Width"]
pub type I2C_MCR2_GFPW_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 3, O, I2C_MCR2_GFPW_A>;
impl<'a, REG, const O: u8> I2C_MCR2_GFPW_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_BYPASS)
    }
    #[doc = "1 clock"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_1(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_1)
    }
    #[doc = "2 clocks"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_2(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_2)
    }
    #[doc = "3 clocks"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_3(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_3)
    }
    #[doc = "4 clocks"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_4(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_4)
    }
    #[doc = "8 clocks"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_8(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_8)
    }
    #[doc = "16 clocks"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_16(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_16)
    }
    #[doc = "32 clocks"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw_32(self) -> &'a mut crate::W<REG> {
        self.variant(I2C_MCR2_GFPW_A::I2C_MCR2_GFPW_32)
    }
}
impl R {
    #[doc = "Bits 4:6 - I2C Glitch Filter Pulse Width"]
    #[inline(always)]
    pub fn i2c_mcr2_gfpw(&self) -> I2C_MCR2_GFPW_R {
        I2C_MCR2_GFPW_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - I2C Glitch Filter Pulse Width"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mcr2_gfpw(&mut self) -> I2C_MCR2_GFPW_W<MCR2_SPEC, 4> {
        I2C_MCR2_GFPW_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C Master Configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCR2_SPEC;
impl crate::RegisterSpec for MCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr2::R`](R) reader structure"]
impl crate::Readable for MCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mcr2::W`](W) writer structure"]
impl crate::Writable for MCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR2 to value 0"]
impl crate::Resettable for MCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
