#[doc = "Register `TIME` reader"]
pub type R = crate::R<TIME_SPEC>;
#[doc = "Register `TIME` writer"]
pub type W = crate::W<TIME_SPEC>;
#[doc = "Field `QEI_TIME` reader - Velocity Timer Current Value"]
pub type QEI_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `QEI_TIME` writer - Velocity Timer Current Value"]
pub type QEI_TIME_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Velocity Timer Current Value"]
    #[inline(always)]
    pub fn qei_time(&self) -> QEI_TIME_R {
        QEI_TIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Velocity Timer Current Value"]
    #[inline(always)]
    #[must_use]
    pub fn qei_time(&mut self) -> QEI_TIME_W<TIME_SPEC, 0> {
        QEI_TIME_W::new(self)
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
#[doc = "QEI Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`time::R`](R) reader structure"]
impl crate::Readable for TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`time::W`](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
