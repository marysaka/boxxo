#[doc = "Register `PBORCTL` reader"]
pub type R = crate::R<PBORCTL_SPEC>;
#[doc = "Register `PBORCTL` writer"]
pub type W = crate::W<PBORCTL_SPEC>;
#[doc = "Field `SYSCTL_PBORCTL_BOR1` reader - VDD under BOR1 Event Action"]
pub type SYSCTL_PBORCTL_BOR1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PBORCTL_BOR1` writer - VDD under BOR1 Event Action"]
pub type SYSCTL_PBORCTL_BOR1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PBORCTL_BOR0` reader - VDD under BOR0 Event Action"]
pub type SYSCTL_PBORCTL_BOR0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PBORCTL_BOR0` writer - VDD under BOR0 Event Action"]
pub type SYSCTL_PBORCTL_BOR0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 1 - VDD under BOR1 Event Action"]
    #[inline(always)]
    pub fn sysctl_pborctl_bor1(&self) -> SYSCTL_PBORCTL_BOR1_R {
        SYSCTL_PBORCTL_BOR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDD under BOR0 Event Action"]
    #[inline(always)]
    pub fn sysctl_pborctl_bor0(&self) -> SYSCTL_PBORCTL_BOR0_R {
        SYSCTL_PBORCTL_BOR0_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VDD under BOR1 Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pborctl_bor1(&mut self) -> SYSCTL_PBORCTL_BOR1_W<PBORCTL_SPEC, 1> {
        SYSCTL_PBORCTL_BOR1_W::new(self)
    }
    #[doc = "Bit 2 - VDD under BOR0 Event Action"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pborctl_bor0(&mut self) -> SYSCTL_PBORCTL_BOR0_W<PBORCTL_SPEC, 2> {
        SYSCTL_PBORCTL_BOR0_W::new(self)
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
#[doc = "Brown-Out Reset Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pborctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pborctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PBORCTL_SPEC;
impl crate::RegisterSpec for PBORCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pborctl::R`](R) reader structure"]
impl crate::Readable for PBORCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pborctl::W`](W) writer structure"]
impl crate::Writable for PBORCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBORCTL to value 0"]
impl crate::Resettable for PBORCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
