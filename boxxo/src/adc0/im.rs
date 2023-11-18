#[doc = "Register `IM` reader"]
pub type R = crate::R<IM_SPEC>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<IM_SPEC>;
#[doc = "Field `ADC_IM_MASK0` reader - SS0 Interrupt Mask"]
pub type ADC_IM_MASK0_R = crate::BitReader;
#[doc = "Field `ADC_IM_MASK0` writer - SS0 Interrupt Mask"]
pub type ADC_IM_MASK0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_MASK1` reader - SS1 Interrupt Mask"]
pub type ADC_IM_MASK1_R = crate::BitReader;
#[doc = "Field `ADC_IM_MASK1` writer - SS1 Interrupt Mask"]
pub type ADC_IM_MASK1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_MASK2` reader - SS2 Interrupt Mask"]
pub type ADC_IM_MASK2_R = crate::BitReader;
#[doc = "Field `ADC_IM_MASK2` writer - SS2 Interrupt Mask"]
pub type ADC_IM_MASK2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_MASK3` reader - SS3 Interrupt Mask"]
pub type ADC_IM_MASK3_R = crate::BitReader;
#[doc = "Field `ADC_IM_MASK3` writer - SS3 Interrupt Mask"]
pub type ADC_IM_MASK3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_DCONSS0` reader - Digital Comparator Interrupt on SS0"]
pub type ADC_IM_DCONSS0_R = crate::BitReader;
#[doc = "Field `ADC_IM_DCONSS0` writer - Digital Comparator Interrupt on SS0"]
pub type ADC_IM_DCONSS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_DCONSS1` reader - Digital Comparator Interrupt on SS1"]
pub type ADC_IM_DCONSS1_R = crate::BitReader;
#[doc = "Field `ADC_IM_DCONSS1` writer - Digital Comparator Interrupt on SS1"]
pub type ADC_IM_DCONSS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_DCONSS2` reader - Digital Comparator Interrupt on SS2"]
pub type ADC_IM_DCONSS2_R = crate::BitReader;
#[doc = "Field `ADC_IM_DCONSS2` writer - Digital Comparator Interrupt on SS2"]
pub type ADC_IM_DCONSS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_IM_DCONSS3` reader - Digital Comparator Interrupt on SS3"]
pub type ADC_IM_DCONSS3_R = crate::BitReader;
#[doc = "Field `ADC_IM_DCONSS3` writer - Digital Comparator Interrupt on SS3"]
pub type ADC_IM_DCONSS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&self) -> ADC_IM_MASK0_R {
        ADC_IM_MASK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&self) -> ADC_IM_MASK1_R {
        ADC_IM_MASK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&self) -> ADC_IM_MASK2_R {
        ADC_IM_MASK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&self) -> ADC_IM_MASK3_R {
        ADC_IM_MASK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn adc_im_dconss0(&self) -> ADC_IM_DCONSS0_R {
        ADC_IM_DCONSS0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn adc_im_dconss1(&self) -> ADC_IM_DCONSS1_R {
        ADC_IM_DCONSS1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn adc_im_dconss2(&self) -> ADC_IM_DCONSS2_R {
        ADC_IM_DCONSS2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn adc_im_dconss3(&self) -> ADC_IM_DCONSS3_R {
        ADC_IM_DCONSS3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_mask0(&mut self) -> ADC_IM_MASK0_W<IM_SPEC, 0> {
        ADC_IM_MASK0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_mask1(&mut self) -> ADC_IM_MASK1_W<IM_SPEC, 1> {
        ADC_IM_MASK1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_mask2(&mut self) -> ADC_IM_MASK2_W<IM_SPEC, 2> {
        ADC_IM_MASK2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_mask3(&mut self) -> ADC_IM_MASK3_W<IM_SPEC, 3> {
        ADC_IM_MASK3_W::new(self)
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_dconss0(&mut self) -> ADC_IM_DCONSS0_W<IM_SPEC, 16> {
        ADC_IM_DCONSS0_W::new(self)
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_dconss1(&mut self) -> ADC_IM_DCONSS1_W<IM_SPEC, 17> {
        ADC_IM_DCONSS1_W::new(self)
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_dconss2(&mut self) -> ADC_IM_DCONSS2_W<IM_SPEC, 18> {
        ADC_IM_DCONSS2_W::new(self)
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    #[must_use]
    pub fn adc_im_dconss3(&mut self) -> ADC_IM_DCONSS3_W<IM_SPEC, 19> {
        ADC_IM_DCONSS3_W::new(self)
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
#[doc = "ADC Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for IM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for IM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
