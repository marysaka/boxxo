#[doc = "Register `SRADC` reader"]
pub type R = crate::R<SRADC_SPEC>;
#[doc = "Register `SRADC` writer"]
pub type W = crate::W<SRADC_SPEC>;
#[doc = "Field `SYSCTL_SRADC_R0` reader - ADC Module 0 Software Reset"]
pub type SYSCTL_SRADC_R0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRADC_R0` writer - ADC Module 0 Software Reset"]
pub type SYSCTL_SRADC_R0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRADC_R1` reader - ADC Module 1 Software Reset"]
pub type SYSCTL_SRADC_R1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRADC_R1` writer - ADC Module 1 Software Reset"]
pub type SYSCTL_SRADC_R1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ADC Module 0 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sradc_r0(&self) -> SYSCTL_SRADC_R0_R {
        SYSCTL_SRADC_R0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Module 1 Software Reset"]
    #[inline(always)]
    pub fn sysctl_sradc_r1(&self) -> SYSCTL_SRADC_R1_R {
        SYSCTL_SRADC_R1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Module 0 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_sradc_r0(&mut self) -> SYSCTL_SRADC_R0_W<SRADC_SPEC, 0> {
        SYSCTL_SRADC_R0_W::new(self)
    }
    #[doc = "Bit 1 - ADC Module 1 Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_sradc_r1(&mut self) -> SYSCTL_SRADC_R1_W<SRADC_SPEC, 1> {
        SYSCTL_SRADC_R1_W::new(self)
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
#[doc = "Analog-to-Digital Converter Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sradc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sradc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRADC_SPEC;
impl crate::RegisterSpec for SRADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sradc::R`](R) reader structure"]
impl crate::Readable for SRADC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sradc::W`](W) writer structure"]
impl crate::Writable for SRADC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRADC to value 0"]
impl crate::Resettable for SRADC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
