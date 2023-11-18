#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `QEI_RIS_INDEX` reader - Index Pulse Asserted"]
pub type QEI_RIS_INDEX_R = crate::BitReader;
#[doc = "Field `QEI_RIS_INDEX` writer - Index Pulse Asserted"]
pub type QEI_RIS_INDEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_RIS_TIMER` reader - Velocity Timer Expired"]
pub type QEI_RIS_TIMER_R = crate::BitReader;
#[doc = "Field `QEI_RIS_TIMER` writer - Velocity Timer Expired"]
pub type QEI_RIS_TIMER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_RIS_DIR` reader - Direction Change Detected"]
pub type QEI_RIS_DIR_R = crate::BitReader;
#[doc = "Field `QEI_RIS_DIR` writer - Direction Change Detected"]
pub type QEI_RIS_DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_RIS_ERROR` reader - Phase Error Detected"]
pub type QEI_RIS_ERROR_R = crate::BitReader;
#[doc = "Field `QEI_RIS_ERROR` writer - Phase Error Detected"]
pub type QEI_RIS_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Index Pulse Asserted"]
    #[inline(always)]
    pub fn qei_ris_index(&self) -> QEI_RIS_INDEX_R {
        QEI_RIS_INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Velocity Timer Expired"]
    #[inline(always)]
    pub fn qei_ris_timer(&self) -> QEI_RIS_TIMER_R {
        QEI_RIS_TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detected"]
    #[inline(always)]
    pub fn qei_ris_dir(&self) -> QEI_RIS_DIR_R {
        QEI_RIS_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Phase Error Detected"]
    #[inline(always)]
    pub fn qei_ris_error(&self) -> QEI_RIS_ERROR_R {
        QEI_RIS_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Asserted"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ris_index(&mut self) -> QEI_RIS_INDEX_W<RIS_SPEC, 0> {
        QEI_RIS_INDEX_W::new(self)
    }
    #[doc = "Bit 1 - Velocity Timer Expired"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ris_timer(&mut self) -> QEI_RIS_TIMER_W<RIS_SPEC, 1> {
        QEI_RIS_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Detected"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ris_dir(&mut self) -> QEI_RIS_DIR_W<RIS_SPEC, 2> {
        QEI_RIS_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Phase Error Detected"]
    #[inline(always)]
    #[must_use]
    pub fn qei_ris_error(&mut self) -> QEI_RIS_ERROR_W<RIS_SPEC, 3> {
        QEI_RIS_ERROR_W::new(self)
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
#[doc = "QEI Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
