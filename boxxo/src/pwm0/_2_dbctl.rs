#[doc = "Register `_2_DBCTL` reader"]
pub type R = crate::R<_2_DBCTL_SPEC>;
#[doc = "Register `_2_DBCTL` writer"]
pub type W = crate::W<_2_DBCTL_SPEC>;
#[doc = "Field `PWM_2_DBCTL_ENABLE` reader - Dead-Band Generator Enable"]
pub type PWM_2_DBCTL_ENABLE_R = crate::BitReader;
#[doc = "Field `PWM_2_DBCTL_ENABLE` writer - Dead-Band Generator Enable"]
pub type PWM_2_DBCTL_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Dead-Band Generator Enable"]
    #[inline(always)]
    pub fn pwm_2_dbctl_enable(&self) -> PWM_2_DBCTL_ENABLE_R {
        PWM_2_DBCTL_ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Dead-Band Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_dbctl_enable(&mut self) -> PWM_2_DBCTL_ENABLE_W<_2_DBCTL_SPEC, 0> {
        PWM_2_DBCTL_ENABLE_W::new(self)
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
#[doc = "PWM2 Dead-Band Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_dbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_dbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2_DBCTL_SPEC;
impl crate::RegisterSpec for _2_DBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2_dbctl::R`](R) reader structure"]
impl crate::Readable for _2_DBCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_2_dbctl::W`](W) writer structure"]
impl crate::Writable for _2_DBCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _2_DBCTL to value 0"]
impl crate::Resettable for _2_DBCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
