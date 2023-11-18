#[doc = "Register `RCGC0` reader"]
pub type R = crate::R<RCGC0_SPEC>;
#[doc = "Register `RCGC0` writer"]
pub type W = crate::W<RCGC0_SPEC>;
#[doc = "Field `SYSCTL_RCGC0_WDT0` reader - WDT0 Clock Gating Control"]
pub type SYSCTL_RCGC0_WDT0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_WDT0` writer - WDT0 Clock Gating Control"]
pub type SYSCTL_RCGC0_WDT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_HIB` reader - HIB Clock Gating Control"]
pub type SYSCTL_RCGC0_HIB_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_HIB` writer - HIB Clock Gating Control"]
pub type SYSCTL_RCGC0_HIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_ADC0SPD` reader - ADC0 Sample Speed"]
pub type SYSCTL_RCGC0_ADC0SPD_R = crate::FieldReader<SYSCTL_RCGC0_ADC0SPD_A>;
#[doc = "ADC0 Sample Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_RCGC0_ADC0SPD_A {
    #[doc = "0: 125K samples/second"]
    SYSCTL_RCGC0_ADC0SPD_125K = 0,
    #[doc = "1: 250K samples/second"]
    SYSCTL_RCGC0_ADC0SPD_250K = 1,
    #[doc = "2: 500K samples/second"]
    SYSCTL_RCGC0_ADC0SPD_500K = 2,
    #[doc = "3: 1M samples/second"]
    SYSCTL_RCGC0_ADC0SPD_1M = 3,
}
impl From<SYSCTL_RCGC0_ADC0SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCGC0_ADC0SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_RCGC0_ADC0SPD_A {
    type Ux = u8;
}
impl SYSCTL_RCGC0_ADC0SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCTL_RCGC0_ADC0SPD_A {
        match self.bits {
            0 => SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_125K,
            1 => SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_250K,
            2 => SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_500K,
            3 => SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc0spd_125k(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_125K
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc0spd_250k(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_250K
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc0spd_500k(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_500K
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc0spd_1m(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_1M
    }
}
#[doc = "Field `SYSCTL_RCGC0_ADC0SPD` writer - ADC0 Sample Speed"]
pub type SYSCTL_RCGC0_ADC0SPD_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, SYSCTL_RCGC0_ADC0SPD_A>;
impl<'a, REG, const O: u8> SYSCTL_RCGC0_ADC0SPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc0spd_125k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_125K)
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc0spd_250k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_250K)
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc0spd_500k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_500K)
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc0spd_1m(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC0SPD_A::SYSCTL_RCGC0_ADC0SPD_1M)
    }
}
#[doc = "Field `SYSCTL_RCGC0_ADC1SPD` reader - ADC1 Sample Speed"]
pub type SYSCTL_RCGC0_ADC1SPD_R = crate::FieldReader<SYSCTL_RCGC0_ADC1SPD_A>;
#[doc = "ADC1 Sample Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_RCGC0_ADC1SPD_A {
    #[doc = "0: 125K samples/second"]
    SYSCTL_RCGC0_ADC1SPD_125K = 0,
    #[doc = "1: 250K samples/second"]
    SYSCTL_RCGC0_ADC1SPD_250K = 1,
    #[doc = "2: 500K samples/second"]
    SYSCTL_RCGC0_ADC1SPD_500K = 2,
    #[doc = "3: 1M samples/second"]
    SYSCTL_RCGC0_ADC1SPD_1M = 3,
}
impl From<SYSCTL_RCGC0_ADC1SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCGC0_ADC1SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_RCGC0_ADC1SPD_A {
    type Ux = u8;
}
impl SYSCTL_RCGC0_ADC1SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCTL_RCGC0_ADC1SPD_A {
        match self.bits {
            0 => SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_125K,
            1 => SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_250K,
            2 => SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_500K,
            3 => SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc1spd_125k(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_125K
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc1spd_250k(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_250K
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc1spd_500k(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_500K
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn is_sysctl_rcgc0_adc1spd_1m(&self) -> bool {
        *self == SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_1M
    }
}
#[doc = "Field `SYSCTL_RCGC0_ADC1SPD` writer - ADC1 Sample Speed"]
pub type SYSCTL_RCGC0_ADC1SPD_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, SYSCTL_RCGC0_ADC1SPD_A>;
impl<'a, REG, const O: u8> SYSCTL_RCGC0_ADC1SPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc1spd_125k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_125K)
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc1spd_250k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_250K)
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc1spd_500k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_500K)
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc1spd_1m(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCGC0_ADC1SPD_A::SYSCTL_RCGC0_ADC1SPD_1M)
    }
}
#[doc = "Field `SYSCTL_RCGC0_ADC0` reader - ADC0 Clock Gating Control"]
pub type SYSCTL_RCGC0_ADC0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_ADC0` writer - ADC0 Clock Gating Control"]
pub type SYSCTL_RCGC0_ADC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_ADC1` reader - ADC1 Clock Gating Control"]
pub type SYSCTL_RCGC0_ADC1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_ADC1` writer - ADC1 Clock Gating Control"]
pub type SYSCTL_RCGC0_ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_PWM0` reader - PWM Clock Gating Control"]
pub type SYSCTL_RCGC0_PWM0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_PWM0` writer - PWM Clock Gating Control"]
pub type SYSCTL_RCGC0_PWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_CAN0` reader - CAN0 Clock Gating Control"]
pub type SYSCTL_RCGC0_CAN0_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_CAN0` writer - CAN0 Clock Gating Control"]
pub type SYSCTL_RCGC0_CAN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_CAN1` reader - CAN1 Clock Gating Control"]
pub type SYSCTL_RCGC0_CAN1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_CAN1` writer - CAN1 Clock Gating Control"]
pub type SYSCTL_RCGC0_CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCGC0_WDT1` reader - WDT1 Clock Gating Control"]
pub type SYSCTL_RCGC0_WDT1_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCGC0_WDT1` writer - WDT1 Clock Gating Control"]
pub type SYSCTL_RCGC0_WDT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - WDT0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_wdt0(&self) -> SYSCTL_RCGC0_WDT0_R {
        SYSCTL_RCGC0_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_hib(&self) -> SYSCTL_RCGC0_HIB_R {
        SYSCTL_RCGC0_HIB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ADC0 Sample Speed"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc0spd(&self) -> SYSCTL_RCGC0_ADC0SPD_R {
        SYSCTL_RCGC0_ADC0SPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - ADC1 Sample Speed"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc1spd(&self) -> SYSCTL_RCGC0_ADC1SPD_R {
        SYSCTL_RCGC0_ADC1SPD_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - ADC0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc0(&self) -> SYSCTL_RCGC0_ADC0_R {
        SYSCTL_RCGC0_ADC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_adc1(&self) -> SYSCTL_RCGC0_ADC1_R {
        SYSCTL_RCGC0_ADC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_pwm0(&self) -> SYSCTL_RCGC0_PWM0_R {
        SYSCTL_RCGC0_PWM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - CAN0 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_can0(&self) -> SYSCTL_RCGC0_CAN0_R {
        SYSCTL_RCGC0_CAN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_can1(&self) -> SYSCTL_RCGC0_CAN1_R {
        SYSCTL_RCGC0_CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - WDT1 Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_rcgc0_wdt1(&self) -> SYSCTL_RCGC0_WDT1_R {
        SYSCTL_RCGC0_WDT1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - WDT0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_wdt0(&mut self) -> SYSCTL_RCGC0_WDT0_W<RCGC0_SPEC, 3> {
        SYSCTL_RCGC0_WDT0_W::new(self)
    }
    #[doc = "Bit 6 - HIB Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_hib(&mut self) -> SYSCTL_RCGC0_HIB_W<RCGC0_SPEC, 6> {
        SYSCTL_RCGC0_HIB_W::new(self)
    }
    #[doc = "Bits 8:9 - ADC0 Sample Speed"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_adc0spd(&mut self) -> SYSCTL_RCGC0_ADC0SPD_W<RCGC0_SPEC, 8> {
        SYSCTL_RCGC0_ADC0SPD_W::new(self)
    }
    #[doc = "Bits 10:11 - ADC1 Sample Speed"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_adc1spd(&mut self) -> SYSCTL_RCGC0_ADC1SPD_W<RCGC0_SPEC, 10> {
        SYSCTL_RCGC0_ADC1SPD_W::new(self)
    }
    #[doc = "Bit 16 - ADC0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_adc0(&mut self) -> SYSCTL_RCGC0_ADC0_W<RCGC0_SPEC, 16> {
        SYSCTL_RCGC0_ADC0_W::new(self)
    }
    #[doc = "Bit 17 - ADC1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_adc1(&mut self) -> SYSCTL_RCGC0_ADC1_W<RCGC0_SPEC, 17> {
        SYSCTL_RCGC0_ADC1_W::new(self)
    }
    #[doc = "Bit 20 - PWM Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_pwm0(&mut self) -> SYSCTL_RCGC0_PWM0_W<RCGC0_SPEC, 20> {
        SYSCTL_RCGC0_PWM0_W::new(self)
    }
    #[doc = "Bit 24 - CAN0 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_can0(&mut self) -> SYSCTL_RCGC0_CAN0_W<RCGC0_SPEC, 24> {
        SYSCTL_RCGC0_CAN0_W::new(self)
    }
    #[doc = "Bit 25 - CAN1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_can1(&mut self) -> SYSCTL_RCGC0_CAN1_W<RCGC0_SPEC, 25> {
        SYSCTL_RCGC0_CAN1_W::new(self)
    }
    #[doc = "Bit 28 - WDT1 Clock Gating Control"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcgc0_wdt1(&mut self) -> SYSCTL_RCGC0_WDT1_W<RCGC0_SPEC, 28> {
        SYSCTL_RCGC0_WDT1_W::new(self)
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
#[doc = "Run Mode Clock Gating Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcgc0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcgc0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCGC0_SPEC;
impl crate::RegisterSpec for RCGC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcgc0::R`](R) reader structure"]
impl crate::Readable for RCGC0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcgc0::W`](W) writer structure"]
impl crate::Writable for RCGC0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCGC0 to value 0"]
impl crate::Resettable for RCGC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
