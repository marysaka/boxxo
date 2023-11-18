#[doc = "Register `DCGCUSB` reader"]
pub type R = crate::R<DCGCUSB_SPEC>;
#[doc = "Register `DCGCUSB` writer"]
pub type W = crate::W<DCGCUSB_SPEC>;
#[doc = "Field `SYSCTL_DCGCUSB_D0` reader - USB Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUSB_D0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGCUSB_D0` writer - USB Module Deep-Sleep Mode Clock Gating Control"]
pub type SYSCTL_DCGCUSB_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcusb_d0(&self) -> SYSCTL_DCGCUSB_D0_R {
        SYSCTL_DCGCUSB_D0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgcusb_d0(&mut self) -> SYSCTL_DCGCUSB_D0_W<DCGCUSB_SPEC, 0> {
        SYSCTL_DCGCUSB_D0_W::new(self)
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
#[doc = "Universal Serial Bus Deep-Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgcusb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgcusb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCUSB_SPEC;
impl crate::RegisterSpec for DCGCUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgcusb::R`](R) reader structure"]
impl crate::Readable for DCGCUSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgcusb::W`](W) writer structure"]
impl crate::Writable for DCGCUSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCUSB to value 0"]
impl crate::Resettable for DCGCUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
