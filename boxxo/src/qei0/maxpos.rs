#[doc = "Register `MAXPOS` reader"]
pub type R = crate::R<MAXPOS_SPEC>;
#[doc = "Register `MAXPOS` writer"]
pub type W = crate::W<MAXPOS_SPEC>;
#[doc = "Field `QEI_MAXPOS` reader - Maximum Position Integrator Value"]
pub type QEI_MAXPOS_R = crate::FieldReader<u32>;
#[doc = "Field `QEI_MAXPOS` writer - Maximum Position Integrator Value"]
pub type QEI_MAXPOS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Maximum Position Integrator Value"]
    #[inline(always)]
    pub fn qei_maxpos(&self) -> QEI_MAXPOS_R {
        QEI_MAXPOS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Maximum Position Integrator Value"]
    #[inline(always)]
    #[must_use]
    pub fn qei_maxpos(&mut self) -> QEI_MAXPOS_W<MAXPOS_SPEC, 0> {
        QEI_MAXPOS_W::new(self)
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
#[doc = "QEI Maximum Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maxpos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maxpos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAXPOS_SPEC;
impl crate::RegisterSpec for MAXPOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxpos::R`](R) reader structure"]
impl crate::Readable for MAXPOS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maxpos::W`](W) writer structure"]
impl crate::Writable for MAXPOS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAXPOS to value 0"]
impl crate::Resettable for MAXPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
