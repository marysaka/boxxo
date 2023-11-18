#[doc = "Register `DC1` reader"]
pub type R = crate::R<DC1_SPEC>;
#[doc = "Register `DC1` writer"]
pub type W = crate::W<DC1_SPEC>;
#[doc = "Field `SYSCTL_DC1_JTAG` reader - JTAG Present"]
pub type SYSCTL_DC1_JTAG_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_JTAG` writer - JTAG Present"]
pub type SYSCTL_DC1_JTAG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_SWD` reader - SWD Present"]
pub type SYSCTL_DC1_SWD_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_SWD` writer - SWD Present"]
pub type SYSCTL_DC1_SWD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_SWO` reader - SWO Trace Port Present"]
pub type SYSCTL_DC1_SWO_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_SWO` writer - SWO Trace Port Present"]
pub type SYSCTL_DC1_SWO_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_WDT0` reader - Watchdog Timer 0 Present"]
pub type SYSCTL_DC1_WDT0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_WDT0` writer - Watchdog Timer 0 Present"]
pub type SYSCTL_DC1_WDT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_PLL` reader - PLL Present"]
pub type SYSCTL_DC1_PLL_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_PLL` writer - PLL Present"]
pub type SYSCTL_DC1_PLL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_TEMP` reader - Temp Sensor Present"]
pub type SYSCTL_DC1_TEMP_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_TEMP` writer - Temp Sensor Present"]
pub type SYSCTL_DC1_TEMP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_HIB` reader - Hibernation Module Present"]
pub type SYSCTL_DC1_HIB_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_HIB` writer - Hibernation Module Present"]
pub type SYSCTL_DC1_HIB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_MPU` reader - MPU Present"]
pub type SYSCTL_DC1_MPU_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_MPU` writer - MPU Present"]
pub type SYSCTL_DC1_MPU_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_ADC0SPD` reader - Max ADC0 Speed"]
pub type SYSCTL_DC1_ADC0SPD_R = crate::FieldReader<SYSCTL_DC1_ADC0SPD_A>;
#[doc = "Max ADC0 Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DC1_ADC0SPD_A {
    #[doc = "0: 125K samples/second"]
    SYSCTL_DC1_ADC0SPD_125K = 0,
    #[doc = "1: 250K samples/second"]
    SYSCTL_DC1_ADC0SPD_250K = 1,
    #[doc = "2: 500K samples/second"]
    SYSCTL_DC1_ADC0SPD_500K = 2,
    #[doc = "3: 1M samples/second"]
    SYSCTL_DC1_ADC0SPD_1M = 3,
}
impl From<SYSCTL_DC1_ADC0SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC1_ADC0SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DC1_ADC0SPD_A {
    type Ux = u8;
}
impl SYSCTL_DC1_ADC0SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCTL_DC1_ADC0SPD_A {
        match self.bits {
            0 => SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_125K,
            1 => SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_250K,
            2 => SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_500K,
            3 => SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc0spd_125k(&self) -> bool {
        *self == SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_125K
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc0spd_250k(&self) -> bool {
        *self == SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_250K
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc0spd_500k(&self) -> bool {
        *self == SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_500K
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc0spd_1m(&self) -> bool {
        *self == SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_1M
    }
}
#[doc = "Field `SYSCTL_DC1_ADC0SPD` writer - Max ADC0 Speed"]
pub type SYSCTL_DC1_ADC0SPD_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, SYSCTL_DC1_ADC0SPD_A>;
impl<'a, REG, const O: u8> SYSCTL_DC1_ADC0SPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc0spd_125k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_125K)
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc0spd_250k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_250K)
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc0spd_500k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_500K)
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc0spd_1m(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC0SPD_A::SYSCTL_DC1_ADC0SPD_1M)
    }
}
#[doc = "Field `SYSCTL_DC1_ADC1SPD` reader - Max ADC1 Speed"]
pub type SYSCTL_DC1_ADC1SPD_R = crate::FieldReader<SYSCTL_DC1_ADC1SPD_A>;
#[doc = "Max ADC1 Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DC1_ADC1SPD_A {
    #[doc = "0: 125K samples/second"]
    SYSCTL_DC1_ADC1SPD_125K = 0,
    #[doc = "1: 250K samples/second"]
    SYSCTL_DC1_ADC1SPD_250K = 1,
    #[doc = "2: 500K samples/second"]
    SYSCTL_DC1_ADC1SPD_500K = 2,
    #[doc = "3: 1M samples/second"]
    SYSCTL_DC1_ADC1SPD_1M = 3,
}
impl From<SYSCTL_DC1_ADC1SPD_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC1_ADC1SPD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DC1_ADC1SPD_A {
    type Ux = u8;
}
impl SYSCTL_DC1_ADC1SPD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCTL_DC1_ADC1SPD_A {
        match self.bits {
            0 => SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_125K,
            1 => SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_250K,
            2 => SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_500K,
            3 => SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_1M,
            _ => unreachable!(),
        }
    }
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc1spd_125k(&self) -> bool {
        *self == SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_125K
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc1spd_250k(&self) -> bool {
        *self == SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_250K
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc1spd_500k(&self) -> bool {
        *self == SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_500K
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn is_sysctl_dc1_adc1spd_1m(&self) -> bool {
        *self == SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_1M
    }
}
#[doc = "Field `SYSCTL_DC1_ADC1SPD` writer - Max ADC1 Speed"]
pub type SYSCTL_DC1_ADC1SPD_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, SYSCTL_DC1_ADC1SPD_A>;
impl<'a, REG, const O: u8> SYSCTL_DC1_ADC1SPD_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125K samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc1spd_125k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_125K)
    }
    #[doc = "250K samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc1spd_250k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_250K)
    }
    #[doc = "500K samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc1spd_500k(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_500K)
    }
    #[doc = "1M samples/second"]
    #[inline(always)]
    pub fn sysctl_dc1_adc1spd_1m(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_ADC1SPD_A::SYSCTL_DC1_ADC1SPD_1M)
    }
}
#[doc = "Field `SYSCTL_DC1_MINSYSDIV` reader - System Clock Divider"]
pub type SYSCTL_DC1_MINSYSDIV_R = crate::FieldReader<SYSCTL_DC1_MINSYSDIV_A>;
#[doc = "System Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_DC1_MINSYSDIV_A {
    #[doc = "1: Divide VCO (400MHZ) by 5 minimum"]
    SYSCTL_DC1_MINSYSDIV_100 = 1,
    #[doc = "2: Divide VCO (400MHZ) by 2*2 + 2 = 6 minimum"]
    SYSCTL_DC1_MINSYSDIV_66 = 2,
    #[doc = "3: Specifies a 50-MHz CPU clock with a PLL divider of 4"]
    SYSCTL_DC1_MINSYSDIV_50 = 3,
    #[doc = "4: Specifies a 40-MHz CPU clock with a PLL divider of 5"]
    SYSCTL_DC1_MINSYSDIV_40 = 4,
    #[doc = "7: Specifies a 25-MHz clock with a PLL divider of 8"]
    SYSCTL_DC1_MINSYSDIV_25 = 7,
    #[doc = "9: Specifies a 20-MHz clock with a PLL divider of 10"]
    SYSCTL_DC1_MINSYSDIV_20 = 9,
}
impl From<SYSCTL_DC1_MINSYSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_DC1_MINSYSDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_DC1_MINSYSDIV_A {
    type Ux = u8;
}
impl SYSCTL_DC1_MINSYSDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_DC1_MINSYSDIV_A> {
        match self.bits {
            1 => Some(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_100),
            2 => Some(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_66),
            3 => Some(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_50),
            4 => Some(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_40),
            7 => Some(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_25),
            9 => Some(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_20),
            _ => None,
        }
    }
    #[doc = "Divide VCO (400MHZ) by 5 minimum"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_100(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_100
    }
    #[doc = "Divide VCO (400MHZ) by 2*2 + 2 = 6 minimum"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_66(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_66
    }
    #[doc = "Specifies a 50-MHz CPU clock with a PLL divider of 4"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_50(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_50
    }
    #[doc = "Specifies a 40-MHz CPU clock with a PLL divider of 5"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_40(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_40
    }
    #[doc = "Specifies a 25-MHz clock with a PLL divider of 8"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_25(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_25
    }
    #[doc = "Specifies a 20-MHz clock with a PLL divider of 10"]
    #[inline(always)]
    pub fn is_sysctl_dc1_minsysdiv_20(&self) -> bool {
        *self == SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_20
    }
}
#[doc = "Field `SYSCTL_DC1_MINSYSDIV` writer - System Clock Divider"]
pub type SYSCTL_DC1_MINSYSDIV_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 4, O, SYSCTL_DC1_MINSYSDIV_A>;
impl<'a, REG, const O: u8> SYSCTL_DC1_MINSYSDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide VCO (400MHZ) by 5 minimum"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_100(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_100)
    }
    #[doc = "Divide VCO (400MHZ) by 2*2 + 2 = 6 minimum"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_66(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_66)
    }
    #[doc = "Specifies a 50-MHz CPU clock with a PLL divider of 4"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_50(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_50)
    }
    #[doc = "Specifies a 40-MHz CPU clock with a PLL divider of 5"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_40(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_40)
    }
    #[doc = "Specifies a 25-MHz clock with a PLL divider of 8"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_25(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_25)
    }
    #[doc = "Specifies a 20-MHz clock with a PLL divider of 10"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv_20(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_DC1_MINSYSDIV_A::SYSCTL_DC1_MINSYSDIV_20)
    }
}
#[doc = "Field `SYSCTL_DC1_ADC0` reader - ADC Module 0 Present"]
pub type SYSCTL_DC1_ADC0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_ADC0` writer - ADC Module 0 Present"]
pub type SYSCTL_DC1_ADC0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_ADC1` reader - ADC Module 1 Present"]
pub type SYSCTL_DC1_ADC1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_ADC1` writer - ADC Module 1 Present"]
pub type SYSCTL_DC1_ADC1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_PWM0` reader - PWM Module 0 Present"]
pub type SYSCTL_DC1_PWM0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_PWM0` writer - PWM Module 0 Present"]
pub type SYSCTL_DC1_PWM0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_PWM1` reader - PWM Module 1 Present"]
pub type SYSCTL_DC1_PWM1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_PWM1` writer - PWM Module 1 Present"]
pub type SYSCTL_DC1_PWM1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_CAN0` reader - CAN Module 0 Present"]
pub type SYSCTL_DC1_CAN0_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_CAN0` writer - CAN Module 0 Present"]
pub type SYSCTL_DC1_CAN0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_CAN1` reader - CAN Module 1 Present"]
pub type SYSCTL_DC1_CAN1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_CAN1` writer - CAN Module 1 Present"]
pub type SYSCTL_DC1_CAN1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_DC1_WDT1` reader - Watchdog Timer1 Present"]
pub type SYSCTL_DC1_WDT1_R = crate::BitReader;
#[doc = "Field `SYSCTL_DC1_WDT1` writer - Watchdog Timer1 Present"]
pub type SYSCTL_DC1_WDT1_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - JTAG Present"]
    #[inline(always)]
    pub fn sysctl_dc1_jtag(&self) -> SYSCTL_DC1_JTAG_R {
        SYSCTL_DC1_JTAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SWD Present"]
    #[inline(always)]
    pub fn sysctl_dc1_swd(&self) -> SYSCTL_DC1_SWD_R {
        SYSCTL_DC1_SWD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWO Trace Port Present"]
    #[inline(always)]
    pub fn sysctl_dc1_swo(&self) -> SYSCTL_DC1_SWO_R {
        SYSCTL_DC1_SWO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_wdt0(&self) -> SYSCTL_DC1_WDT0_R {
        SYSCTL_DC1_WDT0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL Present"]
    #[inline(always)]
    pub fn sysctl_dc1_pll(&self) -> SYSCTL_DC1_PLL_R {
        SYSCTL_DC1_PLL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Temp Sensor Present"]
    #[inline(always)]
    pub fn sysctl_dc1_temp(&self) -> SYSCTL_DC1_TEMP_R {
        SYSCTL_DC1_TEMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hibernation Module Present"]
    #[inline(always)]
    pub fn sysctl_dc1_hib(&self) -> SYSCTL_DC1_HIB_R {
        SYSCTL_DC1_HIB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPU Present"]
    #[inline(always)]
    pub fn sysctl_dc1_mpu(&self) -> SYSCTL_DC1_MPU_R {
        SYSCTL_DC1_MPU_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Max ADC0 Speed"]
    #[inline(always)]
    pub fn sysctl_dc1_adc0spd(&self) -> SYSCTL_DC1_ADC0SPD_R {
        SYSCTL_DC1_ADC0SPD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Max ADC1 Speed"]
    #[inline(always)]
    pub fn sysctl_dc1_adc1spd(&self) -> SYSCTL_DC1_ADC1SPD_R {
        SYSCTL_DC1_ADC1SPD_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - System Clock Divider"]
    #[inline(always)]
    pub fn sysctl_dc1_minsysdiv(&self) -> SYSCTL_DC1_MINSYSDIV_R {
        SYSCTL_DC1_MINSYSDIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ADC Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_adc0(&self) -> SYSCTL_DC1_ADC0_R {
        SYSCTL_DC1_ADC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_adc1(&self) -> SYSCTL_DC1_ADC1_R {
        SYSCTL_DC1_ADC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - PWM Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_pwm0(&self) -> SYSCTL_DC1_PWM0_R {
        SYSCTL_DC1_PWM0_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PWM Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_pwm1(&self) -> SYSCTL_DC1_PWM1_R {
        SYSCTL_DC1_PWM1_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - CAN Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_can0(&self) -> SYSCTL_DC1_CAN0_R {
        SYSCTL_DC1_CAN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CAN Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_can1(&self) -> SYSCTL_DC1_CAN1_R {
        SYSCTL_DC1_CAN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Watchdog Timer1 Present"]
    #[inline(always)]
    pub fn sysctl_dc1_wdt1(&self) -> SYSCTL_DC1_WDT1_R {
        SYSCTL_DC1_WDT1_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JTAG Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_jtag(&mut self) -> SYSCTL_DC1_JTAG_W<DC1_SPEC, 0> {
        SYSCTL_DC1_JTAG_W::new(self)
    }
    #[doc = "Bit 1 - SWD Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_swd(&mut self) -> SYSCTL_DC1_SWD_W<DC1_SPEC, 1> {
        SYSCTL_DC1_SWD_W::new(self)
    }
    #[doc = "Bit 2 - SWO Trace Port Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_swo(&mut self) -> SYSCTL_DC1_SWO_W<DC1_SPEC, 2> {
        SYSCTL_DC1_SWO_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog Timer 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_wdt0(&mut self) -> SYSCTL_DC1_WDT0_W<DC1_SPEC, 3> {
        SYSCTL_DC1_WDT0_W::new(self)
    }
    #[doc = "Bit 4 - PLL Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_pll(&mut self) -> SYSCTL_DC1_PLL_W<DC1_SPEC, 4> {
        SYSCTL_DC1_PLL_W::new(self)
    }
    #[doc = "Bit 5 - Temp Sensor Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_temp(&mut self) -> SYSCTL_DC1_TEMP_W<DC1_SPEC, 5> {
        SYSCTL_DC1_TEMP_W::new(self)
    }
    #[doc = "Bit 6 - Hibernation Module Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_hib(&mut self) -> SYSCTL_DC1_HIB_W<DC1_SPEC, 6> {
        SYSCTL_DC1_HIB_W::new(self)
    }
    #[doc = "Bit 7 - MPU Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_mpu(&mut self) -> SYSCTL_DC1_MPU_W<DC1_SPEC, 7> {
        SYSCTL_DC1_MPU_W::new(self)
    }
    #[doc = "Bits 8:9 - Max ADC0 Speed"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_adc0spd(&mut self) -> SYSCTL_DC1_ADC0SPD_W<DC1_SPEC, 8> {
        SYSCTL_DC1_ADC0SPD_W::new(self)
    }
    #[doc = "Bits 10:11 - Max ADC1 Speed"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_adc1spd(&mut self) -> SYSCTL_DC1_ADC1SPD_W<DC1_SPEC, 10> {
        SYSCTL_DC1_ADC1SPD_W::new(self)
    }
    #[doc = "Bits 12:15 - System Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_minsysdiv(&mut self) -> SYSCTL_DC1_MINSYSDIV_W<DC1_SPEC, 12> {
        SYSCTL_DC1_MINSYSDIV_W::new(self)
    }
    #[doc = "Bit 16 - ADC Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_adc0(&mut self) -> SYSCTL_DC1_ADC0_W<DC1_SPEC, 16> {
        SYSCTL_DC1_ADC0_W::new(self)
    }
    #[doc = "Bit 17 - ADC Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_adc1(&mut self) -> SYSCTL_DC1_ADC1_W<DC1_SPEC, 17> {
        SYSCTL_DC1_ADC1_W::new(self)
    }
    #[doc = "Bit 20 - PWM Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_pwm0(&mut self) -> SYSCTL_DC1_PWM0_W<DC1_SPEC, 20> {
        SYSCTL_DC1_PWM0_W::new(self)
    }
    #[doc = "Bit 21 - PWM Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_pwm1(&mut self) -> SYSCTL_DC1_PWM1_W<DC1_SPEC, 21> {
        SYSCTL_DC1_PWM1_W::new(self)
    }
    #[doc = "Bit 24 - CAN Module 0 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_can0(&mut self) -> SYSCTL_DC1_CAN0_W<DC1_SPEC, 24> {
        SYSCTL_DC1_CAN0_W::new(self)
    }
    #[doc = "Bit 25 - CAN Module 1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_can1(&mut self) -> SYSCTL_DC1_CAN1_W<DC1_SPEC, 25> {
        SYSCTL_DC1_CAN1_W::new(self)
    }
    #[doc = "Bit 28 - Watchdog Timer1 Present"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_dc1_wdt1(&mut self) -> SYSCTL_DC1_WDT1_W<DC1_SPEC, 28> {
        SYSCTL_DC1_WDT1_W::new(self)
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
#[doc = "Device Capabilities 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DC1_SPEC;
impl crate::RegisterSpec for DC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc1::R`](R) reader structure"]
impl crate::Readable for DC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dc1::W`](W) writer structure"]
impl crate::Writable for DC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DC1 to value 0"]
impl crate::Resettable for DC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
