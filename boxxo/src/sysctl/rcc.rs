#[doc = "Register `RCC` reader"]
pub type R = crate::R<RCC_SPEC>;
#[doc = "Register `RCC` writer"]
pub type W = crate::W<RCC_SPEC>;
#[doc = "Field `SYSCTL_RCC_MOSCDIS` reader - Main Oscillator Disable"]
pub type SYSCTL_RCC_MOSCDIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC_MOSCDIS` writer - Main Oscillator Disable"]
pub type SYSCTL_RCC_MOSCDIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC_OSCSRC` reader - Oscillator Source"]
pub type SYSCTL_RCC_OSCSRC_R = crate::FieldReader<SYSCTL_RCC_OSCSRC_A>;
#[doc = "Oscillator Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_RCC_OSCSRC_A {
    #[doc = "0: MOSC"]
    SYSCTL_RCC_OSCSRC_MAIN = 0,
    #[doc = "1: IOSC"]
    SYSCTL_RCC_OSCSRC_INT = 1,
    #[doc = "2: IOSC/4"]
    SYSCTL_RCC_OSCSRC_INT4 = 2,
    #[doc = "3: 30 kHz"]
    SYSCTL_RCC_OSCSRC_30 = 3,
}
impl From<SYSCTL_RCC_OSCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_OSCSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_RCC_OSCSRC_A {
    type Ux = u8;
}
impl SYSCTL_RCC_OSCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SYSCTL_RCC_OSCSRC_A {
        match self.bits {
            0 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_MAIN,
            1 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT,
            2 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT4,
            3 => SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_30,
            _ => unreachable!(),
        }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_main(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_MAIN
    }
    #[doc = "IOSC"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_int(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT
    }
    #[doc = "IOSC/4"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_int4(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT4
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_oscsrc_30(&self) -> bool {
        *self == SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_30
    }
}
#[doc = "Field `SYSCTL_RCC_OSCSRC` writer - Oscillator Source"]
pub type SYSCTL_RCC_OSCSRC_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, SYSCTL_RCC_OSCSRC_A>;
impl<'a, REG, const O: u8> SYSCTL_RCC_OSCSRC_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_main(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_MAIN)
    }
    #[doc = "IOSC"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_int(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT)
    }
    #[doc = "IOSC/4"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_int4(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_INT4)
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc_30(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_OSCSRC_A::SYSCTL_RCC_OSCSRC_30)
    }
}
#[doc = "Field `SYSCTL_RCC_XTAL` reader - Crystal Value"]
pub type SYSCTL_RCC_XTAL_R = crate::FieldReader<SYSCTL_RCC_XTAL_A>;
#[doc = "Crystal Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_RCC_XTAL_A {
    #[doc = "6: 4 MHz"]
    SYSCTL_RCC_XTAL_4MHZ = 6,
    #[doc = "7: 4.096 MHz"]
    SYSCTL_RCC_XTAL_4_09MHZ = 7,
    #[doc = "8: 4.9152 MHz"]
    SYSCTL_RCC_XTAL_4_91MHZ = 8,
    #[doc = "9: 5 MHz"]
    SYSCTL_RCC_XTAL_5MHZ = 9,
    #[doc = "10: 5.12 MHz"]
    SYSCTL_RCC_XTAL_5_12MHZ = 10,
    #[doc = "11: 6 MHz"]
    SYSCTL_RCC_XTAL_6MHZ = 11,
    #[doc = "12: 6.144 MHz"]
    SYSCTL_RCC_XTAL_6_14MHZ = 12,
    #[doc = "13: 7.3728 MHz"]
    SYSCTL_RCC_XTAL_7_37MHZ = 13,
    #[doc = "14: 8 MHz"]
    SYSCTL_RCC_XTAL_8MHZ = 14,
    #[doc = "15: 8.192 MHz"]
    SYSCTL_RCC_XTAL_8_19MHZ = 15,
    #[doc = "16: 10 MHz"]
    SYSCTL_RCC_XTAL_10MHZ = 16,
    #[doc = "17: 12 MHz"]
    SYSCTL_RCC_XTAL_12MHZ = 17,
    #[doc = "18: 12.288 MHz"]
    SYSCTL_RCC_XTAL_12_2MHZ = 18,
    #[doc = "19: 13.56 MHz"]
    SYSCTL_RCC_XTAL_13_5MHZ = 19,
    #[doc = "20: 14.31818 MHz"]
    SYSCTL_RCC_XTAL_14_3MHZ = 20,
    #[doc = "21: 16 MHz"]
    SYSCTL_RCC_XTAL_16MHZ = 21,
    #[doc = "22: 16.384 MHz"]
    SYSCTL_RCC_XTAL_16_3MHZ = 22,
    #[doc = "23: 18.0 MHz"]
    SYSCTL_RCC_XTAL_18MHZ = 23,
    #[doc = "24: 20.0 MHz"]
    SYSCTL_RCC_XTAL_20MHZ = 24,
    #[doc = "25: 24.0 MHz"]
    SYSCTL_RCC_XTAL_24MHZ = 25,
    #[doc = "26: 25.0 MHz"]
    SYSCTL_RCC_XTAL_25MHZ = 26,
}
impl From<SYSCTL_RCC_XTAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_XTAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_RCC_XTAL_A {
    type Ux = u8;
}
impl SYSCTL_RCC_XTAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_RCC_XTAL_A> {
        match self.bits {
            6 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4MHZ),
            7 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_09MHZ),
            8 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_91MHZ),
            9 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5MHZ),
            10 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5_12MHZ),
            11 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6MHZ),
            12 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6_14MHZ),
            13 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_7_37MHZ),
            14 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8MHZ),
            15 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8_19MHZ),
            16 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_10MHZ),
            17 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_12MHZ),
            18 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_12_2MHZ),
            19 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_13_5MHZ),
            20 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_14_3MHZ),
            21 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_16MHZ),
            22 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_16_3MHZ),
            23 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_18MHZ),
            24 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_20MHZ),
            25 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_24MHZ),
            26 => Some(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_25MHZ),
            _ => None,
        }
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_4mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4MHZ
    }
    #[doc = "4.096 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_4_09mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_09MHZ
    }
    #[doc = "4.9152 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_4_91mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_91MHZ
    }
    #[doc = "5 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_5mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5MHZ
    }
    #[doc = "5.12 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_5_12mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5_12MHZ
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_6mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6MHZ
    }
    #[doc = "6.144 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_6_14mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6_14MHZ
    }
    #[doc = "7.3728 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_7_37mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_7_37MHZ
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_8mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8MHZ
    }
    #[doc = "8.192 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_8_19mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8_19MHZ
    }
    #[doc = "10 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_10mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_10MHZ
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_12mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_12MHZ
    }
    #[doc = "12.288 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_12_2mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_12_2MHZ
    }
    #[doc = "13.56 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_13_5mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_13_5MHZ
    }
    #[doc = "14.31818 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_14_3mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_14_3MHZ
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_16mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_16MHZ
    }
    #[doc = "16.384 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_16_3mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_16_3MHZ
    }
    #[doc = "18.0 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_18mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_18MHZ
    }
    #[doc = "20.0 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_20mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_20MHZ
    }
    #[doc = "24.0 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_24mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_24MHZ
    }
    #[doc = "25.0 MHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc_xtal_25mhz(&self) -> bool {
        *self == SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_25MHZ
    }
}
#[doc = "Field `SYSCTL_RCC_XTAL` writer - Crystal Value"]
pub type SYSCTL_RCC_XTAL_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 5, O, SYSCTL_RCC_XTAL_A>;
impl<'a, REG, const O: u8> SYSCTL_RCC_XTAL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4MHZ)
    }
    #[doc = "4.096 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_4_09mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_09MHZ)
    }
    #[doc = "4.9152 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_4_91mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_4_91MHZ)
    }
    #[doc = "5 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_5mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5MHZ)
    }
    #[doc = "5.12 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_5_12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_5_12MHZ)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_6mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6MHZ)
    }
    #[doc = "6.144 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_6_14mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_6_14MHZ)
    }
    #[doc = "7.3728 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_7_37mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_7_37MHZ)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_8mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8MHZ)
    }
    #[doc = "8.192 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_8_19mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_8_19MHZ)
    }
    #[doc = "10 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_10mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_10MHZ)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_12mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_12MHZ)
    }
    #[doc = "12.288 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_12_2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_12_2MHZ)
    }
    #[doc = "13.56 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_13_5mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_13_5MHZ)
    }
    #[doc = "14.31818 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_14_3mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_14_3MHZ)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_16mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_16MHZ)
    }
    #[doc = "16.384 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_16_3mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_16_3MHZ)
    }
    #[doc = "18.0 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_18mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_18MHZ)
    }
    #[doc = "20.0 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_20mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_20MHZ)
    }
    #[doc = "24.0 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_24mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_24MHZ)
    }
    #[doc = "25.0 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal_25mhz(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_XTAL_A::SYSCTL_RCC_XTAL_25MHZ)
    }
}
#[doc = "Field `SYSCTL_RCC_BYPASS` reader - PLL Bypass"]
pub type SYSCTL_RCC_BYPASS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC_BYPASS` writer - PLL Bypass"]
pub type SYSCTL_RCC_BYPASS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC_PWRDN` reader - PLL Power Down"]
pub type SYSCTL_RCC_PWRDN_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC_PWRDN` writer - PLL Power Down"]
pub type SYSCTL_RCC_PWRDN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC_PWMDIV` reader - PWM Unit Clock Divisor"]
pub type SYSCTL_RCC_PWMDIV_R = crate::FieldReader<SYSCTL_RCC_PWMDIV_A>;
#[doc = "PWM Unit Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_RCC_PWMDIV_A {
    #[doc = "0: PWM clock /2"]
    SYSCTL_RCC_PWMDIV_2 = 0,
    #[doc = "1: PWM clock /4"]
    SYSCTL_RCC_PWMDIV_4 = 1,
    #[doc = "2: PWM clock /8"]
    SYSCTL_RCC_PWMDIV_8 = 2,
    #[doc = "3: PWM clock /16"]
    SYSCTL_RCC_PWMDIV_16 = 3,
    #[doc = "4: PWM clock /32"]
    SYSCTL_RCC_PWMDIV_32 = 4,
    #[doc = "5: PWM clock /64"]
    SYSCTL_RCC_PWMDIV_64 = 5,
}
impl From<SYSCTL_RCC_PWMDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC_PWMDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_RCC_PWMDIV_A {
    type Ux = u8;
}
impl SYSCTL_RCC_PWMDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_RCC_PWMDIV_A> {
        match self.bits {
            0 => Some(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_2),
            1 => Some(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_4),
            2 => Some(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_8),
            3 => Some(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_16),
            4 => Some(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_32),
            5 => Some(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_64),
            _ => None,
        }
    }
    #[doc = "PWM clock /2"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_2(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_2
    }
    #[doc = "PWM clock /4"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_4(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_4
    }
    #[doc = "PWM clock /8"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_8(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_8
    }
    #[doc = "PWM clock /16"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_16(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_16
    }
    #[doc = "PWM clock /32"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_32(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_32
    }
    #[doc = "PWM clock /64"]
    #[inline(always)]
    pub fn is_sysctl_rcc_pwmdiv_64(&self) -> bool {
        *self == SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_64
    }
}
#[doc = "Field `SYSCTL_RCC_PWMDIV` writer - PWM Unit Clock Divisor"]
pub type SYSCTL_RCC_PWMDIV_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, SYSCTL_RCC_PWMDIV_A>;
impl<'a, REG, const O: u8> SYSCTL_RCC_PWMDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWM clock /2"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_2(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_2)
    }
    #[doc = "PWM clock /4"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_4(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_4)
    }
    #[doc = "PWM clock /8"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_8(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_8)
    }
    #[doc = "PWM clock /16"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_16(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_16)
    }
    #[doc = "PWM clock /32"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_32(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_32)
    }
    #[doc = "PWM clock /64"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv_64(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC_PWMDIV_A::SYSCTL_RCC_PWMDIV_64)
    }
}
#[doc = "Field `SYSCTL_RCC_USEPWMDIV` reader - Enable PWM Clock Divisor"]
pub type SYSCTL_RCC_USEPWMDIV_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC_USEPWMDIV` writer - Enable PWM Clock Divisor"]
pub type SYSCTL_RCC_USEPWMDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC_USESYSDIV` reader - Enable System Clock Divider"]
pub type SYSCTL_RCC_USESYSDIV_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC_USESYSDIV` writer - Enable System Clock Divider"]
pub type SYSCTL_RCC_USESYSDIV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC_SYSDIV` reader - System Clock Divisor"]
pub type SYSCTL_RCC_SYSDIV_R = crate::FieldReader;
#[doc = "Field `SYSCTL_RCC_SYSDIV` writer - System Clock Divisor"]
pub type SYSCTL_RCC_SYSDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SYSCTL_RCC_ACG` reader - Auto Clock Gating"]
pub type SYSCTL_RCC_ACG_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC_ACG` writer - Auto Clock Gating"]
pub type SYSCTL_RCC_ACG_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline(always)]
    pub fn sysctl_rcc_moscdis(&self) -> SYSCTL_RCC_MOSCDIS_R {
        SYSCTL_RCC_MOSCDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline(always)]
    pub fn sysctl_rcc_oscsrc(&self) -> SYSCTL_RCC_OSCSRC_R {
        SYSCTL_RCC_OSCSRC_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:10 - Crystal Value"]
    #[inline(always)]
    pub fn sysctl_rcc_xtal(&self) -> SYSCTL_RCC_XTAL_R {
        SYSCTL_RCC_XTAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline(always)]
    pub fn sysctl_rcc_bypass(&self) -> SYSCTL_RCC_BYPASS_R {
        SYSCTL_RCC_BYPASS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline(always)]
    pub fn sysctl_rcc_pwrdn(&self) -> SYSCTL_RCC_PWRDN_R {
        SYSCTL_RCC_PWRDN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_pwmdiv(&self) -> SYSCTL_RCC_PWMDIV_R {
        SYSCTL_RCC_PWMDIV_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_usepwmdiv(&self) -> SYSCTL_RCC_USEPWMDIV_R {
        SYSCTL_RCC_USEPWMDIV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline(always)]
    pub fn sysctl_rcc_usesysdiv(&self) -> SYSCTL_RCC_USESYSDIV_R {
        SYSCTL_RCC_USESYSDIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline(always)]
    pub fn sysctl_rcc_sysdiv(&self) -> SYSCTL_RCC_SYSDIV_R {
        SYSCTL_RCC_SYSDIV_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline(always)]
    pub fn sysctl_rcc_acg(&self) -> SYSCTL_RCC_ACG_R {
        SYSCTL_RCC_ACG_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Oscillator Disable"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_moscdis(&mut self) -> SYSCTL_RCC_MOSCDIS_W<RCC_SPEC, 0> {
        SYSCTL_RCC_MOSCDIS_W::new(self)
    }
    #[doc = "Bits 4:5 - Oscillator Source"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_oscsrc(&mut self) -> SYSCTL_RCC_OSCSRC_W<RCC_SPEC, 4> {
        SYSCTL_RCC_OSCSRC_W::new(self)
    }
    #[doc = "Bits 6:10 - Crystal Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_xtal(&mut self) -> SYSCTL_RCC_XTAL_W<RCC_SPEC, 6> {
        SYSCTL_RCC_XTAL_W::new(self)
    }
    #[doc = "Bit 11 - PLL Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_bypass(&mut self) -> SYSCTL_RCC_BYPASS_W<RCC_SPEC, 11> {
        SYSCTL_RCC_BYPASS_W::new(self)
    }
    #[doc = "Bit 13 - PLL Power Down"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_pwrdn(&mut self) -> SYSCTL_RCC_PWRDN_W<RCC_SPEC, 13> {
        SYSCTL_RCC_PWRDN_W::new(self)
    }
    #[doc = "Bits 17:19 - PWM Unit Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_pwmdiv(&mut self) -> SYSCTL_RCC_PWMDIV_W<RCC_SPEC, 17> {
        SYSCTL_RCC_PWMDIV_W::new(self)
    }
    #[doc = "Bit 20 - Enable PWM Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_usepwmdiv(&mut self) -> SYSCTL_RCC_USEPWMDIV_W<RCC_SPEC, 20> {
        SYSCTL_RCC_USEPWMDIV_W::new(self)
    }
    #[doc = "Bit 22 - Enable System Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_usesysdiv(&mut self) -> SYSCTL_RCC_USESYSDIV_W<RCC_SPEC, 22> {
        SYSCTL_RCC_USESYSDIV_W::new(self)
    }
    #[doc = "Bits 23:26 - System Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_sysdiv(&mut self) -> SYSCTL_RCC_SYSDIV_W<RCC_SPEC, 23> {
        SYSCTL_RCC_SYSDIV_W::new(self)
    }
    #[doc = "Bit 27 - Auto Clock Gating"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc_acg(&mut self) -> SYSCTL_RCC_ACG_W<RCC_SPEC, 27> {
        SYSCTL_RCC_ACG_W::new(self)
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
#[doc = "Run-Mode Clock Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_SPEC;
impl crate::RegisterSpec for RCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc::R`](R) reader structure"]
impl crate::Readable for RCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcc::W`](W) writer structure"]
impl crate::Writable for RCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC to value 0"]
impl crate::Resettable for RCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
