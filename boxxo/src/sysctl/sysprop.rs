#[doc = "Register `SYSPROP` reader"]
pub type R = crate::R<SYSPROP_SPEC>;
#[doc = "Register `SYSPROP` writer"]
pub type W = crate::W<SYSPROP_SPEC>;
#[doc = "Field `SYSCTL_SYSPROP_FPU` reader - FPU Present"]
pub type SYSCTL_SYSPROP_FPU_R = crate::BitReader;
#[doc = "Field `SYSCTL_SYSPROP_FPU` writer - FPU Present"]
pub type SYSCTL_SYSPROP_FPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - FPU Present"]
    #[inline(always)]
    pub fn sysctl_sysprop_fpu(&self) -> SYSCTL_SYSPROP_FPU_R {
        SYSCTL_SYSPROP_FPU_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FPU Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_sysprop_fpu(&mut self) -> SYSCTL_SYSPROP_FPU_W<SYSPROP_SPEC, 0> {
        SYSCTL_SYSPROP_FPU_W::new(self)
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
#[doc = "System Properties\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysprop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysprop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSPROP_SPEC;
impl crate::RegisterSpec for SYSPROP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysprop::R`](R) reader structure"]
impl crate::Readable for SYSPROP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sysprop::W`](W) writer structure"]
impl crate::Writable for SYSPROP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSPROP to value 0"]
impl crate::Resettable for SYSPROP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
