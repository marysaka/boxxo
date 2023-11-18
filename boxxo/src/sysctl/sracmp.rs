#[doc = "Register `SRACMP` reader"]
pub type R = crate::R<SRACMP_SPEC>;
#[doc = "Register `SRACMP` writer"]
pub type W = crate::W<SRACMP_SPEC>;
#[doc = "Field `SYSCTL_SRACMP_R0` reader - Analog Comparator Module 0 Software Reset"]
pub type SYSCTL_SRACMP_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRACMP_R0` writer - Analog Comparator Module 0 Software Reset"]
pub type SYSCTL_SRACMP_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sracmp_r0(&self) -> SYSCTL_SRACMP_R0_R {
        SYSCTL_SRACMP_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator Module 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_sracmp_r0(&mut self) -> SYSCTL_SRACMP_R0_W<SRACMP_SPEC, 0> {
        SYSCTL_SRACMP_R0_W::new(self)
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
#[doc = "Analog Comparator Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sracmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sracmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRACMP_SPEC;
impl crate::RegisterSpec for SRACMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sracmp::R`](R) reader structure"]
impl crate::Readable for SRACMP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sracmp::W`](W) writer structure"]
impl crate::Writable for SRACMP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRACMP to value 0"]
impl crate::Resettable for SRACMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
