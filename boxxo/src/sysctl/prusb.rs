#[doc = "Register `PRUSB` reader"]
pub type R = crate::R<PRUSB_SPEC>;
#[doc = "Register `PRUSB` writer"]
pub type W = crate::W<PRUSB_SPEC>;
#[doc = "Field `SYSCTL_PRUSB_R0` reader - USB Module Peripheral Ready"]
pub type SYSCTL_PRUSB_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PRUSB_R0` writer - USB Module Peripheral Ready"]
pub type SYSCTL_PRUSB_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Module Peripheral Ready"]
    #[inline(always)]
    pub fn sysctl_prusb_r0(&self) -> SYSCTL_PRUSB_R0_R {
        SYSCTL_PRUSB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Peripheral Ready"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_prusb_r0(&mut self) -> SYSCTL_PRUSB_R0_W<PRUSB_SPEC, 0> {
        SYSCTL_PRUSB_R0_W::new(self)
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
#[doc = "Universal Serial Bus Peripheral Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prusb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prusb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRUSB_SPEC;
impl crate::RegisterSpec for PRUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prusb::R`](R) reader structure"]
impl crate::Readable for PRUSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prusb::W`](W) writer structure"]
impl crate::Writable for PRUSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRUSB to value 0"]
impl crate::Resettable for PRUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
