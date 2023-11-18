#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `TIMER_IMR_TATOIM` reader - GPTM Timer A Time-Out Interrupt Mask"]
pub type TIMER_IMR_TATOIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_TATOIM` writer - GPTM Timer A Time-Out Interrupt Mask"]
pub type TIMER_IMR_TATOIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_CAMIM` reader - GPTM Timer A Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CAMIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_CAMIM` writer - GPTM Timer A Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CAMIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_CAEIM` reader - GPTM Timer A Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CAEIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_CAEIM` writer - GPTM Timer A Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CAEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_RTCIM` reader - GPTM RTC Interrupt Mask"]
pub type TIMER_IMR_RTCIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_RTCIM` writer - GPTM RTC Interrupt Mask"]
pub type TIMER_IMR_RTCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_TAMIM` reader - GPTM Timer A Match Interrupt Mask"]
pub type TIMER_IMR_TAMIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_TAMIM` writer - GPTM Timer A Match Interrupt Mask"]
pub type TIMER_IMR_TAMIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_TBTOIM` reader - GPTM Timer B Time-Out Interrupt Mask"]
pub type TIMER_IMR_TBTOIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_TBTOIM` writer - GPTM Timer B Time-Out Interrupt Mask"]
pub type TIMER_IMR_TBTOIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_CBMIM` reader - GPTM Timer B Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CBMIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_CBMIM` writer - GPTM Timer B Capture Mode Match Interrupt Mask"]
pub type TIMER_IMR_CBMIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_CBEIM` reader - GPTM Timer B Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CBEIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_CBEIM` writer - GPTM Timer B Capture Mode Event Interrupt Mask"]
pub type TIMER_IMR_CBEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_TBMIM` reader - GPTM Timer B Match Interrupt Mask"]
pub type TIMER_IMR_TBMIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_TBMIM` writer - GPTM Timer B Match Interrupt Mask"]
pub type TIMER_IMR_TBMIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_IMR_WUEIM` reader - GPTM Write Update Error Interrupt Mask"]
pub type TIMER_IMR_WUEIM_R = crate::BitReader;
#[doc = "Field `TIMER_IMR_WUEIM` writer - GPTM Write Update Error Interrupt Mask"]
pub type TIMER_IMR_WUEIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&self) -> TIMER_IMR_TATOIM_R {
        TIMER_IMR_TATOIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&self) -> TIMER_IMR_CAMIM_R {
        TIMER_IMR_CAMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&self) -> TIMER_IMR_CAEIM_R {
        TIMER_IMR_CAEIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&self) -> TIMER_IMR_RTCIM_R {
        TIMER_IMR_RTCIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tamim(&self) -> TIMER_IMR_TAMIM_R {
        TIMER_IMR_TAMIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&self) -> TIMER_IMR_TBTOIM_R {
        TIMER_IMR_TBTOIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&self) -> TIMER_IMR_CBMIM_R {
        TIMER_IMR_CBMIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&self) -> TIMER_IMR_CBEIM_R {
        TIMER_IMR_CBEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbmim(&self) -> TIMER_IMR_TBMIM_R {
        TIMER_IMR_TBMIM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GPTM Write Update Error Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_wueim(&self) -> TIMER_IMR_WUEIM_R {
        TIMER_IMR_WUEIM_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_tatoim(&mut self) -> TIMER_IMR_TATOIM_W<IMR_SPEC, 0> {
        TIMER_IMR_TATOIM_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_camim(&mut self) -> TIMER_IMR_CAMIM_W<IMR_SPEC, 1> {
        TIMER_IMR_CAMIM_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_caeim(&mut self) -> TIMER_IMR_CAEIM_W<IMR_SPEC, 2> {
        TIMER_IMR_CAEIM_W::new(self)
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_rtcim(&mut self) -> TIMER_IMR_RTCIM_W<IMR_SPEC, 3> {
        TIMER_IMR_RTCIM_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_tamim(&mut self) -> TIMER_IMR_TAMIM_W<IMR_SPEC, 4> {
        TIMER_IMR_TAMIM_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_tbtoim(&mut self) -> TIMER_IMR_TBTOIM_W<IMR_SPEC, 8> {
        TIMER_IMR_TBTOIM_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_cbmim(&mut self) -> TIMER_IMR_CBMIM_W<IMR_SPEC, 9> {
        TIMER_IMR_CBMIM_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_cbeim(&mut self) -> TIMER_IMR_CBEIM_W<IMR_SPEC, 10> {
        TIMER_IMR_CBEIM_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_tbmim(&mut self) -> TIMER_IMR_TBMIM_W<IMR_SPEC, 11> {
        TIMER_IMR_TBMIM_W::new(self)
    }
    #[doc = "Bit 16 - GPTM Write Update Error Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn timer_imr_wueim(&mut self) -> TIMER_IMR_WUEIM_W<IMR_SPEC, 16> {
        TIMER_IMR_WUEIM_W::new(self)
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
#[doc = "GPTM Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
