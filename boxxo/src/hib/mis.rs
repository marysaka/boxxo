#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MIS_SPEC>;
#[doc = "Field `HIB_MIS_RTCALT0` reader - RTC Alert 0 Masked Interrupt Status"]
pub type HIB_MIS_RTCALT0_R = crate::BitReader;
#[doc = "Field `HIB_MIS_RTCALT0` writer - RTC Alert 0 Masked Interrupt Status"]
pub type HIB_MIS_RTCALT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_MIS_LOWBAT` reader - Low Battery Voltage Masked Interrupt Status"]
pub type HIB_MIS_LOWBAT_R = crate::BitReader;
#[doc = "Field `HIB_MIS_LOWBAT` writer - Low Battery Voltage Masked Interrupt Status"]
pub type HIB_MIS_LOWBAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_MIS_EXTW` reader - External Wake-Up Masked Interrupt Status"]
pub type HIB_MIS_EXTW_R = crate::BitReader;
#[doc = "Field `HIB_MIS_EXTW` writer - External Wake-Up Masked Interrupt Status"]
pub type HIB_MIS_EXTW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_MIS_WC` reader - Write Complete/Capable Masked Interrupt Status"]
pub type HIB_MIS_WC_R = crate::BitReader;
#[doc = "Field `HIB_MIS_WC` writer - Write Complete/Capable Masked Interrupt Status"]
pub type HIB_MIS_WC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&self) -> HIB_MIS_RTCALT0_R {
        HIB_MIS_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&self) -> HIB_MIS_LOWBAT_R {
        HIB_MIS_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&self) -> HIB_MIS_EXTW_R {
        HIB_MIS_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_wc(&self) -> HIB_MIS_WC_R {
        HIB_MIS_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_mis_rtcalt0(&mut self) -> HIB_MIS_RTCALT0_W<MIS_SPEC, 0> {
        HIB_MIS_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_mis_lowbat(&mut self) -> HIB_MIS_LOWBAT_W<MIS_SPEC, 2> {
        HIB_MIS_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_mis_extw(&mut self) -> HIB_MIS_EXTW_W<MIS_SPEC, 3> {
        HIB_MIS_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn hib_mis_wc(&mut self) -> HIB_MIS_WC_W<MIS_SPEC, 4> {
        HIB_MIS_WC_W::new(self)
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
#[doc = "Hibernation Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
