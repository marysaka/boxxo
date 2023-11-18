#[doc = "Register `MRIS` reader"]
pub type R = crate::R<MRIS_SPEC>;
#[doc = "Register `MRIS` writer"]
pub type W = crate::W<MRIS_SPEC>;
#[doc = "Field `I2C_MRIS_RIS` reader - Master Raw Interrupt Status"]
pub type I2C_MRIS_RIS_R = crate::BitReader;
#[doc = "Field `I2C_MRIS_RIS` writer - Master Raw Interrupt Status"]
pub type I2C_MRIS_RIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MRIS_CLKRIS` reader - Clock Timeout Raw Interrupt Status"]
pub type I2C_MRIS_CLKRIS_R = crate::BitReader;
#[doc = "Field `I2C_MRIS_CLKRIS` writer - Clock Timeout Raw Interrupt Status"]
pub type I2C_MRIS_CLKRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_ris(&self) -> I2C_MRIS_RIS_R {
        I2C_MRIS_RIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mris_clkris(&self) -> I2C_MRIS_CLKRIS_R {
        I2C_MRIS_CLKRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mris_ris(&mut self) -> I2C_MRIS_RIS_W<MRIS_SPEC, 0> {
        I2C_MRIS_RIS_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mris_clkris(&mut self) -> I2C_MRIS_CLKRIS_W<MRIS_SPEC, 1> {
        I2C_MRIS_CLKRIS_W::new(self)
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
#[doc = "I2C Master Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MRIS_SPEC;
impl crate::RegisterSpec for MRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mris::R`](R) reader structure"]
impl crate::Readable for MRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mris::W`](W) writer structure"]
impl crate::Writable for MRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRIS to value 0"]
impl crate::Resettable for MRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
