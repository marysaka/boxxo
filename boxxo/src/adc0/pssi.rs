#[doc = "Register `PSSI` reader"]
pub type R = crate::R<PSSI_SPEC>;
#[doc = "Register `PSSI` writer"]
pub type W = crate::W<PSSI_SPEC>;
#[doc = "Field `ADC_PSSI_SS0` reader - SS0 Initiate"]
pub type ADC_PSSI_SS0_R = crate::BitReader;
#[doc = "Field `ADC_PSSI_SS0` writer - SS0 Initiate"]
pub type ADC_PSSI_SS0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_PSSI_SS1` reader - SS1 Initiate"]
pub type ADC_PSSI_SS1_R = crate::BitReader;
#[doc = "Field `ADC_PSSI_SS1` writer - SS1 Initiate"]
pub type ADC_PSSI_SS1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_PSSI_SS2` reader - SS2 Initiate"]
pub type ADC_PSSI_SS2_R = crate::BitReader;
#[doc = "Field `ADC_PSSI_SS2` writer - SS2 Initiate"]
pub type ADC_PSSI_SS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_PSSI_SS3` reader - SS3 Initiate"]
pub type ADC_PSSI_SS3_R = crate::BitReader;
#[doc = "Field `ADC_PSSI_SS3` writer - SS3 Initiate"]
pub type ADC_PSSI_SS3_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_PSSI_SYNCWAIT` reader - Synchronize Wait"]
pub type ADC_PSSI_SYNCWAIT_R = crate::BitReader;
#[doc = "Field `ADC_PSSI_SYNCWAIT` writer - Synchronize Wait"]
pub type ADC_PSSI_SYNCWAIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_PSSI_GSYNC` reader - Global Synchronize"]
pub type ADC_PSSI_GSYNC_R = crate::BitReader;
#[doc = "Field `ADC_PSSI_GSYNC` writer - Global Synchronize"]
pub type ADC_PSSI_GSYNC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss0(&self) -> ADC_PSSI_SS0_R {
        ADC_PSSI_SS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss1(&self) -> ADC_PSSI_SS1_R {
        ADC_PSSI_SS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss2(&self) -> ADC_PSSI_SS2_R {
        ADC_PSSI_SS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    pub fn adc_pssi_ss3(&self) -> ADC_PSSI_SS3_R {
        ADC_PSSI_SS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    pub fn adc_pssi_syncwait(&self) -> ADC_PSSI_SYNCWAIT_R {
        ADC_PSSI_SYNCWAIT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    pub fn adc_pssi_gsync(&self) -> ADC_PSSI_GSYNC_R {
        ADC_PSSI_GSYNC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS0 Initiate"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pssi_ss0(&mut self) -> ADC_PSSI_SS0_W<PSSI_SPEC, 0> {
        ADC_PSSI_SS0_W::new(self)
    }
    #[doc = "Bit 1 - SS1 Initiate"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pssi_ss1(&mut self) -> ADC_PSSI_SS1_W<PSSI_SPEC, 1> {
        ADC_PSSI_SS1_W::new(self)
    }
    #[doc = "Bit 2 - SS2 Initiate"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pssi_ss2(&mut self) -> ADC_PSSI_SS2_W<PSSI_SPEC, 2> {
        ADC_PSSI_SS2_W::new(self)
    }
    #[doc = "Bit 3 - SS3 Initiate"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pssi_ss3(&mut self) -> ADC_PSSI_SS3_W<PSSI_SPEC, 3> {
        ADC_PSSI_SS3_W::new(self)
    }
    #[doc = "Bit 27 - Synchronize Wait"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pssi_syncwait(&mut self) -> ADC_PSSI_SYNCWAIT_W<PSSI_SPEC, 27> {
        ADC_PSSI_SYNCWAIT_W::new(self)
    }
    #[doc = "Bit 31 - Global Synchronize"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pssi_gsync(&mut self) -> ADC_PSSI_GSYNC_W<PSSI_SPEC, 31> {
        ADC_PSSI_GSYNC_W::new(self)
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
#[doc = "ADC Processor Sample Sequence Initiate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pssi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pssi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSSI_SPEC;
impl crate::RegisterSpec for PSSI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pssi::R`](R) reader structure"]
impl crate::Readable for PSSI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pssi::W`](W) writer structure"]
impl crate::Writable for PSSI_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSSI to value 0"]
impl crate::Resettable for PSSI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
