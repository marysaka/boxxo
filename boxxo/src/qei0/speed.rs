#[doc = "Register `SPEED` reader"]
pub type R = crate::R<SPEED_SPEC>;
#[doc = "Register `SPEED` writer"]
pub type W = crate::W<SPEED_SPEC>;
#[doc = "Field `QEI_SPEED` reader - Velocity"]
pub type QEI_SPEED_R = crate::FieldReader<u32>;
#[doc = "Field `QEI_SPEED` writer - Velocity"]
pub type QEI_SPEED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Velocity"]
    #[inline(always)]
    pub fn qei_speed(&self) -> QEI_SPEED_R {
        QEI_SPEED_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Velocity"]
    #[inline(always)]
    #[must_use]
    pub fn qei_speed(&mut self) -> QEI_SPEED_W<SPEED_SPEC, 0> {
        QEI_SPEED_W::new(self)
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
#[doc = "QEI Velocity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`speed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`speed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPEED_SPEC;
impl crate::RegisterSpec for SPEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`speed::R`](R) reader structure"]
impl crate::Readable for SPEED_SPEC {}
#[doc = "`write(|w| ..)` method takes [`speed::W`](W) writer structure"]
impl crate::Writable for SPEED_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPEED to value 0"]
impl crate::Resettable for SPEED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
