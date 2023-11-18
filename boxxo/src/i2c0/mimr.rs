#[doc = "Register `MIMR` reader"]
pub type R = crate::R<MIMR_SPEC>;
#[doc = "Register `MIMR` writer"]
pub type W = crate::W<MIMR_SPEC>;
#[doc = "Field `I2C_MIMR_IM` reader - Master Interrupt Mask"]
pub type I2C_MIMR_IM_R = crate::BitReader;
#[doc = "Field `I2C_MIMR_IM` writer - Master Interrupt Mask"]
pub type I2C_MIMR_IM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `I2C_MIMR_CLKIM` reader - Clock Timeout Interrupt Mask"]
pub type I2C_MIMR_CLKIM_R = crate::BitReader;
#[doc = "Field `I2C_MIMR_CLKIM` writer - Clock Timeout Interrupt Mask"]
pub type I2C_MIMR_CLKIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_im(&self) -> I2C_MIMR_IM_R {
        I2C_MIMR_IM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_mimr_clkim(&self) -> I2C_MIMR_CLKIM_R {
        I2C_MIMR_CLKIM_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mimr_im(&mut self) -> I2C_MIMR_IM_W<MIMR_SPEC, 0> {
        I2C_MIMR_IM_W::new(self)
    }
    #[doc = "Bit 1 - Clock Timeout Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn i2c_mimr_clkim(&mut self) -> I2C_MIMR_CLKIM_W<MIMR_SPEC, 1> {
        I2C_MIMR_CLKIM_W::new(self)
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
#[doc = "I2C Master Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mimr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mimr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIMR_SPEC;
impl crate::RegisterSpec for MIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mimr::R`](R) reader structure"]
impl crate::Readable for MIMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mimr::W`](W) writer structure"]
impl crate::Writable for MIMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIMR to value 0"]
impl crate::Resettable for MIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
