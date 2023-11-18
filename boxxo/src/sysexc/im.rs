#[doc = "Register `IM` reader"]
pub type R = crate::R<IM_SPEC>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<IM_SPEC>;
#[doc = "Field `SYSEXC_IM_FPIDCIM` reader - Floating-Point Input Denormal Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIDCIM_R = crate::BitReader;
#[doc = "Field `SYSEXC_IM_FPIDCIM` writer - Floating-Point Input Denormal Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIDCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IM_FPDZCIM` reader - Floating-Point Divide By 0 Exception Interrupt Mask"]
pub type SYSEXC_IM_FPDZCIM_R = crate::BitReader;
#[doc = "Field `SYSEXC_IM_FPDZCIM` writer - Floating-Point Divide By 0 Exception Interrupt Mask"]
pub type SYSEXC_IM_FPDZCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IM_FPIOCIM` reader - Floating-Point Invalid Operation Interrupt Mask"]
pub type SYSEXC_IM_FPIOCIM_R = crate::BitReader;
#[doc = "Field `SYSEXC_IM_FPIOCIM` writer - Floating-Point Invalid Operation Interrupt Mask"]
pub type SYSEXC_IM_FPIOCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IM_FPUFCIM` reader - Floating-Point Underflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPUFCIM_R = crate::BitReader;
#[doc = "Field `SYSEXC_IM_FPUFCIM` writer - Floating-Point Underflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPUFCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IM_FPOFCIM` reader - Floating-Point Overflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPOFCIM_R = crate::BitReader;
#[doc = "Field `SYSEXC_IM_FPOFCIM` writer - Floating-Point Overflow Exception Interrupt Mask"]
pub type SYSEXC_IM_FPOFCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSEXC_IM_FPIXCIM` reader - Floating-Point Inexact Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIXCIM_R = crate::BitReader;
#[doc = "Field `SYSEXC_IM_FPIXCIM` writer - Floating-Point Inexact Exception Interrupt Mask"]
pub type SYSEXC_IM_FPIXCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpidcim(&self) -> SYSEXC_IM_FPIDCIM_R {
        SYSEXC_IM_FPIDCIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpdzcim(&self) -> SYSEXC_IM_FPDZCIM_R {
        SYSEXC_IM_FPDZCIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpiocim(&self) -> SYSEXC_IM_FPIOCIM_R {
        SYSEXC_IM_FPIOCIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpufcim(&self) -> SYSEXC_IM_FPUFCIM_R {
        SYSEXC_IM_FPUFCIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpofcim(&self) -> SYSEXC_IM_FPOFCIM_R {
        SYSEXC_IM_FPOFCIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    pub fn sysexc_im_fpixcim(&self) -> SYSEXC_IM_FPIXCIM_R {
        SYSEXC_IM_FPIXCIM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Floating-Point Input Denormal Exception Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_im_fpidcim(&mut self) -> SYSEXC_IM_FPIDCIM_W<IM_SPEC, 0> {
        SYSEXC_IM_FPIDCIM_W::new(self)
    }
    #[doc = "Bit 1 - Floating-Point Divide By 0 Exception Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_im_fpdzcim(&mut self) -> SYSEXC_IM_FPDZCIM_W<IM_SPEC, 1> {
        SYSEXC_IM_FPDZCIM_W::new(self)
    }
    #[doc = "Bit 2 - Floating-Point Invalid Operation Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_im_fpiocim(&mut self) -> SYSEXC_IM_FPIOCIM_W<IM_SPEC, 2> {
        SYSEXC_IM_FPIOCIM_W::new(self)
    }
    #[doc = "Bit 3 - Floating-Point Underflow Exception Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_im_fpufcim(&mut self) -> SYSEXC_IM_FPUFCIM_W<IM_SPEC, 3> {
        SYSEXC_IM_FPUFCIM_W::new(self)
    }
    #[doc = "Bit 4 - Floating-Point Overflow Exception Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_im_fpofcim(&mut self) -> SYSEXC_IM_FPOFCIM_W<IM_SPEC, 4> {
        SYSEXC_IM_FPOFCIM_W::new(self)
    }
    #[doc = "Bit 5 - Floating-Point Inexact Exception Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysexc_im_fpixcim(&mut self) -> SYSEXC_IM_FPIXCIM_W<IM_SPEC, 5> {
        SYSEXC_IM_FPIXCIM_W::new(self)
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
#[doc = "System Exception Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for IM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for IM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
