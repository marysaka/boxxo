#[doc = "Register `SMIS` reader"]
pub type R = crate::R<SMIS_SPEC>;
#[doc = "Register `SMIS` writer"]
pub type W = crate::W<SMIS_SPEC>;
#[doc = "Field `I2C_SMIS_DATAMIS` reader - Data Masked Interrupt Status"]
pub type I2C_SMIS_DATAMIS_R = crate::BitReader;
#[doc = "Field `I2C_SMIS_DATAMIS` writer - Data Masked Interrupt Status"]
pub type I2C_SMIS_DATAMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SMIS_STARTMIS` reader - Start Condition Masked Interrupt Status"]
pub type I2C_SMIS_STARTMIS_R = crate::BitReader;
#[doc = "Field `I2C_SMIS_STARTMIS` writer - Start Condition Masked Interrupt Status"]
pub type I2C_SMIS_STARTMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_SMIS_STOPMIS` reader - Stop Condition Masked Interrupt Status"]
pub type I2C_SMIS_STOPMIS_R = crate::BitReader;
#[doc = "Field `I2C_SMIS_STOPMIS` writer - Stop Condition Masked Interrupt Status"]
pub type I2C_SMIS_STOPMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_datamis(&self) -> I2C_SMIS_DATAMIS_R {
        I2C_SMIS_DATAMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_startmis(&self) -> I2C_SMIS_STARTMIS_R {
        I2C_SMIS_STARTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_smis_stopmis(&self) -> I2C_SMIS_STOPMIS_R {
        I2C_SMIS_STOPMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_smis_datamis(&mut self) -> I2C_SMIS_DATAMIS_W<SMIS_SPEC, 0> {
        I2C_SMIS_DATAMIS_W::new(self)
    }
    #[doc = "Bit 1 - Start Condition Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_smis_startmis(&mut self) -> I2C_SMIS_STARTMIS_W<SMIS_SPEC, 1> {
        I2C_SMIS_STARTMIS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Condition Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_smis_stopmis(&mut self) -> I2C_SMIS_STOPMIS_W<SMIS_SPEC, 2> {
        I2C_SMIS_STOPMIS_W::new(self)
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
#[doc = "I2C Slave Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMIS_SPEC;
impl crate::RegisterSpec for SMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smis::R`](R) reader structure"]
impl crate::Readable for SMIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smis::W`](W) writer structure"]
impl crate::Writable for SMIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMIS to value 0"]
impl crate::Resettable for SMIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
