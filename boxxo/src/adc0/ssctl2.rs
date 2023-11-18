#[doc = "Register `SSCTL2` reader"]
pub type R = crate::R<SSCTL2_SPEC>;
#[doc = "Register `SSCTL2` writer"]
pub type W = crate::W<SSCTL2_SPEC>;
#[doc = "Field `ADC_SSCTL2_D0` reader - 1st Sample Diff Input Select"]
pub type ADC_SSCTL2_D0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_D0` writer - 1st Sample Diff Input Select"]
pub type ADC_SSCTL2_D0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_END0` reader - 1st Sample is End of Sequence"]
pub type ADC_SSCTL2_END0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_END0` writer - 1st Sample is End of Sequence"]
pub type ADC_SSCTL2_END0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_IE0` reader - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_IE0` writer - 1st Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_TS0` reader - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS0_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_TS0` writer - 1st Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_D1` reader - 2nd Sample Diff Input Select"]
pub type ADC_SSCTL2_D1_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_D1` writer - 2nd Sample Diff Input Select"]
pub type ADC_SSCTL2_D1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_END1` reader - 2nd Sample is End of Sequence"]
pub type ADC_SSCTL2_END1_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_END1` writer - 2nd Sample is End of Sequence"]
pub type ADC_SSCTL2_END1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_IE1` reader - 2nd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE1_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_IE1` writer - 2nd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_TS1` reader - 2nd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS1_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_TS1` writer - 2nd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_D2` reader - 3rd Sample Diff Input Select"]
pub type ADC_SSCTL2_D2_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_D2` writer - 3rd Sample Diff Input Select"]
pub type ADC_SSCTL2_D2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_END2` reader - 3rd Sample is End of Sequence"]
pub type ADC_SSCTL2_END2_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_END2` writer - 3rd Sample is End of Sequence"]
pub type ADC_SSCTL2_END2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_IE2` reader - 3rd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE2_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_IE2` writer - 3rd Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_TS2` reader - 3rd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS2_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_TS2` writer - 3rd Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_D3` reader - 4th Sample Diff Input Select"]
pub type ADC_SSCTL2_D3_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_D3` writer - 4th Sample Diff Input Select"]
pub type ADC_SSCTL2_D3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_END3` reader - 4th Sample is End of Sequence"]
pub type ADC_SSCTL2_END3_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_END3` writer - 4th Sample is End of Sequence"]
pub type ADC_SSCTL2_END3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_IE3` reader - 4th Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE3_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_IE3` writer - 4th Sample Interrupt Enable"]
pub type ADC_SSCTL2_IE3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_SSCTL2_TS3` reader - 4th Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS3_R = crate::BitReader;
#[doc = "Field `ADC_SSCTL2_TS3` writer - 4th Sample Temp Sensor Select"]
pub type ADC_SSCTL2_TS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d0(&self) -> ADC_SSCTL2_D0_R {
        ADC_SSCTL2_D0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end0(&self) -> ADC_SSCTL2_END0_R {
        ADC_SSCTL2_END0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie0(&self) -> ADC_SSCTL2_IE0_R {
        ADC_SSCTL2_IE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts0(&self) -> ADC_SSCTL2_TS0_R {
        ADC_SSCTL2_TS0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 2nd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d1(&self) -> ADC_SSCTL2_D1_R {
        ADC_SSCTL2_D1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end1(&self) -> ADC_SSCTL2_END1_R {
        ADC_SSCTL2_END1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie1(&self) -> ADC_SSCTL2_IE1_R {
        ADC_SSCTL2_IE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts1(&self) -> ADC_SSCTL2_TS1_R {
        ADC_SSCTL2_TS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 3rd Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d2(&self) -> ADC_SSCTL2_D2_R {
        ADC_SSCTL2_D2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end2(&self) -> ADC_SSCTL2_END2_R {
        ADC_SSCTL2_END2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie2(&self) -> ADC_SSCTL2_IE2_R {
        ADC_SSCTL2_IE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts2(&self) -> ADC_SSCTL2_TS2_R {
        ADC_SSCTL2_TS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 4th Sample Diff Input Select"]
    #[inline(always)]
    pub fn adc_ssctl2_d3(&self) -> ADC_SSCTL2_D3_R {
        ADC_SSCTL2_D3_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    pub fn adc_ssctl2_end3(&self) -> ADC_SSCTL2_END3_R {
        ADC_SSCTL2_END3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ssctl2_ie3(&self) -> ADC_SSCTL2_IE3_R {
        ADC_SSCTL2_IE3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    pub fn adc_ssctl2_ts3(&self) -> ADC_SSCTL2_TS3_R {
        ADC_SSCTL2_TS3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1st Sample Diff Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_d0(&mut self) -> ADC_SSCTL2_D0_W<SSCTL2_SPEC, 0> {
        ADC_SSCTL2_D0_W::new(self)
    }
    #[doc = "Bit 1 - 1st Sample is End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_end0(&mut self) -> ADC_SSCTL2_END0_W<SSCTL2_SPEC, 1> {
        ADC_SSCTL2_END0_W::new(self)
    }
    #[doc = "Bit 2 - 1st Sample Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ie0(&mut self) -> ADC_SSCTL2_IE0_W<SSCTL2_SPEC, 2> {
        ADC_SSCTL2_IE0_W::new(self)
    }
    #[doc = "Bit 3 - 1st Sample Temp Sensor Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ts0(&mut self) -> ADC_SSCTL2_TS0_W<SSCTL2_SPEC, 3> {
        ADC_SSCTL2_TS0_W::new(self)
    }
    #[doc = "Bit 4 - 2nd Sample Diff Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_d1(&mut self) -> ADC_SSCTL2_D1_W<SSCTL2_SPEC, 4> {
        ADC_SSCTL2_D1_W::new(self)
    }
    #[doc = "Bit 5 - 2nd Sample is End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_end1(&mut self) -> ADC_SSCTL2_END1_W<SSCTL2_SPEC, 5> {
        ADC_SSCTL2_END1_W::new(self)
    }
    #[doc = "Bit 6 - 2nd Sample Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ie1(&mut self) -> ADC_SSCTL2_IE1_W<SSCTL2_SPEC, 6> {
        ADC_SSCTL2_IE1_W::new(self)
    }
    #[doc = "Bit 7 - 2nd Sample Temp Sensor Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ts1(&mut self) -> ADC_SSCTL2_TS1_W<SSCTL2_SPEC, 7> {
        ADC_SSCTL2_TS1_W::new(self)
    }
    #[doc = "Bit 8 - 3rd Sample Diff Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_d2(&mut self) -> ADC_SSCTL2_D2_W<SSCTL2_SPEC, 8> {
        ADC_SSCTL2_D2_W::new(self)
    }
    #[doc = "Bit 9 - 3rd Sample is End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_end2(&mut self) -> ADC_SSCTL2_END2_W<SSCTL2_SPEC, 9> {
        ADC_SSCTL2_END2_W::new(self)
    }
    #[doc = "Bit 10 - 3rd Sample Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ie2(&mut self) -> ADC_SSCTL2_IE2_W<SSCTL2_SPEC, 10> {
        ADC_SSCTL2_IE2_W::new(self)
    }
    #[doc = "Bit 11 - 3rd Sample Temp Sensor Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ts2(&mut self) -> ADC_SSCTL2_TS2_W<SSCTL2_SPEC, 11> {
        ADC_SSCTL2_TS2_W::new(self)
    }
    #[doc = "Bit 12 - 4th Sample Diff Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_d3(&mut self) -> ADC_SSCTL2_D3_W<SSCTL2_SPEC, 12> {
        ADC_SSCTL2_D3_W::new(self)
    }
    #[doc = "Bit 13 - 4th Sample is End of Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_end3(&mut self) -> ADC_SSCTL2_END3_W<SSCTL2_SPEC, 13> {
        ADC_SSCTL2_END3_W::new(self)
    }
    #[doc = "Bit 14 - 4th Sample Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ie3(&mut self) -> ADC_SSCTL2_IE3_W<SSCTL2_SPEC, 14> {
        ADC_SSCTL2_IE3_W::new(self)
    }
    #[doc = "Bit 15 - 4th Sample Temp Sensor Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc_ssctl2_ts3(&mut self) -> ADC_SSCTL2_TS3_W<SSCTL2_SPEC, 15> {
        ADC_SSCTL2_TS3_W::new(self)
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
#[doc = "ADC Sample Sequence Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssctl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssctl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSCTL2_SPEC;
impl crate::RegisterSpec for SSCTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssctl2::R`](R) reader structure"]
impl crate::Readable for SSCTL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssctl2::W`](W) writer structure"]
impl crate::Writable for SSCTL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSCTL2 to value 0"]
impl crate::Resettable for SSCTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
