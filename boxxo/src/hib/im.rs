#[doc = "Register `IM` reader"]
pub type R = crate::R<IM_SPEC>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<IM_SPEC>;
#[doc = "Field `HIB_IM_RTCALT0` reader - RTC Alert 0 Interrupt Mask"]
pub type HIB_IM_RTCALT0_R = crate::BitReader;
#[doc = "Field `HIB_IM_RTCALT0` writer - RTC Alert 0 Interrupt Mask"]
pub type HIB_IM_RTCALT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_IM_LOWBAT` reader - Low Battery Voltage Interrupt Mask"]
pub type HIB_IM_LOWBAT_R = crate::BitReader;
#[doc = "Field `HIB_IM_LOWBAT` writer - Low Battery Voltage Interrupt Mask"]
pub type HIB_IM_LOWBAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_IM_EXTW` reader - External Wake-Up Interrupt Mask"]
pub type HIB_IM_EXTW_R = crate::BitReader;
#[doc = "Field `HIB_IM_EXTW` writer - External Wake-Up Interrupt Mask"]
pub type HIB_IM_EXTW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_IM_WC` reader - External Write Complete/Capable Interrupt Mask"]
pub type HIB_IM_WC_R = crate::BitReader;
#[doc = "Field `HIB_IM_WC` writer - External Write Complete/Capable Interrupt Mask"]
pub type HIB_IM_WC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rtcalt0(&self) -> HIB_IM_RTCALT0_R {
        HIB_IM_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_lowbat(&self) -> HIB_IM_LOWBAT_R {
        HIB_IM_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_extw(&self) -> HIB_IM_EXTW_R {
        HIB_IM_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_wc(&self) -> HIB_IM_WC_R {
        HIB_IM_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hib_im_rtcalt0(&mut self) -> HIB_IM_RTCALT0_W<IM_SPEC, 0> {
        HIB_IM_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hib_im_lowbat(&mut self) -> HIB_IM_LOWBAT_W<IM_SPEC, 2> {
        HIB_IM_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hib_im_extw(&mut self) -> HIB_IM_EXTW_W<IM_SPEC, 3> {
        HIB_IM_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hib_im_wc(&mut self) -> HIB_IM_WC_W<IM_SPEC, 4> {
        HIB_IM_WC_W::new(self)
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
#[doc = "Hibernation Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for IM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for IM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
