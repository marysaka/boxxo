#[doc = "Register `MTPR` reader"]
pub type R = crate::R<MTPR_SPEC>;
#[doc = "Register `MTPR` writer"]
pub type W = crate::W<MTPR_SPEC>;
#[doc = "Field `I2C_MTPR_TPR` reader - Timer Period"]
pub type I2C_MTPR_TPR_R = crate::FieldReader;
#[doc = "Field `I2C_MTPR_TPR` writer - Timer Period"]
pub type I2C_MTPR_TPR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `I2C_MTPR_HS` reader - High-Speed Enable"]
pub type I2C_MTPR_HS_R = crate::BitReader;
#[doc = "Field `I2C_MTPR_HS` writer - High-Speed Enable"]
pub type I2C_MTPR_HS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    pub fn i2c_mtpr_tpr(&self) -> I2C_MTPR_TPR_R {
        I2C_MTPR_TPR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mtpr_hs(&self) -> I2C_MTPR_HS_R {
        I2C_MTPR_HS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Timer Period"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mtpr_tpr(&mut self) -> I2C_MTPR_TPR_W<MTPR_SPEC, 0> {
        I2C_MTPR_TPR_W::new(self)
    }
    #[doc = "Bit 7 - High-Speed Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mtpr_hs(&mut self) -> I2C_MTPR_HS_W<MTPR_SPEC, 7> {
        I2C_MTPR_HS_W::new(self)
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
#[doc = "I2C Master Timer Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTPR_SPEC;
impl crate::RegisterSpec for MTPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtpr::R`](R) reader structure"]
impl crate::Readable for MTPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtpr::W`](W) writer structure"]
impl crate::Writable for MTPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTPR to value 0"]
impl crate::Resettable for MTPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
