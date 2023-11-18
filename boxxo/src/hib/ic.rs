#[doc = "Register `IC` reader"]
pub type R = crate::R<IC_SPEC>;
#[doc = "Register `IC` writer"]
pub type W = crate::W<IC_SPEC>;
#[doc = "Field `HIB_IC_RTCALT0` reader - RTC Alert0 Masked Interrupt Clear"]
pub type HIB_IC_RTCALT0_R = crate::BitReader;
#[doc = "Field `HIB_IC_RTCALT0` writer - RTC Alert0 Masked Interrupt Clear"]
pub type HIB_IC_RTCALT0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_IC_LOWBAT` reader - Low Battery Voltage Masked Interrupt Clear"]
pub type HIB_IC_LOWBAT_R = crate::BitReader;
#[doc = "Field `HIB_IC_LOWBAT` writer - Low Battery Voltage Masked Interrupt Clear"]
pub type HIB_IC_LOWBAT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_IC_EXTW` reader - External Wake-Up Masked Interrupt Clear"]
pub type HIB_IC_EXTW_R = crate::BitReader;
#[doc = "Field `HIB_IC_EXTW` writer - External Wake-Up Masked Interrupt Clear"]
pub type HIB_IC_EXTW_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `HIB_IC_WC` reader - Write Complete/Capable Masked Interrupt Clear"]
pub type HIB_IC_WC_R = crate::BitReader;
#[doc = "Field `HIB_IC_WC` writer - Write Complete/Capable Masked Interrupt Clear"]
pub type HIB_IC_WC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - RTC Alert0 Masked Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_rtcalt0(&self) -> HIB_IC_RTCALT0_R {
        HIB_IC_RTCALT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_lowbat(&self) -> HIB_IC_LOWBAT_R {
        HIB_IC_LOWBAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_extw(&self) -> HIB_IC_EXTW_R {
        HIB_IC_EXTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Clear"]
    #[inline(always)]
    pub fn hib_ic_wc(&self) -> HIB_IC_WC_R {
        HIB_IC_WC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Alert0 Masked Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ic_rtcalt0(&mut self) -> HIB_IC_RTCALT0_W<IC_SPEC, 0> {
        HIB_IC_RTCALT0_W::new(self)
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ic_lowbat(&mut self) -> HIB_IC_LOWBAT_W<IC_SPEC, 2> {
        HIB_IC_LOWBAT_W::new(self)
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ic_extw(&mut self) -> HIB_IC_EXTW_W<IC_SPEC, 3> {
        HIB_IC_EXTW_W::new(self)
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Clear"]
    #[inline(always)]
    #[must_use]
    pub fn hib_ic_wc(&mut self) -> HIB_IC_WC_W<IC_SPEC, 4> {
        HIB_IC_WC_W::new(self)
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
#[doc = "Hibernation Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ic::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ic::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_SPEC;
impl crate::RegisterSpec for IC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic::R`](R) reader structure"]
impl crate::Readable for IC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic::W`](W) writer structure"]
impl crate::Writable for IC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC to value 0"]
impl crate::Resettable for IC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
