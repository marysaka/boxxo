#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `SYSEXC_RIS_FPIDCRIS` reader - Floating-Point Input Denormal Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIDCRIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_RIS_FPIDCRIS` writer - Floating-Point Input Denormal Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIDCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_RIS_FPDZCRIS` reader - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPDZCRIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_RIS_FPDZCRIS` writer - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPDZCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_RIS_FPIOCRIS` reader - Floating-Point Invalid Operation Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIOCRIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_RIS_FPIOCRIS` writer - Floating-Point Invalid Operation Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIOCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_RIS_FPUFCRIS` reader - Floating-Point Underflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPUFCRIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_RIS_FPUFCRIS` writer - Floating-Point Underflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPUFCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_RIS_FPOFCRIS` reader - Floating-Point Overflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPOFCRIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_RIS_FPOFCRIS` writer - Floating-Point Overflow Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPOFCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_RIS_FPIXCRIS` reader - Floating-Point Inexact Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIXCRIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_RIS_FPIXCRIS` writer - Floating-Point Inexact Exception Raw Interrupt Status"]
pub type SYSEXC_RIS_FPIXCRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpidcris(&self) -> SYSEXC_RIS_FPIDCRIS_R {
        SYSEXC_RIS_FPIDCRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpdzcris(&self) -> SYSEXC_RIS_FPDZCRIS_R {
        SYSEXC_RIS_FPDZCRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpiocris(&self) -> SYSEXC_RIS_FPIOCRIS_R {
        SYSEXC_RIS_FPIOCRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpufcris(&self) -> SYSEXC_RIS_FPUFCRIS_R {
        SYSEXC_RIS_FPUFCRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpofcris(&self) -> SYSEXC_RIS_FPOFCRIS_R {
        SYSEXC_RIS_FPOFCRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_ris_fpixcris(&self) -> SYSEXC_RIS_FPIXCRIS_R {
        SYSEXC_RIS_FPIXCRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ris_fpidcris(&mut self) -> SYSEXC_RIS_FPIDCRIS_W<RIS_SPEC, 0> {
        SYSEXC_RIS_FPIDCRIS_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ris_fpdzcris(&mut self) -> SYSEXC_RIS_FPDZCRIS_W<RIS_SPEC, 1> {
        SYSEXC_RIS_FPDZCRIS_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ris_fpiocris(&mut self) -> SYSEXC_RIS_FPIOCRIS_W<RIS_SPEC, 2> {
        SYSEXC_RIS_FPIOCRIS_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ris_fpufcris(&mut self) -> SYSEXC_RIS_FPUFCRIS_W<RIS_SPEC, 3> {
        SYSEXC_RIS_FPUFCRIS_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ris_fpofcris(&mut self) -> SYSEXC_RIS_FPOFCRIS_W<RIS_SPEC, 4> {
        SYSEXC_RIS_FPOFCRIS_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ris_fpixcris(&mut self) -> SYSEXC_RIS_FPIXCRIS_W<RIS_SPEC, 5> {
        SYSEXC_RIS_FPIXCRIS_W::new(self)
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
#[doc = "System Exception Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
