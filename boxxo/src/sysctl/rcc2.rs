#[doc = "Register `RCC2` reader"]
pub type R = crate::R<RCC2_SPEC>;
#[doc = "Register `RCC2` writer"]
pub type W = crate::W<RCC2_SPEC>;
#[doc = "Field `SYSCTL_RCC2_OSCSRC2` reader - Oscillator Source 2"]
pub type SYSCTL_RCC2_OSCSRC2_R = crate::FieldReader<SYSCTL_RCC2_OSCSRC2_A>;
#[doc = "Oscillator Source 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSCTL_RCC2_OSCSRC2_A {
    #[doc = "0: MOSC"]
    SYSCTL_RCC2_OSCSRC2_MO = 0,
    #[doc = "1: PIOSC"]
    SYSCTL_RCC2_OSCSRC2_IO = 1,
    #[doc = "2: PIOSC/4"]
    SYSCTL_RCC2_OSCSRC2_IO4 = 2,
    #[doc = "3: 30 kHz"]
    SYSCTL_RCC2_OSCSRC2_30 = 3,
    #[doc = "7: 32.768 kHz"]
    SYSCTL_RCC2_OSCSRC2_32 = 7,
}
impl From<SYSCTL_RCC2_OSCSRC2_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSCTL_RCC2_OSCSRC2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSCTL_RCC2_OSCSRC2_A {
    type Ux = u8;
}
impl SYSCTL_RCC2_OSCSRC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SYSCTL_RCC2_OSCSRC2_A> {
        match self.bits {
            0 => Some(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_MO),
            1 => Some(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO),
            2 => Some(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO4),
            3 => Some(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_30),
            7 => Some(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_32),
            _ => None,
        }
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_mo(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_MO
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_io(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO
    }
    #[doc = "PIOSC/4"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_io4(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO4
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_30(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_30
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn is_sysctl_rcc2_oscsrc2_32(&self) -> bool {
        *self == SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_32
    }
}
#[doc = "Field `SYSCTL_RCC2_OSCSRC2` writer - Oscillator Source 2"]
pub type SYSCTL_RCC2_OSCSRC2_W<'a, REG, const O: u8> =
    crate::FieldWriter<'a, REG, 3, O, SYSCTL_RCC2_OSCSRC2_A>;
impl<'a, REG, const O: u8> SYSCTL_RCC2_OSCSRC2_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_mo(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_MO)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_io(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO)
    }
    #[doc = "PIOSC/4"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_io4(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_IO4)
    }
    #[doc = "30 kHz"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_30(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_30)
    }
    #[doc = "32.768 kHz"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2_32(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCTL_RCC2_OSCSRC2_A::SYSCTL_RCC2_OSCSRC2_32)
    }
}
#[doc = "Field `SYSCTL_RCC2_BYPASS2` reader - PLL Bypass 2"]
pub type SYSCTL_RCC2_BYPASS2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC2_BYPASS2` writer - PLL Bypass 2"]
pub type SYSCTL_RCC2_BYPASS2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC2_PWRDN2` reader - Power-Down PLL 2"]
pub type SYSCTL_RCC2_PWRDN2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC2_PWRDN2` writer - Power-Down PLL 2"]
pub type SYSCTL_RCC2_PWRDN2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC2_USBPWRDN` reader - Power-Down USB PLL"]
pub type SYSCTL_RCC2_USBPWRDN_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC2_USBPWRDN` writer - Power-Down USB PLL"]
pub type SYSCTL_RCC2_USBPWRDN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC2_SYSDIV2LSB` reader - Additional LSB for SYSDIV2"]
pub type SYSCTL_RCC2_SYSDIV2LSB_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC2_SYSDIV2LSB` writer - Additional LSB for SYSDIV2"]
pub type SYSCTL_RCC2_SYSDIV2LSB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC2_SYSDIV2` reader - System Clock Divisor 2"]
pub type SYSCTL_RCC2_SYSDIV2_R = crate::FieldReader;
#[doc = "Field `SYSCTL_RCC2_SYSDIV2` writer - System Clock Divisor 2"]
pub type SYSCTL_RCC2_SYSDIV2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `SYSCTL_RCC2_DIV400` reader - Divide PLL as 400 MHz vs. 200 MHz"]
pub type SYSCTL_RCC2_DIV400_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC2_DIV400` writer - Divide PLL as 400 MHz vs. 200 MHz"]
pub type SYSCTL_RCC2_DIV400_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RCC2_USERCC2` reader - Use RCC2"]
pub type SYSCTL_RCC2_USERCC2_R = crate::BitReader;
#[doc = "Field `SYSCTL_RCC2_USERCC2` writer - Use RCC2"]
pub type SYSCTL_RCC2_USERCC2_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_oscsrc2(&self) -> SYSCTL_RCC2_OSCSRC2_R {
        SYSCTL_RCC2_OSCSRC2_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_bypass2(&self) -> SYSCTL_RCC2_BYPASS2_R {
        SYSCTL_RCC2_BYPASS2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_pwrdn2(&self) -> SYSCTL_RCC2_PWRDN2_R {
        SYSCTL_RCC2_PWRDN2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Power-Down USB PLL"]
    #[inline(always)]
    pub fn sysctl_rcc2_usbpwrdn(&self) -> SYSCTL_RCC2_USBPWRDN_R {
        SYSCTL_RCC2_USBPWRDN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - Additional LSB for SYSDIV2"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2lsb(&self) -> SYSCTL_RCC2_SYSDIV2LSB_R {
        SYSCTL_RCC2_SYSDIV2LSB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline(always)]
    pub fn sysctl_rcc2_sysdiv2(&self) -> SYSCTL_RCC2_SYSDIV2_R {
        SYSCTL_RCC2_SYSDIV2_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Divide PLL as 400 MHz vs. 200 MHz"]
    #[inline(always)]
    pub fn sysctl_rcc2_div400(&self) -> SYSCTL_RCC2_DIV400_R {
        SYSCTL_RCC2_DIV400_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline(always)]
    pub fn sysctl_rcc2_usercc2(&self) -> SYSCTL_RCC2_USERCC2_R {
        SYSCTL_RCC2_USERCC2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:6 - Oscillator Source 2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_oscsrc2(&mut self) -> SYSCTL_RCC2_OSCSRC2_W<RCC2_SPEC, 4> {
        SYSCTL_RCC2_OSCSRC2_W::new(self)
    }
    #[doc = "Bit 11 - PLL Bypass 2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_bypass2(&mut self) -> SYSCTL_RCC2_BYPASS2_W<RCC2_SPEC, 11> {
        SYSCTL_RCC2_BYPASS2_W::new(self)
    }
    #[doc = "Bit 13 - Power-Down PLL 2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_pwrdn2(&mut self) -> SYSCTL_RCC2_PWRDN2_W<RCC2_SPEC, 13> {
        SYSCTL_RCC2_PWRDN2_W::new(self)
    }
    #[doc = "Bit 14 - Power-Down USB PLL"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_usbpwrdn(&mut self) -> SYSCTL_RCC2_USBPWRDN_W<RCC2_SPEC, 14> {
        SYSCTL_RCC2_USBPWRDN_W::new(self)
    }
    #[doc = "Bit 22 - Additional LSB for SYSDIV2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_sysdiv2lsb(&mut self) -> SYSCTL_RCC2_SYSDIV2LSB_W<RCC2_SPEC, 22> {
        SYSCTL_RCC2_SYSDIV2LSB_W::new(self)
    }
    #[doc = "Bits 23:28 - System Clock Divisor 2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_sysdiv2(&mut self) -> SYSCTL_RCC2_SYSDIV2_W<RCC2_SPEC, 23> {
        SYSCTL_RCC2_SYSDIV2_W::new(self)
    }
    #[doc = "Bit 30 - Divide PLL as 400 MHz vs. 200 MHz"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_div400(&mut self) -> SYSCTL_RCC2_DIV400_W<RCC2_SPEC, 30> {
        SYSCTL_RCC2_DIV400_W::new(self)
    }
    #[doc = "Bit 31 - Use RCC2"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_rcc2_usercc2(&mut self) -> SYSCTL_RCC2_USERCC2_W<RCC2_SPEC, 31> {
        SYSCTL_RCC2_USERCC2_W::new(self)
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
#[doc = "Run-Mode Clock Configuration 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC2_SPEC;
impl crate::RegisterSpec for RCC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc2::R`](R) reader structure"]
impl crate::Readable for RCC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rcc2::W`](W) writer structure"]
impl crate::Writable for RCC2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC2 to value 0"]
impl crate::Resettable for RCC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
