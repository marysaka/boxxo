#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RIS_SPEC>;
#[doc = "Field `HIB_RIS_RTCALT0` reader - RTC Alert 0 Raw Interrupt Status"]
pub type HIB_RIS_RTCALT0_R = crate::BitReader;
#[doc = "Field `HIB_RIS_RTCALT0` writer - RTC Alert 0 Raw Interrupt Status"]
pub type HIB_RIS_RTCALT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_RIS_LOWBAT` reader - Low Battery Voltage Raw Interrupt Status"]
pub type HIB_RIS_LOWBAT_R = crate::BitReader;
#[doc = "Field `HIB_RIS_LOWBAT` writer - Low Battery Voltage Raw Interrupt Status"]
pub type HIB_RIS_LOWBAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_RIS_EXTW` reader - External Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_EXTW_R = crate::BitReader;
#[doc = "Field `HIB_RIS_EXTW` writer - External Wake-Up Raw Interrupt Status"]
pub type HIB_RIS_EXTW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_RIS_WC` reader - Write Complete/Capable Raw Interrupt Status"]
pub type HIB_RIS_WC_R = crate::BitReader;
#[doc = "Field `HIB_RIS_WC` writer - Write Complete/Capable Raw Interrupt Status"]
pub type HIB_RIS_WC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_rtcalt0(&self) -> HIB_RIS_RTCALT0_R {
        HIB_RIS_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_lowbat(&self) -> HIB_RIS_LOWBAT_R {
        HIB_RIS_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_extw(&self) -> HIB_RIS_EXTW_R {
        HIB_RIS_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Raw Interrupt Status"]
    #[inline(always)]
    pub fn hib_ris_wc(&self) -> HIB_RIS_WC_R {
        HIB_RIS_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ris_rtcalt0(&mut self) -> HIB_RIS_RTCALT0_W<RIS_SPEC, 0> {
        HIB_RIS_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ris_lowbat(&mut self) -> HIB_RIS_LOWBAT_W<RIS_SPEC, 2> {
        HIB_RIS_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ris_extw(&mut self) -> HIB_RIS_EXTW_W<RIS_SPEC, 3> {
        HIB_RIS_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - Write Complete/Capable Raw Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ris_wc(&mut self) -> HIB_RIS_WC_W<RIS_SPEC, 4> {
        HIB_RIS_WC_W::new(self)
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
#[doc = "Hibernation Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
