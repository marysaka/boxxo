#[doc = "Register `MMIS` reader"]
pub type R = crate::R<MMIS_SPEC>;
#[doc = "Register `MMIS` writer"]
pub type W = crate::W<MMIS_SPEC>;
#[doc = "Field `I2C_MMIS_MIS` reader - Masked Interrupt Status"]
pub type I2C_MMIS_MIS_R = crate::BitReader;
#[doc = "Field `I2C_MMIS_MIS` writer - Masked Interrupt Status"]
pub type I2C_MMIS_MIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MMIS_CLKMIS` reader - Clock Timeout Masked Interrupt Status"]
pub type I2C_MMIS_CLKMIS_R = crate::BitReader;
#[doc = "Field `I2C_MMIS_CLKMIS` writer - Clock Timeout Masked Interrupt Status"]
pub type I2C_MMIS_CLKMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_mis(&self) -> I2C_MMIS_MIS_R {
        I2C_MMIS_MIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    pub fn i2c_mmis_clkmis(&self) -> I2C_MMIS_CLKMIS_R {
        I2C_MMIS_CLKMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mmis_mis(&mut self) -> I2C_MMIS_MIS_W<MMIS_SPEC, 0> {
        I2C_MMIS_MIS_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mmis_clkmis(&mut self) -> I2C_MMIS_CLKMIS_W<MMIS_SPEC, 1> {
        I2C_MMIS_CLKMIS_W::new(self)
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
#[doc = "I2C Master Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMIS_SPEC;
impl crate::RegisterSpec for MMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmis::R`](R) reader structure"]
impl crate::Readable for MMIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmis::W`](W) writer structure"]
impl crate::Writable for MMIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMIS to value 0"]
impl crate::Resettable for MMIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
