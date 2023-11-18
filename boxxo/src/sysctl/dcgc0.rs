#[doc = "Register `DCGC0` reader"]
pub type R = crate::R<DCGC0_SPEC>;
#[doc = "Register `DCGC0` writer"]
pub type W = crate::W<DCGC0_SPEC>;
#[doc = "Field `SYSCTL_DCGC0_WDT0` reader - WDT0 Clock Gating Control"]
pub type SYSCTL_DCGC0_WDT0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_WDT0` writer - WDT0 Clock Gating Control"]
pub type SYSCTL_DCGC0_WDT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_HIB` reader - HIB Clock Gating Control"]
pub type SYSCTL_DCGC0_HIB_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_HIB` writer - HIB Clock Gating Control"]
pub type SYSCTL_DCGC0_HIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_ADC0` reader - ADC0 Clock Gating Control"]
pub type SYSCTL_DCGC0_ADC0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_ADC0` writer - ADC0 Clock Gating Control"]
pub type SYSCTL_DCGC0_ADC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_ADC1` reader - ADC1 Clock Gating Control"]
pub type SYSCTL_DCGC0_ADC1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_ADC1` writer - ADC1 Clock Gating Control"]
pub type SYSCTL_DCGC0_ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_PWM0` reader - PWM Clock Gating Control"]
pub type SYSCTL_DCGC0_PWM0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_PWM0` writer - PWM Clock Gating Control"]
pub type SYSCTL_DCGC0_PWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_CAN0` reader - CAN0 Clock Gating Control"]
pub type SYSCTL_DCGC0_CAN0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_CAN0` writer - CAN0 Clock Gating Control"]
pub type SYSCTL_DCGC0_CAN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_CAN1` reader - CAN1 Clock Gating Control"]
pub type SYSCTL_DCGC0_CAN1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_CAN1` writer - CAN1 Clock Gating Control"]
pub type SYSCTL_DCGC0_CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DCGC0_WDT1` reader - WDT1 Clock Gating Control"]
pub type SYSCTL_DCGC0_WDT1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DCGC0_WDT1` writer - WDT1 Clock Gating Control"]
pub type SYSCTL_DCGC0_WDT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - WDT0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_wdt0(&self) -> SYSCTL_DCGC0_WDT0_R {
        SYSCTL_DCGC0_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_hib(&self) -> SYSCTL_DCGC0_HIB_R {
        SYSCTL_DCGC0_HIB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_adc0(&self) -> SYSCTL_DCGC0_ADC0_R {
        SYSCTL_DCGC0_ADC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_adc1(&self) -> SYSCTL_DCGC0_ADC1_R {
        SYSCTL_DCGC0_ADC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_pwm0(&self) -> SYSCTL_DCGC0_PWM0_R {
        SYSCTL_DCGC0_PWM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - CAN0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_can0(&self) -> SYSCTL_DCGC0_CAN0_R {
        SYSCTL_DCGC0_CAN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_can1(&self) -> SYSCTL_DCGC0_CAN1_R {
        SYSCTL_DCGC0_CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - WDT1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgc0_wdt1(&self) -> SYSCTL_DCGC0_WDT1_R {
        SYSCTL_DCGC0_WDT1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - WDT0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_wdt0(&mut self) -> SYSCTL_DCGC0_WDT0_W<DCGC0_SPEC, 3> {
        SYSCTL_DCGC0_WDT0_W::new(self)
    }
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_hib(&mut self) -> SYSCTL_DCGC0_HIB_W<DCGC0_SPEC, 6> {
        SYSCTL_DCGC0_HIB_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_adc0(&mut self) -> SYSCTL_DCGC0_ADC0_W<DCGC0_SPEC, 16> {
        SYSCTL_DCGC0_ADC0_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_adc1(&mut self) -> SYSCTL_DCGC0_ADC1_W<DCGC0_SPEC, 17> {
        SYSCTL_DCGC0_ADC1_W::new(self)
    }
    #[doc = "Bit 20 - PWM Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_pwm0(&mut self) -> SYSCTL_DCGC0_PWM0_W<DCGC0_SPEC, 20> {
        SYSCTL_DCGC0_PWM0_W::new(self)
    }
    #[doc = "Bit 24 - CAN0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_can0(&mut self) -> SYSCTL_DCGC0_CAN0_W<DCGC0_SPEC, 24> {
        SYSCTL_DCGC0_CAN0_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_can1(&mut self) -> SYSCTL_DCGC0_CAN1_W<DCGC0_SPEC, 25> {
        SYSCTL_DCGC0_CAN1_W::new(self)
    }
    #[doc = "Bit 28 - WDT1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dcgc0_wdt1(&mut self) -> SYSCTL_DCGC0_WDT1_W<DCGC0_SPEC, 28> {
        SYSCTL_DCGC0_WDT1_W::new(self)
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
#[doc = "Deep Sleep Mode Clock Gating Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGC0_SPEC;
impl crate::RegisterSpec for DCGC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgc0::R`](R) reader structure"]
impl crate::Readable for DCGC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgc0::W`](W) writer structure"]
impl crate::Writable for DCGC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGC0 to value 0"]
impl crate::Resettable for DCGC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
