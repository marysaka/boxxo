#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MIS_SPEC>;
#[doc = "Field `SYSEXC_MIS_FPIDCMIS` reader - Floating-Point Input Denormal Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIDCMIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_MIS_FPIDCMIS` writer - Floating-Point Input Denormal Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIDCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_MIS_FPDZCMIS` reader - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPDZCMIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_MIS_FPDZCMIS` writer - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPDZCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_MIS_FPIOCMIS` reader - Floating-Point Invalid Operation Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIOCMIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_MIS_FPIOCMIS` writer - Floating-Point Invalid Operation Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIOCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_MIS_FPUFCMIS` reader - Floating-Point Underflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPUFCMIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_MIS_FPUFCMIS` writer - Floating-Point Underflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPUFCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_MIS_FPOFCMIS` reader - Floating-Point Overflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPOFCMIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_MIS_FPOFCMIS` writer - Floating-Point Overflow Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPOFCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_MIS_FPIXCMIS` reader - Floating-Point Inexact Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIXCMIS_R = crate::BitReader;
#[doc = "Field `SYSEXC_MIS_FPIXCMIS` writer - Floating-Point Inexact Exception Masked Interrupt Status"]
pub type SYSEXC_MIS_FPIXCMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpidcmis(&self) -> SYSEXC_MIS_FPIDCMIS_R {
        SYSEXC_MIS_FPIDCMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpdzcmis(&self) -> SYSEXC_MIS_FPDZCMIS_R {
        SYSEXC_MIS_FPDZCMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpiocmis(&self) -> SYSEXC_MIS_FPIOCMIS_R {
        SYSEXC_MIS_FPIOCMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpufcmis(&self) -> SYSEXC_MIS_FPUFCMIS_R {
        SYSEXC_MIS_FPUFCMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpofcmis(&self) -> SYSEXC_MIS_FPOFCMIS_R {
        SYSEXC_MIS_FPOFCMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysexc_mis_fpixcmis(&self) -> SYSEXC_MIS_FPIXCMIS_R {
        SYSEXC_MIS_FPIXCMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_mis_fpidcmis(&mut self) -> SYSEXC_MIS_FPIDCMIS_W<MIS_SPEC, 0> {
        SYSEXC_MIS_FPIDCMIS_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_mis_fpdzcmis(&mut self) -> SYSEXC_MIS_FPDZCMIS_W<MIS_SPEC, 1> {
        SYSEXC_MIS_FPDZCMIS_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_mis_fpiocmis(&mut self) -> SYSEXC_MIS_FPIOCMIS_W<MIS_SPEC, 2> {
        SYSEXC_MIS_FPIOCMIS_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_mis_fpufcmis(&mut self) -> SYSEXC_MIS_FPUFCMIS_W<MIS_SPEC, 3> {
        SYSEXC_MIS_FPUFCMIS_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_mis_fpofcmis(&mut self) -> SYSEXC_MIS_FPOFCMIS_W<MIS_SPEC, 4> {
        SYSEXC_MIS_FPOFCMIS_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_mis_fpixcmis(&mut self) -> SYSEXC_MIS_FPIXCMIS_W<MIS_SPEC, 5> {
        SYSEXC_MIS_FPIXCMIS_W::new(self)
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
#[doc = "System Exception Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
