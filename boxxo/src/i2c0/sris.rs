#[doc = "Register `SRIS` reader"]
pub type R = crate::R<SRIS_SPEC>;
#[doc = "Register `SRIS` writer"]
pub type W = crate::W<SRIS_SPEC>;
#[doc = "Field `I2C_SRIS_DATARIS` reader - Data Raw Interrupt Status"]
pub type I2C_SRIS_DATARIS_R = crate::BitReader;
#[doc = "Field `I2C_SRIS_DATARIS` writer - Data Raw Interrupt Status"]
pub type I2C_SRIS_DATARIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SRIS_STARTRIS` reader - Start Condition Raw Interrupt Status"]
pub type I2C_SRIS_STARTRIS_R = crate::BitReader;
#[doc = "Field `I2C_SRIS_STARTRIS` writer - Start Condition Raw Interrupt Status"]
pub type I2C_SRIS_STARTRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SRIS_STOPRIS` reader - Stop Condition Raw Interrupt Status"]
pub type I2C_SRIS_STOPRIS_R = crate::BitReader;
#[doc = "Field `I2C_SRIS_STOPRIS` writer - Stop Condition Raw Interrupt Status"]
pub type I2C_SRIS_STOPRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_dataris(&self) -> I2C_SRIS_DATARIS_R {
        I2C_SRIS_DATARIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_startris(&self) -> I2C_SRIS_STARTRIS_R {
        I2C_SRIS_STARTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    pub fn i2c_sris_stopris(&self) -> I2C_SRIS_STOPRIS_R {
        I2C_SRIS_STOPRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sris_dataris(&mut self) -> I2C_SRIS_DATARIS_W<SRIS_SPEC, 0> {
        I2C_SRIS_DATARIS_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sris_startris(&mut self) -> I2C_SRIS_STARTRIS_W<SRIS_SPEC, 1> {
        I2C_SRIS_STARTRIS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sris_stopris(&mut self) -> I2C_SRIS_STOPRIS_W<SRIS_SPEC, 2> {
        I2C_SRIS_STOPRIS_W::new(self)
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
#[doc = "I2C Slave Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRIS_SPEC;
impl crate::RegisterSpec for SRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sris::R`](R) reader structure"]
impl crate::Readable for SRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sris::W`](W) writer structure"]
impl crate::Writable for SRIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRIS to value 0"]
impl crate::Resettable for SRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
