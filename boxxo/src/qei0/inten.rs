#[doc = "Register `INTEN` reader"]
pub type R = crate::R<INTEN_SPEC>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<INTEN_SPEC>;
#[doc = "Field `QEI_INTEN_INDEX` reader - Index Pulse Detected Interrupt Enable"]
pub type QEI_INTEN_INDEX_R = crate::BitReader;
#[doc = "Field `QEI_INTEN_INDEX` writer - Index Pulse Detected Interrupt Enable"]
pub type QEI_INTEN_INDEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_INTEN_TIMER` reader - Timer Expires Interrupt Enable"]
pub type QEI_INTEN_TIMER_R = crate::BitReader;
#[doc = "Field `QEI_INTEN_TIMER` writer - Timer Expires Interrupt Enable"]
pub type QEI_INTEN_TIMER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_INTEN_DIR` reader - Direction Change Interrupt Enable"]
pub type QEI_INTEN_DIR_R = crate::BitReader;
#[doc = "Field `QEI_INTEN_DIR` writer - Direction Change Interrupt Enable"]
pub type QEI_INTEN_DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_INTEN_ERROR` reader - Phase Error Interrupt Enable"]
pub type QEI_INTEN_ERROR_R = crate::BitReader;
#[doc = "Field `QEI_INTEN_ERROR` writer - Phase Error Interrupt Enable"]
pub type QEI_INTEN_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Index Pulse Detected Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_index(&self) -> QEI_INTEN_INDEX_R {
        QEI_INTEN_INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Expires Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_timer(&self) -> QEI_INTEN_TIMER_R {
        QEI_INTEN_TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_dir(&self) -> QEI_INTEN_DIR_R {
        QEI_INTEN_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Phase Error Interrupt Enable"]
    #[inline(always)]
    pub fn qei_inten_error(&self) -> QEI_INTEN_ERROR_R {
        QEI_INTEN_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qei_inten_index(&mut self) -> QEI_INTEN_INDEX_W<INTEN_SPEC, 0> {
        QEI_INTEN_INDEX_W::new(self)
    }
    #[doc = "Bit 1 - Timer Expires Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qei_inten_timer(&mut self) -> QEI_INTEN_TIMER_W<INTEN_SPEC, 1> {
        QEI_INTEN_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qei_inten_dir(&mut self) -> QEI_INTEN_DIR_W<INTEN_SPEC, 2> {
        QEI_INTEN_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Phase Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qei_inten_error(&mut self) -> QEI_INTEN_ERROR_W<INTEN_SPEC, 3> {
        QEI_INTEN_ERROR_W::new(self)
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
#[doc = "QEI Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for INTEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
