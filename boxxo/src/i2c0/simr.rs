#[doc = "Register `SIMR` reader"]
pub type R = crate::R<SIMR_SPEC>;
#[doc = "Register `SIMR` writer"]
pub type W = crate::W<SIMR_SPEC>;
#[doc = "Field `I2C_SIMR_DATAIM` reader - Data Interrupt Mask"]
pub type I2C_SIMR_DATAIM_R = crate::BitReader;
#[doc = "Field `I2C_SIMR_DATAIM` writer - Data Interrupt Mask"]
pub type I2C_SIMR_DATAIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SIMR_STARTIM` reader - Start Condition Interrupt Mask"]
pub type I2C_SIMR_STARTIM_R = crate::BitReader;
#[doc = "Field `I2C_SIMR_STARTIM` writer - Start Condition Interrupt Mask"]
pub type I2C_SIMR_STARTIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SIMR_STOPIM` reader - Stop Condition Interrupt Mask"]
pub type I2C_SIMR_STOPIM_R = crate::BitReader;
#[doc = "Field `I2C_SIMR_STOPIM` writer - Stop Condition Interrupt Mask"]
pub type I2C_SIMR_STOPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_dataim(&self) -> I2C_SIMR_DATAIM_R {
        I2C_SIMR_DATAIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_startim(&self) -> I2C_SIMR_STARTIM_R {
        I2C_SIMR_STARTIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_simr_stopim(&self) -> I2C_SIMR_STOPIM_R {
        I2C_SIMR_STOPIM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_simr_dataim(&mut self) -> I2C_SIMR_DATAIM_W<SIMR_SPEC, 0> {
        I2C_SIMR_DATAIM_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_simr_startim(&mut self) -> I2C_SIMR_STARTIM_W<SIMR_SPEC, 1> {
        I2C_SIMR_STARTIM_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_simr_stopim(&mut self) -> I2C_SIMR_STOPIM_W<SIMR_SPEC, 2> {
        I2C_SIMR_STOPIM_W::new(self)
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
#[doc = "I2C Slave Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`simr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`simr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SIMR_SPEC;
impl crate::RegisterSpec for SIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`simr::R`](R) reader structure"]
impl crate::Readable for SIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`simr::W`](W) writer structure"]
impl crate::Writable for SIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SIMR to value 0"]
impl crate::Resettable for SIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
