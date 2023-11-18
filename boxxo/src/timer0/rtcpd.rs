#[doc = "Register `RTCPD` reader"]
pub type R = crate::R<RTCPD_SPEC>;
#[doc = "Register `RTCPD` writer"]
pub type W = crate::W<RTCPD_SPEC>;
#[doc = "Field `TIMER_RTCPD_RTCPD` reader - RTC Predivide Counter Value"]
pub type TIMER_RTCPD_RTCPD_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_RTCPD_RTCPD` writer - RTC Predivide Counter Value"]
pub type TIMER_RTCPD_RTCPD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    pub fn timer_rtcpd_rtcpd(&self) -> TIMER_RTCPD_RTCPD_R {
        TIMER_RTCPD_RTCPD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Predivide Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn timer_rtcpd_rtcpd(&mut self) -> TIMER_RTCPD_RTCPD_W<RTCPD_SPEC, 0> {
        TIMER_RTCPD_RTCPD_W::new(self)
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
#[doc = "GPTM RTC Predivide\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcpd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcpd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTCPD_SPEC;
impl crate::RegisterSpec for RTCPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtcpd::R`](R) reader structure"]
impl crate::Readable for RTCPD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtcpd::W`](W) writer structure"]
impl crate::Writable for RTCPD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCPD to value 0"]
impl crate::Resettable for RTCPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
