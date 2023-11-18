#[doc = "Register `DCCMP1` reader"]
pub type R = crate::R<DCCMP1_SPEC>;
#[doc = "Register `DCCMP1` writer"]
pub type W = crate::W<DCCMP1_SPEC>;
#[doc = "Field `ADC_DCCMP1_COMP0` reader - Compare 0"]
pub type ADC_DCCMP1_COMP0_R = crate::FieldReader<u16>;
#[doc = "Field `ADC_DCCMP1_COMP0` writer - Compare 0"]
pub type ADC_DCCMP1_COMP0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
#[doc = "Field `ADC_DCCMP1_COMP1` reader - Compare 1"]
pub type ADC_DCCMP1_COMP1_R = crate::FieldReader<u16>;
#[doc = "Field `ADC_DCCMP1_COMP1` writer - Compare 1"]
pub type ADC_DCCMP1_COMP1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp1_comp0(&self) -> ADC_DCCMP1_COMP0_R {
        ADC_DCCMP1_COMP0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp1_comp1(&self) -> ADC_DCCMP1_COMP1_R {
        ADC_DCCMP1_COMP1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dccmp1_comp0(&mut self) -> ADC_DCCMP1_COMP0_W<DCCMP1_SPEC, 0> {
        ADC_DCCMP1_COMP0_W::new(self)
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dccmp1_comp1(&mut self) -> ADC_DCCMP1_COMP1_W<DCCMP1_SPEC, 16> {
        ADC_DCCMP1_COMP1_W::new(self)
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
#[doc = "ADC Digital Comparator Range 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dccmp1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dccmp1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCCMP1_SPEC;
impl crate::RegisterSpec for DCCMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dccmp1::R`](R) reader structure"]
impl crate::Readable for DCCMP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dccmp1::W`](W) writer structure"]
impl crate::Writable for DCCMP1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCCMP1 to value 0"]
impl crate::Resettable for DCCMP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
