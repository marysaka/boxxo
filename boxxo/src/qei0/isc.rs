#[doc = "Register `ISC` reader"]
pub type R = crate::R<ISC_SPEC>;
#[doc = "Register `ISC` writer"]
pub type W = crate::W<ISC_SPEC>;
#[doc = "Field `QEI_ISC_INDEX` reader - Index Pulse Interrupt"]
pub type QEI_ISC_INDEX_R = crate::BitReader;
#[doc = "Field `QEI_ISC_INDEX` writer - Index Pulse Interrupt"]
pub type QEI_ISC_INDEX_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_ISC_TIMER` reader - Velocity Timer Expired Interrupt"]
pub type QEI_ISC_TIMER_R = crate::BitReader;
#[doc = "Field `QEI_ISC_TIMER` writer - Velocity Timer Expired Interrupt"]
pub type QEI_ISC_TIMER_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_ISC_DIR` reader - Direction Change Interrupt"]
pub type QEI_ISC_DIR_R = crate::BitReader;
#[doc = "Field `QEI_ISC_DIR` writer - Direction Change Interrupt"]
pub type QEI_ISC_DIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `QEI_ISC_ERROR` reader - Phase Error Interrupt"]
pub type QEI_ISC_ERROR_R = crate::BitReader;
#[doc = "Field `QEI_ISC_ERROR` writer - Phase Error Interrupt"]
pub type QEI_ISC_ERROR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&self) -> QEI_ISC_INDEX_R {
        QEI_ISC_INDEX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&self) -> QEI_ISC_TIMER_R {
        QEI_ISC_TIMER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&self) -> QEI_ISC_DIR_R {
        QEI_ISC_DIR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&self) -> QEI_ISC_ERROR_R {
        QEI_ISC_ERROR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qei_isc_index(&mut self) -> QEI_ISC_INDEX_W<ISC_SPEC, 0> {
        QEI_ISC_INDEX_W::new(self)
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qei_isc_timer(&mut self) -> QEI_ISC_TIMER_W<ISC_SPEC, 1> {
        QEI_ISC_TIMER_W::new(self)
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qei_isc_dir(&mut self) -> QEI_ISC_DIR_W<ISC_SPEC, 2> {
        QEI_ISC_DIR_W::new(self)
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn qei_isc_error(&mut self) -> QEI_ISC_ERROR_W<ISC_SPEC, 3> {
        QEI_ISC_ERROR_W::new(self)
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
#[doc = "QEI Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISC_SPEC;
impl crate::RegisterSpec for ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc::R`](R) reader structure"]
impl crate::Readable for ISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isc::W`](W) writer structure"]
impl crate::Writable for ISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for ISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
