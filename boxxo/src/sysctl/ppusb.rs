#[doc = "Register `PPUSB` reader"]
pub type R = crate::R<PPUSB_SPEC>;
#[doc = "Register `PPUSB` writer"]
pub type W = crate::W<PPUSB_SPEC>;
#[doc = "Field `SYSCTL_PPUSB_P0` reader - USB Module Present"]
pub type SYSCTL_PPUSB_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPUSB_P0` writer - USB Module Present"]
pub type SYSCTL_PPUSB_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Module Present"]
    #[inline(always)]
    pub fn sysctl_ppusb_p0(&self) -> SYSCTL_PPUSB_P0_R {
        SYSCTL_PPUSB_P0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppusb_p0(&mut self) -> SYSCTL_PPUSB_P0_W<PPUSB_SPEC, 0> {
        SYSCTL_PPUSB_P0_W::new(self)
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
#[doc = "Universal Serial Bus Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppusb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppusb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPUSB_SPEC;
impl crate::RegisterSpec for PPUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppusb::R`](R) reader structure"]
impl crate::Readable for PPUSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppusb::W`](W) writer structure"]
impl crate::Writable for PPUSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPUSB to value 0"]
impl crate::Resettable for PPUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
