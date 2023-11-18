#[doc = "Register `MCLKOCNT` reader"]
pub type R = crate::R<MCLKOCNT_SPEC>;
#[doc = "Register `MCLKOCNT` writer"]
pub type W = crate::W<MCLKOCNT_SPEC>;
#[doc = "Field `I2C_MCLKOCNT_CNTL` reader - I2C Master Count"]
pub type I2C_MCLKOCNT_CNTL_R = crate::FieldReader;
#[doc = "Field `I2C_MCLKOCNT_CNTL` writer - I2C Master Count"]
pub type I2C_MCLKOCNT_CNTL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - I2C Master Count"]
    #[inline(always)]
    pub fn i2c_mclkocnt_cntl(&self) -> I2C_MCLKOCNT_CNTL_R {
        I2C_MCLKOCNT_CNTL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - I2C Master Count"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mclkocnt_cntl(&mut self) -> I2C_MCLKOCNT_CNTL_W<MCLKOCNT_SPEC, 0> {
        I2C_MCLKOCNT_CNTL_W::new(self)
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
#[doc = "I2C Master Clock Low Timeout Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mclkocnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mclkocnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MCLKOCNT_SPEC;
impl crate::RegisterSpec for MCLKOCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mclkocnt::R`](R) reader structure"]
impl crate::Readable for MCLKOCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mclkocnt::W`](W) writer structure"]
impl crate::Writable for MCLKOCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCLKOCNT to value 0"]
impl crate::Resettable for MCLKOCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
