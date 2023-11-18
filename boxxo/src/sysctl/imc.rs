#[doc = "Register `IMC` reader"]
pub type R = crate::R<IMC_SPEC>;
#[doc = "Register `IMC` writer"]
pub type W = crate::W<IMC_SPEC>;
#[doc = "Field `SYSCTL_IMC_MOFIM` reader - Main Oscillator Fault Interrupt Mask"]
pub type SYSCTL_IMC_MOFIM_R = crate::BitReader;
#[doc = "Field `SYSCTL_IMC_MOFIM` writer - Main Oscillator Fault Interrupt Mask"]
pub type SYSCTL_IMC_MOFIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_IMC_PLLLIM` reader - PLL Lock Interrupt Mask"]
pub type SYSCTL_IMC_PLLLIM_R = crate::BitReader;
#[doc = "Field `SYSCTL_IMC_PLLLIM` writer - PLL Lock Interrupt Mask"]
pub type SYSCTL_IMC_PLLLIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_IMC_USBPLLLIM` reader - USB PLL Lock Interrupt Mask"]
pub type SYSCTL_IMC_USBPLLLIM_R = crate::BitReader;
#[doc = "Field `SYSCTL_IMC_USBPLLLIM` writer - USB PLL Lock Interrupt Mask"]
pub type SYSCTL_IMC_USBPLLLIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_IMC_MOSCPUPIM` reader - MOSC Power Up Interrupt Mask"]
pub type SYSCTL_IMC_MOSCPUPIM_R = crate::BitReader;
#[doc = "Field `SYSCTL_IMC_MOSCPUPIM` writer - MOSC Power Up Interrupt Mask"]
pub type SYSCTL_IMC_MOSCPUPIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_IMC_VDDAIM` reader - VDDA Power OK Interrupt Mask"]
pub type SYSCTL_IMC_VDDAIM_R = crate::BitReader;
#[doc = "Field `SYSCTL_IMC_VDDAIM` writer - VDDA Power OK Interrupt Mask"]
pub type SYSCTL_IMC_VDDAIM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Main Oscillator Fault Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_mofim(&self) -> SYSCTL_IMC_MOFIM_R {
        SYSCTL_IMC_MOFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_plllim(&self) -> SYSCTL_IMC_PLLLIM_R {
        SYSCTL_IMC_PLLLIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB PLL Lock Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_usbplllim(&self) -> SYSCTL_IMC_USBPLLLIM_R {
        SYSCTL_IMC_USBPLLLIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_moscpupim(&self) -> SYSCTL_IMC_MOSCPUPIM_R {
        SYSCTL_IMC_MOSCPUPIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDA Power OK Interrupt Mask"]
    #[inline(always)]
    pub fn sysctl_imc_vddaim(&self) -> SYSCTL_IMC_VDDAIM_R {
        SYSCTL_IMC_VDDAIM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Main Oscillator Fault Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_imc_mofim(&mut self) -> SYSCTL_IMC_MOFIM_W<IMC_SPEC, 3> {
        SYSCTL_IMC_MOFIM_W::new(self)
    }
    #[doc = "Bit 6 - PLL Lock Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_imc_plllim(&mut self) -> SYSCTL_IMC_PLLLIM_W<IMC_SPEC, 6> {
        SYSCTL_IMC_PLLLIM_W::new(self)
    }
    #[doc = "Bit 7 - USB PLL Lock Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_imc_usbplllim(&mut self) -> SYSCTL_IMC_USBPLLLIM_W<IMC_SPEC, 7> {
        SYSCTL_IMC_USBPLLLIM_W::new(self)
    }
    #[doc = "Bit 8 - MOSC Power Up Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_imc_moscpupim(&mut self) -> SYSCTL_IMC_MOSCPUPIM_W<IMC_SPEC, 8> {
        SYSCTL_IMC_MOSCPUPIM_W::new(self)
    }
    #[doc = "Bit 10 - VDDA Power OK Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_imc_vddaim(&mut self) -> SYSCTL_IMC_VDDAIM_W<IMC_SPEC, 10> {
        SYSCTL_IMC_VDDAIM_W::new(self)
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
#[doc = "Interrupt Mask Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMC_SPEC;
impl crate::RegisterSpec for IMC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imc::R`](R) reader structure"]
impl crate::Readable for IMC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imc::W`](W) writer structure"]
impl crate::Writable for IMC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IMC to value 0"]
impl crate::Resettable for IMC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
