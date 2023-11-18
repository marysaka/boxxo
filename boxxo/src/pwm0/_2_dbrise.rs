#[doc = "Register `_2_DBRISE` reader"]
pub type R = crate::R<_2_DBRISE_SPEC>;
#[doc = "Register `_2_DBRISE` writer"]
pub type W = crate::W<_2_DBRISE_SPEC>;
#[doc = "Field `PWM_2_DBRISE_RISEDELAY` reader - Dead-Band Rise Delay"]
pub type PWM_2_DBRISE_RISEDELAY_R = crate::FieldReader<u16>;
#[doc = "Field `PWM_2_DBRISE_RISEDELAY` writer - Dead-Band Rise Delay"]
pub type PWM_2_DBRISE_RISEDELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Dead-Band Rise Delay"]
    #[inline(always)]
    pub fn pwm_2_dbrise_risedelay(&self) -> PWM_2_DBRISE_RISEDELAY_R {
        PWM_2_DBRISE_RISEDELAY_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Dead-Band Rise Delay"]
    #[inline(always)]
    #[must_use]
    pub fn pwm_2_dbrise_risedelay(&mut self) -> PWM_2_DBRISE_RISEDELAY_W<_2_DBRISE_SPEC, 0> {
        PWM_2_DBRISE_RISEDELAY_W::new(self)
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
#[doc = "PWM2 Dead-Band Rising-Edge Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_2_dbrise::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_2_dbrise::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _2_DBRISE_SPEC;
impl crate::RegisterSpec for _2_DBRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_2_dbrise::R`](R) reader structure"]
impl crate::Readable for _2_DBRISE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_2_dbrise::W`](W) writer structure"]
impl crate::Writable for _2_DBRISE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _2_DBRISE to value 0"]
impl crate::Resettable for _2_DBRISE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
