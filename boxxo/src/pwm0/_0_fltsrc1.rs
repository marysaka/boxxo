#[doc = "Register `_0_FLTSRC1` reader"]
pub type R = crate::R<_0_FLTSRC1_SPEC>;
#[doc = "Register `_0_FLTSRC1` writer"]
pub type W = crate::W<_0_FLTSRC1_SPEC>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP0` reader - Digital Comparator 0"]
pub type PWM_0_FLTSRC1_DCMP0_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP0` writer - Digital Comparator 0"]
pub type PWM_0_FLTSRC1_DCMP0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP1` reader - Digital Comparator 1"]
pub type PWM_0_FLTSRC1_DCMP1_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP1` writer - Digital Comparator 1"]
pub type PWM_0_FLTSRC1_DCMP1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP2` reader - Digital Comparator 2"]
pub type PWM_0_FLTSRC1_DCMP2_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP2` writer - Digital Comparator 2"]
pub type PWM_0_FLTSRC1_DCMP2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP3` reader - Digital Comparator 3"]
pub type PWM_0_FLTSRC1_DCMP3_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP3` writer - Digital Comparator 3"]
pub type PWM_0_FLTSRC1_DCMP3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP4` reader - Digital Comparator 4"]
pub type PWM_0_FLTSRC1_DCMP4_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP4` writer - Digital Comparator 4"]
pub type PWM_0_FLTSRC1_DCMP4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP5` reader - Digital Comparator 5"]
pub type PWM_0_FLTSRC1_DCMP5_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP5` writer - Digital Comparator 5"]
pub type PWM_0_FLTSRC1_DCMP5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP6` reader - Digital Comparator 6"]
pub type PWM_0_FLTSRC1_DCMP6_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP6` writer - Digital Comparator 6"]
pub type PWM_0_FLTSRC1_DCMP6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PWM_0_FLTSRC1_DCMP7` reader - Digital Comparator 7"]
pub type PWM_0_FLTSRC1_DCMP7_R = crate::BitReader;
#[doc = "Field `PWM_0_FLTSRC1_DCMP7` writer - Digital Comparator 7"]
pub type PWM_0_FLTSRC1_DCMP7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Digital Comparator 0"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp0(&self) -> PWM_0_FLTSRC1_DCMP0_R {
        PWM_0_FLTSRC1_DCMP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital Comparator 1"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp1(&self) -> PWM_0_FLTSRC1_DCMP1_R {
        PWM_0_FLTSRC1_DCMP1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Comparator 2"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp2(&self) -> PWM_0_FLTSRC1_DCMP2_R {
        PWM_0_FLTSRC1_DCMP2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Comparator 3"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp3(&self) -> PWM_0_FLTSRC1_DCMP3_R {
        PWM_0_FLTSRC1_DCMP3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Digital Comparator 4"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp4(&self) -> PWM_0_FLTSRC1_DCMP4_R {
        PWM_0_FLTSRC1_DCMP4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Comparator 5"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp5(&self) -> PWM_0_FLTSRC1_DCMP5_R {
        PWM_0_FLTSRC1_DCMP5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Digital Comparator 6"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp6(&self) -> PWM_0_FLTSRC1_DCMP6_R {
        PWM_0_FLTSRC1_DCMP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Digital Comparator 7"]
    #[inline(always)]
    pub fn pwm_0_fltsrc1_dcmp7(&self) -> PWM_0_FLTSRC1_DCMP7_R {
        PWM_0_FLTSRC1_DCMP7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp0(&mut self) -> PWM_0_FLTSRC1_DCMP0_W<_0_FLTSRC1_SPEC, 0> {
        PWM_0_FLTSRC1_DCMP0_W::new(self)
    }
    #[doc = "Bit 1 - Digital Comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp1(&mut self) -> PWM_0_FLTSRC1_DCMP1_W<_0_FLTSRC1_SPEC, 1> {
        PWM_0_FLTSRC1_DCMP1_W::new(self)
    }
    #[doc = "Bit 2 - Digital Comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp2(&mut self) -> PWM_0_FLTSRC1_DCMP2_W<_0_FLTSRC1_SPEC, 2> {
        PWM_0_FLTSRC1_DCMP2_W::new(self)
    }
    #[doc = "Bit 3 - Digital Comparator 3"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp3(&mut self) -> PWM_0_FLTSRC1_DCMP3_W<_0_FLTSRC1_SPEC, 3> {
        PWM_0_FLTSRC1_DCMP3_W::new(self)
    }
    #[doc = "Bit 4 - Digital Comparator 4"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp4(&mut self) -> PWM_0_FLTSRC1_DCMP4_W<_0_FLTSRC1_SPEC, 4> {
        PWM_0_FLTSRC1_DCMP4_W::new(self)
    }
    #[doc = "Bit 5 - Digital Comparator 5"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp5(&mut self) -> PWM_0_FLTSRC1_DCMP5_W<_0_FLTSRC1_SPEC, 5> {
        PWM_0_FLTSRC1_DCMP5_W::new(self)
    }
    #[doc = "Bit 6 - Digital Comparator 6"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp6(&mut self) -> PWM_0_FLTSRC1_DCMP6_W<_0_FLTSRC1_SPEC, 6> {
        PWM_0_FLTSRC1_DCMP6_W::new(self)
    }
    #[doc = "Bit 7 - Digital Comparator 7"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_0_fltsrc1_dcmp7(&mut self) -> PWM_0_FLTSRC1_DCMP7_W<_0_FLTSRC1_SPEC, 7> {
        PWM_0_FLTSRC1_DCMP7_W::new(self)
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
#[doc = "PWM0 Fault Source 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_fltsrc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_fltsrc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_FLTSRC1_SPEC;
impl crate::RegisterSpec for _0_FLTSRC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_fltsrc1::R`](R) reader structure"]
impl crate::Readable for _0_FLTSRC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0_fltsrc1::W`](W) writer structure"]
impl crate::Writable for _0_FLTSRC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0_FLTSRC1 to value 0"]
impl crate::Resettable for _0_FLTSRC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
