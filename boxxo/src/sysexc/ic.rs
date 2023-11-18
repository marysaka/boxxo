#[doc = "Register `IC` writer"]
pub type W = crate::W<IC_SPEC>;
#[doc = "Field `SYSEXC_IC_FPIDCIC` writer - Floating-Point Input Denormal Exception Interrupt Clear"]
pub type SYSEXC_IC_FPIDCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IC_FPDZCIC` writer - Floating-Point Divide By 0 Exception Interrupt Clear"]
pub type SYSEXC_IC_FPDZCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IC_FPIOCIC` writer - Floating-Point Invalid Operation Interrupt Clear"]
pub type SYSEXC_IC_FPIOCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IC_FPUFCIC` writer - Floating-Point Underflow Exception Interrupt Clear"]
pub type SYSEXC_IC_FPUFCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IC_FPOFCIC` writer - Floating-Point Overflow Exception Interrupt Clear"]
pub type SYSEXC_IC_FPOFCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IC_FPIXCIC` writer - Floating-Point Inexact Exception Interrupt Clear"]
pub type SYSEXC_IC_FPIXCIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ic_fpidcic(&mut self) -> SYSEXC_IC_FPIDCIC_W<IC_SPEC, 0> {
        SYSEXC_IC_FPIDCIC_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ic_fpdzcic(&mut self) -> SYSEXC_IC_FPDZCIC_W<IC_SPEC, 1> {
        SYSEXC_IC_FPDZCIC_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ic_fpiocic(&mut self) -> SYSEXC_IC_FPIOCIC_W<IC_SPEC, 2> {
        SYSEXC_IC_FPIOCIC_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ic_fpufcic(&mut self) -> SYSEXC_IC_FPUFCIC_W<IC_SPEC, 3> {
        SYSEXC_IC_FPUFCIC_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ic_fpofcic(&mut self) -> SYSEXC_IC_FPOFCIC_W<IC_SPEC, 4> {
        SYSEXC_IC_FPOFCIC_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_ic_fpixcic(&mut self) -> SYSEXC_IC_FPIXCIC_W<IC_SPEC, 5> {
        SYSEXC_IC_FPIXCIC_W::new(self)
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
#[doc = "System Exception Interrupt Clear\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ic::W`](W) writer structure"]
impl crate::Writable for IC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
