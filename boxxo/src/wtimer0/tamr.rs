#[doc = "Register `TAMR` reader"]
pub type R = crate::R<TAMR_SPEC>;
#[doc = "Register `TAMR` writer"]
pub type W = crate::W<TAMR_SPEC>;
#[doc = "Field `TIMER_TAMR_TAMR` reader - GPTM Timer A Mode"]
pub type TIMER_TAMR_TAMR_R = crate::FieldReader<TIMER_TAMR_TAMR_A>;
#[doc = "GPTM Timer A Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMER_TAMR_TAMR_A {
    #[doc = "1: One-Shot Timer mode"]
    TIMER_TAMR_TAMR_1_SHOT = 1,
    #[doc = "2: Periodic Timer mode"]
    TIMER_TAMR_TAMR_PERIOD = 2,
    #[doc = "3: Capture mode"]
    TIMER_TAMR_TAMR_CAP = 3,
}
impl From<TIMER_TAMR_TAMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMER_TAMR_TAMR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIMER_TAMR_TAMR_A {
    type Ux = u8;
}
impl TIMER_TAMR_TAMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TIMER_TAMR_TAMR_A> {
        match self.bits {
            1 => Some(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT),
            2 => Some(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD),
            3 => Some(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP),
            _ => None,
        }
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_1_shot(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_period(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_timer_tamr_tamr_cap(&self) -> bool {
        *self == TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP
    }
}
#[doc = "Field `TIMER_TAMR_TAMR` writer - GPTM Timer A Mode"]
pub type TIMER_TAMR_TAMR_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 2, O, TIMER_TAMR_TAMR_A>;
impl<'a, REG, const O: u8> TIMER_TAMR_TAMR_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_1_shot(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_1_SHOT)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_period(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_PERIOD)
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr_cap(self) -> &'a mut crate::W<REG> {
        self.variant(TIMER_TAMR_TAMR_A::TIMER_TAMR_TAMR_CAP)
    }
}
#[doc = "Field `TIMER_TAMR_TACMR` reader - GPTM Timer A Capture Mode"]
pub type TIMER_TAMR_TACMR_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TACMR` writer - GPTM Timer A Capture Mode"]
pub type TIMER_TAMR_TACMR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAAMS` reader - GPTM Timer A Alternate Mode Select"]
pub type TIMER_TAMR_TAAMS_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAAMS` writer - GPTM Timer A Alternate Mode Select"]
pub type TIMER_TAMR_TAAMS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TACDIR` reader - GPTM Timer A Count Direction"]
pub type TIMER_TAMR_TACDIR_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TACDIR` writer - GPTM Timer A Count Direction"]
pub type TIMER_TAMR_TACDIR_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAMIE` reader - GPTM Timer A Match Interrupt Enable"]
pub type TIMER_TAMR_TAMIE_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAMIE` writer - GPTM Timer A Match Interrupt Enable"]
pub type TIMER_TAMR_TAMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAWOT` reader - GPTM Timer A Wait-on-Trigger"]
pub type TIMER_TAMR_TAWOT_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAWOT` writer - GPTM Timer A Wait-on-Trigger"]
pub type TIMER_TAMR_TAWOT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TASNAPS` reader - GPTM Timer A Snap-Shot Mode"]
pub type TIMER_TAMR_TASNAPS_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TASNAPS` writer - GPTM Timer A Snap-Shot Mode"]
pub type TIMER_TAMR_TASNAPS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAILD` reader - GPTM Timer A Interval Load Write"]
pub type TIMER_TAMR_TAILD_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAILD` writer - GPTM Timer A Interval Load Write"]
pub type TIMER_TAMR_TAILD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAPWMIE` reader - GPTM Timer A PWM Interrupt Enable"]
pub type TIMER_TAMR_TAPWMIE_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAPWMIE` writer - GPTM Timer A PWM Interrupt Enable"]
pub type TIMER_TAMR_TAPWMIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAMRSU` reader - GPTM Timer A Match Register Update"]
pub type TIMER_TAMR_TAMRSU_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAMRSU` writer - GPTM Timer A Match Register Update"]
pub type TIMER_TAMR_TAMRSU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TIMER_TAMR_TAPLO` reader - GPTM Timer A PWM Legacy Operation"]
pub type TIMER_TAMR_TAPLO_R = crate::BitReader;
#[doc = "Field `TIMER_TAMR_TAPLO` writer - GPTM Timer A PWM Legacy Operation"]
pub type TIMER_TAMR_TAPLO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    pub fn timer_tamr_tamr(&self) -> TIMER_TAMR_TAMR_R {
        TIMER_TAMR_TAMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    pub fn timer_tamr_tacmr(&self) -> TIMER_TAMR_TACMR_R {
        TIMER_TAMR_TACMR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    pub fn timer_tamr_taams(&self) -> TIMER_TAMR_TAAMS_R {
        TIMER_TAMR_TAAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    pub fn timer_tamr_tacdir(&self) -> TIMER_TAMR_TACDIR_R {
        TIMER_TAMR_TACDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tamie(&self) -> TIMER_TAMR_TAMIE_R {
        TIMER_TAMR_TAMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    pub fn timer_tamr_tawot(&self) -> TIMER_TAMR_TAWOT_R {
        TIMER_TAMR_TAWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn timer_tamr_tasnaps(&self) -> TIMER_TAMR_TASNAPS_R {
        TIMER_TAMR_TASNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    pub fn timer_tamr_taild(&self) -> TIMER_TAMR_TAILD_R {
        TIMER_TAMR_TAILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    pub fn timer_tamr_tapwmie(&self) -> TIMER_TAMR_TAPWMIE_R {
        TIMER_TAMR_TAPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    pub fn timer_tamr_tamrsu(&self) -> TIMER_TAMR_TAMRSU_R {
        TIMER_TAMR_TAMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    pub fn timer_tamr_taplo(&self) -> TIMER_TAMR_TAPLO_R {
        TIMER_TAMR_TAPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPTM Timer A Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tamr(&mut self) -> TIMER_TAMR_TAMR_W<TAMR_SPEC, 0> {
        TIMER_TAMR_TAMR_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tacmr(&mut self) -> TIMER_TAMR_TACMR_W<TAMR_SPEC, 2> {
        TIMER_TAMR_TACMR_W::new(self)
    }
    #[doc = "Bit 3 - GPTM Timer A Alternate Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_taams(&mut self) -> TIMER_TAMR_TAAMS_W<TAMR_SPEC, 3> {
        TIMER_TAMR_TAAMS_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A Count Direction"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tacdir(&mut self) -> TIMER_TAMR_TACDIR_W<TAMR_SPEC, 4> {
        TIMER_TAMR_TACDIR_W::new(self)
    }
    #[doc = "Bit 5 - GPTM Timer A Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tamie(&mut self) -> TIMER_TAMR_TAMIE_W<TAMR_SPEC, 5> {
        TIMER_TAMR_TAMIE_W::new(self)
    }
    #[doc = "Bit 6 - GPTM Timer A Wait-on-Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tawot(&mut self) -> TIMER_TAMR_TAWOT_W<TAMR_SPEC, 6> {
        TIMER_TAMR_TAWOT_W::new(self)
    }
    #[doc = "Bit 7 - GPTM Timer A Snap-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tasnaps(&mut self) -> TIMER_TAMR_TASNAPS_W<TAMR_SPEC, 7> {
        TIMER_TAMR_TASNAPS_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer A Interval Load Write"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_taild(&mut self) -> TIMER_TAMR_TAILD_W<TAMR_SPEC, 8> {
        TIMER_TAMR_TAILD_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer A PWM Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tapwmie(&mut self) -> TIMER_TAMR_TAPWMIE_W<TAMR_SPEC, 9> {
        TIMER_TAMR_TAPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer A Match Register Update"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_tamrsu(&mut self) -> TIMER_TAMR_TAMRSU_W<TAMR_SPEC, 10> {
        TIMER_TAMR_TAMRSU_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer A PWM Legacy Operation"]
    #[inline(always)]
    #[must_use]
    pub fn timer_tamr_taplo(&mut self) -> TIMER_TAMR_TAPLO_W<TAMR_SPEC, 11> {
        TIMER_TAMR_TAPLO_W::new(self)
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
#[doc = "GPTM Timer A Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TAMR_SPEC;
impl crate::RegisterSpec for TAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamr::R`](R) reader structure"]
impl crate::Readable for TAMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tamr::W`](W) writer structure"]
impl crate::Writable for TAMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMR to value 0"]
impl crate::Resettable for TAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
