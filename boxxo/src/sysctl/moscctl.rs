#[doc = "Register `MOSCCTL` reader"]
pub type R = crate::R<MOSCCTL_SPEC>;
#[doc = "Register `MOSCCTL` writer"]
pub type W = crate::W<MOSCCTL_SPEC>;
#[doc = "Field `SYSCTL_MOSCCTL_CVAL` reader - Clock Validation for MOSC"]
pub type SYSCTL_MOSCCTL_CVAL_R = crate::BitReader;
#[doc = "Field `SYSCTL_MOSCCTL_CVAL` writer - Clock Validation for MOSC"]
pub type SYSCTL_MOSCCTL_CVAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_MOSCCTL_MOSCIM` reader - MOSC Failure Action"]
pub type SYSCTL_MOSCCTL_MOSCIM_R = crate::BitReader;
#[doc = "Field `SYSCTL_MOSCCTL_MOSCIM` writer - MOSC Failure Action"]
pub type SYSCTL_MOSCCTL_MOSCIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_MOSCCTL_NOXTAL` reader - No Crystal Connected"]
pub type SYSCTL_MOSCCTL_NOXTAL_R = crate::BitReader;
#[doc = "Field `SYSCTL_MOSCCTL_NOXTAL` writer - No Crystal Connected"]
pub type SYSCTL_MOSCCTL_NOXTAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    pub fn sysctl_moscctl_cval(&self) -> SYSCTL_MOSCCTL_CVAL_R {
        SYSCTL_MOSCCTL_CVAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    pub fn sysctl_moscctl_moscim(&self) -> SYSCTL_MOSCCTL_MOSCIM_R {
        SYSCTL_MOSCCTL_MOSCIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    pub fn sysctl_moscctl_noxtal(&self) -> SYSCTL_MOSCCTL_NOXTAL_R {
        SYSCTL_MOSCCTL_NOXTAL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Validation for MOSC"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_moscctl_cval(&mut self) -> SYSCTL_MOSCCTL_CVAL_W<MOSCCTL_SPEC, 0> {
        SYSCTL_MOSCCTL_CVAL_W::new(self)
    }
    #[doc = "Bit 1 - MOSC Failure Action"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_moscctl_moscim(&mut self) -> SYSCTL_MOSCCTL_MOSCIM_W<MOSCCTL_SPEC, 1> {
        SYSCTL_MOSCCTL_MOSCIM_W::new(self)
    }
    #[doc = "Bit 2 - No Crystal Connected"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_moscctl_noxtal(&mut self) -> SYSCTL_MOSCCTL_NOXTAL_W<MOSCCTL_SPEC, 2> {
        SYSCTL_MOSCCTL_NOXTAL_W::new(self)
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
#[doc = "Main Oscillator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`moscctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`moscctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MOSCCTL_SPEC;
impl crate::RegisterSpec for MOSCCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moscctl::R`](R) reader structure"]
impl crate::Readable for MOSCCTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`moscctl::W`](W) writer structure"]
impl crate::Writable for MOSCCTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOSCCTL to value 0"]
impl crate::Resettable for MOSCCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
