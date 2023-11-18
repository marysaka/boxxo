#[doc = "Register `MICR` writer"]
pub type W = crate::W<MICR_SPEC>;
#[doc = "Field `I2C_MICR_IC` writer - Master Interrupt Clear"]
pub type I2C_MICR_IC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MICR_CLKIC` writer - Clock Timeout Interrupt Clear"]
pub type I2C_MICR_CLKIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Master Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_micr_ic(&mut self) -> I2C_MICR_IC_W<MICR_SPEC, 0> {
        I2C_MICR_IC_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_micr_clkic(&mut self) -> I2C_MICR_CLKIC_W<MICR_SPEC, 1> {
        I2C_MICR_CLKIC_W::new(self)
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
#[doc = "I2C Master Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`micr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MICR_SPEC;
impl crate::RegisterSpec for MICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`micr::W`](W) writer structure"]
impl crate::Writable for MICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
