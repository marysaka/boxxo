#[doc = "Register `SCGCUSB` reader"]
pub type R = crate::R<SCGCUSB_SPEC>;
#[doc = "Register `SCGCUSB` writer"]
pub type W = crate::W<SCGCUSB_SPEC>;
#[doc = "Field `SYSCTL_SCGCUSB_S0` reader - USB Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUSB_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCUSB_S0` writer - USB Module Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCUSB_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcusb_s0(&self) -> SYSCTL_SCGCUSB_S0_R {
        SYSCTL_SCGCUSB_S0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcusb_s0(&mut self) -> SYSCTL_SCGCUSB_S0_W<SCGCUSB_SPEC, 0> {
        SYSCTL_SCGCUSB_S0_W::new(self)
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
#[doc = "Universal Serial Bus Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcusb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcusb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCUSB_SPEC;
impl crate::RegisterSpec for SCGCUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcusb::R`](R) reader structure"]
impl crate::Readable for SCGCUSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcusb::W`](W) writer structure"]
impl crate::Writable for SCGCUSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCUSB to value 0"]
impl crate::Resettable for SCGCUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
