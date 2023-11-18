#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `SYSCTL_RIS_MOFRIS` reader - Main Oscillator Fault Raw Interrupt Status"]
pub type SYSCTL_RIS_MOFRIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RIS_MOFRIS` writer - Main Oscillator Fault Raw Interrupt Status"]
pub type SYSCTL_RIS_MOFRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RIS_PLLLRIS` reader - PLL Lock Raw Interrupt Status"]
pub type SYSCTL_RIS_PLLLRIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RIS_PLLLRIS` writer - PLL Lock Raw Interrupt Status"]
pub type SYSCTL_RIS_PLLLRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RIS_USBPLLLRIS` reader - USB PLL Lock Raw Interrupt Status"]
pub type SYSCTL_RIS_USBPLLLRIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RIS_USBPLLLRIS` writer - USB PLL Lock Raw Interrupt Status"]
pub type SYSCTL_RIS_USBPLLLRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RIS_MOSCPUPRIS` reader - MOSC Power Up Raw Interrupt Status"]
pub type SYSCTL_RIS_MOSCPUPRIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RIS_MOSCPUPRIS` writer - MOSC Power Up Raw Interrupt Status"]
pub type SYSCTL_RIS_MOSCPUPRIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_RIS_VDDARIS` reader - VDDA Power OK Event Raw Interrupt Status"]
pub type SYSCTL_RIS_VDDARIS_R = crate::BitReader;
#[doc = "Field `SYSCTL_RIS_VDDARIS` writer - VDDA Power OK Event Raw Interrupt Status"]
pub type SYSCTL_RIS_VDDARIS_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 3 - Main Oscillator Fault Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_mofris(&self) -> SYSCTL_RIS_MOFRIS_R {
        SYSCTL_RIS_MOFRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_plllris(&self) -> SYSCTL_RIS_PLLLRIS_R {
        SYSCTL_RIS_PLLLRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_usbplllris(&self) -> SYSCTL_RIS_USBPLLLRIS_R {
        SYSCTL_RIS_USBPLLLRIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_moscpupris(&self) -> SYSCTL_RIS_MOSCPUPRIS_R {
        SYSCTL_RIS_MOSCPUPRIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - VDDA Power OK Event Raw Interrupt Status"]
    #[inline(always)]
    pub fn sysctl_ris_vddaris(&self) -> SYSCTL_RIS_VDDARIS_R {
        SYSCTL_RIS_VDDARIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Main Oscillator Fault Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ris_mofris(&mut self) -> SYSCTL_RIS_MOFRIS_W<RIS_SPEC, 3> {
        SYSCTL_RIS_MOFRIS_W::new(self)
    }
    #[doc = "Bit 6 - PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ris_plllris(&mut self) -> SYSCTL_RIS_PLLLRIS_W<RIS_SPEC, 6> {
        SYSCTL_RIS_PLLLRIS_W::new(self)
    }
    #[doc = "Bit 7 - USB PLL Lock Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ris_usbplllris(&mut self) -> SYSCTL_RIS_USBPLLLRIS_W<RIS_SPEC, 7> {
        SYSCTL_RIS_USBPLLLRIS_W::new(self)
    }
    #[doc = "Bit 8 - MOSC Power Up Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ris_moscpupris(&mut self) -> SYSCTL_RIS_MOSCPUPRIS_W<RIS_SPEC, 8> {
        SYSCTL_RIS_MOSCPUPRIS_W::new(self)
    }
    #[doc = "Bit 10 - VDDA Power OK Event Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_ris_vddaris(&mut self) -> SYSCTL_RIS_VDDARIS_W<RIS_SPEC, 10> {
        SYSCTL_RIS_VDDARIS_W::new(self)
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
#[doc = "Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
