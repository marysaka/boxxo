#[doc = "Register `DC5` reader"]
pub type R = crate::R<DC5_SPEC>;
#[doc = "Register `DC5` writer"]
pub type W = crate::W<DC5_SPEC>;
#[doc = "Field `SYSCTL_DC5_PWM0` reader - PWM0 Pin Present"]
pub type SYSCTL_DC5_PWM0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM0` writer - PWM0 Pin Present"]
pub type SYSCTL_DC5_PWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM1` reader - PWM1 Pin Present"]
pub type SYSCTL_DC5_PWM1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM1` writer - PWM1 Pin Present"]
pub type SYSCTL_DC5_PWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM2` reader - PWM2 Pin Present"]
pub type SYSCTL_DC5_PWM2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM2` writer - PWM2 Pin Present"]
pub type SYSCTL_DC5_PWM2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM3` reader - PWM3 Pin Present"]
pub type SYSCTL_DC5_PWM3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM3` writer - PWM3 Pin Present"]
pub type SYSCTL_DC5_PWM3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM4` reader - PWM4 Pin Present"]
pub type SYSCTL_DC5_PWM4_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM4` writer - PWM4 Pin Present"]
pub type SYSCTL_DC5_PWM4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM5` reader - PWM5 Pin Present"]
pub type SYSCTL_DC5_PWM5_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM5` writer - PWM5 Pin Present"]
pub type SYSCTL_DC5_PWM5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM6` reader - PWM6 Pin Present"]
pub type SYSCTL_DC5_PWM6_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM6` writer - PWM6 Pin Present"]
pub type SYSCTL_DC5_PWM6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWM7` reader - PWM7 Pin Present"]
pub type SYSCTL_DC5_PWM7_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWM7` writer - PWM7 Pin Present"]
pub type SYSCTL_DC5_PWM7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWMESYNC` reader - PWM Extended SYNC Active"]
pub type SYSCTL_DC5_PWMESYNC_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWMESYNC` writer - PWM Extended SYNC Active"]
pub type SYSCTL_DC5_PWMESYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWMEFLT` reader - PWM Extended Fault Active"]
pub type SYSCTL_DC5_PWMEFLT_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWMEFLT` writer - PWM Extended Fault Active"]
pub type SYSCTL_DC5_PWMEFLT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWMFAULT0` reader - PWM Fault 0 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWMFAULT0` writer - PWM Fault 0 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWMFAULT1` reader - PWM Fault 1 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWMFAULT1` writer - PWM Fault 1 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWMFAULT2` reader - PWM Fault 2 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT2_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWMFAULT2` writer - PWM Fault 2 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC5_PWMFAULT3` reader - PWM Fault 3 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT3_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC5_PWMFAULT3` writer - PWM Fault 3 Pin Present"]
pub type SYSCTL_DC5_PWMFAULT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PWM0 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm0(&self) -> SYSCTL_DC5_PWM0_R {
        SYSCTL_DC5_PWM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PWM1 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm1(&self) -> SYSCTL_DC5_PWM1_R {
        SYSCTL_DC5_PWM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PWM2 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm2(&self) -> SYSCTL_DC5_PWM2_R {
        SYSCTL_DC5_PWM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM3 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm3(&self) -> SYSCTL_DC5_PWM3_R {
        SYSCTL_DC5_PWM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM4 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm4(&self) -> SYSCTL_DC5_PWM4_R {
        SYSCTL_DC5_PWM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM5 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm5(&self) -> SYSCTL_DC5_PWM5_R {
        SYSCTL_DC5_PWM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM6 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm6(&self) -> SYSCTL_DC5_PWM6_R {
        SYSCTL_DC5_PWM6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM7 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwm7(&self) -> SYSCTL_DC5_PWM7_R {
        SYSCTL_DC5_PWM7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM Extended SYNC Active"]
    #[inline(always)]
    pub fn sysctl_dc5_pwmesync(&self) -> SYSCTL_DC5_PWMESYNC_R {
        SYSCTL_DC5_PWMESYNC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM Extended Fault Active"]
    #[inline(always)]
    pub fn sysctl_dc5_pwmeflt(&self) -> SYSCTL_DC5_PWMEFLT_R {
        SYSCTL_DC5_PWMEFLT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - PWM Fault 0 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwmfault0(&self) -> SYSCTL_DC5_PWMFAULT0_R {
        SYSCTL_DC5_PWMFAULT0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PWM Fault 1 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwmfault1(&self) -> SYSCTL_DC5_PWMFAULT1_R {
        SYSCTL_DC5_PWMFAULT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PWM Fault 2 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwmfault2(&self) -> SYSCTL_DC5_PWMFAULT2_R {
        SYSCTL_DC5_PWMFAULT2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PWM Fault 3 Pin Present"]
    #[inline(always)]
    pub fn sysctl_dc5_pwmfault3(&self) -> SYSCTL_DC5_PWMFAULT3_R {
        SYSCTL_DC5_PWMFAULT3_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PWM0 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm0(&mut self) -> SYSCTL_DC5_PWM0_W<DC5_SPEC, 0> {
        SYSCTL_DC5_PWM0_W::new(self)
    }
    #[doc = "Bit 1 - PWM1 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm1(&mut self) -> SYSCTL_DC5_PWM1_W<DC5_SPEC, 1> {
        SYSCTL_DC5_PWM1_W::new(self)
    }
    #[doc = "Bit 2 - PWM2 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm2(&mut self) -> SYSCTL_DC5_PWM2_W<DC5_SPEC, 2> {
        SYSCTL_DC5_PWM2_W::new(self)
    }
    #[doc = "Bit 3 - PWM3 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm3(&mut self) -> SYSCTL_DC5_PWM3_W<DC5_SPEC, 3> {
        SYSCTL_DC5_PWM3_W::new(self)
    }
    #[doc = "Bit 4 - PWM4 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm4(&mut self) -> SYSCTL_DC5_PWM4_W<DC5_SPEC, 4> {
        SYSCTL_DC5_PWM4_W::new(self)
    }
    #[doc = "Bit 5 - PWM5 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm5(&mut self) -> SYSCTL_DC5_PWM5_W<DC5_SPEC, 5> {
        SYSCTL_DC5_PWM5_W::new(self)
    }
    #[doc = "Bit 6 - PWM6 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm6(&mut self) -> SYSCTL_DC5_PWM6_W<DC5_SPEC, 6> {
        SYSCTL_DC5_PWM6_W::new(self)
    }
    #[doc = "Bit 7 - PWM7 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwm7(&mut self) -> SYSCTL_DC5_PWM7_W<DC5_SPEC, 7> {
        SYSCTL_DC5_PWM7_W::new(self)
    }
    #[doc = "Bit 20 - PWM Extended SYNC Active"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwmesync(&mut self) -> SYSCTL_DC5_PWMESYNC_W<DC5_SPEC, 20> {
        SYSCTL_DC5_PWMESYNC_W::new(self)
    }
    #[doc = "Bit 21 - PWM Extended Fault Active"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwmeflt(&mut self) -> SYSCTL_DC5_PWMEFLT_W<DC5_SPEC, 21> {
        SYSCTL_DC5_PWMEFLT_W::new(self)
    }
    #[doc = "Bit 24 - PWM Fault 0 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwmfault0(&mut self) -> SYSCTL_DC5_PWMFAULT0_W<DC5_SPEC, 24> {
        SYSCTL_DC5_PWMFAULT0_W::new(self)
    }
    #[doc = "Bit 25 - PWM Fault 1 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwmfault1(&mut self) -> SYSCTL_DC5_PWMFAULT1_W<DC5_SPEC, 25> {
        SYSCTL_DC5_PWMFAULT1_W::new(self)
    }
    #[doc = "Bit 26 - PWM Fault 2 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwmfault2(&mut self) -> SYSCTL_DC5_PWMFAULT2_W<DC5_SPEC, 26> {
        SYSCTL_DC5_PWMFAULT2_W::new(self)
    }
    #[doc = "Bit 27 - PWM Fault 3 Pin Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc5_pwmfault3(&mut self) -> SYSCTL_DC5_PWMFAULT3_W<DC5_SPEC, 27> {
        SYSCTL_DC5_PWMFAULT3_W::new(self)
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
#[doc = "Device Capabilities 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC5_SPEC;
impl crate::RegisterSpec for DC5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc5::R`](R) reader structure"]
impl crate::Readable for DC5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc5::W`](W) writer structure"]
impl crate::Writable for DC5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC5 to value 0"]
impl crate::Resettable for DC5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
