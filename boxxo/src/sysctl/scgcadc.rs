#[doc = "Register `SCGCADC` reader"]
pub type R = crate::R<SCGCADC_SPEC>;
#[doc = "Register `SCGCADC` writer"]
pub type W = crate::W<SCGCADC_SPEC>;
#[doc = "Field `SYSCTL_SCGCADC_S0` reader - ADC Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCADC_S0` writer - ADC Module 0 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SCGCADC_S1` reader - ADC Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SCGCADC_S1` writer - ADC Module 1 Sleep Mode Clock Gating Control"]
pub type SYSCTL_SCGCADC_S1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcadc_s0(&self) -> SYSCTL_SCGCADC_S0_R {
        SYSCTL_SCGCADC_S0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcadc_s1(&self) -> SYSCTL_SCGCADC_S1_R {
        SYSCTL_SCGCADC_S1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcadc_s0(&mut self) -> SYSCTL_SCGCADC_S0_W<SCGCADC_SPEC, 0> {
        SYSCTL_SCGCADC_S0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_scgcadc_s1(&mut self) -> SYSCTL_SCGCADC_S1_W<SCGCADC_SPEC, 1> {
        SYSCTL_SCGCADC_S1_W::new(self)
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
#[doc = "Analog-to-Digital Converter Sleep Mode Clock Gating Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgcadc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgcadc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCGCADC_SPEC;
impl crate::RegisterSpec for SCGCADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgcadc::R`](R) reader structure"]
impl crate::Readable for SCGCADC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scgcadc::W`](W) writer structure"]
impl crate::Writable for SCGCADC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCADC to value 0"]
impl crate::Resettable for SCGCADC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
