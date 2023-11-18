#[doc = "Register `_2_MINFLTPER` reader"]
pub type R = crate::R<_2_MINFLTPER_SPEC>;
#[doc = "Register `_2_MINFLTPER` writer"]
pub type W = crate::W<_2_MINFLTPER_SPEC>;
#[doc = "Field `PWM_2_MINFLTPER_MFP` reader - Minimum Fault Period"]
pub type PWM_2_MINFLTPER_MFP_R = crate::FieldReader<u16>;
#[doc = "Field `PWM_2_MINFLTPER_MFP` writer - Minimum Fault Period"]
pub type PWM_2_MINFLTPER_MFP_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_2_minfltper_mfp(&self) -> PWM_2_MINFLTPER_MFP_R {
        PWM_2_MINFLTPER_MFP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum Fault Period"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_minfltper_mfp(&mut self) -> PWM_2_MINFLTPER_MFP_W<_2_MINFLTPER_SPEC, 0> {
        PWM_2_MINFLTPER_MFP_W::new(self)
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
#[doc = "PWM2 Minimum Fault Period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_minfltper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_minfltper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2_MINFLTPER_SPEC;
impl crate::RegisterSpec for _2_MINFLTPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2_minfltper::R`](R) reader structure"]
impl crate::Readable for _2_MINFLTPER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_2_minfltper::W`](W) writer structure"]
impl crate::Writable for _2_MINFLTPER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _2_MINFLTPER to value 0"]
impl crate::Resettable for _2_MINFLTPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
