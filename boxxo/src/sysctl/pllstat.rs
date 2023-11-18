#[doc = "Register `PLLSTAT` reader"]
pub type R = crate::R<PLLSTAT_SPEC>;
#[doc = "Register `PLLSTAT` writer"]
pub type W = crate::W<PLLSTAT_SPEC>;
#[doc = "Field `SYSCTL_PLLSTAT_LOCK` reader - PLL Lock"]
pub type SYSCTL_PLLSTAT_LOCK_R = crate::BitReader;
#[doc = "Field `SYSCTL_PLLSTAT_LOCK` writer - PLL Lock"]
pub type SYSCTL_PLLSTAT_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - PLL Lock"]
    #[inline(always)]
    pub fn sysctl_pllstat_lock(&self) -> SYSCTL_PLLSTAT_LOCK_R {
        SYSCTL_PLLSTAT_LOCK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Lock"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pllstat_lock(&mut self) -> SYSCTL_PLLSTAT_LOCK_W<PLLSTAT_SPEC, 0> {
        SYSCTL_PLLSTAT_LOCK_W::new(self)
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
#[doc = "PLL Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLSTAT_SPEC;
impl crate::RegisterSpec for PLLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstat::R`](R) reader structure"]
impl crate::Readable for PLLSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllstat::W`](W) writer structure"]
impl crate::Writable for PLLSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLSTAT to value 0"]
impl crate::Resettable for PLLSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
