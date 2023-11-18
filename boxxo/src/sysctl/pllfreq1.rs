#[doc = "Register `PLLFREQ1` reader"]
pub type R = crate::R<PLLFREQ1_SPEC>;
#[doc = "Register `PLLFREQ1` writer"]
pub type W = crate::W<PLLFREQ1_SPEC>;
#[doc = "Field `SYSCTL_PLLFREQ1_N` reader - PLL N Value"]
pub type SYSCTL_PLLFREQ1_N_R = crate::FieldReader;
#[doc = "Field `SYSCTL_PLLFREQ1_N` writer - PLL N Value"]
pub type SYSCTL_PLLFREQ1_N_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SYSCTL_PLLFREQ1_Q` reader - PLL Q Value"]
pub type SYSCTL_PLLFREQ1_Q_R = crate::FieldReader;
#[doc = "Field `SYSCTL_PLLFREQ1_Q` writer - PLL Q Value"]
pub type SYSCTL_PLLFREQ1_Q_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_n(&self) -> SYSCTL_PLLFREQ1_N_R {
        SYSCTL_PLLFREQ1_N_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_q(&self) -> SYSCTL_PLLFREQ1_Q_R {
        SYSCTL_PLLFREQ1_Q_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pllfreq1_n(&mut self) -> SYSCTL_PLLFREQ1_N_W<PLLFREQ1_SPEC, 0> {
        SYSCTL_PLLFREQ1_N_W::new(self)
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pllfreq1_q(&mut self) -> SYSCTL_PLLFREQ1_Q_W<PLLFREQ1_SPEC, 8> {
        SYSCTL_PLLFREQ1_Q_W::new(self)
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
#[doc = "PLL Frequency 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllfreq1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllfreq1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLFREQ1_SPEC;
impl crate::RegisterSpec for PLLFREQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllfreq1::R`](R) reader structure"]
impl crate::Readable for PLLFREQ1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllfreq1::W`](W) writer structure"]
impl crate::Writable for PLLFREQ1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLFREQ1 to value 0"]
impl crate::Resettable for PLLFREQ1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
