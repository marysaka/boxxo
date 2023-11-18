#[doc = "Register `SRHIB` reader"]
pub type R = crate::R<SRHIB_SPEC>;
#[doc = "Register `SRHIB` writer"]
pub type W = crate::W<SRHIB_SPEC>;
#[doc = "Field `SYSCTL_SRHIB_R0` reader - Hibernation Module Software Reset"]
pub type SYSCTL_SRHIB_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRHIB_R0` writer - Hibernation Module Software Reset"]
pub type SYSCTL_SRHIB_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Hibernation Module Software Reset"]
    #[inline(always)]
    pub fn sysctl_srhib_r0(&self) -> SYSCTL_SRHIB_R0_R {
        SYSCTL_SRHIB_R0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hibernation Module Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srhib_r0(&mut self) -> SYSCTL_SRHIB_R0_W<SRHIB_SPEC, 0> {
        SYSCTL_SRHIB_R0_W::new(self)
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
#[doc = "Hibernation Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srhib::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srhib::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRHIB_SPEC;
impl crate::RegisterSpec for SRHIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srhib::R`](R) reader structure"]
impl crate::Readable for SRHIB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srhib::W`](W) writer structure"]
impl crate::Writable for SRHIB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRHIB to value 0"]
impl crate::Resettable for SRHIB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
