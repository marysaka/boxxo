#[doc = "Register `PLLFREQ0` reader"]
pub type R = crate::R<PLLFREQ0_SPEC>;
#[doc = "Register `PLLFREQ0` writer"]
pub type W = crate::W<PLLFREQ0_SPEC>;
#[doc = "Field `SYSCTL_PLLFREQ0_MINT` reader - PLL M Integer Value"]
pub type SYSCTL_PLLFREQ0_MINT_R = crate::FieldReader<u16>;
#[doc = "Field `SYSCTL_PLLFREQ0_MINT` writer - PLL M Integer Value"]
pub type SYSCTL_PLLFREQ0_MINT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `SYSCTL_PLLFREQ0_MFRAC` reader - PLL M Fractional Value"]
pub type SYSCTL_PLLFREQ0_MFRAC_R = crate::FieldReader<u16>;
#[doc = "Field `SYSCTL_PLLFREQ0_MFRAC` writer - PLL M Fractional Value"]
pub type SYSCTL_PLLFREQ0_MFRAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mint(&self) -> SYSCTL_PLLFREQ0_MINT_R {
        SYSCTL_PLLFREQ0_MINT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq0_mfrac(&self) -> SYSCTL_PLLFREQ0_MFRAC_R {
        SYSCTL_PLLFREQ0_MFRAC_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - PLL M Integer Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pllfreq0_mint(&mut self) -> SYSCTL_PLLFREQ0_MINT_W<PLLFREQ0_SPEC, 0> {
        SYSCTL_PLLFREQ0_MINT_W::new(self)
    }
    #[doc = "Bits 10:19 - PLL M Fractional Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_pllfreq0_mfrac(&mut self) -> SYSCTL_PLLFREQ0_MFRAC_W<PLLFREQ0_SPEC, 10> {
        SYSCTL_PLLFREQ0_MFRAC_W::new(self)
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
#[doc = "PLL Frequency 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pllfreq0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pllfreq0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLLFREQ0_SPEC;
impl crate::RegisterSpec for PLLFREQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllfreq0::R`](R) reader structure"]
impl crate::Readable for PLLFREQ0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pllfreq0::W`](W) writer structure"]
impl crate::Writable for PLLFREQ0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLFREQ0 to value 0"]
impl crate::Resettable for PLLFREQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
