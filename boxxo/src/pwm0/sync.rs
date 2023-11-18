#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SYNC_SPEC>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SYNC_SPEC>;
#[doc = "Field `PWM_SYNC_SYNC0` reader - Reset Generator 0 Counter"]
pub type PWM_SYNC_SYNC0_R = crate::BitReader;
#[doc = "Field `PWM_SYNC_SYNC0` writer - Reset Generator 0 Counter"]
pub type PWM_SYNC_SYNC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_SYNC_SYNC1` reader - Reset Generator 1 Counter"]
pub type PWM_SYNC_SYNC1_R = crate::BitReader;
#[doc = "Field `PWM_SYNC_SYNC1` writer - Reset Generator 1 Counter"]
pub type PWM_SYNC_SYNC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_SYNC_SYNC2` reader - Reset Generator 2 Counter"]
pub type PWM_SYNC_SYNC2_R = crate::BitReader;
#[doc = "Field `PWM_SYNC_SYNC2` writer - Reset Generator 2 Counter"]
pub type PWM_SYNC_SYNC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_SYNC_SYNC3` reader - Reset Generator 3 Counter"]
pub type PWM_SYNC_SYNC3_R = crate::BitReader;
#[doc = "Field `PWM_SYNC_SYNC3` writer - Reset Generator 3 Counter"]
pub type PWM_SYNC_SYNC3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Reset Generator 0 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync0(&self) -> PWM_SYNC_SYNC0_R {
        PWM_SYNC_SYNC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset Generator 1 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync1(&self) -> PWM_SYNC_SYNC1_R {
        PWM_SYNC_SYNC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset Generator 2 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync2(&self) -> PWM_SYNC_SYNC2_R {
        PWM_SYNC_SYNC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Generator 3 Counter"]
    #[inline(always)]
    pub fn pwm_sync_sync3(&self) -> PWM_SYNC_SYNC3_R {
        PWM_SYNC_SYNC3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Generator 0 Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_sync0(&mut self) -> PWM_SYNC_SYNC0_W<SYNC_SPEC, 0> {
        PWM_SYNC_SYNC0_W::new(self)
    }
    #[doc = "Bit 1 - Reset Generator 1 Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_sync1(&mut self) -> PWM_SYNC_SYNC1_W<SYNC_SPEC, 1> {
        PWM_SYNC_SYNC1_W::new(self)
    }
    #[doc = "Bit 2 - Reset Generator 2 Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_sync2(&mut self) -> PWM_SYNC_SYNC2_W<SYNC_SPEC, 2> {
        PWM_SYNC_SYNC2_W::new(self)
    }
    #[doc = "Bit 3 - Reset Generator 3 Counter"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_sync_sync3(&mut self) -> PWM_SYNC_SYNC3_W<SYNC_SPEC, 3> {
        PWM_SYNC_SYNC3_W::new(self)
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
#[doc = "PWM Time Base Sync\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
