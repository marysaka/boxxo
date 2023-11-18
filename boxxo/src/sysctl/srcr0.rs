#[doc = "Register `SRCR0` reader"]
pub type R = crate::R<SRCR0_SPEC>;
#[doc = "Register `SRCR0` writer"]
pub type W = crate::W<SRCR0_SPEC>;
#[doc = "Field `SYSCTL_SRCR0_WDT0` reader - WDT0 Reset Control"]
pub type SYSCTL_SRCR0_WDT0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_WDT0` writer - WDT0 Reset Control"]
pub type SYSCTL_SRCR0_WDT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_HIB` reader - HIB Reset Control"]
pub type SYSCTL_SRCR0_HIB_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_HIB` writer - HIB Reset Control"]
pub type SYSCTL_SRCR0_HIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_ADC0` reader - ADC0 Reset Control"]
pub type SYSCTL_SRCR0_ADC0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_ADC0` writer - ADC0 Reset Control"]
pub type SYSCTL_SRCR0_ADC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_ADC1` reader - ADC1 Reset Control"]
pub type SYSCTL_SRCR0_ADC1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_ADC1` writer - ADC1 Reset Control"]
pub type SYSCTL_SRCR0_ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_PWM0` reader - PWM Reset Control"]
pub type SYSCTL_SRCR0_PWM0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_PWM0` writer - PWM Reset Control"]
pub type SYSCTL_SRCR0_PWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_CAN0` reader - CAN0 Reset Control"]
pub type SYSCTL_SRCR0_CAN0_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_CAN0` writer - CAN0 Reset Control"]
pub type SYSCTL_SRCR0_CAN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_CAN1` reader - CAN1 Reset Control"]
pub type SYSCTL_SRCR0_CAN1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_CAN1` writer - CAN1 Reset Control"]
pub type SYSCTL_SRCR0_CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_SRCR0_WDT1` reader - WDT1 Reset Control"]
pub type SYSCTL_SRCR0_WDT1_R = crate::BitReader;
#[doc = "Field `SYSCTL_SRCR0_WDT1` writer - WDT1 Reset Control"]
pub type SYSCTL_SRCR0_WDT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - WDT0 Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_wdt0(&self) -> SYSCTL_SRCR0_WDT0_R {
        SYSCTL_SRCR0_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - HIB Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_hib(&self) -> SYSCTL_SRCR0_HIB_R {
        SYSCTL_SRCR0_HIB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC0 Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_adc0(&self) -> SYSCTL_SRCR0_ADC0_R {
        SYSCTL_SRCR0_ADC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_adc1(&self) -> SYSCTL_SRCR0_ADC1_R {
        SYSCTL_SRCR0_ADC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_pwm0(&self) -> SYSCTL_SRCR0_PWM0_R {
        SYSCTL_SRCR0_PWM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - CAN0 Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_can0(&self) -> SYSCTL_SRCR0_CAN0_R {
        SYSCTL_SRCR0_CAN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_can1(&self) -> SYSCTL_SRCR0_CAN1_R {
        SYSCTL_SRCR0_CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - WDT1 Reset Control"]
    #[inline(always)]
    pub fn sysctl_srcr0_wdt1(&self) -> SYSCTL_SRCR0_WDT1_R {
        SYSCTL_SRCR0_WDT1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - WDT0 Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_wdt0(&mut self) -> SYSCTL_SRCR0_WDT0_W<SRCR0_SPEC, 3> {
        SYSCTL_SRCR0_WDT0_W::new(self)
    }
    #[doc = "Bit 6 - HIB Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_hib(&mut self) -> SYSCTL_SRCR0_HIB_W<SRCR0_SPEC, 6> {
        SYSCTL_SRCR0_HIB_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_adc0(&mut self) -> SYSCTL_SRCR0_ADC0_W<SRCR0_SPEC, 16> {
        SYSCTL_SRCR0_ADC0_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_adc1(&mut self) -> SYSCTL_SRCR0_ADC1_W<SRCR0_SPEC, 17> {
        SYSCTL_SRCR0_ADC1_W::new(self)
    }
    #[doc = "Bit 20 - PWM Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_pwm0(&mut self) -> SYSCTL_SRCR0_PWM0_W<SRCR0_SPEC, 20> {
        SYSCTL_SRCR0_PWM0_W::new(self)
    }
    #[doc = "Bit 24 - CAN0 Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_can0(&mut self) -> SYSCTL_SRCR0_CAN0_W<SRCR0_SPEC, 24> {
        SYSCTL_SRCR0_CAN0_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_can1(&mut self) -> SYSCTL_SRCR0_CAN1_W<SRCR0_SPEC, 25> {
        SYSCTL_SRCR0_CAN1_W::new(self)
    }
    #[doc = "Bit 28 - WDT1 Reset Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_srcr0_wdt1(&mut self) -> SYSCTL_SRCR0_WDT1_W<SRCR0_SPEC, 28> {
        SYSCTL_SRCR0_WDT1_W::new(self)
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
#[doc = "Software Reset Control 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCR0_SPEC;
impl crate::RegisterSpec for SRCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcr0::R`](R) reader structure"]
impl crate::Readable for SRCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcr0::W`](W) writer structure"]
impl crate::Writable for SRCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCR0 to value 0"]
impl crate::Resettable for SRCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
