#[doc = "Register `ACTSS` reader"]
pub type R = crate::R<ACTSS_SPEC>;
#[doc = "Register `ACTSS` writer"]
pub type W = crate::W<ACTSS_SPEC>;
#[doc = "Field `ADC_ACTSS_ASEN0` reader - ADC SS0 Enable"]
pub type ADC_ACTSS_ASEN0_R = crate::BitReader;
#[doc = "Field `ADC_ACTSS_ASEN0` writer - ADC SS0 Enable"]
pub type ADC_ACTSS_ASEN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ACTSS_ASEN1` reader - ADC SS1 Enable"]
pub type ADC_ACTSS_ASEN1_R = crate::BitReader;
#[doc = "Field `ADC_ACTSS_ASEN1` writer - ADC SS1 Enable"]
pub type ADC_ACTSS_ASEN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ACTSS_ASEN2` reader - ADC SS2 Enable"]
pub type ADC_ACTSS_ASEN2_R = crate::BitReader;
#[doc = "Field `ADC_ACTSS_ASEN2` writer - ADC SS2 Enable"]
pub type ADC_ACTSS_ASEN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ACTSS_ASEN3` reader - ADC SS3 Enable"]
pub type ADC_ACTSS_ASEN3_R = crate::BitReader;
#[doc = "Field `ADC_ACTSS_ASEN3` writer - ADC SS3 Enable"]
pub type ADC_ACTSS_ASEN3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_ACTSS_BUSY` reader - ADC Busy"]
pub type ADC_ACTSS_BUSY_R = crate::BitReader;
#[doc = "Field `ADC_ACTSS_BUSY` writer - ADC Busy"]
pub type ADC_ACTSS_BUSY_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&self) -> ADC_ACTSS_ASEN0_R {
        ADC_ACTSS_ASEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&self) -> ADC_ACTSS_ASEN1_R {
        ADC_ACTSS_ASEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&self) -> ADC_ACTSS_ASEN2_R {
        ADC_ACTSS_ASEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&self) -> ADC_ACTSS_ASEN3_R {
        ADC_ACTSS_ASEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn adc_actss_busy(&self) -> ADC_ACTSS_BUSY_R {
        ADC_ACTSS_BUSY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_actss_asen0(&mut self) -> ADC_ACTSS_ASEN0_W<ACTSS_SPEC, 0> {
        ADC_ACTSS_ASEN0_W::new(self)
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_actss_asen1(&mut self) -> ADC_ACTSS_ASEN1_W<ACTSS_SPEC, 1> {
        ADC_ACTSS_ASEN1_W::new(self)
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_actss_asen2(&mut self) -> ADC_ACTSS_ASEN2_W<ACTSS_SPEC, 2> {
        ADC_ACTSS_ASEN2_W::new(self)
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_actss_asen3(&mut self) -> ADC_ACTSS_ASEN3_W<ACTSS_SPEC, 3> {
        ADC_ACTSS_ASEN3_W::new(self)
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    #[must_use]
    pub fn adc_actss_busy(&mut self) -> ADC_ACTSS_BUSY_W<ACTSS_SPEC, 16> {
        ADC_ACTSS_BUSY_W::new(self)
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
#[doc = "ADC Active Sample Sequencer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actss::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actss::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACTSS_SPEC;
impl crate::RegisterSpec for ACTSS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actss::R`](R) reader structure"]
impl crate::Readable for ACTSS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`actss::W`](W) writer structure"]
impl crate::Writable for ACTSS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTSS to value 0"]
impl crate::Resettable for ACTSS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
