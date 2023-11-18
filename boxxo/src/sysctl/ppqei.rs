#[doc = "Register `PPQEI` reader"]
pub type R = crate::R<PPQEI_SPEC>;
#[doc = "Register `PPQEI` writer"]
pub type W = crate::W<PPQEI_SPEC>;
#[doc = "Field `SYSCTL_PPQEI_P0` reader - QEI Module 0 Present"]
pub type SYSCTL_PPQEI_P0_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPQEI_P0` writer - QEI Module 0 Present"]
pub type SYSCTL_PPQEI_P0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PPQEI_P1` reader - QEI Module 1 Present"]
pub type SYSCTL_PPQEI_P1_R = crate::BitReader;
#[doc = "Field `SYSCTL_PPQEI_P1` writer - QEI Module 1 Present"]
pub type SYSCTL_PPQEI_P1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - QEI Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppqei_p0(&self) -> SYSCTL_PPQEI_P0_R {
        SYSCTL_PPQEI_P0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QEI Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppqei_p1(&self) -> SYSCTL_PPQEI_P1_R {
        SYSCTL_PPQEI_P1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - QEI Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppqei_p0(&mut self) -> SYSCTL_PPQEI_P0_W<PPQEI_SPEC, 0> {
        SYSCTL_PPQEI_P0_W::new(self)
    }
    #[doc = "Bit 1 - QEI Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ppqei_p1(&mut self) -> SYSCTL_PPQEI_P1_W<PPQEI_SPEC, 1> {
        SYSCTL_PPQEI_P1_W::new(self)
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
#[doc = "Quadrature Encoder Interface Peripheral Present\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppqei::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppqei::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPQEI_SPEC;
impl crate::RegisterSpec for PPQEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppqei::R`](R) reader structure"]
impl crate::Readable for PPQEI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppqei::W`](W) writer structure"]
impl crate::Writable for PPQEI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPQEI to value 0"]
impl crate::Resettable for PPQEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
