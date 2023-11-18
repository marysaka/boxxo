#[doc = "Register `TBMR` reader"]
pub type R = crate::R<TBMR_SPEC>;
#[doc = "Register `TBMR` writer"]
pub type W = crate::W<TBMR_SPEC>;
#[doc = "Field `TIMER_TBMR_TBMR` reader - GPTM Timer B Mode"]
pub type TIMER_TBMR_TBMR_R = crate::FieldReader<TIMER_TBMR_TBMR_A>;
#[doc = "GPTM Timer B Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_TBMR_TBMR_A {
    #[doc = "1: One-Shot Timer mode"]
    TIMER_TBMR_TBMR_1_SHOT = 1,
    #[doc = "2: Periodic Timer mode"]
    TIMER_TBMR_TBMR_PERIOD = 2,
    #[doc = "3: Capture mode"]
    TIMER_TBMR_TBMR_CAP = 3,
}
impl From<TIMER_TBMR_TBMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TBMR_TBMR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_TBMR_TBMR_A {
    type Ux = u8;
}
impl TIMER_TBMR_TBMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMER_TBMR_TBMR_A> {
        match self.bits {
            1 => Some(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT),
            2 => Some(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD),
            3 => Some(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP),
            _ => None,
        }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_1_shot(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_period(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_timer_tbmr_tbmr_cap(&self) -> bool {
        *self == TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP
    }
}
#[doc = "Field `TIMER_TBMR_TBMR` writer - GPTM Timer B Mode"]
pub type TIMER_TBMR_TBMR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, TIMER_TBMR_TBMR_A>;
impl<'a, REG, const O: u8> TIMER_TBMR_TBMR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_1_shot(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_period(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr_cap(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_TBMR_TBMR_A::TIMER_TBMR_TBMR_CAP)
    }
}
#[doc = "Field `TIMER_TBMR_TBCMR` reader - GPTM Timer B Capture Mode"]
pub type TIMER_TBMR_TBCMR_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBCMR` writer - GPTM Timer B Capture Mode"]
pub type TIMER_TBMR_TBCMR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBAMS` reader - GPTM Timer B Alternate Mode Select"]
pub type TIMER_TBMR_TBAMS_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBAMS` writer - GPTM Timer B Alternate Mode Select"]
pub type TIMER_TBMR_TBAMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBCDIR` reader - GPTM Timer B Count Direction"]
pub type TIMER_TBMR_TBCDIR_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBCDIR` writer - GPTM Timer B Count Direction"]
pub type TIMER_TBMR_TBCDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBMIE` reader - GPTM Timer B Match Interrupt Enable"]
pub type TIMER_TBMR_TBMIE_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBMIE` writer - GPTM Timer B Match Interrupt Enable"]
pub type TIMER_TBMR_TBMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBWOT` reader - GPTM Timer B Wait-on-Trigger"]
pub type TIMER_TBMR_TBWOT_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBWOT` writer - GPTM Timer B Wait-on-Trigger"]
pub type TIMER_TBMR_TBWOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBSNAPS` reader - GPTM Timer B Snap-Shot Mode"]
pub type TIMER_TBMR_TBSNAPS_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBSNAPS` writer - GPTM Timer B Snap-Shot Mode"]
pub type TIMER_TBMR_TBSNAPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBILD` reader - GPTM Timer B Interval Load Write"]
pub type TIMER_TBMR_TBILD_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBILD` writer - GPTM Timer B Interval Load Write"]
pub type TIMER_TBMR_TBILD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBPWMIE` reader - GPTM Timer B PWM Interrupt Enable"]
pub type TIMER_TBMR_TBPWMIE_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBPWMIE` writer - GPTM Timer B PWM Interrupt Enable"]
pub type TIMER_TBMR_TBPWMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBMRSU` reader - GPTM Timer B Match Register Update"]
pub type TIMER_TBMR_TBMRSU_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBMRSU` writer - GPTM Timer B Match Register Update"]
pub type TIMER_TBMR_TBMRSU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TBMR_TBPLO` reader - GPTM Timer B PWM Legacy Operation"]
pub type TIMER_TBMR_TBPLO_R = crate::BitReader;
#[doc = "Field `TIMER_TBMR_TBPLO` writer - GPTM Timer B PWM Legacy Operation"]
pub type TIMER_TBMR_TBPLO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbmr(&self) -> TIMER_TBMR_TBMR_R {
        TIMER_TBMR_TBMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbcmr(&self) -> TIMER_TBMR_TBCMR_R {
        TIMER_TBMR_TBCMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tbmr_tbams(&self) -> TIMER_TBMR_TBAMS_R {
        TIMER_TBMR_TBAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    pub fn timer_tbmr_tbcdir(&self) -> TIMER_TBMR_TBCDIR_R {
        TIMER_TBMR_TBCDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbmie(&self) -> TIMER_TBMR_TBMIE_R {
        TIMER_TBMR_TBMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tbmr_tbwot(&self) -> TIMER_TBMR_TBWOT_R {
        TIMER_TBMR_TBWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tbmr_tbsnaps(&self) -> TIMER_TBMR_TBSNAPS_R {
        TIMER_TBMR_TBSNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    pub fn timer_tbmr_tbild(&self) -> TIMER_TBMR_TBILD_R {
        TIMER_TBMR_TBILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tbmr_tbpwmie(&self) -> TIMER_TBMR_TBPWMIE_R {
        TIMER_TBMR_TBPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    pub fn timer_tbmr_tbmrsu(&self) -> TIMER_TBMR_TBMRSU_R {
        TIMER_TBMR_TBMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tbmr_tbplo(&self) -> TIMER_TBMR_TBPLO_R {
        TIMER_TBMR_TBPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer B Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbmr(&mut self) -> TIMER_TBMR_TBMR_W<TBMR_SPEC, 0> {
        TIMER_TBMR_TBMR_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer B Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbcmr(&mut self) -> TIMER_TBMR_TBCMR_W<TBMR_SPEC, 2> {
        TIMER_TBMR_TBCMR_W::new(self)
    }
    #[doc = "Bit 3 - GPTM Timer B Alternate Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbams(&mut self) -> TIMER_TBMR_TBAMS_W<TBMR_SPEC, 3> {
        TIMER_TBMR_TBAMS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer B Count Direction"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbcdir(&mut self) -> TIMER_TBMR_TBCDIR_W<TBMR_SPEC, 4> {
        TIMER_TBMR_TBCDIR_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer B Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbmie(&mut self) -> TIMER_TBMR_TBMIE_W<TBMR_SPEC, 5> {
        TIMER_TBMR_TBMIE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer B Wait-on-Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbwot(&mut self) -> TIMER_TBMR_TBWOT_W<TBMR_SPEC, 6> {
        TIMER_TBMR_TBWOT_W::new(self)
    }
    #[doc = "Bit 7 - GPTM Timer B Snap-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbsnaps(&mut self) -> TIMER_TBMR_TBSNAPS_W<TBMR_SPEC, 7> {
        TIMER_TBMR_TBSNAPS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B Interval Load Write"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbild(&mut self) -> TIMER_TBMR_TBILD_W<TBMR_SPEC, 8> {
        TIMER_TBMR_TBILD_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B PWM Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbpwmie(&mut self) -> TIMER_TBMR_TBPWMIE_W<TBMR_SPEC, 9> {
        TIMER_TBMR_TBPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B Match Register Update"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbmrsu(&mut self) -> TIMER_TBMR_TBMRSU_W<TBMR_SPEC, 10> {
        TIMER_TBMR_TBMRSU_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B PWM Legacy Operation"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tbmr_tbplo(&mut self) -> TIMER_TBMR_TBPLO_W<TBMR_SPEC, 11> {
        TIMER_TBMR_TBPLO_W::new(self)
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
#[doc = "GPTM Timer B Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBMR_SPEC;
impl crate::RegisterSpec for TBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbmr::R`](R) reader structure"]
impl crate::Readable for TBMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbmr::W`](W) writer structure"]
impl crate::Writable for TBMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBMR to value 0"]
impl crate::Resettable for TBMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
