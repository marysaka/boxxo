#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `SYSCTL_MISC_MOFMIS` reader - Main Oscillator Fault Masked Interrupt Status"]
pub type SYSCTL_MISC_MOFMIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_MISC_MOFMIS` writer - Main Oscillator Fault Masked Interrupt Status"]
pub type SYSCTL_MISC_MOFMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_MISC_PLLLMIS` reader - PLL Lock Masked Interrupt Status"]
pub type SYSCTL_MISC_PLLLMIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_MISC_PLLLMIS` writer - PLL Lock Masked Interrupt Status"]
pub type SYSCTL_MISC_PLLLMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_MISC_USBPLLLMIS` reader - USB PLL Lock Masked Interrupt Status"]
pub type SYSCTL_MISC_USBPLLLMIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_MISC_USBPLLLMIS` writer - USB PLL Lock Masked Interrupt Status"]
pub type SYSCTL_MISC_USBPLLLMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_MISC_MOSCPUPMIS` reader - MOSC Power Up Masked Interrupt Status"]
pub type SYSCTL_MISC_MOSCPUPMIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_MISC_MOSCPUPMIS` writer - MOSC Power Up Masked Interrupt Status"]
pub type SYSCTL_MISC_MOSCPUPMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_MISC_VDDAMIS` reader - VDDA Power OK Masked Interrupt Status"]
pub type SYSCTL_MISC_VDDAMIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_MISC_VDDAMIS` writer - VDDA Power OK Masked Interrupt Status"]
pub type SYSCTL_MISC_VDDAMIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Main Oscillator Fault Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_mofmis(&self) -> SYSCTL_MISC_MOFMIS_R {
        SYSCTL_MISC_MOFMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_plllmis(&self) -> SYSCTL_MISC_PLLLMIS_R {
        SYSCTL_MISC_PLLLMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_usbplllmis(&self) -> SYSCTL_MISC_USBPLLLMIS_R {
        SYSCTL_MISC_USBPLLLMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_moscpupmis(&self) -> SYSCTL_MISC_MOSCPUPMIS_R {
        SYSCTL_MISC_MOSCPUPMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDA Power OK Masked Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_misc_vddamis(&self) -> SYSCTL_MISC_VDDAMIS_R {
        SYSCTL_MISC_VDDAMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Main Oscillator Fault Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_misc_mofmis(&mut self) -> SYSCTL_MISC_MOFMIS_W<MISC_SPEC, 3> {
        SYSCTL_MISC_MOFMIS_W::new(self)
    }
    #[doc = "Bit 6 - PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_misc_plllmis(&mut self) -> SYSCTL_MISC_PLLLMIS_W<MISC_SPEC, 6> {
        SYSCTL_MISC_PLLLMIS_W::new(self)
    }
    #[doc = "Bit 7 - USB PLL Lock Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_misc_usbplllmis(&mut self) -> SYSCTL_MISC_USBPLLLMIS_W<MISC_SPEC, 7> {
        SYSCTL_MISC_USBPLLLMIS_W::new(self)
    }
    #[doc = "Bit 8 - MOSC Power Up Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_misc_moscpupmis(&mut self) -> SYSCTL_MISC_MOSCPUPMIS_W<MISC_SPEC, 8> {
        SYSCTL_MISC_MOSCPUPMIS_W::new(self)
    }
    #[doc = "Bit 10 - VDDA Power OK Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_misc_vddamis(&mut self) -> SYSCTL_MISC_VDDAMIS_W<MISC_SPEC, 10> {
        SYSCTL_MISC_VDDAMIS_W::new(self)
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
#[doc = "Masked Interrupt Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
