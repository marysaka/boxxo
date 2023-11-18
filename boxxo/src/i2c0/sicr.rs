#[doc = "Register `SICR` writer"]
pub type W = crate::W<SICR_SPEC>;
#[doc = "Field `I2C_SICR_DATAIC` writer - Data Interrupt Clear"]
pub type I2C_SICR_DATAIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SICR_STARTIC` writer - Start Condition Interrupt Clear"]
pub type I2C_SICR_STARTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SICR_STOPIC` writer - Stop Condition Interrupt Clear"]
pub type I2C_SICR_STOPIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Data Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sicr_dataic(&mut self) -> I2C_SICR_DATAIC_W<SICR_SPEC, 0> {
        I2C_SICR_DATAIC_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sicr_startic(&mut self) -> I2C_SICR_STARTIC_W<SICR_SPEC, 1> {
        I2C_SICR_STARTIC_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sicr_stopic(&mut self) -> I2C_SICR_STOPIC_W<SICR_SPEC, 2> {
        I2C_SICR_STOPIC_W::new(self)
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
#[doc = "I2C Slave Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SICR_SPEC;
impl crate::RegisterSpec for SICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sicr::W`](W) writer structure"]
impl crate::Writable for SICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SICR to value 0"]
impl crate::Resettable for SICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
