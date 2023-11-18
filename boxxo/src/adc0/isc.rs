#[doc = "Register `ISC` reader"]
pub type R = crate::R<ISC_SPEC>;
#[doc = "Register `ISC` writer"]
pub type W = crate::W<ISC_SPEC>;
#[doc = "Field `ADC_ISC_IN0` reader - SS0 Interrupt Status and Clear"]
pub type ADC_ISC_IN0_R = crate::BitReader;
#[doc = "Field `ADC_ISC_IN0` writer - SS0 Interrupt Status and Clear"]
pub type ADC_ISC_IN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_IN1` reader - SS1 Interrupt Status and Clear"]
pub type ADC_ISC_IN1_R = crate::BitReader;
#[doc = "Field `ADC_ISC_IN1` writer - SS1 Interrupt Status and Clear"]
pub type ADC_ISC_IN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_IN2` reader - SS2 Interrupt Status and Clear"]
pub type ADC_ISC_IN2_R = crate::BitReader;
#[doc = "Field `ADC_ISC_IN2` writer - SS2 Interrupt Status and Clear"]
pub type ADC_ISC_IN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_IN3` reader - SS3 Interrupt Status and Clear"]
pub type ADC_ISC_IN3_R = crate::BitReader;
#[doc = "Field `ADC_ISC_IN3` writer - SS3 Interrupt Status and Clear"]
pub type ADC_ISC_IN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_DCINSS0` reader - Digital Comparator Interrupt Status on SS0"]
pub type ADC_ISC_DCINSS0_R = crate::BitReader;
#[doc = "Field `ADC_ISC_DCINSS0` writer - Digital Comparator Interrupt Status on SS0"]
pub type ADC_ISC_DCINSS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_DCINSS1` reader - Digital Comparator Interrupt Status on SS1"]
pub type ADC_ISC_DCINSS1_R = crate::BitReader;
#[doc = "Field `ADC_ISC_DCINSS1` writer - Digital Comparator Interrupt Status on SS1"]
pub type ADC_ISC_DCINSS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_DCINSS2` reader - Digital Comparator Interrupt Status on SS2"]
pub type ADC_ISC_DCINSS2_R = crate::BitReader;
#[doc = "Field `ADC_ISC_DCINSS2` writer - Digital Comparator Interrupt Status on SS2"]
pub type ADC_ISC_DCINSS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ISC_DCINSS3` reader - Digital Comparator Interrupt Status on SS3"]
pub type ADC_ISC_DCINSS3_R = crate::BitReader;
#[doc = "Field `ADC_ISC_DCINSS3` writer - Digital Comparator Interrupt Status on SS3"]
pub type ADC_ISC_DCINSS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&self) -> ADC_ISC_IN0_R {
        ADC_ISC_IN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&self) -> ADC_ISC_IN1_R {
        ADC_ISC_IN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&self) -> ADC_ISC_IN2_R {
        ADC_ISC_IN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&self) -> ADC_ISC_IN3_R {
        ADC_ISC_IN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn adc_isc_dcinss0(&self) -> ADC_ISC_DCINSS0_R {
        ADC_ISC_DCINSS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn adc_isc_dcinss1(&self) -> ADC_ISC_DCINSS1_R {
        ADC_ISC_DCINSS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn adc_isc_dcinss2(&self) -> ADC_ISC_DCINSS2_R {
        ADC_ISC_DCINSS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn adc_isc_dcinss3(&self) -> ADC_ISC_DCINSS3_R {
        ADC_ISC_DCINSS3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_in0(&mut self) -> ADC_ISC_IN0_W<ISC_SPEC, 0> {
        ADC_ISC_IN0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_in1(&mut self) -> ADC_ISC_IN1_W<ISC_SPEC, 1> {
        ADC_ISC_IN1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_in2(&mut self) -> ADC_ISC_IN2_W<ISC_SPEC, 2> {
        ADC_ISC_IN2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_in3(&mut self) -> ADC_ISC_IN3_W<ISC_SPEC, 3> {
        ADC_ISC_IN3_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_dcinss0(&mut self) -> ADC_ISC_DCINSS0_W<ISC_SPEC, 16> {
        ADC_ISC_DCINSS0_W::new(self)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_dcinss1(&mut self) -> ADC_ISC_DCINSS1_W<ISC_SPEC, 17> {
        ADC_ISC_DCINSS1_W::new(self)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_dcinss2(&mut self) -> ADC_ISC_DCINSS2_W<ISC_SPEC, 18> {
        ADC_ISC_DCINSS2_W::new(self)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_isc_dcinss3(&mut self) -> ADC_ISC_DCINSS3_W<ISC_SPEC, 19> {
        ADC_ISC_DCINSS3_W::new(self)
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
#[doc = "ADC Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ISC_SPEC;
impl crate::RegisterSpec for ISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isc::R`](R) reader structure"]
impl crate::Readable for ISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`isc::W`](W) writer structure"]
impl crate::Writable for ISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISC to value 0"]
impl crate::Resettable for ISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
