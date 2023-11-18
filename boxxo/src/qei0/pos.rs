#[doc = "Register `POS` reader"]
pub type R = crate::R<POS_SPEC>;
#[doc = "Register `POS` writer"]
pub type W = crate::W<POS_SPEC>;
#[doc = "Field `QEI_POS` reader - Current Position Integrator Value"]
pub type QEI_POS_R = crate::FieldReader<u32>;
#[doc = "Field `QEI_POS` writer - Current Position Integrator Value"]
pub type QEI_POS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Position Integrator Value"]
    #[inline(always)]
    pub fn qei_pos(&self) -> QEI_POS_R {
        QEI_POS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current Position Integrator Value"]
    #[inline(always)]
    #[must_use]
    pub fn qei_pos(&mut self) -> QEI_POS_W<POS_SPEC, 0> {
        QEI_POS_W::new(self)
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
#[doc = "QEI Position\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pos::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pos::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POS_SPEC;
impl crate::RegisterSpec for POS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pos::R`](R) reader structure"]
impl crate::Readable for POS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pos::W`](W) writer structure"]
impl crate::Writable for POS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POS to value 0"]
impl crate::Resettable for POS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
