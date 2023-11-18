#[doc = "Register `DCISC` reader"]
pub type R = crate::R<DCISC_SPEC>;
#[doc = "Register `DCISC` writer"]
pub type W = crate::W<DCISC_SPEC>;
#[doc = "Field `ADC_DCISC_DCINT0` reader - Digital Comparator 0 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT0_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT0` writer - Digital Comparator 0 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT1` reader - Digital Comparator 1 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT1_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT1` writer - Digital Comparator 1 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT2` reader - Digital Comparator 2 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT2_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT2` writer - Digital Comparator 2 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT3` reader - Digital Comparator 3 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT3_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT3` writer - Digital Comparator 3 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT4` reader - Digital Comparator 4 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT4_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT4` writer - Digital Comparator 4 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT5` reader - Digital Comparator 5 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT5_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT5` writer - Digital Comparator 5 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT5_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT6` reader - Digital Comparator 6 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT6_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT6` writer - Digital Comparator 6 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT6_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_DCISC_DCINT7` reader - Digital Comparator 7 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT7_R = crate::BitReader;
#[doc = "Field `ADC_DCISC_DCINT7` writer - Digital Comparator 7 Interrupt Status and Clear"]
pub type ADC_DCISC_DCINT7_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Digital Comparator 0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint0(&self) -> ADC_DCISC_DCINT0_R {
        ADC_DCISC_DCINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital Comparator 1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint1(&self) -> ADC_DCISC_DCINT1_R {
        ADC_DCISC_DCINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Digital Comparator 2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint2(&self) -> ADC_DCISC_DCINT2_R {
        ADC_DCISC_DCINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Comparator 3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint3(&self) -> ADC_DCISC_DCINT3_R {
        ADC_DCISC_DCINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Digital Comparator 4 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint4(&self) -> ADC_DCISC_DCINT4_R {
        ADC_DCISC_DCINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Digital Comparator 5 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint5(&self) -> ADC_DCISC_DCINT5_R {
        ADC_DCISC_DCINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Digital Comparator 6 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint6(&self) -> ADC_DCISC_DCINT6_R {
        ADC_DCISC_DCINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Digital Comparator 7 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint7(&self) -> ADC_DCISC_DCINT7_R {
        ADC_DCISC_DCINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Digital Comparator 0 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint0(&mut self) -> ADC_DCISC_DCINT0_W<DCISC_SPEC, 0> {
        ADC_DCISC_DCINT0_W::new(self)
    }
    #[doc = "Bit 1 - Digital Comparator 1 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint1(&mut self) -> ADC_DCISC_DCINT1_W<DCISC_SPEC, 1> {
        ADC_DCISC_DCINT1_W::new(self)
    }
    #[doc = "Bit 2 - Digital Comparator 2 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint2(&mut self) -> ADC_DCISC_DCINT2_W<DCISC_SPEC, 2> {
        ADC_DCISC_DCINT2_W::new(self)
    }
    #[doc = "Bit 3 - Digital Comparator 3 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint3(&mut self) -> ADC_DCISC_DCINT3_W<DCISC_SPEC, 3> {
        ADC_DCISC_DCINT3_W::new(self)
    }
    #[doc = "Bit 4 - Digital Comparator 4 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint4(&mut self) -> ADC_DCISC_DCINT4_W<DCISC_SPEC, 4> {
        ADC_DCISC_DCINT4_W::new(self)
    }
    #[doc = "Bit 5 - Digital Comparator 5 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint5(&mut self) -> ADC_DCISC_DCINT5_W<DCISC_SPEC, 5> {
        ADC_DCISC_DCINT5_W::new(self)
    }
    #[doc = "Bit 6 - Digital Comparator 6 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint6(&mut self) -> ADC_DCISC_DCINT6_W<DCISC_SPEC, 6> {
        ADC_DCISC_DCINT6_W::new(self)
    }
    #[doc = "Bit 7 - Digital Comparator 7 Interrupt Status and Clear"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dcisc_dcint7(&mut self) -> ADC_DCISC_DCINT7_W<DCISC_SPEC, 7> {
        ADC_DCISC_DCINT7_W::new(self)
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
#[doc = "ADC Digital Comparator Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcisc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcisc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCISC_SPEC;
impl crate::RegisterSpec for DCISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcisc::R`](R) reader structure"]
impl crate::Readable for DCISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcisc::W`](W) writer structure"]
impl crate::Writable for DCISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCISC to value 0"]
impl crate::Resettable for DCISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
