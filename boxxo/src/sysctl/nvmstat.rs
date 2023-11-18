#[doc = "Register `NVMSTAT` reader"]
pub type R = crate::R<NVMSTAT_SPEC>;
#[doc = "Register `NVMSTAT` writer"]
pub type W = crate::W<NVMSTAT_SPEC>;
#[doc = "Field `SYSCTL_NVMSTAT_FWB` reader - 32 Word Flash Write Buffer Active"]
pub type SYSCTL_NVMSTAT_FWB_R = crate::BitReader;
#[doc = "Field `SYSCTL_NVMSTAT_FWB` writer - 32 Word Flash Write Buffer Active"]
pub type SYSCTL_NVMSTAT_FWB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 32 Word Flash Write Buffer Active"]
    #[inline(always)]
    pub fn sysctl_nvmstat_fwb(&self) -> SYSCTL_NVMSTAT_FWB_R {
        SYSCTL_NVMSTAT_FWB_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 32 Word Flash Write Buffer Active"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_nvmstat_fwb(&mut self) -> SYSCTL_NVMSTAT_FWB_W<NVMSTAT_SPEC, 0> {
        SYSCTL_NVMSTAT_FWB_W::new(self)
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
#[doc = "Non-Volatile Memory Information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvmstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvmstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NVMSTAT_SPEC;
impl crate::RegisterSpec for NVMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvmstat::R`](R) reader structure"]
impl crate::Readable for NVMSTAT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nvmstat::W`](W) writer structure"]
impl crate::Writable for NVMSTAT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVMSTAT to value 0"]
impl crate::Resettable for NVMSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
