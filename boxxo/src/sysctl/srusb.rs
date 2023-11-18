#[doc = "Register `SRUSB` reader"]
pub type R = crate::R<SRUSB_SPEC>;
#[doc = "Register `SRUSB` writer"]
pub type W = crate::W<SRUSB_SPEC>;
#[doc = "Field `SYSCTL_SRUSB_R0` reader - USB Module Software Reset"]
pub type SYSCTL_SRUSB_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRUSB_R0` writer - USB Module Software Reset"]
pub type SYSCTL_SRUSB_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - USB Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srusb_r0(&self) -> SYSCTL_SRUSB_R0_R {
        SYSCTL_SRUSB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Module Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srusb_r0(&mut self) -> SYSCTL_SRUSB_R0_W<SRUSB_SPEC, 0> {
        SYSCTL_SRUSB_R0_W::new(self)
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
#[doc = "Universal Serial Bus Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srusb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srusb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRUSB_SPEC;
impl crate::RegisterSpec for SRUSB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srusb::R`](R) reader structure"]
impl crate::Readable for SRUSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srusb::W`](W) writer structure"]
impl crate::Writable for SRUSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRUSB to value 0"]
impl crate::Resettable for SRUSB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
