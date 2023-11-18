#[doc = "Register `PIOSCCAL` reader"]
pub type R = crate::R<PIOSCCAL_SPEC>;
#[doc = "Register `PIOSCCAL` writer"]
pub type W = crate::W<PIOSCCAL_SPEC>;
#[doc = "Field `SYSCTL_PIOSCCAL_UT` reader - User Trim Value"]
pub type SYSCTL_PIOSCCAL_UT_R = crate::FieldReader;
#[doc = "Field `SYSCTL_PIOSCCAL_UT` writer - User Trim Value"]
pub type SYSCTL_PIOSCCAL_UT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `SYSCTL_PIOSCCAL_UPDATE` reader - Update Trim"]
pub type SYSCTL_PIOSCCAL_UPDATE_R = crate::BitReader;
#[doc = "Field `SYSCTL_PIOSCCAL_UPDATE` writer - Update Trim"]
pub type SYSCTL_PIOSCCAL_UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PIOSCCAL_CAL` reader - Start Calibration"]
pub type SYSCTL_PIOSCCAL_CAL_R = crate::BitReader;
#[doc = "Field `SYSCTL_PIOSCCAL_CAL` writer - Start Calibration"]
pub type SYSCTL_PIOSCCAL_CAL_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SYSCTL_PIOSCCAL_UTEN` reader - Use User Trim Value"]
pub type SYSCTL_PIOSCCAL_UTEN_R = crate::BitReader;
#[doc = "Field `SYSCTL_PIOSCCAL_UTEN` writer - Use User Trim Value"]
pub type SYSCTL_PIOSCCAL_UTEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:6 - User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_ut(&self) -> SYSCTL_PIOSCCAL_UT_R {
        SYSCTL_PIOSCCAL_UT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Update Trim"]
    #[inline(always)]
    pub fn sysctl_piosccal_update(&self) -> SYSCTL_PIOSCCAL_UPDATE_R {
        SYSCTL_PIOSCCAL_UPDATE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start Calibration"]
    #[inline(always)]
    pub fn sysctl_piosccal_cal(&self) -> SYSCTL_PIOSCCAL_CAL_R {
        SYSCTL_PIOSCCAL_CAL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Use User Trim Value"]
    #[inline(always)]
    pub fn sysctl_piosccal_uten(&self) -> SYSCTL_PIOSCCAL_UTEN_R {
        SYSCTL_PIOSCCAL_UTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - User Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_piosccal_ut(&mut self) -> SYSCTL_PIOSCCAL_UT_W<PIOSCCAL_SPEC, 0> {
        SYSCTL_PIOSCCAL_UT_W::new(self)
    }
    #[doc = "Bit 8 - Update Trim"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_piosccal_update(&mut self) -> SYSCTL_PIOSCCAL_UPDATE_W<PIOSCCAL_SPEC, 8> {
        SYSCTL_PIOSCCAL_UPDATE_W::new(self)
    }
    #[doc = "Bit 9 - Start Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_piosccal_cal(&mut self) -> SYSCTL_PIOSCCAL_CAL_W<PIOSCCAL_SPEC, 9> {
        SYSCTL_PIOSCCAL_CAL_W::new(self)
    }
    #[doc = "Bit 31 - Use User Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn sysctl_piosccal_uten(&mut self) -> SYSCTL_PIOSCCAL_UTEN_W<PIOSCCAL_SPEC, 31> {
        SYSCTL_PIOSCCAL_UTEN_W::new(self)
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
#[doc = "Precision Internal Oscillator Calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`piosccal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`piosccal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIOSCCAL_SPEC;
impl crate::RegisterSpec for PIOSCCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`piosccal::R`](R) reader structure"]
impl crate::Readable for PIOSCCAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`piosccal::W`](W) writer structure"]
impl crate::Writable for PIOSCCAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIOSCCAL to value 0"]
impl crate::Resettable for PIOSCCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
